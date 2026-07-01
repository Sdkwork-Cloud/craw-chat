#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';

const standardChecks = [
  'test:sdkwork-workspace-structure-standard',
  'test:web-framework-standard',
  'test:web-backend-standard',
  'test:database-framework-standard',
  'test:rpc-framework-standard',
  'test:utils-standard',
  'test:h5-utils-standard',
  'test:h5-drive-app-sdk-integration',
  'test:sdkwork-im-h5-architecture-standard',
  'test:sdkwork-im-pc-architecture-standard',
  'test:sdkwork-im-pc-sdk-integration',
  'test:flutter-drive-standard',
  'test:chat-drive-upload-attribution-standard',
  'test:production-security-standard',
  'test:app-context-module-standard',
  'test:runtime-standard',
  'test:retention-enforcement-standard',
  'test:observability-bootstrap-standard',
  'test:sdkwork-im-iam-application-bootstrap-standard',
  'test:im-member-capability-alignment',
  'test:runtime-id-standard',
  'test:deprecated-service-boundary',
  'test:topology-baggage',
  'test:rtc-signaling-boundary',
  'test:rpc-contract',
  'test:database-naming-standard',
  'test:postgresql-ubuntu-wsl-guide',
  'test:postgresql-pnpm-db-command',
  'test:deployment-docs-encoding',
  'test:governed-docs-encoding',
  'test:review-step-docs-encoding',
  'test:release-docs-encoding',
  'test:architecture-docs-encoding',
  'test:docs-strip-damage',
  'test:component-spec-consistency',
  'test:apis-authority-standard',
  'check:api-response-envelope',
  'check:tailwind-integration',
  'check:agent-workflow-standard',
  'check:unified-postgres-profile',
  'check:pnpm-script-standard',
  'check:dependency-management',
  'test:sdkwork-im-pc-i18n',
  'test:sdkwork-im-pc-request-context',
  'test:sdkwork-im-pc-html-sanitize',
  'test:commercial-deployment-contract',
  'test:sdkwork-im-pc-safe-url',
  'test:im-app-context-rust',
  'test:push-provider-standard',
  'test:sdkwork-im-session-gateway-rust',
  'test:sdkwork-im-pc-communication-settings-sdk-boundary',
  'test:sdkwork-im-pc-sidebar-module-sdk-boundary',
  'test:sdkwork-im-pc-secure-session-storage',
  'test:sdkwork-im-pc-message-list-virtualization',
  'test:sdkwork-im-session-gateway-ha',
  'test:sdkwork-im-realtime-cluster-dev',
  'test:sdkwork-im-realtime-api-paths-contract',
  'test:sdkwork-im-realtime-org-scope-standard',
  'test:sdkwork-im-room-capability-standard',
  'test:sdkwork-im-session-gateway-rpc-bin',
  'test:session-gateway-rpc-bin-rust',
  'test:sdkwork-im-pc-dev-command',
  'test:sdkwork-im-pc-message-actions',
  'test:sdkwork-im-pc-sidebar-modules',
  'test:workflow-commercial-gates',
  'test:im-v3-sdk-family-contract',
];

for (const scriptName of standardChecks) {
  const result = spawnSync(pnpmExecutable, ['run', scriptName], {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const compositionVerify = spawnSync(
  process.execPath,
  ['../sdkwork-specs/tools/verify-repo.mjs', '--root', repoRoot],
  {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  },
);
if (compositionVerify.status !== 0) {
  process.exit(compositionVerify.status ?? 1);
}

const workspaceAudit = spawnSync(
  process.execPath,
  ['../sdkwork-specs/tools/audit-iam-embedded-bootstrap-workspace.mjs'],
  {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  },
);
if (workspaceAudit.status !== 0) {
  process.exit(workspaceAudit.status ?? 1);
}

process.stdout.write('sdkwork-im standards verification passed\n');
