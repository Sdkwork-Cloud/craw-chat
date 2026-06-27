#!/usr/bin/env node
/**
 * Materialize per-T1 commerce app SDK families from the retired monolith OpenAPI snapshot.
 * Owner repos: sdkwork-catalog, sdkwork-shop, sdkwork-order.
 */

import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const imRepoRoot = path.resolve(scriptDir, '..', '..');
const workspaceRoot = path.resolve(imRepoRoot, '..');
const GENERATOR_BIN = path.resolve(workspaceRoot, 'sdkwork-sdk-generator', 'bin', 'sdkgen.js');

const T1_APP_SDK_FAMILIES = [
  {
    repoId: 'sdkwork-catalog',
    familyName: 'sdkwork-catalog-app-sdk',
    authorityName: 'sdkwork-catalog-app-api',
    packageName: '@sdkwork/catalog-app-sdk',
    displayTitle: 'SDKWork Catalog App API',
    apiPrefix: '/app/v3/api',
    defaultBaseUrl: 'http://127.0.0.1:18079',
    pathPrefixes: ['/app/v3/api/catalog', '/app/v3/api/cart'],
    sourceOpenapiRelative: 'apis/app-api/catalog/catalog-app-api.openapi.json',
  },
  {
    repoId: 'sdkwork-shop',
    familyName: 'sdkwork-shop-app-sdk',
    authorityName: 'sdkwork-shop-app-api',
    packageName: '@sdkwork/shop-app-sdk',
    displayTitle: 'SDKWork Shop App API',
    apiPrefix: '/app/v3/api',
    defaultBaseUrl: 'http://127.0.0.1:18079',
    pathPrefixes: ['/app/v3/api/shops'],
    sourceOpenapiRelative: 'apis/app-api/shop/shop-app-api.openapi.json',
  },
  {
    repoId: 'sdkwork-order',
    familyName: 'sdkwork-order-app-sdk',
    authorityName: 'sdkwork-order-app-api',
    packageName: '@sdkwork/order-app-sdk',
    displayTitle: 'SDKWork Order App API',
    apiPrefix: '/app/v3/api',
    defaultBaseUrl: 'http://127.0.0.1:18079',
    pathPrefixes: [
      '/app/v3/api/orders',
      '/app/v3/api/checkout',
      '/app/v3/api/fulfillments',
      '/app/v3/api/shipments',
      '/app/v3/api/after_sales',
    ],
    sourceOpenapiRelative: 'apis/app-api/order/order-app-api.openapi.json',
  },
];

function fail(message) {
  process.stderr.write(`[materialize-commerce-t1-app-sdks] ${message}\n`);
  process.exit(1);
}

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function writeJson(filePath, value) {
  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}


function readOwnerOpenapi(repoRoot, relativePath) {
  const filePath = path.join(repoRoot, relativePath);
  if (!existsSync(filePath)) {
    fail(`missing owner OpenAPI authority: ${filePath}`);
  }
  return readJson(filePath);
}

function writeAssemblyFiles(repoRoot, family, openapi, operationCount) {
  const familyRoot = path.join(repoRoot, 'sdks', family.familyName);
  const authorityFile = `${family.authorityName}.openapi.json`;
  const authorityPath = path.join(familyRoot, 'openapi', authorityFile);
  const sdkgenPath = path.join(familyRoot, 'openapi', `${family.authorityName}.sdkgen.json`);

  writeJson(authorityPath, openapi);
  writeJson(sdkgenPath, openapi);
  writeJson(path.join(repoRoot, family.sourceOpenapiRelative), openapi);
  writeJson(path.join(familyRoot, '.sdkwork-assembly.json'), {
    schemaVersion: 1,
    workspace: family.familyName,
    title: family.displayTitle,
    apiVersion: openapi.info?.version ?? '1.0.0',
    openapiVersion: openapi.openapi ?? '3.1.2',
    sdkOwner: family.repoId,
    apiAuthority: family.authorityName,
    sourceAuthoritySpec: `../../${family.sourceOpenapiRelative}`,
    authoritySpec: `openapi/${authorityFile}`,
    generationInputSpec: `openapi/${family.authorityName}.sdkgen.json`,
    ownerOnlyOperationCount: operationCount,
    sdkDependencies: [],
    generator: {
      package: '@sdkwork/sdk-generator',
      entrypoint: '../../../sdkwork-sdk-generator/bin/sdkgen.js',
      version: '1.0.0',
      standardProfile: 'sdkwork-v3',
    },
    derivedSpecs: {
      default: `openapi/${family.authorityName}.sdkgen.json`,
    },
    discoverySurface: {
      sdkTarget: 'app',
      apiPrefix: family.apiPrefix,
      schemaUrl: `${family.apiPrefix}/openapi.json`,
      generatedProtocols: ['http-openapi'],
      manualTransports: [],
    },
    languages: [
      {
        language: 'typescript',
        workspace: `${family.familyName}-typescript`,
        generationState: 'source_ready',
        releaseState: 'not_published',
        packagePath: `${family.familyName}-typescript/generated/server-openapi`,
        manifestPath: `${family.familyName}-typescript/generated/server-openapi/package.json`,
        name: family.packageName,
        version: '0.1.0',
        description: `Generator-owned TypeScript transport SDK for ${family.displayTitle}.`,
        generatedPath: `${family.familyName}-typescript/generated/server-openapi`,
      },
    ],
    metadata: {
      managedBy: family.repoId,
      standardProfile: 'sdkwork-v3',
      supportedLanguageSubset: ['typescript'],
    },
  });

  return { familyRoot, sdkgenPath };
}

function countOperations(openapi) {
  let count = 0;
  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (['get', 'post', 'put', 'patch', 'delete'].includes(method) && operation?.operationId) {
        count += 1;
      }
    }
  }
  return count;
}

function runSdkgen(family, familyRoot, sdkgenPath) {
  if (!existsSync(GENERATOR_BIN)) {
    fail(`sdkgen not found: ${GENERATOR_BIN}`);
  }
  const outputPath = path.join(
    familyRoot,
    `${family.familyName}-typescript`,
    'generated',
    'server-openapi',
  );
  const result = spawnSync(
    'node',
    [
      GENERATOR_BIN,
      'generate',
      '--input',
      sdkgenPath,
      '--output',
      outputPath,
      '--name',
      family.familyName,
      '--type',
      'app',
      '--language',
      'typescript',
      '--base-url',
      family.defaultBaseUrl,
      '--api-prefix',
      family.apiPrefix,
      '--fixed-sdk-version',
      '0.1.0',
      '--sdk-root',
      familyRoot,
      '--sdk-name',
      family.familyName,
      '--package-name',
      family.packageName,
      '--standard-profile',
      'sdkwork-v3',
    ],
    { cwd: familyRoot, stdio: 'inherit' },
  );
  if (result.status !== 0) {
    fail(`sdkgen failed for ${family.familyName}`);
  }
}

function patchGeneratedPackageJson(familyRoot, family) {
  const packageJsonPath = path.join(
    familyRoot,
    `${family.familyName}-typescript`,
    'generated',
    'server-openapi',
    'package.json',
  );
  if (!existsSync(packageJsonPath)) {
    fail(`missing generated package.json for ${family.familyName}`);
  }
  const packageJson = readJson(packageJsonPath);
  packageJson.name = family.packageName;
  writeJson(packageJsonPath, packageJson);
}

function main() {
  const checkOnly = process.argv.includes('--check');

  for (const family of T1_APP_SDK_FAMILIES) {
    const repoRoot = path.resolve(workspaceRoot, family.repoId);
    if (!existsSync(repoRoot)) {
      fail(`missing T1 repo: ${repoRoot}`);
    }
    const openapi = readOwnerOpenapi(repoRoot, family.sourceOpenapiRelative);
    const operationCount = countOperations(openapi);
    const { familyRoot, sdkgenPath } = writeAssemblyFiles(repoRoot, family, openapi, operationCount);
    process.stdout.write(
      `[materialize-commerce-t1-app-sdks] ${family.familyName}: ${operationCount} operations\n`,
    );
    if (!checkOnly) {
      runSdkgen(family, familyRoot, sdkgenPath);
      patchGeneratedPackageJson(familyRoot, family);
    }
  }
}

main();
