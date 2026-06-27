#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const rootCargo = read('Cargo.toml');
assert.match(
  rootCargo,
  /sdkwork_utils_rust\s*=\s*\{[^}]*sdkwork-utils/u,
  'Cargo.toml must declare sdkwork_utils_rust workspace dependency for sdkwork-utils integration',
);

const workflow = JSON.parse(read('sdkwork.workflow.json'));
const dependencyIds = new Set((workflow.dependencies || []).map((dependency) => dependency.id));
assert(
  dependencyIds.has('sdkwork-utils'),
  'sdkwork.workflow.json must declare sdkwork-utils sibling checkout',
);

const pnpmWorkspace = read('pnpm-workspace.yaml');
assert.match(
  pnpmWorkspace,
  /sdkwork-utils\/packages\/sdkwork-utils-typescript/u,
  'pnpm-workspace.yaml must include sdkwork-utils-typescript sibling package',
);

const pcCorePackage = JSON.parse(
  read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/package.json'),
);
assert(
  pcCorePackage.dependencies?.['@sdkwork/utils'],
  '@sdkwork/im-pc-core must depend on @sdkwork/utils for shared utility standardization',
);

for (const relativePath of [
  'services/sdkwork-comms-conversation-service/src/runtime.rs',
  'adapters/postgres-realtime/src/lib.rs',
  'adapters/postgres-journal/src/lib.rs',
  'crates/im-app-context/src/lib.rs',
  'services/audit-service/src/lib.rs',
  'services/social-service/src/runtime.rs',
  'services/social-service/src/postgres/direct_chat.rs',
  'services/social-service/src/friendship.rs',
]) {
  const source = read(relativePath);
  assert.match(
    source,
    /sdkwork_utils_rust/u,
    `${relativePath} must consume sdkwork-utils-rust instead of local crypto helpers`,
  );
}

for (const relativePath of [
  'services/sdkwork-comms-conversation-service/src/runtime.rs',
  'adapters/postgres-realtime/src/lib.rs',
  'adapters/postgres-journal/src/lib.rs',
  'crates/im-app-context/src/lib.rs',
  'services/audit-service/src/lib.rs',
  'services/social-service/src/runtime.rs',
  'services/social-service/src/postgres/direct_chat.rs',
  'services/social-service/src/friendship.rs',
  'services/session-gateway/src/cluster_route_event_auth.rs',
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /use sha2::/u,
    `${relativePath} must not import sha2 directly when sdkwork-utils provides crypto helpers`,
  );
}

for (const relativePath of [
  'services/social-service/src/friendship.rs',
  'crates/im-app-context/src/lib.rs',
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /use base64::/u,
    `${relativePath} must use sdkwork_utils_rust base64url helpers instead of direct base64 imports`,
  );
}

const packageWorkflow = read('.github/workflows/package.yml');
assert.match(packageWorkflow, /sdkwork_utils_ref/u, 'package workflow must expose sdkwork_utils_ref');
assert.match(
  packageWorkflow,
  /SDKWORK_UTILS_REF/u,
  'package workflow dependency_refs_json must include SDKWORK_UTILS_REF',
);

process.stdout.write('sdkwork-im utils standard passed\n');
