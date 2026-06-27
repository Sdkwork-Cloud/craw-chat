const DEV_GATEWAY_HTTP_URL_CACHE_KEY = 'sdkwork-im-pc:dev-gateway-http-url';

export function readDiscoveredDevGatewayHttpUrl(): string | undefined {
  if (typeof sessionStorage === 'undefined') {
    return undefined;
  }

  try {
    const value = sessionStorage.getItem(DEV_GATEWAY_HTTP_URL_CACHE_KEY);
    return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
  } catch {
    return undefined;
  }
}

export function writeDiscoveredDevGatewayHttpUrl(baseUrl: string): void {
  if (typeof sessionStorage === 'undefined') {
    return;
  }

  try {
    sessionStorage.setItem(DEV_GATEWAY_HTTP_URL_CACHE_KEY, baseUrl);
  } catch {
    // Best-effort cache for local dev gateway discovery.
  }
}
