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
const adminAnnouncementsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/AdminAnnouncements.tsx',
);
const adminSettingsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/AdminSettings.tsx',
);
const consoleIntegrationsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-integrations/src/ConsoleIntegrations.tsx',
);
const consoleSettingsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-settings/src/ConsoleSettings.tsx',
);
const adminLayoutSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-core/src/AdminLayout.tsx',
);
const consoleLayoutSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-core/src/ConsoleLayout.tsx',
);

assert.match(
  groupServiceSource,
  /getImSdkClientWithSession/u,
  'Console communication group service must consume conversation data through the IM SDK runtime wrapper.',
);
assert.match(
  groupServiceSource,
  /\.chat\.inbox\.list\s*\(/u,
  'Console communication group service must list conversation-backed groups through IM SDK inbox pagination.',
);
assert.match(
  groupServiceSource,
  /\.chat\.inbox\.list\s*\(\s*\{[\s\S]*conversationType:\s*['"]group['"]/u,
  'Console communication group service must request server-side group filtering instead of paging the mixed inbox then filtering locally.',
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

function assertUnavailableCapability(source, constantName, label) {
  assert.match(
    source,
    new RegExp(`export\\s+const\\s+${constantName}\\s*=`, 'u'),
    `${label} must export its contract-unavailable reason.`,
  );
  assert.match(
    source,
    /available:\s*false\s+as\s+const/u,
    `${label} must expose an explicit unavailable capability instead of a throwing pseudo-service.`,
  );
  assert.match(
    source,
    new RegExp(`reason:\\s*${constantName}`, 'u'),
    `${label} capability must refer to the published unavailable reason.`,
  );
  assert.doesNotMatch(
    source,
    /throw\s+new\s+Error/u,
    `${label} must not use a throwing pseudo-service for a reachable UI surface.`,
  );
}

function assertUnavailablePage(source, constantName, label) {
  assert.match(
    source,
    /ConsoleContractEmptyState/u,
    `${label} must render an explicit contract-unavailable state.`,
  );
  assert.match(
    source,
    new RegExp(`description=\\{${constantName}\\}`, 'u'),
    `${label} must show the service's explicit unavailable reason.`,
  );
  assert.doesNotMatch(
    source,
    /<(?:button|input|select|textarea)\b/u,
    `${label} must not present editable or actionable controls without a generated SDK contract.`,
  );
}

assertUnavailableCapability(
  adminAnnouncementServiceSource,
  'ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE',
  'Admin announcements',
);
assertUnavailableCapability(
  adminSettingsServiceSource,
  'ADMIN_SETTINGS_CONTRACT_UNAVAILABLE',
  'Admin settings',
);
assertUnavailableCapability(
  sysSettingsServiceSource,
  'CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE',
  'Console settings',
);
assertUnavailableCapability(
  integrationServiceSource,
  'CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE',
  'Console integrations',
);

assertUnavailablePage(
  adminAnnouncementsPageSource,
  'ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE',
  'Admin announcements page',
);
assertUnavailablePage(
  adminSettingsPageSource,
  'ADMIN_SETTINGS_CONTRACT_UNAVAILABLE',
  'Admin settings page',
);
assertUnavailablePage(
  consoleIntegrationsPageSource,
  'CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE',
  'Console integrations page',
);
assertUnavailablePage(
  consoleSettingsPageSource,
  'CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE',
  'Console settings page',
);

for (const [label, layoutSource, navId, routePath] of [
  ['console integrations', consoleLayoutSource, 'integrations', 'integrations'],
  ['console settings', consoleLayoutSource, 'settings', 'settings'],
  ['admin announcements', adminLayoutSource, 'announcements', 'announcements'],
  ['admin settings', adminLayoutSource, 'settings', 'settings'],
]) {
  assert.doesNotMatch(
    layoutSource,
    new RegExp(`\\{\\s*id:\\s*['\"]${navId}['\"]\\s*,\\s*icon:`, 'u'),
    `${label} must be hidden from navigation until an authoritative SDK capability exists.`,
  );
  assert.match(
    layoutSource,
    new RegExp(`<Route\\s+path=[\"']${routePath}[\"']`, 'u'),
    `${label} must preserve its legacy direct route so bookmarked URLs fail closed explicitly.`,
  );
}

console.log('sdkwork im pc communication and settings SDK boundary contract passed.');
