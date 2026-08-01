#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function readText(root, ...segments) {
  return fs.readFileSync(path.join(root, ...segments), 'utf8');
}

function readJson(root, ...segments) {
  return JSON.parse(readText(root, ...segments));
}

const rootPackageJson = readJson(repoRoot, 'package.json');
const appPackageJson = readJson(appRoot, 'package.json');
const corePackageJson = readJson(appRoot, 'packages', 'sdkwork-im-h5-core', 'package.json');
const coreComponentSpec = readJson(
  appRoot,
  'packages',
  'sdkwork-im-h5-core',
  'specs',
  'component.spec.json',
);
const sdkBootstrapSource = readText(appRoot, 'src', 'bootstrap', 'sdkClients.ts');
const capabilityBootstrapSource = readText(appRoot, 'src', 'bootstrap', 'index.ts');
const tokenManagerSource = readText(appRoot, 'src', 'bootstrap', 'tokenManager.ts');
const driveClientSource = readText(
  appRoot,
  'packages',
  'sdkwork-im-h5-core',
  'src',
  'sdk',
  'driveAppSdkClient.ts',
);
const driveCompatibilitySource = readText(
  appRoot,
  'packages',
  'sdkwork-im-h5-cloud-drive',
  'src',
  'index.ts',
);
const driveOwnerServiceSource = readText(
  workspaceRoot,
  'sdkwork-drive',
  'apps',
  'sdkwork-drive-common',
  'packages',
  'sdkwork-drive-mobile-react-drive',
  'src',
  'services',
  'CloudDriveService.ts',
);

assert.equal(
  rootPackageJson.scripts?.['test:h5-drive-app-sdk-integration'],
  'node apps/sdkwork-im-h5/scripts/drive-app-sdk-integration-contract.test.mjs',
  'The repository must expose the H5 Drive SDK integration contract.',
);
assert.equal(
  corePackageJson.dependencies?.['@sdkwork/drive-app-sdk'],
  'workspace:*',
  'H5 core must consume the owner Drive app SDK through the workspace package.',
);
assert.equal(
  appPackageJson.dependencies?.['@sdkwork/drive-mobile-react-drive'],
  'workspace:*',
  'The H5 app must consume the owner Drive mobile UI package.',
);
assert.ok(
  coreComponentSpec.contracts?.sdkDependencies?.some(
    (dependency) => dependency.workspace === 'sdkwork-drive-app-sdk'
      && dependency.surface === 'app-api',
  ),
  'H5 core component metadata must declare the Drive app SDK dependency.',
);
assert.match(
  driveClientSource,
  /createDriveAppClient\s+as\s+createGeneratedDriveAppClient[\s\S]*from '@sdkwork\/drive-app-sdk'/u,
  'H5 core must construct Drive through the owner SDK factory.',
);
assert.match(
  sdkBootstrapSource,
  /initDriveAppSdkClient\([\s\S]*tokenManager/u,
  'H5 bootstrap must inject its TokenManager into the Drive app SDK.',
);
assert.match(
  capabilityBootstrapSource,
  /configureCloudDriveRuntime\(\{ client: sdkClients\.driveAppSdkClient \}\)/u,
  'H5 bootstrap must inject its shared Drive app SDK client into the owner mobile runtime.',
);
for (const clientInitialization of [
  /initImSdkClient\([\s\S]*tokenManager/u,
  /initNotaryH5AppSdkClient\([\s\S]*tokenManager/u,
]) {
  assert.match(
    sdkBootstrapSource,
    clientInitialization,
    'Authenticated H5 SDK clients must share the bootstrap TokenManager.',
  );
}
assert.match(
  tokenManagerSource,
  /cachedBinding\s*=\s*createTokenManager\(\)/u,
  'H5 bootstrap must own one cached TokenManager binding.',
);
assert.doesNotMatch(
  `${driveClientSource}\n${sdkBootstrapSource}`,
  /fetch\(|axios|Authorization|Access-Token/u,
  'H5 Drive integration must not assemble raw HTTP or credential headers.',
);
assert.match(
  driveCompatibilitySource,
  /export \* from '@sdkwork\/drive-mobile-react-drive'/u,
  'The local H5 Drive package must remain a compatibility export of the owner UI package.',
);
assert.match(
  driveOwnerServiceSource,
  /CloudDriveCapabilityUnavailableError/u,
  'Drive UI must fail closed before its owner runtime port is composed.',
);
for (const ownerSdkOperation of [
  /client\.drive\.spaces\.list/u,
  /client\.drive\.nodes\.list/u,
  /client\.drive\.nodes\.folders\.create/u,
  /client\.drive\.nodes\.update/u,
  /client\.drive\.nodes\.delete/u,
  /client\.uploader\.upload/u,
]) {
  assert.match(
    driveOwnerServiceSource,
    ownerSdkOperation,
    'Drive owner UI operations must delegate to the injected Drive app SDK.',
  );
}
assert.doesNotMatch(
  driveOwnerServiceSource,
  /fetch\s*\(|axios|Authorization|Access-Token|localStorage|sessionStorage|Math\.random|\/mock\//u,
  'Drive owner UI must not own transport, credentials, browser persistence, or fake data.',
);

console.log('sdkwork im H5 Drive app SDK integration contract passed.');
