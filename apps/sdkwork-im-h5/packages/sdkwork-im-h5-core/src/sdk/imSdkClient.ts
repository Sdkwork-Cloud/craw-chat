import {
  ImSdkClient,
  ImWebSocketAuthOptions,
  IM_REALTIME_WS,
  type ImSdkClientOptions,
} from "@sdkwork/im-sdk";
import {
  DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL,
  DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL,
  VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_WEBSOCKET_URL,
} from "../config/topologyEnvKeys";
import {
  getImH5GlobalTokenManager,
  readImH5IamSessionTokens,
  type ImH5IamSession,
} from "../session/iamSession";

let imSdkClient: ImSdkClient | null = null;
let imSdkClientSessionKey: string | null = null;

const SDKWORK_APP_API_PREFIX = "/app/v3/api";
const SDKWORK_IM_API_PREFIX = "/im/v3/api";

function readEnvValue(...keys: string[]): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  };
  for (const key of keys) {
    const value = meta.env?.[key];
    if (typeof value === "string" && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function stripSdkOwnedPathSuffix(pathname: string, suffixes: string[]): string {
  const normalizedPathname = pathname.replace(/\/+$/u, "");
  if (!normalizedPathname || normalizedPathname === "/") {
    return "";
  }
  for (const suffix of suffixes) {
    const normalizedSuffix = `/${suffix.replace(/^\/+|\/+$/gu, "")}`;
    if (normalizedPathname === normalizedSuffix) {
      return "";
    }
    if (normalizedPathname.endsWith(normalizedSuffix)) {
      return normalizedPathname.slice(0, -normalizedSuffix.length) || "";
    }
  }
  return normalizedPathname;
}

function normalizeHttpSdkBaseUrl(value: string): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(parsedUrl.pathname, [
      SDKWORK_APP_API_PREFIX,
      SDKWORK_IM_API_PREFIX,
    ]);
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

function normalizeWebSocketSdkBaseUrl(value: string): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== "ws:" && parsedUrl.protocol !== "wss:") {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(parsedUrl.pathname, [
      IM_REALTIME_WS,
      SDKWORK_IM_API_PREFIX,
    ]);
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

function deriveWebSocketBaseUrlFromHttpBaseUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  try {
    const parsedUrl = new URL(normalizeHttpSdkBaseUrl(value));
    parsedUrl.protocol = parsedUrl.protocol === "https:" ? "wss:" : "ws:";
    return normalizeWebSocketSdkBaseUrl(parsedUrl.toString());
  } catch {
    return undefined;
  }
}

function resolveLocalDevImApiBaseUrl(): string | undefined {
  if (typeof import.meta.env !== "undefined" && !import.meta.env.DEV) {
    return undefined;
  }
  return DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL;
}

function resolveLocalDevImWebSocketBaseUrl(): string | undefined {
  if (typeof import.meta.env !== "undefined" && !import.meta.env.DEV) {
    return undefined;
  }
  return DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL;
}

function resolveSameOriginHttpBaseUrl(): string | undefined {
  if (typeof window === "undefined") {
    return undefined;
  }
  const origin = window.location.origin;
  return typeof origin === "string" && origin.length > 0 ? origin : undefined;
}

function resolveSameOriginWebSocketBaseUrl(): string | undefined {
  if (typeof window === "undefined") {
    return undefined;
  }
  const { protocol, host } = window.location;
  if (!host) {
    return undefined;
  }
  return `${protocol === "https:" ? "wss" : "ws"}://${host}`;
}

export function resolveImSdkApiBaseUrl(): string {
  const baseUrl =
    readEnvValue(
      VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL,
      VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
    )
    ?? resolveLocalDevImApiBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      "Sdkwork IM H5 SDK API base URL is not configured. Set VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL.",
    );
  }
  return normalizeHttpSdkBaseUrl(baseUrl);
}

export function resolveImSdkWebSocketBaseUrl(): string {
  const explicitBaseUrl = readEnvValue(
    VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_WEBSOCKET_URL,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  );
  const baseUrl =
    explicitBaseUrl
    ?? deriveWebSocketBaseUrlFromHttpBaseUrl(
      readEnvValue(
        VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL,
        VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
      ),
    )
    ?? resolveLocalDevImWebSocketBaseUrl()
    ?? resolveSameOriginWebSocketBaseUrl();
  if (!baseUrl) {
    throw new Error(
      "Sdkwork IM H5 SDK websocket base URL is not configured. Set VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_WEBSOCKET_URL.",
    );
  }
  return explicitBaseUrl ? normalizeWebSocketSdkBaseUrl(baseUrl) : baseUrl;
}

function resolveAccessToken(session?: ImH5IamSession | null): string | undefined {
  const token = session?.accessToken?.trim();
  return token || undefined;
}

function resolveAuthToken(session?: ImH5IamSession | null): string | undefined {
  const token = session?.authToken?.trim();
  return token || undefined;
}

export function createImSdkClientOptions(session?: ImH5IamSession | null): ImSdkClientOptions {
  const currentSession = session ?? readImH5IamSessionTokens();
  const tokenManager = getImH5GlobalTokenManager();
  return {
    apiBaseUrl: resolveImSdkApiBaseUrl(),
    websocketBaseUrl: resolveImSdkWebSocketBaseUrl(),
    accessToken: resolveAccessToken(currentSession),
    authToken: resolveAuthToken(currentSession),
    platform: "h5",
    tokenProvider: tokenManager,
    webSocketAuth: ImWebSocketAuthOptions.automatic({
      credentialProvider: () => resolveAuthToken(readImH5IamSessionTokens()),
    }),
  };
}

function createImSdkClientSessionKey(session?: ImH5IamSession | null): string {
  const currentSession = session ?? readImH5IamSessionTokens();
  const context = currentSession?.context;
  return JSON.stringify({
    accessToken: resolveAccessToken(currentSession) ?? null,
    authToken: resolveAuthToken(currentSession) ?? null,
    organizationId: context?.organizationId ?? null,
    sessionId: currentSession?.sessionId ?? null,
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

export function getImSdkClientWithSession(session = readImH5IamSessionTokens()): ImSdkClient {
  const sessionKey = createImSdkClientSessionKey(session);
  if (imSdkClient && imSdkClientSessionKey === sessionKey) {
    return imSdkClient;
  }
  imSdkClientSessionKey = sessionKey;
  return initImSdkClient(createImSdkClientOptions(session));
}

export function resetImSdkClient(): void {
  imSdkClient = null;
  imSdkClientSessionKey = null;
}
