import {
  createClient,
  type SdkworkImAppClient as GeneratedSdkworkImAppClient,
  type SdkworkAppConfig,
} from '@sdkwork-internal/im-app-api-generated';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';
import { resolveApplicationOrPlatformHttpBaseUrlOrThrow } from './sdkBaseUrls';
import type { Interceptors } from '@sdkwork/sdk-common';

export type SdkworkImAppClient = GeneratedSdkworkImAppClient;
export type SdkworkImAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let appSdkClient: SdkworkImAppClient | null = null;

export function resolveAppSdkBaseUrl(): string {
  return resolveApplicationOrPlatformHttpBaseUrlOrThrow();
}

export function createAppSdkClientConfig(session?: SdkworkChatSession | null): SdkworkImAppClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  };
}

export function initAppSdkClient(
  config: SdkworkImAppClientConfig = createAppSdkClientConfig(),
): SdkworkImAppClient {
  appSdkClient = createClient(config);
  return appSdkClient;
}

export function getAppSdkClient(): SdkworkImAppClient {
  return appSdkClient ?? initAppSdkClient();
}

export function getAppSdkClientWithSession(session = readAppSdkSessionTokens()): SdkworkImAppClient {
  return initAppSdkClient(createAppSdkClientConfig(session));
}

export function resetAppSdkClient(): void {
  appSdkClient = null;
}
