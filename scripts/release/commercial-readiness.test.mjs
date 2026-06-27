import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { mkdtemp, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import test from 'node:test';

import {
  COMMAND_FAILURE_EXIT_CODE,
  assessCapacityEvidenceIndex,
  assessPreReleaseEvidenceIndex,
  buildCommercialReadinessChecks,
  resolvePnpmExecutable,
  runCommercialReadiness,
  shouldUseShellForCommand,
} from './commercial-readiness.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');

test('commercial readiness checks cover the verified frontend and backend gate commands', () => {
  const checks = buildCommercialReadinessChecks({
    repoRoot,
    platform: 'win32',
  });

  assert.deepEqual(
    checks.map((check) => check.id),
    [
      'pc-install',
      'pc-lint',
      'pc-build',
      'h5-lint',
      'h5-build',
      'h5-architecture-standard',
      'flutter-mobile-architecture-standard',
      'im-sdk-flutter-composed-test',
      'flutter-mobile-analyze',
      'flutter-mobile-test',
      'pc-e2e-smoke',
      'pc-playwright-e2e',
      'pc-auth-appbase-ui-contract',
      'pc-domain-app-sdk-auth-runtime',
      'pc-notary-app-sdk-integration',
      'pc-drive-app-sdk-integration',
      'pc-knowledgebase-app-sdk-integration',
      'pc-commerce-app-sdk-integration',
      'pc-mail-app-sdk-integration',
      'pc-community-app-sdk-integration',
      'pc-course-app-sdk-integration',
      'pc-aiot-devices-sdk-integration',
      'pc-qr-scan-standard',
      'step11-scenario-catalog',
      'step11-ha-dr-drill',
      'commercial-deployment-contract',
      'topology-baggage',
      'dependency-management',
      'workflow-commercial-gates',
      'retention-enforcement-standard',
      'observability-bootstrap-standard',
      'im-app-sdk-flutter-parity',
      'governance-service-tests',
      'gateway-integration-tests',
      'session-gateway-tests',
    ],
  );

  assert.equal(resolvePnpmExecutable('win32'), 'pnpm.cmd');
  assert.equal(checks[0].command, 'pnpm.cmd');
  assert.equal(checks[0].env?.npm_config_update_notifier, 'false');
  for (const check of checks) {
    assert.equal(existsSync(check.cwd), true, `${check.id} cwd must exist: ${check.cwd}`);
  }
  assert.equal(
    checks.find((check) => check.id === 'pc-install')?.cwd,
    repoRoot,
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-install')?.args,
    ['install', '--frozen-lockfile', '--ignore-scripts'],
  );
  assert.equal(
    checks.find((check) => check.id === 'pc-lint')?.cwd,
    path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-lint')?.args,
    ['run', 'lint'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-build')?.args,
    ['run', 'build'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-e2e-smoke')?.args,
    ['scripts/dev/sdkwork-im-pc-e2e-smoke.test.mjs'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-playwright-e2e')?.args,
    ['scripts/dev/sdkwork-im-pc-playwright-e2e.test.mjs'],
  );
  assert.equal(
    checks.find((check) => check.id === 'pc-e2e-smoke')?.cwd,
    repoRoot,
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-auth-appbase-ui-contract')?.args,
    ['scripts/auth-appbase-ui-contract.test.mjs'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-domain-app-sdk-auth-runtime')?.args,
    ['run', 'test:domain-app-sdk-auth-runtime'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-notary-app-sdk-integration')?.args,
    ['run', 'test:notary-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-drive-app-sdk-integration')?.args,
    ['run', 'test:drive-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-knowledgebase-app-sdk-integration')?.args,
    ['run', 'test:knowledgebase-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-commerce-app-sdk-integration')?.args,
    ['run', 'test:commerce-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-mail-app-sdk-integration')?.args,
    ['run', 'test:mail-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-community-app-sdk-integration')?.args,
    ['run', 'test:community-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-course-app-sdk-integration')?.args,
    ['run', 'test:course-app-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-aiot-devices-sdk-integration')?.args,
    ['run', 'test:aiot-devices-sdk-integration'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'pc-qr-scan-standard')?.args,
    ['run', 'test:qr-scan-standard'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'dependency-management')?.args,
    ['run', 'check:dependency-management'],
  );
  assert.equal(
    checks.find((check) => check.id === 'dependency-management')?.cwd,
    repoRoot,
  );
  assert.equal(
    checks.find((check) => check.id === 'dependency-management')?.env?.npm_config_update_notifier,
    'false',
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'workflow-commercial-gates')?.args,
    ['run', 'test:workflow-commercial-gates'],
  );
  assert.equal(
    checks.find((check) => check.id === 'workflow-commercial-gates')?.cwd,
    repoRoot,
  );
  assert.equal(
    checks.find((check) => check.id === 'workflow-commercial-gates')?.env?.npm_config_update_notifier,
    'false',
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'governance-service-tests')?.args,
    ['test', '-p', 'governance-service', '--tests'],
  );
  assert.equal(
    checks.find((check) => check.id === 'pc-build')?.env?.npm_config_update_notifier,
    'false',
  );
  assert.equal(
    checks.find((check) => check.id === 'governance-service-tests')?.env,
    undefined,
  );
  assert.equal(shouldUseShellForCommand('pnpm.cmd', 'win32'), true);
  assert.equal(shouldUseShellForCommand('cargo', 'win32'), false);
});

test('capacity evidence assessment blocks template-only commercial readiness claims', () => {
  const assessment = assessCapacityEvidenceIndex({
    tier: 'Capacity Tier',
    state: 'template_only_pending_execution',
    collectionSummary: {
      pendingSlots: 7,
      collectedSlots: 0,
      requiredSlots: 7,
    },
    evidenceSlots: [
      { id: 'connection_capacity', status: 'pending_collection' },
      { id: 'message_capacity', status: 'pending_collection' },
    ],
  });

  assert.equal(assessment.ok, false);
  assert.match(assessment.summary, /Capacity Tier/);
  assert.match(assessment.summary, /template_only_pending_execution/);
  assert.match(assessment.summary, /7 pending slots/);
  assert.match(assessment.blockers.join('\n'), /connection_capacity/);
  assert.match(assessment.blockers.join('\n'), /message_capacity/);
});

test('pre-release evidence assessment accepts fully collected gate-blocked evidence', () => {
  const assessment = assessPreReleaseEvidenceIndex({
    tier: 'Pre-Release Tier',
    state: 'evidence_collected_gate_blocked',
    collectionSummary: {
      pendingSlots: 0,
      collectedSlots: 7,
      requiredSlots: 7,
    },
    evidenceSlots: Array.from({ length: 7 }, (_, index) => ({
      id: `slot_${index}`,
      status: 'collected',
    })),
  });

  assert.equal(assessment.ok, true);
  assert.match(assessment.summary, /gate-blocked pending dedicated pre-release topology sign-off/i);
  assert.equal(assessment.blockers.length, 0);
});

test('pre-release evidence assessment blocks template-only claims', () => {
  const assessment = assessPreReleaseEvidenceIndex({
    tier: 'Pre-Release Tier',
    state: 'template_only_pending_execution',
    collectionSummary: {
      pendingSlots: 7,
      collectedSlots: 0,
      requiredSlots: 7,
    },
    evidenceSlots: [
      { id: 'connection_metrics', status: 'pending_collection' },
    ],
  });

  assert.equal(assessment.ok, false);
  assert.match(assessment.summary, /template_only_pending_execution/);
});

test('capacity evidence assessment accepts fully collected capacity evidence', () => {
  const assessment = assessCapacityEvidenceIndex({
    tier: 'Capacity Tier',
    state: 'evidence_collected_gate_passed',
    collectionSummary: {
      pendingSlots: 0,
      collectedSlots: 7,
      requiredSlots: 7,
    },
    evidenceSlots: [
      { id: 'connection_capacity', status: 'collected' },
      { id: 'message_capacity', status: 'collected' },
    ],
  });

  assert.equal(assessment.ok, true);
  assert.match(assessment.summary, /fully collected/i);
  assert.equal(assessment.blockers.length, 0);
});

test('release README documents the commercial readiness command and honest capacity blocker', async () => {
  const releaseReadmePath = path.join(repoRoot, 'docs', 'release', 'README.md');
  const releaseReadme = await readFile(releaseReadmePath, 'utf8');

  assert.match(releaseReadme, /node scripts\/release\/commercial-readiness\.mjs/);
  assert.match(releaseReadme, /capacity-tier-evidence-index\.json/);
  assert.match(releaseReadme, /pre-release-tier-evidence-index\.json/);
  assert.match(releaseReadme, /Playwright/);
  assert.match(releaseReadme, /exit code `?1`?/i);
  assert.match(releaseReadme, /exit code `?2`?/i);
});

test('deployment validation index links the unified commercial readiness gate', async () => {
  const operatorIndexPath = path.join(
    repoRoot,
    'docs',
    '部署',
    '兼容矩阵与SDK-CLI-operator验证索引.md',
  );
  const operatorIndex = await readFile(operatorIndexPath, 'utf8');

  assert.match(operatorIndex, /node scripts\/release\/commercial-readiness\.mjs/);
  assert.match(operatorIndex, /docs\/release\/README\.md/);
  assert.match(operatorIndex, /exit code `?1`?/i);
});

test('commercial readiness converts thrown command execution errors into a controlled command failure result', async () => {
  const logs = createLoggerCapture();

  const result = await runCommercialReadiness({
    repoRoot,
    logger: logs.logger,
    runCheck: async (check) => {
      if (check.id === 'pc-install') {
        throw new Error('spawn pnpm ENOENT');
      }

      return {
        ...check,
        ok: true,
        exitCode: 0,
      };
    },
  });

  assert.equal(result.ok, false);
  assert.equal(result.exitCode, COMMAND_FAILURE_EXIT_CODE);
  assert.equal(result.capacityAssessment, null);
  assert.equal(result.preReleaseAssessment, null);
  assert.equal(result.checks.length, 0);
  assert.deepEqual(result.failure, {
    stage: 'pc-install',
    summary: 'spawn pnpm ENOENT',
  });
  assert.match(logs.stderr.join('\n'), /pc-install/);
  assert.match(logs.stderr.join('\n'), /spawn pnpm ENOENT/);
});

test('commercial readiness rejects missing configured working directories before spawning commands', async () => {
  const tempRepoRoot = await mkdtemp(path.join(os.tmpdir(), 'commercial-readiness-missing-cwd-'));
  const logs = createLoggerCapture();

  const result = await runCommercialReadiness({
    repoRoot: path.join(tempRepoRoot, 'missing-repo-root'),
    logger: logs.logger,
  });

  assert.equal(result.ok, false);
  assert.equal(result.exitCode, COMMAND_FAILURE_EXIT_CODE);
  assert.equal(result.capacityAssessment, null);
  assert.equal(result.preReleaseAssessment, null);
  assert.equal(result.checks.length, 0);
  assert.equal(result.failure.stage, 'pc-install');
  assert.match(result.failure.summary, /configured cwd does not exist/);
  assert.match(result.failure.summary, /missing-repo-root/);
  assert.match(logs.stderr.join('\n'), /pc-install/);
});

test('commercial readiness converts malformed capacity evidence into a controlled command failure result', async () => {
  const tempRepoRoot = await mkdtemp(path.join(os.tmpdir(), 'commercial-readiness-'));
  const preReleaseDir = path.join(tempRepoRoot, 'artifacts', 'perf', 'step-11', 'pre-release');
  const evidenceDir = path.join(tempRepoRoot, 'artifacts', 'perf', 'step-11', 'capacity');
  await mkdir(preReleaseDir, { recursive: true });
  await mkdir(evidenceDir, { recursive: true });
  await writeFile(
    path.join(preReleaseDir, 'pre-release-tier-evidence-index.json'),
    JSON.stringify({
      tier: 'Pre-Release Tier',
      state: 'evidence_collected_gate_blocked',
      collectionSummary: { pendingSlots: 0, collectedSlots: 7, requiredSlots: 7 },
      evidenceSlots: [{ id: 'connection_metrics', status: 'collected' }],
    }),
    'utf8',
  );
  await writeFile(
    path.join(evidenceDir, 'capacity-tier-evidence-index.json'),
    '{"tier":"Capacity Tier",',
    'utf8',
  );
  const logs = createLoggerCapture();

  const result = await runCommercialReadiness({
    repoRoot: tempRepoRoot,
    logger: logs.logger,
    runCheck: async (check) => ({
      ...check,
      ok: true,
      exitCode: 0,
    }),
  });

  assert.equal(result.ok, false);
  assert.equal(result.exitCode, COMMAND_FAILURE_EXIT_CODE);
  assert.equal(result.capacityAssessment, null);
  assert.equal(result.preReleaseAssessment, null);
  assert.equal(result.checks.length, buildCommercialReadinessChecks({ repoRoot: tempRepoRoot }).length);
  assert.equal(result.failure.stage, 'capacity-evidence-load');
  assert.match(result.failure.summary, /JSON/i);
  assert.match(result.failure.evidenceIndexPath, /capacity-tier-evidence-index\.json$/);
  assert.match(logs.stderr.join('\n'), /capacity-evidence-load/i);
  assert.match(logs.stderr.join('\n'), /JSON/i);
});

function createLoggerCapture() {
  const stdout = [];
  const stderr = [];

  return {
    stdout,
    stderr,
    logger: {
      log(message) {
        stdout.push(String(message));
      },
      error(message) {
        stderr.push(String(message));
      },
    },
  };
}
