import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');
const aiotDevicePackage = path.resolve(
  appRoot,
  '..',
  '..',
  '..',
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-device',
  'package.json',
);
const aiotIotPackage = path.resolve(
  appRoot,
  '..',
  '..',
  '..',
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-iot',
  'package.json',
);
const sdkworkImDevicePackage = path.resolve(
  appRoot,
  'packages',
  'sdkwork-im-pc-devices',
  'package.json',
);
const sdkworkImDeviceView = path.resolve(
  appRoot,
  'packages',
  'sdkwork-im-pc-devices',
  'src',
  'DevicesView.tsx',
);
const aiotIntegrationSource = readAppText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'aiotPcIntegration.ts');

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8');
}

function readAppText(...segments) {
  return readText(path.join(appRoot, ...segments));
}

function readRepoText(relativePath) {
  return readText(path.join(repoRoot, relativePath));
}

const moduleRegistrySource = readRepoText(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/moduleRegistry.ts',
);
const packageJson = readJson(path.join(appRoot, 'package.json'));

const devicePackage = readJson(aiotDevicePackage);
const iotPackage = readJson(aiotIotPackage);
const sdkworkImDevicePackageJson = readJson(sdkworkImDevicePackage);

assert.equal(
  packageJson.scripts?.['test:aiot-devices-sdk-integration'],
  'node scripts/sdkwork-chat-pc-aiot-devices-contract.test.mjs',
  'Chat PC must expose a dedicated AIoT devices SDK integration contract script.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"devices"/u,
  'Devices must be enabled for commercial runtime after AIoT app SDK integration.',
);

assert.equal(
  devicePackage.sdkwork?.product,
  'sdkwork-aiot',
  'Canonical device package must live in the sdkwork-aiot product workspace.',
);
assert.ok(
  Array.isArray(devicePackage.sdkwork?.supersedes) && devicePackage.sdkwork.supersedes.includes('@sdkwork/device-pc-react'),
  '@sdkwork/aiot-pc-console-device must supersede the legacy @sdkwork/device-pc-react package.',
);
assert.equal(
  devicePackage.dependencies?.['@sdkwork/aiot-app-sdk'],
  'workspace:*',
  '@sdkwork/aiot-pc-console-device must depend on @sdkwork/aiot-app-sdk for canonical device catalog integration.',
);

assert.equal(
  iotPackage.sdkwork?.product,
  'sdkwork-aiot',
  'Canonical IoT package must live in the sdkwork-aiot product workspace.',
);
assert.ok(
  Array.isArray(iotPackage.sdkwork?.supersedes) && iotPackage.sdkwork.supersedes.includes('@sdkwork/iot-pc-react'),
  '@sdkwork/aiot-pc-console-iot must supersede the legacy @sdkwork/iot-pc-react package.',
);

const aiotDeviceServiceSource = readText(
  path.resolve(
    appRoot,
    '..',
    '..',
    '..',
    'sdkwork-aiot',
    'apps',
    'sdkwork-aiot-pc',
    'packages',
    'sdkwork-aiot-pc-console-device',
    'src',
    'device-service.ts',
  ),
);
assert.match(
  aiotDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Canonical device service must consume sdkwork-aiot-app-sdk.',
);
assert.match(
  aiotDeviceServiceSource,
  /listDevicePage\s*\(/u,
  'Canonical device service must list devices through aiot-app-core listDevicePage.',
);
assert.doesNotMatch(
  aiotDeviceServiceSource,
  /\bfetch\s*\(/u,
  'Canonical device service must not use raw fetch.',
);

const aiotIotServiceSource = readText(
  path.resolve(
    appRoot,
    '..',
    '..',
    '..',
    'sdkwork-aiot',
    'apps',
    'sdkwork-aiot-pc',
    'packages',
    'sdkwork-aiot-pc-console-iot',
    'src',
    'iot-service.ts',
  ),
);
assert.match(
  aiotIotServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Canonical IoT service must consume sdkwork-aiot-app-sdk.',
);
assert.match(
  aiotIotServiceSource,
  /listDevicePage\s*\(/u,
  'Canonical IoT service must load fleet nodes through aiot-app-core listDevicePage.',
);
assert.doesNotMatch(
  aiotIotServiceSource,
  /\bfetch\s*\(/u,
  'Canonical IoT service must not use raw fetch.',
);

assert.equal(
  sdkworkImDevicePackageJson.dependencies?.['@sdkwork/aiot-pc-console-device'],
  'workspace:*',
  'Sdkwork IM device adapter must depend on @sdkwork/aiot-pc-console-device instead of embedding device UI.',
);
assert.equal(
  sdkworkImDevicePackageJson.dependencies?.['@sdkwork/aiot-backend-sdk'],
  undefined,
  'Sdkwork IM user-facing device package must not depend on the AIoT backend SDK.',
);

const sdkworkImDeviceViewSource = readText(sdkworkImDeviceView);
assert.match(
  sdkworkImDeviceViewSource,
  /@sdkwork\/aiot-pc-console-device/u,
  'Sdkwork IM device adapter must render canonical AIoT device console surfaces.',
);
assert.doesNotMatch(
  sdkworkImDeviceViewSource,
  /\bfetch\s*\(/u,
  'Sdkwork IM device adapter must not use raw fetch.',
);
assert.doesNotMatch(
  sdkworkImDeviceViewSource,
  /\/im\/v3\/api\/device|\/im\/v3\/api\/devices/u,
  'Sdkwork IM device adapter must not call retired Sdkwork IM device APIs.',
);

assert.match(
  aiotIntegrationSource,
  /syncImSessionToAiotPc/u,
  'aiotPcIntegration must bridge IM session into AIoT PC runtime session.',
);
assert.match(
  aiotIntegrationSource,
  /getImHostedAiotAppSdkClient/u,
  'aiotPcIntegration must expose a hosted AIoT app SDK client for IM settings flows.',
);

const rootCargoSource = readRepoText('Cargo.toml');
const imGatewayCargoSource = readRepoText('services/sdkwork-im-cloud-gateway/Cargo.toml');
const sessionGatewayCargoSource = readRepoText('services/session-gateway/Cargo.toml');
const imPlatformCargoSource = readRepoText('crates/im-platform-contracts/Cargo.toml');
const imPlatformExportsSource = readRepoText('crates/im-platform-contracts/src/lib.rs');
const imPlatformProviderSource = readRepoText('crates/im-platform-contracts/src/provider.rs');
const imGatewayLibSource = readRepoText('services/sdkwork-im-cloud-gateway/src/lib.rs');

for (const retiredRustMember of [
  'adapters/iot-access-local',
  'adapters/iot-mqtt',
  'crates/sdkwork-im-contract-iot',
]) {
  assert.doesNotMatch(
    rootCargoSource,
    new RegExp(retiredRustMember.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `Sdkwork IM Rust workspace must remove retired local device/IOT member ${retiredRustMember}.`,
  );
}

for (const [label, source] of [
  ['sdkwork-im-cloud-gateway Cargo.toml', imGatewayCargoSource],
  ['session-gateway Cargo.toml', sessionGatewayCargoSource],
  ['im-platform-contracts Cargo.toml', imPlatformCargoSource],
]) {
  assert.doesNotMatch(
    source,
    /im-adapter-iot-access-local|im-adapter-iot-mqtt|sdkwork-im-contract-iot/u,
    `${label} must not depend on retired Sdkwork IM-owned device/IOT crates.`,
  );
}

for (const [label, source] of [
  ['im-platform-contracts exports', imPlatformExportsSource],
  ['im-platform-contracts provider contracts', imPlatformProviderSource],
  ['sdkwork-im-cloud-gateway lib', imGatewayLibSource],
]) {
  assert.doesNotMatch(
    source,
    /DeviceAccessProvider|IotProtocolAdapter|DeviceTwin|DeviceSubject|iot-access-local|iot-mqtt/u,
    `${label} must not retain Sdkwork IM-owned device/IOT provider or twin contracts.`,
  );
}

for (const dependencyName of [
  'sdkwork-aiot-contract',
  'sdkwork-aiot-http-api',
  'sdkwork-aiot-runtime',
  'sdkwork-aiot-transport',
]) {
  assert.doesNotMatch(
    rootCargoSource,
    new RegExp(`${dependencyName}\\s*=`),
    `Sdkwork IM Rust workspace must not integrate ${dependencyName}; AIoT runtime API traffic is routed through sdkwork-api-cloud-gateway.`,
  );
  assert.doesNotMatch(
    imGatewayCargoSource,
    new RegExp(`${dependencyName}\\.workspace\\s*=\\s*true`),
    `sdkwork-im-cloud-gateway must not consume ${dependencyName}; AIoT runtime API traffic is routed through sdkwork-api-cloud-gateway.`,
  );
}

assert.doesNotMatch(
  imGatewayLibSource,
  /mod aiot_bridge;|sdkwork_aiot_http_api|aiot_app_api_server|aiot_backend_api_server/u,
  'sdkwork-im-cloud-gateway must not keep a product-local SDKWork AIoT Rust backend bridge.',
);
assert.doesNotMatch(
  imGatewayLibSource,
  /\/app\/v3\/api\/iot|\/backend\/v3\/api\/iot|aiot_bridge::|standard_app_api_server|standard_admin_api_server/u,
  'sdkwork-im-cloud-gateway must not mount AIoT app/backend API prefixes; sdkwork-api-cloud-gateway owns those foundation surfaces.',
);

console.log('sdkwork im pc AIoT devices SDK contract passed.');
