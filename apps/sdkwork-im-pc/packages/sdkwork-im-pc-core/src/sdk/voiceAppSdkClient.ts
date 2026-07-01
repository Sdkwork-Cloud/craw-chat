import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/voice-app-sdk';
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

export type VoiceAppSdkClient = SdkworkAppClient;
export type VoiceAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let voiceAppSdkClient: VoiceAppSdkClient | null = null;

export function createVoiceAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): VoiceAppSdkClientConfig {
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

export function initVoiceAppSdkClient(
  config: VoiceAppSdkClientConfig = createVoiceAppSdkClientConfig(),
): VoiceAppSdkClient {
  voiceAppSdkClient = createClient(config);
  return voiceAppSdkClient;
}

export function getVoiceAppSdkClient(): VoiceAppSdkClient {
  return voiceAppSdkClient ?? initVoiceAppSdkClient();
}

export function getVoiceAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): VoiceAppSdkClient {
  return initVoiceAppSdkClient(createVoiceAppSdkClientConfig(session));
}

export function resetVoiceAppSdkClient(): void {
  voiceAppSdkClient = null;
}
