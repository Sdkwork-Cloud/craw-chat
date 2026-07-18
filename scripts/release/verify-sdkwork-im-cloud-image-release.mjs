#!/usr/bin/env node

import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { renderCloudManifests } from './materialize-sdkwork-im-kubernetes.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function requireCleanRevision() {
  const status = execFileSync('git', ['status', '--porcelain'], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  if (status.trim()) {
    throw new Error('cloud image release verification requires a clean Git worktree');
  }
  return execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: repoRoot,
    encoding: 'utf8',
  }).trim();
}

try {
  const imageLockPath = String(process.env.SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE ?? '').trim();
  if (!imageLockPath) {
    throw new Error(
      'SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE is required and must reference build-produced image digests',
    );
  }

  const imageLock = JSON.parse(readFileSync(path.resolve(imageLockPath), 'utf8'));
  const temporaryRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-cloud-release-'));
  try {
    const bundle = renderCloudManifests({
      repoRoot,
      imageLock,
      outputRoot: path.join(temporaryRoot, 'cloud'),
      expectedRevision: requireCleanRevision(),
    });
    console.log(
      `[cloud-image-release] verified ${bundle.files.length} immutable files for ${bundle.releaseId}`,
    );
  } finally {
    rmSync(temporaryRoot, { recursive: true, force: true });
  }
} catch (error) {
  console.error(`[cloud-image-release] ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}
