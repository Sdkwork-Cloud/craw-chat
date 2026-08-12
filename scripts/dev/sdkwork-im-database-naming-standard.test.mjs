import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  DEPLOYMENT_DOC_FILES,
  readDeploymentDoc,
} from '../lib/deployment-docs.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function readDeployment(relativePath) {
  return readDeploymentDoc(repoRoot, relativePath);
}

function readWorkspace(relativePath) {
  return fs.readFileSync(path.join(workspaceRoot, ...relativePath.split('/')), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function unique(values) {
  return [...new Set(values)];
}

function extractAll(regex, source, group = 1) {
  return unique([...source.matchAll(regex)].map((match) => match[group].toLowerCase()));
}

function extractRustRawStrings(source) {
  return [...source.matchAll(/r#"(.*?)"#/gsu)].map((match) => match[1]);
}

const prefixRegistry = readJson('database/contract/prefix-registry.json');
const tableRegistry = readJson('database/contract/table-registry.json');
const canonicalBaselineMigration = 'database/ddl/baseline/postgres/0001_im_baseline.sql';
const activeMigrationFiles = [
  canonicalBaselineMigration,
  ...fs
    .readdirSync(path.join(repoRoot, 'database', 'migrations', 'postgres'))
    .filter((entry) => entry.endsWith('.up.sql'))
    .sort((left, right) => left.localeCompare(right))
    .map((entry) => `database/migrations/postgres/${entry}`),
];
const baselineSchema = read(canonicalBaselineMigration).toLowerCase();
const schema = activeMigrationFiles
  .map((entry) => read(entry))
  .join('\n')
  .toLowerCase();
const databaseSpec = readWorkspace('sdkwork-specs/DATABASE_SPEC.md');
const cargoManifest = read('Cargo.toml');
const runtimeIdCrate = read('crates/sdkwork-im-runtime-id/src/lib.rs');
const sdkworkWorkflow = readJson('sdkwork.workflow.json');
const sharedSdkReleaseSources = readJson('config/shared-sdk-release-sources.json');
const chatPcPnpmWorkspace = read('pnpm-workspace.yaml');
const chatPcTsconfig = readJson('apps/sdkwork-im-pc/tsconfig.json');
const chatPcViteConfig = read('apps/sdkwork-im-pc/vite.config.ts');
const componentSpec = readJson('specs/component.spec.json');
const localSpecReadme = read('specs/README.md');
const namingDoc = readDeployment(DEPLOYMENT_DOC_FILES.databaseNaming);
const ubuntuWslGuide = readDeployment(DEPLOYMENT_DOC_FILES.ubuntuWslGuide);

assert.equal(prefixRegistry.kind, 'sdkwork.database.prefix-registry');
const imPrefix = prefixRegistry.prefixes.find((entry) => entry.prefix === 'im_');
assert.ok(imPrefix, 'canonical prefix registry must register im_ for instant-messaging tables');
assert.equal(imPrefix.status, 'active');
assert.equal(imPrefix.business_domain, 'instant_messaging');
assert.match(imPrefix.allowed_table_name_pattern, /\^im_/u);
assert.ok(imPrefix.forbidden_aliases.includes('chat'));
assert.ok(imPrefix.forbidden_aliases.includes('comms'));

assert.equal(prefixRegistry.non_im_prefix_policy.must_not_use_im_prefix, true);
assert.equal(prefixRegistry.non_im_prefix_policy.keep_existing_business_prefix, true);
assert.match(
  prefixRegistry.non_im_prefix_policy.description,
  /outside the instant messaging bounded context/u,
  'non-IM tables must not be swept into the im_ prefix',
);

assert.equal(tableRegistry.kind, 'sdkwork.database.table-registry');

const registeredTables = tableRegistry.tables.map((entry) => entry.table_name);
const baselineRegisteredTables = tableRegistry.tables
  .filter((entry) => entry.migration.startsWith('database/ddl/baseline/postgres/'))
  .map((entry) => entry.table_name);
assert.equal(
  registeredTables.length,
  unique(registeredTables).length,
  'database table registry must not contain duplicate table names',
);
assert.equal(registeredTables.length, 55, 'canonical IM table registry must contain 55 tables');
assert.equal(baselineRegisteredTables.length, 55, 'PostgreSQL baseline must own 55 tables');

for (const entry of tableRegistry.tables) {
  assert.equal(entry.module_prefix, 'im', `${entry.table_name} must register module_prefix=im`);
  const allowedBoundedContexts = new Set([
    'instant_messaging',
    'social',
    'organization',
    'messaging',
    'user',
  ]);
  assert.ok(
    allowedBoundedContexts.has(entry.bounded_context),
    `${entry.table_name} must belong to a registered IM bounded context`,
  );
  assert.match(entry.table_name, /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u);
  assert.ok(entry.table_profile, `${entry.table_name} must declare a table profile`);
  assert.ok(entry.write_owner, `${entry.table_name} must declare a write owner`);
  const migrationPath = entry.migration;
  const lifecycleStatus = entry.lifecycle_status;
  assert.ok(
    lifecycleStatus === 'active' || lifecycleStatus === 'expanding',
    `${entry.table_name} has unsupported lifecycle_status=${lifecycleStatus}`,
  );
  if (lifecycleStatus === 'active') {
    assert.match(
      migrationPath,
      /^database\/(?:ddl\/baseline\/postgres\/[0-9]{4}_[a-z0-9_]+\.sql|migrations\/postgres\/[0-9]{4}_[a-z0-9_]+\.up\.sql)$/u,
      `${entry.table_name} must point to its immutable PostgreSQL baseline or versioned up migration`,
    );
  } else {
    assert.match(
      migrationPath,
      /^database\/migrations\/postgres\/[0-9]{4}_[a-z0-9_]+\.up\.sql$/u,
      `${entry.table_name} expanding lifecycle must point to a PostgreSQL up migration`,
    );
  }
  assert.ok(
    fs.existsSync(path.join(repoRoot, ...migrationPath.split('/'))),
    `${entry.table_name} migration file must exist: ${migrationPath}`,
  );
  const migrationSource = read(migrationPath).toLowerCase();
  assert.ok(
    migrationSource.includes(`create table`) && migrationSource.includes(entry.table_name),
    `${entry.table_name} must be created in its registered migration ${migrationPath}`,
  );
  if (migrationPath.includes('/migrations/postgres/')) {
    const downMigrationPath = migrationPath.replace(/\.up\.sql$/u, '.down.sql');
    assert.ok(
      fs.existsSync(path.join(repoRoot, ...downMigrationPath.split('/'))),
      `${entry.table_name} migration must have a paired down migration: ${downMigrationPath}`,
    );
    const downMigrationSource = read(downMigrationPath).toLowerCase();
    assert.match(
      downMigrationSource,
      new RegExp(`\\bdrop\\s+table(?:\\s+if\\s+exists)?\\s+${entry.table_name}\\b`, 'u'),
      `${entry.table_name} must be dropped in its paired down migration ${downMigrationPath}`,
    );
  }
}

const migrationTables = extractAll(
  /\bcreate\s+table(?:\s+if\s+not\s+exists)?\s+([a-z_][a-z0-9_]*)\s*\(/giu,
  schema,
);
assert.ok(migrationTables.length > 0, 'core IM schema must define database tables');
assert.equal(migrationTables.length, 55, 'baseline plus PostgreSQL migrations must define 55 IM tables');
for (const table of migrationTables) {
  assert.match(
    table,
    /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `instant messaging migration table ${table} must use the im_ prefix`,
  );
  assert.ok(
    registeredTables.includes(table),
    `instant messaging migration table ${table} must be listed in database/contract/table-registry.json`,
  );
  assert.ok(table.length <= 63, `${table} should fit PostgreSQL's default identifier length`);
}

for (const table of baselineRegisteredTables) {
  assert.ok(
    migrationTables.includes(table),
    `baseline-registered IM table ${table} must exist in the canonical baseline`,
  );
}
assert.ok(
  !migrationTables.includes('__manual_smoke_check'),
  'manual smoke check tables must not be created by checked-in migrations',
);
assert.ok(
  !registeredTables.includes('__manual_smoke_check'),
  'manual smoke check tables must not be registered as IM business tables',
);

for (const forbiddenPrefix of imPrefix.forbidden_aliases) {
  assert.ok(
    !migrationTables.some((table) => table.startsWith(`${forbiddenPrefix}_`)),
    `IM migration must not use product/project/generic prefix ${forbiddenPrefix}_`,
  );
}

const schemaWithoutComments = baselineSchema.replace(/^--[^\n]*$/gmu, '');
const constraintNames = extractAll(
  /\bconstraint\s+(?!if\b)([a-z_][a-z0-9_]*)\b/giu,
  schemaWithoutComments,
);
for (const constraintName of constraintNames) {
  assert.match(
    constraintName,
    /^(pk|uk|fk|chk)_im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `IM schema constraint ${constraintName} must be visibly tied to im_`,
  );
  assert.ok(
    constraintName.length <= 63,
    `${constraintName} should fit PostgreSQL's default identifier length`,
  );
}

const indexNames = extractAll(
  /\bcreate\s+(?:unique\s+)?index\s+if\s+not\s+exists\s+([a-z_][a-z0-9_]*)\b/giu,
  baselineSchema,
);
for (const indexName of indexNames) {
  assert.match(
    indexName,
    /^(idx|uk)_im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `IM schema index ${indexName} must be visibly tied to im_`,
  );
  assert.ok(indexName.length <= 63, `${indexName} should fit PostgreSQL's default identifier length`);
}

const sqlContractFiles = [
  'crates/im-postgres-realtime-contracts/src/lib.rs',
  'adapters/postgres-realtime/src/lib.rs',
];
const imTableNameHints =
  /(?:conversation|message|realtime|presence|route|rtc|audit|notification|automation|projection|stream|commit|outbox|inbox|idempotency|subscription|checkpoint|fence)/u;
for (const relativePath of sqlContractFiles) {
  const source = read(relativePath).toLowerCase();
  const sqlSource = extractRustRawStrings(source).join('\n').toLowerCase();
  const referencedTables = extractAll(
    /\b(?:from|join|insert\s+into|update|delete\s+from)\s+([a-z_][a-z0-9_]*)\b/giu,
    sqlSource,
  );
  for (const table of referencedTables.filter((name) => imTableNameHints.test(name))) {
    assert.match(
      table,
      /^im_/u,
      `${relativePath} references IM-like table ${table}; IM storage tables must use im_`,
    );
    assert.ok(
      registeredTables.includes(table),
      `${relativePath} references ${table}, which must be registered`,
    );
  }
}

assert.ok(
  componentSpec.canonicalSpecs.some((entry) => entry.file === 'DATABASE_SPEC.md'),
  'component spec must reference the root DATABASE_SPEC authority',
);
assert.ok(
  componentSpec.localExtensionSpecs.some(
    (entry) => entry.path === '../database/contract/prefix-registry.json',
  ),
  'component spec must expose the canonical database prefix registry',
);
assert.ok(
  componentSpec.localExtensionSpecs.some(
    (entry) => entry.path === '../database/contract/table-registry.json',
  ),
  'component spec must expose the canonical database table registry',
);

for (const required of [
  'database/contract/prefix-registry.json',
  'database/contract/table-registry.json',
  'database-table-naming-standard.md',
  'im_',
  'non-IM',
]) {
  assert.ok(localSpecReadme.includes(required), `specs/README.md must mention ${required}`);
}

for (const required of [
  'database/contract/prefix-registry.json',
  'database/contract/table-registry.json',
  'im_',
]) {
  assert.ok(namingDoc.includes(required), `database naming documentation must mention ${required}`);
}
assert.match(namingDoc, /non-im/iu, 'database naming documentation must mention non-IM scope');
assert.ok(
  namingDoc.includes('sdkwork_ai_dev.__manual_smoke_check'),
  'database naming documentation must document the manual smoke check exception',
);
assert.ok(
  ubuntuWslGuide.includes('CREATE TABLE IF NOT EXISTS sdkwork_ai_dev.__manual_smoke_check'),
  'Ubuntu/WSL guide may use a non-IM manual smoke check table for connectivity verification',
);
assert.ok(
  ubuntuWslGuide.includes('DROP TABLE sdkwork_ai_dev.__manual_smoke_check'),
  'manual smoke check table must be dropped in the same guide',
);

assert.match(
  databaseSpec,
  /Runtime business tables `MUST` use a stable `int64` logical primary identifier named `id`/u,
  'root DATABASE_SPEC.md must require explicit generated IDs for runtime INSERTs',
);
assert.match(
  databaseSpec,
  /The value of `id` `MUST` be generated by an approved SDKWork ID provider before insert/u,
  'root DATABASE_SPEC.md must require business IDs to be generated before insert',
);
assert.match(
  databaseSpec,
  /SDKWork Rust implementations `MUST` reuse the approved SDKWork platform ID service/u,
  'root DATABASE_SPEC.md must require Rust implementations to reuse the platform ID service',
);

assert.match(
  cargoManifest,
  /"crates\/sdkwork-im-runtime-id"/u,
  'sdkwork-im workspace must include the runtime ID capability crate',
);
assert.match(
  cargoManifest,
  /sdkwork_id\s*=\s*\{\s*path\s*=\s*"\.\.\/sdkwork-appbase\/crates\/sdkwork-platform-id-service"/u,
  'sdkwork-im must consume the appbase platform ID service Snowflake generator instead of a local fork',
);
assert.match(
  runtimeIdCrate,
  /use sdkwork_id::\{/u,
  'sdkwork-im runtime ID crate must use the appbase Snowflake generator',
);
assert.match(
  runtimeIdCrate,
  /pub const SDKWORK_IM_ID_NODE_ID_ENV/u,
  'sdkwork-im runtime ID generation must require an explicit node ID env key',
);
assert.match(
  runtimeIdCrate,
  /failure_handling:\s*"database_first_then_fail_closed"/u,
  'sdkwork-im runtime ID strategy must fail closed when database-backed allocation is unavailable',
);
assert.match(
  runtimeIdCrate,
  /pub fn runtime_id_fallback_is_forbidden[\s\S]*if deployment_is_explicit\s*\{\s*return true;/u,
  'sdkwork-im runtime ID generation must forbid static Snowflake fallback for explicit deployment profiles',
);
assert.match(
  runtimeIdCrate,
  /return Arc::new\(UnavailableIdGenerator/u,
  'sdkwork-im runtime ID generation must surface an unavailable generator instead of reusing a static production node',
);

const appbaseReleaseDependency = sdkworkWorkflow.dependencies.find(
  (entry) => entry.id === 'sdkwork-appbase',
);
assert.ok(
  appbaseReleaseDependency,
  'sdkwork-im release workflow must declare the sdkwork-appbase dependency used by runtime IDs and IAM SDKs',
);
assert.equal(appbaseReleaseDependency.repository, 'Sdkwork-Cloud/sdkwork-appbase');
assert.equal(appbaseReleaseDependency.refInput, 'SDKWORK_APPBASE_REF');
assert.equal(appbaseReleaseDependency.tokenSecret, 'SDKWORK_RELEASE_TOKEN');
assert.match(
  appbaseReleaseDependency.ref,
  /^[0-9a-f]{40}$/u,
  'sdkwork-appbase release dependency must be pinned to a reproducible commit ref',
);

const appbaseReleaseSource = sharedSdkReleaseSources.sources['sdkwork-appbase'];
assert.ok(appbaseReleaseSource, 'shared SDK release sources must include sdkwork-appbase');
assert.equal(appbaseReleaseSource.repoUrl, 'https://github.com/Sdkwork-Cloud/sdkwork-appbase.git');
assert.ok(appbaseReleaseSource.ref, 'sdkwork-appbase shared SDK release source must declare a ref');

function pnpmWorkspaceDeclaresPackage(workspaceSource, packagePath) {
  const normalized = workspaceSource.replace(/["']/g, '');
  return normalized.includes(`- ${packagePath}`);
}

for (const requiredWorkspacePackage of [
  '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript',
  '../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript',
  '../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-contracts',
  '../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-sdk-ports',
  '../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-runtime',
  '../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-pc-react',
  '../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react',
]) {
  assert.ok(
    pnpmWorkspaceDeclaresPackage(chatPcPnpmWorkspace, requiredWorkspacePackage),
    `sdkwork-im repository pnpm workspace must declare appbase source package ${requiredWorkspacePackage}`,
  );
}
for (const [packageName, tsconfigPath, vitePath] of [
  [
    '@sdkwork/iam-app-sdk',
    '../../../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/src/index.ts',
    '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/src/index.ts',
  ],
  [
    '@sdkwork/iam-backend-sdk',
    '../../../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/src/index.ts',
    '../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/src/index.ts',
  ],
]) {
  assert.deepEqual(
    chatPcTsconfig.compilerOptions?.paths?.[packageName],
    [tsconfigPath],
    `apps/sdkwork-im-pc tsconfig must resolve ${packageName} through the composed facade source entry`,
  );
  assert.ok(
    chatPcViteConfig.includes(vitePath),
    `apps/sdkwork-im-pc Vite config must resolve ${packageName} through the composed facade source entry`,
  );
}
for (const forbiddenWorkspacePackage of [
  '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/generated/server-openapi',
  '../sdkwork-iam/sdks/sdkwork-iam-backend-sdk/sdkwork-iam-backend-sdk-typescript/generated/server-openapi',
]) {
  assert.equal(
    pnpmWorkspaceDeclaresPackage(chatPcPnpmWorkspace, forbiddenWorkspacePackage),
    false,
    `sdkwork-im repository pnpm workspace must consume IAM SDK composed facades instead of generated transport package ${forbiddenWorkspacePackage}`,
  );
}
assert.doesNotMatch(
  chatPcPnpmWorkspace,
  /\blink:/u,
  'sdkwork-im must consume appbase source packages through pnpm workspace declarations, not link: aliases',
);

const postgresSchemaTestFiles = [
  'services/session-gateway/tests/database_schema_contract_test.rs',
  'services/session-gateway/tests/postgres_realtime_live_runtime_test.rs',
  'services/session-gateway/tests/postgres_realtime_websocket_live_drill_test.rs',
  'crates/im-postgres-realtime-contracts/tests/postgres_realtime_contracts_test.rs',
  'adapters/postgres-realtime/tests/postgres_realtime_live_integration_test.rs',
];
const postgresSchemaScriptFiles = [
  'scripts/dev/sdkwork-im-runtime-id-standard.test.mjs',
];
for (const relativePath of postgresSchemaTestFiles) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /deployments\/database\/postgres\/migrations/u,
    `${relativePath} must not reference retired deployments/database migration paths`,
  );
  assert.match(
    source,
    /database\/ddl\/baseline\/postgres\/0001_im_baseline\.sql/u,
    `${relativePath} must load the canonical PostgreSQL baseline DDL`,
  );
}

for (const relativePath of postgresSchemaScriptFiles) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /deployments\/database\/postgres\/migrations/u,
    `${relativePath} must not reference retired deployments/database migration paths`,
  );
  assert.match(
    source,
    /0001_im_baseline\.sql/u,
    `${relativePath} must validate the canonical PostgreSQL baseline DDL`,
  );
}

console.log('sdkwork-chat database naming standard contract passed');
