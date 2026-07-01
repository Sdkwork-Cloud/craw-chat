#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const iamRepoRoot = path.resolve(repoRoot, '..', 'sdkwork-iam');
const capabilitySpec = JSON.parse(
  fs.readFileSync(path.join(repoRoot, 'specs/im-member-capability.spec.json'), 'utf8'),
);

function read(relativePath, root = repoRoot) {
  return fs.readFileSync(path.join(root, relativePath), 'utf8');
}

function readJson(relativePath, root = repoRoot) {
  return JSON.parse(read(relativePath, root));
}

const appManifest = readJson('sdkwork.app.config.json');
const bootstrapScope = appManifest.backend?.accessTokenPermissionScope ?? [];
for (const forbidden of capabilitySpec.bootstrapAccessTokenScope.forbiddenCodes) {
  assert.equal(
    bootstrapScope.includes(forbidden),
    false,
    `bootstrap accessTokenPermissionScope must not include admin-only code ${forbidden}`,
  );
}
for (const allowed of capabilitySpec.bootstrapAccessTokenScope.allowedCodes) {
  assert.ok(
    bootstrapScope.includes(allowed),
    `bootstrap accessTokenPermissionScope must include ${allowed}`,
  );
}

const roleCatalogSource = read('crates/sdkwork-iam-bootstrap/src/role_catalog.rs', iamRepoRoot);
const iamKernelManifest = readJson(
  'iam/modules/iam-kernel/iam.module.manifest.json',
  iamRepoRoot,
);
const appUserGrant =
  iamKernelManifest.roles?.roleGrantExtensions?.find(
    (entry) => entry.roleCode === capabilitySpec.memberRoleCode,
  ) ?? null;
assert.ok(appUserGrant, 'iam-kernel manifest must declare app_user roleGrantExtensions');

for (const pattern of capabilitySpec.requiredAppUserPermissionPatterns) {
  assert.ok(
    appUserGrant.patterns.includes(pattern),
    `app_user must grant ${pattern} in iam-kernel manifest`,
  );
  assert.match(
    roleCatalogSource,
    new RegExp(`"${pattern.replaceAll('.', '\\.')}"`),
    `app_user must grant ${pattern} in role_catalog.rs`,
  );
}

const iamOpenApiSource = read('apis/app-api/iam/sdkwork-iam-app-api.openapi.yaml', iamRepoRoot);
for (const route of capabilitySpec.openapiSelfServiceRoutes) {
  assert.match(
    iamOpenApiSource,
    new RegExp(
      `"operationId": "${route.operationId}"[\\s\\S]*?"x-sdkwork-permission": "${route.requiredPermission.replaceAll('.', '\\.')}"`,
    ),
    `OpenAPI ${route.operationId} must declare x-sdkwork-permission ${route.requiredPermission}`,
  );
}

const iamManifestSource = read(
  'crates/sdkwork-routes-iam-app-api/src/manifest.rs',
  iamRepoRoot,
);
for (const route of capabilitySpec.openapiSelfServiceRoutes) {
  assert.match(
    iamManifestSource,
    new RegExp(
      `"${route.operationId}"[\\s\\S]*?with_required_permission\\("${route.requiredPermission.replaceAll('.', '\\.')}"\\)`,
    ),
    `IAM route manifest ${route.operationId} must enforce ${route.requiredPermission}`,
  );
}

const standaloneGatewayMain = read('services/sdkwork-im-standalone-gateway/src/main.rs');
assert.match(
  standaloneGatewayMain,
  /application_router\s*\n\s*\.merge\(iam_router\)/u,
  'standalone gateway must merge embedded IAM router after application router so IAM wins over proxy catch-alls',
);
assert.doesNotMatch(
  standaloneGatewayMain,
  /iam_router\s*\n\s*\.merge\(application_router\)/u,
  'standalone gateway must not merge application router after IAM (proxy would override embedded IAM)',
);

for (const relativePath of [
  'apps/sdkwork-im-pc/specs/component.spec.json',
  'apps/sdkwork-im-h5/specs/component.spec.json',
  'apps/sdkwork-im-flutter-mobile/specs/component.spec.json',
]) {
  const componentSpec = readJson(relativePath);
  assert.ok(
    componentSpec.contracts?.permissionComposition,
    `${relativePath} must declare contracts.permissionComposition`,
  );
  assert.equal(
    componentSpec.contracts.permissionComposition.inheritanceMode,
    'module-catalog-with-overrides',
    `${relativePath} permissionComposition.inheritanceMode`,
  );
  const moduleIds = new Set(
    (componentSpec.contracts.permissionComposition.moduleCatalogRefs ?? []).map(
      (entry) => entry.moduleId,
    ),
  );
  for (const moduleId of ['knowledge', 'mail']) {
    assert.ok(moduleIds.has(moduleId), `${relativePath} must inherit ${moduleId} IMF catalog`);
  }
}

for (const embeddedModule of capabilitySpec.embeddedModuleManifests ?? []) {
  const manifestPath = path.resolve(repoRoot, embeddedModule.manifestRef);
  assert.ok(fs.existsSync(manifestPath), `embedded IMF manifest must exist: ${manifestPath}`);
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
  const appUserGrant =
    manifest.roles?.roleGrantExtensions?.find(
      (entry) => entry.roleCode === capabilitySpec.memberRoleCode,
    ) ?? null;
  assert.ok(
    appUserGrant,
    `${embeddedModule.moduleId} IMF manifest must declare app_user roleGrantExtensions`,
  );
}

const knowledgeRouteManifest = read(
  '../sdkwork-knowledgebase/crates/sdkwork-routes-knowledgebase-app-api/src/http_route_manifest.rs',
);
assert.match(
  knowledgeRouteManifest,
  /"spaces\.create"[\s\S]*?"knowledge\.spaces\.write"/u,
  'knowledge spaces.create must require knowledge.spaces.write',
);

const mailAppBuild = read('../sdkwork-mail/crates/sdkwork-routes-mail-app-api/build.rs');
assert.match(
  mailAppBuild,
  /with_required_permission/,
  'mail app route generator must emit required_permission from route manifest',
);

console.log('IM member capability alignment check passed.');
