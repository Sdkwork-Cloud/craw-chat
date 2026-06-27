import {

  createAiotAppClient,

  type SdkworkAiotAppClient as GeneratedSdkworkAiotAppClient,

  type SdkworkAiotAppClientConfig,

} from '@sdkwork/aiot-app-sdk';

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



export type SdkworkAiotAppClient = GeneratedSdkworkAiotAppClient;

export type SdkworkChatAiotAppClientConfig = SdkworkAiotAppClientConfig & {

  interceptors?: Interceptors;

};



let aiotAppSdkClient: SdkworkAiotAppClient | null = null;



export function createAiotAppSdkClientConfig(

  session?: SdkworkChatSession | null,

): SdkworkChatAiotAppClientConfig {

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



export function initAiotAppSdkClient(

  config: SdkworkChatAiotAppClientConfig = createAiotAppSdkClientConfig(),

): SdkworkAiotAppClient {

  aiotAppSdkClient = createAiotAppClient(config);

  return aiotAppSdkClient;

}



export function getAiotAppSdkClient(): SdkworkAiotAppClient {

  return aiotAppSdkClient ?? initAiotAppSdkClient();

}



export function getAiotAppSdkClientWithSession(

  session = readAppSdkSessionTokens(),

): SdkworkAiotAppClient {

  return initAiotAppSdkClient(createAiotAppSdkClientConfig(session));

}



export function resetAiotAppSdkClient(): void {

  aiotAppSdkClient = null;

}
