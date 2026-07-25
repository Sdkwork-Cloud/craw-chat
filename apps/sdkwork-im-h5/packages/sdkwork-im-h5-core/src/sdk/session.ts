import type { IamAppContext } from '@sdkwork/iam-contracts';
import type { AuthTokenManager, AuthTokens, Interceptors, RequestConfig } from '@sdkwork/sdk-common';

export interface SdkworkImH5SessionUser {
  avatar?: string;
  chatId?: string;
  displayName?: string;
  email?: string;
  id?: string | number;
  name?: string;
  nickname?: string;
  phone?: string;
  userId?: string;
  username?: string;
}

export interface SdkworkImH5SessionTokens {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
}

export interface SdkworkImH5AppContext extends IamAppContext {
  actorId?: string;
  actorKind?: string;
  deviceId?: string;
}

export interface SdkworkImH5Session extends SdkworkImH5SessionTokens {
  context?: SdkworkImH5AppContext;
  expiresAt?: number;
  sessionId?: string;
  user?: SdkworkImH5SessionUser;
}

export interface SdkworkImH5SessionChangedDetail {
  session: SdkworkImH5Session | null;
}

export type SdkworkImH5RequestContext = Partial<SdkworkImH5AppContext>;

const SDKWORK_IM_H5_SESSION_KEY = 'sdkwork-im-h5:session:v1';
const SDKWORK_IM_H5_LEGACY_LOCAL_STORAGE_KEY = 'sdkwork-im-h5:session:v1';
export const IM_H5_IAM_SESSION_CHANGED_EVENT = 'sdkwork-im-h5:auth-session-changed';

let sdkworkImH5GlobalTokenManager: AuthTokenManager | null = null;
let sdkworkImH5GlobalTokenManagerSession: SdkworkImH5Session | null = null;
let persistedSessionRawValueCache: string | null | undefined;

function readPersistedSessionRawValue(): string | null {
  if (persistedSessionRawValueCache !== undefined) {
    return persistedSessionRawValueCache;
  }

  migrateLegacyLocalStorage();
  const value = getSessionStorage()?.getItem(SDKWORK_IM_H5_SESSION_KEY) ?? null;
  persistedSessionRawValueCache = value;
  return value;
}

function writePersistedSessionRawValue(value: string): void {
  persistedSessionRawValueCache = value;
  getSessionStorage()?.setItem(SDKWORK_IM_H5_SESSION_KEY, value);
  getLocalStorage()?.removeItem(SDKWORK_IM_H5_LEGACY_LOCAL_STORAGE_KEY);
}

function removePersistedSessionRawValue(): void {
  persistedSessionRawValueCache = null;
  getSessionStorage()?.removeItem(SDKWORK_IM_H5_SESSION_KEY);
  getLocalStorage()?.removeItem(SDKWORK_IM_H5_LEGACY_LOCAL_STORAGE_KEY);
}

function getLocalStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.localStorage;
}

function getSessionStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.sessionStorage;
}

function migrateLegacyLocalStorage(): void {
  const sessionStorage = getSessionStorage();
  const localStorage = getLocalStorage();
  if (!sessionStorage || !localStorage) {
    return;
  }

  const legacyValue = localStorage.getItem(SDKWORK_IM_H5_LEGACY_LOCAL_STORAGE_KEY);
  if (!legacyValue) {
    return;
  }

  if (!sessionStorage.getItem(SDKWORK_IM_H5_SESSION_KEY)) {
    sessionStorage.setItem(SDKWORK_IM_H5_SESSION_KEY, legacyValue);
  }
  localStorage.removeItem(SDKWORK_IM_H5_LEGACY_LOCAL_STORAGE_KEY);
}

function dispatchAppSdkSessionChanged(session: SdkworkImH5Session | null): void {
  if (typeof window === 'undefined') {
    return;
  }
  // Notify registered listeners (e.g. appAuthService TTL cache) before the
  // window event so caches are invalidated atomically with the storage write.
  for (const listener of sessionChangeListeners) {
    try {
      listener(session);
    } catch {
      // Listener errors must not block session persistence.
    }
  }
  window.dispatchEvent(new CustomEvent(
    IM_H5_IAM_SESSION_CHANGED_EVENT,
    { detail: { session } },
  ));
}

const sessionChangeListeners = new Set<(session: SdkworkImH5Session | null) => void>();

export function onAppSdkSessionPersisted(listener: (session: SdkworkImH5Session | null) => void): () => void {
  sessionChangeListeners.add(listener);
  return () => {
    sessionChangeListeners.delete(listener);
  };
}

function normalizeToken(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeStringArray(value: unknown): string[] {
  if (Array.isArray(value)) {
    return value
      .map((item) => normalizeString(item))
      .filter(Boolean) as string[];
  }

  const normalized = normalizeString(value);
  if (!normalized) {
    return [];
  }
  const separator = normalized.includes(',') ? /,/u : /\s+/u;
  return normalized
    .split(separator)
    .map((item) => item.trim())
    .filter(Boolean);
}

function normalizeExpiresAt(value: unknown): number | undefined {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }
  if (typeof value !== 'string' || !value.trim()) {
    return undefined;
  }
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) ? timestamp : undefined;
}

function decodeBase64UrlJson(value: string): Record<string, unknown> | undefined {
  try {
    const normalized = value.replace(/-/gu, '+').replace(/_/gu, '/');
    const padded = normalized.padEnd(Math.ceil(normalized.length / 4) * 4, '=');
    const binary = atob(padded);
    const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
    const text = new TextDecoder().decode(bytes);
    const parsed = JSON.parse(text);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

function decodeJwtClaims(token?: string): Record<string, unknown> | undefined {
  const normalized = normalizeToken(token);
  if (!normalized) {
    return undefined;
  }
  const [, payload] = normalized.split('.');
  if (!payload) {
    return undefined;
  }
  return decodeBase64UrlJson(payload);
}

function readSessionJwtClaims(session?: SdkworkImH5Session | null): Record<string, unknown>[] {
  return [
    decodeJwtClaims(session?.accessToken),
    decodeJwtClaims(session?.authToken),
  ].filter(Boolean) as Record<string, unknown>[];
}

function pickClaimString(
  session: SdkworkImH5Session | null | undefined,
  claimKeys: string[],
  ...fallbacks: unknown[]
): string | undefined {
  for (const claims of readSessionJwtClaims(session)) {
    for (const key of claimKeys) {
      const value = normalizeString(claims[key]);
      if (value) {
        return value;
      }
    }
  }
  return normalizeString(fallbacks.find((value) => normalizeString(value)));
}

function pickClaimStringArray(
  session: SdkworkImH5Session | null | undefined,
  claimKeys: string[],
  fallback?: unknown,
): string[] {
  for (const claims of readSessionJwtClaims(session)) {
    for (const key of claimKeys) {
      const values = normalizeStringArray(claims[key]);
      if (values.length > 0) {
        return values;
      }
    }
  }
  return normalizeStringArray(fallback);
}

function normalizeContext(value: unknown): SdkworkImH5AppContext | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }
  const context = value as Partial<IamAppContext>;
  const record = value as Record<string, unknown>;
  const appId = normalizeString(context.appId) ?? normalizeString(record.app_id);
  const tenantId = normalizeString(context.tenantId) ?? normalizeString(record.tenant_id);
  const userId = normalizeString(context.userId) ?? normalizeString(record.user_id);
  const sessionId = normalizeString(context.sessionId) ?? normalizeString(record.session_id);
  const organizationId = normalizeString(context.organizationId) ?? normalizeString(record.organization_id);
  const environment = normalizeString(context.environment) ?? normalizeString(record.env);
  const deploymentMode = normalizeString(context.deploymentMode) ?? normalizeString(record.deployment_mode);
  const authLevel = normalizeString(context.authLevel) ?? normalizeString(record.auth_level);
  if (!appId || !tenantId || !userId) {
    return undefined;
  }
  return {
    ...context,
    appId,
    ...(organizationId ? { organizationId } : {}),
    tenantId,
    userId,
    sessionId: sessionId ?? '',
    environment: (environment ?? 'dev') as IamAppContext['environment'],
    deploymentMode: (deploymentMode ?? 'saas') as IamAppContext['deploymentMode'],
    authLevel: (authLevel ?? 'password') as IamAppContext['authLevel'],
    dataScope: normalizeStringArray(context.dataScope ?? record.data_scope),
    permissionScope: normalizeStringArray(context.permissionScope ?? record.permission_scope),
  };
}

export function normalizeSdkworkImH5SessionUser(value: unknown): SdkworkImH5SessionUser | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }
  const user = value as Partial<SdkworkImH5SessionUser>;
  const id = normalizeString(user.userId) ?? normalizeString(user.id);
  const avatar = normalizeString(user.avatar);
  const userRecord = value as Record<string, unknown>;
  const chatId = normalizeString(userRecord.chatId)
    ?? normalizeString(userRecord.chat_id)
    ?? normalizeString(userRecord.imId)
    ?? normalizeString(userRecord.im_id)
    ?? normalizeString(userRecord.sdkworkImId)
    ?? normalizeString(userRecord.sdkwork_im_id);
  const normalized: SdkworkImH5SessionUser = {
    ...(avatar ? { avatar } : {}),
    ...(chatId ? { chatId } : {}),
    ...(normalizeString(user.displayName) ? { displayName: normalizeString(user.displayName) } : {}),
    ...(normalizeString(user.email) ? { email: normalizeString(user.email) } : {}),
    ...(id ? { id } : {}),
    ...(normalizeString(user.name) ? { name: normalizeString(user.name) } : {}),
    ...(normalizeString(user.nickname) ? { nickname: normalizeString(user.nickname) } : {}),
    ...(normalizeString(user.phone) ? { phone: normalizeString(user.phone) } : {}),
    ...(normalizeString(user.userId) ? { userId: normalizeString(user.userId) } : {}),
    ...(normalizeString(user.username) ? { username: normalizeString(user.username) } : {}),
  };
  return Object.keys(normalized).length > 0 ? normalized : undefined;
}

function normalizeSession(value: unknown): SdkworkImH5Session | null {
  if (!value || typeof value !== 'object') {
    return null;
  }

  const candidate = value as SdkworkImH5Session & { expiresAt?: unknown };
  const context = normalizeContext(candidate.context);
  const sessionId = normalizeString(candidate.sessionId) ?? normalizeString(context?.sessionId);
  const session: SdkworkImH5Session = {
    accessToken: normalizeToken(candidate.accessToken),
    authToken: normalizeToken(candidate.authToken),
    refreshToken: normalizeToken(candidate.refreshToken),
    ...(context ? { context } : {}),
    ...(typeof candidate.expiresAt !== 'undefined'
      ? { expiresAt: normalizeExpiresAt(candidate.expiresAt) }
      : {}),
    ...(sessionId ? { sessionId } : {}),
    ...(normalizeSdkworkImH5SessionUser(candidate.user) ? { user: normalizeSdkworkImH5SessionUser(candidate.user) } : {}),
  };

  return session.authToken && session.accessToken ? session : null;
}

export function readAppSdkSessionTokens(): SdkworkImH5Session | null {
  const rawValue = readPersistedSessionRawValue();
  if (!rawValue) {
    return null;
  }

  try {
    return normalizeSession(JSON.parse(rawValue));
  } catch {
    removePersistedSessionRawValue();
    return null;
  }
}

export function persistAppSdkSessionTokens(session: SdkworkImH5Session): SdkworkImH5Session {
  const normalizedSession = normalizeSession(session);
  if (!normalizedSession) {
    clearAppSdkSessionTokens();
    throw new Error('Sdkwork IM H5 session requires authToken and accessToken.');
  }

  writePersistedSessionRawValue(JSON.stringify(normalizedSession));
  syncSdkworkImH5GlobalTokenManager(normalizedSession);
  dispatchAppSdkSessionChanged(normalizedSession);
  return normalizedSession;
}

export function applyAppSdkSessionTokens(session: SdkworkImH5Session): SdkworkImH5Session {
  return persistAppSdkSessionTokens(session);
}

export function clearAppSdkSessionTokens(): void {
  removePersistedSessionRawValue();
  syncSdkworkImH5GlobalTokenManager(null);
  dispatchAppSdkSessionChanged(null);
}

export function resolveAppSdkAccessToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.accessToken;
}

export function resolveAppSdkAuthToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.authToken;
}

export function resolveAppSdkRefreshToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.refreshToken;
}

export function resolveAppSdkTenantId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(session, ['tenantId', 'tenant_id', 'tid'], session?.context?.tenantId);
}

export function resolveAppSdkOrganizationId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['organizationId', 'organization_id', 'orgId', 'org_id', 'oid'],
    session?.context?.organizationId,
  );
}

export function resolveAppSdkUserId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['userId', 'user_id', 'uid', 'sub', 'principalId', 'principal_id', 'accountId', 'account_id'],
    session?.context?.userId,
    session?.user?.userId,
    session?.user?.id,
  );
}

export function resolveAppSdkSessionId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['sessionId', 'session_id', 'sid'],
    session?.sessionId,
    session?.context?.sessionId,
  );
}

function resolveAppSdkAppId(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['appId', 'app_id', 'azp', 'aud'], session?.context?.appId);
}

function resolveAppSdkEnvironment(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['environment', 'env'], session?.context?.environment);
}

function resolveAppSdkDeploymentMode(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['deploymentMode', 'deployment_mode'], session?.context?.deploymentMode);
}

function resolveAppSdkAuthLevel(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['authLevel', 'auth_level', 'acr'], session?.context?.authLevel);
}

export function resolveAppSdkActorId(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['actorId', 'actor_id'], session?.context?.actorId);
}

export function resolveAppSdkActorKind(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['actorKind', 'actor_kind'], session?.context?.actorKind);
}

function resolveAppSdkDeviceId(session?: SdkworkImH5Session | null): string | undefined {
  return pickClaimString(session, ['deviceId', 'device_id'], session?.context?.deviceId);
}

export function createSdkworkImH5RequestContext(
  session = readAppSdkSessionTokens(),
): SdkworkImH5RequestContext | undefined {
  const context = session?.context;
  const appId = resolveAppSdkAppId(session);
  const tenantId = resolveAppSdkTenantId(session);
  const organizationId = resolveAppSdkOrganizationId(session);
  const userId = resolveAppSdkUserId(session);
  const sessionId = resolveAppSdkSessionId(session);
  const environment = resolveAppSdkEnvironment(session) as IamAppContext['environment'] | undefined;
  const deploymentMode = resolveAppSdkDeploymentMode(session) as IamAppContext['deploymentMode'] | undefined;
  const authLevel = resolveAppSdkAuthLevel(session) as IamAppContext['authLevel'] | undefined;
  const actorId = resolveAppSdkActorId(session);
  const actorKind = resolveAppSdkActorKind(session);
  const deviceId = resolveAppSdkDeviceId(session);
  const dataScope = pickClaimStringArray(session, ['dataScope', 'data_scope'], context?.dataScope);
  const permissionScope = pickClaimStringArray(
    session,
    ['permissionScope', 'permission_scope', 'scope', 'scp'],
    context?.permissionScope,
  );
  const requestContext: SdkworkImH5RequestContext = {
    ...(appId ? { appId } : {}),
    ...(tenantId ? { tenantId } : {}),
    ...(organizationId ? { organizationId } : {}),
    ...(userId ? { userId } : {}),
    ...(sessionId ? { sessionId } : {}),
    ...(environment ? { environment } : {}),
    ...(deploymentMode ? { deploymentMode } : {}),
    ...(authLevel ? { authLevel } : {}),
    ...(dataScope.length ? { dataScope } : {}),
    ...(permissionScope.length ? { permissionScope } : {}),
    ...(actorId ? { actorId } : {}),
    ...(actorKind ? { actorKind } : {}),
    ...(deviceId ? { deviceId } : {}),
  };
  return Object.keys(requestContext).length > 0 ? requestContext : undefined;
}

export function createSdkworkImH5RequestContextInterceptors(
  _sessionOrReader?: SdkworkImH5Session | null | (() => SdkworkImH5Session | null),
): Interceptors {
  return {
    request: [
      (config: RequestConfig): RequestConfig => {
        return {
          ...config,
          headers: { ...(config.headers ?? {}) },
        };
      },
    ],
    response: [],
    error: [],
  };
}

export function createSdkworkImH5SessionTokenManager(
  sessionOrReader?: SdkworkImH5Session | null | (() => SdkworkImH5Session | null),
): AuthTokenManager {
  let currentSession = typeof sessionOrReader === 'function' ? null : sessionOrReader ?? null;
  let transientAccessToken: string | undefined;
  const readConfiguredSession = () => (
    typeof sessionOrReader === 'function'
      ? sessionOrReader()
      : currentSession
  );
  const readCurrentSession = () => readConfiguredSession() ?? readAppSdkSessionTokens();
  const resolveManagedAccessToken = () => (
    resolveAppSdkAccessToken(readCurrentSession()) ?? transientAccessToken
  );
  const isExpired = () => {
    const expiresAt = readCurrentSession()?.expiresAt;
    return typeof expiresAt === 'number' && Number.isFinite(expiresAt) && Date.now() >= expiresAt;
  };
  const storeTransientAccessToken = (accessToken: string | undefined): void => {
    transientAccessToken = accessToken ? normalizeToken(accessToken) : undefined;
    currentSession = null;
  };
  const updateTokens = (tokens: AuthTokens): void => {
    const existing = readCurrentSession() ?? {};
    const nextAccessToken = tokens.accessToken ?? existing.accessToken;
    const nextAuthToken = tokens.authToken ?? existing.authToken;

    if (nextAccessToken && !nextAuthToken) {
      storeTransientAccessToken(nextAccessToken);
      return;
    }

    transientAccessToken = undefined;
    const expiresAt = typeof tokens.expiresAt === 'number' && Number.isFinite(tokens.expiresAt)
      ? tokens.expiresAt
      : typeof tokens.expiresIn === 'number' && Number.isFinite(tokens.expiresIn)
        ? Date.now() + tokens.expiresIn * 1000
        : existing.expiresAt;
    currentSession = applyAppSdkSessionTokens({
      ...existing,
      accessToken: nextAccessToken,
      authToken: nextAuthToken,
      refreshToken: tokens.refreshToken ?? existing.refreshToken,
      ...(expiresAt ? { expiresAt } : {}),
    });
  };
  const patchTokens = (tokens: Partial<SdkworkImH5SessionTokens>): void => {
    const existing = readCurrentSession() ?? {};
    const next = {
      ...existing,
      ...tokens,
    };
    const normalizedSession = normalizeSession(next);
    if (!normalizedSession) {
      const accessOnly = normalizeToken(next.accessToken);
      const authOnly = normalizeToken(next.authToken);
      if (accessOnly && !authOnly) {
        storeTransientAccessToken(accessOnly);
        return;
      }

      transientAccessToken = undefined;
      currentSession = null;
      clearAppSdkSessionTokens();
      return;
    }
    transientAccessToken = undefined;
    currentSession = applyAppSdkSessionTokens(normalizedSession);
  };

  const hasDualTokens = () => {
    const current = readCurrentSession();
    return Boolean(current?.authToken && current?.accessToken);
  };

  return {
    getAuthToken: () => resolveAppSdkAuthToken(readCurrentSession()),
    getAccessToken: () => resolveManagedAccessToken(),
    getRefreshToken: () => resolveAppSdkRefreshToken(readCurrentSession()),
    getTokens: () => {
      const current = readCurrentSession();
      const accessToken = resolveManagedAccessToken();
      return {
        ...(accessToken ? { accessToken } : {}),
        ...(current?.authToken ? { authToken: current.authToken } : {}),
        ...(current?.refreshToken ? { refreshToken: current.refreshToken } : {}),
        ...(current?.expiresAt ? { expiresAt: current.expiresAt } : {}),
      };
    },
    setTokens: updateTokens,
    setAccessToken: (token: string) => patchTokens({ accessToken: normalizeToken(token) }),
    setAuthToken: (token: string) => patchTokens({ authToken: normalizeToken(token) }),
    setRefreshToken: (token: string) => patchTokens({ refreshToken: normalizeToken(token) }),
    clearTokens: () => {
      currentSession = null;
      transientAccessToken = undefined;
      clearAppSdkSessionTokens();
    },
    clearAuthToken: () => patchTokens({ authToken: undefined }),
    clearAccessToken: () => {
      transientAccessToken = undefined;
      patchTokens({ accessToken: undefined });
    },
    isExpired,
    isValid: () => hasDualTokens() && !isExpired(),
    hasToken: () => hasDualTokens(),
    hasAuthToken: () => Boolean(resolveAppSdkAuthToken(readCurrentSession())),
    hasAccessToken: () => Boolean(resolveManagedAccessToken()),
    willExpireIn: (seconds: number) => {
      const expiresAt = readCurrentSession()?.expiresAt;
      return typeof expiresAt === 'number' && Number.isFinite(expiresAt) && Date.now() + seconds * 1000 >= expiresAt;
    },
  };
}

export function syncSdkworkImH5GlobalTokenManager(session: SdkworkImH5Session | null = readAppSdkSessionTokens()): void {
  sdkworkImH5GlobalTokenManagerSession = session ? normalizeSession(session) : null;
}

export function getSdkworkImH5GlobalTokenManager(): AuthTokenManager {
  syncSdkworkImH5GlobalTokenManager(readAppSdkSessionTokens());
  if (!sdkworkImH5GlobalTokenManager) {
    sdkworkImH5GlobalTokenManager = createSdkworkImH5SessionTokenManager(
      () => sdkworkImH5GlobalTokenManagerSession,
    );
  }
  return sdkworkImH5GlobalTokenManager;
}

export function isAppSdkSessionExpired(session = readAppSdkSessionTokens()): boolean {
  const expiresAt = session?.expiresAt;
  return typeof expiresAt === 'number' && Number.isFinite(expiresAt) && Date.now() >= expiresAt;
}

export function isAppSdkSessionAuthenticated(session = readAppSdkSessionTokens()): boolean {
  return Boolean(session?.authToken && session?.accessToken) && !isAppSdkSessionExpired(session);
}

/**
 * Resolves the permission scope codes attached to a session.
 *
 * Sources, in priority order: JWT claim `permissionScope`/`permission_scope`/`scope`/`scp`
 * (access then auth token), falling back to the normalized `SdkworkImH5AppContext.permissionScope`.
 * Matches the projection used by `createSdkworkImH5RequestContext` so route guards and
 * request interceptors see the same authorization surface.
 */
export function resolveAppSdkPermissionScope(session = readAppSdkSessionTokens()): string[] {
  const context = session?.context;
  return pickClaimStringArray(
    session,
    ['permissionScope', 'permission_scope', 'scope', 'scp'],
    context?.permissionScope,
  );
}

/**
 * Verifies whether a session grants a permission code, mirroring the backend
 * `AppContext::has_permission` semantics:
 *
 * - empty permission never grants;
 * - `*` and `tenant.admin` grant everything;
 * - exact code match grants;
 * - `<prefix>.*` wildcards grant any deeper code in the same namespace.
 */
export function hasAppSdkPermission(
  session: SdkworkImH5Session | null | undefined,
  permission: string,
): boolean {
  const trimmed = permission.trim();
  if (!trimmed) {
    return false;
  }

  const scope = resolveAppSdkPermissionScope(session ?? undefined);
  if (scope.includes('*') || scope.includes('tenant.admin') || scope.includes(trimmed)) {
    return true;
  }

  const segments = trimmed.split('.');
  for (let index = 1; index < segments.length; index++) {
    if (scope.includes(`${segments.slice(0, index).join('.')}.*`)) {
      return true;
    }
  }

  return false;
}

/**
 * Convenience overload that reads the persisted session for callers that only
 * have the permission code in scope (e.g. synchronous module-mount checks).
 */
export function hasAppSdkPermissionForCurrentSession(permission: string): boolean {
  return hasAppSdkPermission(readAppSdkSessionTokens(), permission);
}
