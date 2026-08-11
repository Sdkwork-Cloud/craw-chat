import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const migrationRoot = path.join(repoRoot, 'database', 'migrations', 'postgres');
const baselineRoot = path.join(repoRoot, 'database', 'ddl', 'baseline', 'postgres');
const integrationTables = [
  'im_conversation_agent_assignments',
  'im_conversation_agent_binding',
  'im_agent_dispatch',
];

test('IM Agents integration schema is baseline-owned and IM-owned', () => {
  // The three IM-owned tables and their guards were squashed into the
  // immutable baseline (`0001_im_baseline.sql`). The retired additive
  // migrations `0005_agents_integration_expand` / `0006_subject_guard` /
  // `0008_system_assignment_actor` / `0009_zero_source_aggregate_version`
  // were removed from `database/migrations/postgres/` and must not reappear.
  const baseline = readFileSync(path.join(baselineRoot, '0001_im_baseline.sql'), 'utf8');

  for (const table of integrationTables) {
    assert.match(baseline, new RegExp(`CREATE\\s+TABLE\\s+IF\\s+NOT\\s+EXISTS\\s+${table}\\b`, 'iu'));
  }

  // Subject guard: positive tenant/org scope and actor identifiers.
  assert.match(baseline, /tenant_id\s*>\s*0\s+AND\s+organization_id\s*>=\s*0\s+AND\s+assigned_by\s*>\s*0/iu);
  assert.match(baseline, /created_by\s*>\s*0\s+AND\s+updated_by\s*>\s*0/iu);
  assert.match(baseline, /tenant_id\s*>\s*0\s+AND\s+organization_id\s*>=\s*0\s+AND\s+requested_by\s*>\s*0/iu);
  // System-actor compatibility: `assigned_by >= 0` relaxation.
  assert.match(baseline, /tenant_id\s*>\s*0\s+AND\s+organization_id\s*>=\s*0\s+AND\s+assigned_by\s*>=\s*0/iu);
  // Generation/version guards.
  assert.match(baseline, /assignment_generation\s*>\s*0\s+AND\s+source_aggregate_version\s*>\s*0/iu);
  assert.match(baseline, /binding_id\s+VARCHAR\(128\)/iu);
  assert.match(baseline, /FOR\s+UPDATE\s+SKIP\s+LOCKED|idx_im_agent_dispatch_worker/iu);
  assert.match(baseline, /tenant_id\s+BIGINT\s+NOT\s+NULL/iu);
  assert.match(baseline, /organization_id\s+BIGINT\s+NOT\s+NULL/iu);
  assert.doesNotMatch(
    baseline,
    /\b(?:CREATE|ALTER|REFERENCES|INSERT|UPDATE|DELETE)\b[^;]*\bai_agent_/iu,
  );
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

test('IM Agents database contract 2.1.0 is active and range-safe', () => {
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

  assert.equal(manifest.contractVersion, '2.1.0');
  assert.match(contract, /contract_version:\s*2\.1\.0/u);
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
      'database/ddl/baseline/postgres/0001_im_baseline.sql',
      `${table} must retain immutable baseline provenance`,
    );
  }
  assert.match(adapter, /value\s*>\s*i64::MAX\s+as\s+u64/u);
  assert.match(adapter, /validate_signed_id\(request\.message_seq/u);
  assert.match(adapter, /validate_signed_id\([\s\S]*request\.assignment_generation/u);
});
