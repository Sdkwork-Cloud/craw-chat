import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/shop-app-sdk';
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

export type ShopAppSdkClient = SdkworkAppClient;
export type ShopAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let shopAppSdkClient: ShopAppSdkClient | null = null;

export function createShopAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): ShopAppSdkClientConfig {
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

export function initShopAppSdkClient(
  config: ShopAppSdkClientConfig = createShopAppSdkClientConfig(),
): ShopAppSdkClient {
  shopAppSdkClient = createClient(config);
  return shopAppSdkClient;
}

export function getShopAppSdkClient(): ShopAppSdkClient {
  return shopAppSdkClient ?? initShopAppSdkClient();
}

export function getShopAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): ShopAppSdkClient {
  return initShopAppSdkClient(createShopAppSdkClientConfig(session));
}

export function resetShopAppSdkClient(): void {
  shopAppSdkClient = null;
}
