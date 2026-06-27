const APP_API_PREFIX = "/app/v3/api";

function stripAppApiSuffix(pathname: string): string {
  const normalized = pathname.replace(/\/+$/u, "");
  if (!normalized || normalized === APP_API_PREFIX) {
    return "";
  }
  if (normalized.endsWith(APP_API_PREFIX)) {
    return normalized.slice(0, -APP_API_PREFIX.length) || "";
  }
  return normalized;
}

export function resolveAppSdkBaseUrl(apiBaseUrl: string): string {
  try {
    const parsed = new URL(apiBaseUrl);
    const normalizedPath = stripAppApiSuffix(parsed.pathname);
    return `${parsed.origin}${normalizedPath}`;
  } catch {
    return apiBaseUrl.replace(/\/app\/v3\/api\/?$/u, "");
  }
}
