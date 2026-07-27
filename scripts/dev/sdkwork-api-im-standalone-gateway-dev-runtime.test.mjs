import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  createStandaloneGatewayCargoEnv,
  resolveStandaloneGatewayBindEnv,
} from './sdkwork-api-im-standalone-gateway-dev-runtime.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const defaultCargoEnv = createStandaloneGatewayCargoEnv({
  env: {},
  repoRoot,
});
assert.equal(
  defaultCargoEnv.env.CARGO_TARGET_DIR,
  path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-api-im-standalone-gateway-dev'),
  'pnpm dev:server must build into an isolated target dir so a locked gateway executable cannot block rebuilds',
);
assert.equal(
  defaultCargoEnv.usingDefaultTargetDir,
  true,
  'default pnpm dev:server cargo target dir should be reported as an automatic dev fallback',
);

const explicitCargoEnv = createStandaloneGatewayCargoEnv({
  env: {
    CARGO_TARGET_DIR: path.join(repoRoot, 'custom-target'),
  },
  repoRoot,
});
assert.equal(
  explicitCargoEnv.env.CARGO_TARGET_DIR,
  path.join(repoRoot, 'custom-target'),
  'pnpm dev:server must respect an explicitly configured CARGO_TARGET_DIR',
);
assert.equal(
  explicitCargoEnv.usingDefaultTargetDir,
  false,
  'explicit CARGO_TARGET_DIR must not be reported as the automatic dev fallback',
);

const fallbackBindEnv = await resolveStandaloneGatewayBindEnv({
  env: {},
  isPortAvailable: async (port) => port === 18081,
  maxAttempts: 3,
});
assert.equal(
  fallbackBindEnv.bindAddr,
  '127.0.0.1:18081',
  'pnpm dev:server must choose the next available local gateway bind when 18079 is already occupied',
);
assert.equal(
  fallbackBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND,
  '127.0.0.1:18081',
  'pnpm dev:server must pass the selected bind to the Rust gateway',
);
assert.equal(
  fallbackBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18081',
  'pnpm dev:server must expose the selected gateway URL to browser SDK env resolution',
);
assert.equal(
  fallbackBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'ws://127.0.0.1:18081',
  'pnpm dev:server must expose the selected websocket URL when the default gateway port is busy',
);
assert.equal(
  fallbackBindEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18081',
  'pnpm dev:server must keep Vite HTTP env aligned with the selected gateway bind',
);
assert.equal(
  fallbackBindEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'ws://127.0.0.1:18081',
  'pnpm dev:server must keep Vite websocket env aligned with the selected gateway bind',
);
assert.equal(
  fallbackBindEnv.portChanged,
  true,
  'pnpm dev:server must report when it had to move off the default gateway port',
);

const reservedDrivePortBindEnv = await resolveStandaloneGatewayBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:28079',
  },
  isPortAvailable: async (port) => port >= 28080,
  maxAttempts: 4,
});
assert.equal(
  reservedDrivePortBindEnv.bindAddr,
  '127.0.0.1:28082',
  'pnpm dev:server must skip reserved internal runtime ports',
);
assert.equal(
  reservedDrivePortBindEnv.portChanged,
  true,
  'pnpm dev:server must report reserved internal runtime port skips as automatic port fallback',
);

const explicitBindEnv = await resolveStandaloneGatewayBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:28079',
  },
  isPortAvailable: async (port) => port === 28079,
});
assert.equal(
  explicitBindEnv.bindAddr,
  '127.0.0.1:28079',
  'pnpm dev:server must keep an explicit SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND when that port is available',
);
assert.equal(
  explicitBindEnv.portChanged,
  false,
  'explicit available server binds must not be reported as automatic port fallback',
);

const wildcardBindEnv = await resolveStandaloneGatewayBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '0.0.0.0:18079',
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'http://127.0.0.1:18079',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'ws://127.0.0.1:18079',
  },
  isPortAvailable: async (port, host) => port === 18079 && host === '0.0.0.0',
  maxAttempts: 1,
});
assert.equal(wildcardBindEnv.bindAddr, '0.0.0.0:18079');
assert.equal(
  wildcardBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'wildcard server binds must preserve the topology-owned browser-reachable public URL',
);
assert.equal(
  wildcardBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'ws://127.0.0.1:18079',
  'wildcard server binds must preserve the topology-owned browser-reachable websocket URL',
);

const wildcardBindWithoutPublicUrl = await resolveStandaloneGatewayBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '0.0.0.0:18079',
  },
  isPortAvailable: async () => true,
  maxAttempts: 1,
});
assert.equal(
  wildcardBindWithoutPublicUrl.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'wildcard binds without an explicit public URL must derive a loopback-safe URL',
);

const explicitBusyBindEnv = await resolveStandaloneGatewayBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:18079',
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'http://127.0.0.1:18079',
    VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'http://127.0.0.1:18079',
  },
  isPortAvailable: async (port) => port === 18081,
  maxAttempts: 3,
});
assert.equal(
  explicitBusyBindEnv.bindAddr,
  '127.0.0.1:18081',
  'pnpm dev:server must rotate off an explicit topology bind when that port is already occupied',
);
assert.equal(
  explicitBusyBindEnv.portChanged,
  true,
  'topology default binds must report automatic port fallback when 18079 is busy',
);

await assert.rejects(
  () => resolveStandaloneGatewayBindEnv({
    env: {
      SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:28079',
    },
    isPortAvailable: async () => false,
    maxAttempts: 2,
  }),
  /No available sdkwork-api-im-standalone-gateway port found from 28079/u,
  'pnpm dev:server must fail clearly when no candidate port is available from an explicit bind',
);

const startScript = fs.readFileSync(
  path.join(repoRoot, 'scripts/gateway-standalone-run.mjs'),
  'utf8',
);
assert.match(
  startScript,
  /resolveStandaloneGatewayBindEnv/u,
  'pnpm dev:server startup must use the canonical gateway bind helper',
);
assert.match(
  startScript,
  /resolveStandaloneGatewayBindEnv\(\{[\s\S]*maxAttempts:\s*1/u,
  'sdkwork-app startup must keep the topology-resolved standalone gateway bind instead of drifting to another port',
);

const gatewayDevRunner = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/run-standalone-gateway-dev.mjs'),
  'utf8',
);
const standaloneGatewayMain = fs.readFileSync(
  path.join(repoRoot, 'crates/sdkwork-api-im-standalone-gateway/src/main.rs'),
  'utf8',
);
assert.match(
  standaloneGatewayMain,
  /RouterProductRuntimeOptions::desktop_for_api_assembly_host/u,
  'standalone gateway product assets must not duplicate portal APIs owned by sdkwork-api-im-assembly',
);
assert.match(
  standaloneGatewayMain,
  /\.merge\(api_assembly\.router\)/u,
  'standalone gateway must mount the canonical IM API assembly as an indivisible router',
);
assert.match(
  gatewayDevRunner,
  /validate-api-assembly\.mjs/u,
  'standalone gateway dev builds must validate the canonical IM API assembly before Cargo compilation',
);
assert.ok(
  gatewayDevRunner.indexOf('validateApiAssembly(repoRoot)')
    < gatewayDevRunner.indexOf("'build'"),
  'API assembly validation must run before the standalone gateway Cargo build',
);

const topology = JSON.parse(
  fs.readFileSync(path.join(repoRoot, 'specs/topology.spec.json'), 'utf8'),
);
const publicIngress = topology.surfaces['application.public-ingress'];
assert.ok(
  publicIngress.healthAttempts >= 300,
  'application.public-ingress must allow a bounded five-minute first-build health window',
);
assert.equal(publicIngress.healthIntervalMs, 1000);
assert.equal(publicIngress.healthTimeoutMs, 2000);
assert.match(
  fs.readFileSync(path.join(repoRoot, 'scripts/lib/im-pc-dev.mjs'), 'utf8'),
  /createStandaloneGatewayCargoEnv/u,
  'standalone dev orchestration must use the shared cargo target isolation helper',
);
