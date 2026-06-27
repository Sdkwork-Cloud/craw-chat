import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
    ).href,
  );
}

test('commercial gates governance node test catalog publishes the exact governed test file set', async () => {
  const module = await loadModule();

  assert.equal(typeof module.listCommercialGatesGovernanceNodeTestFiles, 'function');
  assert.deepEqual(
    module.listCommercialGatesGovernanceNodeTestFiles(),
    [
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/commercial-gates-governance-node-test-catalog.test.mjs',
      'scripts/run-commercial-gates-governance-node-tests.test.mjs',
      'scripts/im-commercial-gates-watch-catalog.test.mjs',
      'scripts/im-commercial-gates-step-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
      'scripts/dev/sdkwork-im-pc-dev-command.test.mjs',
      'scripts/dev/sdkwork-im-pc-i18n.test.mjs',
      'scripts/dev/sdkwork-im-pc-sidebar-modules.test.mjs',
      'scripts/dev/sdkwork-im-pc-im-api-standard.test.mjs',
      'scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs',
      'scripts/dev/sdkwork-im-iam-application-bootstrap-standard.test.mjs',
      'scripts/dev/sdkwork-im-bootstrap-access-token.test.mjs',
      'scripts/dev/sdkwork-im-rtc-signaling-boundary.test.mjs',
      'scripts/dev/sdkwork-im-runtime-standard.test.mjs',
      'scripts/dev/sdkwork-im-retention-enforcement-standard.test.mjs',
      'scripts/dev/sdkwork-im-observability-bootstrap-standard.test.mjs',
      'scripts/dev/sdkwork-im-runtime-id-standard.test.mjs',
      'scripts/dev/sdkwork-im-deprecated-service-boundary.test.mjs',
      'scripts/dev/sdkwork-im-topology-baggage.test.mjs',
      'scripts/dev/sdkwork-im-web-framework-standard.test.mjs',
      'scripts/dev/sdkwork-im-database-framework-standard.test.mjs',
      'scripts/dev/sdkwork-im-database-naming-standard.test.mjs',
      'scripts/dev/sdkwork-im-component-spec-consistency.test.mjs',
      'scripts/dev/sdkwork-im-apis-authority-standard.test.mjs',
      'scripts/dev/sdkwork-im-rpc-contract.test.mjs',
      'scripts/dev/sdkwork-im-sdk-websocket-contract-node.test.mjs',
      'scripts/sdkwork-workspace-structure-standard.test.mjs',
      'apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/sdk-runtime-token-manager-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/notary-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/drive-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/knowledgebase-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/commerce-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/mail-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/community-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/course-app-sdk-integration-contract.test.mjs',
      'apps/sdkwork-im-pc/scripts/sdkwork-chat-pc-aiot-devices-contract.test.mjs',
      'scripts/release/commercial-readiness.test.mjs',
    ],
  );
});

test('commercial gates governance node test catalog exposes strict file lookup helpers', async () => {
  const module = await loadModule();

  assert.equal(typeof module.findCommercialGatesGovernanceNodeTestFile, 'function');
  assert.equal(typeof module.listCommercialGatesGovernanceNodeTestFilesByPaths, 'function');

  assert.equal(
    module.findCommercialGatesGovernanceNodeTestFile('scripts/strict-contract-catalog.test.mjs'),
    'scripts/strict-contract-catalog.test.mjs',
  );
  assert.deepEqual(
    module.listCommercialGatesGovernanceNodeTestFilesByPaths([
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
    ]),
    [
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
    ],
  );
  assert.throws(
    () => module.findCommercialGatesGovernanceNodeTestFile('scripts/missing-commercial-governance.test.mjs'),
    /missing commercial gates governance node test file.*missing-commercial-governance/i,
  );
});
