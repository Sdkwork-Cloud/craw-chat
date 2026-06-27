import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8').replace(/\r\n/gu, '\n');
}

const rootCargo = readText('Cargo.toml');
assert.match(
  rootCargo,
  /sdkwork_id\s*=\s*\{\s*path\s*=\s*"\.\.\/sdkwork-appbase\/crates\/sdkwork-platform-id-service"/u,
  'Rust runtime ID generation must reuse sdkwork-appbase sdkwork-platform-id-service via the sdkwork_id workspace alias.',
);
assert.match(
  rootCargo,
  /"crates\/sdkwork-im-runtime-id"/u,
  'sdkwork-im-runtime-id must be a workspace crate so database insert code can use the standard runtime ID generator.',
);

const runtimeIdSource = readText('crates', 'sdkwork-im-runtime-id', 'src', 'lib.rs');
for (const expectedText of [
  'use sdkwork_id::{',
  'SnowflakeIdGenerator',
  'pub const SDKWORK_IM_ID_NODE_ID_ENV: &str = "SDKWORK_IM_ID_NODE_ID";',
  'clock_rollback: "reject_and_alert"',
  'node_conflict: "database_backed_auto_allocation"',
  'failure_handling: "database_first_then_env_fallback"',
]) {
  assert.match(
    runtimeIdSource,
    new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `runtime ID source must contain standard fragment: ${expectedText}`,
  );
}
assert.doesNotMatch(
  runtimeIdSource,
  /rand::|random::<|Uuid::|uuid::|bigserial|nextval|max\s*\(\s*id\s*\)/iu,
  'runtime ID generation must not implement random IDs, UUID-derived numeric IDs, database sequences, or MAX(id)+1.',
);

for (const relativePath of [
  'deployments/templates/server.env.example',
  'deployments/templates/quickstart-server-compose.env.example',
]) {
  const template = readText(...relativePath.split('/'));
  assert.match(
    template,
    /^SDKWORK_IM_ID_NODE_ID=\d+$/mu,
    `${relativePath} must declare an explicit Snowflake node id for distributed deployments.`,
  );
}

const releasePlanner = readText('scripts', 'release', 'plan-sdkwork-im-install-packages.mjs');
assert.match(
  releasePlanner,
  /'SDKWORK_IM_ID_NODE_ID'/u,
  'release package database/runtime env override policy must preserve SDKWORK_IM_ID_NODE_ID.',
);

const postgresMigration = readText(
  'database',
  'ddl',
  'baseline',
  'postgres',
  '0001_im_legacy_baseline.sql',
);
assert.doesNotMatch(
  postgresMigration,
  /\b(?:bigserial|serial)\b|generated\s+.*\s+identity|\bnextval\s*\(/iu,
  'runtime PostgreSQL schema must not allocate runtime business IDs through database auto-increment primitives.',
);

const spaceServiceCargo = readText('services', 'space-service', 'Cargo.toml');
assert.match(
  spaceServiceCargo,
  /sdkwork-im-runtime-id\s*=\s*\{\s*path\s*=\s*"\.\.\/\.\.\/crates\/sdkwork-im-runtime-id"/u,
  'space-service must depend on sdkwork-im-runtime-id for Snowflake entity ids.',
);

const spaceServiceIdSource = readText('services', 'space-service', 'src', 'id.rs');
for (const expectedText of [
  'RuntimeSnowflakeIdGenerator',
  'build_runtime_id_generator',
  'next_entity_id',
  'IdGenerator',
]) {
  assert.match(
    spaceServiceIdSource,
    new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `space-service id module must include ${expectedText}`,
  );
}

const spaceServiceHttp = readText('services', 'space-service', 'src', 'http.rs');
assert.match(
  spaceServiceHttp,
  /pub id_generator: Arc<dyn IdGenerator>/u,
  'space-service AppState must expose id_generator for snowflake entity allocation.',
);

for (const handlerPath of ['space.rs', 'group.rs', 'channel.rs']) {
  const source = readText('services', 'space-service', 'src', handlerPath);
  assert.match(
    source,
    /next_entity_id\(&state\.id_generator\)/u,
    `services/space-service/src/${handlerPath} must allocate entity ids through the shared snowflake generator`,
  );
  assert.doesNotMatch(
    source,
    /fn generate_id\(/u,
    `services/space-service/src/${handlerPath} must not keep ad-hoc generate_id helpers`,
  );
}

const socialServiceCargo = readText('services', 'social-service', 'Cargo.toml');
assert.match(
  socialServiceCargo,
  /sdkwork-im-runtime-id\s*=\s*\{\s*path\s*=\s*"\.\.\/\.\.\/crates\/sdkwork-im-runtime-id"/u,
  'social-service must depend on sdkwork-im-runtime-id for Postgres-backed entity ids.',
);

const socialServiceIdSource = readText('services', 'social-service', 'src', 'postgres', 'id.rs');
for (const expectedText of [
  'RuntimeSnowflakeIdGenerator',
  'build_runtime_id_generator',
  'next_entity_id',
  'IdGenerator',
]) {
  assert.match(
    socialServiceIdSource,
    new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `social-service id module must include ${expectedText}`,
  );
}

const socialPostgresHttp = readText('services', 'social-service', 'src', 'postgres', 'http.rs');
assert.match(
  socialPostgresHttp,
  /pub id_generator: Arc<dyn IdGenerator>/u,
  'social-service PostgresAppState must expose id_generator for snowflake entity allocation.',
);

assert.equal(
  fs.existsSync(path.join(repoRoot, 'services', 'social-service', 'src', 'organization.rs')),
  false,
  'social-service must not keep the uncompiled organization scaffold; space routes belong to space-service.',
);

for (const handlerPath of ['block.rs', 'direct_chat.rs']) {
  const source = readText('services', 'social-service', 'src', 'postgres', handlerPath);
  assert.match(
    source,
    /next_entity_id\(&state\.id_generator\)/u,
    `services/social-service/src/postgres/${handlerPath} must allocate entity ids through the shared snowflake generator`,
  );
  assert.doesNotMatch(
    source,
    /fn generate_id\(/u,
    `services/social-service/src/postgres/${handlerPath} must not keep ad-hoc generate_id helpers`,
  );
}

console.log('sdkwork-chat runtime id standard contract passed');
