import {
  createClient as createCommunityAppSdkClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from 'sdkwork-community-app-sdk-generated-typescript';
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

export type CommunityAppSdkClient = SdkworkAppClient;
export type { SdkworkAppConfig };
export type CommunityAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let communityAppSdkClient: CommunityAppSdkClient | null = null;

export function createCommunityAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): CommunityAppSdkClientConfig {
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

export function initCommunityAppSdkClient(
  config: CommunityAppSdkClientConfig = createCommunityAppSdkClientConfig(),
): CommunityAppSdkClient {
  communityAppSdkClient = createCommunityAppSdkClient(config);
  return communityAppSdkClient;
}

export function getCommunityAppSdkClient(): CommunityAppSdkClient {
  return communityAppSdkClient ?? initCommunityAppSdkClient();
}

export function getCommunityAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CommunityAppSdkClient {
  return initCommunityAppSdkClient(createCommunityAppSdkClientConfig(session));
}

export function resetCommunityAppSdkClient(): void {
  communityAppSdkClient = null;
}
