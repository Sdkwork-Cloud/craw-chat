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

const groupServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/services/GroupService.ts',
);
const consoleAnnouncementServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/services/AnnouncementService.ts',
);
const adminAnnouncementServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/services/AdminAnnouncementService.ts',
);
const integrationServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-integrations/src/services/IntegrationService.ts',
);
const adminSettingsServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/services/AdminSettingsService.ts',
);
const sysSettingsServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-settings/src/services/SysSettingsService.ts',
);

assert.match(
  groupServiceSource,
  /getImSdkClientWithSession/u,
  'Console communication group service must consume conversation data through the IM SDK runtime wrapper.',
);
assert.match(
  groupServiceSource,
  /\.chat\.inbox\.retrieve\s*\(/u,
  'Console communication group service must list conversation-backed groups through IM SDK inbox retrieval.',
);

for (const [label, source] of [
  ['console communication group service', groupServiceSource],
  ['console announcement service', consoleAnnouncementServiceSource],
  ['admin announcement service', adminAnnouncementServiceSource],
  ['admin settings service', adminSettingsServiceSource],
  ['console system settings service', sysSettingsServiceSource],
  ['console integration service', integrationServiceSource],
]) {
  assert.doesNotMatch(
    source,
    /mock|mockConsoleFetch|mockConsolePost|mockAdminFetch|mockAdminPost|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
    `${label} must not keep local stand-ins, artificial delays, raw HTTP, or manual auth header logic.`,
  );
}

assert.match(
  adminSettingsServiceSource,
  /backend settings contract is not available/u,
  'Admin settings writes must fail closed until the backend settings SDK contract exists.',
);
assert.match(
  sysSettingsServiceSource,
  /console settings contract is not available/u,
  'Console settings writes must fail closed until the console settings SDK contract exists.',
);
assert.match(
  integrationServiceSource,
  /console integration contract is not available/u,
  'Console integration reads must fail closed until the integration SDK contract exists.',
);

console.log('sdkwork im pc communication and settings SDK boundary contract passed.');
