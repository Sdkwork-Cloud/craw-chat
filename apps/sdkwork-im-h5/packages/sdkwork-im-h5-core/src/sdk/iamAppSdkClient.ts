import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/iam-app-sdk';

export type { SdkworkAppClient as SdkworkIamAppClient };

let iamAppSdkClient: SdkworkAppClient | null = null;

function resolveIamAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_IAM_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_IAM_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createIamAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveIamAppBaseUrl(),
    accessToken: config.accessToken,
    authToken: config.authToken,
    tenantId: config.tenantId,
    organizationId: config.organizationId,
    headers: config.headers,
    platform: 'h5',
    authMode: config.authMode ?? 'dual-token',
    tokenManager: config.tokenManager,
  };
}

export function initIamAppSdkClient(
  config: SdkworkAppConfig = createIamAppSdkClientConfig(),
): SdkworkAppClient {
  iamAppSdkClient = createClient(config);
  return iamAppSdkClient;
}

export function getIamAppSdkClient(): SdkworkAppClient {
  return iamAppSdkClient ?? initIamAppSdkClient();
}

export function resetIamAppSdkClient(): void {
  iamAppSdkClient = null;
}
