import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  createSdkworkImServerCargoEnv,
  resolveSdkworkImServerBindEnv,
} from './sdkwork-im-server-dev-runtime.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const defaultCargoEnv = createSdkworkImServerCargoEnv({
  env: {},
  repoRoot,
});
assert.equal(
  defaultCargoEnv.env.CARGO_TARGET_DIR,
  path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-im-server-dev'),
  'pnpm dev:server must build into an isolated target dir so locked target/debug/sdkwork-im-server.exe cannot block rebuilds',
);
assert.equal(
  defaultCargoEnv.usingDefaultTargetDir,
  true,
  'default pnpm dev:server cargo target dir should be reported as an automatic dev fallback',
);

const explicitCargoEnv = createSdkworkImServerCargoEnv({
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

const fallbackBindEnv = await resolveSdkworkImServerBindEnv({
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

const reservedDrivePortBindEnv = await resolveSdkworkImServerBindEnv({
  env: {},
  isPortAvailable: async (port) => port >= 18080,
  maxAttempts: 3,
});
assert.equal(
  reservedDrivePortBindEnv.bindAddr,
  '127.0.0.1:18081',
  'pnpm dev:server must skip 18080 because the default Drive app-api dependency binds there',
);
assert.equal(
  reservedDrivePortBindEnv.portChanged,
  true,
  'pnpm dev:server must report the reserved Drive port skip as an automatic port fallback',
);

const explicitBindEnv = await resolveSdkworkImServerBindEnv({
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

const explicitBusyBindEnv = await resolveSdkworkImServerBindEnv({
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
  () => resolveSdkworkImServerBindEnv({
    env: {
      SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:28079',
    },
    isPortAvailable: async () => false,
    maxAttempts: 2,
  }),
  /No available sdkwork-im server port found from 28079/u,
  'pnpm dev:server must fail clearly when no candidate port is available from an explicit bind',
);

const startScript = fs.readFileSync(
  path.join(repoRoot, 'scripts/im-server-dev.mjs'),
  'utf8',
);
assert.match(
  startScript,
  /createSdkworkImServerCargoEnv/u,
  'pnpm dev:server startup must use the shared cargo target isolation helper',
);
assert.match(
  startScript,
  /resolveSdkworkImServerBindEnv/u,
  'pnpm dev:server startup must use the shared gateway bind resolver',
);
