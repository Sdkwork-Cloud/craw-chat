import { VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL } from './topologyEnvKeys';
import {
  readDiscoveredDevGatewayHttpUrl,
  writeDiscoveredDevGatewayHttpUrl,
} from './devGatewayDiscoveryCache';

const DEFAULT_DEV_GATEWAY_HOST = '127.0.0.1';
const DEFAULT_DEV_GATEWAY_START_PORT = 18079;
const DEFAULT_DEV_GATEWAY_PORT_ATTEMPTS = 20;
const DEV_GATEWAY_PROBE_TIMEOUT_MS = 400;

function isRuntimeDev(): boolean {
  const env = (import.meta.env ?? {}) as { DEV?: boolean | string };
  if (env.DEV === true || env.DEV === 'true') {
    return true;
  }
  if (env.DEV === false || env.DEV === 'false') {
    return false;
  }
  return typeof window !== 'undefined';
}

function readConfiguredApplicationHttpUrl(): string | undefined {
  const value = (import.meta.env ?? {})[VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

async function probeDevGatewayHealth(baseUrl: string): Promise<boolean> {
  const controller = typeof AbortController !== 'undefined' ? new AbortController() : undefined;
  const timeoutId = controller
    ? globalThis.setTimeout(() => controller.abort(), DEV_GATEWAY_PROBE_TIMEOUT_MS)
    : undefined;

  try {
    const response = await fetch(`${baseUrl.replace(/\/+$/u, '')}/healthz`, {
      method: 'GET',
      cache: 'no-store',
      ...(controller ? { signal: controller.signal } : {}),
    });
    return response.ok;
  } catch {
    return false;
  } finally {
    if (timeoutId !== undefined) {
      globalThis.clearTimeout(timeoutId);
    }
  }
}

export async function discoverLocalDevGatewayHttpUrl(): Promise<string | undefined> {
  if (!isRuntimeDev()) {
    return undefined;
  }

  const cached = readDiscoveredDevGatewayHttpUrl();
  if (cached && await probeDevGatewayHealth(cached)) {
    return cached;
  }

  for (let offset = 0; offset < DEFAULT_DEV_GATEWAY_PORT_ATTEMPTS; offset += 1) {
    const port = DEFAULT_DEV_GATEWAY_START_PORT + offset;
    const candidate = `http://${DEFAULT_DEV_GATEWAY_HOST}:${port}`;
    if (await probeDevGatewayHealth(candidate)) {
      writeDiscoveredDevGatewayHttpUrl(candidate);
      return candidate;
    }
  }

  return undefined;
}

export async function bootstrapLocalDevGatewayDiscovery(): Promise<string | undefined> {
  if (!isRuntimeDev() || readConfiguredApplicationHttpUrl()) {
    return undefined;
  }

  return discoverLocalDevGatewayHttpUrl();
}
