import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readExists(relativePath) {
  const absolutePath = path.join(repoRoot, ...relativePath.split('/'));
  assert.ok(fs.existsSync(absolutePath), `expected file ${relativePath}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

const materializeWrites = readExists('adapters/social-postgres/src/materialize_writes.rs');
for (const symbol of [
  'materialize_commits_on_transaction',
  'materialize_commit_on',
  'FRIEND_REQUEST_UPDATE_STATUS_SQL',
  'FRIENDSHIP_UPSERT_ACTIVE_PAIR_SQL',
]) {
  assert.ok(materializeWrites.includes(symbol), `materialize_writes must implement ${symbol}`);
}
assert.match(
  materializeWrites,
  /pub fn materialize_commits_on_transaction\(\s*\n\s*txn: &mut postgres::Transaction/,
  'multi-commit social materialization must run on the caller-owned PostgreSQL transaction',
);

const writeAuthority = readExists('services/social-service/src/postgres_write_authority.rs');
assert.match(
  writeAuthority,
  /if commits\.is_empty\(\)[\s\S]*write_commits_on_transaction/,
  'Social write authority must route all commit batches through the caller-owned transactional materialize',
);
assert.match(
  writeAuthority,
  /append_batch_with_allocated_sequences_in_transaction/,
  'journal rows and normalized Social rows must commit or roll back as one database unit',
);
assert.doesNotMatch(
  writeAuthority,
  /commits\.len\(\)\s*>\s*1/,
  'Social write authority must not gate transactions on multi-commit length only',
);

const lib = readExists('adapters/social-postgres/src/lib.rs');
assert.match(
  lib,
  /pub use materialize_writes::materialize_commits_on_transaction/u,
  'social-postgres lib must export materialize_commits_on_transaction',
);

const socialDoc = readExists('docs/architecture/tech/TECH-im-social-open-api-alignment.md');
assert.match(
  socialDoc,
  /single PostgreSQL transaction|单.*PG.*事务|one PostgreSQL transaction/i,
  'social alignment doc must document transactional multi-commit materialization',
);

const auditDoc = readExists('docs/COMMUNICATION_FEATURE_AUDIT_REPORT.md');
assert.match(
  auditDoc,
  /Social.*单 PG 事务|Social.*single PG transaction/i,
  'audit report must record social materializer transactional batch alignment',
);

const commercialGates = readExists('.github/workflows/im-commercial-gates.yml');
assert.ok(
  commercialGates.includes('sdkwork-im-social-materializer-standard.test.mjs'),
  'im-commercial-gates.yml must run social materializer standard test',
);

process.stdout.write('sdkwork-im social materializer standard passed\n');
