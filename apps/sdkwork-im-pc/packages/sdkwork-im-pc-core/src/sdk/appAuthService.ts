import { createSdkworkAuthAppbaseIntegration } from '@sdkwork/auth-pc-react';
import {
  clearSdkworkChatIamRuntimeSession,
  getSdkworkChatIamRuntime,
  resetSdkworkChatAuthenticatedSdkClients,
  resetSdkworkChatIamRuntime,
} from './appAuthRuntime';
import { resolveDesktopOfflinePrincipalScope } from './desktopOfflineScope';
import { purgeDesktopOfflinePrincipalCache } from './desktopOfflineStore';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  isAppSdkSessionAuthenticated,
  normalizeSdkworkChatSessionUser,
  onAppSdkSessionPersisted,
  readAppSdkSessionTokens,
  type SdkworkChatSession,
  type SdkworkChatSessionUser,
} from './session';

export interface AppAuthService {
  getCurrentSession(): Promise<SdkworkChatSession | null>;
  logout(): Promise<void>;
}

interface RuntimeSessionPayload {
  accessToken?: string;
  authToken?: string;
  context?: unknown;
  expiresAt?: number | string;
  refreshToken?: string;
  sessionId?: string;
  user?: SdkworkChatSessionUser;
  userInfo?: SdkworkChatSessionUser;
}

type RuntimeSessionContext = NonNullable<SdkworkChatSession['context']>;

function isDuplicateCapabilityPackageError(error: unknown): boolean {
  if (!(error instanceof Error)) {
    return false;
  }

  return error.message.includes('Duplicate capability package');
}

function createSdkworkChatAuthIntegration() {
  try {
    return createSdkworkAuthAppbaseIntegration({
      app: {
        id: 'sdkwork-im-pc',
        title: 'Sdkwork IM PC',
      },
      basePath: '/auth',
      extraPackageNames: [
        '@sdkwork/im-pc-react',
      ],
    });
  } catch (error) {
    if (isDuplicateCapabilityPackageError(error)) {
      return createSdkworkAuthAppbaseIntegration({
        app: {
          id: 'sdkwork-im-pc',
          title: 'Sdkwork IM PC',
        },
        basePath: '/auth',
      });
    }

    throw error;
  }
}

const sdkworkChatAuthIntegration = createSdkworkChatAuthIntegration();

export const sdkworkChatAuthAppbaseManifest = sdkworkChatAuthIntegration.manifest;

export const sdkworkChatAuthRoutes = sdkworkChatAuthIntegration.routes;

export const sdkworkChatAuthAppbaseMeta = sdkworkChatAuthIntegration.appbaseMeta;

function normalizeContextString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeContextStringArray(value: unknown): string[] {
  if (Array.isArray(value)) {
    return value
      .map((item) => normalizeContextString(item))
      .filter(Boolean) as string[];
  }

  const normalized = normalizeContextString(value);
  if (!normalized) {
    return [];
  }

  return normalized
    .split(normalized.includes(',') ? /,/u : /\s+/u)
    .map((item) => item.trim())
    .filter(Boolean);
}

function normalizeRuntimeSessionContext(value: unknown): SdkworkChatSession['context'] | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }

  const context = value as Record<string, unknown>;
  const appId = normalizeContextString(context.appId ?? context.app_id);
  const tenantId = normalizeContextString(context.tenantId ?? context.tenant_id);
  const userId = normalizeContextString(context.userId ?? context.user_id);
  const sessionId = normalizeContextString(context.sessionId ?? context.session_id);
  if (!appId || !tenantId || !userId || !sessionId) {
    return undefined;
  }

  return {
    appId,
    authLevel: (normalizeContextString(context.authLevel ?? context.auth_level) ?? 'password') as RuntimeSessionContext['authLevel'],
    dataScope: normalizeContextStringArray(context.dataScope ?? context.data_scope),
    deploymentMode: (normalizeContextString(context.deploymentMode ?? context.deployment_mode) ?? 'saas') as RuntimeSessionContext['deploymentMode'],
    environment: (normalizeContextString(context.environment ?? context.env) ?? 'dev') as RuntimeSessionContext['environment'],
    ...(normalizeContextString(context.organizationId ?? context.organization_id)
      ? { organizationId: normalizeContextString(context.organizationId ?? context.organization_id) }
      : {}),
    permissionScope: normalizeContextStringArray(context.permissionScope ?? context.permission_scope),
    sessionId,
    tenantId,
    userId,
  };
}

function toSession(data: RuntimeSessionPayload): SdkworkChatSession {
  const expiresAt = typeof data.expiresAt === 'string' ? Date.parse(data.expiresAt) : data.expiresAt;
  const context = normalizeRuntimeSessionContext(data.context);
  return {
    accessToken: data.accessToken,
    authToken: data.authToken,
    refreshToken: data.refreshToken,
    ...(context ? { context } : {}),
    ...(expiresAt ? { expiresAt } : {}),
    ...(data.sessionId ?? context?.sessionId ? { sessionId: data.sessionId ?? context?.sessionId } : {}),
    ...(normalizeSdkworkChatSessionUser(data.user ?? data.userInfo)
      ? { user: normalizeSdkworkChatSessionUser(data.user ?? data.userInfo) }
      : {}),
  };
}

function isAuthSessionRejectedError(error: unknown): boolean {
  if (!error || typeof error !== 'object') {
    return false;
  }

  const candidate = error as {
    status?: number;
    statusCode?: number;
    response?: { status?: number };
    cause?: unknown;
  };
  const status = candidate.status
    ?? candidate.statusCode
    ?? candidate.response?.status;
  if (status === 401 || status === 403) {
    return true;
  }

  const message = error instanceof Error ? error.message : String(error);
  return /\b401\b/u.test(message) || /\b403\b/u.test(message) || /unauthorized/iu.test(message);
}

let currentSessionRefreshPromise: Promise<SdkworkChatSession | null> | null = null;

// TTL cache for getCurrentSession: AuthGate and ChatLayout both call
// getCurrentSession() at startup. Without a cache, the second caller always
// re-hits the network because the single-flight promise is already cleared.
// A short TTL lets sequential startup callers share one network round-trip.
const CURRENT_SESSION_TTL_MS = 30_000;
let currentSessionCache: { session: SdkworkChatSession | null; expiresAt: number } | null = null;

// Keep the TTL cache in sync when the session is persisted or cleared by
// other code paths (token refresh, login commit, logout). This refreshes the
// cached value and resets the TTL window so callers see the latest session.
onAppSdkSessionPersisted((session) => {
  currentSessionCache = { session, expiresAt: Date.now() + CURRENT_SESSION_TTL_MS };
});

async function refreshCurrentSessionFromServer(): Promise<SdkworkChatSession | null> {
  const storedSession = readAppSdkSessionTokens();
  if (!isAppSdkSessionAuthenticated(storedSession)) {
    clearSdkworkChatIamRuntimeSession();
    return null;
  }

  try {
    const session = await getSdkworkChatIamRuntime().service.auth.sessions.current.retrieve();
    return applyAppSdkSessionTokens(toSession(session as unknown as RuntimeSessionPayload));
  } catch (error) {
    if (isAuthSessionRejectedError(error)) {
      clearSdkworkChatIamRuntimeSession();
      resetSdkworkChatIamRuntime();
      return null;
    }
    return storedSession;
  }
}

export const appAuthService: AppAuthService = {
  async getCurrentSession() {
    // Serve from TTL cache when fresh.
    if (currentSessionCache && Date.now() < currentSessionCache.expiresAt) {
      return currentSessionCache.session;
    }
    // Collapse concurrent callers onto a single in-flight request.
    if (currentSessionRefreshPromise) {
      return currentSessionRefreshPromise;
    }

    currentSessionCache = null;
    currentSessionRefreshPromise = refreshCurrentSessionFromServer().finally(() => {
      currentSessionRefreshPromise = null;
    });
    const session = await currentSessionRefreshPromise;
    currentSessionCache = { session, expiresAt: Date.now() + CURRENT_SESSION_TTL_MS };
    return session;
  },

  async logout() {
    const offlineScope = resolveDesktopOfflinePrincipalScope(readAppSdkSessionTokens());
    let remoteLogoutError: unknown;
    let offlinePurgeError: unknown;
    try {
      await getSdkworkChatIamRuntime().service.auth.sessions.current.delete();
    } catch (error) {
      remoteLogoutError = error;
    }
    try {
      if (offlineScope) {
        await purgeDesktopOfflinePrincipalCache(offlineScope);
      }
    } catch (error) {
      offlinePurgeError = error;
    } finally {
      currentSessionCache = null;
      clearAppSdkSessionTokens();
      resetSdkworkChatAuthenticatedSdkClients();
      resetSdkworkChatIamRuntime();
    }
    if (remoteLogoutError && offlinePurgeError) {
      throw new AggregateError(
        [remoteLogoutError, offlinePurgeError],
        'Remote logout and desktop offline cache purge both failed.',
      );
    }
    if (offlinePurgeError) {
      throw offlinePurgeError;
    }
    if (remoteLogoutError) {
      throw remoteLogoutError;
    }
  },
};
