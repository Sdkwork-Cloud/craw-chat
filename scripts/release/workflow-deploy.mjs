#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const MODULE_PATH = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(MODULE_PATH), '..', '..');
const DEPLOYCTL = path.resolve(REPO_ROOT, '..', 'sdkwork-specs', 'tools', 'deployctl.mjs');
const PROFILE = /^(standalone|cloud)$/u;
const ENVIRONMENT = /^(test|staging|production)$/u;

function required(env, key) {
  const value = String(env[key] ?? '').trim();
  if (!value) throw new Error(`${key} is required for side-effecting deployment`);
  return value;
}

function createDeployPlan({ env = process.env, root = REPO_ROOT } = {}) {
  const deploymentProfile = required(env, 'SDKWORK_DEPLOYMENT_PROFILE');
  const environment = required(env, 'SDKWORK_DEPLOY_ENVIRONMENT');
  if (!PROFILE.test(deploymentProfile)) throw new Error('SDKWORK_DEPLOYMENT_PROFILE must be standalone or cloud');
  if (!ENVIRONMENT.test(environment)) throw new Error('SDKWORK_DEPLOY_ENVIRONMENT must be test, staging, or production');
  const evidencePath = path.resolve(required(env, 'SDKWORK_ARTIFACT_EVIDENCE_PATH'));
  if (!existsSync(evidencePath)) throw new Error(`artifact evidence does not exist: ${evidencePath}`);
  const evidence = JSON.parse(readFileSync(evidencePath, 'utf8'));
  const artifactId = String(evidence.artifactId ?? '').trim();
  const digest = String(evidence.digest ?? '').trim();
  if (!artifactId) throw new Error('artifact evidence artifactId is required');
  if (!/^sha256:[a-f0-9]{64}$/u.test(digest)) throw new Error('artifact evidence digest must be immutable sha256');
  const artifactRoot = path.resolve(root, '.sdkwork', 'artifacts');
  return {
    command: process.execPath,
    args: [
      DEPLOYCTL,
      'apply',
      '--root', root,
      '--profile', `${deploymentProfile}.${environment}`,
      '--environment', environment,
      '--artifact-id', artifactId,
      '--artifact-digest', digest,
      '--artifact-evidence', evidencePath,
      '--artifact-root', artifactRoot,
      '--rollback-target', required(env, 'SDKWORK_DEPLOY_ROLLBACK_TARGET'),
      '--approval-ref', required(env, 'SDKWORK_DEPLOY_APPROVAL_REF'),
    ],
    cwd: root,
    evidencePath,
  };
}

function runDeployPlan(plan, { spawn = spawnSync } = {}) {
  const result = spawn(plan.command, plan.args, {
    cwd: plan.cwd,
    env: process.env,
    shell: false,
    stdio: 'inherit',
  });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`deployctl apply failed with exit code ${result.status ?? 1}`);
}

async function main() {
  const plan = createDeployPlan();
  runDeployPlan(plan);
  return 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === MODULE_PATH) {
  main().then((code) => { process.exitCode = code; }).catch((error) => {
    console.error(`[sdkwork-im-workflow-deploy] ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  });
}

export { createDeployPlan, runDeployPlan };
