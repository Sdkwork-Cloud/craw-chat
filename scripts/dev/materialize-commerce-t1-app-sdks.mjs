#!/usr/bin/env node
/**
 * Materialize per-T1 commerce app SDK families from the retired monolith OpenAPI snapshot.
 * Owner repos: sdkwork-catalog, sdkwork-shop, sdkwork-order, sdkwork-membership.
 */

import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const imRepoRoot = path.resolve(scriptDir, '..', '..');
const workspaceRoot = path.resolve(imRepoRoot, '..');
const GENERATOR_BIN = path.resolve(workspaceRoot, 'sdkwork-sdk-generator', 'bin', 'sdkgen.js');
const COMMERCE_MONOLITH_OPENAPI = path.join(
  workspaceRoot,
  'sdkwork-mall',
  'sdks',
  'sdkwork-commerce-app-sdk',
  'sdkwork-commerce-app-sdk-typescript',
  'generated',
  'server-openapi',
  'source-openapi.json',
);

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
  {
    repoId: 'sdkwork-membership',
    familyName: 'sdkwork-membership-app-sdk',
    authorityName: 'sdkwork-membership-app-api',
    packageName: '@sdkwork/membership-app-sdk',
    displayTitle: 'SDKWork Membership App API',
    apiPrefix: '/app/v3/api',
    defaultBaseUrl: 'http://127.0.0.1:18079',
    pathPrefixes: ['/app/v3/api/memberships'],
    sourceOpenapiRelative: 'apis/app-api/membership/membership-app-api.openapi.json',
    sliceFromMonolith: true,
    skipSdkgen: true,
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


function readOwnerOpenapi(repoRoot, family) {
  const filePath = path.join(repoRoot, family.sourceOpenapiRelative);
  if (existsSync(filePath)) {
    return readJson(filePath);
  }
  if (family.sliceFromMonolith) {
    return sliceOpenapiFromMonolith(family);
  }
  fail(`missing owner OpenAPI authority: ${filePath}`);
}

function collectSchemaRefs(value, refs = new Set()) {
  if (Array.isArray(value)) {
    for (const item of value) {
      collectSchemaRefs(item, refs);
    }
    return refs;
  }
  if (value && typeof value === 'object') {
    if (typeof value.$ref === 'string' && value.$ref.startsWith('#/components/schemas/')) {
      refs.add(value.$ref.replace('#/components/schemas/', ''));
    }
    for (const nested of Object.values(value)) {
      collectSchemaRefs(nested, refs);
    }
  }
  return refs;
}

function sliceOpenapiFromMonolith(family) {
  if (!existsSync(COMMERCE_MONOLITH_OPENAPI)) {
    fail(`missing commerce monolith OpenAPI snapshot: ${COMMERCE_MONOLITH_OPENAPI}`);
  }
  const monolith = readJson(COMMERCE_MONOLITH_OPENAPI);
  const paths = {};
  for (const [routePath, pathItem] of Object.entries(monolith.paths ?? {})) {
    if (family.pathPrefixes.some((prefix) => routePath === prefix || routePath.startsWith(`${prefix}/`))) {
      paths[routePath] = pathItem;
    }
  }
  if (Object.keys(paths).length === 0) {
    fail(`no paths matched for ${family.familyName}`);
  }

  const schemaNames = new Set(['ProblemDetail']);
  for (const pathItem of Object.values(paths)) {
    collectSchemaRefs(pathItem, schemaNames);
  }

  let pending = [...schemaNames];
  const schemas = {};
  while (pending.length > 0) {
    const schemaName = pending.pop();
    if (schemas[schemaName] || !monolith.components?.schemas?.[schemaName]) {
      continue;
    }
    schemas[schemaName] = monolith.components.schemas[schemaName];
    const nested = new Set();
    collectSchemaRefs(schemas[schemaName], nested);
    for (const name of nested) {
      if (!schemas[name]) {
        pending.push(name);
      }
    }
  }

  return {
    openapi: monolith.openapi ?? '3.1.2',
    info: {
      title: family.displayTitle,
      version: '1.0.0',
      description: `App/client contract for ${family.repoId} capability (split from retired sdkwork-commerce monolith).`,
      'x-sdkwork-api-authority': family.authorityName,
      'x-sdkwork-sdk-family': family.familyName,
      'x-sdkwork-audience': 'App, desktop, mobile, H5, and user-facing clients.',
      'x-sdkwork-owner': family.repoId,
    },
    servers: [
      {
        url: family.defaultBaseUrl,
        description: `Local ${family.repoId} via IM gateway`,
      },
    ],
    tags: (monolith.tags ?? []).filter((tag) => tag.name === 'memberships'),
    paths,
    components: {
      schemas,
      securitySchemes: monolith.components?.securitySchemes ?? {},
    },
  };
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
    const openapi = readOwnerOpenapi(repoRoot, family);
    const operationCount = countOperations(openapi);
    const { familyRoot, sdkgenPath } = writeAssemblyFiles(repoRoot, family, openapi, operationCount);
    process.stdout.write(
      `[materialize-commerce-t1-app-sdks] ${family.familyName}: ${operationCount} operations\n`,
    );
    if (!checkOnly && !family.skipSdkgen) {
      runSdkgen(family, familyRoot, sdkgenPath);
      patchGeneratedPackageJson(familyRoot, family);
    }
  }
}

main();
