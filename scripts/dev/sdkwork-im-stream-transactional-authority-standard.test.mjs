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

// The `0004_stream_transactional_authority` migration was squashed into the
// immutable baseline; the stream transactional schema (optimistic version
// column, active-session partial index, version guard) is baseline-owned now.
const baseline = readText('database', 'ddl', 'baseline', 'postgres', '0001_im_baseline.sql');
assert.match(baseline, /CREATE TABLE IF NOT EXISTS im_stream_sessions/u);
assert.match(baseline, /version BIGINT NOT NULL DEFAULT 1 CHECK \(version > 0\)/u);
assert.match(
  baseline,
  /CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_active[\s\S]*?WHERE stream_state NOT IN \('completed', 'aborted', 'expired'\)/u,
);
assert.match(baseline, /chk_im_stream_sessions_version/u);
const streamRegistry = JSON.parse(
  readText('database', 'contract', 'table-registry.json'),
).tables.filter((entry) => entry.table_name === 'im_stream_sessions');
assert.equal(streamRegistry.length, 1, 'im_stream_sessions must be registered exactly once');
assert.equal(
  streamRegistry[0].migration,
  'database/ddl/baseline/postgres/0001_im_baseline.sql',
  'im_stream_sessions must retain immutable baseline provenance',
);

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
