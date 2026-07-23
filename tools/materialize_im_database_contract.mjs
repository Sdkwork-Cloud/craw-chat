#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { parse, stringify } from 'yaml';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const databaseRoot = path.join(repositoryRoot, 'database');
const contractRoot = path.join(databaseRoot, 'contract');
const manifestPath = path.join(databaseRoot, 'database.manifest.json');
const schemaPath = path.join(contractRoot, 'schema.yaml');
const tableRegistryPath = path.join(contractRoot, 'table-registry.json');
const writeMode = process.argv.includes('--write');

for (const retiredRegistry of [
  'specs/database-prefix-registry.json',
  'specs/database-table-registry.json',
]) {
  assert.equal(
    fs.existsSync(path.join(repositoryRoot, retiredRegistry)),
    false,
    `${retiredRegistry} is a redundant database authority and must remain deleted`,
  );
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function listSqlFiles(directoryPath, suffix) {
  return fs
    .readdirSync(directoryPath)
    .filter((entry) => entry.endsWith(suffix))
    .sort((left, right) => left.localeCompare(right))
    .map((entry) => path.join(directoryPath, entry));
}

function collectCreatedTables(sql) {
  return [...sql.matchAll(/\bCREATE\s+TABLE(?:\s+IF\s+NOT\s+EXISTS)?\s+([a-z_][a-z0-9_]*)\s*\(/giu)]
    .map((match) => match[1].toLowerCase());
}

function relativePath(filePath) {
  return path.relative(repositoryRoot, filePath).replaceAll('\\', '/');
}

function collectLifecycleSources() {
  const baselineFiles = listSqlFiles(
    path.join(databaseRoot, 'ddl', 'baseline', 'postgres'),
    '.sql',
  );
  const migrationFiles = listSqlFiles(
    path.join(databaseRoot, 'migrations', 'postgres'),
    '.up.sql',
  );
  return [...baselineFiles, ...migrationFiles].map((filePath) => ({
    filePath,
    relativePath: relativePath(filePath),
    source: fs.readFileSync(filePath, 'utf8'),
  }));
}

function uniqueInOrder(values) {
  return [...new Set(values)];
}

function assertUnique(values, label) {
  assert.equal(values.length, new Set(values).size, `${label} must not contain duplicates`);
}

const manifest = readJson(manifestPath);
assert.equal(
  manifest.baselineStrategy,
  'baseline-plus-migrations',
  'IM database lifecycle must compose its immutable baseline with versioned migrations',
);
assert.deepEqual(manifest.engines, ['postgres'], 'PostgreSQL must remain the IM runtime authority');

const lifecycleSources = collectLifecycleSources();
const effectiveTableNames = uniqueInOrder(
  lifecycleSources.flatMap(({ source }) => collectCreatedTables(source)),
);
assert.ok(effectiveTableNames.length > 0, 'database lifecycle sources must create IM tables');
for (const tableName of effectiveTableNames) {
  assert.match(tableName, /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u);
}

const registry = readJson(tableRegistryPath);
assert.equal(registry.kind, 'sdkwork.database.table-registry');
const registryByName = new Map();
for (const table of registry.tables ?? []) {
  for (const field of [
    'table_name',
    'owner',
    'compliance_level',
    'lifecycle_status',
    'module_prefix',
    'bounded_context',
    'table_profile',
    'write_owner',
    'migration',
  ]) {
    assert.ok(table[field], `${table.table_name ?? '<unnamed>'} must declare ${field}`);
  }
  assert.equal(typeof table.system_of_record, 'boolean');
  assert.match(table.table_name, /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u);
  assert.equal(table.module_prefix, 'im');
  assert.ok(!registryByName.has(table.table_name), `duplicate table contract ${table.table_name}`);
  registryByName.set(table.table_name, table);

  const migrationSource = lifecycleSources.find(
    (candidate) => candidate.relativePath === table.migration,
  );
  assert.ok(migrationSource, `${table.table_name} references unknown lifecycle source ${table.migration}`);
  assert.ok(
    collectCreatedTables(migrationSource.source).includes(table.table_name),
    `${table.migration} must create registered table ${table.table_name}`,
  );
}

const registryTableNames = [...registryByName.keys()];
assertUnique(registryTableNames, 'database table registry');
assert.deepEqual(
  [...registryTableNames].sort(),
  [...effectiveTableNames].sort(),
  'canonical table registry must equal baseline-plus-migrations lifecycle tables',
);

const schema = parse(fs.readFileSync(schemaPath, 'utf8'));
assert.equal(schema.kind, 'sdkwork.database.schema');
assert.equal(String(schema.contract_version), manifest.contractVersion);
const schemaTableNames = (schema.tables ?? []).map((table) => table.name);
assertUnique(schemaTableNames, 'database schema contract');
assert.deepEqual(
  [...schemaTableNames].sort(),
  [...effectiveTableNames].sort(),
  'schema contract must equal baseline-plus-migrations lifecycle tables',
);

const orderedRegistry = {
  ...registry,
  tables: effectiveTableNames.map((tableName) => registryByName.get(tableName)),
};
const orderedSchema = {
  ...schema,
  tables: effectiveTableNames.map((tableName) => ({
    name: tableName,
    lifecycle_status: registryByName.get(tableName).lifecycle_status,
    owner: registryByName.get(tableName).owner,
  })),
};

if (writeMode) {
  fs.writeFileSync(tableRegistryPath, `${JSON.stringify(orderedRegistry, null, 2)}\n`);
  fs.writeFileSync(schemaPath, stringify(orderedSchema, { lineWidth: 0 }));
} else {
  assert.deepEqual(
    registryTableNames,
    effectiveTableNames,
    'canonical table registry order is stale; run pnpm db:materialize:contract',
  );
  assert.deepEqual(
    schemaTableNames,
    effectiveTableNames,
    'schema contract order is stale; run pnpm db:materialize:contract',
  );
}

process.stdout.write(
  `${writeMode ? 'materialized' : 'validated'} ${effectiveTableNames.length} IM tables from baseline plus ${lifecycleSources.length - 1} PostgreSQL migrations\n`,
);
