#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {
  COMMERCE_T1_APP_API_AUTHORITIES,
  COMMERCE_T1_APP_SDK_PACKAGES,
  COMMERCE_T1_APP_SDK_WORKSPACE_PATHS,
  COMMERCE_T1_EXTERNAL_UPSTREAM_ENV_KEY_GROUPS,
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

function readJsonFile(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function pathRegex(value) {
  return value.replace(/\\/g, '/').split('/').map(escapeRegExp).join('[\\\\/]');
}

const packageJson = readJson('package.json');
const tsconfigSource = readText('tsconfig.json');
const viteConfigSource = readText('vite.config.ts');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const shopServiceSource = fs.readFileSync(
  path.resolve(repoRoot, '..', 'sdkwork-shop', 'apps', 'sdkwork-shop-pc', 'packages', 'sdkwork-shop-pc-consumer', 'src', 'services', 'ShopService.ts'),
  'utf8',
);
const ordersServiceSource = fs.readFileSync(
  path.resolve(repoRoot, '..', 'sdkwork-shop', 'apps', 'sdkwork-shop-pc', 'packages', 'sdkwork-shop-pc-orders', 'src', 'services', 'OrdersService.ts'),
  'utf8',
);
const commerceIntegrationSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'commercePcIntegration.ts');
const imShopAdapterSource = readText('packages', 'sdkwork-im-pc-shop', 'src', 'index.tsx');
const appAuthRuntimeSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'appAuthRuntime.ts');
const membershipAppSdkClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'membershipPcIntegration.ts');
const apiCloudGatewayConfigSource = readRepoText('configs', 'sdkwork-api-cloud-gateway.sdkwork-im.development.toml');

assert.equal(
  packageJson.scripts?.['test:commerce-app-sdk-integration'],
  'node scripts/commerce-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated commerce T1 app SDK integration contract script.',
);

for (const [capability, packageName] of Object.entries(COMMERCE_T1_APP_SDK_PACKAGES)) {
  const workspaceRelative = COMMERCE_T1_APP_SDK_WORKSPACE_PATHS[capability];
  const workspacePackageRoot = workspaceRelative.replace(/\/src\/index\.ts$/u, '');
  const facadeEntry = path.resolve(repoRoot, workspaceRelative);
  const facadeEntryFromApp = path
    .relative(appRoot, facadeEntry)
    .replace(/\\/g, '/');
  assert.ok(
    fs.existsSync(facadeEntry),
    `Composed ${packageName} facade must exist at ${facadeEntry}`,
  );
  assert.match(
    tsconfigSource,
    new RegExp(`"${escapeRegExp(packageName)}"[\\s\\S]*${pathRegex(facadeEntryFromApp)}`, 'u'),
    `tsconfig must map ${packageName} to the sibling composed facade`,
  );
  assert.match(
    viteConfigSource,
    new RegExp(`find:\\s*'${packageName.replaceAll('/', '\\/')}'`, 'u'),
    `Vite must alias ${packageName} to the sibling composed facade`,
  );
  assert.match(
    pnpmWorkspaceSource,
    new RegExp(escapeRegExp(workspacePackageRoot), 'u'),
    `repository root pnpm-workspace.yaml must include ${packageName} composed facade package`,
  );
  assert.doesNotMatch(
    pnpmWorkspaceSource,
    new RegExp(escapeRegExp(`${workspacePackageRoot}/generated/server-openapi`), 'u'),
    `repository root pnpm-workspace.yaml must not include ${packageName} generated transport as a consumer workspace entry`,
  );
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

assert.equal(
  readJson('packages', 'sdkwork-im-pc-core', 'package.json').dependencies?.['@sdkwork/shop-pc-core'],
  'workspace:*',
  'Chat PC core must bridge IM session into canonical @sdkwork/shop-pc-core.',
);

assert.match(
  imShopAdapterSource,
  /@sdkwork\/shop-pc-consumer/u,
  'IM shop adapter must consume canonical @sdkwork/shop-pc-consumer.',
);

assert.match(
  commerceIntegrationSource,
  /syncImSessionToCommercePc/u,
  'commercePcIntegration must bridge IM session into commerce PC runtime.',
);
assert.match(
  commerceIntegrationSource,
  /getImHostedCatalogAppSdkClient/u,
  'commercePcIntegration must expose hosted catalog SDK client.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/shop-pc-consumer/u,
  'Vite config must alias @sdkwork/shop-pc-consumer for embedded shop surfaces.',
);
assert.match(
  viteConfigSource,
  /@sdkwork\/shop-pc-orders/u,
  'Vite config must alias @sdkwork/shop-pc-orders for embedded orders surfaces.',
);

assert.match(
  appAuthRuntimeSource,
  /getImHostedCatalogAppSdkClient\(\)[\s\S]*getImHostedOrderAppSdkClient\(\)[\s\S]*getImHostedShopAppSdkClient\(\)/u,
  'Auth runtime must register hosted catalog, order, and shop T1 SDK clients.',
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
  const externalUpstreamKeys =
    componentSpec.integration?.foundationApiGateway?.explicitExternalUpstreamEnvKeys?.[authority];
  const expected = COMMERCE_T1_EXTERNAL_UPSTREAM_ENV_KEY_GROUPS.find(
    (group) => group.some((key) => key.includes(authority.replace(/^sdkwork-/, '').replace(/-app-api$/, '').replace(/-/g, '_').toUpperCase())),
  );
  if (authority === 'sdkwork-catalog-app-api' || authority === 'sdkwork-order-app-api' || authority === 'sdkwork-shop-app-api') {
    assert.ok(
      externalUpstreamKeys?.length,
      `component.spec.json must document explicit external upstream env keys for ${authority}.`,
    );
    assert.deepEqual(
      externalUpstreamKeys,
      expected,
      `component.spec.json explicit external upstream keys must match commerce T1 registry for ${authority}.`,
    );
  }
}

for (const repoId of ['sdkwork-catalog', 'sdkwork-shop', 'sdkwork-order']) {
  const capability = repoId.replace(/^sdkwork-/u, '');
  const sdkFamily = `${repoId}-app-sdk`;
  const familyRoot = path.join(workspaceRoot, repoId, 'sdks', sdkFamily);
  const manifestPath = path.join(familyRoot, 'sdk-manifest.json');
  const componentSpecPath = path.join(familyRoot, 'specs', 'component.spec.json');

  assert.ok(fs.existsSync(manifestPath), `T1 repo ${repoId} must publish sdk-manifest.json for its app SDK family.`);
  assert.ok(
    fs.existsSync(componentSpecPath),
    `T1 repo ${repoId} must publish specs/component.spec.json for its app SDK family.`,
  );

  const manifest = readJsonFile(manifestPath);
  const component = readJsonFile(componentSpecPath);
  assert.equal(manifest.sdkFamily, sdkFamily, `${repoId} app SDK manifest must declare the SDK family.`);
  assert.equal(manifest.sdkName, sdkFamily, `${repoId} app SDK manifest must use SDK family as sdkName.`);
  assert.equal(manifest.sdkOwner, repoId, `${repoId} app SDK manifest must declare the repository owner.`);
  assert.equal(manifest.apiAuthority, `${repoId}-app-api`, `${repoId} app SDK manifest must declare app-api authority.`);
  assert.equal(
    manifest.generationInputSpec,
    `openapi/${repoId}-app-api.sdkgen.json`,
    `${repoId} app SDK manifest must point to the derived sdkgen input.`,
  );
  assert.deepEqual(manifest.sdkDependencies, [], `${repoId} app SDK manifest must explicitly declare no SDK dependencies.`);
  assert.equal(manifest.packageName, `@sdkwork/${capability}-app-sdk`, `${repoId} app SDK manifest must declare the composed consumer package.`);
  assert.equal(
    manifest.transportPackageName,
    `${sdkFamily}-generated-typescript`,
    `${repoId} app SDK manifest must declare the generated transport package.`,
  );
  assert.equal(component.component?.type, 'sdk-family', `${repoId} app SDK component spec must classify the family.`);
  assert.equal(component.component?.root, `sdks/${sdkFamily}`, `${repoId} app SDK component spec must use the SDK family root.`);
  assert.equal(component.sdk?.family, manifest.sdkFamily, `${repoId} app SDK component spec must mirror sdk-manifest family.`);
  assert.equal(component.sdk?.authority, manifest.apiAuthority, `${repoId} app SDK component spec must mirror sdk-manifest authority.`);
  assert.equal(component.sdk?.sdkOwner, manifest.sdkOwner, `${repoId} app SDK component spec must mirror sdk-manifest owner.`);
  assert.equal(component.sdk?.packageName, manifest.packageName, `${repoId} app SDK component spec must mirror sdk-manifest package.`);
  assert.deepEqual(
    component.contracts?.sdkDependencies,
    manifest.sdkDependencies,
    `${repoId} app SDK component spec must mirror sdk-manifest sdkDependencies.`,
  );
  assert.deepEqual(
    component.contracts?.dependencyApiExports,
    [],
    `${repoId} app SDK component spec must explicitly declare no dependency API exports.`,
  );
}

assert.match(
  shopServiceSource,
  /getShippingAddresses[\s\S]*PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE/u,
  'Shop service must fail closed when shipping address contract is unavailable.',
);

console.log('commerce T1 app SDK integration contract checks passed');
