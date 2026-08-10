import {
  createKnowledgebaseAppClient as createGeneratedKnowledgebaseAppClient,
  type SdkworkAppConfig,
  type SdkworkKnowledgebaseAppClient,
} from '@sdkwork/knowledgebase-app-sdk';

export type { SdkworkKnowledgebaseAppClient };

let knowledgebaseAppSdkClient: SdkworkKnowledgebaseAppClient | null = null;

function resolveKnowledgebaseAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createKnowledgebaseAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveKnowledgebaseAppBaseUrl(),
    accessToken: config.accessToken,
    authToken: config.authToken,
    authMode: 'dual-token',
    headers: config.headers,
    platform: 'h5',
    tenantId: config.tenantId,
    organizationId: config.organizationId,
    tokenManager: config.tokenManager,
  };
}

export function initKnowledgebaseAppSdkClient(
  config: SdkworkAppConfig = createKnowledgebaseAppSdkClientConfig(),
): SdkworkKnowledgebaseAppClient {
  knowledgebaseAppSdkClient = createGeneratedKnowledgebaseAppClient(config);
  return knowledgebaseAppSdkClient;
}

export function getKnowledgebaseAppSdkClient(): SdkworkKnowledgebaseAppClient {
  return knowledgebaseAppSdkClient ?? initKnowledgebaseAppSdkClient();
}

export function getKnowledgebaseAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkKnowledgebaseAppClient {
  return initKnowledgebaseAppSdkClient(createKnowledgebaseAppSdkClientConfig(config));
}

export function resetKnowledgebaseAppSdkClient(): void {
  knowledgebaseAppSdkClient = null;
}

export function createKnowledgebaseAppClient(
  config: SdkworkAppConfig = createKnowledgebaseAppSdkClientConfig(),
): SdkworkKnowledgebaseAppClient {
  return createGeneratedKnowledgebaseAppClient(config);
}
