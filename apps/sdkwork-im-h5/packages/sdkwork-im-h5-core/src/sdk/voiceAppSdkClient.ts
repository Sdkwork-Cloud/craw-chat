/**
 * Voice app SDK client construction (H5).
 *
 * The generated `@sdkwork/voice-app-sdk` composed facade is constructed once
 * in bootstrap and injected into feature services. UI packages MUST NOT
 * construct clients; they consume `getVoiceAppSdkClient()` from core.
 */

import {
  createClient as createGeneratedVoiceAppClient,
  type SdkworkAppClient as SdkworkVoiceAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/voice-app-sdk';

export type { SdkworkVoiceAppClient };

let voiceAppSdkClient: SdkworkVoiceAppClient | null = null;

function resolveVoiceAppBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_VOICE_APP_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_VOICE_APP_API_BASE_URL
    ?? meta.env?.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL
    ?? meta.env?.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL
    ?? meta.env?.SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

export function createVoiceAppSdkClientConfig(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkAppConfig {
  return {
    baseUrl: config.baseUrl ?? resolveVoiceAppBaseUrl(),
    accessToken: config.accessToken,
    authToken: config.authToken,
    headers: config.headers,
    platform: 'h5',
    tokenManager: config.tokenManager,
  };
}

export function initVoiceAppSdkClient(
  config: SdkworkAppConfig = createVoiceAppSdkClientConfig(),
): SdkworkVoiceAppClient {
  voiceAppSdkClient = createGeneratedVoiceAppClient(config);
  return voiceAppSdkClient;
}

export function getVoiceAppSdkClient(): SdkworkVoiceAppClient {
  return voiceAppSdkClient ?? initVoiceAppSdkClient();
}

export function getVoiceAppSdkClientWithSession(
  config: Partial<SdkworkAppConfig> = {},
): SdkworkVoiceAppClient {
  return initVoiceAppSdkClient(createVoiceAppSdkClientConfig(config));
}

export function resetVoiceAppSdkClient(): void {
  voiceAppSdkClient = null;
}

export function createVoiceAppSdkClient(
  config: SdkworkAppConfig = createVoiceAppSdkClientConfig(),
): SdkworkVoiceAppClient {
  return createGeneratedVoiceAppClient(config);
}
