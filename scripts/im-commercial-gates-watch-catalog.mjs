import { createStrictKeyedCatalog } from './strict-contract-catalog.mjs';

function createWatchRequirement(path, message) {
  return {
    path,
    message,
  };
}

export const IM_COMMERCIAL_GATES_WORKFLOW_WATCH_REQUIREMENTS = [
  createWatchRequirement(
    '.github/workflows/im-commercial-gates.yml',
    'im commercial gates workflow must watch its own workflow file',
  ),
  createWatchRequirement(
    '.github/workflows/package.yml',
    'im commercial gates workflow must watch release package workflow dependency ref inputs',
  ),
  createWatchRequirement(
    'Cargo.toml',
    'im commercial gates workflow must watch the Rust workspace manifest',
  ),
  createWatchRequirement(
    'Cargo.lock',
    'im commercial gates workflow must watch the Rust dependency lockfile',
  ),
  createWatchRequirement(
    'config/shared-sdk-release-sources.json',
    'im commercial gates workflow must watch shared SDK release source pins',
  ),
  createWatchRequirement(
    'package.json',
    'im commercial gates workflow must watch the root workspace package because it owns the repository governance entrypoints',
  ),
  createWatchRequirement(
    'sdkwork.workflow.json',
    'im commercial gates workflow must watch SDKWork release dependency declarations',
  ),
  createWatchRequirement(
    'README.md',
    'im commercial gates workflow must watch the primary repository README',
  ),
  createWatchRequirement(
    'apps/sdkwork-im-pc/**',
    'im commercial gates workflow must watch the Sdkwork IM PC Vite workspace',
  ),
  createWatchRequirement(
    'crates/**',
    'im commercial gates workflow must watch the Rust crates workspace',
  ),
  createWatchRequirement(
    'deployments/**',
    'im commercial gates workflow must watch deployment descriptors and packaging inputs',
  ),
  createWatchRequirement(
    'docs/**',
    'im commercial gates workflow must watch the documentation workspace',
  ),
  createWatchRequirement(
    'scripts/build-sdkwork-im-desktop-assets.mjs',
    'im commercial gates workflow must watch the desktop asset build entrypoint',
  ),
  createWatchRequirement(
    'scripts/commercial-gates-governance-node-test-catalog.mjs',
    'im commercial gates workflow must watch the commercial gates governance node test catalog module',
  ),
  createWatchRequirement(
    'scripts/commercial-gates-governance-node-test-catalog.test.mjs',
    'im commercial gates workflow must watch the commercial gates governance node test catalog contract test',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-contracts.mjs',
    'im commercial gates workflow must watch the workflow contract module',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-step-contract-catalog.mjs',
    'im commercial gates workflow must watch the governed workflow step contract catalog module',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-step-contract-catalog.test.mjs',
    'im commercial gates workflow must watch the governed workflow step contract catalog contract test',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-watch-catalog.mjs',
    'im commercial gates workflow must watch the governed workflow watch catalog module',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-watch-catalog.test.mjs',
    'im commercial gates workflow must watch the governed workflow watch catalog contract test',
  ),
  createWatchRequirement(
    'scripts/im-commercial-gates-workflow.test.mjs',
    'im commercial gates workflow must watch the workflow contract test',
  ),
  createWatchRequirement(
    'scripts/prepare-ci-dependencies.mjs',
    'im commercial gates workflow must watch the SDKWork dependency materializer',
  ),
  createWatchRequirement(
    'scripts/run-commercial-gates-governance-node-tests.mjs',
    'im commercial gates workflow must watch the repository-owned governance node test runner',
  ),
  createWatchRequirement(
    'scripts/run-commercial-gates-governance-node-tests.test.mjs',
    'im commercial gates workflow must watch the governance node test runner contract test',
  ),
  createWatchRequirement(
    'scripts/strict-contract-catalog.mjs',
    'im commercial gates workflow must watch the shared strict contract catalog helper',
  ),
  createWatchRequirement(
    'scripts/strict-contract-catalog.test.mjs',
    'im commercial gates workflow must watch the shared strict contract catalog contract test',
  ),
  createWatchRequirement(
    'scripts/dev/**',
    'im commercial gates workflow must watch shared development runtime helpers',
  ),
  createWatchRequirement(
    'scripts/release/**',
    'im commercial gates workflow must watch release and readiness helper scripts',
  ),
  createWatchRequirement(
    'services/**',
    'im commercial gates workflow must watch the services subtree',
  ),
  createWatchRequirement(
    'sdks/**',
    'im commercial gates workflow must watch typed SDK workspace inputs',
  ),
  createWatchRequirement(
    'vendor/**',
    'im commercial gates workflow must watch the vendored dependency subtree',
  ),
];

const imCommercialGatesWorkflowWatchCatalog = createStrictKeyedCatalog({
  entries: IM_COMMERCIAL_GATES_WORKFLOW_WATCH_REQUIREMENTS,
  getKey: ({ path }) => path,
  duplicateKeyMessagePrefix: 'duplicate im commercial gates workflow watch requirement path',
  missingKeyMessagePrefix: 'missing im commercial gates workflow watch requirement',
});

export function listImCommercialGatesWorkflowWatchRequirements() {
  return imCommercialGatesWorkflowWatchCatalog.list();
}

export function findImCommercialGatesWorkflowWatchRequirement(watchPath) {
  return imCommercialGatesWorkflowWatchCatalog.find(watchPath);
}

export function listImCommercialGatesWorkflowWatchRequirementsByPaths(watchPaths = []) {
  return imCommercialGatesWorkflowWatchCatalog.listByKeys(watchPaths);
}

export function listImCommercialGatesWorkflowWatchPaths() {
  return imCommercialGatesWorkflowWatchCatalog.list().map(({ path }) => path);
}
