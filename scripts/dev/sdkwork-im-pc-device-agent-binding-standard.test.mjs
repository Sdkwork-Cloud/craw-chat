#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readSibling(...segments) {
  return fs.readFileSync(path.join(workspaceRoot, ...segments), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const packageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/package.json');
const dependencies = packageJson.dependencies ?? {};
const devicesAdapterSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/DevicesView.tsx');
const aiotIntegrationSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/aiotPcIntegration.ts');
const canonicalDeviceServiceSource = readSibling(
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-device',
  'src',
  'device-service.ts',
);

assert.equal(
  dependencies['@sdkwork/aiot-backend-sdk'],
  undefined,
  'non-admin PC devices adapter must not depend on the AIoT backend SDK',
);
assert.ok(
  dependencies['@sdkwork/aiot-pc-console-device'],
  'PC devices adapter must depend on canonical AIoT PC device package',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/services/DeviceService.ts')),
  'IM must not keep duplicate DeviceService; canonical implementation lives in sdkwork-aiot-pc-console-device',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx')),
  'IM must not keep duplicate bind-agent UI; canonical implementation lives in sdkwork-aiot-pc',
);
assert.match(
  devicesAdapterSource,
  /SdkworkDevicePage/u,
  'PC devices adapter must embed canonical AIoT device page only',
);
assert.match(
  aiotIntegrationSource,
  /getImHostedAiotAppSdkClient/u,
  'IM AIoT integration must expose hosted SDK accessor for sibling packages',
);
assert.doesNotMatch(
  canonicalDeviceServiceSource,
  /@sdkwork\/aiot-backend-sdk|backendClient|BackendClient|getBackendClient/u,
  'canonical AIoT device service must not import backend SDK clients',
);
assert.match(
  canonicalDeviceServiceSource,
  /listDevicePage/u,
  'canonical AIoT device service must list devices through paginated AIoT app SDK helpers',
);

console.log('sdkwork-im-pc device adapter standard contract passed');
