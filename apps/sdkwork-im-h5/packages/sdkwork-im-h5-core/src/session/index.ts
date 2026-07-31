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
