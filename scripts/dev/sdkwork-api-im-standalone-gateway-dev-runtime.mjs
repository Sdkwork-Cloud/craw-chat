import net from 'node:net';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const DEFAULT_GATEWAY_HOST = '127.0.0.1';
const DEFAULT_GATEWAY_PORT = 18089;
const DEFAULT_MAX_GATEWAY_PORT_ATTEMPTS = 50;
const DEFAULT_RESERVED_GATEWAY_PORTS = new Set([
  28080, // session-gateway internal runtime when launched independently
  28081, // governance-service internal runtime when launched independently
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
    host: normalizeText(host) ?? DEFAULT_GATEWAY_HOST,
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

function publicUrlHostForBindHost(host) {
  if (host === '0.0.0.0') {
    return '127.0.0.1';
  }
  if (host === '::') {
    return '[::1]';
  }
  return host.includes(':') ? `[${host}]` : host;
}

export function isTcpPortAvailable(port, host = DEFAULT_GATEWAY_HOST) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.unref();
    server.once('error', () => resolve(false));
    server.listen({ host, port }, () => {
      server.close(() => resolve(true));
    });
  });
}

function probeTcpConnect(host, port, timeoutMs = 400) {
  return new Promise((resolve) => {
    const socket = net.createConnection({ host, port });
    const finish = (result) => {
      clearTimeout(timer);
      socket.destroy();
      resolve(result);
    };
    const timer = setTimeout(() => finish(false), timeoutMs);
    socket.once('connect', () => finish(true));
    socket.once('error', () => finish(false));
  });
}

export function windowsListeningPidsForPort(port, { run = spawnSync } = {}) {
  const result = run('netstat.exe', ['-ano', '-p', 'tcp'], {
    encoding: 'utf8',
    windowsHide: true,
  });
  if (result.error || result.status !== 0) {
    return new Set();
  }
  const pids = new Set();
  for (const line of String(result.stdout ?? '').split(/\r?\n/u)) {
    const match = line.match(/^\s*TCP\s+\S+:(\d+)\s+\S+\s+LISTENING\s+(\d+)\s*$/u);
    if (match && Number(match[1]) === port) {
      pids.add(Number(match[2]));
    }
  }
  return pids;
}

/**
 * Explain why the gateway start port cannot be bound, so the "no available
 * port" failure carries an actionable cause instead of a bare count.
 * A port that accepts connections but has no Windows listener is mirrored
 * from WSL2/Docker or another VM (the local dev gateway cannot bind it).
 */
export async function diagnoseGatewayPortBlocked({
  host = DEFAULT_GATEWAY_HOST,
  port,
  platform = process.platform,
  listListeningPids = windowsListeningPidsForPort,
  probeConnect = probeTcpConnect,
} = {}) {
  if (platform === 'win32') {
    const pids = listListeningPids(port);
    if (pids.size > 0) {
      return `${host}:${port} is occupied by PID(s) ${[...pids].sort((left, right) => left - right).join(', ')}; stop the stale process and retry`;
    }
    if (await probeConnect(host, port)) {
      return `${host}:${port} accepts connections but has no Windows listener; the port is likely mirrored from WSL2/Docker or another VM and cannot be bound by the local dev gateway`;
    }
  }
  return `${host}:${port} could not be bound by this process`;
}

export function createStandaloneGatewayCargoEnv({
  env = process.env,
  platform = process.platform,
  repoRoot,
} = {}) {
  if (!repoRoot) {
    throw new Error('repoRoot is required for standalone gateway cargo env resolution');
  }

  const explicitTargetDir = normalizeText(env.CARGO_TARGET_DIR);
  const cargoTargetDir = explicitTargetDir
    ? path.resolve(repoRoot, explicitTargetDir)
    : path.join(repoRoot, 'target', 'sdkwork', 'sdkwork-api-im-standalone-gateway-dev');
  const explicitIncremental = normalizeText(env.CARGO_INCREMENTAL);

  return {
    env: {
      ...env,
      CARGO_TARGET_DIR: cargoTargetDir,
      ...(platform === 'win32' && !explicitIncremental ? { CARGO_INCREMENTAL: '0' } : {}),
    },
    usingDefaultTargetDir: !explicitTargetDir,
  };
}

function createBindEnvResult(env, host, port, requestedPort) {
  const bindAddr = `${host}:${port}`;
  const portChanged = port !== requestedPort;
  const publicHost = publicUrlHostForBindHost(host);
  const derivedHttpUrl = `http://${publicHost}:${port}`;
  const derivedWebsocketUrl = `ws://${publicHost}:${port}`;
  const httpUrl = !portChanged
    ? normalizeText(env[APPLICATION_PUBLIC_HTTP_URL_ENV]) ?? derivedHttpUrl
    : derivedHttpUrl;
  const websocketUrl = !portChanged
    ? normalizeText(env[APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]) ?? derivedWebsocketUrl
    : derivedWebsocketUrl;
  const viteHttpUrl = !portChanged
    ? normalizeText(env[VITE_APPLICATION_PUBLIC_HTTP_URL_ENV]) ?? httpUrl
    : httpUrl;
  const viteWebsocketUrl = !portChanged
    ? normalizeText(env[VITE_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]) ?? websocketUrl
    : websocketUrl;
  return {
    bindAddr,
    env: {
      ...env,
      [APPLICATION_PUBLIC_INGRESS_BIND_ENV]: bindAddr,
      [APPLICATION_PUBLIC_HTTP_URL_ENV]: httpUrl,
      [APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]: websocketUrl,
      [VITE_APPLICATION_PUBLIC_HTTP_URL_ENV]: viteHttpUrl,
      [VITE_APPLICATION_PUBLIC_WEBSOCKET_URL_ENV]: viteWebsocketUrl,
    },
    portChanged,
  };
}

export async function resolveStandaloneGatewayBindEnv({
  env = process.env,
  isPortAvailable = isTcpPortAvailable,
  maxAttempts = DEFAULT_MAX_GATEWAY_PORT_ATTEMPTS,
  reservedPorts = DEFAULT_RESERVED_GATEWAY_PORTS,
  diagnose = diagnoseGatewayPortBlocked,
} = {}) {
  const explicitBind = parseBindAddr(env[APPLICATION_PUBLIC_INGRESS_BIND_ENV]);
  const host = explicitBind?.host ?? DEFAULT_GATEWAY_HOST;
  const startPort = explicitBind?.port ?? DEFAULT_GATEWAY_PORT;
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

  const diagnosis = await diagnose({ host, port: startPort }).catch(
    () => `${host}:${startPort} could not be bound`,
  );
  throw new Error(
    `No available sdkwork-api-im-standalone-gateway port found from ${startPort} after ${maxAttempts} attempts; ${diagnosis}`,
  );
}
