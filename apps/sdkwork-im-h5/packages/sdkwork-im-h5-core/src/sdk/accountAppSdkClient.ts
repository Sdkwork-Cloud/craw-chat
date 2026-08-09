import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/account-app-sdk';

export type { SdkworkAppClient as SdkworkAccountAppClient };

let accountAppSdkClient: SdkworkAppClient | null = null;

function resolveAccountAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_ACCOUNT_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_ACCOUNT_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createAccountAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveAccountAppBaseUrl(),
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

export function initAccountAppSdkClient(
  config: SdkworkAppConfig = createAccountAppSdkClientConfig(),
): SdkworkAppClient {
  accountAppSdkClient = createClient(config);
  return accountAppSdkClient;
}

export function getAccountAppSdkClient(): SdkworkAppClient {
  return accountAppSdkClient ?? initAccountAppSdkClient();
}

export function getAccountAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppClient {
  return initAccountAppSdkClient(createAccountAppSdkClientConfig(config));
}

export function resetAccountAppSdkClient(): void {
  accountAppSdkClient = null;
}
