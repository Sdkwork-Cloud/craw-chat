import {
  createClient,
  type SdkworkAppClient as GeneratedSdkworkAppbaseAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/iam-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';
import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';

export type SdkworkAppbaseAppClient = GeneratedSdkworkAppbaseAppClient;
export type SdkworkAppbaseAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let appbaseAppSdkClient: SdkworkAppbaseAppClient | null = null;

export function createAppbaseAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkAppbaseAppClientConfig {
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

export function initAppbaseAppSdkClient(
  config: SdkworkAppbaseAppClientConfig = createAppbaseAppSdkClientConfig(),
): SdkworkAppbaseAppClient {
  appbaseAppSdkClient = createClient(config);
  return appbaseAppSdkClient;
}

export function getAppbaseAppSdkClient(): SdkworkAppbaseAppClient {
  return appbaseAppSdkClient ?? initAppbaseAppSdkClient();
}

export function getAppbaseAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkAppbaseAppClient {
  return initAppbaseAppSdkClient(createAppbaseAppSdkClientConfig(session));
}

export function resetAppbaseAppSdkClient(): void {
  appbaseAppSdkClient = null;
}
