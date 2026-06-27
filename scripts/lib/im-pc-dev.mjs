#!/usr/bin/env node

import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs';
import { ensurePostgresDevDatabaseReady } from '../dev/ensure-postgres-dev-database.mjs';
import { resolvePostgresDevProfile } from '../dev/sdkwork-im-postgres-dev-profile.mjs';
import { mergeSdkworkImBootstrapAccessTokenEnv } from '../dev/sdkwork-im-bootstrap-access-token.mjs';
import { resolveSdkworkImSharedDatabaseConfig } from '../dev/sdkwork-im-shared-database.mjs';
import {
  createSdkworkImServerCargoEnv,
  resolveSdkworkImServerBindEnv,
} from '../dev/sdkwork-im-server-dev-runtime.mjs';
import {
  IAM_APPLICATION_BOOTSTRAP_ENV,
  resolveIamDevEnv,
  resolveStandaloneGatewayConfigPath,
} from './im-topology.mjs';
import { resolveImProductSiteDirEnv } from './im-product-site-dirs.mjs';
import { resolveRealtimeClusterDevEnv } from './im-realtime-cluster-dev.mjs';
import {
  COMMERCE_T1_APP_API_AUTHORITIES,
  COMMERCE_T1_SPLIT_OVERRIDE_ENV_KEY_GROUPS,
} from '../dev/commerce-t1-capabilities.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..');
export const SDKWORK_IM_PC_DEV_HOST_ENV = 'SDKWORK_IM_PC_DEV_HOST';
export const SDKWORK_IM_PC_DEV_PORT_ENV = 'SDKWORK_IM_PC_DEV_PORT';
export const DEFAULT_SDKWORK_IM_PC_DEV_HOST = '127.0.0.1';
export const DEFAULT_SDKWORK_IM_PC_DEV_PORT = 4176;
export const DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BIND = '127.0.0.1:3900';
export const DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BASE_URL = `http://${DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BIND}`;
const MAX_DEV_PORT_ATTEMPTS = 50;
const SDKWORK_API_CLOUD_GATEWAY_BASE_URL_ENV_KEYS = [
  'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
  'SDKWORK_API_CLOUD_GATEWAY_BASE_URL',
];
const DRIVE_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_DRIVE_APP_API_UPSTREAM',
  'SDKWORK_DRIVE_APP_API_UPSTREAM',
  'SDKWORK_DRIVE_APP_API_BASE_URL',
];
const NOTARY_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_NOTARY_APP_API_UPSTREAM',
  'SDKWORK_NOTARY_APP_API_UPSTREAM',
  'SDKWORK_NOTARY_APP_API_BASE_URL',
];
const COMMERCE_T1_APP_API_UPSTREAM_ENV_KEYS = Object.freeze(
  Object.fromEntries(
    COMMERCE_T1_APP_API_AUTHORITIES.map((authority, index) => [
      authority,
      Object.freeze([...COMMERCE_T1_SPLIT_OVERRIDE_ENV_KEY_GROUPS[index]]),
    ]),
  ),
);
const MAIL_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_MAIL_APP_API_UPSTREAM',
  'SDKWORK_MAIL_APP_API_UPSTREAM',
  'SDKWORK_MAIL_APP_API_BASE_URL',
];
const COMMUNITY_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM',
  'SDKWORK_COMMUNITY_APP_API_UPSTREAM',
  'SDKWORK_COMMUNITY_APP_API_BASE_URL',
];
const COURSE_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_COURSE_APP_API_UPSTREAM',
  'SDKWORK_COURSE_APP_API_UPSTREAM',
  'SDKWORK_COURSE_APP_API_BASE_URL',
];
const KNOWLEDGEBASE_APP_API_UPSTREAM_ENV_KEYS = [
  'SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM',
  'SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM',
  'SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL',
];
const SDKWORK_API_CLOUD_GATEWAY_AUTOSTART_ENV_KEYS = [
  'SDKWORK_IM_PLATFORM_API_GATEWAY_AUTOSTART',
  'SDKWORK_API_CLOUD_GATEWAY_AUTOSTART',
];

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

function normalizeGatewayBind(value, label = 'SDKWORK_API_CLOUD_GATEWAY_BIND') {
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

export function resolveSdkworkApiGatewayBind(env = process.env) {
  return normalizeGatewayBind(env.SDKWORK_API_CLOUD_GATEWAY_BIND) ?? DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BIND;
}

export function resolveDeploymentProfile(env = process.env) {
  const explicit = normalizeText(env.SDKWORK_IM_DEPLOYMENT_PROFILE);
  if (explicit === 'standalone' || explicit === 'cloud') {
    return explicit;
  }
  const retiredHosting = normalizeText(env.SDKWORK_IM_HOSTING);
  if (retiredHosting === 'self-hosted') {
    return 'standalone';
  }
  if (retiredHosting === 'cloud-hosted') {
    return 'cloud';
  }
  return 'standalone';
}

export function resolveServiceLayout(env = process.env) {
  return normalizeText(env.SDKWORK_IM_SERVICE_LAYOUT) ?? 'unified-process';
}

export function isStandaloneUnifiedProcess(env = process.env) {
  return resolveDeploymentProfile(env) === 'standalone'
    && resolveServiceLayout(env) === 'unified-process';
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

export function resolveSdkworkApiGatewayBaseUrl(env = process.env) {
  if (isStandaloneUnifiedProcess(env)) {
    return resolveApplicationPublicHttpUrl(env);
  }
  for (const key of SDKWORK_API_CLOUD_GATEWAY_BASE_URL_ENV_KEYS) {
    const baseUrl = normalizeUpstreamBaseUrl(env[key], key);
    if (baseUrl) {
      return baseUrl;
    }
  }
  return `http://${resolveSdkworkApiGatewayBind(env)}`;
}

export function resolveDriveAppApiUpstream(env = process.env) {
  for (const key of DRIVE_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveNotaryAppApiUpstream(env = process.env) {
  for (const key of NOTARY_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveCommerceT1AppApiUpstream(authority, env = process.env) {
  const keys = COMMERCE_T1_APP_API_UPSTREAM_ENV_KEYS[authority] ?? [];
  for (const key of keys) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveCatalogAppApiUpstream(env = process.env) {
  return resolveCommerceT1AppApiUpstream('sdkwork-catalog-app-api', env);
}

export function resolveOrderAppApiUpstream(env = process.env) {
  return resolveCommerceT1AppApiUpstream('sdkwork-order-app-api', env);
}

export function resolveShopAppApiUpstream(env = process.env) {
  return resolveCommerceT1AppApiUpstream('sdkwork-shop-app-api', env);
}

export function resolveMailAppApiUpstream(env = process.env) {
  for (const key of MAIL_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveCommunityAppApiUpstream(env = process.env) {
  for (const key of COMMUNITY_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveCourseAppApiUpstream(env = process.env) {
  for (const key of COURSE_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

export function resolveKnowledgebaseAppApiUpstream(env = process.env) {
  for (const key of KNOWLEDGEBASE_APP_API_UPSTREAM_ENV_KEYS) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return resolveSdkworkApiGatewayBaseUrl(env);
}

function resolveExplicitAppApiUpstream(env, keys) {
  for (const key of keys) {
    const upstream = normalizeUpstreamBaseUrl(env[key], key);
    if (upstream) {
      return upstream;
    }
  }
  return undefined;
}

function shouldAutostartSdkworkApiGateway(env) {
  for (const key of SDKWORK_API_CLOUD_GATEWAY_AUTOSTART_ENV_KEYS) {
    const value = normalizeText(env[key]);
    if (!value) {
      continue;
    }
    return !['0', 'false', 'off', 'no'].includes(value.toLowerCase());
  }
  return true;
}

export function isSdkworkApiGatewayManagedExternally(env = process.env) {
  const managedExternally = normalizeText(env.SDKWORK_IM_PLATFORM_API_GATEWAY_MANAGED_EXTERNALLY);
  return Boolean(managedExternally)
    && !['0', 'false', 'off', 'no'].includes(managedExternally.toLowerCase());
}

export function createStandaloneGatewayProcess({
  env,
  repoRoot: resolvedRepoRoot,
  gatewayWillStart = true,
}) {
  if (!gatewayWillStart || !shouldAutostartSdkworkApiGateway(env)) {
    return undefined;
  }

  const configPath = resolveStandaloneGatewayConfigPath(env, resolvedRepoRoot);
  const iamDevEnv = resolveIamDevEnv(env, resolvedRepoRoot);
  const gatewayEnv = {
    ...iamDevEnv,
    ...env,
    ...IAM_APPLICATION_BOOTSTRAP_ENV,
    ...resolveRealtimeClusterDevEnv({ ...iamDevEnv, ...env }),
    SDKWORK_IM_STANDALONE_GATEWAY_CONFIG: configPath,
    SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT:
      normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT) ?? 'development',
    CARGO_TARGET_DIR: normalizeText(env.SDKWORK_IM_STANDALONE_GATEWAY_CARGO_TARGET_DIR)
      ?? path.join(resolvedRepoRoot, '.runtime', 'cargo-target', 'sdkwork-im-standalone-gateway-dev'),
  };

  return {
    args: [
      'run',
      '-p',
      'sdkwork-im-standalone-gateway',
      '--bin',
      'sdkwork-im-standalone-gateway',
      '--',
      '--config',
      configPath,
    ],
    command: cargoCommand(),
    cwd: resolvedRepoRoot,
    env: gatewayEnv,
    label: 'sdkwork-im-standalone-gateway',
    shell: false,
  };
}

export function createManagedSdkworkApiGatewayProcess({
  env,
  repoRoot: resolvedRepoRoot,
}) {
  if (!shouldAutostartSdkworkApiGateway(env)) {
    return undefined;
  }

  const apiGatewayWorkspaceRoot = path.resolve(resolvedRepoRoot, '..', 'sdkwork-api-cloud-gateway');
  const iamDevEnv = resolveIamDevEnv(env, resolvedRepoRoot);
  const gatewayEnv = {
    ...iamDevEnv,
    ...env,
    ...IAM_APPLICATION_BOOTSTRAP_ENV,
    CARGO_TARGET_DIR: normalizeText(env.SDKWORK_API_CLOUD_GATEWAY_CARGO_TARGET_DIR)
      ?? path.join(apiGatewayWorkspaceRoot, 'target', 'chat-pc-dev'),
    SDKWORK_API_CLOUD_GATEWAY_BIND: resolveSdkworkApiGatewayBind(env),
  };
  const gatewayMode = normalizeText(env.SDKWORK_API_CLOUD_GATEWAY_MODE);
  if (gatewayMode) {
    gatewayEnv.SDKWORK_API_CLOUD_GATEWAY_MODE = gatewayMode;
  }

  return {
    args: [
      'run',
      '-p',
      'sdkwork-api-cloud-gateway',
      '--bin',
      'sdkwork-api-cloud-gateway',
      '--',
      '--config',
      'configs/sdkwork-api-cloud-gateway.development.toml.example',
    ],
    command: cargoCommand(),
    cwd: apiGatewayWorkspaceRoot,
    env: gatewayEnv,
    label: 'sdkwork-api-cloud-gateway',
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
  port = DEFAULT_SDKWORK_IM_PC_DEV_PORT,
} = {}) {
  const resolvedPort = normalizePort(port, SDKWORK_IM_PC_DEV_PORT_ENV);
  const originHosts = [host, 'localhost']
    .map((value) => normalizeText(value))
    .filter((value, index, values) => value && values.indexOf(value) === index);
  return originHosts
    .map((originHost) => `http://${originHost}:${resolvedPort}`)
    .join(',');
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
    if (token === '--database') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--database requires postgres or sqlite');
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
  if (!['postgres', 'postgresql', 'sqlite'].includes(options.database)) {
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
  const databaseProfile = options.database === 'postgresql'
    ? 'postgres'
    : options.database ?? defaultDatabaseProfile;
  const defaultEnvFile = databaseProfile === 'postgres'
    ? resolveDefaultPostgresEnvFile(resolvedRepoRoot)
    : undefined;
  const postgresDevProfile = databaseProfile === 'postgres'
    ? resolvePostgresDevProfile({
      env: {
        ...env,
        ...serverEnv,
      },
      repoRoot: resolvedRepoRoot,
    })
    : undefined;
  const devEnvFile = databaseProfile === 'postgres'
    ? postgresDevProfile.fileEnv
    : loadSdkworkChatPcDevEnvFile(options.envFile ?? defaultEnvFile, {
      repoRoot: resolvedRepoRoot,
    });
  const requestedEnv = {
    ...env,
    ...devEnvFile,
    ...(postgresDevProfile?.env ?? {}),
  };
  const cargoEnv = createSdkworkImServerCargoEnv({
    env: {
      ...requestedEnv,
      ...serverEnv,
    },
    repoRoot: resolvedRepoRoot,
  });
  const mergedEnv = {
    ...cargoEnv.env,
  };
  if (databaseProfile === 'sqlite') {
    delete mergedEnv.SDKWORK_IM_DATABASE_ENGINE;
    delete mergedEnv.SDKWORK_IM_DATABASE_HOST;
    delete mergedEnv.SDKWORK_IM_DATABASE_PORT;
    delete mergedEnv.SDKWORK_IM_DATABASE_NAME;
    delete mergedEnv.SDKWORK_IM_DATABASE_SCHEMA;
    delete mergedEnv.SDKWORK_IM_DATABASE_USERNAME;
    delete mergedEnv.SDKWORK_IM_DATABASE_PASSWORD;
    delete mergedEnv.SDKWORK_IM_DATABASE_SSL_MODE;
    delete mergedEnv.SDKWORK_IM_DATABASE_URL;
    delete mergedEnv.SDKWORK_IM_DATABASE_MAX_CONNECTIONS;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PROVIDER;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_HOST;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PORT;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_NAME;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_SCHEMA;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_USERNAME;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PASSWORD;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_SSLMODE;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_URL;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS;
    mergedEnv.SDKWORK_IM_DEPLOYMENT_MODE = 'desktop';
    mergedEnv.SDKWORK_IM_DATABASE_ENGINE = 'sqlite';
  }
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
  const platformApiGatewayBaseUrl = resolveSdkworkApiGatewayBaseUrl({
    ...mergedEnv,
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
  });
  const standaloneUnified = isStandaloneUnifiedProcess(mergedEnv);
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
  const explicitDriveAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, DRIVE_APP_API_UPSTREAM_ENV_KEYS);
  const explicitNotaryAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, NOTARY_APP_API_UPSTREAM_ENV_KEYS);
  const explicitCommerceT1AppApiUpstreams = standaloneUnified
    ? {}
    : Object.fromEntries(
      COMMERCE_T1_APP_API_AUTHORITIES.flatMap((authority) => {
        const keys = COMMERCE_T1_APP_API_UPSTREAM_ENV_KEYS[authority] ?? [];
        const upstream = resolveExplicitAppApiUpstream(mergedEnv, keys);
        if (!upstream) {
          return [];
        }
        const imKey = keys.find((key) => key.startsWith('SDKWORK_IM_'));
        return imKey ? [[imKey, upstream]] : [];
      }),
    );
  const explicitMailAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, MAIL_APP_API_UPSTREAM_ENV_KEYS);
  const explicitCommunityAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, COMMUNITY_APP_API_UPSTREAM_ENV_KEYS);
  const explicitCourseAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, COURSE_APP_API_UPSTREAM_ENV_KEYS);
  const explicitKnowledgebaseAppApiUpstream = standaloneUnified
    ? undefined
    : resolveExplicitAppApiUpstream(mergedEnv, KNOWLEDGEBASE_APP_API_UPSTREAM_ENV_KEYS);
  const sharedDatabaseEnv = resolveSdkworkImSharedDatabaseConfig({
    env: mergedEnv,
    repoRoot: resolvedRepoRoot,
  }).env;
  const iamDevEnv = resolveIamDevEnv({ ...mergedEnv, ...sharedDatabaseEnv }, resolvedRepoRoot);
  const gatewayServerEnv = {
    ...mergedEnv,
    ...sharedDatabaseEnv,
    ...iamDevEnv,
    SDKWORK_IM_BROWSER_ORIGINS: mergedEnv.SDKWORK_IM_BROWSER_ORIGINS
      ?? createSdkworkChatBrowserOrigins(devServer),
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: applicationPublicHttpUrl,
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: applicationPublicWebSocketUrl,
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: platformApiGatewayBaseUrl,
    SDKWORK_API_CLOUD_GATEWAY_BIND: standaloneUnified
      ? normalizeGatewayBind(
        mergedEnv.SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND,
        'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND',
      ) ?? resolveSdkworkApiGatewayBind(mergedEnv)
      : resolveSdkworkApiGatewayBind(mergedEnv),
    ...(explicitDriveAppApiUpstream
      ? { SDKWORK_IM_DRIVE_APP_API_UPSTREAM: explicitDriveAppApiUpstream }
      : {}),
    ...(explicitNotaryAppApiUpstream
      ? { SDKWORK_IM_NOTARY_APP_API_UPSTREAM: explicitNotaryAppApiUpstream }
      : {}),
    ...explicitCommerceT1AppApiUpstreams,
    ...(explicitMailAppApiUpstream
      ? { SDKWORK_IM_MAIL_APP_API_UPSTREAM: explicitMailAppApiUpstream }
      : {}),
    ...(explicitCommunityAppApiUpstream
      ? { SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM: explicitCommunityAppApiUpstream }
      : {}),
    ...(explicitCourseAppApiUpstream
      ? { SDKWORK_IM_COURSE_APP_API_UPSTREAM: explicitCourseAppApiUpstream }
      : {}),
    ...(explicitKnowledgebaseAppApiUpstream
      ? { SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM: explicitKnowledgebaseAppApiUpstream }
      : {}),
  };
  if (standaloneUnified) {
    for (const authority of COMMERCE_T1_APP_API_AUTHORITIES) {
      for (const key of COMMERCE_T1_APP_API_UPSTREAM_ENV_KEYS[authority] ?? []) {
        if (key.startsWith('SDKWORK_IM_')) {
          delete gatewayServerEnv[key];
        }
      }
    }
    for (const key of [
      'SDKWORK_IM_DRIVE_APP_API_UPSTREAM',
      'SDKWORK_IM_NOTARY_APP_API_UPSTREAM',
      'SDKWORK_IM_MAIL_APP_API_UPSTREAM',
      'SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM',
      'SDKWORK_IM_COURSE_APP_API_UPSTREAM',
      'SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM',
    ]) {
      delete gatewayServerEnv[key];
    }
  }
  const managedStandaloneGatewayProcess = standaloneUnified
    ? createStandaloneGatewayProcess({
      env: gatewayServerEnv,
      repoRoot: resolvedRepoRoot,
    })
    : undefined;
  const managedSdkworkApiGatewayProcess = standaloneUnified
    ? undefined
    : createManagedSdkworkApiGatewayProcess({
      env: mergedEnv,
      repoRoot: resolvedRepoRoot,
    });
  const processes = [];
  if (standaloneUnified) {
    if (managedStandaloneGatewayProcess) {
      processes.push(managedStandaloneGatewayProcess);
    }
  } else {
    processes.push({
      ...shared,
      args: ['dev:server'],
      env: managedSdkworkApiGatewayProcess
        ? {
            ...gatewayServerEnv,
            SDKWORK_IM_PLATFORM_API_GATEWAY_MANAGED_EXTERNALLY: 'true',
          }
        : gatewayServerEnv,
      label: 'sdkwork-im-server',
    });
  }
  processes.push({
    ...shared,
    env: rendererEnv,
    args: target.pnpmArgs,
    label: target.label,
  });
  if (managedSdkworkApiGatewayProcess) {
    processes.push(managedSdkworkApiGatewayProcess);
  }
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
  resolveServerBindEnv = resolveSdkworkImServerBindEnv,
  spawnImpl = spawn,
  stdout = process.stdout,
  stderr = process.stderr,
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
  const resolvedServerBind = serverBindGateway?.label === 'sdkwork-im-server'
    || serverBindGateway?.label === 'sdkwork-im-standalone-gateway'
    ? await resolveServerBindEnv({
      env: serverBindGateway.env,
    })
    : { env: {} };
  const plan = createSdkworkChatPcDevPlan({
    argv,
    devServerHost: initialPlan.devServer.host,
    devServerPort: resolvedDevPort,
    env: envWithSiteDirs,
    repoRoot: resolvedRepoRoot,
    serverEnv: resolvedServerBind.env,
  });
  if (plan.dryRun) {
    stdout.write(`${formatPlan(plan)}\n`);
    return 0;
  }

  const gatewayProcess = plan.processes.find((entry) => (
    entry.label === 'sdkwork-im-server' || entry.label === 'sdkwork-im-standalone-gateway'
  ));
  if (gatewayProcess) {
    await ensurePostgresDevDatabaseReady({
      env: gatewayProcess.env,
      repoRoot: resolvedRepoRoot,
      stdout,
      stderr,
    });
  }

  const children = [];
  let shuttingDown = false;

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

  for (const entry of plan.processes) {
    const child = spawnImpl(entry.command, entry.args, {
      cwd: entry.cwd,
      env: entry.env,
      shell: entry.shell,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    children.push(child);

    child.stdout?.on('data', (chunk) => prefixOutput(entry.label, stdout, chunk));
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
