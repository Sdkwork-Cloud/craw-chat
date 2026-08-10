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
const knowledgebaseClientSource = readText(
  appRoot,
  'packages',
  'sdkwork-im-h5-core',
  'src',
  'sdk',
  'knowledgebaseAppSdkClient.ts',
);
const knowledgeCompatibilitySource = readText(
  appRoot,
  'packages',
  'sdkwork-im-h5-knowledge',
  'src',
  'index.ts',
);
const knowledgeOwnerServiceSource = readText(
  workspaceRoot,
  'sdkwork-knowledgebase',
  'apps',
  'sdkwork-knowledgebase-common',
  'packages',
  'sdkwork-knowledgebase-mobile-react-knowledge',
  'src',
  'services',
  'KnowledgeBaseService.ts',
);

assert.equal(
  rootPackageJson.scripts?.['test:h5-knowledgebase-app-sdk-integration'],
  'node apps/sdkwork-im-h5/scripts/knowledgebase-app-sdk-integration-contract.test.mjs',
  'The repository must expose the H5 Knowledgebase SDK integration contract.',
);
assert.equal(
  corePackageJson.dependencies?.['@sdkwork/knowledgebase-app-sdk'],
  'workspace:*',
  'H5 core must consume the owner Knowledgebase app SDK through the workspace package.',
);
assert.equal(
  appPackageJson.dependencies?.['@sdkwork/knowledgebase-app-sdk'],
  'workspace:*',
  'The H5 app must consume the owner Knowledgebase app SDK through the workspace package.',
);
assert.equal(
  appPackageJson.dependencies?.['@sdkwork/knowledgebase-mobile-react-knowledge'],
  'workspace:*',
  'The H5 app must consume the owner Knowledgebase mobile UI package.',
);
assert.ok(
  coreComponentSpec.contracts?.sdkDependencies?.some(
    (dependency) => dependency.workspace === 'sdkwork-knowledgebase-app-sdk'
      && dependency.surface === 'app-api',
  ),
  'H5 core component metadata must declare the Knowledgebase app SDK dependency.',
);
assert.match(
  knowledgebaseClientSource,
  /createKnowledgebaseAppClient\s+as\s+createGeneratedKnowledgebaseAppClient[\s\S]*from '@sdkwork\/knowledgebase-app-sdk'/u,
  'H5 core must construct Knowledgebase through the owner SDK factory.',
);
assert.match(
  sdkBootstrapSource,
  /initKnowledgebaseAppSdkClient\([\s\S]*tokenManager/u,
  'H5 bootstrap must inject its TokenManager into the Knowledgebase app SDK.',
);
assert.match(
  capabilityBootstrapSource,
  /configureKnowledgeBaseRuntime\(\{[\s\S]*client: sdkClients\.knowledgebaseAppSdkClient[\s\S]*resolveScopeKey/u,
  'H5 bootstrap must inject its shared Knowledgebase app SDK client and a registry scope into the owner mobile runtime.',
);
assert.match(
  tokenManagerSource,
  /cachedBinding\s*=\s*createTokenManager\(\)/u,
  'H5 bootstrap must own one cached TokenManager binding.',
);
assert.doesNotMatch(
  `${knowledgebaseClientSource}\n${sdkBootstrapSource}`,
  /fetch\(|axios|Authorization|Access-Token/u,
  'H5 Knowledgebase integration must not assemble raw HTTP or credential headers.',
);
assert.match(
  knowledgeCompatibilitySource,
  /export \* from '@sdkwork\/knowledgebase-mobile-react-knowledge'/u,
  'The local H5 Knowledgebase package must remain a compatibility export of the owner UI package.',
);
assert.match(
  knowledgeOwnerServiceSource,
  /KnowledgeBaseCapabilityUnavailableError/u,
  'Knowledgebase UI must fail closed before its owner runtime port is composed.',
);
assert.match(
  knowledgeOwnerServiceSource,
  /configureKnowledgeBaseRuntime/u,
  'Knowledgebase UI must expose a runtime composition port for the host to inject.',
);
for (const ownerSdkOperation of [
  /client\.knowledge\.spaces\.create/u,
  /client\.knowledge\.spaces\.retrieve/u,
  /client\.knowledge\.spaces\.update/u,
  /client\.knowledge\.spaces\.delete/u,
  /client\.knowledge\.documents\.list/u,
  /client\.knowledge\.documents\.create/u,
  /client\.knowledge\.documents\.retrieve/u,
  /client\.knowledge\.documents\.update/u,
  /client\.knowledge\.documents\.delete/u,
  /client\.knowledge\.documents\.content\.list/u,
  /client\.knowledge\.ingests\.create/u,
]) {
  assert.match(
    knowledgeOwnerServiceSource,
    ownerSdkOperation,
    'Knowledgebase owner UI operations must delegate to the injected Knowledgebase app SDK.',
  );
}
assert.doesNotMatch(
  knowledgeOwnerServiceSource,
  /fetch\s*\(|axios|Authorization|Access-Token|Math\.random|\/mock\//u,
  'Knowledgebase owner UI must not own transport, credentials, or fake data.',
);

console.log('sdkwork im H5 Knowledgebase app SDK integration contract passed.');
