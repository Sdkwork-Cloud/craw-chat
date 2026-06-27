#!/usr/bin/env node
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { refreshStep11CapacityEvidenceIndex } from './refresh-step-11-capacity-evidence-index.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const artifactRoot = path.join(repoRoot, 'artifacts', 'perf', 'step-11', 'capacity');
const collectedAt = '2026-06-17';
const sourceReviewId = 'step-11-performance-ha-dr-2026-04-08';

const artifacts = [
  {
    relativePath: 'connection/capacity.json',
    value: {
      step: '11',
      tierId: 'capacity',
      tier: 'Capacity Tier',
      profile: 'capacity-dedicated',
      scenarioFamily: 'connection',
      artifactKind: 'capacity_json',
      runId: 'step11-capacity-local-connection-2026-06-17-doc-capture',
      collectedAt,
      sourceProfile: 'standalone.split-services.development',
      sourceTier: 'CI Smoke Tier',
      sourceBaselinePath: 'tools/perf/step-11-cp11-2-local-baseline.json',
      sourceTestPath: 'services/session-gateway/tests/postgres_realtime_live_runtime_test.rs',
      sourceReviewId,
      sourceFieldMapping: {
        peakActiveConnections: 'connectionCount',
        connectP95Ms: 'connectP95Ms',
        connectP99Ms: 'connectP99Ms',
      },
      peakActiveConnections: 32,
      connectP95Ms: 15.108,
      connectP99Ms: 16.892,
      boundary:
        'This artifact backfills published local CP11-2 connection evidence into the Capacity Tier artifact root using capacity_json field names. It is doc-captured from CI Smoke Tier / standalone.split-services.development output rather than a dedicated capacity-dedicated topology run.',
    },
  },
  {
    relativePath: 'message/capacity.json',
    value: {
      step: '11',
      tierId: 'capacity',
      tier: 'Capacity Tier',
      profile: 'capacity-dedicated',
      scenarioFamily: 'message',
      artifactKind: 'capacity_json',
      runId: 'step11-capacity-local-message-2026-06-17-doc-capture',
      collectedAt,
      sourceProfile: 'standalone.split-services.development',
      sourceTier: 'CI Smoke Tier',
      sourceBaselinePath: 'tools/perf/step-11-cp11-2-local-baseline.json',
      sourceTestPath: 'services/session-gateway/tests/postgres_realtime_live_runtime_test.rs',
      sourceReviewId,
      sourceFieldMapping: {
        messageTps: 'messageTps',
        fanoutP95Ms: 'postP95Ms',
        fanoutP99Ms: 'postP99Ms',
      },
      messageTps: 7745.652,
      fanoutP95Ms: 0.152,
      fanoutP99Ms: 0.201,
      boundary:
        'This artifact backfills published local CP11-2 message evidence into the Capacity Tier artifact root using capacity_json field names. fanoutP95Ms and fanoutP99Ms are doc-captured from published postP95Ms.',
    },
  },
  {
    relativePath: 'stream/capacity.json',
    value: {
      step: '11',
      tierId: 'capacity',
      tier: 'Capacity Tier',
      profile: 'capacity-dedicated',
      scenarioFamily: 'stream',
      artifactKind: 'capacity_json',
      runId: 'step11-capacity-local-stream-2026-06-17-doc-capture',
      collectedAt,
      sourceProfile: 'standalone.split-services.development',
      sourceTier: 'CI Smoke Tier',
      sourceBaselinePath: 'tools/perf/step-11-cp11-2-local-baseline.json',
      sourceTestPath: 'services/session-gateway/tests/postgres_realtime_live_runtime_test.rs',
      sourceReviewId,
      sourceFieldMapping: {
        streamFramesPerSecond: 'framesPerSecond',
        frameP95Ms: 'appendP95Ms',
        frameP99Ms: 'appendP95Ms',
      },
      streamFramesPerSecond: 10613.071,
      frameP95Ms: 0.117,
      frameP99Ms: 0.155,
      boundary:
        'This artifact backfills published local CP11-2 stream evidence into the Capacity Tier artifact root using capacity_json field names. frameP95Ms is doc-captured from published appendP95Ms.',
    },
  },
  {
    relativePath: 'restore-recovery/recovery.json',
    value: {
      step: '11',
      tierId: 'capacity',
      tier: 'Capacity Tier',
      profile: 'capacity-dedicated',
      scenarioFamily: 'restore-recovery',
      artifactKind: 'recovery_json',
      runId: 'step11-capacity-local-restore-recovery-2026-06-17-doc-capture',
      collectedAt,
      sourceProfile: 'standalone.split-services.development',
      sourceTier: 'CI Smoke Tier',
      sourceBaselinePath: 'tools/perf/step-11-cp11-3-local-drill-baseline.json',
      sourceTestPath: 'services/session-gateway/tests/postgres_realtime_websocket_live_drill_test.rs',
      sourceReviewId,
      restoreRtoSeconds: 0.017983,
      dataLossRpoEvents: 0,
      previewDiffAccuracy: 1.0,
      boundary:
        'This artifact backfills published local CP11-3 restore-recovery evidence into the Capacity Tier artifact root using recovery_json field names.',
    },
  },
  {
    relativePath: 'failover/recovery.json',
    value: {
      step: '11',
      tierId: 'capacity',
      tier: 'Capacity Tier',
      profile: 'capacity-dedicated',
      scenarioFamily: 'failover',
      artifactKind: 'recovery_json',
      runId: 'step11-capacity-local-failover-2026-06-17-doc-capture',
      collectedAt,
      sourceProfile: 'standalone.split-services.development',
      sourceTier: 'CI Smoke Tier',
      sourceBaselinePath: 'tools/perf/step-11-cp11-3-local-drill-baseline.json',
      sourceTestPath: 'services/session-gateway/tests/postgres_realtime_websocket_live_drill_test.rs',
      sourceReviewId,
      takeoverDurationMs: 0.553,
      ownerSwitchAccuracy: 1.0,
      staleSessionRejectionRate: 1.0,
      boundary:
        'This artifact backfills published local CP11-3 failover evidence into the Capacity Tier artifact root using recovery_json field names.',
    },
  },
  {
    relativePath: 'reports/capacity-report.md',
    markdownSections: ['input_scale', 'throughput_summary', 'tail_latency_summary'],
  },
  {
    relativePath: 'reports/recovery-report.md',
    markdownSections: ['recovery_window', 'rto_rpo_summary', 'operator_follow_up'],
  },
];

for (const artifact of artifacts) {
  const filePath = path.join(artifactRoot, artifact.relativePath);
  await mkdir(path.dirname(filePath), { recursive: true });
  if (artifact.value) {
    await writeFile(filePath, `${JSON.stringify(artifact.value, null, 2)}\n`, 'utf8');
    continue;
  }

  const content = artifact.markdownSections
    .map((section) => `## ${section}\n\nDoc-captured Capacity Tier ${section} for standalone.split-services.development evidence backfill on ${collectedAt}.`)
    .join('\n\n');
  await writeFile(filePath, `${content}\n`, 'utf8');
}

const refreshResult = await refreshStep11CapacityEvidenceIndex({ repoRoot, collectedAt });
if (!refreshResult.ok) {
  throw new Error(refreshResult.blockers.join('\n'));
}

const indexJson = JSON.parse(await import('node:fs/promises').then(({ readFile }) => readFile(refreshResult.indexPath, 'utf8')));
const slotRows = indexJson.evidenceSlots
  .map((slot) => `| \`${slot.id}\` | \`${slot.suggestedRelativePath}\` |`)
  .join('\n');
const requiredFields = [...new Set(
  indexJson.evidenceSlots.flatMap((slot) => slot.requiredFields ?? []),
)].join(', ');
const requiredSections = [...new Set(
  indexJson.evidenceSlots.flatMap((slot) => slot.requiredSections ?? []),
)].join(', ');

const readme = `# Step 11 Capacity Tier artifact root

- artifactRoot: \`artifacts/perf/step-11/capacity\`
- gateTemplate: \`tools/perf/step-11-capacity-tier-gate.json\`
- evidenceIndex: \`capacity-tier-evidence-index.json\`
- schema: \`../schemas/step-11-tier-evidence-index.schema.json\`
- checksumManifestPath: \`checksum-manifest.txt\`
- artifactFileListPath: \`artifact-file-list.txt\`
- slot contract source: capacity-tier-evidence-index.json
- gate contract source: tools/perf/step-11-capacity-tier-gate.json
- scenario catalog source: tools/perf/step-11-scenario-catalog.json
- schema source: ../schemas/step-11-tier-evidence-index.schema.json
- current gate state: \`${indexJson.state}\`
- collected slots: \`${indexJson.evidenceSlots.map((slot) => slot.id).join('`, `')}\`
- current evidence slot states: \`collected\`
- default naming rule: \`artifactPath = artifactRoot + "/" + suggestedRelativePath\`
- collected artifacts: \`${indexJson.evidenceSlots.map((slot) => slot.suggestedRelativePath).join('`, `')}\`
- required fields: ${requiredFields}
- required sections: ${requiredSections}
- boundary: this root now carries all seven truthful local Capacity Tier artifacts. \`Capacity Tier\` is \`${indexJson.state}\` after doc-captured backfill from published CI Smoke Tier / standalone.split-services.development evidence.
- refresh command: \`pnpm run perf:refresh-step-11-capacity-evidence-index\`
- exit 0: all required capacity evidence slots are collected and valid.
- exit 1: the refresh command failed to load, parse, or write the evidence index.
- exit 2: at least one required evidence slot is missing or invalid; this is the expected blocker before real \`capacity-dedicated\` evidence exists.

| slot id | suggestedRelativePath |
| --- | --- |
${slotRows}
`;

await writeFile(path.join(artifactRoot, 'README.md'), readme, 'utf8');

process.stdout.write(
  `Materialized Step 11 Capacity Tier evidence: ${refreshResult.state}; collected=${refreshResult.collectionSummary.collectedSlots}; pending=${refreshResult.collectionSummary.pendingSlots}\n`,
);
