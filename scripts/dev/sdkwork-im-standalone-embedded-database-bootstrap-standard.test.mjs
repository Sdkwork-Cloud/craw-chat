#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const mainSource = read('crates/sdkwork-api-im-standalone-gateway/src/main.rs');
const embeddedSource = read(
  'crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs',
);
const gatewayCargo = read('crates/sdkwork-api-im-standalone-gateway/Cargo.toml');
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
  /connect_any_database_and_install_schema/u,
  'drive embedded database sync must install schema through the shared Drive lifecycle entrypoint',
);

assert.match(
  embeddedSource,
  /bootstrap_knowledgebase_database_from_env/u,
  'knowledgebase embedded database sync must use sdkwork-database lifecycle bootstrap',
);

assert.match(
  embeddedSource,
  /validate_workspace_server_database_env/u,
  'embedded dependency bootstrap must validate one process-level workspace database profile',
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
  /normalize_workspace_postgres_url/u,
  'drive postgres pool bootstrap must normalize the workspace PostgreSQL profile',
);

process.stdout.write(
  'sdkwork-im standalone embedded database bootstrap standard contract passed\n',
);
