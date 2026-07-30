#!/usr/bin/env node

import { spawn } from 'node:child_process';
import fs from 'node:fs';
import http from 'node:http';
import https from 'node:https';
import net from 'node:net';
import path from 'node:path';
import process from 'node:process';
import tls from 'node:tls';
import { fileURLToPath } from 'node:url';

import {
  IM_WEB_CLIENTS,
  isCanonicalImApiPath,
  resolveAvailableImWebClient,
} from '../lib/im-web-client-routing.mjs';

const scriptPath = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(scriptPath), '..', '..');
const viteRunnerPath = path.join(repoRoot, 'scripts', 'dev', 'run-vite-cli.mjs');
const DEFAULT_INGRESS_BIND = '0.0.0.0:3801';
const RENDERER_READY_TIMEOUT_MS = 120_000;

const CLIENT_DEFINITIONS = Object.freeze({
  [IM_WEB_CLIENTS.PC]: {
    defaultPort: 4176,
    label: 'sdkwork-im-pc',
    root: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/136.0',
  },
  [IM_WEB_CLIENTS.H5]: {
    defaultPort: 4178,
    label: 'sdkwork-im-h5',
    root: path.join(repoRoot, 'apps', 'sdkwork-im-h5'),
    userAgent: 'Mozilla/5.0 (Linux; Android 15; Mobile) Chrome/136.0',
  },
});

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function parsePort(value, label) {
  const normalized = normalizeText(value);
  if (!normalized || !/^\d+$/u.test(normalized)) {
    throw new Error(`${label} must be a TCP port number`);
  }
  const port = Number.parseInt(normalized, 10);
  if (!Number.isInteger(port) || port < 1 || port > 65_535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

export function parseAdaptiveWebBind(value = DEFAULT_INGRESS_BIND) {
  const normalized = normalizeText(value) ?? DEFAULT_INGRESS_BIND;
  const bracketed = /^\[([^\]]+)\]:(\d+)$/u.exec(normalized);
  const plain = /^([^:]+):(\d+)$/u.exec(normalized);
  const match = bracketed ?? plain;
  if (!match) {
    throw new Error('SDKWORK_IM_WEB_DEV_INGRESS_BIND must use host:port');
  }
  return {
    host: match[1],
    port: parsePort(match[2], 'SDKWORK_IM_WEB_DEV_INGRESS_BIND'),
  };
}

function normalizeHttpUrl(value, label) {
  const normalized = normalizeText(value);
  if (!normalized) {
    throw new Error(`${label} is required`);
  }
  const url = new URL(normalized);
  if (!['http:', 'https:'].includes(url.protocol)) {
    throw new Error(`${label} must use http or https`);
  }
  return url;
}

function resolveDevelopmentPublicOrigin(environment = process.env) {
  const explicit = normalizeText(environment.SDKWORK_IM_WEB_PUBLIC_ORIGIN);
  if (explicit) {
    return normalizeHttpUrl(explicit, 'SDKWORK_IM_WEB_PUBLIC_ORIGIN');
  }
  const deployment = JSON.parse(
    fs.readFileSync(path.join(repoRoot, 'etc', 'sdkwork.deployment.config.json'), 'utf8'),
  );
  const lifecycleEnvironment = normalizeText(environment.SDKWORK_IM_ENVIRONMENT) ?? 'development';
  return normalizeHttpUrl(
    deployment.environments?.[lifecycleEnvironment]?.applicationOrigin,
    `deployment applicationOrigin for ${lifecycleEnvironment}`,
  );
}

function webSocketUrlFromHttpUrl(url) {
  const websocketUrl = new URL(url);
  websocketUrl.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
  return websocketUrl;
}

function rendererSourceExists(definition) {
  return [
    path.join(definition.root, 'package.json'),
    path.join(definition.root, 'index.html'),
    path.join(definition.root, 'vite.config.ts'),
  ].every((filePath) => {
    try {
      return fs.statSync(filePath).isFile();
    } catch {
      return false;
    }
  });
}

function rendererEnvironment({
  client,
  environment,
  port,
  publicOrigin,
}) {
  const deploymentProfile = normalizeText(environment.SDKWORK_IM_DEPLOYMENT_PROFILE)
    ?? 'standalone';
  const publicHttpUrl = publicOrigin.toString().replace(/\/$/u, '');
  const publicWebSocketUrl = webSocketUrlFromHttpUrl(publicOrigin)
    .toString()
    .replace(/\/$/u, '');
  const rendererEnv = {
    ...environment,
    SDKWORK_IM_PC_DEV_HOST: '127.0.0.1',
    SDKWORK_IM_PC_DEV_PORT: String(port),
  };
  if (deploymentProfile === 'standalone') {
    Object.assign(rendererEnv, {
      VITE_SDKWORK_DRIVE_APP_API_BASE_URL: publicHttpUrl,
      VITE_SDKWORK_IAM_API_BASE_URL: publicHttpUrl,
      VITE_SDKWORK_IM_API_BASE_URL: publicHttpUrl,
      VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: publicHttpUrl,
      VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: publicWebSocketUrl,
      VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL: publicHttpUrl,
      VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: publicHttpUrl,
    });
  }
  rendererEnv.SDKWORK_IM_WEB_CLIENT = client;
  return rendererEnv;
}

function startRenderer({ client, environment, publicOrigin }) {
  const definition = CLIENT_DEFINITIONS[client];
  if (!rendererSourceExists(definition)) {
    process.stderr.write(
      `[sdkwork-im-web] ${definition.label} source is unavailable; using the other renderer\n`,
    );
    return undefined;
  }
  const portEnv = client === IM_WEB_CLIENTS.PC
    ? 'SDKWORK_IM_PC_INTERNAL_DEV_PORT'
    : 'SDKWORK_IM_H5_INTERNAL_DEV_PORT';
  const port = parsePort(environment[portEnv] ?? definition.defaultPort, portEnv);
  const child = spawn(
    process.execPath,
    [viteRunnerPath, '--host', '127.0.0.1', '--port', String(port), '--strictPort'],
    {
      cwd: definition.root,
      env: rendererEnvironment({ client, environment, port, publicOrigin }),
      shell: false,
      stdio: 'inherit',
      windowsHide: process.platform === 'win32',
    },
  );
  return {
    child,
    client,
    definition,
    ready: false,
    target: new URL(`http://127.0.0.1:${port}`),
  };
}

function waitForRenderer(renderer, timeoutMs = RENDERER_READY_TIMEOUT_MS) {
  return new Promise((resolve) => {
    const startedAt = Date.now();
    let settled = false;
    const finish = (ready) => {
      if (settled) {
        return;
      }
      settled = true;
      clearInterval(timer);
      renderer.ready = ready;
      resolve(ready);
    };
    const probe = () => {
      const request = http.get(renderer.target, {
        headers: { 'user-agent': renderer.definition.userAgent },
      }, (response) => {
        response.resume();
        finish((response.statusCode ?? 500) < 500);
      });
      request.setTimeout(1_000, () => request.destroy());
      request.once('error', () => {
        if (Date.now() - startedAt >= timeoutMs) {
          finish(false);
        }
      });
    };
    const timer = setInterval(probe, 500);
    renderer.child.once('exit', () => finish(false));
    probe();
  });
}

function proxyHeaders(request, target) {
  return {
    ...request.headers,
    host: target.host,
    'x-forwarded-host': request.headers.host ?? '',
    'x-forwarded-proto': request.socket.encrypted ? 'https' : 'http',
  };
}

function appendUserAgentVary(headers) {
  const vary = String(headers.vary ?? '')
    .split(',')
    .map((value) => value.trim())
    .filter(Boolean);
  if (!vary.some((value) => value.toLowerCase() === 'user-agent')) {
    vary.push('user-agent');
  }
  return {
    ...headers,
    vary: vary.join(', '),
  };
}

function proxyHttp(request, response, target, onError, { varyUserAgent = false } = {}) {
  const transport = target.protocol === 'https:' ? https : http;
  const upstream = transport.request({
    headers: proxyHeaders(request, target),
    hostname: target.hostname,
    method: request.method,
    path: request.url,
    port: target.port || undefined,
    protocol: target.protocol,
  }, (upstreamResponse) => {
    const headers = varyUserAgent
      ? appendUserAgentVary(upstreamResponse.headers)
      : upstreamResponse.headers;
    response.writeHead(upstreamResponse.statusCode ?? 502, headers);
    upstreamResponse.pipe(response);
  });
  upstream.once('error', (error) => onError(error));
  if (request.readableEnded || request.complete) {
    upstream.end();
  } else {
    request.pipe(upstream);
  }
}

function writeProxyFailure(response, message) {
  if (response.headersSent || response.destroyed) {
    response.destroy();
    return;
  }
  response.writeHead(502, {
    'content-type': 'text/plain; charset=utf-8',
    'cache-control': 'no-store',
  });
  response.end(message);
}

function availableRendererClients(renderers) {
  return [...renderers.values()]
    .filter((renderer) => renderer.ready)
    .map((renderer) => renderer.client);
}

function proxyRendererRequest(request, response, renderers) {
  const preferred = resolveAvailableImWebClient({
    availableClients: availableRendererClients(renderers),
    userAgent: request.headers['user-agent'],
  });
  if (!preferred) {
    writeProxyFailure(response, 'No Sdkwork IM browser renderer is available.');
    return;
  }
  const fallback = preferred === IM_WEB_CLIENTS.PC ? IM_WEB_CLIENTS.H5 : IM_WEB_CLIENTS.PC;
  const proxyOptions = { varyUserAgent: true };
  proxyHttp(request, response, renderers.get(preferred).target, (firstError) => {
    const fallbackRenderer = renderers.get(fallback);
    if (!fallbackRenderer?.ready || !['GET', 'HEAD'].includes(request.method ?? 'GET')) {
      writeProxyFailure(response, `Sdkwork IM ${preferred} renderer is unavailable: ${firstError.message}`);
      return;
    }
    proxyHttp(request, response, fallbackRenderer.target, (fallbackError) => {
      writeProxyFailure(
        response,
        `Sdkwork IM browser renderers are unavailable: ${firstError.message}; ${fallbackError.message}`,
      );
    }, proxyOptions);
  }, proxyOptions);
}

function serializeUpgradeRequest(request, target) {
  const lines = [`${request.method} ${request.url} HTTP/${request.httpVersion}`];
  const headers = proxyHeaders(request, target);
  for (const [name, value] of Object.entries(headers)) {
    if (Array.isArray(value)) {
      for (const item of value) {
        lines.push(`${name}: ${item}`);
      }
    } else if (value !== undefined) {
      lines.push(`${name}: ${value}`);
    }
  }
  return `${lines.join('\r\n')}\r\n\r\n`;
}

function openUpgradeTunnel(request, socket, head, target, onError) {
  let connected = false;
  const port = Number.parseInt(target.port || (target.protocol === 'https:' ? '443' : '80'), 10);
  const upstream = target.protocol === 'https:'
    ? tls.connect({ host: target.hostname, port, servername: target.hostname })
    : net.connect({ host: target.hostname, port });
  upstream.once(target.protocol === 'https:' ? 'secureConnect' : 'connect', () => {
    connected = true;
    upstream.write(serializeUpgradeRequest(request, target));
    if (head.length > 0) {
      upstream.write(head);
    }
    socket.pipe(upstream).pipe(socket);
  });
  upstream.once('error', (error) => {
    upstream.destroy();
    onError(error, connected);
  });
}

function proxyUpgrade(request, socket, head, target, fallbackTarget) {
  openUpgradeTunnel(request, socket, head, target, (firstError, connected) => {
    if (connected || !fallbackTarget) {
      socket.destroy(firstError);
      return;
    }
    openUpgradeTunnel(request, socket, head, fallbackTarget, (fallbackError) => {
      socket.destroy(fallbackError);
    });
  });
}

export function createAdaptiveServer({ apiTarget, renderers }) {
  const server = http.createServer((request, response) => {
    if (isCanonicalImApiPath(request.url)) {
      proxyHttp(request, response, apiTarget, (error) => {
        writeProxyFailure(response, `Sdkwork IM application ingress is unavailable: ${error.message}`);
      });
      return;
    }
    proxyRendererRequest(request, response, renderers);
  });
  server.on('upgrade', (request, socket, head) => {
    if (isCanonicalImApiPath(request.url)) {
      proxyUpgrade(request, socket, head, apiTarget);
      return;
    }
    const preferred = resolveAvailableImWebClient({
      availableClients: availableRendererClients(renderers),
      userAgent: request.headers['user-agent'],
    });
    const fallback = preferred === IM_WEB_CLIENTS.PC ? IM_WEB_CLIENTS.H5 : IM_WEB_CLIENTS.PC;
    const target = preferred ? renderers.get(preferred)?.target : undefined;
    const fallbackTarget = renderers.get(fallback)?.ready ? renderers.get(fallback).target : undefined;
    if (!target) {
      socket.destroy();
      return;
    }
    proxyUpgrade(request, socket, head, target, fallbackTarget);
  });
  return server;
}

function terminateRenderer(renderer) {
  if (!renderer?.child?.pid || renderer.child.exitCode !== null) {
    return;
  }
  renderer.child.kill('SIGTERM');
}

export async function main(environment = process.env) {
  const bind = parseAdaptiveWebBind(environment.SDKWORK_IM_WEB_DEV_INGRESS_BIND);
  const apiTarget = normalizeHttpUrl(
    environment.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',
  );
  const publicOrigin = resolveDevelopmentPublicOrigin(environment);
  const renderers = new Map();
  for (const client of [IM_WEB_CLIENTS.PC, IM_WEB_CLIENTS.H5]) {
    const renderer = startRenderer({ client, environment, publicOrigin });
    if (renderer) {
      renderers.set(client, renderer);
    }
  }
  if (renderers.size === 0) {
    throw new Error('Neither apps/sdkwork-im-pc nor apps/sdkwork-im-h5 is available');
  }

  await Promise.all([...renderers.values()].map(async (renderer) => {
    const ready = await waitForRenderer(renderer);
    if (!ready) {
      process.stderr.write(
        `[sdkwork-im-web] ${renderer.definition.label} did not become ready; using the other renderer\n`,
      );
    }
    renderer.child.on('exit', (code, signal) => {
      renderer.ready = false;
      process.stderr.write(
        `[sdkwork-im-web] ${renderer.definition.label} stopped (${signal ?? code ?? 'unknown'}); fallback remains active\n`,
      );
    });
  }));

  if (availableRendererClients(renderers).length === 0) {
    throw new Error('Neither the PC nor H5 renderer became ready');
  }

  const server = createAdaptiveServer({ apiTarget, renderers });
  const shutdown = () => {
    server.close();
    for (const renderer of renderers.values()) {
      terminateRenderer(renderer);
    }
  };
  process.once('SIGINT', shutdown);
  process.once('SIGTERM', shutdown);
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(bind.port, bind.host, resolve);
  });
  process.stdout.write(
    `[sdkwork-im-web] adaptive PC/H5 ingress ready at ${publicOrigin.toString()}\n`,
  );
  return server;
}

if (process.argv[1] && path.resolve(process.argv[1]) === scriptPath) {
  main().catch((error) => {
    process.stderr.write(`[sdkwork-im-web] ${error instanceof Error ? error.message : String(error)}\n`);
    process.exit(1);
  });
}
