import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  resolveStandaloneGatewayDevExecutable,
  waitForDevGatewayExecutableUnlock,
} from './wait-for-dev-gateway-exe-unlock.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-gateway-unlock-'));
const executablePath = path.join(tempDir, 'sdkwork-im-standalone-gateway.exe');

fs.writeFileSync(executablePath, 'gateway');

const missing = await waitForDevGatewayExecutableUnlock({
  executablePath: path.join(tempDir, 'missing-gateway.exe'),
});
assert.equal(missing.unlocked, true);

const unlocked = await waitForDevGatewayExecutableUnlock({ executablePath });
assert.equal(unlocked.unlocked, true);

const resolved = resolveStandaloneGatewayDevExecutable({
  env: {
    CARGO_TARGET_DIR: path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-im-standalone-gateway-dev'),
  },
  repoRoot,
});
assert.match(
  resolved.replaceAll('\\', '/'),
  /\/\.runtime\/cargo-target\/sdkwork-im-standalone-gateway-dev\/debug\/sdkwork-im-standalone-gateway\.exe$/u,
);

console.log('wait-for-dev-gateway-exe-unlock.test.mjs passed');
