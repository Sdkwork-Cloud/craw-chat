import type { AuthTokenManager } from '@sdkwork/sdk-common';

export interface ImH5SessionUser {
  id: string;
  name: string;
  avatar?: string;
  status?: "online" | "offline" | "busy";
}

export interface ImH5SessionPort {
  readonly tokenManager: AuthTokenManager;
}

export type ImH5SessionChangeListener = () => void;

const sessionChangeListeners = new Set<ImH5SessionChangeListener>();

export function registerImH5SessionChangeListener(
  listener: ImH5SessionChangeListener,
): () => void {
  sessionChangeListeners.add(listener);
  return () => {
    sessionChangeListeners.delete(listener);
  };
}

export function notifyImH5SessionChanged(): void {
  for (const listener of sessionChangeListeners) {
    try {
      listener();
    } catch {
      // A module teardown failure must not block other registered modules.
    }
  }
}

/**
 * Logout executor injected by the app root.
 *
 * The IAM runtime composition (and the server-side session revoke) is owned
 * by the app root bootstrap; feature packages must not construct raw HTTP,
 * manual auth headers, or their own runtime. The app root registers its
 * executor here so any feature surface (e.g. the settings page) can request
 * a full logout through `requestImH5SessionLogout`.
 */
export type ImH5SessionLogoutHandler = () => Promise<void>;

let sessionLogoutHandler: ImH5SessionLogoutHandler | null = null;

export function registerImH5SessionLogoutHandler(
  handler: ImH5SessionLogoutHandler | null,
): () => void {
  sessionLogoutHandler = handler;
  return () => {
    if (sessionLogoutHandler === handler) {
      sessionLogoutHandler = null;
    }
  };
}

export async function requestImH5SessionLogout(): Promise<void> {
  if (!sessionLogoutHandler) {
    return;
  }
  await sessionLogoutHandler();
}
