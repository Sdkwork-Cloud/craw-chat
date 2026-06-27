import {
  createNotaryAppClient,
  type SdkworkAppConfig,
  type SdkworkNotaryAppClient as GeneratedSdkworkNotaryAppClient,
} from '@sdkwork/notary-app-sdk';
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

export type SdkworkNotaryAppClient = GeneratedSdkworkNotaryAppClient;
export type SdkworkNotaryAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let notaryAppSdkClient: SdkworkNotaryAppClient | null = null;

export function createNotaryAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkNotaryAppClientConfig {
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

export function initNotaryAppSdkClient(
  config: SdkworkNotaryAppClientConfig = createNotaryAppSdkClientConfig(),
): SdkworkNotaryAppClient {
  notaryAppSdkClient = createNotaryAppClient(config);
  return notaryAppSdkClient;
}

export function getNotaryAppSdkClient(): SdkworkNotaryAppClient {
  return notaryAppSdkClient ?? initNotaryAppSdkClient();
}

export function getNotaryAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkNotaryAppClient {
  return initNotaryAppSdkClient(createNotaryAppSdkClientConfig(session));
}

export function resetNotaryAppSdkClient(): void {
  notaryAppSdkClient = null;
}
