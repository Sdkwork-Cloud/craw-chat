import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';

import {
  loadCloudServiceBuilds,
  validateCloudImageBuildInput,
} from './build-sdkwork-im-cloud-image.mjs';
import { loadImageInventory } from './materialize-sdkwork-im-kubernetes.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');

test('cloud build inventory exactly covers the governed image inventory', () => {
  const builds = loadCloudServiceBuilds(repoRoot);
  const images = loadImageInventory(repoRoot);
  assert.deepEqual(Object.keys(builds.services).sort(), Object.keys(images.images).sort());
  for (const [service, build] of Object.entries(builds.services)) {
    assert.match(build.package, /^[a-z0-9][a-z0-9-]+$/u, service);
    assert.match(build.binary, /^[a-z0-9][a-z0-9-]+$/u, service);
    assert.ok(Number.isInteger(build.port) && build.port > 0 && build.port <= 65535, service);
  }
});

test('cloud image build rejects mutable bases and latest output tags', () => {
  const inventory = loadCloudServiceBuilds(repoRoot);
  const valid = {
    service: 'streaming-service',
    runtimeImage: `registry.example/runtime@sha256:${'a'.repeat(64)}`,
    tag: 'ghcr.io/sdkwork/streaming-service:1.0.0-rc.1',
  };
  assert.equal(validateCloudImageBuildInput(valid, inventory).binary, 'streaming-service');
  assert.throws(
    () => validateCloudImageBuildInput({ ...valid, runtimeImage: 'debian:bookworm-slim' }, inventory),
    /pinned by sha256/u,
  );
  assert.throws(
    () => validateCloudImageBuildInput({ ...valid, tag: 'ghcr.io/sdkwork/streaming-service:latest' }, inventory),
    /non-latest/u,
  );
});
