#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const CAPACITY_INDEX_RELATIVE_PATH = 'artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json';
const INDEX_FILE_NAME = 'capacity-tier-evidence-index.json';
const CHECKSUM_MANIFEST_FILE_NAME = 'checksum-manifest.txt';
const ARTIFACT_FILE_LIST_NAME = 'artifact-file-list.txt';

export async function refreshStep11CapacityEvidenceIndex({
  repoRoot = resolveRepoRoot(),
  collectedAt = currentIsoDate(),
  dryRun = false,
} = {}) {
  const normalizedRepoRoot = path.resolve(repoRoot);
  const indexPath = path.join(normalizedRepoRoot, CAPACITY_INDEX_RELATIVE_PATH);
  const indexJson = JSON.parse(await readFile(indexPath, 'utf8'));
  const artifactRootRelative = toPosixPath(indexJson.artifactRoot);
  const artifactRoot = path.resolve(normalizedRepoRoot, artifactRootRelative);
  const blockers = [];
  const refreshedSlots = [];

  for (const slot of indexJson.evidenceSlots ?? []) {
    const refreshedSlot = await refreshEvidenceSlot({
      slot,
      repoRoot: normalizedRepoRoot,
      artifactRoot,
      artifactRootRelative,
      collectedAt,
      blockers,
    });
    refreshedSlots.push(refreshedSlot);
  }

  const collectionSummary = buildCollectionSummary(refreshedSlots);
  const refreshedIndex = {
    ...indexJson,
    state: resolveCollectionState(collectionSummary),
    collectionSummary,
    evidenceSlots: refreshedSlots,
  };

  const checksumManifestPath = path.resolve(normalizedRepoRoot, refreshedIndex.checksumManifestPath);
  const artifactFileListPath = path.resolve(normalizedRepoRoot, refreshedIndex.artifactFileListPath);
  const checksumManifest = buildChecksumManifest(refreshedIndex);
  const artifactFileList = buildArtifactFileList(refreshedIndex);

  if (!dryRun) {
    await mkdir(path.dirname(indexPath), { recursive: true });
    await writeFile(indexPath, `${JSON.stringify(refreshedIndex, null, 2)}\n`, 'utf8');
    await writeFile(checksumManifestPath, checksumManifest, 'utf8');
    await writeFile(artifactFileListPath, artifactFileList, 'utf8');
  }

  return {
    ok: collectionSummary.pendingSlots === 0,
    blockers,
    state: refreshedIndex.state,
    indexPath,
    checksumManifestPath,
    artifactFileListPath,
    collectionSummary,
  };
}

async function refreshEvidenceSlot({ slot, repoRoot, artifactRoot, artifactRootRelative, collectedAt, blockers }) {
  const suggestedRelativePath = toPosixPath(slot.suggestedRelativePath);
  const artifactPath = path.resolve(artifactRoot, suggestedRelativePath);
  assertInsideRoot({ root: artifactRoot, target: artifactPath, label: slot.id });

  if (!existsSync(artifactPath)) {
    blockers.push(`${slot.id} is missing ${toPosixPath(path.relative(repoRoot, artifactPath))}.`);
    return pendingSlot(slot);
  }

  const validation = await validateSlotArtifact(slot, artifactPath);
  if (!validation.ok) {
    blockers.push(`${slot.id} failed validation: ${validation.errors.join('; ')}.`);
    return pendingSlot(slot);
  }

  const fileStats = await stat(artifactPath);
  const checksumSha256 = await sha256File(artifactPath);

  return {
    ...slot,
    status: 'collected',
    artifactPath: path.posix.join(artifactRootRelative, suggestedRelativePath),
    collectedAt,
    sizeBytes: fileStats.size,
    checksumSha256,
  };
}

async function validateSlotArtifact(slot, artifactPath) {
  if (Array.isArray(slot.requiredFields) && slot.requiredFields.length > 0) {
    return validateJsonFields(slot, artifactPath);
  }
  if (Array.isArray(slot.requiredSections) && slot.requiredSections.length > 0) {
    return validateMarkdownSections(slot, artifactPath);
  }

  return { ok: true, errors: [] };
}

async function validateJsonFields(slot, artifactPath) {
  const errors = [];
  let parsed;
  try {
    parsed = JSON.parse(await readFile(artifactPath, 'utf8'));
  } catch (error) {
    return { ok: false, errors: [`artifact must contain valid JSON (${formatErrorMessage(error)})`] };
  }

  for (const field of slot.requiredFields) {
    if (!Object.hasOwn(parsed, field) || parsed[field] === null) {
      errors.push(`missing required field ${field}`);
    }
  }

  return { ok: errors.length === 0, errors };
}

async function validateMarkdownSections(slot, artifactPath) {
  const source = await readFile(artifactPath, 'utf8');
  const errors = [];
  for (const section of slot.requiredSections) {
    const sectionPattern = new RegExp(`^#{1,6}\\s+${escapeRegExp(section)}\\s*$`, 'mu');
    if (!sectionPattern.test(source)) {
      errors.push(`missing required section ${section}`);
    }
  }

  return { ok: errors.length === 0, errors };
}

function pendingSlot(slot) {
  return {
    ...slot,
    status: 'pending_collection',
    artifactPath: null,
    collectedAt: null,
    sizeBytes: null,
    checksumSha256: null,
  };
}

function buildCollectionSummary(slots) {
  const totalSlots = slots.length;
  const requiredSlots = slots.filter((slot) => slot.required === true).length;
  const optionalSlots = totalSlots - requiredSlots;
  const collectedSlots = slots.filter((slot) => slot.status === 'collected').length;
  const skippedOptionalSlots = slots.filter((slot) => slot.status === 'skipped_optional').length;
  const pendingSlots = slots.filter((slot) => slot.required === true && slot.status === 'pending_collection').length;

  return {
    totalSlots,
    requiredSlots,
    optionalSlots,
    collectedSlots,
    pendingSlots,
    skippedOptionalSlots,
  };
}

function resolveCollectionState(collectionSummary) {
  if (collectionSummary.pendingSlots === 0) {
    return 'evidence_collected_gate_passed';
  }
  if (collectionSummary.collectedSlots > 0) {
    return 'evidence_partially_collected';
  }

  return 'template_only_pending_execution';
}

function buildChecksumManifest(indexJson) {
  const lines = [
    '# Step 11 Capacity Tier checksum manifest',
    `# state: ${indexJson.state}`,
  ];
  for (const slot of indexJson.evidenceSlots) {
    if (slot.status === 'collected') {
      lines.push(`${slot.checksumSha256}  ${slot.suggestedRelativePath}`);
    }
  }
  lines.push('');

  return lines.join('\n');
}

function buildArtifactFileList(indexJson) {
  const lines = [
    '# Step 11 Capacity Tier artifact file list',
    `# state: ${indexJson.state}`,
    INDEX_FILE_NAME,
    CHECKSUM_MANIFEST_FILE_NAME,
    ARTIFACT_FILE_LIST_NAME,
  ];
  for (const slot of indexJson.evidenceSlots) {
    if (slot.status === 'collected') {
      lines.push(slot.suggestedRelativePath);
    }
  }
  lines.push('');

  return lines.join('\n');
}

async function sha256File(filePath) {
  const source = await readFile(filePath);
  const digest = createHash('sha256').update(source).digest('hex');

  return `sha256:${digest}`;
}

function assertInsideRoot({ root, target, label }) {
  const relativePath = path.relative(root, target);
  if (relativePath.startsWith('..') || path.isAbsolute(relativePath)) {
    throw new Error(`${label} artifact path escapes artifact root: ${target}`);
  }
}

function parseArgs(argv) {
  const parsed = {};
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--repo-root') {
      parsed.repoRoot = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--collected-at') {
      parsed.collectedAt = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--dry-run') {
      parsed.dryRun = true;
    }
  }

  return parsed;
}

function currentIsoDate() {
  return new Date().toISOString().slice(0, 10);
}

function resolveRepoRoot() {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
}

function toPosixPath(value) {
  return String(value).replaceAll('\\', '/');
}

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}

function formatErrorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

function isDirectExecution() {
  return process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);
}

if (isDirectExecution()) {
  try {
    const result = await refreshStep11CapacityEvidenceIndex(parseArgs(process.argv.slice(2)));
    process.stdout.write(
      `Step 11 Capacity Tier evidence refresh state: ${result.state}; collected=${result.collectionSummary.collectedSlots}; pending=${result.collectionSummary.pendingSlots}\n`,
    );
    for (const blocker of result.blockers) {
      process.stderr.write(`${blocker}\n`);
    }
    process.exitCode = result.ok ? 0 : 2;
  } catch (error) {
    process.stderr.write(`Step 11 Capacity Tier evidence refresh failed: ${formatErrorMessage(error)}\n`);
    process.exitCode = 1;
  }
}
