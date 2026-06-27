import {
  createClient,
  type SdkworkBackendClient as GeneratedSdkworkAppbaseBackendClient,
  type SdkworkBackendConfig,
} from '@sdkwork/iam-backend-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from '@sdkwork/im-pc-core';
import { resolveBackendSdkBaseUrl } from './backendSdkClient';

export type SdkworkAppbaseBackendClient = GeneratedSdkworkAppbaseBackendClient;
export type SdkworkAppbaseBackendClientConfig = SdkworkBackendConfig & {
  interceptors?: Interceptors;
};

let appbaseBackendSdkClient: SdkworkAppbaseBackendClient | null = null;

export function createAppbaseBackendSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkAppbaseBackendClientConfig {
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

export function initAppbaseBackendSdkClient(
  config: SdkworkAppbaseBackendClientConfig = createAppbaseBackendSdkClientConfig(),
): SdkworkAppbaseBackendClient {
  appbaseBackendSdkClient = createClient(config);
  return appbaseBackendSdkClient;
}

export function getAppbaseBackendSdkClient(): SdkworkAppbaseBackendClient {
  return appbaseBackendSdkClient ?? initAppbaseBackendSdkClient();
}

export function getAppbaseBackendSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkAppbaseBackendClient {
  return initAppbaseBackendSdkClient(createAppbaseBackendSdkClientConfig(session));
}

export function resetAppbaseBackendSdkClient(): void {
  appbaseBackendSdkClient = null;
}
