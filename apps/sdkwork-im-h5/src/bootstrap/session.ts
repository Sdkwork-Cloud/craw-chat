import type {
  SdkworkAppbasePcAuthSessionBridgeSession,
} from '@sdkwork/auth-runtime-pc-react';

export const IM_H5_IAM_SESSION_CHANGED_EVENT = 'sdkwork:im:h5:iam:session:changed';

const IM_H5_SESSION_STORAGE_KEY = 'sdkwork-im-h5-session';

export interface ImH5SessionStorageLike {
  getItem(key: string): string | null;
  removeItem(key: string): void;
  setItem(key: string, value: string): void;
}

export type ImH5PersistedSession = SdkworkAppbasePcAuthSessionBridgeSession;

export interface ImH5SessionBridgeOptions {
  notifySessionChanged?: (session: ImH5PersistedSession | null) => void;
  storage?: ImH5SessionStorageLike | null;
}

export interface ImH5SessionValidationRuntime {
  clearSession(): Promise<void>;
  hydrateTokenManager(): Promise<{
    accessToken?: string | null;
    authToken?: string | null;
  }>;
  retrieveCurrentSession(): Promise<unknown>;
}

function resolveBrowserStorage(): ImH5SessionStorageLike | null {
  return typeof globalThis.localStorage === 'undefined'
    ? null
    : globalThis.localStorage;
}

function normalizeOptionalString(value: unknown): string | undefined {
  const normalized = typeof value === 'string' ? value.trim() : '';
  return normalized || undefined;
}

function normalizeSession(value: unknown): ImH5PersistedSession | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  const record = value as Record<string, unknown>;
  const accessToken = normalizeOptionalString(record.accessToken);
  const authToken = normalizeOptionalString(record.authToken);
  if (!accessToken || !authToken) {
    return null;
  }

  const refreshToken = normalizeOptionalString(record.refreshToken);
  const expiresAt = normalizeOptionalString(record.expiresAt);
  const sessionId = normalizeOptionalString(record.sessionId);
  const context = record.context && typeof record.context === 'object' && !Array.isArray(record.context)
    ? record.context as ImH5PersistedSession['context']
    : undefined;

  return {
    accessToken,
    authToken,
    ...(context ? { context } : {}),
    ...(expiresAt ? { expiresAt } : {}),
    ...(refreshToken ? { refreshToken } : {}),
    ...(sessionId ? { sessionId } : {}),
    ...(record.user !== undefined ? { user: record.user } : {}),
  };
}

export function readImH5PersistedSession(
  storage: ImH5SessionStorageLike | null = resolveBrowserStorage(),
): ImH5PersistedSession | null {
  if (!storage) {
    return null;
  }
  try {
    const raw = storage.getItem(IM_H5_SESSION_STORAGE_KEY);
    return raw ? normalizeSession(JSON.parse(raw)) : null;
  } catch {
    return null;
  }
}

export function createImH5SessionBridge(
  options: ImH5SessionBridgeOptions = {},
) {
  const storage = options.storage === undefined
    ? resolveBrowserStorage()
    : options.storage;
  const notifySessionChanged = options.notifySessionChanged ?? emitImH5SessionChanged;

  return {
    clearSession(): void {
      storage?.removeItem(IM_H5_SESSION_STORAGE_KEY);
      notifySessionChanged(null);
    },
    commitSession(session: ImH5PersistedSession): ImH5PersistedSession {
      const normalized = normalizeSession(session);
      if (!normalized) {
        throw new Error('A complete IAM dual-token session is required.');
      }
      storage?.setItem(IM_H5_SESSION_STORAGE_KEY, JSON.stringify(normalized));
      return normalized;
    },
    readSession(): ImH5PersistedSession | null {
      return readImH5PersistedSession(storage);
    },
  };
}

/**
 * Restores the persisted dual-token session and re-validates it against
 * appbase IAM after a page refresh.
 *
 * Mirrors the PC reference semantics (`refreshCurrentSessionFromServer` in
 * `sdkwork-im-pc-core`): only a definitive server rejection (HTTP 401/403 —
 * the session was revoked or expired) clears the persisted session. Transient
 * failures (offline, timeout, gateway 5xx) keep the persisted session so a
 * refresh never logs the user out — mobile H5 must survive network hiccups.
 */
export async function restoreAndValidateImH5Session(
  runtime: ImH5SessionValidationRuntime,
): Promise<boolean> {
  let tokens: Awaited<ReturnType<ImH5SessionValidationRuntime['hydrateTokenManager']>> = {};
  try {
    tokens = await runtime.hydrateTokenManager();
  } catch {
    // Storage unavailable or unreadable: nothing to restore.
  }
  if (!tokens.accessToken || !tokens.authToken) {
    await runtime.clearSession();
    return false;
  }

  try {
    await runtime.retrieveCurrentSession();
    return true;
  } catch (error) {
    if (isImH5SessionRejectedError(error)) {
      // The IAM side definitively rejected the session: clear it and let
      // AuthGate fall back to the login surface.
      await runtime.clearSession();
      return false;
    }
    // Transient failure: keep the hydrated session and stay authenticated.
    return true;
  }
}

/**
 * Determines whether a current-session validation failure is a definitive
 * credential rejection (HTTP 401/403 or an unauthorized-flagged error) rather
 * than a transient transport/server failure.
 */
function isImH5SessionRejectedError(error: unknown): boolean {
  if (!error || typeof error !== 'object') {
    return false;
  }
  const candidate = error as {
    httpStatus?: number;
    status?: number;
    statusCode?: number;
    response?: { status?: number };
  };
  const status = candidate.httpStatus
    ?? candidate.status
    ?? candidate.statusCode
    ?? candidate.response?.status;
  if (status === 401 || status === 403) {
    return true;
  }
  const message = error instanceof Error ? error.message : String(error);
  return /\b401\b/u.test(message) || /\b403\b/u.test(message) || /unauthorized/iu.test(message);
}

export function emitImH5SessionChanged(
  session: ImH5PersistedSession | null,
): void {
  if (typeof globalThis.dispatchEvent !== 'function' || typeof CustomEvent === 'undefined') {
    return;
  }
  globalThis.dispatchEvent(new CustomEvent(IM_H5_IAM_SESSION_CHANGED_EVENT, {
    detail: { session },
  }));
}
