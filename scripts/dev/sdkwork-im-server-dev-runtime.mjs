import net from 'node:net';
import path from 'node:path';

const DEFAULT_SERVER_HOST = '127.0.0.1';
const DEFAULT_SERVER_PORT = 18079;
const DEFAULT_MAX_SERVER_PORT_ATTEMPTS = 50;
const DEFAULT_RESERVED_SERVER_PORTS = new Set([
  18080, // session-gateway when started as a separate internal upstream
  18081, // governance-service default bind (see governance-service-bin)
]);
const APPLICATION_PUBLIC_INGRESS_BIND_ENV = 'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND';
const APPLICATION_PUBLIC_HTTP_URL_ENV = 'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL';
const APPLICATION_PUBLIC_WEBSOCKET_URL_ENV = 'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL';
const VITE_APPLICATION_PUBLIC_HTTP_URL_ENV = 'VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL';
const VITE_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV = 'VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function normalizePort(value, label = 'port') {
  const normalized = normalizeText(value);
  if (!normalized || !/^\d+$/u.test(normalized)) {
    throw new Error(`${label} must be a TCP port number`);
  }
  const port = Number.parseInt(normalized, 10);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

function parseBindAddr(value, label = APPLICATION_PUBLIC_INGRESS_BIND_ENV) {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }

  const lastColonIndex = normalized.lastIndexOf(':');
  if (lastColonIndex <= 0 || lastColonIndex === normalized.length - 1) {
    throw new Error(`${label} must use host:port, got ${normalized}`);
  }

  const host = normalized.slice(0, lastColonIndex).replace(/^\[|\]$/gu, '');
  return {
    host: normalizeText(host) ?? DEFAULT_SERVER_HOST,
    port: normalizePort(normalized.slice(lastColonIndex + 1), label),
  };
}

function isReservedPort(reservedPorts, port) {
  if (!reservedPorts) {
    return false;
  }
  if (typeof reservedPorts.has === 'function') {
    return reservedPorts.has(port);
  }
  if (Array.isArray(reservedPorts)) {
    return reservedPorts.includes(port);
  }
  return false;
}

export function isTcpPortAvailable(port, host = DEFAULT_SERVER_HOST) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.unref();
    server.once('error', () => resolve(false));
    server.listen({ host, port }, () => {
      server.close(() => resolve(true));
    });
  });
}

export function createSdkworkImServerCargoEnv({
  env = process.env,
  repoRoot,
} = {}) {
  if (!repoRoot) {
    throw new Error('repoRoot is required for sdkwork-im server cargo env resolution');
  }

  const explicitTargetDir = normalizeText(env.CARGO_TARGET_DIR);
  const cargoTargetDir = explicitTargetDir
    ? path.resolve(repoRoot, explicitTargetDir)
    : path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-im-server-dev');

  return {
    env: {
      ...env,
      CARGO_TARGET_DIR: cargoTargetDir,
    },
    usingDefaultTargetDir: !explicitTargetDir,
  };
}

function createBindEnvResult(env, host, port, requestedPort) {
  const bindAddr = `${host}:${port}`;
  const httpUrl = `http://${bindAddr}`;
  const websocketUrl = `ws://${bindAddr}`;
  return {
    bindAddr,
    env: {
      ...env,
      [APPLICATION_PUBLIC_INGRESS_BIND_ENV]: bindAddr,
      [APPLICATION_PUBLIC_HTTP_URL_ENV]: httpUrl,
      [APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]: websocketUrl,
      [VITE_APPLICATION_PUBLIC_HTTP_URL_ENV]: httpUrl,
      [VITE_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]: websocketUrl,
    },
    portChanged: port !== requestedPort,
  };
}

export async function resolveSdkworkImServerBindEnv({
  env = process.env,
  isPortAvailable = isTcpPortAvailable,
  maxAttempts = DEFAULT_MAX_SERVER_PORT_ATTEMPTS,
  reservedPorts = DEFAULT_RESERVED_SERVER_PORTS,
} = {}) {
  const explicitBind = parseBindAddr(env[APPLICATION_PUBLIC_INGRESS_BIND_ENV]);
  const host = explicitBind?.host ?? DEFAULT_SERVER_HOST;
  const startPort = explicitBind?.port ?? DEFAULT_SERVER_PORT;
  const requestedPort = startPort;

  for (let offset = 0; offset < maxAttempts; offset += 1) {
    const candidatePort = startPort + offset;
    if (candidatePort > 65535) {
      break;
    }
    if (isReservedPort(reservedPorts, candidatePort)) {
      continue;
    }
    if (await isPortAvailable(candidatePort, host)) {
      return createBindEnvResult(env, host, candidatePort, requestedPort);
    }
  }

  throw new Error(
    `No available sdkwork-im server port found from ${startPort} after ${maxAttempts} attempts`,
  );
}
