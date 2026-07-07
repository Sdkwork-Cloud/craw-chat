import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

export const COMMAND_FAILURE_EXIT_CODE = 1;
export const READINESS_BLOCKED_EXIT_CODE = 2;

const SHA256_CHECKSUM_PATTERN = /^(?:sha256:)?[a-f0-9]{64}$/iu;
const STORE_CONTROLLED_SOURCE_TYPES = new Set(['APP_STORE', 'MARKETPLACE', 'STORE']);
const SIGNATURE_EVIDENCE_PATHS = [
  ['signature'],
  ['signing'],
  ['signingEvidence'],
  ['notarization'],
  ['metadata', 'signature'],
  ['metadata', 'signing'],
  ['metadata', 'signingEvidence'],
  ['metadata', 'notarization'],
];
const SBOM_EVIDENCE_PATHS = [
  ['sbom'],
  ['sbomRef'],
  ['sbomUrl'],
  ['sbomPath'],
  ['metadata', 'sbom'],
  ['metadata', 'sbomRef'],
  ['metadata', 'sbomUrl'],
  ['metadata', 'sbomPath'],
];
const PROVENANCE_EVIDENCE_PATHS = [
  ['provenance'],
  ['provenanceRef'],
  ['provenanceUrl'],
  ['provenancePath'],
  ['attestation'],
  ['attestationRef'],
  ['metadata', 'provenance'],
  ['metadata', 'provenanceRef'],
  ['metadata', 'attestation'],
  ['metadata', 'attestationRef'],
  ['metadata', 'artifactAttestation'],
];

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
    CI: 'true',
    npm_config_update_notifier: 'false',
  };

  return [
    {
      id: 'pc-install',
      label: 'Sdkwork IM workspace frozen install',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['install', '--frozen-lockfile', '--lockfile-only', '--ignore-scripts'],
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
      id: 'chat-drive-upload-attribution-standard',
      label: 'IM chat Drive upload attribution standard (PC/H5/Flutter)',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-chat-drive-upload-attribution-standard.test.mjs'],
    },
    {
      id: 'production-security-standard',
      label: 'IM production security standard (JWT secret, principal bootstrap)',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-production-security-standard.test.mjs'],
    },
    {
      id: 'app-context-module-standard',
      label: 'IM app-context single-source module standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-app-context-module-standard.test.mjs'],
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
      id: 'pc-voice-app-sdk-integration',
      label: 'Sdkwork IM PC voice app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:voice-app-sdk-integration'],
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
      id: 'social-materializer-standard',
      label: 'Social materializer transactional batch standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-social-materializer-standard.test.mjs'],
    },
    {
      id: 'space-materializer-standard',
      label: 'Space materializer transactional batch standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-space-materializer-standard.test.mjs'],
    },
    {
      id: 'monorepo-frozen-install-standard',
      label: 'Monorepo frozen install workspace alignment standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-monorepo-frozen-install-standard.test.mjs'],
    },
    {
      id: 'pc-client-pagination-standard',
      label: 'Sdkwork IM PC client pagination standard',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-pc-client-pagination-standard.test.mjs'],
    },
    {
      id: 'rtc-signaling-boundary-standard',
      label: 'IM RTC signaling boundary standard (no embedded media stack in IM repo)',
      cwd: repoRoot,
      command: nodeExecutable,
      args: ['scripts/dev/sdkwork-im-rtc-signaling-boundary.test.mjs'],
    },
    {
      id: 'projection-tier-standard',
      label: 'Sdkwork IM projection tier standard',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:projection-tier-standard'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'three-capabilities-standard',
      label: 'Sdkwork IM weak-network / 10K group / desktop offline alignment standard',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:three-capabilities-standard'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'portal-alignment-standard',
      label: 'Sdkwork IM portal aggregation alignment standard',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:portal-alignment-standard'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'portal-service-tests',
      label: 'Portal service HTTP smoke and snapshot unit tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'im-portal-snapshots', '-p', 'portal-service', '--test', 'http_smoke_test'],
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
      id: 'calls-service-tests',
      label: 'IM calls service tests (epoch + fencing, participant authorization)',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'calls-service', '--lib'],
    },
    {
      id: 'im-domain-core-tests',
      label: 'IM domain core tests (RtcSession epoch merge)',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'im-domain-core', '--lib'],
    },
    {
      id: 'streaming-service-tests',
      label: 'Streaming service tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'streaming-service', '--tests'],
    },
    {
      id: 'social-service-tests',
      label: 'Social service tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'social-service', '--tests'],
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
    passedStates: ['evidence_collected_gate_passed'],
    passedSummary: (tier) => `${tier} evidence is fully collected and ready for pre-release sign-off.`,
  });
}

export function assessAppReleaseEvidence(appManifest) {
  const blockers = [];
  const checksumRequired = appManifest?.security?.checksumRequired === true;
  const signatureRequired = appManifest?.security?.signatureRequired === true;
  const sbomRequired = appManifest?.security?.sbomRequired === true;
  const packages = Array.isArray(appManifest?.artifacts?.installConfig?.packages)
    ? appManifest.artifacts.installConfig.packages
    : [];

  packages.forEach((releasePackage, index) => {
    if (!isEnabledManifestEntry(releasePackage) || isStoreControlledPackage(releasePackage)) {
      return;
    }

    const packageId = formatManifestEntryId(releasePackage, `package[${index}]`);
    if (checksumRequired) {
      const checksum = typeof releasePackage.checksum === 'string'
        ? releasePackage.checksum.trim()
        : '';
      if (!SHA256_CHECKSUM_PATTERN.test(checksum)) {
        blockers.push(
          `${packageId} is an enabled direct distribution package but checksum is missing or not a SHA-256 value.`,
        );
      }
    }

    if (signatureRequired && !hasPackageEvidence(releasePackage, SIGNATURE_EVIDENCE_PATHS)) {
      blockers.push(
        `${packageId} is an enabled direct distribution package but signature evidence is missing while security.signatureRequired=true.`,
      );
    }

    if (sbomRequired && !hasPackageEvidence(releasePackage, SBOM_EVIDENCE_PATHS)) {
      blockers.push(
        `${packageId} is an enabled direct distribution package but SBOM evidence is missing while security.sbomRequired=true.`,
      );
    }

    if (sbomRequired && !hasPackageEvidence(releasePackage, PROVENANCE_EVIDENCE_PATHS)) {
      blockers.push(
        `${packageId} is an enabled direct distribution package but provenance or attestation evidence is missing while security.sbomRequired=true.`,
      );
    }
  });

  for (const mediaAsset of collectEnabledMediaAssets(appManifest)) {
    if (mediaAsset.asset.metadata?.generatedPlaceholder === true) {
      blockers.push(
        `${mediaAsset.id} at ${mediaAsset.location} still has metadata.generatedPlaceholder=true and cannot be used as commercial release media evidence.`,
      );
    }
  }

  if (blockers.length > 0) {
    return {
      ok: false,
      summary: `App release evidence has ${blockers.length} blocker(s).`,
      blockers,
    };
  }

  return {
    ok: true,
    summary: 'App release evidence is complete for the current manifest gate.',
    blockers: [],
  };
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

export function resolveAppManifestPath(repoRoot = resolveRepoRoot()) {
  return path.join(repoRoot, 'sdkwork.app.config.json');
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

export async function loadAppManifest(repoRoot = resolveRepoRoot()) {
  const appManifestPath = resolveAppManifestPath(repoRoot);
  const source = await readFile(appManifestPath, 'utf8');

  return {
    appManifestPath,
    manifestJson: JSON.parse(source),
  };
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
        appReleaseAssessment: null,
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
        appReleaseAssessment: null,
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
        appReleaseAssessment: null,
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
        appReleaseAssessment: null,
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

  const tierAssessmentResults = Object.fromEntries(
    tierAssessments.map(({ resultKey, evidenceIndexPath, assessment }) => [
      resultKey,
      {
        ...assessment,
        evidenceIndexPath,
      },
    ]),
  );

  let appManifest;
  try {
    appManifest = await loadAppManifest(repoRoot);
  } catch (error) {
    const appManifestPath = resolveAppManifestPath(repoRoot);
    const summary = formatErrorSummary(error);
    logger.error(
      `[commercial-readiness] failed to load app-release-evidence manifest ${appManifestPath}: ${summary}`,
    );
    return {
      ok: false,
      exitCode: COMMAND_FAILURE_EXIT_CODE,
      checks: results,
      ...tierAssessmentResults,
      appReleaseAssessment: null,
      failure: {
        stage: 'app-release-evidence-load',
        summary,
        appManifestPath,
      },
    };
  }

  const appReleaseAssessment = assessAppReleaseEvidence(appManifest.manifestJson);
  if (!appReleaseAssessment.ok) {
    logger.error(`[commercial-readiness] blocked by appReleaseAssessment: ${appReleaseAssessment.summary}`);
    for (const blocker of appReleaseAssessment.blockers) {
      logger.error(`[commercial-readiness] ${blocker}`);
    }

    return {
      ok: false,
      exitCode: READINESS_BLOCKED_EXIT_CODE,
      checks: results,
      ...tierAssessmentResults,
      appReleaseAssessment: {
        ...appReleaseAssessment,
        appManifestPath: appManifest.appManifestPath,
      },
    };
  }

  logger.log(`[commercial-readiness] ${appReleaseAssessment.summary}`);

  return {
    ok: true,
    exitCode: 0,
    checks: results,
    ...tierAssessmentResults,
    appReleaseAssessment: {
      ...appReleaseAssessment,
      appManifestPath: appManifest.appManifestPath,
    },
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

function collectEnabledMediaAssets(appManifest) {
  const media = appManifest?.media;
  const assets = [];

  appendMediaAsset(assets, 'media.icons.primary', media?.icons?.primary);
  appendMediaAssets(assets, 'media.icons.platform', media?.icons?.platform);
  appendMediaAssets(assets, 'media.screenshots', media?.screenshots);
  appendMediaAssets(assets, 'media.previews', media?.previews);

  return assets;
}

function appendMediaAssets(assets, location, mediaAssets) {
  if (!Array.isArray(mediaAssets)) {
    return;
  }

  mediaAssets.forEach((asset, index) => {
    appendMediaAsset(assets, `${location}[${index}]`, asset);
  });
}

function appendMediaAsset(assets, location, asset) {
  if (!isManifestObject(asset) || !isEnabledManifestEntry(asset)) {
    return;
  }

  assets.push({
    asset,
    id: formatManifestEntryId(asset, location),
    location,
  });
}

function isStoreControlledPackage(releasePackage) {
  const sourceType = typeof releasePackage?.sourceType === 'string'
    ? releasePackage.sourceType.toUpperCase()
    : '';

  return STORE_CONTROLLED_SOURCE_TYPES.has(sourceType);
}

function isEnabledManifestEntry(entry) {
  return isManifestObject(entry) && entry.enabled !== false;
}

function isManifestObject(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function formatManifestEntryId(entry, fallback) {
  return typeof entry?.id === 'string' && entry.id.length > 0 ? entry.id : fallback;
}

function hasPackageEvidence(releasePackage, evidencePaths) {
  return evidencePaths.some((segments) => hasEvidenceValue(readNestedValue(releasePackage, segments)));
}

function readNestedValue(source, segments) {
  let value = source;
  for (const segment of segments) {
    if (!isManifestObject(value) || !(segment in value)) {
      return undefined;
    }
    value = value[segment];
  }

  return value;
}

function hasEvidenceValue(value) {
  if (typeof value === 'string') {
    return value.trim().length > 0;
  }
  if (Array.isArray(value)) {
    return value.length > 0;
  }
  if (isManifestObject(value)) {
    return Object.keys(value).length > 0;
  }

  return false;
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
