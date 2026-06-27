import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/knowledgebase-app-sdk';
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

export type KnowledgebaseAppSdkClient = SdkworkAppClient;
export type KnowledgebaseAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let knowledgebaseAppSdkClient: KnowledgebaseAppSdkClient | null = null;

export function createKnowledgebaseAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): KnowledgebaseAppSdkClientConfig {
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

export function initKnowledgebaseAppSdkClient(
  config: KnowledgebaseAppSdkClientConfig = createKnowledgebaseAppSdkClientConfig(),
): KnowledgebaseAppSdkClient {
  knowledgebaseAppSdkClient = createClient(config);
  return knowledgebaseAppSdkClient;
}

export function getKnowledgebaseAppSdkClient(): KnowledgebaseAppSdkClient {
  return knowledgebaseAppSdkClient ?? initKnowledgebaseAppSdkClient();
}

export function getKnowledgebaseAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): KnowledgebaseAppSdkClient {
  return initKnowledgebaseAppSdkClient(createKnowledgebaseAppSdkClientConfig(session));
}

export function resetKnowledgebaseAppSdkClient(): void {
  knowledgebaseAppSdkClient = null;
}
