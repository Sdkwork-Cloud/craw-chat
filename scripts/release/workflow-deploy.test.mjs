import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { createDeployPlan } from './workflow-deploy.mjs';

let tempRoot;

test.beforeEach(() => {
  tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-workflow-deploy-'));
});
test.afterEach(() => rmSync(tempRoot, { recursive: true, force: true }));

test('creates an explicit deployctl apply plan from immutable evidence', () => {
  const evidencePath = path.join(tempRoot, '.sdkwork', 'artifacts', '.sdkwork', 'evidence', 'demo.json');
  mkdirSync(path.dirname(evidencePath), { recursive: true });
  writeFileSync(evidencePath, JSON.stringify({
    artifactId: 'demo-0.1.0',
    digest: `sha256:${'a'.repeat(64)}`,
  }));
  const plan = createDeployPlan({
    env: {
      SDKWORK_DEPLOYMENT_PROFILE: 'standalone',
      SDKWORK_DEPLOY_ENVIRONMENT: 'production',
      SDKWORK_ARTIFACT_EVIDENCE_PATH: evidencePath,
      SDKWORK_DEPLOY_ROLLBACK_TARGET: 'release-0.0.9',
      SDKWORK_DEPLOY_APPROVAL_REF: 'change-1234',
    },
    root: tempRoot,
  });
  assert.ok(plan.args.includes('standalone.production'));
  assert.ok(plan.args.includes('release-0.0.9'));
  assert.ok(plan.args.includes('change-1234'));
});

test('rejects deployment before side effects when approval selection is absent', () => {
  const evidencePath = path.join(tempRoot, 'evidence.json');
  mkdirSync(tempRoot, { recursive: true });
  writeFileSync(evidencePath, JSON.stringify({ artifactId: 'demo', digest: `sha256:${'b'.repeat(64)}` }));
  assert.throws(
    () => createDeployPlan({
      env: {
        SDKWORK_DEPLOYMENT_PROFILE: 'cloud',
        SDKWORK_DEPLOY_ENVIRONMENT: 'production',
        SDKWORK_ARTIFACT_EVIDENCE_PATH: evidencePath,
        SDKWORK_DEPLOY_ROLLBACK_TARGET: 'release-0.0.9',
      },
      root: tempRoot,
    }),
    /SDKWORK_DEPLOY_APPROVAL_REF/u,
  );
});
