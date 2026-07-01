#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {
  COMMERCE_T1_APP_API_AUTHORITIES,
  COMMERCE_T1_APP_SDK_PACKAGES,
  COMMERCE_T1_APP_SDK_WORKSPACE_PATHS,
  COMMERCE_T1_SPLIT_OVERRIDE_ENV_KEY_GROUPS,
} from '../../../scripts/dev/commerce-t1-capabilities.mjs';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readRepoJson(...segments) {
  return JSON.parse(readRepoText(...segments));
}

const packageJson = readJson('package.json');
const tsconfigSource = readText('tsconfig.json');
const viteConfigSource = readText('vite.config.ts');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const shopServiceSource = readText('packages', 'sdkwork-im-pc-shop', 'src', 'services', 'ShopService.ts');
const ordersServiceSource = readText('packages', 'sdkwork-im-pc-orders', 'src', 'services', 'OrdersService.ts');
const appAuthRuntimeSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'appAuthRuntime.ts');
const membershipAppSdkClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'membershipAppSdkClient.ts');
const apiCloudGatewayConfigSource = readRepoText('configs', 'sdkwork-api-cloud-gateway.sdkwork-im.development.toml');

assert.equal(
  packageJson.scripts?.['test:commerce-app-sdk-integration'],
  'node scripts/commerce-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated commerce T1 app SDK integration contract script.',
);

for (const [capability, packageName] of Object.entries(COMMERCE_T1_APP_SDK_PACKAGES)) {
  const workspaceRelative = COMMERCE_T1_APP_SDK_WORKSPACE_PATHS[capability];
  const workspaceRelativeFromApp = path
    .relative(appRoot, path.resolve(repoRoot, workspaceRelative))
    .replace(/\\/g, '/');
  const generatedEntry = path.resolve(repoRoot, workspaceRelative, 'src', 'index.ts');
  assert.ok(
    fs.existsSync(generatedEntry),
    `Generated ${packageName} transport must exist at ${generatedEntry}`,
  );
  assert.match(
    tsconfigSource,
    new RegExp(`"${packageName.replaceAll('/', '\\/')}"[\\s\\S]*${workspaceRelativeFromApp.replaceAll('\\', '[\\\\/]')}`, 'u'),
    `tsconfig must map ${packageName} to sibling generated transport`,
  );
  assert.match(
    viteConfigSource,
    new RegExp(`find:\\s*'${packageName.replaceAll('/', '\\/')}'`, 'u'),
    `Vite must alias ${packageName} to sibling generated transport`,
  );
  if (capability === 'membership') {
    assert.match(
      pnpmWorkspaceSource,
      /apps\/sdkwork-im-pc\/packages\/\*/u,
      'repository root pnpm-workspace.yaml must include IM PC packages for membership transport',
    );
  } else {
    assert.match(
      pnpmWorkspaceSource,
      new RegExp(workspaceRelative.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
      `repository root pnpm-workspace.yaml must include ${packageName} generated transport`,
    );
  }
}

assert.doesNotMatch(
  viteConfigSource,
  /sdkwork-im-pc-commerce-t1-composed-app-sdk|@sdkwork\/commerce-app-sdk/u,
  'IM PC must not reference retired composed commerce SDK aliases.',
);

assert.match(
  shopServiceSource,
  /getCatalogAppSdkClientWithSession[\s\S]*getOrderAppSdkClientWithSession/u,
  'Shop service must consume catalog and order T1 app SDK clients.',
);

assert.match(
  ordersServiceSource,
  /getOrderAppSdkClientWithSession[\s\S]*getShopAppSdkClientWithSession/u,
  'Orders service must consume order and shop T1 app SDK clients.',
);

assert.match(
  appAuthRuntimeSource,
  /getCatalogAppSdkClient\(\)[\s\S]*getOrderAppSdkClient\(\)[\s\S]*getShopAppSdkClient\(\)/u,
  'Auth runtime must register catalog, order, and shop T1 SDK clients.',
);

assert.match(
  appAuthRuntimeSource,
  /bootstrapMembershipPcIntegrationForIm[\s\S]*rebootstrapMembershipPcIntegrationForIm/u,
  'Auth runtime must bootstrap and rebootstrap membership PC integration on session changes.',
);

assert.match(
  membershipAppSdkClientSource,
  /@sdkwork\/membership-app-sdk[\s\S]*createMembershipsApi/u,
  'Membership app SDK client must consume the membership transport surface.',
);

assert.match(
  apiCloudGatewayConfigSource,
  /serviceId = "sdkwork-membership-app-api"[\s\S]*apiPrefix = "\/app\/v3\/api\/memberships"/u,
  'IM api-cloud-gateway development config must route memberships before IAM catch-all.',
);

assert.match(
  devRunnerSource,
  /resolveImApiCloudGatewayConfigPath[\s\S]*imApiCloudGatewayConfigPath/u,
  'PC dev runner must launch api-cloud-gateway with IM-owned development config.',
);

assert.match(
  devRunnerSource,
  /COMMERCE_T1_APP_API_AUTHORITIES|SDKWORK_CATALOG_APP_API|SDKWORK_ORDER_APP_API|SDKWORK_SHOP_APP_API/u,
  'PC dev runner must bridge T1 commerce app-api upstream overrides.',
);

for (const authority of ['sdkwork-catalog-app-api', 'sdkwork-order-app-api', 'sdkwork-shop-app-api', 'sdkwork-membership-app-api']) {
  assert.match(
    gatewayConfigSource,
    new RegExp(authority.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Gateway config must expose upstream resolution for ${authority}.`,
  );
}

for (const authority of COMMERCE_T1_APP_API_AUTHORITIES) {
  const splitKeys = componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.[authority];
  const expected = COMMERCE_T1_SPLIT_OVERRIDE_ENV_KEY_GROUPS.find(
    (group) => group.some((key) => key.includes(authority.replace(/^sdkwork-/, '').replace(/-app-api$/, '').replace(/-/g, '_').toUpperCase())),
  );
  if (authority === 'sdkwork-catalog-app-api' || authority === 'sdkwork-order-app-api' || authority === 'sdkwork-shop-app-api') {
    assert.ok(splitKeys?.length, `component.spec.json must document split override env keys for ${authority}.`);
    assert.deepEqual(splitKeys, expected, `component.spec.json split override keys must match commerce T1 registry for ${authority}.`);
  }
}

for (const repoId of ['sdkwork-catalog', 'sdkwork-shop', 'sdkwork-order']) {
  const assemblyPath = path.join(
    workspaceRoot,
    repoId,
    'sdks',
    `${repoId}-app-sdk`,
    '.sdkwork-assembly.json',
  );
  assert.ok(fs.existsSync(assemblyPath), `T1 repo ${repoId} must publish .sdkwork-assembly.json for its app SDK family.`);
}

console.log('commerce T1 app SDK integration contract checks passed');
