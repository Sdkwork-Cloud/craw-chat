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

const dashboardPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-dashboard/src/ConsoleDashboard.tsx',
);
const billingPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-infrastructure/src/AdminBilling.tsx',
);
const billingServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-infrastructure/src/services/AdminBillingService.ts',
);
const consoleIntegrationsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-integrations/src/ConsoleIntegrations.tsx',
);
const consoleSettingsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-settings/src/ConsoleSettings.tsx',
);
const adminAnnouncementsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/AdminAnnouncements.tsx',
);
const adminSettingsPageSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-admin-operations/src/AdminSettings.tsx',
);

for (const [label, source] of [
  ['console integrations', consoleIntegrationsPageSource],
  ['console settings', consoleSettingsPageSource],
  ['admin announcements', adminAnnouncementsPageSource],
  ['admin settings', adminSettingsPageSource],
]) {
  assert.match(
    source,
    /ConsoleContractEmptyState/u,
    `${label} must show a deliberate unavailable state rather than an empty or throwing screen.`,
  );
  assert.doesNotMatch(
    source,
    /<(?:button|input|select|textarea)\b/u,
    `${label} must not expose client controls for an unpublished API contract.`,
  );
}

assert.match(
  dashboardPageSource,
  /onRetry=\{load\}/u,
  'Dashboard failures must have a real retry path.',
);
assert.match(
  dashboardPageSource,
  /role="alert"/u,
  'Dashboard failures must be communicated explicitly to assistive technology and users.',
);
assert.match(
  dashboardPageSource,
  /view\.metrics\.length\s*>\s*0/u,
  'Dashboard metrics must render only when the server supplied a non-empty metric set.',
);
assert.doesNotMatch(
  dashboardPageSource,
  /securityAlerts|No security alert records are available\./u,
  'Dashboard must not manufacture a security-alert feed from operational health data.',
);
assert.doesNotMatch(
  dashboardPageSource,
  /setPeriod|Generate report|Quick invite|Deploy store|Publish product|View monitoring dashboard/u,
  'Dashboard must not expose operations that have no implemented SDK-backed handler.',
);

assert.match(
  billingServiceSource,
  /export\s+const\s+BILLING_EVENTS_PAGE_SIZE\s*=\s*20/u,
  'Billing events must declare a bounded server page size.',
);
assert.match(
  billingServiceSource,
  /\.admin\.billing\.events\.list\s*\(\s*\{\s*pageSize:\s*BILLING_EVENTS_PAGE_SIZE\s*\}\s*\)/u,
  'Billing events must request a bounded server page rather than a potentially unbounded list.',
);
assert.match(
  billingPageSource,
  /setRequestVersion/u,
  'Billing failures must have a real retry path.',
);
assert.match(
  billingPageSource,
  /No billing event records are available\./u,
  'Billing must distinguish an empty event page from a fabricated transaction list.',
);
assert.doesNotMatch(
  billingPageSource,
  /Export CSV|Configure Plans|View All|FileText/u,
  'Billing must not expose export, plan-management, list-navigation, or receipt controls without SDK-backed handlers.',
);

console.log('sdkwork im pc reachable surface fail-closed contract passed.');
