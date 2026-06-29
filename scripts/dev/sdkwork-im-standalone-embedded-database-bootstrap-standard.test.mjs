#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const mainSource = read('services/sdkwork-im-standalone-gateway/src/main.rs');
const embeddedSource = read(
  'services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs',
);
const gatewayCargo = read('services/sdkwork-im-standalone-gateway/Cargo.toml');
const driveInstaller = read(
  '../sdkwork-drive/crates/sdkwork-drive-workspace-service/src/infrastructure/sql/installer.rs',
);

assert.match(
  mainSource,
  /bootstrap_embedded_dependency_databases\(\)/u,
  'standalone gateway must synchronize embedded dependency databases before mounting routes',
);

assert.match(
  embeddedSource,
  /pub async fn bootstrap_embedded_dependency_databases/u,
  'embedded dependency routes must expose a database lifecycle sync entrypoint',
);

assert.match(
  embeddedSource,
  /bootstrap_drive_database_from_env/u,
  'drive embedded database sync must use sdkwork-database lifecycle bootstrap',
);

assert.match(
  embeddedSource,
  /bootstrap_knowledgebase_database_from_env/u,
  'knowledgebase embedded database sync must use sdkwork-database lifecycle bootstrap',
);

assert.match(
  embeddedSource,
  /normalize_embedded_dependency_database_urls/u,
  'embedded dependency env must normalize PostgreSQL search_path for unified schema profiles',
);

assert.match(
  embeddedSource,
  /apply_embedded_dependency_app_roots/u,
  'embedded dependency env must set sibling app roots for database module discovery',
);

assert.match(
  gatewayCargo,
  /sdkwork-knowledgebase-database-host/u,
  'standalone gateway must depend on knowledgebase database host for lifecycle sync',
);

assert.match(
  gatewayCargo,
  /sdkwork-mail-database-host/u,
  'standalone gateway must depend on mail database host for lifecycle sync',
);

assert.match(
  driveInstaller,
  /postgres_url_with_search_path/u,
  'drive postgres pool bootstrap must honor unified PostgreSQL search_path',
);

process.stdout.write(
  'sdkwork-im standalone embedded database bootstrap standard contract passed\n',
);
