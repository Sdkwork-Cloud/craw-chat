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

const dashboardServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-dashboard/src/services/DashboardService.ts',
);
const securityServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-security/src/services/SecurityService.ts',
);

assert.match(
  dashboardServiceSource,
  /getAppSdkClientWithSession/u,
  'Console dashboard service must consume tenant portal data through the IM app SDK runtime wrapper.',
);
assert.match(
  dashboardServiceSource,
  /\.portal\.dashboard\.retrieve\s*\(/u,
  'Console dashboard service must retrieve dashboard metrics through client.portal.dashboard.retrieve().',
);
assert.match(
  dashboardServiceSource,
  /\.portal\.conversationSnapshot\.retrieve\s*\(/u,
  'Console dashboard service must retrieve conversation activity through client.portal.conversationSnapshot.retrieve().',
);
assert.doesNotMatch(
  dashboardServiceSource,
  /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Console dashboard service must not keep mock data, fake delay, raw HTTP, or manual auth header logic.',
);

assert.match(
  securityServiceSource,
  /getAppSdkClientWithSession/u,
  'Console security service must consume user-facing governance data through the IM app SDK runtime wrapper.',
);
assert.match(
  securityServiceSource,
  /\.portal\.governance\.retrieve\s*\(/u,
  'Console security service must retrieve governance health through client.portal.governance.retrieve().',
);
assert.match(
  securityServiceSource,
  /\.portal\.access\.retrieve\s*\(/u,
  'Console security service must retrieve user-facing access activity through client.portal.access.retrieve().',
);
assert.doesNotMatch(
  securityServiceSource,
  /getBackendSdkClientWithSession|\.ops\.|\.audit\.records/u,
  'Console security service must not consume backend SDK operations; backend security/audit records belong to admin.',
);
assert.doesNotMatch(
  securityServiceSource,
  /mock|mockConsoleFetch|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Console security service must not keep mock data, fake delay, raw HTTP, or manual auth header logic.',
);

const commonsApiSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/api.ts');
const consoleRolesSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-roles/src/ConsoleRoles.tsx');
const integrationServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-integrations/src/services/IntegrationService.ts',
);
const consoleAnalyticsSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-security/src/ConsoleAnalytics.tsx',
);

assert.match(
  commonsApiSource,
  /pc console api contract is not available/u,
  'PC commons api helpers must fail closed until console api contracts exist.',
);
assert.doesNotMatch(
  commonsApiSource,
  /mockConsoleFetch|mockAdminFetch|setTimeout/u,
  'PC commons api must not keep mock fetch helpers or fake delays.',
);
assert.match(
  consoleRolesSource,
  /roleService\.getRoles/u,
  'Console roles surface must load roles through the appbase SDK-backed role service.',
);
assert.doesNotMatch(
  consoleRolesSource,
  /mockRoles/u,
  'Console roles surface must not keep embedded demo role datasets.',
);
assert.match(
  integrationServiceSource,
  /console integration contract is not available/u,
  'Console integration service must fail closed until the integration SDK contract exists.',
);
assert.doesNotMatch(
  integrationServiceSource,
  /mockApps|setTimeout/u,
  'Console integration service must not keep mock datasets or fake delays.',
);
assert.match(
  consoleAnalyticsSource,
  /ConsoleContractEmptyState/u,
  'Console analytics must render contract-empty state until analytics SDK exists.',
);
assert.doesNotMatch(
  consoleAnalyticsSource,
  /8,245|Mock Chart/u,
  'Console analytics must not keep embedded demo metrics or charts.',
);

console.log('sdkwork im pc console SDK integration contract passed.');
