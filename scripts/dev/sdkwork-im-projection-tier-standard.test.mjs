import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function readExists(relativePath) {
  const absolutePath = path.join(repoRoot, ...relativePath.split('/'));
  assert.ok(fs.existsSync(absolutePath), `expected file ${relativePath}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

const envExample = read('.env.postgres.example');
for (const required of [
  'SDKWORK_IM_PROJECTION_TIMELINE_MEMORY_CAP=1000',
  'SDKWORK_IM_PROJECTION_CURSOR_HS256_SECRET=',
]) {
  assert.ok(envExample.includes(required), `.env.postgres.example must document ${required}`);
}

const bootstrap = readExists('services/projection-service/src/bootstrap.rs');
assert.match(
  bootstrap,
  /configure_durable_timeline/,
  'projection bootstrap must wire tiered durable timeline when Postgres is enabled',
);
assert.match(
  bootstrap,
  /tiered timeline \(postgres durable \+ in-memory hot cache\)/,
  'projection bootstrap must log tiered timeline configuration',
);

const timelineTier = readExists('services/projection-service/src/timeline_tier.rs');
for (const symbol of [
  'PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT',
  'resolve_timeline_window',
  'timeline_window_from_durable_store',
  'trim_timeline_to_cap',
]) {
  assert.ok(timelineTier.includes(symbol), `timeline_tier must implement ${symbol}`);
}

const timelineStore = readExists('adapters/postgres-projection/src/timeline_store.rs');
assert.match(
  timelineStore,
  /LOAD_TIMELINE_WINDOW_SQL/,
  'postgres projection timeline store must use SQL keyset window reads',
);

const access = readExists('services/projection-service/src/access.rs');
assert.match(
  access,
  /parse_member_directory_list_cursor/,
  'member directory must use signed keyset cursor parser',
);
assert.match(
  access,
  /numeric offset \{label\} cursors are unsupported/,
  'every environment must reject numeric projection cursors via the shared keyset parser',
);
assert.doesNotMatch(
  access,
  /is_production_like_im_environment|legacy_offset|Cursor::Offset/,
  'projection cursor parsing must not retain environment-specific offset compatibility',
);

const projectionModels = readExists('services/projection-service/src/model.rs');
assert.doesNotMatch(
  projectionModels,
  /Offset\(usize\)/,
  'projection list cursor models must remain keyset-only',
);

const commercialGates = readExists('.github/workflows/im-commercial-gates.yml');
assert.ok(
  commercialGates.includes('sdkwork-im-projection-tier-standard.test.mjs'),
  'im-commercial-gates.yml must run projection tier standard test',
);

process.stdout.write('sdkwork-im projection tier standard passed\n');
