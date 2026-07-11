export interface ImH5AppbaseCallbackSession {
  accessToken: string;
  authToken: string;
}

const CALLBACK_KEYS = {
  accessToken: ["accessToken", "access_token"],
  authToken: ["authToken", "auth_token", "token"],
} as const;

function readParam(params: URLSearchParams, keys: readonly string[]): string {
  for (const key of keys) {
    const value = params.get(key)?.trim();
    if (value) {
      return value;
    }
  }
  return "";
}

export function parseAppbaseCallbackSession(
  search = window.location.search,
  hash = window.location.hash,
): ImH5AppbaseCallbackSession | null {
  const hashQuery = hash.includes("?") ? hash.slice(hash.indexOf("?") + 1) : hash.replace(/^#/, "");
  const params = new URLSearchParams(search);
  for (const [key, value] of new URLSearchParams(hashQuery)) {
    if (!params.has(key)) {
      params.set(key, value);
    }
  }

  const accessToken = readParam(params, CALLBACK_KEYS.accessToken);
  const authToken = readParam(params, CALLBACK_KEYS.authToken);
  if (!accessToken || !authToken) {
    return null;
  }

  return { accessToken, authToken };
}

export function stripAppbaseCallbackFromLocation(): void {
  const url = new URL(window.location.href);
  for (const key of [
    ...CALLBACK_KEYS.accessToken,
    ...CALLBACK_KEYS.authToken,
  ]) {
    url.searchParams.delete(key);
  }

  if (url.hash.includes("?")) {
    const [hashPath = "", hashQuery = ""] = url.hash.split("?");
    const hashParams = new URLSearchParams(hashQuery);
    for (const key of [
      ...CALLBACK_KEYS.accessToken,
      ...CALLBACK_KEYS.authToken,
    ]) {
      hashParams.delete(key);
    }
    const nextHashQuery = hashParams.toString();
    url.hash = nextHashQuery ? `${hashPath}?${nextHashQuery}` : hashPath;
  }

  window.history.replaceState({}, document.title, `${url.pathname}${url.search}${url.hash}`);
}
