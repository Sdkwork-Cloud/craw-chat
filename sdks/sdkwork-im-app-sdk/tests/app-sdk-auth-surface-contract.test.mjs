import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import { sdkFamilyConfig } from '../bin/sdk-family-config.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(__dirname, '..');
const sdkworkImRoot = path.resolve(sdkRoot, '..', '..');
const appbaseRoot = path.resolve(sdkworkImRoot, '..', 'sdkwork-appbase');

const appPrefix = '/app/v3/api';

const appbaseOwnedAppRoutes = [
  'auth/password_reset_requests',
  'auth/password_resets',
  'auth/registrations',
  'auth/sessions',
  'auth/sessions/refresh',
  'auth/sessions/current',
  'iam/users/current',
  'iam/organizations',
  'iam/organizations/tree',
  'iam/organization_memberships',
  'iam/departments',
  'iam/departments/tree',
  'iam/department_assignments',
  'iam/positions',
  'iam/position_assignments',
  'iam/role_bindings',
  'system/iam/runtime',
  'system/iam/verification_policy',
  'oauth/authorization_urls',
  'oauth/device_authorizations',
  'oauth/device_authorizations/{}',
  'oauth/device_authorizations/{}/scans',
  'oauth/device_authorizations/{}/password_completions',
  'oauth/sessions',
];

const forbiddenGeneratedSurface = [
  /oauthAuthorization|OAuthAuthorization/u,
  /deviceAuthorization|DeviceAuthorization|deviceAuthorizations|DeviceAuthorizations/u,
  /passwordReset|PasswordReset/u,
  /verificationCode|VerificationCode/u,
  /qrAuth|QrAuth/u,
  /IamUser|IamOrganization|IamDepartment|IamPosition|IamRoleBinding/u,
  /AuthSession|CreateAuthSession|RefreshAuthSession|UpdateCurrentSession/u,
  /src\/api\/auth|src\\api\\auth/u,
  /src\/api\/iam|src\\api\\iam/u,
  /src\/api\/open-platform|src\\api\\open-platform/u,
  /src\/api\/system|src\\api\\system/u,
];

const scannedTextExtensions = new Set([
  '.dart',
  '.d.ts',
  '.js',
  '.json',
  '.md',
  '.rs',
  '.ts',
  '.yaml',
  '.yml',
]);

const yaml = await loadGeneratorYaml(sdkRoot);

function readText(relativePath) {
  return fs.readFileSync(path.join(sdkRoot, relativePath), 'utf8');
}

function readYaml(relativePath) {
  return yaml.load(readText(relativePath));
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function readExternalYaml(filePath) {
  return yaml.load(fs.readFileSync(filePath, 'utf8'));
}

function relativeRoute(pathKey, prefix) {
  assert.ok(pathKey.startsWith(`${prefix}/`), `${pathKey} must start with ${prefix}/`);
  return pathKey.slice(prefix.length + 1).replace(/\{[^}]+\}/g, '{}');
}

function routeSet(document, prefix) {
  return new Set(Object.keys(document.paths ?? {}).map((pathKey) => relativeRoute(pathKey, prefix)));
}

function assertRoutesPresent(label, document, prefix, expectedRoutes) {
  const routes = routeSet(document, prefix);
  const missing = expectedRoutes.filter((route) => !routes.has(route));
  assert.deepEqual(
    missing,
    [],
    `${label} must own the appbase identity/session/IAM/OAuth device authorization routes.`,
  );
}

function assertRoutesAbsent(label, document, prefix, forbiddenRoutes) {
  const routes = routeSet(document, prefix);
  const overlaps = forbiddenRoutes.filter((route) => routes.has(route));
  assert.deepEqual(
    overlaps,
    [],
    `${label} must not regenerate sdkwork-appbase-owned app-api routes; consume sdkwork-iam-app-sdk instead.`,
  );
}

function collectTextFiles(rootPath) {
  if (!fs.existsSync(rootPath)) {
    return [];
  }
  const files = [];
  const visit = (targetPath) => {
    const stats = fs.statSync(targetPath);
    if (stats.isDirectory()) {
      for (const entry of fs.readdirSync(targetPath)) {
        if (['node_modules', 'dist', 'build', '.dart_tool', '.sdkwork', 'manual-backups', 'tmp'].includes(entry)) {
          continue;
        }
        visit(path.join(targetPath, entry));
      }
      return;
    }
    if (stats.isFile() && scannedTextExtensions.has(path.extname(targetPath))) {
      files.push(targetPath);
    }
  };
  visit(rootPath);
  return files.sort();
}

function assertGeneratedSurfaceAbsent(label, generatedRoots) {
  const violations = [];
  for (const generatedRoot of generatedRoots) {
    for (const filePath of collectTextFiles(path.join(sdkRoot, generatedRoot))) {
      const source = fs.readFileSync(filePath, 'utf8');
      for (const pattern of forbiddenGeneratedSurface) {
        const match = pattern.exec(source);
        if (match) {
          violations.push(`${path.relative(sdkRoot, filePath).replaceAll('\\', '/')}: ${match[0]}`);
        }
      }
    }
  }
  assert.deepEqual(
    violations,
    [],
    `${label} generated transport must not expose appbase auth/IAM/session/verification/OAuth device authorization surface.`,
  );
}

function dependencyByWorkspace(dependencies, workspace) {
  return dependencies.find((dependency) => dependency.workspace === workspace);
}

const appbaseAuthority = readExternalYaml(path.join(
  appbaseRoot,
  'sdks',
  'sdkwork-iam-app-sdk',
  'openapi',
  'sdkwork-iam-app-api.openapi.yaml',
));
const sdkworkImAuthority = readYaml('openapi/sdkwork-im-app-api.openapi.yaml');
const sdkworkImDerived = readYaml('openapi/sdkwork-im-app-api.sdkgen.yaml');
const sdkworkImFlutterDerived = readYaml('openapi/sdkwork-im-app-api.flutter.sdkgen.yaml');
const assembly = readJson('.sdkwork-assembly.json');
const componentSpec = readJson('specs/component.spec.json');
const readme = readText('README.md');

assertRoutesPresent('sdkwork-iam-app-sdk authority OpenAPI', appbaseAuthority, appPrefix, appbaseOwnedAppRoutes);
assertRoutesAbsent('sdkwork-im app authority OpenAPI', sdkworkImAuthority, appPrefix, appbaseOwnedAppRoutes);
assertRoutesAbsent('sdkwork-im app sdkgen OpenAPI', sdkworkImDerived, appPrefix, appbaseOwnedAppRoutes);
assertRoutesAbsent('sdkwork-im app Flutter sdkgen OpenAPI', sdkworkImFlutterDerived, appPrefix, appbaseOwnedAppRoutes);

assert.equal(
  sdkFamilyConfig.ownsIdentityLifecycle,
  false,
  'sdkwork-im-app-sdk must declare ownsIdentityLifecycle: false and consume appbase identity/session capability.',
);
for (const appbaseRoute of appbaseOwnedAppRoutes) {
  const pathPattern = `${appPrefix}/${appbaseRoute.replaceAll('{}', '{')}`;
  assert.ok(
    !sdkFamilyConfig.requiredPaths.some((requiredPath) => requiredPath.startsWith(pathPattern.replace('{', ''))),
    `sdk-family-config requiredPaths must not include appbase-owned route ${appbaseRoute}.`,
  );
}

const configDependency = dependencyByWorkspace(sdkFamilyConfig.sdkDependencies ?? [], 'sdkwork-iam-app-sdk');
const assemblyDependency = dependencyByWorkspace(assembly.sdkDependencies ?? [], 'sdkwork-iam-app-sdk');
const componentSpecDependency = dependencyByWorkspace(componentSpec.contracts?.sdkDependencies ?? [], 'sdkwork-iam-app-sdk');

assert.ok(configDependency, 'sdk-family-config must declare sdkwork-iam-app-sdk dependency.');
assert.deepEqual(assemblyDependency, configDependency, '.sdkwork-assembly.json appbase dependency must match sdk-family-config.mjs.');
assert.deepEqual(
  componentSpecDependency,
  configDependency,
  'specs/component.spec.json appbase dependency must match sdk-family-config.mjs.',
);
assert.equal(configDependency.required, true, 'sdkwork-iam-app-sdk dependency must be required.');
assert.equal(configDependency.dependencyMode, 'consumer-sdk', 'sdkwork-iam-app-sdk dependency must use consumer-sdk mode.');
assert.equal(
  configDependency.generatedTransportImportPolicy,
  'forbidden',
  'sdkwork-iam-app-sdk dependency must be forbidden in generated app transport.',
);

for (const marker of [
  'sdkwork-iam-app-sdk',
  'appbase-identity-and-session-capability',
  'consumer-sdk',
  'generatedTransportImportPolicy',
  'forbidden',
  '@sdkwork/iam-app-sdk',
  'sdkwork_iam_app_sdk',
  'SDKWork.Appbase.AppSdk',
  'ownsIdentityLifecycle: false',
]) {
  assert.ok(readme.includes(marker), `README.md must document appbase dependency marker ${marker}.`);
}

assertGeneratedSurfaceAbsent('sdkwork-im-app-sdk', [
  'sdkwork-im-app-sdk-typescript/generated/server-openapi',
  'sdkwork-im-app-sdk-flutter/generated/server-openapi',
  'sdkwork-im-app-sdk-rust/generated/server-openapi',
]);

console.log('sdkwork-im-app-sdk appbase dependency boundary contract passed');
