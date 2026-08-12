import {
  createClient as createCompanyAppSdkClient,
  type SdkworkCompanyAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/company-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';

import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';

export type CompanyAppSdkClient = SdkworkCompanyAppClient;
export type { SdkworkAppConfig };
export type CompanyAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let companyAppSdkClient: CompanyAppSdkClient | null = null;
let companyPcRuntimeBootstrapped = false;
let companySessionListenerRegistered = false;

export function createCompanyAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): CompanyAppSdkClientConfig {
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

export function initCompanyAppSdkClient(
  config: CompanyAppSdkClientConfig = createCompanyAppSdkClientConfig(),
): CompanyAppSdkClient {
  companyAppSdkClient = createCompanyAppSdkClient(config);
  return companyAppSdkClient;
}

export function getCompanyAppSdkClient(): CompanyAppSdkClient {
  return companyAppSdkClient ?? initCompanyAppSdkClient();
}

export function getCompanyAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CompanyAppSdkClient {
  return initCompanyAppSdkClient(createCompanyAppSdkClientConfig(session));
}

export function resetCompanyAppSdkClient(): void {
  companyAppSdkClient = null;
}

export function syncImSessionToCompanyPc(session = readAppSdkSessionTokens()): void {
  if (!session?.authToken || !session.accessToken) {
    resetCompanyAppSdkClient();
    return;
  }

  resetCompanyAppSdkClient();
  void resolveAppSdkBaseUrl();
}

export function bootstrapCompanyPcForIm(): void {
  syncImSessionToCompanyPc();
  companyPcRuntimeBootstrapped = true;

  if (!companySessionListenerRegistered && typeof window !== 'undefined') {
    window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
      syncImSessionToCompanyPc();
    });
    companySessionListenerRegistered = true;
  }
}

export function isCompanyPcRuntimeBootstrapped(): boolean {
  return companyPcRuntimeBootstrapped;
}

export function resetCompanyPcIntegration(): void {
  companyPcRuntimeBootstrapped = false;
  resetCompanyAppSdkClient();
}

export type CompanyPcHostConfigurator = (options: {
  adapter: import('@sdkwork/company-pc-company').CompanyPcHostAdapter;
}) => void;

export function ensureCompanyPcRuntimeOnModule(
  configureHost: (adapter: import('@sdkwork/company-pc-company').CompanyPcHostAdapter) => void,
  adapter: import('@sdkwork/company-pc-company').CompanyPcHostAdapter,
): void {
  configureHost(adapter);
  companyPcRuntimeBootstrapped = true;
}
