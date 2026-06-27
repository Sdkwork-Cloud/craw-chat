import {
  createClient,
  type SdkworkAppClient as GeneratedSdkworkAgentAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/agents-app-sdk';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';
import { resolveAppSdkBaseUrl } from './appSdkClient';
import type { Interceptors } from '@sdkwork/sdk-common';

export type SdkworkAgentAppClient = GeneratedSdkworkAgentAppClient;
export type SdkworkAgentAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let agentAppSdkClient: SdkworkAgentAppClient | null = null;

export function createAgentAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): SdkworkAgentAppClientConfig {
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

export function initAgentAppSdkClient(
  config: SdkworkAgentAppClientConfig = createAgentAppSdkClientConfig(),
): SdkworkAgentAppClient {
  agentAppSdkClient = createClient(config);
  return agentAppSdkClient;
}

export function getAgentAppSdkClient(): SdkworkAgentAppClient {
  return agentAppSdkClient ?? initAgentAppSdkClient();
}

export function getAgentAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkAgentAppClient {
  return initAgentAppSdkClient(createAgentAppSdkClientConfig(session));
}

export function resetAgentAppSdkClient(): void {
  agentAppSdkClient = null;
}

export function useAgentAppSdkClient(): SdkworkAgentAppClient {
  return getAgentAppSdkClientWithSession();
}
