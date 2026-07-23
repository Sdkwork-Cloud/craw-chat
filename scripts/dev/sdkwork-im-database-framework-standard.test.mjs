#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const rootCargo = read('Cargo.toml');
assert.match(
  rootCargo,
  /sdkwork_database_config\s*=\s*\{[^}]*sdkwork-database-config/u,
  'Cargo.toml must declare sdkwork-database-config workspace dependency',
);
assert.match(
  rootCargo,
  /sdkwork_database_sqlx\s*=\s*\{[^}]*sdkwork-database-sqlx/u,
  'Cargo.toml must declare sdkwork-database-sqlx workspace dependency',
);

const postgresAdapters = [
  'adapters/postgres-journal/Cargo.toml',
  'adapters/postgres-realtime/Cargo.toml',
  'adapters/social-postgres/Cargo.toml',
];
for (const relativePath of postgresAdapters) {
  const cargo = read(relativePath);
  assert.match(
    cargo,
    /sdkwork_database_config\.workspace\s*=\s*true/u,
    `${relativePath} must consume sdkwork-database-config from workspace dependencies`,
  );
}

const databasePoolCargo = read('crates/sdkwork-im-database-pool/Cargo.toml');
assert.match(databasePoolCargo, /sdkwork_database_sqlx\.workspace\s*=\s*true/u);
assert.match(databasePoolCargo, /sdkwork_database_config\.workspace\s*=\s*true/u);

const databasePoolLib = read('crates/sdkwork-im-database-pool/src/lib.rs');
assert.match(databasePoolLib, /create_im_database_pool_from_env/u);
assert.match(databasePoolLib, /bootstrap_im_database_from_env/u);

const databaseHostCargo = read('crates/sdkwork-im-database-host/Cargo.toml');
assert.match(databaseHostCargo, /sdkwork_database_lifecycle/u);

const specsReadme = read('specs/README.md');
assert.match(specsReadme, /DATABASE_SPEC\.md/u);

const postgresBaselinePath = 'database/ddl/baseline/postgres/0001_im_baseline.sql';
assert.ok(
  fs.existsSync(path.join(repoRoot, postgresBaselinePath)),
  `${postgresBaselinePath} must exist for L2 lifecycle`,
);

const databaseManifest = readJson('database/database.manifest.json');
assert.deepEqual(
  databaseManifest.engines,
  ['postgres'],
  'IM runtime persistence engines must match the PostgreSQL-only database manifest',
);
assert.equal(
  databaseManifest.defaultEngine,
  'postgres',
  'IM runtime persistence must default to PostgreSQL',
);
assert.equal(
  databaseManifest.baselineStrategy,
  'baseline-plus-migrations',
  'IM lifecycle must keep the immutable baseline plus versioned migrations',
);

const rootPackage = readJson('package.json');
const materializeContractCommand = rootPackage.scripts?.['db:materialize:contract'] ?? '';
assert.equal(
  materializeContractCommand,
  'node tools/materialize_im_database_contract.mjs --write',
  'db:materialize:contract must compose the IM baseline and PostgreSQL migrations',
);
assert.equal(
  rootPackage.scripts?.['db:contract:check'],
  'node tools/materialize_im_database_contract.mjs',
  'db:contract:check must verify the composed database contract without writing it',
);

const materializer = read('tools/materialize_im_database_contract.mjs');
assert.match(materializer, /databaseRoot, 'migrations', 'postgres'/u);
assert.match(materializer, /\.up\.sql/u);
assert.match(materializer, /baseline-plus-migrations/u);
for (const retiredRegistry of [
  'specs/database-prefix-registry.json',
  'specs/database-table-registry.json',
]) {
  assert.equal(
    fs.existsSync(path.join(repoRoot, retiredRegistry)),
    false,
    `${retiredRegistry} must remain deleted; database/contract is the single authority`,
  );
}

process.stdout.write('sdkwork-im database framework standard contract passed\n');
