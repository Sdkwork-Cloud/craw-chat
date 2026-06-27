export interface ImH5Environment {
  applicationPublicHttpUrl: string;
  applicationPublicWebSocketUrl: string;
  appbaseAppApiBaseUrl: string;
}

function normalizeBaseUrl(value: string | undefined, fallback: string): string {
  const normalized = String(value ?? "").trim();
  return normalized || fallback;
}

function deriveAppApiBaseUrl(applicationPublicHttpUrl: string): string {
  return `${applicationPublicHttpUrl.replace(/\/+$/u, "")}/app/v3/api`;
}

export function resolveEnvironment(): ImH5Environment {
  const applicationPublicHttpUrl = normalizeBaseUrl(
    import.meta.env.VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL
      ?? import.meta.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
    "http://127.0.0.1:18079",
  );

  const applicationPublicWebSocketUrl = normalizeBaseUrl(
    import.meta.env.VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_WEBSOCKET_URL
      ?? import.meta.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
    applicationPublicHttpUrl.replace(/^http/u, "ws"),
  );

  return {
    applicationPublicHttpUrl,
    applicationPublicWebSocketUrl,
    appbaseAppApiBaseUrl: normalizeBaseUrl(
      import.meta.env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
      deriveAppApiBaseUrl(applicationPublicHttpUrl),
    ),
  };
}
