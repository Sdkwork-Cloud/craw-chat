import assert from 'node:assert/strict';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');
const governanceCatalog = await import(
  pathToFileURL(
    path.join(repoRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
  ).href,
);
const watchCatalog = await import(
  pathToFileURL(
    path.join(repoRoot, 'scripts', 'im-commercial-gates-watch-catalog.mjs'),
  ).href,
);
const stepContractCatalog = await import(
  pathToFileURL(
    path.join(repoRoot, 'scripts', 'im-commercial-gates-step-contract-catalog.mjs'),
  ).href,
);

const DEFAULT_COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS =
  governanceCatalog.listCommercialGatesGovernanceNodeTestFiles();
const DEFAULT_IM_COMMERCIAL_GATES_WORKFLOW_WATCH_PATHS =
  watchCatalog.listImCommercialGatesWorkflowWatchPaths();
const DEFAULT_IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS =
  stepContractCatalog.listImCommercialGatesWorkflowStepContracts();

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function escapeRegexLiteral(value) {
  return String(value).replace(/[|\\{}()[\]^$+*?.]/g, '\\$&');
}

function createFixtureRoot() {
  const fixtureRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-commercial-gates-workflow-'));
  mkdirSync(path.join(fixtureRoot, '.github', 'workflows'), { recursive: true });
  mkdirSync(path.join(fixtureRoot, 'scripts'), { recursive: true });
  return fixtureRoot;
}

function createCommercialGatesGovernanceCatalogFixtureSource({
  testFiles = DEFAULT_COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS,
} = {}) {
  return `
const COMMERCIAL_GATES_GOVERNANCE_NODE_TEST_FILES = ${JSON.stringify(testFiles, null, 2)};

export function listCommercialGatesGovernanceNodeTestFiles() {
  return [...COMMERCIAL_GATES_GOVERNANCE_NODE_TEST_FILES];
}
`;
}

function createCommercialGatesGovernanceRunnerFixtureSource({
  testFiles = DEFAULT_COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS,
} = {}) {
  return `
import { listCommercialGatesGovernanceNodeTestFiles } from './commercial-gates-governance-node-test-catalog.mjs';

export function listCommercialGatesGovernanceNodeTests() {
  return listCommercialGatesGovernanceNodeTestFiles();
}

export function createCommercialGatesGovernanceNodeTestPlan({
  cwd = '.',
  env = {},
  nodeExecutable = 'node',
} = {}) {
  return {
    command: nodeExecutable,
    args: ['--test', '--experimental-test-isolation=none', ...listCommercialGatesGovernanceNodeTests()],
    cwd,
    env,
    shell: false,
    windowsHide: false,
  };
}

export function runCommercialGatesGovernanceNodeTests() {
  return { status: 0 };
}
`;
}

function writeImCommercialGatesFixture({
  workflowText = read('.github/workflows/im-commercial-gates.yml'),
  packageJson = JSON.parse(read('package.json')),
  testFiles = DEFAULT_COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS,
} = {}) {
  const fixtureRoot = createFixtureRoot();
  writeFileSync(
    path.join(fixtureRoot, '.github', 'workflows', 'im-commercial-gates.yml'),
    workflowText,
    'utf8',
  );
  writeFileSync(
    path.join(fixtureRoot, 'package.json'),
    JSON.stringify(packageJson, null, 2),
    'utf8',
  );
  writeFileSync(
    path.join(fixtureRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
    createCommercialGatesGovernanceCatalogFixtureSource({ testFiles }),
    'utf8',
  );
  writeFileSync(
    path.join(fixtureRoot, 'scripts', 'run-commercial-gates-governance-node-tests.mjs'),
    createCommercialGatesGovernanceRunnerFixtureSource({ testFiles }),
    'utf8',
  );
  return fixtureRoot;
}

async function loadContracts() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'im-commercial-gates-contracts.mjs'),
    ).href,
  );
}

test('repository exposes a governed im commercial gates workflow with repository-owned node governance entrypoints', async () => {
  const workflowPath = path.join(repoRoot, '.github', 'workflows', 'im-commercial-gates.yml');
  assert.equal(existsSync(workflowPath), true, 'missing .github/workflows/im-commercial-gates.yml');

  const packageJson = JSON.parse(read('package.json'));
  const workflow = read('.github/workflows/im-commercial-gates.yml');
  const contractSource = read('scripts/im-commercial-gates-contracts.mjs');

  assert.equal(
    packageJson.scripts['check:commercial-readiness'],
    'node scripts/release/commercial-readiness.mjs',
  );
  assert.equal(
    packageJson.scripts['test:workflow-commercial-gates'],
    'node scripts/run-commercial-gates-governance-node-tests.mjs',
  );
  assert.match(workflow, /pull_request:\s*[\s\S]*?paths:/);
  assert.match(workflow, /workflow_dispatch:/);
  assert.match(workflow, /permissions:\s*contents:\s*read/);
  assert.doesNotMatch(
    workflow,
    /^\s+(?:contents|id-token|attestations|artifact-metadata|packages):\s*write$/m,
  );
  assert.match(workflow, /FORCE_JAVASCRIPT_ACTIONS_TO_NODE24:\s*'true'/);
  assert.match(workflow, /actions\/checkout@v5/);
  assert.match(workflow, /actions\/setup-node@v5/);
  assert.match(
    workflow,
    /CARGO_TARGET_DIR:\s*\$\{\{\s*runner\.temp\s*\}\}\/sdkwork-cargo-target/u,
    'Windows cargo target directory must use the GitHub runner temp directory instead of a machine-specific drive path',
  );
  assert.doesNotMatch(
    workflow,
    /\b[A-Z]:[\\/](?!Program Files(?: \(x86\))?[\\/]Git[\\/])/u,
    'workflow source/build paths must not contain machine-specific absolute drive paths',
  );
  for (const watchedPath of DEFAULT_IM_COMMERCIAL_GATES_WORKFLOW_WATCH_PATHS) {
    assert.match(workflow, new RegExp(escapeRegexLiteral(watchedPath)));
  }
  for (const contract of DEFAULT_IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS) {
    assert.match(workflow, new RegExp(contract.patternSource));
  }
  assert.doesNotMatch(
    workflow,
    /apps\/control-plane/,
    'im commercial gates workflow must not reference the retired control-plane path',
  );
  assert.doesNotMatch(
    workflow,
    /apps\/sdkwork-im-admin|apps\/sdkwork-im-portal/,
    'im commercial gates workflow must not reference retired admin or portal app paths',
  );
  assert.match(contractSource, /im-commercial-gates-watch-catalog\.mjs/);
  assert.match(contractSource, /im-commercial-gates-step-contract-catalog\.mjs/);

  const contracts = await loadContracts();
  await assert.doesNotReject(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot,
    }),
  );
});

test('im commercial gates workflow contract helper rejects workflows that omit the explicit read-only token permissions', async () => {
  const contracts = await loadContracts();

  const fixtureRoot = writeImCommercialGatesFixture({
    workflowText: read('.github/workflows/im-commercial-gates.yml').replace(
      /permissions:\r?\n\s+contents:\s*read\r?\n\r?\n/,
      '',
    ),
  });

  await assert.rejects(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot: fixtureRoot,
    }),
    /read-only GITHUB_TOKEN baseline|permissions/i,
  );
});

test('im commercial gates workflow contract helper rejects workflows that inline commercial governance tests instead of the repository runner', async () => {
  const contracts = await loadContracts();

  const fixtureRoot = writeImCommercialGatesFixture({
    workflowText: read('.github/workflows/im-commercial-gates.yml').replace(
      /run:\s*node scripts\/run-commercial-gates-governance-node-tests\.mjs/,
      `run: node --test ${DEFAULT_COMMERCIAL_GATES_GOVERNANCE_NODE_TESTS.join(' ')}`,
    ),
  });

  await assert.rejects(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot: fixtureRoot,
    }),
    /repository-owned runner|run-commercial-gates-governance-node-tests/i,
  );
});

test('im commercial gates workflow contract helper rejects workflows that still reference the retired control-plane path', async () => {
  const contracts = await loadContracts();

  const fixtureRoot = writeImCommercialGatesFixture({
    workflowText: `${read('.github/workflows/im-commercial-gates.yml')}

      - name: Retired control-plane path
        working-directory: apps/control-plane
        run: node --version
`,
  });

  await assert.rejects(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot: fixtureRoot,
    }),
    /retired control-plane path|apps\/control-plane/i,
  );
});

test('im commercial gates workflow contract helper rejects workflows that still reference retired admin or portal paths', async () => {
  const contracts = await loadContracts();

  const fixtureRoot = writeImCommercialGatesFixture({
    workflowText: `${read('.github/workflows/im-commercial-gates.yml')}

      - name: Retired portal path
        working-directory: apps/sdkwork-im-portal
        run: node --version
`,
  });

  await assert.rejects(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot: fixtureRoot,
    }),
    /retired admin or portal app paths|apps\/sdkwork-im-portal/i,
  );
});

test('im commercial gates workflow contract helper rejects workflows that do not watch the repository-owned governance runner inputs', async () => {
  const contracts = await loadContracts();

  const fixtureRoot = writeImCommercialGatesFixture({
    workflowText: read('.github/workflows/im-commercial-gates.yml')
      .replace(/^.*scripts\/run-commercial-gates-governance-node-tests\.mjs.*\r?\n/gm, '')
      .replace(/^.*scripts\/run-commercial-gates-governance-node-tests\.test\.mjs.*\r?\n/gm, ''),
  });

  await assert.rejects(
    contracts.assertImCommercialGatesWorkflowContracts({
      repoRoot: fixtureRoot,
    }),
    /governance node test runner|run-commercial-gates-governance-node-tests/i,
  );
});
