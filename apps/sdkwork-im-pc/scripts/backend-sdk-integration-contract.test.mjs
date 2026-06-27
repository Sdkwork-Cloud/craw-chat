import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import assert from 'node:assert/strict';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const backendSdkClientSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/src/sdk/backendSdkClient.ts',
);
const coreIndexSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/index.ts');
const adminCoreSdkIndexSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/src/sdk/index.ts',
);
const adminDashboardServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-dashboard/src/services/AdminDashboardService.ts',
);
const infraStatusServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-infrastructure/src/services/InfraStatusService.ts',
);
const adminBillingServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-infrastructure/src/services/AdminBillingService.ts',
);
const adminComplianceServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/services/AdminComplianceService.ts',
);
const messageAuditServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/services/MessageAuditService.ts',
);

assert.match(
  backendSdkClientSource,
  /from ['"]@sdkwork-internal\/im-backend-api-generated['"]/u,
  'Admin core backend SDK wrapper must import the generated IM backend SDK package.',
);
assert.match(
  backendSdkClientSource,
  /SdkworkImBackendClient/u,
  'Admin core backend SDK wrapper must expose the product-scoped SdkworkImBackendClient.',
);
assert.match(
  backendSdkClientSource,
  /getSdkworkChatGlobalTokenManager/u,
  'Admin backend SDK wrapper must share the runtime global TokenManager.',
);
assert.match(
  backendSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'Admin backend SDK wrapper must attach dynamic SDKWork AppContext request interceptors.',
);
assert.match(
  backendSdkClientSource,
  /VITE_SDKWORK_IM_BACKEND_API_BASE_URL/u,
  'Admin backend SDK wrapper must resolve a backend API base URL surface explicitly.',
);
assert.doesNotMatch(
  backendSdkClientSource,
  /\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Admin backend SDK wrapper must not use raw HTTP or assemble auth headers manually.',
);
assert.doesNotMatch(
  coreIndexSource,
  /backendSdkClient/u,
  'PC core package must not export backend SDK wrappers; backend SDK exports belong to admin-core/sdk.',
);
assert.match(
  adminCoreSdkIndexSource,
  /export \* from ['"]\.\/backendSdkClient['"]/u,
  'Admin core sdk subpath must export the product backend SDK wrapper.',
);

for (const [label, source] of [
  ['admin dashboard service', adminDashboardServiceSource],
  ['infrastructure status service', infraStatusServiceSource],
  ['admin billing service', adminBillingServiceSource],
  ['admin compliance service', adminComplianceServiceSource],
]) {
  assert.match(
    source,
    /@sdkwork\/im-admin-core\/sdk[\s\S]*getBackendSdkClientWithSession/u,
    `${label} must receive backend/operator data through the admin-core generated IM backend SDK wrapper.`,
  );
  assert.doesNotMatch(
    source,
    /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
    `${label} must not keep mock data, fake delay, raw HTTP, or manual auth header logic.`,
  );
}

assert.doesNotMatch(
  messageAuditServiceSource,
  /getBackendSdkClientWithSession|\.audit\.records\.list\s*\(/u,
  'User-facing console message audit service must not consume backend audit records; move audit workflows to admin or add an app-api console contract.',
);

assert.match(adminDashboardServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(adminDashboardServiceSource, /\.ops\.cluster\.retrieve\s*\(/u);
assert.match(adminDashboardServiceSource, /\.audit\.records\.list\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.cluster\.retrieve\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.diagnostics\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.summary\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.events\.summary\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.events\.list\s*\(/u);
assert.match(adminComplianceServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(adminComplianceServiceSource, /\.audit\.records\.list\s*\(/u);

console.log('sdkwork im pc backend SDK integration contract passed.');
