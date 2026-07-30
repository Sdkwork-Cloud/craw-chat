import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  resolveStandaloneGatewayDevExecutable,
  resolveStandaloneGatewayDevTargetDir,
  waitForDevGatewayExecutableUnlock,
} from './wait-for-dev-gateway-exe-unlock.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const tempDir = fs.mkdtempSync(
  path.join(os.tmpdir(), 'sdkwork-api-im-standalone-gateway-unlock-'),
);
const executablePath = path.join(tempDir, 'sdkwork-api-im-standalone-gateway.exe');

fs.writeFileSync(executablePath, 'gateway');

const missing = await waitForDevGatewayExecutableUnlock({
  executablePath: path.join(tempDir, 'missing-gateway.exe'),
});
assert.equal(missing.unlocked, true);

const unlocked = await waitForDevGatewayExecutableUnlock({ executablePath });
assert.equal(unlocked.unlocked, true);

const defaultTargetDir = resolveStandaloneGatewayDevTargetDir({ env: {}, repoRoot });
assert.equal(
  defaultTargetDir,
  path.join(repoRoot, 'target', 'sdkwork', 'sdkwork-api-im-standalone-gateway-dev'),
  'the default build target must contain the executable selected by the launcher',
);

const relativeTargetDir = resolveStandaloneGatewayDevTargetDir({
  env: { CARGO_TARGET_DIR: 'custom-target' },
  repoRoot,
});
assert.equal(
  relativeTargetDir,
  path.join(repoRoot, 'custom-target'),
  'relative Cargo targets must resolve from the gateway workspace root',
);

const resolved = resolveStandaloneGatewayDevExecutable({
  env: {
    CARGO_TARGET_DIR: path.join(repoRoot, 'target', 'sdkwork', 'sdkwork-api-im-standalone-gateway-dev'),
  },
  repoRoot,
});
assert.match(
  resolved.replaceAll('\\', '/'),
  /\/target\/sdkwork\/sdkwork-api-im-standalone-gateway-dev\/debug\/sdkwork-api-im-standalone-gateway\.exe$/u,
);

console.log('wait-for-dev-gateway-exe-unlock.test.mjs passed');
