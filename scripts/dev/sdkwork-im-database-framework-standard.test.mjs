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

for (const engine of ['postgres', 'sqlite']) {
  const migrationUp = `database/migrations/${engine}/0001_im_legacy_baseline.up.sql`;
  const migrationDown = `database/migrations/${engine}/0001_im_legacy_baseline.down.sql`;
  assert.ok(fs.existsSync(path.join(repoRoot, migrationUp)), `${migrationUp} must exist for L2 lifecycle`);
  assert.ok(fs.existsSync(path.join(repoRoot, migrationDown)), `${migrationDown} must exist for L2 lifecycle`);
}

process.stdout.write('sdkwork-im database framework standard contract passed\n');
