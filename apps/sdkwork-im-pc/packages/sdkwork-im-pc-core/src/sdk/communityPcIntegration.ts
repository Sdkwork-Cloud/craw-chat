import {
  createClient as createCommunityAppSdkClient,
  type SdkworkCommunityAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/community-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

export type CommunityAppSdkClient = SdkworkCommunityAppClient;
export type { SdkworkAppConfig };
export type CommunityAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let communityAppSdkClient: CommunityAppSdkClient | null = null;
let communityPcRuntimeBootstrapped = false;
let communitySessionListenerRegistered = false;

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

export function syncImSessionToCommunityPc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetCommunityAppSdkClient();
    return;
  }

  resetCommunityAppSdkClient();
  void resolveAppSdkBaseUrl();
}

export function bootstrapCommunityPcForIm(): void {
  syncImSessionToCommunityPc();
  communityPcRuntimeBootstrapped = true;

  if (!communitySessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToCommunityPc();
    });
    communitySessionListenerRegistered = true;
  }
}

export function isCommunityPcRuntimeBootstrapped(): boolean {
  return communityPcRuntimeBootstrapped;
}

export function resetCommunityPcIntegration(): void {
  communityPcRuntimeBootstrapped = false;
  resetCommunityAppSdkClient();
}
