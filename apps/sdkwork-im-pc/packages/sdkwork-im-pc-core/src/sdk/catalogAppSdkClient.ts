import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/catalog-app-sdk';
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

export type CatalogAppSdkClient = SdkworkAppClient;
export type CatalogAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let catalogAppSdkClient: CatalogAppSdkClient | null = null;

export function createCatalogAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): CatalogAppSdkClientConfig {
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

export function initCatalogAppSdkClient(
  config: CatalogAppSdkClientConfig = createCatalogAppSdkClientConfig(),
): CatalogAppSdkClient {
  catalogAppSdkClient = createClient(config);
  return catalogAppSdkClient;
}

export function getCatalogAppSdkClient(): CatalogAppSdkClient {
  return catalogAppSdkClient ?? initCatalogAppSdkClient();
}

export function getCatalogAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CatalogAppSdkClient {
  return initCatalogAppSdkClient(createCatalogAppSdkClientConfig(session));
}

export function resetCatalogAppSdkClient(): void {
  catalogAppSdkClient = null;
}
