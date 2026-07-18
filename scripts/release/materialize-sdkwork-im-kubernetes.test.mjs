import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  loadImageInventory,
  renderCloudManifests,
  validateImageLock,
} from './materialize-sdkwork-im-kubernetes.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');
const sourceRevision = 'a'.repeat(40);

function fixtureLock() {
  const inventory = loadImageInventory(repoRoot);
  return {
    schemaVersion: 1,
    releaseId: 'test-fixture-1',
    sourceRevision,
    images: Object.fromEntries(
      Object.entries(inventory.images).map(([service, repository], index) => [
        service,
        {
          repository,
          digest: `sha256:${(index + 1).toString(16).padStart(64, '0')}`,
        },
      ]),
    ),
  };
}

test('materializes every cloud Deployment with an immutable image digest', () => {
  const temporaryRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-k8s-test-'));
  try {
    const outputRoot = path.join(temporaryRoot, 'cloud');
    const bundle = renderCloudManifests({
      repoRoot,
      imageLock: fixtureLock(),
      outputRoot,
      expectedRevision: sourceRevision,
    });

    assert.equal(bundle.releaseId, 'test-fixture-1');
    assert.ok(bundle.files.length > 0);
    for (const file of bundle.files.filter(({ path: filePath }) => /\.ya?ml$/u.test(filePath))) {
      const content = readFileSync(path.join(outputRoot, file.path), 'utf8');
      assert.doesNotMatch(content, /image:\s*[^\s]+:(?:latest|main|master)\b/u);
    }

    const inventory = loadImageInventory(repoRoot);
    const rendered = bundle.files
      .filter(({ path: filePath }) => /\.ya?ml$/u.test(filePath))
      .map(({ path: filePath }) => readFileSync(path.join(outputRoot, filePath), 'utf8'))
      .join('\n');
    for (const [service, repository] of Object.entries(inventory.images)) {
      assert.match(rendered, new RegExp(`name: ${service}\\b`, 'u'));
      assert.match(rendered, new RegExp(`${repository.replaceAll('/', '\\/')}@sha256:[a-f0-9]{64}`, 'u'));
    }

    const persistedBundle = JSON.parse(
      readFileSync(path.join(outputRoot, 'bundle-manifest.json'), 'utf8'),
    );
    assert.deepEqual(persistedBundle, bundle);
    assert.ok(bundle.files.every((file) => /^[a-f0-9]{64}$/u.test(file.sha256)));
  } finally {
    rmSync(temporaryRoot, { recursive: true, force: true });
  }
});

test('rejects incomplete, mutable, or repository-substituted locks', () => {
  const inventory = loadImageInventory(repoRoot);

  const incomplete = fixtureLock();
  delete incomplete.images['streaming-service'];
  assert.throws(() => validateImageLock(incomplete, inventory), /service set/u);

  const mutable = fixtureLock();
  mutable.images['streaming-service'].digest = 'latest';
  assert.throws(() => validateImageLock(mutable, inventory), /64 lowercase hex/u);

  const substituted = fixtureLock();
  substituted.images['streaming-service'].repository = 'attacker.invalid/streaming';
  assert.throws(() => validateImageLock(substituted, inventory), /repository/u);
});
