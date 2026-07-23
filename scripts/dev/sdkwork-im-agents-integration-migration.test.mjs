import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const migrationRoot = path.join(repoRoot, 'database', 'migrations', 'postgres');
const integrationTables = [
  'im_conversation_agent_assignments',
  'im_conversation_agent_binding',
  'im_agent_dispatch',
];

test('IM Agents integration migration is paired and IM-owned', () => {
  const name = '0005_agents_integration_expand';
  const up = readFileSync(path.join(migrationRoot, `${name}.up.sql`), 'utf8');
  const down = readFileSync(path.join(migrationRoot, `${name}.down.sql`), 'utf8');
  const subjectGuardUp = readFileSync(
    path.join(migrationRoot, '0006_agents_integration_subject_guard.up.sql'),
    'utf8',
  );
  const systemActorCompatibilityUp = readFileSync(
    path.join(migrationRoot, '0008_allow_system_assignment_actor.up.sql'),
    'utf8',
  );
  const zeroBasedAggregateVersionUp = readFileSync(
    path.join(migrationRoot, '0009_allow_zero_source_aggregate_version.up.sql'),
    'utf8',
  );

  for (const table of integrationTables) {
    assert.match(up, new RegExp(`CREATE\\s+TABLE\\s+${table}\\b`, 'iu'));
    assert.match(down, new RegExp(`DROP\\s+TABLE\\s+${table}\\b`, 'iu'));
  }

  assert.match(up, /binding_id\s+VARCHAR\(128\)/iu);
  assert.match(up, /FOR UPDATE SKIP LOCKED|idx_im_agent_dispatch_worker/iu);
  assert.match(up, /tenant_id\s+BIGINT\s+NOT\s+NULL/iu);
  assert.match(up, /organization_id\s+BIGINT\s+NOT\s+NULL/iu);
  assert.match(subjectGuardUp, /tenant_id\s*>\s*0\s+AND\s+organization_id\s*>=\s*0/iu);
  assert.match(subjectGuardUp, /assigned_by\s*>\s*0/iu);
  assert.match(systemActorCompatibilityUp, /assigned_by\s*>=\s*0/iu);
  assert.match(
    zeroBasedAggregateVersionUp,
    /assignment_generation\s*>\s*0\s+AND\s+source_aggregate_version\s*>=\s*0/iu,
  );
  assert.match(subjectGuardUp, /created_by\s*>\s*0\s+AND\s+updated_by\s*>\s*0/iu);
  assert.match(subjectGuardUp, /requested_by\s*>\s*0/iu);
  assert.match(subjectGuardUp, /NOT\s+VALID/iu);
  assert.match(subjectGuardUp, /VALIDATE\s+CONSTRAINT/iu);
  assert.match(down, /rollback refused/iu);
  assert.doesNotMatch(up, /\b(?:CREATE|ALTER|REFERENCES|INSERT|UPDATE|DELETE)\b[^;]*\bai_agent_/iu);
});

test('new IM migrations are paired', () => {
  const files = readdirSync(migrationRoot);
  for (const upName of files.filter(
    (fileName) => /^(?:000[5-9]|00[1-9][0-9]|0[1-9][0-9]{2}|[1-9][0-9]{3})_.*\.up\.sql$/u.test(fileName),
  )) {
    const downName = upName.replace(/\.up\.sql$/u, '.down.sql');
    assert.ok(files.includes(downName), `${upName} must have ${downName}`);
  }
});

test('IM Agents database contract 2.0 is active and range-safe', () => {
  const manifest = JSON.parse(
    readFileSync(path.join(repoRoot, 'database', 'database.manifest.json'), 'utf8'),
  );
  const contract = readFileSync(
    path.join(repoRoot, 'database', 'contract', 'schema.yaml'),
    'utf8',
  );
  const registry = JSON.parse(
    readFileSync(path.join(repoRoot, 'database', 'contract', 'table-registry.json'), 'utf8'),
  );
  const adapter = readFileSync(
    path.join(repoRoot, 'adapters', 'postgres-journal', 'src', 'agent_integration_store.rs'),
    'utf8',
  );

  assert.equal(manifest.contractVersion, '2.0.0');
  assert.match(contract, /contract_version:\s*2\.0\.0/u);
  for (const table of integrationTables) {
    assert.match(
      contract,
      new RegExp(
        `name:[^\\S\\r\\n]*${table}[^\\S\\r\\n]*\\r?\\n[^\\S\\r\\n]*lifecycle_status:[^\\S\\r\\n]*active`,
        'u',
      ),
    );
    const entry = registry.tables.find((candidate) => candidate.table_name === table);
    assert.equal(entry?.lifecycle_status, 'active', `${table} must be active in the table registry`);
    assert.equal(
      entry?.migration,
      'database/migrations/postgres/0005_agents_integration_expand.up.sql',
      `${table} must retain immutable migration provenance`,
    );
  }
  assert.match(adapter, /value\s*>\s*i64::MAX\s+as\s+u64/u);
  assert.match(adapter, /validate_signed_id\(request\.message_seq/u);
  assert.match(adapter, /validate_signed_id\([\s\S]*request\.assignment_generation/u);
});
