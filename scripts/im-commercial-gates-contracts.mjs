import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { listImCommercialGatesWorkflowStepContracts } from './im-commercial-gates-step-contract-catalog.mjs';
import { listImCommercialGatesWorkflowWatchRequirements } from './im-commercial-gates-watch-catalog.mjs';

function read(repoRoot, relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function escapeRegexLiteral(value) {
  return String(value).replace(/[|\\{}()[\]^$+*?.]/g, '\\$&');
}

function createLiteralPattern(value) {
  return new RegExp(escapeRegexLiteral(value));
}

export async function assertImCommercialGatesWorkflowContracts({
  repoRoot,
} = {}) {
  const workflowPath = path.join(repoRoot, '.github', 'workflows', 'im-commercial-gates.yml');

  assert.equal(existsSync(workflowPath), true, 'missing .github/workflows/im-commercial-gates.yml');

  const workflow = read(repoRoot, path.join('.github', 'workflows', 'im-commercial-gates.yml'));
  const rootPackage = JSON.parse(read(repoRoot, 'package.json'));

  assert.equal(
    rootPackage.scripts['check:commercial-readiness'],
    'node scripts/release/commercial-readiness.mjs',
    'root package must expose the canonical commercial readiness entrypoint',
  );
  assert.equal(
    rootPackage.scripts['test:workflow-commercial-gates'],
    'node scripts/run-commercial-gates-governance-node-tests.mjs',
    'root package must expose the repository-owned commercial gates governance node test runner',
  );
  assert.match(
    workflow,
    /push:\s*[\s\S]*?branches:\s*[\s\S]*?-\s*main/,
    'im commercial gates workflow must retain main branch push verification coverage',
  );
  assert.match(
    workflow,
    /pull_request:\s*[\s\S]*?paths:/,
    'im commercial gates workflow must scope pull_request triggers through explicit watched paths',
  );
  assert.match(workflow, /workflow_dispatch:/);
  assert.match(
    workflow,
    /permissions:\s*contents:\s*read/,
    'im commercial gates workflow must declare an explicit read-only GITHUB_TOKEN baseline',
  );
  assert.doesNotMatch(
    workflow,
    /^\s+(?:contents|id-token|attestations|artifact-metadata|packages):\s*write$/m,
    'im commercial gates workflow must not request release-grade write permissions',
  );
  assert.match(
    workflow,
    /FORCE_JAVASCRIPT_ACTIONS_TO_NODE24:\s*'true'/,
    'im commercial gates workflow must opt GitHub JavaScript actions into the Node 24 runtime to avoid hosted-runner deprecation drift',
  );
  assert.match(workflow, /actions\/checkout@v5/);
  assert.match(workflow, /actions\/setup-node@v5/);

  for (const requirement of listImCommercialGatesWorkflowWatchRequirements()) {
    assert.match(
      workflow,
      createLiteralPattern(requirement.path),
      requirement.message,
    );
  }
  for (const contract of listImCommercialGatesWorkflowStepContracts()) {
    assert.match(
      workflow,
      new RegExp(contract.patternSource),
      contract.message,
    );
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

  const governanceCatalog = await import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
    ).href,
  );
  const governanceRunner = await import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'run-commercial-gates-governance-node-tests.mjs'),
    ).href,
  );

  assert.equal(typeof governanceCatalog.listCommercialGatesGovernanceNodeTestFiles, 'function');
  assert.equal(typeof governanceRunner.listCommercialGatesGovernanceNodeTests, 'function');
  assert.equal(typeof governanceRunner.createCommercialGatesGovernanceNodeTestPlan, 'function');
  assert.equal(typeof governanceRunner.runCommercialGatesGovernanceNodeTests, 'function');

  const governedNodeTests = governanceRunner.listCommercialGatesGovernanceNodeTests();
  assert.ok(
    governedNodeTests.includes('scripts/release/commercial-readiness.test.mjs'),
    'commercial gates governance node test runner must include the commercial readiness contract test',
  );
  assert.ok(
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs'),
    'commercial gates governance node test runner must include the Sdkwork IM PC appbase auth UI contract test',
  );
  assert.ok(
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/notary-app-sdk-integration-contract.test.mjs'),
    'commercial gates governance node test runner must include the Sdkwork IM PC notary app SDK integration contract test',
  );
  assert.ok(
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/drive-app-sdk-integration-contract.test.mjs'),
    'commercial gates governance node test runner must include the Sdkwork IM PC drive app SDK integration contract test',
  );
  assert.ok(
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/knowledgebase-app-sdk-integration-contract.test.mjs'),
    'commercial gates governance node test runner must include the Sdkwork IM PC knowledgebase app SDK integration contract test',
  );
  assert.ok(
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/commerce-app-sdk-integration-contract.test.mjs'),
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/mail-app-sdk-integration-contract.test.mjs'),
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/course-app-sdk-integration-contract.test.mjs'),
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/community-app-sdk-integration-contract.test.mjs'),
    governedNodeTests.includes('apps/sdkwork-im-pc/scripts/sdkwork-chat-pc-aiot-devices-contract.test.mjs'),
    'commercial gates governance node test runner must include the Sdkwork IM PC commerce app SDK integration contract test',
  );
  assert.ok(
    governedNodeTests.includes('scripts/im-commercial-gates-workflow.test.mjs'),
    'commercial gates governance node test runner must include the im commercial gates workflow contract test',
  );
  assert.ok(
    governedNodeTests.includes('scripts/im-commercial-gates-watch-catalog.test.mjs')
      && governedNodeTests.includes('scripts/im-commercial-gates-step-contract-catalog.test.mjs'),
    'commercial gates governance node test runner must include the watched surface and step contract catalog tests',
  );
  assert.deepEqual(
    governedNodeTests,
    governanceCatalog.listCommercialGatesGovernanceNodeTestFiles(),
    'commercial gates governance node test runner must own the exact governed test set',
  );
  assert.deepEqual(
    governanceRunner.createCommercialGatesGovernanceNodeTestPlan({
      cwd: '.',
      env: {},
      nodeExecutable: 'node',
    }).args,
    ['--test', '--experimental-test-isolation=none', ...governanceCatalog.listCommercialGatesGovernanceNodeTestFiles()],
    'commercial gates governance node test runner must use the governed node test isolation mode in the repository-owned runner plan',
  );
}
