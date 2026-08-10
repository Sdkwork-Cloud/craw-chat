import {
  createClient,
  type SdkworkAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/agents-app-sdk';

export type { SdkworkAppClient as SdkworkAgentsAppClient };

let agentsAppSdkClient: SdkworkAppClient | null = null;

/**
 * Resolve the agents app SDK gateway root.
 *
 * The generated agents SDK rejects same-origin `"/"` as an empty base URL, so
 * this resolver produces a concrete gateway root. The final fallback is the
 * browser origin, keeping the same-origin semantics the other H5 SDKs get
 * from `"/"` while satisfying the agents SDK validation.
 */
function resolveAgentsAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const resolved = meta.env?.SDKWORK_AGENTS_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_AGENTS_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL
    ?? meta.env?.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL
    ?? meta.env?.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL
    ?? meta.env?.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL
    ?? resolveBrowserOrigin();
  if (typeof resolved !== 'string' || resolved.trim().length === 0) {
    throw new Error(
      'Agents App SDK requires a gateway root. Set SDKWORK_AGENTS_APP_API_BASE_URL ' +
        '(or SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL / SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL).',
    );
  }
  return resolved.trim();
}

function resolveBrowserOrigin(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  const origin = window.location?.origin;
  if (typeof origin === 'string' && origin.trim().length > 0 && origin !== 'null') {
    return origin.trim();
  }
  return undefined;
}

export function createAgentsAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveAgentsAppBaseUrl(),
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

export function initAgentsAppSdkClient(
  config: SdkworkAppConfig = createAgentsAppSdkClientConfig(),
): SdkworkAppClient {
  agentsAppSdkClient = createClient(config);
  return agentsAppSdkClient;
}

export function getAgentsAppSdkClient(): SdkworkAppClient {
  return agentsAppSdkClient ?? initAgentsAppSdkClient();
}

export function getAgentsAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppClient {
  return initAgentsAppSdkClient(createAgentsAppSdkClientConfig(config));
}

export function resetAgentsAppSdkClient(): void {
  agentsAppSdkClient = null;
}
