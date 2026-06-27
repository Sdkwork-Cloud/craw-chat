import assert from 'node:assert/strict';
import { mkdir, mkdtemp, readFile, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  refreshStep11CapacityEvidenceIndex,
} from './refresh-step-11-capacity-evidence-index.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');
const capacityIndexPath = path.join(
  repoRoot,
  'artifacts',
  'perf',
  'step-11',
  'capacity',
  'capacity-tier-evidence-index.json',
);

test('refreshStep11CapacityEvidenceIndex collects complete capacity artifacts without fabricating results', async () => {
  const tempRoot = await createCapacityFixtureRoot('complete');
  await writeCapacityArtifact(tempRoot, 'connection/capacity.json', {
    runId: 'cap-connection-2026-06-12',
    peakActiveConnections: 12000,
    connectP95Ms: 180,
    connectP99Ms: 260,
  });
  await writeCapacityArtifact(tempRoot, 'message/capacity.json', {
    runId: 'cap-message-2026-06-12',
    messageTps: 4500,
    fanoutP95Ms: 210,
    fanoutP99Ms: 320,
  });
  await writeCapacityArtifact(tempRoot, 'stream/capacity.json', {
    runId: 'cap-stream-2026-06-12',
    streamFramesPerSecond: 9800,
    frameP95Ms: 95,
    frameP99Ms: 145,
  });
  await writeCapacityArtifact(tempRoot, 'restore-recovery/recovery.json', {
    runId: 'cap-restore-2026-06-12',
    restoreRtoSeconds: 42,
    dataLossRpoEvents: 0,
    previewDiffAccuracy: 1,
  });
  await writeCapacityArtifact(tempRoot, 'failover/recovery.json', {
    runId: 'cap-failover-2026-06-12',
    takeoverDurationMs: 1400,
    ownerSwitchAccuracy: 1,
    staleSessionRejectionRate: 1,
  });
  await writeReportArtifact(tempRoot, 'reports/capacity-report.md', [
    'input_scale',
    'throughput_summary',
    'tail_latency_summary',
  ]);
  await writeReportArtifact(tempRoot, 'reports/recovery-report.md', [
    'recovery_window',
    'rto_rpo_summary',
    'operator_follow_up',
  ]);

  const result = await refreshStep11CapacityEvidenceIndex({ repoRoot: tempRoot, collectedAt: '2026-06-12' });
  const refreshedIndex = JSON.parse(await readFile(result.indexPath, 'utf8'));
  const checksumManifest = await readFile(result.checksumManifestPath, 'utf8');
  const artifactFileList = await readFile(result.artifactFileListPath, 'utf8');

  assert.equal(result.ok, true);
  assert.equal(refreshedIndex.state, 'evidence_collected_gate_passed');
  assert.equal(refreshedIndex.collectionSummary.collectedSlots, 7);
  assert.equal(refreshedIndex.collectionSummary.pendingSlots, 0);
  assert.equal(refreshedIndex.evidenceSlots.every((slot) => slot.status === 'collected'), true);
  for (const slot of refreshedIndex.evidenceSlots) {
    assert.equal(slot.collectedAt, '2026-06-12');
    assert.match(slot.checksumSha256, /^sha256:[a-f0-9]{64}$/u);
    assert.equal(typeof slot.sizeBytes, 'number');
    assert.equal(slot.artifactPath, path.posix.join(refreshedIndex.artifactRoot, slot.suggestedRelativePath));
    assert.match(checksumManifest, new RegExp(`${slot.checksumSha256.replace(':', ':')}  ${escapeRegExp(slot.suggestedRelativePath)}`));
    assert.match(artifactFileList, new RegExp(`^${escapeRegExp(slot.suggestedRelativePath)}$`, 'mu'));
  }
});

test('refreshStep11CapacityEvidenceIndex leaves missing capacity artifacts pending with blockers', async () => {
  const tempRoot = await createCapacityFixtureRoot('missing');
  await writeCapacityArtifact(tempRoot, 'connection/capacity.json', {
    runId: 'cap-connection-2026-06-12',
    peakActiveConnections: 12000,
    connectP95Ms: 180,
    connectP99Ms: 260,
  });

  const result = await refreshStep11CapacityEvidenceIndex({ repoRoot: tempRoot, collectedAt: '2026-06-12' });
  const refreshedIndex = JSON.parse(await readFile(result.indexPath, 'utf8'));

  assert.equal(result.ok, false);
  assert.equal(refreshedIndex.state, 'evidence_partially_collected');
  assert.equal(refreshedIndex.collectionSummary.collectedSlots, 1);
  assert.equal(refreshedIndex.collectionSummary.pendingSlots, 6);
  assert.match(result.blockers.join('\n'), /message_capacity/);
  assert.equal(refreshedIndex.evidenceSlots.find((slot) => slot.id === 'connection_capacity').status, 'collected');
  assert.equal(refreshedIndex.evidenceSlots.find((slot) => slot.id === 'message_capacity').status, 'pending_collection');
});

test('repository exposes the capacity evidence refresh command and schema accepts readiness pass state', async () => {
  const packageJson = JSON.parse(await readFile(path.join(repoRoot, 'package.json'), 'utf8'));
  const schemaJson = JSON.parse(await readFile(
    path.join(repoRoot, 'artifacts', 'perf', 'step-11', 'schemas', 'step-11-tier-evidence-index.schema.json'),
    'utf8',
  ));

  assert.equal(
    packageJson.scripts['perf:refresh-step-11-capacity-evidence-index'],
    'node scripts/perf/refresh-step-11-capacity-evidence-index.mjs',
  );
  assert.equal(schemaJson.properties.state.enum.includes('evidence_collected_gate_passed'), true);
});

test('capacity artifact README stays synchronized with evidence index contract and CLI exit codes', async () => {
  const indexJson = JSON.parse(await readFile(capacityIndexPath, 'utf8'));
  const readme = await readFile(
    path.join(repoRoot, 'artifacts', 'perf', 'step-11', 'capacity', 'README.md'),
    'utf8',
  );

  assert.match(readme, /slot contract source: capacity-tier-evidence-index\.json/u);
  assert.match(readme, /gate contract source: tools\/perf\/step-11-capacity-tier-gate\.json/u);
  assert.match(readme, /scenario catalog source: tools\/perf\/step-11-scenario-catalog\.json/u);
  assert.match(readme, /schema source: \.\.\/schemas\/step-11-tier-evidence-index\.schema\.json/u);
  assert.match(readme, /exit 0: all required capacity evidence slots are collected and valid\./u);
  assert.match(readme, /exit 1: the refresh command failed to load, parse, or write the evidence index\./u);
  assert.match(
    readme,
    /exit 2: at least one required evidence slot is missing or invalid; this is the expected blocker before real `capacity-dedicated` evidence exists\./u,
  );

  for (const slot of indexJson.evidenceSlots) {
    assert.equal(readme.includes(`| \`${slot.id}\` | \`${slot.suggestedRelativePath}\` |`), true);
    for (const requiredField of slot.requiredFields ?? []) {
      assert.match(readme, new RegExp(`\\b${escapeRegExp(requiredField)}\\b`, 'u'));
    }
    for (const requiredSection of slot.requiredSections ?? []) {
      assert.match(readme, new RegExp(`\\b${escapeRegExp(requiredSection)}\\b`, 'u'));
    }
  }
});

async function createCapacityFixtureRoot(name) {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), `capacity-evidence-${name}-`));
  const targetIndexPath = path.join(
    tempRoot,
    'artifacts',
    'perf',
    'step-11',
    'capacity',
    'capacity-tier-evidence-index.json',
  );
  await mkdir(path.dirname(targetIndexPath), { recursive: true });
  await writeFile(targetIndexPath, await readFile(capacityIndexPath, 'utf8'), 'utf8');

  return tempRoot;
}

async function writeCapacityArtifact(repoRoot, relativePath, value) {
  const filePath = path.join(repoRoot, 'artifacts', 'perf', 'step-11', 'capacity', relativePath);
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function writeReportArtifact(repoRoot, relativePath, sections) {
  const filePath = path.join(repoRoot, 'artifacts', 'perf', 'step-11', 'capacity', relativePath);
  await mkdir(path.dirname(filePath), { recursive: true });
  const content = sections.map((section) => `## ${section}\n\nCollected evidence for ${section}.`).join('\n\n');
  await writeFile(filePath, `${content}\n`, 'utf8');
}

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}
