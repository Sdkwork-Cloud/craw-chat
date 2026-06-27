import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

export const COMMAND_FAILURE_EXIT_CODE = 1;
export const READINESS_BLOCKED_EXIT_CODE = 2;

export function resolvePnpmExecutable(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

export function resolveFlutterExecutable(platform = process.platform) {
  return platform === 'win32' ? 'flutter.bat' : 'flutter';
}

export function shouldUseShellForCommand(command, platform = process.platform) {
  const normalized = String(command ?? '').trim();
  return platform === 'win32' && /\.(cmd|bat)$/i.test(normalized);
}

export function buildCommercialReadinessChecks({
  repoRoot = resolveRepoRoot(),
  platform = process.platform,
} = {}) {
  const pnpmExecutable = resolvePnpmExecutable(platform);
  const flutterExecutable = resolveFlutterExecutable(platform);
  const nodeExecutable = process.execPath;
  const pnpmRuntimeEnv = {
    npm_config_update_notifier: 'false',
  };

  return [
    {
      id: 'pc-install',
      label: 'Sdkwork IM workspace frozen install',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['install', '--frozen-lockfile', '--ignore-scripts'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-lint',
      label: 'Sdkwork IM PC typecheck',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'lint'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-build',
      label: 'Sdkwork IM PC production build',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'build'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'h5-lint',
      label: 'Sdkwork IM H5 typecheck',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-h5'),
      command: pnpmExecutable,
      args: ['run', 'lint'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'h5-build',
      label: 'Sdkwork IM H5 production build',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-h5'),
      command: pnpmExecutable,
      args: ['run', 'build'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'h5-architecture-standard',
      label: 'Sdkwork IM H5 architecture standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-h5-architecture-standard.test.mjs'],
    },
    {
      id: 'flutter-mobile-architecture-standard',
      label: 'Sdkwork IM Flutter mobile architecture standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-flutter-mobile-architecture-standard.test.mjs'],
    },
    {
      id: 'im-sdk-flutter-composed-test',
      label: 'Sdkwork IM Flutter composed realtime unit tests',
      cwd: path.join(repoRoot, 'sdks', 'sdkwork-im-sdk', 'sdkwork-im-sdk-flutter', 'composed', 'im_sdk_composed'),
      command: flutterExecutable,
      args: ['test'],
    },
    {
      id: 'flutter-mobile-analyze',
      label: 'Sdkwork IM Flutter mobile analyze',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-flutter-mobile'),
      command: flutterExecutable,
      args: ['analyze', '--no-fatal-infos'],
    },
    {
      id: 'flutter-mobile-test',
      label: 'Sdkwork IM Flutter mobile widget tests',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-flutter-mobile'),
      command: flutterExecutable,
      args: ['test'],
    },
    {
      id: 'pc-e2e-smoke',
      label: 'Sdkwork IM PC production e2e smoke',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-pc-e2e-smoke.test.mjs'],
    },
    {
      id: 'pc-playwright-e2e',
      label: 'Sdkwork IM PC Playwright production shell + authenticated chat e2e',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-pc-playwright-e2e.test.mjs'],
    },
    {
      id: 'pc-auth-appbase-ui-contract',
      label: 'Sdkwork IM PC appbase auth UI contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: nodeExecutable,
      args: ['scripts/auth-appbase-ui-contract.test.mjs'],
    },
    {
      id: 'pc-domain-app-sdk-auth-runtime',
      label: 'Sdkwork IM PC domain app SDK auth runtime contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:domain-app-sdk-auth-runtime'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-notary-app-sdk-integration',
      label: 'Sdkwork IM PC notary app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:notary-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-drive-app-sdk-integration',
      label: 'Sdkwork IM PC drive app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:drive-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-knowledgebase-app-sdk-integration',
      label: 'Sdkwork IM PC knowledgebase app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:knowledgebase-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-commerce-app-sdk-integration',
      label: 'Sdkwork IM PC commerce app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:commerce-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-mail-app-sdk-integration',
      label: 'Sdkwork IM PC mail app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:mail-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-community-app-sdk-integration',
      label: 'Sdkwork IM PC community app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:community-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-course-app-sdk-integration',
      label: 'Sdkwork IM PC course app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:course-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-aiot-devices-sdk-integration',
      label: 'Sdkwork IM PC AIoT devices SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:aiot-devices-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-qr-scan-standard',
      label: 'Sdkwork IM PC QR scan standard contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:qr-scan-standard'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'step11-scenario-catalog',
      label: 'Step 11 scenario catalog contract',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-step11-scenario-catalog.test.mjs'],
    },
    {
      id: 'step11-ha-dr-drill',
      label: 'Step 11 HA/DR local drill gate',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/run-step11-ha-dr-drill.mjs'],
    },
    {
      id: 'commercial-deployment-contract',
      label: 'Commercial deployment contract (K8s, staging, observability, dependabot)',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-commercial-deployment-contract.test.mjs'],
    },
    {
      id: 'topology-baggage',
      label: 'Topology v2 baggage contract',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:topology-baggage'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'dependency-management',
      label: 'SDKWork dependency management standard',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'check:dependency-management'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'workflow-commercial-gates',
      label: 'Workflow commercial governance gates',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:workflow-commercial-gates'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'retention-enforcement-standard',
      label: 'IM retention enforcement governance contract',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-retention-enforcement-standard.test.mjs'],
    },
    {
      id: 'observability-bootstrap-standard',
      label: 'IM observability bootstrap governance contract',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-observability-bootstrap-standard.test.mjs'],
    },
    {
      id: 'im-app-sdk-flutter-parity',
      label: 'Sdkwork IM app SDK Flutter/TypeScript parity',
      cwd: path.join(repoRoot, 'sdks', 'sdkwork-im-app-sdk'),
      command: nodeExecutable,
      args: ['bin/verify-sdk.mjs'],
    },
    {
      id: 'governance-service-tests',
      label: 'Governance service tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'governance-service', '--tests'],
    },
    {
      id: 'gateway-integration-tests',
      label: 'Sdkwork IM gateway integration tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'sdkwork-im-cloud-gateway', '--tests'],
    },
    {
      id: 'session-gateway-tests',
      label: 'Session gateway tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'session-gateway', '--tests'],
    },
  ];
}

export function assessCapacityEvidenceIndex(indexJson) {
  return assessStep11TierEvidenceIndex(indexJson, {
    tierLabel: 'Capacity Tier',
    passedStates: ['evidence_collected_gate_passed'],
    passedSummary: (tier) => `${tier} evidence is fully collected and ready for commercial sign-off.`,
  });
}

export function assessPreReleaseEvidenceIndex(indexJson) {
  return assessStep11TierEvidenceIndex(indexJson, {
    tierLabel: 'Pre-Release Tier',
    passedStates: ['evidence_collected_gate_blocked', 'evidence_collected_gate_passed'],
    passedSummary: (tier, state) => state === 'evidence_collected_gate_passed'
      ? `${tier} evidence is fully collected and ready for pre-release sign-off.`
      : `${tier} evidence is fully collected but remains gate-blocked pending dedicated pre-release topology sign-off.`,
  });
}

function assessStep11TierEvidenceIndex(indexJson, options) {
  const tier = typeof indexJson?.tier === 'string' ? indexJson.tier : options.tierLabel;
  const state = typeof indexJson?.state === 'string' ? indexJson.state : 'unknown';
  const pendingSlots = normalizePendingSlots(indexJson?.collectionSummary?.pendingSlots);
  const collectedSlots = normalizePendingSlots(indexJson?.collectionSummary?.collectedSlots);
  const requiredSlots = normalizePendingSlots(indexJson?.collectionSummary?.requiredSlots);
  const pendingEvidenceIds = Array.isArray(indexJson?.evidenceSlots)
    ? indexJson.evidenceSlots
      .filter((slot) => slot?.status === 'pending_collection')
      .map((slot) => slot?.id)
      .filter((slotId) => typeof slotId === 'string' && slotId.length > 0)
    : [];

  if (state === 'template_only_pending_execution') {
    return {
      ok: false,
      summary: `${tier} remains ${state} with ${pendingSlots} pending slots.`,
      blockers: pendingEvidenceIds.length > 0
        ? pendingEvidenceIds.map((slotId) => `${slotId} is still pending collection.`)
        : [`${tier} is still template-only and must not be treated as collected evidence.`],
    };
  }

  if (pendingSlots > 0) {
    return {
      ok: false,
      summary: `${tier} remains ${state} with ${pendingSlots} pending slots.`,
      blockers: pendingEvidenceIds.map((slotId) => `${slotId} is still pending collection.`),
    };
  }

  if (
    options.passedStates.includes(state)
    && (requiredSlots === 0 || collectedSlots >= requiredSlots)
  ) {
    return {
      ok: true,
      summary: options.passedSummary(tier, state),
      blockers: [],
    };
  }

  return {
    ok: false,
    summary: `${tier} remains ${state} with incomplete collected evidence (${collectedSlots}/${requiredSlots}).`,
    blockers: [`${tier} state ${state} is not an accepted commercial readiness outcome.`],
  };
}

export function resolveCapacityEvidenceIndexPath(repoRoot = resolveRepoRoot()) {
  return resolveStep11TierEvidenceIndexPath(repoRoot, 'capacity', 'capacity-tier-evidence-index.json');
}

export function resolvePreReleaseEvidenceIndexPath(repoRoot = resolveRepoRoot()) {
  return resolveStep11TierEvidenceIndexPath(repoRoot, 'pre-release', 'pre-release-tier-evidence-index.json');
}

function resolveStep11TierEvidenceIndexPath(repoRoot, tierId, fileName) {
  return path.join(
    repoRoot,
    'artifacts',
    'perf',
    'step-11',
    tierId,
    fileName,
  );
}

export async function loadCapacityEvidenceIndex(repoRoot = resolveRepoRoot()) {
  return loadStep11TierEvidenceIndex(resolveCapacityEvidenceIndexPath(repoRoot));
}

export async function loadPreReleaseEvidenceIndex(repoRoot = resolveRepoRoot()) {
  return loadStep11TierEvidenceIndex(resolvePreReleaseEvidenceIndexPath(repoRoot));
}

async function loadStep11TierEvidenceIndex(evidenceIndexPath) {
  const source = await readFile(evidenceIndexPath, 'utf8');

  return {
    evidenceIndexPath,
    indexJson: JSON.parse(source),
  };
}

export async function runCommercialReadiness({
  repoRoot = resolveRepoRoot(),
  platform = process.platform,
  logger = console,
  runCheck = executeCheck,
} = {}) {
  const checks = buildCommercialReadinessChecks({ repoRoot, platform });
  const results = [];

  for (const check of checks) {
    logger.log(`[commercial-readiness] running ${check.id}: ${formatCommand(check)}`);
    let result;
    try {
      result = await runCheck(check);
    } catch (error) {
      const summary = formatErrorSummary(error);
      logger.error(`[commercial-readiness] failed ${check.id} due to execution error: ${summary}`);
      return {
        ok: false,
        exitCode: COMMAND_FAILURE_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        preReleaseAssessment: null,
        failure: {
          stage: check.id,
          summary,
        },
      };
    }

    results.push(result);
    if (!result.ok) {
      logger.error(`[commercial-readiness] failed ${check.id} with exit code ${result.exitCode}.`);
      return {
        ok: false,
        exitCode: COMMAND_FAILURE_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        preReleaseAssessment: null,
        failure: {
          stage: check.id,
          summary: `exit code ${result.exitCode}`,
        },
      };
    }

    logger.log(`[commercial-readiness] passed ${check.id}`);
  }

  const tierAssessments = [];
  for (const tierGate of [
    {
      stage: 'pre-release-evidence-load',
      load: () => loadPreReleaseEvidenceIndex(repoRoot),
      resolvePath: () => resolvePreReleaseEvidenceIndexPath(repoRoot),
      assess: assessPreReleaseEvidenceIndex,
      resultKey: 'preReleaseAssessment',
    },
    {
      stage: 'capacity-evidence-load',
      load: () => loadCapacityEvidenceIndex(repoRoot),
      resolvePath: () => resolveCapacityEvidenceIndexPath(repoRoot),
      assess: assessCapacityEvidenceIndex,
      resultKey: 'capacityAssessment',
    },
  ]) {
    let evidenceIndex;
    try {
      evidenceIndex = await tierGate.load();
    } catch (error) {
      const evidenceIndexPath = tierGate.resolvePath();
      const summary = formatErrorSummary(error);
      logger.error(
        `[commercial-readiness] failed to load ${tierGate.stage} index ${evidenceIndexPath}: ${summary}`,
      );
      return {
        ok: false,
        exitCode: COMMAND_FAILURE_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        preReleaseAssessment: null,
        failure: {
          stage: tierGate.stage,
          summary,
          evidenceIndexPath,
        },
      };
    }

    const { evidenceIndexPath, indexJson } = evidenceIndex;
    const assessment = tierGate.assess(indexJson);
    tierAssessments.push({
      resultKey: tierGate.resultKey,
      evidenceIndexPath,
      assessment,
    });

    if (!assessment.ok) {
      logger.error(`[commercial-readiness] blocked by ${tierGate.resultKey}: ${assessment.summary}`);
      for (const blocker of assessment.blockers) {
        logger.error(`[commercial-readiness] ${blocker}`);
      }

      return {
        ok: false,
        exitCode: READINESS_BLOCKED_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        preReleaseAssessment: null,
        ...Object.fromEntries(
          tierAssessments.map(({ resultKey, evidenceIndexPath, assessment: tierAssessment }) => [
            resultKey,
            {
              ...tierAssessment,
              evidenceIndexPath,
            },
          ]),
        ),
      };
    }

    logger.log(`[commercial-readiness] ${assessment.summary}`);
  }

  return {
    ok: true,
    exitCode: 0,
    checks: results,
    ...Object.fromEntries(
      tierAssessments.map(({ resultKey, evidenceIndexPath, assessment }) => [
        resultKey,
        {
          ...assessment,
          evidenceIndexPath,
        },
      ]),
    ),
  };
}

async function executeCheck(check) {
  if (!existsSync(check.cwd)) {
    throw new Error(`configured cwd does not exist: ${check.cwd}`);
  }

  const exitCode = await spawnCommand(check.command, check.args, {
    cwd: check.cwd,
    env: check.env,
    stdio: 'inherit',
  });

  return {
    ...check,
    ok: exitCode === 0,
    exitCode,
  };
}

function spawnCommand(command, args, options) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      ...options,
      env: options?.env ? { ...process.env, ...options.env } : process.env,
      shell: shouldUseShellForCommand(command, process.platform),
    });

    child.once('error', reject);
    child.once('exit', (code, signal) => {
      if (signal) {
        resolve(1);
        return;
      }

      resolve(code ?? 1);
    });
  });
}

function formatCommand(check) {
  return [check.command, ...check.args].join(' ');
}

function normalizePendingSlots(value) {
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed >= 0 ? parsed : 0;
}

function formatErrorSummary(error) {
  if (error instanceof Error && typeof error.message === 'string' && error.message.length > 0) {
    return error.message;
  }

  return String(error);
}

function resolveRepoRoot() {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const result = await runCommercialReadiness();
  process.exitCode = result.exitCode;
}
