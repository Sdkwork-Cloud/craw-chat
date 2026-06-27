#!/usr/bin/env node

import {
  DEFAULT_DEV_PROFILE_ID,
  applyProfileEnv,
  loadProfile,
} from '../lib/im-topology.mjs';
import {
  deriveWebSocketBaseUrlFromHttpBaseUrl,
  isStandaloneUnifiedProcess,
  resolveApplicationPublicHttpUrl,
  resolveSdkworkApiGatewayBaseUrl,
} from '../lib/im-pc-dev.mjs';
import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs';

const DEFAULT_DEV_GATEWAY_HOST = '127.0.0.1';
const DEFAULT_DEV_GATEWAY_START_PORT = 18079;
const DEFAULT_DEV_GATEWAY_PORT_ATTEMPTS = 20;
const DEV_GATEWAY_PROBE_TIMEOUT_MS = 500;

function hasConfiguredApplicationHttpUrl(env) {
  return Boolean(
    String(env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL ?? env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL ?? '').trim(),
  );
}

async function probeDevGatewayHealth(baseUrl) {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), DEV_GATEWAY_PROBE_TIMEOUT_MS);
  try {
    const response = await fetch(`${baseUrl.replace(/\/+$/u, '')}/healthz`, {
      method: 'GET',
      cache: 'no-store',
      signal: controller.signal,
    });
    return response.ok;
  } catch {
    return false;
  } finally {
    clearTimeout(timeoutId);
  }
}

export async function discoverRunningSdkworkImGatewayHttpUrl({
  host = DEFAULT_DEV_GATEWAY_HOST,
  startPort = DEFAULT_DEV_GATEWAY_START_PORT,
  maxAttempts = DEFAULT_DEV_GATEWAY_PORT_ATTEMPTS,
} = {}) {
  for (let offset = 0; offset < maxAttempts; offset += 1) {
    const port = startPort + offset;
    const candidate = `http://${host}:${port}`;
    if (await probeDevGatewayHealth(candidate)) {
      return candidate;
    }
  }
  return undefined;
}

function applyTopologyProfileDefaults(env) {
  try {
    const profile = loadProfile(DEFAULT_DEV_PROFILE_ID);
    return applyProfileEnv({ ...env }, profile);
  } catch {
    return { ...env };
  }
}

export async function resolveSdkworkImPcViteDevEnv(env = process.env) {
  let mergedEnv = applyTopologyProfileDefaults(env);

  if (!hasConfiguredApplicationHttpUrl(mergedEnv)) {
    const discoveredGateway = await discoverRunningSdkworkImGatewayHttpUrl();
    if (discoveredGateway) {
      mergedEnv = {
        ...mergedEnv,
        SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: discoveredGateway,
        VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: discoveredGateway,
      };
      if (isStandaloneUnifiedProcess(mergedEnv)) {
        mergedEnv = {
          ...mergedEnv,
          SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: discoveredGateway,
          VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: discoveredGateway,
          SDKWORK_IAM_APP_API_BASE_URL: discoveredGateway,
          VITE_SDKWORK_IAM_APP_API_BASE_URL: discoveredGateway,
        };
      }
    }
  }

  const applicationPublicHttpUrl = resolveApplicationPublicHttpUrl(mergedEnv);
  const applicationPublicWebSocketUrl = deriveWebSocketBaseUrlFromHttpBaseUrl(applicationPublicHttpUrl);
  const platformApiGatewayBaseUrl = resolveSdkworkApiGatewayBaseUrl({
    ...mergedEnv,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
  });
  const rendererInputEnv = {
    ...mergedEnv,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
  };
  const resolvedRendererEnv = resolveSdkworkChatIamCommandEnv({
    env: rendererInputEnv,
    iamMode: 'desktop-local',
    target: 'browser-dev',
  });
  if (resolvedRendererEnv.errors.length > 0) {
    throw new Error(resolvedRendererEnv.errors.join('\n'));
  }
  return resolvedRendererEnv.env;
}
