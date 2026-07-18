#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const migrationSql = fs.readFileSync(
  path.join(repoRoot, 'database', 'migrations', 'sqlite', '0004_stream_transactional_authority.up.sql'),
  'utf8',
);
const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-stream-migration-'));
const database = new DatabaseSync(path.join(directory, 'migration.sqlite'));

try {
  database.exec(`
    CREATE TABLE im_stream_sessions (
      tenant_id TEXT NOT NULL,
      organization_id TEXT NOT NULL,
      stream_id TEXT NOT NULL,
      stream_state TEXT NOT NULL,
      PRIMARY KEY (tenant_id, organization_id, stream_id)
    );
    INSERT INTO im_stream_sessions VALUES ('100001', '100001', 'stream-a', 'active');
  `);
  database.exec(migrationSql);
  const columns = database.prepare('PRAGMA table_info(im_stream_sessions)').all();
  assert(columns.some((column) => column.name === 'version' && column.notnull === 1));
  assert.equal(
    database.prepare("SELECT version FROM im_stream_sessions WHERE stream_id = 'stream-a'").get().version,
    1,
  );
  const indexes = database.prepare('PRAGMA index_list(im_stream_sessions)').all();
  assert(indexes.some((index) => index.name === 'idx_im_stream_sessions_active'));
  console.log('sdkwork-im stream transactional authority SQLite migration test passed');
} finally {
  database.close();
  fs.rmSync(directory, { recursive: true, force: true });
}
