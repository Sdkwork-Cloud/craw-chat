import {
  createClient,
  type SdkworkImAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/im-app-sdk';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';
import { resolveApplicationOrPlatformHttpBaseUrlOrThrow } from './sdkBaseUrls';
import type { Interceptors } from '@sdkwork/sdk-common';

export type { SdkworkImAppClient };
export type SdkworkImAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let appSdkClient: SdkworkImAppClient | null = null;

export function resolveAppSdkBaseUrl(): string {
  return resolveApplicationOrPlatformHttpBaseUrlOrThrow();
}

export function createAppSdkClientConfig(session?: SdkworkChatSession | null): SdkworkImAppClientConfig {
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

export function initAppSdkClient(
  config: SdkworkImAppClientConfig = createAppSdkClientConfig(),
): SdkworkImAppClient {
  appSdkClient = createClient(config);
  return appSdkClient;
}

export function getAppSdkClient(): SdkworkImAppClient {
  return appSdkClient ?? initAppSdkClient();
}

export function getAppSdkClientWithSession(session = readAppSdkSessionTokens()): SdkworkImAppClient {
  return initAppSdkClient(createAppSdkClientConfig(session));
}

export function resetAppSdkClient(): void {
  appSdkClient = null;
  portalHomeInFlight = null;
}

// Shared in-flight deduplication for portal.home.retrieve(). Multiple
// services (SettingsService, EnterpriseService, WorkspaceService) call this
// endpoint at startup; without sharing, each fires its own network request.
let portalHomeInFlight: Promise<unknown> | null = null;

export async function retrievePortalHome<TClient extends { portal: { home: { retrieve: () => Promise<TRaw> } }; }, TRaw = unknown>(
  client: TClient,
): Promise<TRaw> {
  if (portalHomeInFlight) {
    return portalHomeInFlight as Promise<TRaw>;
  }
  const promise = client.portal.home.retrieve().finally(() => {
    if (portalHomeInFlight === promise) {
      portalHomeInFlight = null;
    }
  });
  portalHomeInFlight = promise;
  return promise;
}
