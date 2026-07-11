import type { IamAppContext } from "@sdkwork/iam-contracts";
import {
  createTokenManager,
  type AuthTokenManager,
  type AuthTokens,
} from "@sdkwork/sdk-common";

import type { ImH5AppSession } from "./appSession";

export interface ImH5IamSessionUser {
  displayName?: string;
  email?: string;
  id?: string;
  name?: string;
  nickname?: string;
  userId?: string;
  username?: string;
}

export interface ImH5IamSession {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: IamAppContext;
  user?: ImH5IamSessionUser;
}

export const IM_H5_IAM_SESSION_STORAGE_KEY = "sdkwork-im-h5:session:v1";
export const IM_H5_IAM_SESSION_CHANGED_EVENT = "sdkwork-im-h5:auth-session-changed";

let imH5GlobalTokenManager: AuthTokenManager | null = null;

function getStorage(): Storage | undefined {
  if (typeof window === "undefined") {
    return undefined;
  }
  return window.sessionStorage;
}

function normalizeToken(value: unknown): string | undefined {
  return typeof value === "string" && value.trim().length > 0 ? value.trim() : undefined;
}

function dispatchImH5IamSessionChanged(session: ImH5IamSession | null): void {
  if (typeof window === "undefined") {
    return;
  }
  window.dispatchEvent(
    new CustomEvent(IM_H5_IAM_SESSION_CHANGED_EVENT, {
      detail: { session },
    }),
  );
}

function parseStoredImH5IamSession(raw: string): ImH5IamSession | null {
  try {
    const parsed = JSON.parse(raw) as Partial<ImH5IamSession>;
    const accessToken = normalizeToken(parsed.accessToken);
    const authToken = normalizeToken(parsed.authToken);
    if (!accessToken || !authToken) {
      return null;
    }

    return {
      accessToken,
      authToken,
      ...(normalizeToken(parsed.refreshToken) ? { refreshToken: parsed.refreshToken } : {}),
      ...(parsed.sessionId ? { sessionId: parsed.sessionId } : {}),
      ...(parsed.context ? { context: parsed.context } : {}),
      ...(parsed.user ? { user: parsed.user } : {}),
    };
  } catch {
    return null;
  }
}

export function readImH5IamSessionTokens(): ImH5IamSession | null {
  const storage = getStorage();
  if (!storage) {
    return null;
  }

  const raw = storage.getItem(IM_H5_IAM_SESSION_STORAGE_KEY);
  if (!raw) {
    return null;
  }

  const session = parseStoredImH5IamSession(raw);
  if (!session || !normalizeToken(session.accessToken) || !normalizeToken(session.authToken)) {
    return null;
  }
  return session;
}

export function applyImH5IamSessionTokens(session: ImH5IamSession): ImH5IamSession {
  const storage = getStorage();
  const accessToken = normalizeToken(session.accessToken);
  const authToken = normalizeToken(session.authToken);
  if (!accessToken || !authToken) {
    throw new Error("IM H5 IAM session requires both accessToken and authToken.");
  }
  const normalized: ImH5IamSession = {
    accessToken,
    authToken,
    ...(normalizeToken(session.refreshToken) ? { refreshToken: session.refreshToken } : {}),
    ...(session.sessionId ? { sessionId: session.sessionId } : {}),
    ...(session.context ? { context: session.context } : {}),
    ...(session.user ? { user: session.user } : {}),
  };

  if (storage) {
    storage.setItem(IM_H5_IAM_SESSION_STORAGE_KEY, JSON.stringify(normalized));
  }

  const tokenManager = getImH5GlobalTokenManager();
  tokenManager.setTokens({
    ...(normalized.accessToken ? { accessToken: normalized.accessToken } : {}),
    ...(normalized.authToken ? { authToken: normalized.authToken } : {}),
    ...(normalized.refreshToken ? { refreshToken: normalized.refreshToken } : {}),
  });

  dispatchImH5IamSessionChanged(normalized);
  return normalized;
}

export function clearImH5IamSessionTokens(): void {
  const storage = getStorage();
  storage?.removeItem(IM_H5_IAM_SESSION_STORAGE_KEY);
  getImH5GlobalTokenManager().clearTokens();
  dispatchImH5IamSessionChanged(null);
}

export function isImH5IamSessionAuthenticated(session: ImH5IamSession | null): boolean {
  return Boolean(normalizeToken(session?.accessToken) && normalizeToken(session?.authToken));
}

export function toImH5AppSession(session: ImH5IamSession | null): ImH5AppSession | null {
  if (!isImH5IamSessionAuthenticated(session)) {
    return null;
  }
  const tenantId = session!.context?.tenantId?.trim();
  const organizationId = session!.context?.organizationId?.trim();
  const userId =
    session!.context?.userId?.trim()
    || session!.user?.userId?.trim()
    || session!.user?.id?.trim();
  if (!tenantId || !organizationId || !userId) {
    return null;
  }
  return {
    accessToken: session!.accessToken!.trim(),
    authToken: session!.authToken!.trim(),
    tenantId,
    organizationId,
    userId,
  };
}

export function getImH5GlobalTokenManager(): AuthTokenManager {
  if (!imH5GlobalTokenManager) {
    imH5GlobalTokenManager = createTokenManager();
    const snapshot = readImH5IamSessionTokens();
    if (snapshot) {
      imH5GlobalTokenManager.setTokens({
        ...(snapshot.accessToken ? { accessToken: snapshot.accessToken } : {}),
        ...(snapshot.authToken ? { authToken: snapshot.authToken } : {}),
        ...(snapshot.refreshToken ? { refreshToken: snapshot.refreshToken } : {}),
      } as AuthTokens);
    }
  }
  return imH5GlobalTokenManager;
}
