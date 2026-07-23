import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import pg from 'pg';

import { resolvePostgresDevProfile } from './sdkwork-im-postgres-dev-profile.mjs';

const { Client } = pg;
const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const migrationRoot = path.join(repoRoot, 'database', 'migrations', 'postgres');
const migrationName = '0011_typed_message_interaction_principals';
const upSql = fs.readFileSync(path.join(migrationRoot, `${migrationName}.up.sql`), 'utf8');
const downSql = fs.readFileSync(path.join(migrationRoot, `${migrationName}.down.sql`), 'utf8');

const legacySchemaSql = `
CREATE TABLE im_conversation_messages (
    tenant_id TEXT NOT NULL,
    message_id BIGINT NOT NULL,
    CONSTRAINT uk_smoke_message UNIQUE (tenant_id, message_id)
);

CREATE TABLE im_message_reactions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    message_id BIGINT NOT NULL,
    user_id TEXT NOT NULL,
    reaction_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_reactions PRIMARY KEY (
        tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type
    )
);
CREATE INDEX idx_im_message_reactions_user
    ON im_message_reactions (tenant_id, organization_id, user_id, created_at DESC);

CREATE TABLE im_message_pins (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    message_id BIGINT NOT NULL,
    pinned_by_user_id TEXT NOT NULL,
    pin_reason TEXT,
    pinned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_pins PRIMARY KEY (
        tenant_id, organization_id, conversation_id, message_id
    )
);
CREATE INDEX idx_im_message_pins_user
    ON im_message_pins (tenant_id, organization_id, pinned_by_user_id, pinned_at DESC);

INSERT INTO im_conversation_messages (tenant_id, message_id) VALUES ('tenant-smoke', 9001);
INSERT INTO im_message_reactions (
    tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type
) VALUES ('tenant-smoke', '0', 'conversation-smoke', 9001, 'user-1', 'thumbs_up');
INSERT INTO im_message_pins (
    tenant_id, organization_id, conversation_id, message_id, pinned_by_user_id
) VALUES ('tenant-smoke', '0', 'conversation-smoke', 9001, 'user-1');
`;

async function columns(client, tableName) {
  const result = await client.query(
    `SELECT column_name
       FROM information_schema.columns
      WHERE table_schema = current_schema() AND table_name = $1
      ORDER BY ordinal_position`,
    [tableName],
  );
  return result.rows.map((row) => row.column_name);
}

test('typed message interaction PostgreSQL migration applies and rolls back', async () => {
  const profile = resolvePostgresDevProfile({ repoRoot });
  assert.equal(profile.kind, 'postgresql');
  const client = new Client({ connectionString: profile.databaseUrl });
  const schema = `im_message_mutation_smoke_${process.pid}_${Date.now()}`;
  await client.connect();
  try {
    await client.query(`CREATE SCHEMA "${schema}"`);
    await client.query(`SET search_path TO "${schema}"`);
    await client.query(legacySchemaSql);
    await client.query(upSql);

    const reactionColumns = await columns(client, 'im_message_reactions');
    assert.ok(reactionColumns.includes('actor_principal_kind'));
    assert.ok(reactionColumns.includes('actor_principal_id'));
    assert.ok(!reactionColumns.includes('user_id'));
    const pinColumns = await columns(client, 'im_message_pins');
    assert.ok(pinColumns.includes('pinned_by_principal_kind'));
    assert.ok(pinColumns.includes('pinned_by_principal_id'));
    assert.ok(!pinColumns.includes('pinned_by_user_id'));

    const reaction = await client.query(
      `SELECT actor_principal_kind, actor_principal_id FROM im_message_reactions`,
    );
    assert.deepEqual(reaction.rows, [
      { actor_principal_kind: 'user', actor_principal_id: 'user-1' },
    ]);
    const pin = await client.query(
      `SELECT pinned_by_principal_kind, pinned_by_principal_id FROM im_message_pins`,
    );
    assert.deepEqual(pin.rows, [
      { pinned_by_principal_kind: 'user', pinned_by_principal_id: 'user-1' },
    ]);

    await client.query(downSql);
    const rolledBackReactionColumns = await columns(client, 'im_message_reactions');
    assert.ok(rolledBackReactionColumns.includes('user_id'));
    assert.ok(!rolledBackReactionColumns.includes('actor_principal_kind'));
    assert.ok(!rolledBackReactionColumns.includes('actor_principal_id'));
    const rolledBackPinColumns = await columns(client, 'im_message_pins');
    assert.ok(rolledBackPinColumns.includes('pinned_by_user_id'));
    assert.ok(!rolledBackPinColumns.includes('pinned_by_principal_kind'));
    assert.ok(!rolledBackPinColumns.includes('pinned_by_principal_id'));
  } finally {
    await client.query('ROLLBACK').catch(() => {});
    await client.query('SET search_path TO public').catch(() => {});
    await client.query(`DROP SCHEMA IF EXISTS "${schema}" CASCADE`).catch(() => {});
    await client.end();
  }
});
