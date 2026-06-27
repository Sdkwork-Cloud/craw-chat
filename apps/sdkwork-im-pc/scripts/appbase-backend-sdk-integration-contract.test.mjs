import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const packageJsonSource = read('apps/sdkwork-im-pc/package.json');
const tsconfigSource = read('apps/sdkwork-im-pc/tsconfig.json');
const viteConfigSource = read('apps/sdkwork-im-pc/vite.config.ts');
const coreIndexSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/index.ts');
const adminCoreSdkIndexSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/src/sdk/index.ts',
);
const appbaseBackendSdkClientSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/src/sdk/appbaseBackendSdkClient.ts',
);
const tenantServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-tenants/src/services/TenantService.ts',
);
const globalUserServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-tenants/src/services/GlobalUserService.ts',
);
const consoleUserServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-users/src/services/UserService.ts',
);
const consoleRoleServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-roles/src/services/RoleService.ts',
);

for (const [label, source] of [
  ['package.json', packageJsonSource],
  ['tsconfig.json', tsconfigSource],
  ['vite.config.ts', viteConfigSource],
]) {
  assert.match(
    source,
    /@sdkwork\/iam-backend-sdk/u,
    `${label} must register the appbase backend dependency SDK for IAM admin capabilities.`,
  );
}

assert.match(
  appbaseBackendSdkClientSource,
  /from ['"]@sdkwork\/iam-backend-sdk['"]/u,
  'Admin core appbase backend wrapper must import the generated appbase backend SDK package.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /SdkworkBackendClient/u,
  'Admin core appbase backend wrapper must expose the generated SdkworkBackendClient.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /getSdkworkChatGlobalTokenManager/u,
  'Admin appbase backend wrapper must share the runtime global TokenManager.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'Admin appbase backend wrapper must attach dynamic SDKWork AppContext request interceptors.',
);
assert.doesNotMatch(
  appbaseBackendSdkClientSource,
  /\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Admin appbase backend wrapper must not use raw HTTP or assemble auth headers manually.',
);
assert.doesNotMatch(
  coreIndexSource,
  /appbaseBackendSdkClient/u,
  'PC core package must not export appbase backend SDK wrappers; backend SDK exports belong to admin-core/sdk.',
);
assert.match(
  adminCoreSdkIndexSource,
  /export \* from ['"]\.\/appbaseBackendSdkClient['"]/u,
  'Admin core sdk subpath must export the appbase backend SDK wrapper.',
);

for (const [label, source] of [
  ['tenant service', tenantServiceSource],
  ['global user service', globalUserServiceSource],
]) {
  assert.match(
    source,
    /@sdkwork\/im-admin-core\/sdk[\s\S]*getAppbaseBackendSdkClientWithSession/u,
    `${label} must consume IAM admin data through the admin-core appbase backend SDK wrapper.`,
  );
  assert.doesNotMatch(
    source,
    /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
    `${label} must not keep mock data, fake delay, raw HTTP, or manual auth header logic.`,
  );
}

for (const [label, source] of [
  ['console user service', consoleUserServiceSource],
  ['console role service', consoleRoleServiceSource],
]) {
  assert.doesNotMatch(
    source,
    /getAppbaseBackendSdkClientWithSession|\.iam\.users\.(?:list|delete)\s*\(|\.iam\.roles\.(?:list|update)\s*\(/u,
    `${label} must not consume appbase backend IAM administration resources from a user-facing console package.`,
  );
  assert.match(
    source,
    /getAppbaseAppSdkClientWithSession/u,
    `${label} must use the appbase app SDK wrapper when a user-facing console IAM read surface exists.`,
  );
}

assert.match(tenantServiceSource, /\.iam\.tenants\.list\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.list\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.update\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.delete\s*\(/u);
assert.match(consoleUserServiceSource, /\.iam\.organizationMemberships\.list\s*\(/u);
assert.match(consoleRoleServiceSource, /\.iam\.roleBindings\.list\s*\(/u);

console.log('sdkwork im pc appbase backend SDK integration contract passed.');
