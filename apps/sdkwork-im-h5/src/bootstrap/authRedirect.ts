/**
 * Post-authentication redirect target resolution for the H5 auth gate.
 *
 * The target survives two flows:
 * - in-page password/code login — carried by the `redirect` query parameter
 *   of `/auth/login`;
 * - the WeChat official-account authorization round trip — the callback URL
 *   drops the `redirect` parameter, so the target is persisted in
 *   `sessionStorage` before the auth gate sends the user to the login page
 *   and consumed after authentication.
 *
 * Both sources are validated as internal app targets (open-redirect guard).
 */

export const AUTH_HOME_PATH = '/';
export const IM_H5_AUTH_REDIRECT_STORAGE_KEY = 'sdkwork:im:h5:auth:redirect';

export interface ImH5AuthRedirectStorageLike {
  getItem(key: string): string | null;
  removeItem(key: string): void;
  setItem(key: string, value: string): void;
}

function resolveBrowserStorage(): ImH5AuthRedirectStorageLike | null {
  if (typeof window === 'undefined') {
    return null;
  }
  try {
    return window.sessionStorage;
  } catch {
    return null;
  }
}

/**
 * Redirect targets must stay inside the app: a leading slash, no
 * protocol-relative `//` prefix, no URL scheme, and no re-entry into the auth
 * surface (which would loop the gate).
 */
export function isSafeInternalTarget(target: string): boolean {
  const trimmed = target.trim();
  if (!trimmed.startsWith('/') || trimmed.startsWith('//')) {
    return false;
  }
  if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(trimmed)) {
    return false;
  }
  const pathname = trimmed.split(/[?#]/)[0] ?? '';
  return !isAuthPathname(pathname);
}

function isAuthPathname(pathname: string): boolean {
  return pathname === '/auth' || pathname.startsWith('/auth/');
}

/**
 * Resolves where to land after a successful authentication on an auth route:
 * the validated `redirect` query parameter first, then the persisted target
 * (WeChat round trip), then the app home.
 */
export function resolveImH5AuthRedirectTarget(
  search: string,
  storage?: ImH5AuthRedirectStorageLike,
): string {
  const queryTarget = new URLSearchParams(search).get('redirect');
  if (queryTarget && isSafeInternalTarget(queryTarget)) {
    return queryTarget;
  }
  const persisted = readAuthRedirectTarget(storage);
  return persisted && isSafeInternalTarget(persisted) ? persisted : AUTH_HOME_PATH;
}

/** Persists the intended target before the user is sent to the login page. */
export function persistImH5AuthRedirectTarget(
  target: string,
  storage?: ImH5AuthRedirectStorageLike,
): void {
  const store = storage ?? resolveBrowserStorage();
  if (!store) {
    return;
  }
  try {
    store.setItem(IM_H5_AUTH_REDIRECT_STORAGE_KEY, target);
  } catch {
    // Storage may be unavailable (private mode); the query parameter still
    // covers in-page login flows.
  }
}

/** Clears the persisted target once the user has landed after authentication. */
export function clearImH5AuthRedirectTarget(storage?: ImH5AuthRedirectStorageLike): void {
  const store = storage ?? resolveBrowserStorage();
  if (!store) {
    return;
  }
  try {
    store.removeItem(IM_H5_AUTH_REDIRECT_STORAGE_KEY);
  } catch {
    // Best-effort; the value is one-shot and validated on read.
  }
}

function readAuthRedirectTarget(storage?: ImH5AuthRedirectStorageLike): string | null {
  const store = storage ?? resolveBrowserStorage();
  if (!store) {
    return null;
  }
  try {
    const value = store.getItem(IM_H5_AUTH_REDIRECT_STORAGE_KEY);
    return value && value.trim() ? value.trim() : null;
  } catch {
    return null;
  }
}
