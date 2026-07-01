import { IM_REALTIME_WS } from '@sdkwork/im-sdk';
import {
  DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL,
  DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL,
  DEFAULT_LOCAL_PLATFORM_API_GATEWAY_HTTP_URL,
  VITE_SDKWORK_IAM_APP_API_BASE_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
} from './topologyEnvKeys';
import { readDiscoveredDevGatewayHttpUrl } from './devGatewayDiscoveryCache';

const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_BACKEND_API_PREFIX = '/backend/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';

type RuntimeImportMetaEnv = Record<string, string | boolean | undefined> & {
  DEV?: boolean | 'true' | 'false';
};

function readRuntimeImportMetaEnv(): RuntimeImportMetaEnv {
  return (import.meta.env ?? {}) as RuntimeImportMetaEnv;
}

export function hasViteImportMetaEnv(): boolean {
  return typeof import.meta.env !== 'undefined';
}

export function readSdkBaseUrlEnvValue(key: string): string | undefined {
  const value = readRuntimeImportMetaEnv()[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function readNodeEnvValue(key: string): string | undefined {
  const processLike = (globalThis as {
    process?: {
      env?: Record<string, string | undefined>;
    };
  }).process;
  const value = processLike?.env?.[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

export function isSdkRuntimeDev(): boolean {
  const env = readRuntimeImportMetaEnv();
  if (env.DEV === true || env.DEV === 'true') {
    return true;
  }
  if (env.DEV === false || env.DEV === 'false') {
    return false;
  }
  const nodeEnv = readNodeEnvValue('NODE_ENV');
  if (nodeEnv) {
    return nodeEnv !== 'production';
  }
  return typeof window === 'undefined';
}

export function stripSdkOwnedPathSuffix(pathname: string, suffixes: string[]): string {
  const normalizedPathname = pathname.replace(/\/+$/u, '');
  if (!normalizedPathname || normalizedPathname === '/') {
    return '';
  }

  for (const suffix of suffixes) {
    const normalizedSuffix = `/${suffix.replace(/^\/+|\/+$/gu, '')}`;
    if (normalizedPathname === normalizedSuffix) {
      return '';
    }
    if (normalizedPathname.endsWith(normalizedSuffix)) {
      return normalizedPathname.slice(0, -normalizedSuffix.length) || '';
    }
  }

  return normalizedPathname;
}

export function normalizeHttpSdkBaseUrl(
  value: string,
  sdkOwnedPathSuffixes: string[] = [
    SDKWORK_APP_API_PREFIX,
    SDKWORK_BACKEND_API_PREFIX,
    SDKWORK_IM_API_PREFIX,
  ],
): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

export function normalizeWebSocketSdkBaseUrl(
  value: string,
  sdkOwnedPathSuffixes: string[] = [
    IM_REALTIME_WS,
    SDKWORK_IM_API_PREFIX,
  ],
): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== 'ws:' && parsedUrl.protocol !== 'wss:') {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

export function resolveSameOriginHttpBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const origin = window.location.origin;
  return typeof origin === 'string' && origin.length > 0 ? origin : undefined;
}

export function resolveSameOriginWebSocketBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const { protocol, host } = window.location;
  if (!host) {
    return undefined;
  }
  return `${protocol === 'https:' ? 'wss' : 'ws'}://${host}`;
}

function resolveLocalDevPlatformBaseUrl(): string | undefined {
  if (hasViteImportMetaEnv()) {
    if (!import.meta.env.DEV) {
      return undefined;
    }
  } else if (!isSdkRuntimeDev()) {
    return undefined;
  }
  return DEFAULT_LOCAL_PLATFORM_API_GATEWAY_HTTP_URL;
}

function resolveLocalDevApplicationHttpBaseUrl(): string | undefined {
  if (hasViteImportMetaEnv()) {
    if (!import.meta.env.DEV) {
      return undefined;
    }
  } else if (!isSdkRuntimeDev()) {
    return undefined;
  }
  return DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL;
}

export function deriveWebSocketBaseUrlFromHttpBaseUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizeHttpSdkBaseUrl(value));
    parsedUrl.protocol = parsedUrl.protocol === 'https:' ? 'wss:' : 'ws:';
    return normalizeWebSocketSdkBaseUrl(parsedUrl.toString());
  } catch {
    return undefined;
  }
}

export function resolveAppbaseAppApiBaseUrl(): string | undefined {
  return readSdkBaseUrlEnvValue(VITE_SDKWORK_IAM_APP_API_BASE_URL)
    ?? readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL)
    ?? readSdkBaseUrlEnvValue('VITE_SDKWORK_APPBASE_APP_API_BASE_URL')
    ?? readSdkBaseUrlEnvValue('VITE_SDKWORK_SDK_BASE_URL')
    ?? readDiscoveredDevGatewayHttpUrl()
    ?? resolveLocalDevPlatformBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
}

export function resolveProductAppApiBaseUrl(): string | undefined {
  return readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL)
    ?? readSdkBaseUrlEnvValue('VITE_SDKWORK_IM_SDK_BASE_URL')
    ?? readDiscoveredDevGatewayHttpUrl()
    ?? resolveLocalDevPlatformBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
}

export function resolveImApiBaseUrl(): string | undefined {
  return readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL)
    ?? readSdkBaseUrlEnvValue('VITE_SDKWORK_IM_SDK_BASE_URL')
    ?? readDiscoveredDevGatewayHttpUrl()
    ?? resolveLocalDevApplicationHttpBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
}

export function resolveApplicationOrPlatformHttpBaseUrl(): string | undefined {
  return resolveImApiBaseUrl() ?? resolveAppbaseAppApiBaseUrl();
}

export function resolveApplicationOrPlatformHttpBaseUrlOrThrow(): string {
  const baseUrl = resolveApplicationOrPlatformHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork application SDK base URL is not configured. Set VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL.',
    );
  }
  return normalizeHttpSdkBaseUrl(baseUrl);
}

export function resolveImApiBaseUrlOrThrow(): string {
  const baseUrl = resolveImApiBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork IM SDK API base URL is not configured. Set VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL.',
    );
  }
  return normalizeHttpSdkBaseUrl(baseUrl);
}

export function resolveImWebSocketBaseUrlOrThrow(): string {
  const baseUrl = resolveImWebSocketBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork IM SDK websocket base URL is not configured. Set VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL.',
    );
  }
  return baseUrl;
}

export function resolveProductBackendApiBaseUrl(): string | undefined {
  return readSdkBaseUrlEnvValue('VITE_SDKWORK_IM_BACKEND_API_BASE_URL')
    ?? readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL)
    ?? resolveLocalDevPlatformBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
}

export function resolveAppbaseBackendApiBaseUrl(): string | undefined {
  return readSdkBaseUrlEnvValue('VITE_SDKWORK_IAM_BACKEND_API_BASE_URL')
    ?? readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL)
    ?? readSdkBaseUrlEnvValue('VITE_SDKWORK_APPBASE_BACKEND_API_BASE_URL')
    ?? resolveLocalDevPlatformBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
}

export function resolveImWebSocketBaseUrl(): string | undefined {
  const explicitBaseUrl = readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL);
  if (explicitBaseUrl) {
    return normalizeWebSocketSdkBaseUrl(explicitBaseUrl);
  }
  return deriveWebSocketBaseUrlFromHttpBaseUrl(
    readSdkBaseUrlEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL),
  )
    ?? (resolveLocalDevApplicationHttpBaseUrl()
      ? DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL
      : undefined)
    ?? resolveSameOriginWebSocketBaseUrl();
}
