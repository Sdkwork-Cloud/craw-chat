import {
  createClient,
  type SdkworkImAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/im-app-sdk';
import {
  createSdkworkImH5RequestContextInterceptors,
  getSdkworkImH5GlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkImH5Session,
} from './session';
import { resolveApplicationOrPlatformHttpBaseUrlOrThrow } from './sdkBaseUrls';
import type { Interceptors } from '@sdkwork/sdk-common';

export type { SdkworkImAppClient };
export type SdkworkImAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let appSdkClient: SdkworkImAppClient | null = null;

export function resolveAppSdkBaseUrl(): string {
  return resolveApplicationOrPlatformHttpBaseUrlOrThrow();
}

export function createAppSdkClientConfig(session?: SdkworkImH5Session | null): SdkworkImAppClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkImH5RequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'h5',
    tokenManager: getSdkworkImH5GlobalTokenManager(),
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
