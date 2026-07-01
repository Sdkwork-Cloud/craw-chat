import { ImSdkClient, ImWebSocketAuthOptions, IM_REALTIME_WS, type ImSdkClientOptions } from '@sdkwork/im-sdk';
import {
  DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL,
  DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL,
} from './topologyEnvKeys';
import {
  resolveImApiBaseUrlOrThrow,
  resolveImWebSocketBaseUrlOrThrow,
} from './sdkBaseUrls';
import {
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  resolveAppSdkSessionId,
  type SdkworkChatSession,
} from './session';

let imSdkClient: ImSdkClient | null = null;
let imSdkClientSessionKey: string | null = null;

export function resolveImSdkApiBaseUrl(): string {
  return resolveImApiBaseUrlOrThrow();
}

export function resolveImSdkWebSocketBaseUrl(): string {
  return resolveImWebSocketBaseUrlOrThrow();
}

export function createImSdkClientOptions(session?: SdkworkChatSession | null): ImSdkClientOptions {
  const currentSession = session ?? readAppSdkSessionTokens();
  const tokenManager = getSdkworkChatGlobalTokenManager();
  return {
    apiBaseUrl: resolveImSdkApiBaseUrl(),
    websocketBaseUrl: resolveImSdkWebSocketBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    platform: 'pc',
    tokenProvider: tokenManager,
    webSocketAuth: ImWebSocketAuthOptions.automatic({
      credentialProvider: () => resolveAppSdkAuthToken(readAppSdkSessionTokens()),
    }),
  };
}

function createImSdkClientSessionKey(session?: SdkworkChatSession | null): string {
  const currentSession = session ?? readAppSdkSessionTokens();
  const context = currentSession?.context;
  return JSON.stringify({
    accessToken: resolveAppSdkAccessToken(currentSession) ?? null,
    authToken: resolveAppSdkAuthToken(currentSession) ?? null,
    organizationId: context?.organizationId ?? null,
    sessionId: resolveAppSdkSessionId(currentSession) ?? null,
    tenantId: context?.tenantId ?? null,
    userId: context?.userId ?? currentSession?.user?.userId ?? currentSession?.user?.id ?? null,
  });
}

export function initImSdkClient(options: ImSdkClientOptions = createImSdkClientOptions()): ImSdkClient {
  imSdkClient = new ImSdkClient(options);
  imSdkClientSessionKey = null;
  return imSdkClient;
}

export function getImSdkClient(): ImSdkClient {
  return imSdkClient ?? initImSdkClient();
}

export function getImSdkClientWithSession(session = readAppSdkSessionTokens()): ImSdkClient {
  const sessionKey = createImSdkClientSessionKey(session);
  if (imSdkClient && imSdkClientSessionKey === sessionKey) {
    return imSdkClient;
  }
  imSdkClient = new ImSdkClient(createImSdkClientOptions(session));
  imSdkClientSessionKey = sessionKey;
  return imSdkClient;
}

export function resetImSdkClient(): void {
  imSdkClient = null;
  imSdkClientSessionKey = null;
}

export {
  DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL,
  DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL,
  IM_REALTIME_WS,
};
