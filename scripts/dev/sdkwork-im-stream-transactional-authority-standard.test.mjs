import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8').replace(/\r\n/gu, '\n');
}

const contract = readText('crates', 'sdkwork-im-contract-stream', 'src', 'lib.rs');
for (const required of [
  'organization_id',
  'check_ready',
  'create_session',
  'append_frame',
  'transition_session',
  'list_frames_after',
]) {
  assert.match(contract, new RegExp(`\\b${required}\\b`, 'u'));
}
assert.doesNotMatch(contract, /StreamStateRecord|\bload_state\b|\bsave_state\b/u);

const runtime = readText('services', 'streaming-service', 'src', 'state.rs');
const runtimeStruct = runtime.match(/pub struct StreamingRuntime \{[\s\S]*?\n\}/u)?.[0] ?? '';
assert.doesNotMatch(runtimeStruct, /HashMap|BTreeMap|Vec<StreamFrame>|StreamSessionRecord>/u);
assert.match(runtime, /MAX_CONCURRENCY_RETRIES:\s*usize\s*=\s*8/u);
assert.match(runtime, /page_size \+ 1/u);
assert.match(runtime, /record_append_version_conflict/u);
assert.match(runtime, /record_capacity_rejection/u);
assert.match(runtime, /check_store_ready/u);

const postgres = readText('adapters', 'postgres-journal', 'src', 'stream_state_store.rs');
assert.match(
  postgres,
  /where tenant_id = \$1 and organization_id = \$2 and stream_id = \$3 and frame_seq > \$4\s+order by frame_seq asc\s+limit \$5/u,
);
assert.match(postgres, /for update/u);
assert.match(postgres, /and version = \$17/u);
assert.match(postgres, /select pg_advisory_xact_lock/u);
assert.match(postgres, /simple_query\("select 1"\)/u);
assert.doesNotMatch(postgres, /DEFAULT_ORGANIZATION_ID/u);
const insertFrameSql = postgres.match(/const INSERT_FRAME_SQL:[\s\S]*?"#;/u)?.[0] ?? '';
assert.doesNotMatch(insertFrameSql, /on conflict[^\n]*do nothing/iu);

for (const migration of [
  ['database', 'migrations', 'postgres', '0004_stream_transactional_authority.up.sql'],
  ['database', 'migrations', 'sqlite', '0004_stream_transactional_authority.up.sql'],
]) {
  assert.equal(fs.existsSync(path.join(repoRoot, ...migration)), true, `missing ${migration.join('/')}`);
}

const liveIntegration = readText(
  'adapters',
  'postgres-journal',
  'tests',
  'stream_state_live_integration_test.rs',
);
assert.match(liveIntegration, /organization/u);
assert.match(liveIntegration, /thread::spawn/u);
assert.match(liveIntegration, /StreamAppendOutcome::Applied/u);
assert.match(liveIntegration, /list_frames_after/u);

const app = readText('services', 'streaming-service', 'src', 'app.rs');
assert.match(app, /StreamStoreReadiness/u);
assert.match(app, /skip_metrics\(\)/u);
assert.match(app, /render_runtime_metrics_prometheus/u);

console.log('sdkwork-im stream transactional authority standard passed');
