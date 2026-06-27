import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/order-app-sdk';
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

export type OrderAppSdkClient = SdkworkAppClient;
export type OrderAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let orderAppSdkClient: OrderAppSdkClient | null = null;

export function createOrderAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): OrderAppSdkClientConfig {
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

export function initOrderAppSdkClient(
  config: OrderAppSdkClientConfig = createOrderAppSdkClientConfig(),
): OrderAppSdkClient {
  orderAppSdkClient = createClient(config);
  return orderAppSdkClient;
}

export function getOrderAppSdkClient(): OrderAppSdkClient {
  return orderAppSdkClient ?? initOrderAppSdkClient();
}

export function getOrderAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): OrderAppSdkClient {
  return initOrderAppSdkClient(createOrderAppSdkClientConfig(session));
}

export function resetOrderAppSdkClient(): void {
  orderAppSdkClient = null;
}
