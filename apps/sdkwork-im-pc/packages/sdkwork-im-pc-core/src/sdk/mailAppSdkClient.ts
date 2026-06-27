import {
  createMailAppSdkClient,
  type MailAppSdkClient,
  type SdkworkAppConfig,
} from '@sdkwork/mail-app-sdk';
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

export type { MailAppSdkClient };
export type MailAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let mailAppSdkClient: MailAppSdkClient | null = null;

export function createMailAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): MailAppSdkClientConfig {
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

export function initMailAppSdkClient(
  config: MailAppSdkClientConfig = createMailAppSdkClientConfig(),
): MailAppSdkClient {
  mailAppSdkClient = createMailAppSdkClient(config);
  return mailAppSdkClient;
}

export function getMailAppSdkClient(): MailAppSdkClient {
  return mailAppSdkClient ?? initMailAppSdkClient();
}

export function getMailAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): MailAppSdkClient {
  return initMailAppSdkClient(createMailAppSdkClientConfig(session));
}

export function resetMailAppSdkClient(): void {
  mailAppSdkClient = null;
}
