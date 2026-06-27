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
const sdkworkImDeviceService = path.resolve(
  appRoot,
  'packages',
  'sdkwork-im-pc-devices',
  'src',
  'services',
  'DeviceService.ts',
);

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8');
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
  /\.iot\.devicesList\s*\(/u,
  'Canonical device service must list devices through client.iot.devicesList.',
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
  /\.iot\.devicesList\s*\(/u,
  'Canonical IoT service must load fleet nodes through client.iot.devicesList.',
);
assert.doesNotMatch(
  aiotIotServiceSource,
  /\bfetch\s*\(/u,
  'Canonical IoT service must not use raw fetch.',
);

assert.equal(
  sdkworkImDevicePackageJson.dependencies?.['@sdkwork/aiot-app-sdk'],
  'workspace:*',
  'Sdkwork IM device bridge must depend on @sdkwork/aiot-app-sdk for AIoT device reads.',
);
assert.equal(
  sdkworkImDevicePackageJson.dependencies?.['@sdkwork/aiot-backend-sdk'],
  undefined,
  'Sdkwork IM user-facing device package must not depend on the AIoT backend SDK.',
);

const sdkworkImDeviceServiceSource = readText(sdkworkImDeviceService);
assert.match(
  sdkworkImDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Sdkwork IM device bridge must consume sdkwork-aiot app SDK for user-visible reads.',
);
assert.doesNotMatch(
  sdkworkImDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-backend-sdk["']/u,
  'Sdkwork IM user-facing device service must not import sdkwork-aiot backend SDK.',
);
for (const forbiddenUserFacingBackendCall of [
  /\.iot\.devices\.create\s*\(/u,
  /\.iot\.devices\.update\s*\(/u,
  /\.iot\.devices\.delete\s*\(/u,
  /\.iot\.devices\.twin\.update\s*\(/u,
  /\.iot\.devices\.commands\.list\s*\(/u,
  /\.iot\.devices\.commands\.cancel\s*\(/u,
  /\/backend\/v3\/api\/iot/u,
]) {
  assert.doesNotMatch(
    sdkworkImDeviceServiceSource,
    forbiddenUserFacingBackendCall,
    `Sdkwork IM user-facing device service must not route through AIoT backend-admin operations: ${forbiddenUserFacingBackendCall}`,
  );
}
for (const requiredAppSdkCall of [
  /\.iot\.devicesList\s*\(/u,
  /\.iot\.devicesRetrieve\s*\(/u,
  /\.iot\.devicesCommandsCreate\s*\(/u,
]) {
  assert.match(
    sdkworkImDeviceServiceSource,
    requiredAppSdkCall,
    `Sdkwork IM device service must route user-visible device operations through AIoT app SDK: ${requiredAppSdkCall}`,
  );
}
assert.match(
  sdkworkImDeviceServiceSource,
  /unsupportedAppDeviceManagementCapability/u,
  'Sdkwork IM device service must fail closed for backend-admin device management gaps in the user-facing package.',
);
assert.doesNotMatch(
  sdkworkImDeviceServiceSource,
  /\bfetch\s*\(/u,
  'Sdkwork IM device bridge must not use raw fetch.',
);
assert.doesNotMatch(
  sdkworkImDeviceServiceSource,
  /\/im\/v3\/api\/device|\/im\/v3\/api\/devices/u,
  'Sdkwork IM device bridge must not call retired Sdkwork IM IM device APIs.',
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
