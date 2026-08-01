import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/order-app-sdk';

export type { SdkworkAppClient };

let orderAppSdkClient: SdkworkAppClient | null = null;

function resolveOrderAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_ORDER_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_ORDER_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createOrderAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveOrderAppBaseUrl(),
    accessToken: config.accessToken,
    authToken: config.authToken,
    tenantId: config.tenantId,
    organizationId: config.organizationId,
    headers: config.headers,
    platform: 'h5',
    authMode: config.authMode,
    tokenManager: config.tokenManager,
  };
}

export function initOrderAppSdkClient(
  config: SdkworkAppConfig = createOrderAppSdkClientConfig(),
): SdkworkAppClient {
  orderAppSdkClient = createClient(config);
  return orderAppSdkClient;
}

export function getOrderAppSdkClient(): SdkworkAppClient {
  return orderAppSdkClient ?? initOrderAppSdkClient();
}

export function getOrderAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppClient {
  return initOrderAppSdkClient(createOrderAppSdkClientConfig(config));
}

export function resetOrderAppSdkClient(): void {
  orderAppSdkClient = null;
}
