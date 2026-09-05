/**
 * Browser-safe subset of `sdkwork-specs/tools/browser-cloud-api-base.mjs`.
 *
 * The canonical specs tool is a Node script (`node:fs` / `node:path` plus a
 * `webserver/build-from-topology.mjs` dependency chain that transitively pulls
 * `node:url` / `fileURLToPath`). Importing it from browser code made the Vite
 * bundle crash at runtime with
 * `TypeError: (0 , aa.fileURLToPath) is not a function`
 * (sdkBaseUrls chunk, cloud dev edge). Browser code must therefore use this
 * pure-TypeScript mirror, which inlines exactly the functions reachable from
 * `resolveBrowserCloudSdkBaseUrl`:
 *
 *   resolveBrowserCloudSdkBaseUrl -> resolveCloudApiOriginForHost
 *     -> normalizeCloudApiOriginList / derivePlatformGatewayHostFromPageHost
 *     -> baseDomainFromHost / environmentSuffix (host-registry §9 semantics)
 *
 * Keep behavior byte-compatible with the specs tool; the
 * `sdkwork-im-web-domain-routing-standard` contract and ENVIRONMENT_SPEC §5.1.0.1
 * rely on identical host-derivation results on both sides.
 *
 * Authority: ENVIRONMENT_SPEC §5.1.0.1, APP_RUNTIME_TOPOLOGY_NAMING.md §9.
 */

const CLOUD_API_BASE_URL_SEPARATORS = /[,;]+/u;

const NON_PRODUCTION_SUFFIXES = ['-dev', '-test', '-staging'];

/** PLATFORM_GATEWAY_ROLE from sdkwork-specs/tools/webserver/host-registry.mjs. */
const PLATFORM_GATEWAY_ROLE = 'api';

export function splitCloudApiBaseUrlList(raw: string): string[] {
  return String(raw ?? '')
    .split(CLOUD_API_BASE_URL_SEPARATORS)
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

export function normalizeCloudApiOrigin(value: string): string {
  const token = String(value ?? '').trim();
  if (!token) {
    throw new Error('cloud API base URL must be a non-empty absolute HTTP(S) URL');
  }
  let parsed: URL;
  try {
    parsed = new URL(token);
  } catch {
    throw new Error(`cloud API base URL is not a valid URL: ${token}`);
  }
  if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') {
    throw new Error(`cloud API base URL must use HTTP(S): ${token}`);
  }
  return parsed.origin;
}

export function normalizeCloudApiOriginList(raw: string | string[]): string[] {
  const entries = splitCloudApiBaseUrlList(
    Array.isArray(raw) ? raw.join(';') : raw,
  );
  if (entries.length === 0) {
    throw new Error('cloudApiBaseUrl must declare at least one absolute HTTP(S) URL');
  }
  const origins = entries.map((entry) => normalizeCloudApiOrigin(entry));
  return [...new Set(origins)];
}

/** Mirrors `normalizeHost` in sdkwork-specs/tools/webserver/host-registry.mjs. */
function normalizeHost(host: string): string {
  return typeof host === 'string' ? host.trim().toLowerCase() : '';
}

/** Mirrors `baseDomainFromHost` in sdkwork-specs/tools/webserver/host-registry.mjs. */
export function baseDomainFromHost(host: string): string | null {
  const parts = normalizeHost(host).split('.');
  if (parts.length < 2) {
    return null;
  }
  return parts.slice(-2).join('.');
}

/** Mirrors `environmentSuffix` in sdkwork-specs/tools/webserver/host-registry.mjs. */
export function environmentSuffix(environment: string): string {
  if (environment === 'production') return '';
  if (environment === 'development') return '-dev';
  if (environment === 'test') return '-test';
  if (environment === 'staging') return '-staging';
  if (environment === 'demo') return '-demo';
  return '';
}

function expectedPlatformGatewayHost(environment: string, baseDomain: string): string {
  const suffix = environmentSuffix(environment);
  return `${PLATFORM_GATEWAY_ROLE}${suffix}.${baseDomain}`;
}

export function derivePlatformGatewayHostFromPageHost(
  pageHost: string,
  environment: string,
): string | null {
  const hostname = normalizeHost(pageHost);
  const baseDomain = baseDomainFromHost(hostname);
  if (!baseDomain) {
    return null;
  }
  const roleLabel = hostname.slice(0, -(baseDomain.length + 1));
  let envSuffix = '';
  for (const suffix of NON_PRODUCTION_SUFFIXES) {
    if (roleLabel.endsWith(suffix)) {
      envSuffix = suffix;
      break;
    }
  }
  const configuredSuffix = environmentSuffix(environment);
  if (envSuffix !== configuredSuffix) {
    return null;
  }
  return expectedPlatformGatewayHost(environment, baseDomain);
}

export function resolveCloudApiOriginForHost(
  configuredOrigins: string | string[],
  pageHost: string,
  environment: string,
): string {
  const origins = normalizeCloudApiOriginList(configuredOrigins);
  const hostname = normalizeHost(pageHost);
  const pageBaseDomain = baseDomainFromHost(hostname);
  const expectedApiHost = pageBaseDomain
    ? expectedPlatformGatewayHost(environment, pageBaseDomain)
    : null;
  if (expectedApiHost) {
    const exact = origins.find((origin) => new URL(origin).hostname === expectedApiHost);
    if (exact) {
      return exact;
    }
  }
  if (pageBaseDomain) {
    const matched = origins.find(
      (origin) => baseDomainFromHost(new URL(origin).hostname) === pageBaseDomain,
    );
    if (matched) {
      return matched;
    }
  }
  const derivedHost = derivePlatformGatewayHostFromPageHost(hostname, environment);
  if (derivedHost) {
    const derived = origins.find((origin) => new URL(origin).hostname === derivedHost);
    if (derived) {
      return derived;
    }
    const protocol = origins[0] ? new URL(origins[0]).protocol : 'https:';
    return `${protocol}//${derivedHost}`;
  }
  return origins[0];
}

function inferEnvironmentFromPageHost(pageHost: string): string {
  const hostname = normalizeHost(pageHost);
  const baseDomain = baseDomainFromHost(hostname);
  if (!baseDomain) {
    return 'production';
  }
  const roleLabel = hostname.slice(0, -(baseDomain.length + 1));
  if (roleLabel.endsWith('-dev')) return 'development';
  if (roleLabel.endsWith('-test')) return 'test';
  if (roleLabel.endsWith('-staging')) return 'staging';
  return 'production';
}

export function resolveBrowserCloudSdkBaseUrl(
  configuredValue: string | undefined,
  options: { pageHost?: string; environment?: string } = {},
): string {
  const value = String(configuredValue ?? '').trim();
  if (!value || value === '/') {
    return value;
  }
  if (!CLOUD_API_BASE_URL_SEPARATORS.test(value)) {
    return value;
  }
  const pageHost = options.pageHost
    ?? (typeof globalThis !== 'undefined'
      && globalThis.window
      && typeof globalThis.window.location?.hostname === 'string'
      ? globalThis.window.location.hostname
      : undefined);
  if (!pageHost) {
    return normalizeCloudApiOriginList(value)[0];
  }
  const environment = String(options.environment ?? '').trim() || inferEnvironmentFromPageHost(pageHost);
  return resolveCloudApiOriginForHost(value, pageHost, environment);
}
