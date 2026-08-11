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

const spaceWrites = readExists('adapters/social-postgres/src/space_materialize_writes.rs');
for (const symbol of [
  'materialize_space_commits_on_transaction',
  'materialize_space_commit_on',
  'SPACE_MEMBER_LOCK_PARENT_SQL',
  'SPACE_MEMBER_COUNT_SQL',
  'insert_space_member_within_capacity',
]) {
  assert.ok(spaceWrites.includes(symbol), `space_materialize_writes must implement ${symbol}`);
}
assert.match(
  spaceWrites,
  /pub fn materialize_space_commits_on_transaction\(\s*\n\s*txn: &mut postgres::Transaction/,
  'space multi-commit materialization must run on the caller-owned PostgreSQL transaction',
);

const writeAuthority = readExists('services/space-service/src/write_authority.rs');
assert.match(
  writeAuthority,
  /if commits\.is_empty\(\)[\s\S]*materialize_space_commits_on_transaction/,
  'Space write authority must route all commit batches through transactional materialize',
);
assert.match(
  writeAuthority,
  /append_batch_with_allocated_sequences_in_transaction/,
  'journal rows and normalized Space rows must commit or roll back as one database unit',
);
assert.doesNotMatch(
  writeAuthority,
  /compensat(?:e|ion)/u,
  'space multi-commit materialization must rely on one PostgreSQL transaction instead of ad hoc compensation',
);
assert.match(
  writeAuthority,
  /let commits = vec!\[[\s\S]*SpaceCreated[\s\S]*SpaceMemberJoined/,
  'space create must emit space.created + space.member_joined multi-commit batch',
);

const lib = readExists('adapters/social-postgres/src/lib.rs');
assert.match(
  lib,
  /pub use space_materialize_writes::\{[\s\S]*materialize_space_commits_on_transaction/u,
  'social-postgres lib must export materialize_space_commits_on_transaction',
);

const spaceDoc = readExists('docs/architecture/tech/TECH-im-space-open-api-alignment.md');
assert.match(
  spaceDoc,
  /single PostgreSQL transaction|单.*PG.*事务|one PostgreSQL transaction/i,
  'space alignment doc must document transactional multi-commit materialization',
);

process.stdout.write('sdkwork-im space materializer standard passed\n');
