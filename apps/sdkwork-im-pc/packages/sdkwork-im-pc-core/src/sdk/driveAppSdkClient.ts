import {
  createDriveAppClient,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient as GeneratedSdkworkDriveAppClient,
} from '@sdkwork/drive-app-sdk';
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

export type SdkworkDriveAppClient = GeneratedSdkworkDriveAppClient;
export type SdkworkDriveAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let driveAppSdkClient: SdkworkDriveAppClient | null = null;

export function createDriveAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkDriveAppClientConfig {
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

export function initDriveAppSdkClient(
  config: SdkworkDriveAppClientConfig = createDriveAppSdkClientConfig(),
): SdkworkDriveAppClient {
  driveAppSdkClient = createDriveAppClient(config);
  return driveAppSdkClient;
}

export function getDriveAppSdkClient(): SdkworkDriveAppClient {
  return driveAppSdkClient ?? initDriveAppSdkClient();
}

export function getDriveAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkDriveAppClient {
  return initDriveAppSdkClient(createDriveAppSdkClientConfig(session));
}

export function resetDriveAppSdkClient(): void {
  driveAppSdkClient = null;
}
