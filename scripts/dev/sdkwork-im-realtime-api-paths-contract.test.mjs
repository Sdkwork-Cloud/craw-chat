import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { materializeSdkworkImRealtimeApiPaths } from './materialize-sdkwork-im-realtime-api-paths.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const tsPath = path.join(
  repoRoot,
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime-api-paths.ts',
);

const materialized = materializeSdkworkImRealtimeApiPaths();
const committed = fs.readFileSync(tsPath, 'utf8');

assert.equal(
  committed,
  materialized,
  'TypeScript realtime-api-paths must match materialized output from Rust authority',
);

const tsPathConsumers = [
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts',
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/sdkBaseUrls.ts',
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/imSdkClient.ts',
];

const mjsPathConsumers = ['apps/sdkwork-im-pc/scripts/sdkwork-im-iam-env.mjs'];

for (const relativePath of tsPathConsumers) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.doesNotMatch(
    source,
    /['"]\/im\/v3\/api\/realtime\/ws['"]/u,
    `${relativePath} must not hardcode the realtime websocket path literal`,
  );
  assert.match(
    source,
    /IM_REALTIME_WS/u,
    `${relativePath} must import IM_REALTIME_WS from the SDK paths authority`,
  );
}

for (const relativePath of mjsPathConsumers) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.doesNotMatch(
    source,
    /['"]\/im\/v3\/api\/realtime\/ws['"]/u,
    `${relativePath} must not hardcode the realtime websocket path literal`,
  );
  assert.match(
    source,
    /readRustRealtimePathConstants/u,
    `${relativePath} must resolve realtime websocket path from Rust path authority`,
  );
}

const rustPathConsumers = [
  'services/sdkwork-im-cloud-gateway/src/lib.rs',
  'crates/sdkwork-im-web-bootstrap/src/lib.rs',
  'crates/sdkwork-routes-im-realtime-open-api/src/paths.rs',
  'tools/chat-cli/src/realtime.rs',
];

for (const relativePath of rustPathConsumers) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.match(
    source,
    /sdkwork_im_realtime_api_paths/u,
    `${relativePath} must consume sdkwork-im-realtime-api-paths`,
  );
}

console.log('sdkwork im realtime api paths contract passed.');
