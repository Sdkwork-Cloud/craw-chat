import {
  createClient,
  type SdkworkImBackendClient as GeneratedSdkworkImBackendClient,
  type SdkworkBackendConfig,
} from '@sdkwork-internal/im-backend-api-generated';
import type { Interceptors } from '@sdkwork/sdk-common';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core';

export type SdkworkImBackendClient = GeneratedSdkworkImBackendClient;
export type SdkworkImBackendClientConfig = SdkworkBackendConfig & {
  interceptors?: Interceptors;
};

let backendSdkClient: SdkworkImBackendClient | null = null;
const SDKWORK_BACKEND_API_PREFIX = '/backend/v3/api';
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';

import { stripSdkOwnedPathSuffix } from '@sdkwork/im-pc-core/sdk/sdkBaseUrls';

function readEnvValue(key: string): string | undefined {
  const value = import.meta.env?.[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeBackendSdkBaseUrl(value: string): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(parsedUrl.pathname, [
      SDKWORK_BACKEND_API_PREFIX,
      SDKWORK_APP_API_PREFIX,
      SDKWORK_IM_API_PREFIX,
    ]);
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

function resolveLocalDevBackendApiBaseUrl(): string | undefined {
  if (!import.meta.env.DEV) {
    return undefined;
  }
  return 'http://127.0.0.1:18079';
}

function resolveSameOriginHttpBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const origin = window.location.origin;
  return typeof origin === 'string' && origin.length > 0 ? origin : undefined;
}

export function resolveBackendSdkBaseUrl(): string {
  const baseUrl = readEnvValue('VITE_SDKWORK_IM_BACKEND_API_BASE_URL')
    ?? readEnvValue('VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL')
    ?? readEnvValue('VITE_SDKWORK_IAM_APP_API_BASE_URL')
    ?? resolveLocalDevBackendApiBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork IM backend SDK base URL is not configured. Set VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL.',
    );
  }
  return normalizeBackendSdkBaseUrl(baseUrl);
}

export function createBackendSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkImBackendClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveBackendSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc-admin',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  };
}

export function initBackendSdkClient(
  config: SdkworkImBackendClientConfig = createBackendSdkClientConfig(),
): SdkworkImBackendClient {
  backendSdkClient = createClient(config);
  return backendSdkClient;
}

export function getBackendSdkClient(): SdkworkImBackendClient {
  return backendSdkClient ?? initBackendSdkClient();
}

export function getBackendSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkImBackendClient {
  return initBackendSdkClient(createBackendSdkClientConfig(session));
}

export function resetBackendSdkClient(): void {
  backendSdkClient = null;
}
