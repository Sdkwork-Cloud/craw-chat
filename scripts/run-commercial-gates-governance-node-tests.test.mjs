import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');
const fixtureRepoRoot = repoRoot;
const governanceCatalog = await import(
  pathToFileURL(
    path.join(repoRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
  ).href,
);

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'run-commercial-gates-governance-node-tests.mjs'),
    ).href,
  );
}

test('commercial gates governance node test runner exposes the governed test set and canonical node plan', async () => {
  const module = await loadModule();

  assert.equal(typeof module.listCommercialGatesGovernanceNodeTests, 'function');
  assert.equal(typeof module.createCommercialGatesGovernanceNodeTestPlan, 'function');

  const testFiles = module.listCommercialGatesGovernanceNodeTests();
  assert.deepEqual(
    testFiles,
    governanceCatalog.listCommercialGatesGovernanceNodeTestFiles(),
  );

  const plan = module.createCommercialGatesGovernanceNodeTestPlan({
    cwd: fixtureRepoRoot,
    env: { SDKWORK_RELEASE_MODE: '1' },
    nodeExecutable: 'node-custom',
  });
  assert.equal(plan.command, 'node-custom');
  assert.deepEqual(
    plan.args,
    ['--test', '--experimental-test-isolation=none', ...testFiles],
  );
  assert.equal(plan.cwd, fixtureRepoRoot);
  assert.deepEqual(plan.env, { SDKWORK_RELEASE_MODE: '1' });
  assert.equal(plan.shell, false);
  assert.equal(plan.windowsHide, process.platform === 'win32');
});

test('commercial gates governance node test runner imports the governed catalog as its single test-list source', async () => {
  const runnerSource = await readFile(
    path.join(repoRoot, 'scripts', 'run-commercial-gates-governance-node-tests.mjs'),
    'utf8',
  );

  assert.match(runnerSource, /commercial-gates-governance-node-test-catalog\.mjs/);
  assert.doesNotMatch(runnerSource, /export const COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS\b/);
});

test('commercial gates governance node test runner executes the canonical node test command through spawnSync', async () => {
  const module = await loadModule();

  const calls = [];
  const result = module.runCommercialGatesGovernanceNodeTests({
    cwd: fixtureRepoRoot,
    env: { SDKWORK_ENV: '1' },
    nodeExecutable: 'node-custom',
    spawnSyncImpl(command, args, options) {
      calls.push({ command, args, options });
      return {
        status: 0,
        stdout: '',
        stderr: '',
      };
    },
  });

  assert.equal(result.status, 0);
  assert.deepEqual(calls, [
    {
      command: 'node-custom',
      args: [
        '--test',
        '--experimental-test-isolation=none',
        ...module.listCommercialGatesGovernanceNodeTests(),
      ],
      options: {
        cwd: fixtureRepoRoot,
        env: { SDKWORK_ENV: '1' },
        shell: false,
        stdio: 'inherit',
        windowsHide: process.platform === 'win32',
      },
    },
  ]);
});
