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

export async function restoreAndValidateImH5Session(
  runtime: ImH5SessionValidationRuntime,
): Promise<boolean> {
  try {
    const tokens = await runtime.hydrateTokenManager();
    if (tokens.accessToken && tokens.authToken) {
      await runtime.retrieveCurrentSession();
      return true;
    }
  } catch {
    // Invalid or expired sessions converge on the same terminal cleanup below.
  }
  await runtime.clearSession();
  return false;
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
