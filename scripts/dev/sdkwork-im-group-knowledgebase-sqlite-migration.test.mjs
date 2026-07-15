#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const migrationPath = path.join(
  repoRoot,
  'database',
  'migrations',
  'sqlite',
  '0003_group_knowledgebase_binding.up.sql',
);
const migrationSql = fs.readFileSync(migrationPath, 'utf8');

const LEGACY_SCHEMA_SQL = `
CREATE TABLE im_conversation_knowledge_space_link (
    id INTEGER NOT NULL,
    link_uuid TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER,
    knowledge_space_uuid TEXT,
    knowledgebase_binding_id INTEGER,
    lifecycle_state TEXT NOT NULL DEFAULT 'provisioning',
    provisioning_operation_id TEXT,
    creation_idempotency_key TEXT NOT NULL,
    last_source_event_id TEXT,
    membership_epoch INTEGER NOT NULL DEFAULT 0,
    last_synchronized_membership_epoch INTEGER NOT NULL DEFAULT 0,
    last_error_code TEXT,
    last_error_at TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TEXT,
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, organization_id, conversation_id),
    UNIQUE (id),
    UNIQUE (link_uuid)
);

CREATE TABLE im_group_knowledge_launch_tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    ticket_hash TEXT NOT NULL UNIQUE,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER NOT NULL,
    knowledge_space_uuid TEXT NOT NULL,
    binding_version INTEGER NOT NULL,
    membership_epoch INTEGER NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    issuing_app_id TEXT,
    issued_by TEXT NOT NULL,
    idempotency_key_hash TEXT NOT NULL,
    request_fingerprint_hash TEXT NOT NULL,
    ticket_ciphertext TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    consumed_at TEXT,
    consumed_by_service TEXT,
    consumed_trace_id TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (
        tenant_id, organization_id, conversation_id, actor_kind, actor_id,
        principal_kind, principal_id, session_id, idempotency_key_hash
    )
);
`;

function runSql(database, sql) {
  database.exec(sql);
}

function queryScalar(database, sql) {
  const row = database.prepare(sql).get();
  assert.ok(row, `query returned no row: ${sql}`);
  const [value] = Object.values(row);
  return String(value);
}

function tableColumns(database, tableName) {
  return database
    .prepare(`SELECT name FROM pragma_table_info('${tableName}') ORDER BY cid;`)
    .all()
    .map((row) => row.name);
}

function withTemporaryDatabase(callback) {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-group-kb-migration-'));
  const databasePath = path.join(directory, 'migration.sqlite');
  const database = new DatabaseSync(databasePath);
  try {
    callback(database);
  } finally {
    database.close();
    fs.rmSync(directory, { recursive: true, force: true });
  }
}

function seedLegacySchema(database) {
  runSql(database, LEGACY_SCHEMA_SQL);
}

function runMigrationExpectingRollback(database) {
  let error;
  try {
    runSql(database, migrationSql);
  } catch (caught) {
    error = caught;
  }
  assert.ok(error instanceof Error, 'migration unexpectedly succeeded');
  assert.match(error.message, /constraint failed|NOT NULL constraint failed/u);
  database.exec('ROLLBACK;');
}

function assertMigrationRollbackForLegacyActiveLink() {
  withTemporaryDatabase((database) => {
    seedLegacySchema(database);
    runSql(
      database,
      `
INSERT INTO im_conversation_knowledge_space_link (
    id, link_uuid, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    lifecycle_state, creation_idempotency_key, created_by, updated_by
) VALUES (
    1, 'link-legacy-active', 'tenant-1', '1', 'conversation-1',
    9, 'space-uuid-1', 11, 'active', 'create-1', 'owner-1', 'owner-1'
);
`,
    );

    runMigrationExpectingRollback(database);
    assert.equal(
      queryScalar(
        database,
        'SELECT COUNT(*) AS value FROM im_conversation_knowledge_space_link;',
      ),
      '1',
      'a rejected legacy active link must leave the original row intact',
    );
    assert.ok(
      !tableColumns(database, 'im_conversation_knowledge_space_link').includes(
        'knowledgebase_binding_uuid',
      ),
      'failed migration must roll back to the legacy link table shape',
    );
    assert.equal(
      queryScalar(
        database,
        "SELECT COUNT(*) AS value FROM sqlite_master WHERE name LIKE '%_legacy_0003';",
      ),
      '0',
      'failed migration must not leave a renamed legacy table behind',
    );
  });
}

function assertMigrationRollbackForLegacyLaunchTicket() {
  withTemporaryDatabase((database) => {
    seedLegacySchema(database);
    runSql(
      database,
      `
INSERT INTO im_group_knowledge_launch_tickets (
    id, ticket_hash, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, binding_version, membership_epoch,
    actor_kind, actor_id, principal_kind, principal_id, session_id,
    issued_by, idempotency_key_hash, request_fingerprint_hash,
    ticket_ciphertext, expires_at, created_at
) VALUES (
    1, 'ticket-hash-legacy', 'tenant-1', '1', 'conversation-1',
    9, 'space-uuid-1', 1, 0, 'user', 'owner-1', 'user', 'owner-1',
    'session-1', 'owner-1', 'idempotency-hash-1', 'fingerprint-hash-1',
    'ciphertext-1', '2026-01-02T00:00:00Z', '2026-01-01T00:00:00Z'
);
`,
    );

    runMigrationExpectingRollback(database);
    assert.equal(
      queryScalar(database, 'SELECT COUNT(*) AS value FROM im_group_knowledge_launch_tickets;'),
      '1',
      'a rejected legacy launch ticket must leave the original row intact',
    );
    assert.ok(
      tableColumns(database, 'im_group_knowledge_launch_tickets').includes('binding_version'),
      'failed migration must restore the legacy launch-ticket table shape',
    );
    assert.ok(
      !tableColumns(database, 'im_group_knowledge_launch_tickets').includes(
        'knowledgebase_binding_uuid',
      ),
      'failed migration must not manufacture an immutable ticket target',
    );
  });
}

function validProvisioningLinkSql(tenantId, organizationId, id) {
  return `
INSERT INTO im_conversation_knowledge_space_link (
    id, link_uuid, tenant_id, organization_id, conversation_id,
    lifecycle_state, creation_idempotency_key, created_by, updated_by
) VALUES (
    ${id}, 'link-${id}', '${tenantId}', '${organizationId}', 'conversation-${id}',
    'provisioning', 'create-${id}', 'owner-1', 'owner-1'
);
`;
}

function validLaunchTicketSql(tenantId, organizationId, id) {
  return `
INSERT INTO im_group_knowledge_launch_tickets (
    id, ticket_hash, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
    actor_kind, actor_id, principal_kind, principal_id, session_id,
    issued_by, idempotency_key_hash, request_fingerprint_hash,
    ticket_ciphertext, expires_at, created_at
) VALUES (
    ${id}, 'ticket-hash-${id}', '${tenantId}', '${organizationId}', 'conversation-ticket-${id}',
    9, 'space-uuid-${id}', 11, 'binding-uuid-${id}', 1, 0,
    'user', 'owner-1', 'user', 'owner-1', 'session-1', 'owner-1',
    'idempotency-hash-${id}', 'fingerprint-hash-${id}', 'ciphertext-${id}',
    '2026-01-02T00:00:00Z', '2026-01-01T00:00:00Z'
);
`;
}

function assertCleanUpgradeAndSignedI64Boundary() {
  withTemporaryDatabase((database) => {
    runSql(database, migrationSql);

    const linkColumns = tableColumns(database, 'im_conversation_knowledge_space_link');
    assert.ok(linkColumns.includes('knowledgebase_binding_uuid'));

    const ticketColumns = tableColumns(database, 'im_group_knowledge_launch_tickets');
    assert.ok(
      ['knowledgebase_binding_id', 'knowledgebase_binding_uuid', 'upstream_link_generation'].every(
        (column) => ticketColumns.includes(column),
      ),
      'upgraded launch tickets must persist the complete immutable target fence',
    );
    assert.ok(
      !ticketColumns.includes('binding_version'),
      'upgraded launch tickets must not retain the retired IM link version field',
    );

    const maxSignedI64 = '9223372036854775807';
    const overflowSignedI64 = '9223372036854775808';
    runSql(database, validProvisioningLinkSql(maxSignedI64, maxSignedI64, 1));
    runSql(database, validLaunchTicketSql(maxSignedI64, maxSignedI64, 1));

    assert.throws(
      () => runSql(database, validProvisioningLinkSql(overflowSignedI64, maxSignedI64, 2)),
      /tenant_id|constraint failed/u,
    );
    assert.throws(
      () => runSql(database, validLaunchTicketSql(overflowSignedI64, maxSignedI64, 2)),
      /tenant_id|constraint failed/u,
    );
    assert.throws(
      () => runSql(database, validProvisioningLinkSql(maxSignedI64, overflowSignedI64, 3)),
      /organization_id|constraint failed/u,
    );
    assert.throws(
      () => runSql(database, validLaunchTicketSql(maxSignedI64, overflowSignedI64, 3)),
      /organization_id|constraint failed/u,
    );

    for (const [index, invalidTenantId] of ['0', '01', ' 1', '1 ', 'tenant-1'].entries()) {
      const id = index + 10;
      assert.throws(
        () => runSql(database, validProvisioningLinkSql(invalidTenantId, maxSignedI64, id)),
        /tenant_id|constraint failed/u,
      );
      assert.throws(
        () => runSql(database, validLaunchTicketSql(invalidTenantId, maxSignedI64, id)),
        /tenant_id|constraint failed/u,
      );
    }
  });
}

assertMigrationRollbackForLegacyActiveLink();
assertMigrationRollbackForLegacyLaunchTicket();
assertCleanUpgradeAndSignedI64Boundary();

process.stdout.write('sdkwork-im group knowledgebase SQLite migration smoke test passed\n');
