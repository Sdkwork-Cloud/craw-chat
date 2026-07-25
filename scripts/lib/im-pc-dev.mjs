#!/usr/bin/env node

import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  formatResolvedNetworkAccessLines,
  formatNetworkUrlHost,
  resolveNetworkInterfaceSnapshot,
  resolveNonLoopbackIpAddresses,
} from '@sdkwork/app-topology/network-access';

import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs';
import { ensurePostgresDevDatabaseReady } from '../dev/ensure-postgres-dev-database.mjs';
import { terminateStaleDevGatewayProcesses } from '../dev/terminate-stale-dev-gateway-processes.mjs';
import { resolvePostgresDevProfile } from '../dev/sdkwork-im-postgres-dev-profile.mjs';
import { mergeSdkworkImBootstrapAccessTokenEnv } from '../dev/sdkwork-im-bootstrap-access-token.mjs';
import { resolveSdkworkImSharedDatabaseConfig } from '../dev/sdkwork-im-shared-database.mjs';
import {
  createStandaloneGatewayCargoEnv,
  resolveStandaloneGatewayBindEnv,
} from '../dev/sdkwork-api-im-standalone-gateway-dev-runtime.mjs';
import {
  IAM_APPLICATION_BOOTSTRAP_ENV,
  resolveIamDevEnv,
} from './im-topology.mjs';
import { resolveImProductSiteDirEnv } from './im-product-site-dirs.mjs';
import { resolveRealtimeClusterDevEnv } from './im-realtime-cluster-dev.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..');
export const SDKWORK_IM_PC_DEV_HOST_ENV = 'SDKWORK_IM_PC_DEV_HOST';
export const SDKWORK_IM_PC_DEV_PORT_ENV = 'SDKWORK_IM_PC_DEV_PORT';
export const DEFAULT_SDKWORK_IM_PC_DEV_HOST = '0.0.0.0';
export const DEFAULT_SDKWORK_IM_PC_DEV_PORT = 4176;
const MAX_DEV_PORT_ATTEMPTS = 50;

const TARGETS = Object.freeze({
  browser: {
    label: 'sdkwork-im-pc-browser',
    pnpmArgs: ['--dir', 'apps/sdkwork-im-pc', 'dev'],
  },
  desktop: {
    label: 'sdkwork-im-pc-desktop',
    pnpmArgs: ['--dir', 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop', 'dev:desktop'],
  },
});

function pnpmCommand() {
  return process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

function pnpmShell() {
  return process.platform === 'win32';
}

function cargoCommand() {
  return process.platform === 'win32' ? 'cargo.exe' : 'cargo';
}

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function normalizeUpstreamBaseUrl(value, label) {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }
  let parsedUrl;
  try {
    parsedUrl = new URL(normalized);
  } catch {
    throw new Error(`${label} must be a valid absolute http(s) URL`);
  }
  if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
    throw new Error(`${label} must be a valid absolute http(s) URL`);
  }
  return normalized.replace(/\/+$/u, '');
}

function normalizeGatewayBind(value, label = 'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND') {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }
  if (normalized.startsWith('http://') || normalized.startsWith('https://')) {
    throw new Error(`${label} must be a host:port bind address, not a URL`);
  }
  return normalized;
}

export function deriveWebSocketBaseUrlFromHttpBaseUrl(httpBaseUrl) {
  const normalized = normalizeText(httpBaseUrl);
  if (!normalized) {
    return undefined;
  }
  const parsedUrl = new URL(normalized);
  if (parsedUrl.protocol === 'http:') {
    parsedUrl.protocol = 'ws:';
  } else if (parsedUrl.protocol === 'https:') {
    parsedUrl.protocol = 'wss:';
  } else {
    throw new Error(`cannot derive websocket URL from non-http base URL: ${normalized}`);
  }
  return parsedUrl.toString().replace(/\/+$/u, '');
}

export function resolveDeploymentProfile(env = process.env) {
  const explicit = normalizeText(env.SDKWORK_IM_DEPLOYMENT_PROFILE);
  if (explicit === 'standalone' || explicit === 'cloud') {
    return explicit;
  }
  return 'standalone';
}

export function isStandaloneSingleIngress(env = process.env) {
  return resolveDeploymentProfile(env) === 'standalone';
}

export function resolveApplicationPublicHttpUrl(env = process.env) {
  const explicit = normalizeUpstreamBaseUrl(
    env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',
  );
  if (explicit) {
    return explicit;
  }
  const bind = normalizeGatewayBind(
    env.SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND,
    'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND',
  );
  if (bind) {
    return `http://${bind}`;
  }
  return 'http://127.0.0.1:18079';
}

export function resolvePlatformApiGatewayBaseUrl(env = process.env) {
  if (isStandaloneSingleIngress(env)) {
    return resolveApplicationPublicHttpUrl(env);
  }
  const baseUrl = normalizeUpstreamBaseUrl(
    env.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
    'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
  );
  if (!baseUrl) {
    throw new Error(
      'cloud development requires SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
    );
  }
  return baseUrl;
}

export function createStandaloneGatewayProcess({
  env,
  repoRoot: resolvedRepoRoot,
  gatewayWillStart = true,
}) {
  if (!gatewayWillStart) {
    return undefined;
  }

  const iamDevEnv = resolveIamDevEnv(env, resolvedRepoRoot);
  const gatewayEnv = {
    ...iamDevEnv,
    ...env,
    ...IAM_APPLICATION_BOOTSTRAP_ENV,
    ...resolveRealtimeClusterDevEnv({ ...iamDevEnv, ...env }),
    SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT:
      normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT) ?? 'development',
    CARGO_TARGET_DIR: normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_CARGO_TARGET_DIR)
      ?? path.join(resolvedRepoRoot, '.runtime', 'cargo-target', 'sdkwork-api-im-standalone-gateway-dev'),
  };

  return {
    args: [path.join(resolvedRepoRoot, 'scripts/dev/run-standalone-gateway-dev.mjs')],
    command: process.execPath,
    cwd: resolvedRepoRoot,
    env: gatewayEnv,
    label: 'sdkwork-api-im-standalone-gateway',
    shell: false,
  };
}

function normalizePort(value, label = 'port') {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }
  if (!/^\d+$/u.test(normalized)) {
    throw new Error(`${label} must be a TCP port number`);
  }
  const port = Number.parseInt(normalized, 10);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

export function resolveSdkworkChatPcDevServer({
  env = process.env,
  host,
  port,
} = {}) {
  const resolvedHost = normalizeText(host)
    ?? normalizeText(env[SDKWORK_IM_PC_DEV_HOST_ENV])
    ?? DEFAULT_SDKWORK_IM_PC_DEV_HOST;
  const resolvedPort = normalizePort(
    port ?? env[SDKWORK_IM_PC_DEV_PORT_ENV] ?? DEFAULT_SDKWORK_IM_PC_DEV_PORT,
    SDKWORK_IM_PC_DEV_PORT_ENV,
  );
  return {
    host: resolvedHost,
    port: resolvedPort,
    url: `http://${resolvedHost}:${resolvedPort}`,
  };
}

export function createSdkworkChatBrowserOrigins({
  host = DEFAULT_SDKWORK_IM_PC_DEV_HOST,
  networkHosts = [],
  port = DEFAULT_SDKWORK_IM_PC_DEV_PORT,
} = {}) {
  const resolvedPort = normalizePort(port, SDKWORK_IM_PC_DEV_PORT_ENV);
  const originHosts = [
    ...(host === '0.0.0.0' ? ['127.0.0.1'] : host === '::' ? ['::1'] : [host]),
    'localhost',
    ...networkHosts,
  ]
    .map((value) => normalizeText(value))
    .filter((value, index, values) => value && values.indexOf(value) === index);
  return originHosts
    .map((originHost) => `http://${formatNetworkUrlHost(originHost)}:${resolvedPort}`)
    .join(',');
}

function isPrivateIpv4Address(address) {
  const octets = String(address).split('.').map((value) => Number.parseInt(value, 10));
  if (octets.length !== 4 || octets.some((value) => !Number.isInteger(value))) {
    return false;
  }
  return octets[0] === 10
    || (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31)
    || (octets[0] === 192 && octets[1] === 168);
}

function isPrivateIpv6Address(address) {
  const normalized = String(address).split('%', 1)[0].toLowerCase();
  return normalized.startsWith('fc') || normalized.startsWith('fd');
}

export function resolveLocalNetworkHosts({
  networkInterfaces = os.networkInterfaces,
} = {}) {
  return resolveNonLoopbackIpAddresses(networkInterfaces).filter((address) => (
    address.includes(':')
      ? isPrivateIpv6Address(address)
      : isPrivateIpv4Address(address)
  ));
}

export function resolveSdkworkChatAccessNetworkHosts({
  networkInterfaces = os.networkInterfaces,
} = {}) {
  return resolveNonLoopbackIpAddresses(networkInterfaces).filter((address) => (
    !address.includes(':') || isPrivateIpv6Address(address)
  ));
}

export function createSdkworkChatPcAccessUrls({
  networkHosts = resolveLocalNetworkHosts(),
  port = DEFAULT_SDKWORK_IM_PC_DEV_PORT,
} = {}) {
  const resolvedPort = normalizePort(port, SDKWORK_IM_PC_DEV_PORT_ENV);
  return {
    localUrl: `http://localhost:${resolvedPort}`,
    networkUrls: networkHosts.map((host) => `http://${formatNetworkUrlHost(host)}:${resolvedPort}`),
  };
}

export function formatSdkworkChatPcAccessLinks(accessUrls) {
  return [
    '[sdkwork-im] application started successfully',
    ...formatResolvedNetworkAccessLines(accessUrls, {
      prefix: '[sdkwork-im] ',
      unavailableText: 'no private IPv4 LAN address detected',
    }),
  ].join('\n');
}

export async function waitForSdkworkChatApplicationReady({
  baseUrl,
  fetchImpl = globalThis.fetch,
  intervalMs = 500,
  maxAttempts = 600,
  wait = (delayMs) => new Promise((resolve) => setTimeout(resolve, delayMs)),
} = {}) {
  if (!baseUrl || typeof fetchImpl !== 'function') {
    throw new Error('application readiness requires an HTTP base URL and fetch implementation');
  }
  const healthUrl = `${baseUrl.replace(/\/+$/u, '')}/healthz`;
  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    try {
      const response = await fetchImpl(healthUrl, { cache: 'no-store' });
      if (response.ok) {
        return healthUrl;
      }
    } catch {
      // The gateway may still be compiling or binding its socket.
    }
    if (attempt + 1 < maxAttempts) {
      await wait(intervalMs);
    }
  }
  throw new Error(`application gateway did not become ready at ${healthUrl}`);
}

export function isTcpPortAvailable(port, host = DEFAULT_SDKWORK_IM_PC_DEV_HOST) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.unref();
    server.once('error', () => resolve(false));
    server.listen({ host, port }, () => {
      server.close(() => resolve(true));
    });
  });
}

export async function resolveAvailableSdkworkChatPcDevPort({
  env = process.env,
  host,
  startPort,
  maxAttempts = MAX_DEV_PORT_ATTEMPTS,
  isPortAvailable = isTcpPortAvailable,
} = {}) {
  const devServer = resolveSdkworkChatPcDevServer({
    env,
    host,
    port: startPort,
  });
  for (let offset = 0; offset < maxAttempts; offset += 1) {
    const candidatePort = devServer.port + offset;
    if (candidatePort > 65535) {
      break;
    }
    if (await isPortAvailable(candidatePort, devServer.host)) {
      return candidatePort;
    }
  }
  throw new Error(
    `No available Sdkwork IM PC dev port found from ${devServer.port} after ${maxAttempts} attempts`,
  );
}

function stripOptionalQuotes(value) {
  if (
    (value.startsWith('"') && value.endsWith('"'))
    || (value.startsWith("'") && value.endsWith("'"))
  ) {
    return value.slice(1, -1);
  }
  return value;
}

function parseEnvFileContent(content) {
  const values = {};
  for (const [lineIndex, rawLine] of String(content ?? '').split(/\r?\n/u).entries()) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const normalizedLine = line.startsWith('export ') ? line.slice('export '.length).trim() : line;
    const separatorIndex = normalizedLine.indexOf('=');
    if (separatorIndex <= 0) {
      throw new Error(`Invalid env file line ${lineIndex + 1}: ${rawLine}`);
    }
    const name = normalizedLine.slice(0, separatorIndex).trim();
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name)) {
      throw new Error(`Invalid env variable name on line ${lineIndex + 1}: ${name}`);
    }
    const value = stripOptionalQuotes(normalizedLine.slice(separatorIndex + 1).trim());
    values[name] = value;
  }
  return values;
}

function resolveEnvFilePath(envFile, root) {
  const normalized = normalizeText(envFile);
  if (!normalized) {
    return undefined;
  }
  return path.isAbsolute(normalized) ? normalized : path.resolve(root, normalized);
}

function resolveDefaultPostgresEnvFile(root) {
  return path.resolve(root, '.env.postgres');
}

export function loadSdkworkChatPcDevEnvFile(envFile, {
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const envFilePath = resolveEnvFilePath(envFile, resolvedRepoRoot);
  if (!envFilePath) {
    return {};
  }
  if (!fs.existsSync(envFilePath)) {
    throw new Error(`Sdkwork IM PC dev env file does not exist: ${envFilePath}`);
  }
  return parseEnvFileContent(fs.readFileSync(envFilePath, 'utf8'));
}

export function parseSdkworkChatPcDevArgs(argv = []) {
  const options = {
    clientOnly: false,
    database: undefined,
    dryRun: false,
    envFile: undefined,
    target: 'browser',
  };
  const tokens = Array.isArray(argv) ? [...argv] : [];
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token === '--dry-run') {
      options.dryRun = true;
      continue;
    }
    if (token === '--client-only') {
      options.clientOnly = true;
      continue;
    }
    if (token === '--database') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--database requires postgres');
      }
      options.database = value;
      index += 1;
      continue;
    }
    if (token === '--target') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--target requires browser or desktop');
      }
      options.target = value;
      index += 1;
      continue;
    }
    if (token === '--dev-env-file') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--dev-env-file requires a path');
      }
      options.envFile = value;
      index += 1;
      continue;
    }
    throw new Error(`Unknown sdkwork-im-pc dev argument: ${token}`);
  }
  if (!['postgres', 'postgresql'].includes(options.database)) {
    if (options.database === undefined) {
      return options;
    }
    throw new Error(`Unsupported sdkwork-im-pc dev database: ${options.database}`);
  }
  return options;
}

export function createSdkworkChatPcDevPlan({
  argv = [],
  devServerHost,
  devServerPort,
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
  serverEnv = {},
} = {}) {
  const options = parseSdkworkChatPcDevArgs(argv);
  const target = TARGETS[options.target];
  if (!target) {
    throw new Error(`Unsupported sdkwork-im-pc dev target: ${options.target}`);
  }
  const defaultDatabaseProfile = 'postgres';
  const databaseProfile = options.clientOnly ? undefined : defaultDatabaseProfile;
  const defaultEnvFile = databaseProfile === 'postgres'
    ? resolveDefaultPostgresEnvFile(resolvedRepoRoot)
    : undefined;
  const customDevEnvFile = options.envFile
    ? loadSdkworkChatPcDevEnvFile(options.envFile, {
      repoRoot: resolvedRepoRoot,
    })
    : undefined;
  const postgresDevProfile = databaseProfile === 'postgres'
    ? resolvePostgresDevProfile({
      env: {
        ...env,
        ...serverEnv,
      },
      extraEnv: customDevEnvFile ?? {},
      repoRoot: resolvedRepoRoot,
    })
    : undefined;
  const devEnvFile = databaseProfile === 'postgres'
    ? (customDevEnvFile ?? postgresDevProfile.fileEnv)
    : {};
  const requestedEnv = {
    ...env,
    ...devEnvFile,
    ...(postgresDevProfile?.env ?? {}),
  };
  const cargoEnv = options.clientOnly
    ? { env: requestedEnv }
    : createStandaloneGatewayCargoEnv({
        env: {
          ...requestedEnv,
          ...serverEnv,
        },
        repoRoot: resolvedRepoRoot,
      });
  const mergedEnv = {
    ...cargoEnv.env,
  };
  const devServer = resolveSdkworkChatPcDevServer({
    env: mergedEnv,
    host: devServerHost,
    port: devServerPort,
  });
  mergedEnv[SDKWORK_IM_PC_DEV_HOST_ENV] = devServer.host;
  mergedEnv[SDKWORK_IM_PC_DEV_PORT_ENV] = String(devServer.port);
  const applicationPublicHttpUrl = resolveApplicationPublicHttpUrl(mergedEnv);
  const applicationPublicWebSocketUrl = deriveWebSocketBaseUrlFromHttpBaseUrl(
    applicationPublicHttpUrl,
  );
  const platformApiGatewayBaseUrl = resolvePlatformApiGatewayBaseUrl({
    ...mergedEnv,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
  });
  const standaloneSingleIngress = isStandaloneSingleIngress(mergedEnv);
  const rendererInputEnv = {
    ...mergedEnv,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
  };
  const command = pnpmCommand();
  const shared = {
    command,
    cwd: resolvedRepoRoot,
    env: mergedEnv,
    shell: pnpmShell(),
  };
  const resolvedRendererEnv = resolveSdkworkChatIamCommandEnv({
    env: rendererInputEnv,
    iamMode: 'desktop-local',
    target: 'desktop-dev',
  });
  if (resolvedRendererEnv.errors.length > 0) {
    throw new Error(resolvedRendererEnv.errors.join('\n'));
  }
  const rendererEnv = mergeSdkworkImBootstrapAccessTokenEnv(resolvedRendererEnv.env);
  const rendererProcess = {
    ...shared,
    env: rendererEnv,
    args: target.pnpmArgs,
    label: target.label,
  };
  if (options.clientOnly) {
    return {
      devServer,
      dryRun: options.dryRun,
      target: options.target,
      processes: [rendererProcess],
    };
  }
  const sharedDatabaseEnv = resolveSdkworkImSharedDatabaseConfig({
    env: mergedEnv,
    repoRoot: resolvedRepoRoot,
  }).env;
  const iamDevEnv = resolveIamDevEnv({ ...mergedEnv, ...sharedDatabaseEnv }, resolvedRepoRoot);
  const gatewayServerEnv = {
    ...mergedEnv,
    ...iamDevEnv,
    ...sharedDatabaseEnv,
    SDKWORK_IM_BROWSER_ORIGINS: mergedEnv.SDKWORK_IM_BROWSER_ORIGINS
      ?? createSdkworkChatBrowserOrigins(devServer),
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
  };
  for (const key of Object.keys(gatewayServerEnv)) {
    if (/^SDKWORK_(?:IM_)?[A-Z0-9_]+_APP_API_UPSTREAM$/u.test(key)) {
      delete gatewayServerEnv[key];
    }
  }
  const managedStandaloneGatewayProcess = standaloneSingleIngress
    ? createStandaloneGatewayProcess({
      env: gatewayServerEnv,
      repoRoot: resolvedRepoRoot,
    })
    : undefined;
  const processes = [];
  if (managedStandaloneGatewayProcess) {
    processes.push(managedStandaloneGatewayProcess);
  }
  processes.push(rendererProcess);
  return {
    devServer,
    dryRun: options.dryRun,
    target: options.target,
    processes,
  };
}

function formatPlan(plan) {
  return plan.processes
    .map((entry) => `[${entry.label}] ${entry.command} ${entry.args.join(' ')}`)
    .join('\n');
}

function prefixOutput(label, stream, chunk) {
  const text = String(chunk ?? '');
  for (const line of text.split(/\r?\n/u)) {
    if (line.length > 0) {
      stream.write(`[${label}] ${line}\n`);
    }
  }
}

function terminateProcessTree(child) {
  if (!child?.pid) {
    return;
  }

  if (process.platform === 'win32') {
    spawnSync('taskkill.exe', ['/PID', String(child.pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
    });
    return;
  }

  child.kill();
}

export async function runSdkworkChatPcDev({
  argv = process.argv.slice(2),
  env = process.env,
  findAvailableDevPort = resolveAvailableSdkworkChatPcDevPort,
  repoRoot: resolvedRepoRoot = repoRoot,
  resolveServerBindEnv = resolveStandaloneGatewayBindEnv,
  spawnImpl = spawn,
  stdout = process.stdout,
  stderr = process.stderr,
  networkInterfaces = os.networkInterfaces,
  waitForApplicationReady = waitForSdkworkChatApplicationReady,
  ensureDatabaseReady = ensurePostgresDevDatabaseReady,
} = {}) {
  const siteDirEnv = await resolveImProductSiteDirEnv({
    buildEnv: env,
    env,
    onFallback: ({ fallbackDir, label, sourceDir }) => {
      process.stdout.write(
        `[sdkwork-im-pc-dev] ${label} source not found at ${path.relative(resolvedRepoRoot, sourceDir)}; using ${path.relative(resolvedRepoRoot, fallbackDir)}\n`,
      );
    },
    repoRoot: resolvedRepoRoot,
  });
  const envWithSiteDirs = {
    ...env,
    ...siteDirEnv,
  };
  const initialPlan = createSdkworkChatPcDevPlan({
    argv,
    env: envWithSiteDirs,
    repoRoot: resolvedRepoRoot,
  });
  const resolvedDevPort = await findAvailableDevPort({
    env: initialPlan.processes.at(-1).env,
    host: initialPlan.devServer.host,
    startPort: initialPlan.devServer.port,
  });
  const serverPortPlan = createSdkworkChatPcDevPlan({
    argv,
    devServerHost: initialPlan.devServer.host,
    devServerPort: resolvedDevPort,
    env: envWithSiteDirs,
    repoRoot: resolvedRepoRoot,
  });
  const serverBindGateway = serverPortPlan.processes[0];
  const shouldResolveServerBind =
    serverBindGateway?.label === 'sdkwork-api-im-standalone-gateway';
  if (shouldResolveServerBind) {
    terminateStaleDevGatewayProcesses({ stdout });
  }
  const resolvedServerBind = shouldResolveServerBind
    ? await resolveServerBindEnv({
      env: serverBindGateway.env,
    })
    : { env: {} };
  if (resolvedServerBind.portChanged) {
    stdout.write(
      `[sdkwork-im-pc-dev] 127.0.0.1:18079 is busy; using http://${resolvedServerBind.bindAddr}\n`,
    );
  }
  const interfaces = resolveNetworkInterfaceSnapshot(networkInterfaces);
  const networkHosts = resolveLocalNetworkHosts({ networkInterfaces: interfaces });
  const accessNetworkHosts = resolveSdkworkChatAccessNetworkHosts({
    networkInterfaces: interfaces,
  });
  const runtimeEnv = {
    ...envWithSiteDirs,
    SDKWORK_IM_BROWSER_ORIGINS: envWithSiteDirs.SDKWORK_IM_BROWSER_ORIGINS
      ?? createSdkworkChatBrowserOrigins({
        host: initialPlan.devServer.host,
        networkHosts,
        port: resolvedDevPort,
      }),
  };
  const plan = createSdkworkChatPcDevPlan({
    argv,
    devServerHost: initialPlan.devServer.host,
    devServerPort: resolvedDevPort,
    env: runtimeEnv,
    repoRoot: resolvedRepoRoot,
    serverEnv: {
      ...resolvedServerBind.env,
      SDKWORK_IM_BROWSER_ORIGINS: runtimeEnv.SDKWORK_IM_BROWSER_ORIGINS,
    },
  });
  if (plan.dryRun) {
    stdout.write(`${formatPlan(plan)}\n`);
    return 0;
  }

  const gatewayProcess = plan.processes.find((entry) => (
    entry.label === 'sdkwork-api-im-standalone-gateway'
  ));
  if (gatewayProcess) {
    await ensureDatabaseReady({
      env: gatewayProcess.env,
      repoRoot: resolvedRepoRoot,
      stdout,
      stderr,
    });
  }

  const children = [];
  let shuttingDown = false;
  let accessLinksPrinted = false;
  let gatewayReady = !gatewayProcess;
  let readinessCheckStarted = false;
  let rendererReady = plan.target !== 'browser';
  let gatewayOutput = '';
  let rendererOutput = '';
  const rendererProcess = plan.processes.find((entry) => entry.label === TARGETS.browser.label);
  const applicationBaseUrl = rendererProcess?.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL
    ?? rendererProcess?.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL;
  const accessUrls = createSdkworkChatPcAccessUrls({
    networkHosts: accessNetworkHosts,
    port: resolvedDevPort,
  });

  function shutdown(exceptChild) {
    if (shuttingDown) {
      return;
    }
    shuttingDown = true;
    for (const child of children) {
      if (child !== exceptChild && child.exitCode == null && child.signalCode == null) {
        terminateProcessTree(child);
      }
    }
  }

  function startReadinessCheckIfReady() {
    if (
      plan.target !== 'browser'
      || readinessCheckStarted
      || !rendererReady
      || !gatewayReady
      || shuttingDown
    ) {
      return;
    }
    readinessCheckStarted = true;
    void waitForApplicationReady({ baseUrl: applicationBaseUrl })
      .then(() => {
        if (!accessLinksPrinted && !shuttingDown) {
          accessLinksPrinted = true;
          stdout.write(`${formatSdkworkChatPcAccessLinks(accessUrls)}\n`);
        }
      })
      .catch((error) => {
        stderr.write(
          `[sdkwork-im-pc-dev] ${error instanceof Error ? error.message : String(error)}\n`,
        );
      });
  }

  for (const entry of plan.processes) {
    const child = spawnImpl(entry.command, entry.args, {
      cwd: entry.cwd,
      env: entry.env,
      shell: entry.shell,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    children.push(child);

    child.stdout?.on('data', (chunk) => {
      prefixOutput(entry.label, stdout, chunk);
      if (entry.label === TARGETS.browser.label && !rendererReady) {
        rendererOutput = `${rendererOutput}${String(chunk ?? '')}`.slice(-4_096);
        rendererReady = /Local:\s+http:\/\//u.test(
          rendererOutput.replaceAll(/\u001b\[[0-9;]*m/gu, ''),
        );
      }
      if (entry === gatewayProcess && !gatewayReady) {
        gatewayOutput = `${gatewayOutput}${String(chunk ?? '')}`.slice(-4_096);
        gatewayReady = /Listening on http:\/\//u.test(
          gatewayOutput.replaceAll(/\u001b\[[0-9;]*m/gu, ''),
        );
      }
      startReadinessCheckIfReady();
    });
    child.stderr?.on('data', (chunk) => prefixOutput(entry.label, stderr, chunk));
    child.on('error', (error) => {
      stderr.write(`[${entry.label}] ${error instanceof Error ? error.message : String(error)}\n`);
      shutdown(child);
      process.exitCode = 1;
    });
    child.on('exit', (code, signal) => {
      if (shuttingDown) {
        return;
      }
      shutdown(child);
      if (code && code !== 0) {
        stderr.write(`[${entry.label}] exited with code ${code}\n`);
        process.exitCode = code;
        return;
      }
      if (signal) {
        stderr.write(`[${entry.label}] exited with signal ${signal}\n`);
        process.exitCode = 1;
      }
    });
  }

  const stop = () => shutdown();
  process.once('SIGINT', stop);
  process.once('SIGTERM', stop);
  return undefined;
}
