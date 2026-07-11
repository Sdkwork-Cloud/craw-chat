//! Principal-scoped, bounded desktop offline cache backed by SQLite.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const OFFLINE_DB_FILE: &str = "offline-im-cache.sqlite";
const OFFLINE_SCHEMA_VERSION: i64 = 3;
const DEFAULT_PAGE_LIMIT: usize = 20;
const MAX_PAGE_LIMIT: usize = 200;
const MAX_WRITE_BATCH: usize = 200;
const MAX_BATCH_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECORD_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;
const MAX_CURSOR_BYTES: usize = 64 * 1024;
const PENDING_SEND_CLAIM_LEASE_MS: i64 = 60_000;
const MAX_PENDING_SEND_ROWS_PER_SCOPE: i64 = 10_000;
const MAX_PENDING_SEND_BYTES_PER_SCOPE: i64 = 64 * 1024 * 1024;

#[derive(Clone, Copy)]
struct PendingSendQuarantinePolicy {
    retention_ms: i64,
    row_limit: i64,
    byte_budget: i64,
}

const PENDING_SEND_QUARANTINE_POLICY: PendingSendQuarantinePolicy = PendingSendQuarantinePolicy {
    retention_ms: 30 * 24 * 60 * 60 * 1_000,
    row_limit: 1_000,
    byte_budget: 16 * 1024 * 1024,
};

#[derive(Clone, Copy)]
struct CachePolicy {
    retention_ms: i64,
    conversation_row_limit: i64,
    conversation_byte_budget: i64,
    message_row_limit: i64,
    message_byte_budget: i64,
    cursor_row_limit: i64,
    cursor_byte_budget: i64,
}

const OFFLINE_CACHE_POLICY: CachePolicy = CachePolicy {
    retention_ms: 30 * 24 * 60 * 60 * 1_000,
    conversation_row_limit: 10_000,
    conversation_byte_budget: 32 * 1024 * 1024,
    message_row_limit: 100_000,
    message_byte_budget: 192 * 1024 * 1024,
    cursor_row_limit: 1_000,
    cursor_byte_budget: 1024 * 1024,
};

static OFFLINE_DB: Mutex<Option<Connection>> = Mutex::new(None);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePrincipalScope {
    pub tenant_id: String,
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineMessageRecord {
    pub scope: OfflinePrincipalScope,
    pub conversation_id: String,
    pub message_seq: i64,
    pub message_id: String,
    pub payload_json: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineConversationRecord {
    pub scope: OfflinePrincipalScope,
    pub conversation_id: String,
    pub payload_json: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePendingSendRecord {
    pub scope: OfflinePrincipalScope,
    pub client_msg_id: String,
    pub conversation_id: String,
    pub payload_json: String,
    pub created_at: String,
    pub attempt_count: i64,
}

fn offline_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("resolve app data dir failed: {error}"))?;
    Ok(app_data_dir.join(OFFLINE_DB_FILE))
}

fn unix_epoch_millis() -> Result<i64, String> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("system clock is before unix epoch: {error}"))?
        .as_millis();
    i64::try_from(millis).map_err(|_| "unix epoch milliseconds exceed i64".to_owned())
}

fn validate_identifier(field: &str, value: &str) -> Result<(), String> {
    let length = value.trim().len();
    if length == 0 || length > 256 {
        return Err(format!("{field} must contain between 1 and 256 characters"));
    }
    Ok(())
}

fn validate_scope(scope: &OfflinePrincipalScope) -> Result<(), String> {
    validate_identifier("tenantId", scope.tenant_id.as_str())?;
    validate_identifier("organizationId", scope.organization_id.as_str())?;
    validate_identifier("principalId", scope.principal_id.as_str())?;
    if !matches!(
        scope.principal_kind.as_str(),
        "user" | "agent" | "system" | "service"
    ) {
        return Err("principalKind must be user, agent, system, or service".into());
    }
    Ok(())
}

fn validate_payload(field: &str, value: &str, max_bytes: usize) -> Result<(), String> {
    if value.len() > max_bytes {
        return Err(format!("{field} exceeds the {max_bytes} byte limit"));
    }
    Ok(())
}

fn validate_write_batch<T>(records: &[T], payload_bytes: usize) -> Result<(), String> {
    if records.len() > MAX_WRITE_BATCH {
        return Err(format!(
            "offline write batch exceeds the {MAX_WRITE_BATCH} record limit"
        ));
    }
    if payload_bytes > MAX_BATCH_PAYLOAD_BYTES {
        return Err(format!(
            "offline write batch exceeds the {MAX_BATCH_PAYLOAD_BYTES} byte limit"
        ));
    }
    Ok(())
}

fn normalize_limit(limit: Option<usize>) -> i64 {
    limit.unwrap_or(DEFAULT_PAGE_LIMIT).clamp(1, MAX_PAGE_LIMIT) as i64
}

fn initialize_offline_schema(connection: &Connection) -> Result<(), String> {
    let version = connection
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(|error| format!("read offline sqlite schema version failed: {error}"))?;
    if version > OFFLINE_SCHEMA_VERSION {
        return Err(format!(
            "offline sqlite schema version {version} is newer than supported version {OFFLINE_SCHEMA_VERSION}"
        ));
    }

    if version < 2 {
        // The application is pre-launch. Legacy rows lack organization and principal
        // ownership, so assigning them to the current account would cross security
        // boundaries. Rebuild fail-closed instead of guessing ownership.
        connection
            .execute_batch(
                r#"
                DROP TABLE IF EXISTS offline_pending_sends;
                DROP TABLE IF EXISTS offline_sync_cursors;
                DROP TABLE IF EXISTS offline_messages;
                DROP TABLE IF EXISTS offline_conversations;
                PRAGMA auto_vacuum = INCREMENTAL;
                VACUUM;
                "#,
            )
            .map_err(|error| format!("rebuild legacy offline sqlite schema failed: {error}"))?;
    }

    if version == 2 {
        with_immediate_transaction(connection, |connection| {
            connection
                .execute_batch(
                    r#"
                    ALTER TABLE offline_pending_sends
                        ADD COLUMN queue_status TEXT NOT NULL DEFAULT 'pending'
                            CHECK (queue_status IN ('pending', 'quarantined'));
                    ALTER TABLE offline_pending_sends
                        ADD COLUMN quarantine_reason TEXT;
                    ALTER TABLE offline_pending_sends
                        ADD COLUMN quarantined_at_ms INTEGER;
                    PRAGMA user_version = 3;
                    "#,
                )
                .map_err(|error| {
                    format!("migrate offline sqlite schema v2 to v3 failed: {error}")
                })?;
            Ok(())
        })?;
    }

    connection
        .execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS offline_conversations (
                tenant_id TEXT NOT NULL,
                organization_id TEXT NOT NULL,
                principal_kind TEXT NOT NULL,
                principal_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                cached_at_ms INTEGER NOT NULL,
                PRIMARY KEY (
                    tenant_id, organization_id, principal_kind, principal_id, conversation_id
                )
            );
            CREATE INDEX IF NOT EXISTS idx_offline_conversations_scope_updated
                ON offline_conversations (
                    tenant_id, organization_id, principal_kind, principal_id,
                    cached_at_ms DESC, conversation_id
                );

            CREATE TABLE IF NOT EXISTS offline_messages (
                tenant_id TEXT NOT NULL,
                organization_id TEXT NOT NULL,
                principal_kind TEXT NOT NULL,
                principal_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                message_seq INTEGER NOT NULL CHECK (message_seq > 0),
                message_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                cached_at_ms INTEGER NOT NULL,
                PRIMARY KEY (
                    tenant_id, organization_id, principal_kind, principal_id,
                    conversation_id, message_seq
                )
            );
            CREATE INDEX IF NOT EXISTS idx_offline_messages_scope_conversation_seq
                ON offline_messages (
                    tenant_id, organization_id, principal_kind, principal_id,
                    conversation_id, message_seq DESC
                );
            CREATE INDEX IF NOT EXISTS idx_offline_messages_scope_cached
                ON offline_messages (
                    tenant_id, organization_id, principal_kind, principal_id,
                    cached_at_ms, message_seq
                );

            CREATE TABLE IF NOT EXISTS offline_sync_cursors (
                tenant_id TEXT NOT NULL,
                organization_id TEXT NOT NULL,
                principal_kind TEXT NOT NULL,
                principal_id TEXT NOT NULL,
                cursor_scope TEXT NOT NULL,
                cursor_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                cached_at_ms INTEGER NOT NULL,
                PRIMARY KEY (
                    tenant_id, organization_id, principal_kind, principal_id, cursor_scope
                )
            );
            CREATE INDEX IF NOT EXISTS idx_offline_sync_cursors_scope_cached
                ON offline_sync_cursors (
                    tenant_id, organization_id, principal_kind, principal_id, cached_at_ms
                );

            CREATE TABLE IF NOT EXISTS offline_pending_sends (
                tenant_id TEXT NOT NULL,
                organization_id TEXT NOT NULL,
                principal_kind TEXT NOT NULL,
                principal_id TEXT NOT NULL,
                client_msg_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                created_at_ms INTEGER NOT NULL,
                attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
                flush_claim_id TEXT,
                flush_claimed_at_ms INTEGER,
                flush_claim_expires_at_ms INTEGER,
                queue_status TEXT NOT NULL DEFAULT 'pending'
                    CHECK (queue_status IN ('pending', 'quarantined')),
                quarantine_reason TEXT,
                quarantined_at_ms INTEGER,
                PRIMARY KEY (
                    tenant_id, organization_id, principal_kind, principal_id, client_msg_id
                ),
                CHECK (
                    (flush_claim_id IS NULL AND flush_claimed_at_ms IS NULL AND flush_claim_expires_at_ms IS NULL)
                    OR
                    (flush_claim_id IS NOT NULL AND flush_claimed_at_ms IS NOT NULL AND flush_claim_expires_at_ms IS NOT NULL)
                ),
                CHECK (
                    (queue_status = 'pending' AND quarantine_reason IS NULL AND quarantined_at_ms IS NULL)
                    OR
                    (queue_status = 'quarantined' AND quarantine_reason IS NOT NULL AND quarantined_at_ms IS NOT NULL
                        AND flush_claim_id IS NULL AND flush_claimed_at_ms IS NULL AND flush_claim_expires_at_ms IS NULL)
                )
            );
            CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_scope_created
                ON offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    created_at_ms, client_msg_id
                );
            CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_scope_claim
                ON offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    flush_claim_expires_at_ms, flush_claim_id
                );
            CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_scope_status_created
                ON offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    queue_status, created_at_ms, client_msg_id
                );

            PRAGMA user_version = 3;
            "#,
        )
        .map_err(|error| format!("initialize offline sqlite schema failed: {error}"))?;
    Ok(())
}

fn open_connection(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create offline db parent dir failed: {error}"))?;
    }
    let connection = Connection::open(path)
        .map_err(|error| format!("open offline sqlite db failed: {error}"))?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|error| format!("configure offline sqlite busy timeout failed: {error}"))?;
    connection
        .execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
            PRAGMA wal_autocheckpoint = 1000;
            "#,
        )
        .map_err(|error| format!("configure offline sqlite connection failed: {error}"))?;
    initialize_offline_schema(&connection)?;
    Ok(connection)
}

fn with_immediate_transaction<R>(
    connection: &Connection,
    operation: impl FnOnce(&Connection) -> Result<R, String>,
) -> Result<R, String> {
    connection
        .execute("BEGIN IMMEDIATE", [])
        .map_err(|error| format!("begin offline sqlite transaction failed: {error}"))?;
    match operation(connection) {
        Ok(value) => match connection.execute("COMMIT", []) {
            Ok(_) => Ok(value),
            Err(error) => {
                let _ = connection.execute("ROLLBACK", []);
                Err(format!("commit offline sqlite transaction failed: {error}"))
            }
        },
        Err(error) => {
            let _ = connection.execute("ROLLBACK", []);
            Err(error)
        }
    }
}

fn with_connection<R>(
    app: &AppHandle,
    operation: impl FnOnce(&Connection) -> Result<R, String>,
) -> Result<R, String> {
    let path = offline_db_path(app)?;
    let mut guard = OFFLINE_DB
        .lock()
        .map_err(|_| "offline db mutex poisoned".to_owned())?;
    if guard.is_none() {
        *guard = Some(open_connection(path.as_path())?);
    }
    let connection = guard
        .as_ref()
        .ok_or_else(|| "offline db connection unavailable".to_owned())?;
    operation(connection)
}

async fn with_connection_blocking<R, F>(app: AppHandle, operation: F) -> Result<R, String>
where
    R: Send + 'static,
    F: FnOnce(&Connection) -> Result<R, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(move || with_connection(&app, operation))
        .await
        .map_err(|error| format!("offline sqlite blocking task failed: {error}"))?
}

fn enforce_cache_policy(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    now_ms: i64,
    policy: CachePolicy,
) -> Result<(), String> {
    let cutoff_ms = now_ms.saturating_sub(policy.retention_ms);
    for table in [
        "offline_messages",
        "offline_conversations",
        "offline_sync_cursors",
    ] {
        connection
            .execute(
                format!(
                    "DELETE FROM {table} WHERE tenant_id = ?1 AND organization_id = ?2 AND principal_kind = ?3 AND principal_id = ?4 AND cached_at_ms < ?5"
                )
                .as_str(),
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    cutoff_ms
                ],
            )
            .map_err(|error| format!("purge expired {table} rows failed: {error}"))?;
    }

    trim_cache_table(
        connection,
        scope,
        "offline_messages",
        "payload_json",
        "cached_at_ms DESC, message_seq DESC",
        policy.message_row_limit,
        policy.message_byte_budget,
    )?;
    trim_cache_table(
        connection,
        scope,
        "offline_conversations",
        "payload_json",
        "cached_at_ms DESC, conversation_id DESC",
        policy.conversation_row_limit,
        policy.conversation_byte_budget,
    )?;
    trim_cache_table(
        connection,
        scope,
        "offline_sync_cursors",
        "cursor_json",
        "cached_at_ms DESC, cursor_scope DESC",
        policy.cursor_row_limit,
        policy.cursor_byte_budget,
    )?;
    Ok(())
}

fn trim_cache_table(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    table: &str,
    payload_column: &str,
    newest_order: &str,
    row_limit: i64,
    byte_budget: i64,
) -> Result<(), String> {
    let sql = format!(
        r#"
        WITH ranked AS (
            SELECT rowid,
                   ROW_NUMBER() OVER (ORDER BY {newest_order}) AS row_number,
                   SUM(LENGTH(CAST({payload_column} AS BLOB)))
                       OVER (ORDER BY {newest_order}) AS cumulative_bytes
            FROM {table}
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
        )
        DELETE FROM {table}
        WHERE rowid IN (
            SELECT rowid FROM ranked
            WHERE row_number > ?5 OR cumulative_bytes > ?6
        )
        "#
    );
    connection
        .execute(
            sql.as_str(),
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id,
                row_limit,
                byte_budget
            ],
        )
        .map_err(|error| format!("enforce {table} cache budget failed: {error}"))?;
    Ok(())
}

fn list_messages_for_scope(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    conversation_id: &str,
    before_seq: Option<i64>,
    limit: i64,
) -> Result<Vec<OfflineMessageRecord>, String> {
    validate_scope(scope)?;
    validate_identifier("conversationId", conversation_id)?;
    if before_seq.is_some_and(|value| value <= 0) {
        return Err("beforeSeq must be greater than zero when supplied".into());
    }
    let mut statement = connection
        .prepare(
            r#"
            SELECT conversation_id, message_seq, message_id, payload_json, updated_at
            FROM offline_messages
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
              AND conversation_id = ?5
              AND (?6 IS NULL OR message_seq < ?6)
            ORDER BY message_seq DESC
            LIMIT ?7
            "#,
        )
        .map_err(|error| format!("prepare offline message list failed: {error}"))?;
    let rows = statement
        .query_map(
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id,
                conversation_id,
                before_seq,
                limit
            ],
            |row| {
                Ok(OfflineMessageRecord {
                    scope: scope.clone(),
                    conversation_id: row.get(0)?,
                    message_seq: row.get(1)?,
                    message_id: row.get(2)?,
                    payload_json: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )
        .map_err(|error| format!("query offline messages failed: {error}"))?;
    let mut items = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("collect offline messages failed: {error}"))?;
    items.reverse();
    Ok(items)
}

fn map_pending_send_row(
    row: &rusqlite::Row<'_>,
    scope: &OfflinePrincipalScope,
) -> rusqlite::Result<OfflinePendingSendRecord> {
    Ok(OfflinePendingSendRecord {
        scope: scope.clone(),
        client_msg_id: row.get(0)?,
        conversation_id: row.get(1)?,
        payload_json: row.get(2)?,
        created_at: row.get(3)?,
        attempt_count: row.get(4)?,
    })
}

fn claim_pending_sends(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    claim_id: &str,
    now_ms: i64,
    lease_ms: i64,
    limit: i64,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    validate_scope(scope)?;
    validate_identifier("claimId", claim_id)?;
    if lease_ms <= 0 {
        return Err("offline pending send claim lease must be positive".into());
    }
    let expires_at_ms = now_ms
        .checked_add(lease_ms)
        .ok_or_else(|| "offline pending send claim lease overflow".to_owned())?;
    with_immediate_transaction(connection, |connection| {
        connection
            .execute(
                r#"
                UPDATE offline_pending_sends
                SET flush_claim_id = ?5,
                    flush_claimed_at_ms = ?6,
                    flush_claim_expires_at_ms = ?7,
                    attempt_count = attempt_count + 1
                WHERE rowid IN (
                    SELECT rowid
                    FROM offline_pending_sends
                    WHERE tenant_id = ?1
                      AND organization_id = ?2
                      AND principal_kind = ?3
                      AND principal_id = ?4
                      AND queue_status = 'pending'
                      AND (flush_claim_id IS NULL OR flush_claim_expires_at_ms <= ?6)
                    ORDER BY created_at_ms ASC, client_msg_id ASC
                    LIMIT ?8
                )
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    claim_id,
                    now_ms,
                    expires_at_ms,
                    limit
                ],
            )
            .map_err(|error| format!("claim offline pending sends failed: {error}"))?;

        let mut statement = connection
            .prepare(
                r#"
                SELECT client_msg_id, conversation_id, payload_json, created_at, attempt_count
                FROM offline_pending_sends
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND flush_claim_id = ?5
                ORDER BY created_at_ms ASC, client_msg_id ASC
                "#,
            )
            .map_err(|error| format!("prepare claimed offline pending sends failed: {error}"))?;
        let rows = statement
            .query_map(
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    claim_id
                ],
                |row| map_pending_send_row(row, scope),
            )
            .map_err(|error| format!("query claimed offline pending sends failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect claimed offline pending sends failed: {error}"))
    })
}

fn acknowledge_pending_send(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    client_msg_id: &str,
    claim_id: &str,
) -> Result<bool, String> {
    validate_scope(scope)?;
    validate_identifier("clientMsgId", client_msg_id)?;
    validate_identifier("claimId", claim_id)?;
    let deleted = connection
        .execute(
            r#"
            DELETE FROM offline_pending_sends
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
              AND client_msg_id = ?5
              AND flush_claim_id = ?6
            "#,
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id,
                client_msg_id,
                claim_id
            ],
        )
        .map_err(|error| format!("acknowledge offline pending send failed: {error}"))?;
    Ok(deleted > 0)
}

fn quarantine_pending_send(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    client_msg_id: &str,
    claim_id: &str,
    reason: &str,
    now_ms: i64,
) -> Result<bool, String> {
    validate_scope(scope)?;
    validate_identifier("clientMsgId", client_msg_id)?;
    validate_identifier("claimId", claim_id)?;
    validate_payload("quarantineReason", reason, 1_024)?;
    if reason.trim().is_empty() {
        return Err("quarantineReason must not be empty".into());
    }
    with_immediate_transaction(connection, |connection| {
        let changed = connection
            .execute(
                r#"
                UPDATE offline_pending_sends
                SET queue_status = 'quarantined',
                    quarantine_reason = ?7,
                    quarantined_at_ms = ?8,
                    flush_claim_id = NULL,
                    flush_claimed_at_ms = NULL,
                    flush_claim_expires_at_ms = NULL
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND client_msg_id = ?5
                  AND flush_claim_id = ?6
                  AND queue_status = 'pending'
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    client_msg_id,
                    claim_id,
                    reason,
                    now_ms
                ],
            )
            .map_err(|error| format!("quarantine offline pending send failed: {error}"))?;
        if changed > 0 {
            enforce_pending_send_quarantine_policy(
                connection,
                scope,
                now_ms,
                PENDING_SEND_QUARANTINE_POLICY,
            )?;
        }
        Ok(changed > 0)
    })
}

fn enforce_pending_send_quarantine_policy(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    now_ms: i64,
    policy: PendingSendQuarantinePolicy,
) -> Result<(), String> {
    let cutoff_ms = now_ms.saturating_sub(policy.retention_ms);
    connection
        .execute(
            r#"
            DELETE FROM offline_pending_sends
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
              AND queue_status = 'quarantined'
              AND quarantined_at_ms < ?5
            "#,
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id,
                cutoff_ms
            ],
        )
        .map_err(|error| {
            format!("purge expired offline pending send quarantine failed: {error}")
        })?;
    connection
        .execute(
            r#"
            WITH ranked AS (
                SELECT rowid,
                       ROW_NUMBER() OVER (
                           ORDER BY quarantined_at_ms DESC, client_msg_id DESC
                       ) AS row_number,
                       SUM(
                           LENGTH(CAST(payload_json AS BLOB))
                           + LENGTH(CAST(quarantine_reason AS BLOB))
                       ) OVER (
                           ORDER BY quarantined_at_ms DESC, client_msg_id DESC
                       ) AS cumulative_bytes
                FROM offline_pending_sends
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND queue_status = 'quarantined'
            )
            DELETE FROM offline_pending_sends
            WHERE rowid IN (
                SELECT rowid FROM ranked
                WHERE row_number > ?5 OR cumulative_bytes > ?6
            )
            "#,
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id,
                policy.row_limit,
                policy.byte_budget
            ],
        )
        .map_err(|error| {
            format!("enforce offline pending send quarantine budget failed: {error}")
        })?;
    Ok(())
}

fn purge_principal_cache(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
) -> Result<usize, String> {
    validate_scope(scope)?;
    with_immediate_transaction(connection, |connection| {
        let mut deleted = 0usize;
        for table in [
            "offline_sync_cursors",
            "offline_messages",
            "offline_conversations",
        ] {
            deleted = deleted.saturating_add(
                connection
                    .execute(
                        format!(
                            "DELETE FROM {table} WHERE tenant_id = ?1 AND organization_id = ?2 AND principal_kind = ?3 AND principal_id = ?4"
                        )
                        .as_str(),
                        params![
                            &scope.tenant_id,
                            &scope.organization_id,
                            &scope.principal_kind,
                            &scope.principal_id
                        ],
                    )
                    .map_err(|error| format!("purge {table} principal cache failed: {error}"))?,
            );
        }
        Ok(deleted)
    })
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_init(app: AppHandle) -> Result<(), String> {
    with_connection_blocking(app, |_| Ok(())).await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_upsert_conversations(
    app: AppHandle,
    records: Vec<OfflineConversationRecord>,
) -> Result<usize, String> {
    let payload_bytes = records.iter().fold(0usize, |total, record| {
        total.saturating_add(record.payload_json.len())
    });
    validate_write_batch(records.as_slice(), payload_bytes)?;
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        with_immediate_transaction(connection, |connection| {
            let mut scopes = Vec::new();
            for record in &records {
                validate_scope(&record.scope)?;
                validate_identifier("conversationId", record.conversation_id.as_str())?;
                validate_payload(
                    "conversation payload",
                    record.payload_json.as_str(),
                    MAX_RECORD_PAYLOAD_BYTES,
                )?;
                if !scopes.contains(&record.scope) {
                    scopes.push(record.scope.clone());
                }
                connection
                    .execute(
                        r#"
                        INSERT INTO offline_conversations (
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, payload_json, updated_at, cached_at_ms
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                        ON CONFLICT(
                            tenant_id, organization_id, principal_kind, principal_id, conversation_id
                        ) DO UPDATE SET
                            payload_json = excluded.payload_json,
                            updated_at = excluded.updated_at,
                            cached_at_ms = excluded.cached_at_ms
                        "#,
                        params![
                            &record.scope.tenant_id,
                            &record.scope.organization_id,
                            &record.scope.principal_kind,
                            &record.scope.principal_id,
                            &record.conversation_id,
                            &record.payload_json,
                            &record.updated_at,
                            now_ms
                        ],
                    )
                    .map_err(|error| format!("upsert offline conversation failed: {error}"))?;
            }
            for scope in &scopes {
                enforce_cache_policy(connection, scope, now_ms, OFFLINE_CACHE_POLICY)?;
            }
            Ok(records.len())
        })
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_list_conversations(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    limit: Option<usize>,
) -> Result<Vec<OfflineConversationRecord>, String> {
    validate_scope(&scope)?;
    let limit = normalize_limit(limit);
    with_connection_blocking(app, move |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT conversation_id, payload_json, updated_at
                FROM offline_conversations
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                ORDER BY cached_at_ms DESC, conversation_id ASC
                LIMIT ?5
                "#,
            )
            .map_err(|error| format!("prepare offline conversation list failed: {error}"))?;
        let rows = statement
            .query_map(
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    limit
                ],
                |row| {
                    Ok(OfflineConversationRecord {
                        scope: scope.clone(),
                        conversation_id: row.get(0)?,
                        payload_json: row.get(1)?,
                        updated_at: row.get(2)?,
                    })
                },
            )
            .map_err(|error| format!("query offline conversations failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline conversations failed: {error}"))
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_upsert_messages(
    app: AppHandle,
    records: Vec<OfflineMessageRecord>,
) -> Result<usize, String> {
    let payload_bytes = records.iter().fold(0usize, |total, record| {
        total.saturating_add(record.payload_json.len())
    });
    validate_write_batch(records.as_slice(), payload_bytes)?;
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        with_immediate_transaction(connection, |connection| {
            let mut scopes = Vec::new();
            for record in &records {
                validate_scope(&record.scope)?;
                validate_identifier("conversationId", record.conversation_id.as_str())?;
                validate_identifier("messageId", record.message_id.as_str())?;
                if record.message_seq <= 0 {
                    return Err("messageSeq must be greater than zero".into());
                }
                validate_payload(
                    "message payload",
                    record.payload_json.as_str(),
                    MAX_RECORD_PAYLOAD_BYTES,
                )?;
                if !scopes.contains(&record.scope) {
                    scopes.push(record.scope.clone());
                }
                connection
                    .execute(
                        r#"
                        INSERT INTO offline_messages (
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, message_seq, message_id, payload_json,
                            updated_at, cached_at_ms
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                        ON CONFLICT(
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, message_seq
                        ) DO UPDATE SET
                            message_id = excluded.message_id,
                            payload_json = excluded.payload_json,
                            updated_at = excluded.updated_at,
                            cached_at_ms = excluded.cached_at_ms
                        "#,
                        params![
                            &record.scope.tenant_id,
                            &record.scope.organization_id,
                            &record.scope.principal_kind,
                            &record.scope.principal_id,
                            &record.conversation_id,
                            record.message_seq,
                            &record.message_id,
                            &record.payload_json,
                            &record.updated_at,
                            now_ms
                        ],
                    )
                    .map_err(|error| format!("upsert offline message failed: {error}"))?;
            }
            for scope in &scopes {
                enforce_cache_policy(connection, scope, now_ms, OFFLINE_CACHE_POLICY)?;
            }
            Ok(records.len())
        })
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_list_messages(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    conversation_id: String,
    before_seq: Option<i64>,
    limit: Option<usize>,
) -> Result<Vec<OfflineMessageRecord>, String> {
    let limit = normalize_limit(limit);
    with_connection_blocking(app, move |connection| {
        list_messages_for_scope(
            connection,
            &scope,
            conversation_id.as_str(),
            before_seq,
            limit,
        )
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_get_sync_cursor(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    cursor_scope: String,
) -> Result<Option<String>, String> {
    validate_scope(&scope)?;
    validate_identifier("cursorScope", cursor_scope.as_str())?;
    with_connection_blocking(app, move |connection| {
        connection
            .query_row(
                r#"
                SELECT cursor_json
                FROM offline_sync_cursors
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND cursor_scope = ?5
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    cursor_scope
                ],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| format!("read offline sync cursor failed: {error}"))
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_set_sync_cursor(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    cursor_scope: String,
    cursor_json: String,
    updated_at: String,
) -> Result<(), String> {
    validate_scope(&scope)?;
    validate_identifier("cursorScope", cursor_scope.as_str())?;
    validate_payload("cursorJson", cursor_json.as_str(), MAX_CURSOR_BYTES)?;
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        with_immediate_transaction(connection, |connection| {
            connection
                .execute(
                    r#"
                    INSERT INTO offline_sync_cursors (
                        tenant_id, organization_id, principal_kind, principal_id,
                        cursor_scope, cursor_json, updated_at, cached_at_ms
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                    ON CONFLICT(
                        tenant_id, organization_id, principal_kind, principal_id, cursor_scope
                    ) DO UPDATE SET
                        cursor_json = excluded.cursor_json,
                        updated_at = excluded.updated_at,
                        cached_at_ms = excluded.cached_at_ms
                    "#,
                    params![
                        &scope.tenant_id,
                        &scope.organization_id,
                        &scope.principal_kind,
                        &scope.principal_id,
                        cursor_scope,
                        cursor_json,
                        updated_at,
                        now_ms
                    ],
                )
                .map_err(|error| format!("upsert offline sync cursor failed: {error}"))?;
            enforce_cache_policy(connection, &scope, now_ms, OFFLINE_CACHE_POLICY)
        })
    })
    .await
}

fn ensure_pending_send_capacity(
    connection: &Connection,
    record: &OfflinePendingSendRecord,
) -> Result<(), String> {
    let (row_count, payload_bytes, existing_rows, existing_bytes): (i64, i64, i64, i64) =
        connection
        .query_row(
            r#"
            SELECT COUNT(*),
                   COALESCE(SUM(LENGTH(CAST(payload_json AS BLOB))), 0),
                   COALESCE(SUM(CASE WHEN client_msg_id = ?5 THEN 1 ELSE 0 END), 0),
                   COALESCE(MAX(CASE WHEN client_msg_id = ?5 THEN LENGTH(CAST(payload_json AS BLOB)) ELSE 0 END), 0)
            FROM offline_pending_sends
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
              AND queue_status = 'pending'
            "#,
            params![
                &record.scope.tenant_id,
                &record.scope.organization_id,
                &record.scope.principal_kind,
                &record.scope.principal_id,
                &record.client_msg_id
            ],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|error| format!("read offline pending send capacity failed: {error}"))?;
    let is_new = existing_rows == 0;
    let next_rows = row_count + i64::from(is_new);
    let next_bytes = payload_bytes
        .saturating_sub(existing_bytes)
        .saturating_add(record.payload_json.len() as i64);
    if next_rows > MAX_PENDING_SEND_ROWS_PER_SCOPE || next_bytes > MAX_PENDING_SEND_BYTES_PER_SCOPE
    {
        return Err(
            "offline pending send queue capacity exceeded; reconnect before sending more messages"
                .into(),
        );
    }
    Ok(())
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_enqueue_pending_send(
    app: AppHandle,
    record: OfflinePendingSendRecord,
) -> Result<(), String> {
    validate_scope(&record.scope)?;
    validate_identifier("clientMsgId", record.client_msg_id.as_str())?;
    validate_identifier("conversationId", record.conversation_id.as_str())?;
    validate_payload(
        "pending send payload",
        record.payload_json.as_str(),
        MAX_RECORD_PAYLOAD_BYTES,
    )?;
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        with_immediate_transaction(connection, |connection| {
            ensure_pending_send_capacity(connection, &record)?;
            let changed = connection
                .execute(
                    r#"
                    INSERT INTO offline_pending_sends (
                        tenant_id, organization_id, principal_kind, principal_id,
                        client_msg_id, conversation_id, payload_json, created_at,
                        created_at_ms, attempt_count
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                    ON CONFLICT(
                        tenant_id, organization_id, principal_kind, principal_id, client_msg_id
                    ) DO UPDATE SET
                        conversation_id = excluded.conversation_id,
                        payload_json = excluded.payload_json,
                        created_at = excluded.created_at,
                        created_at_ms = excluded.created_at_ms,
                        attempt_count = excluded.attempt_count,
                        flush_claim_id = NULL,
                        flush_claimed_at_ms = NULL,
                        flush_claim_expires_at_ms = NULL,
                        queue_status = 'pending',
                        quarantine_reason = NULL,
                        quarantined_at_ms = NULL
                    WHERE offline_pending_sends.flush_claim_id IS NULL
                       OR offline_pending_sends.flush_claim_expires_at_ms <= ?9
                    "#,
                    params![
                        &record.scope.tenant_id,
                        &record.scope.organization_id,
                        &record.scope.principal_kind,
                        &record.scope.principal_id,
                        &record.client_msg_id,
                        &record.conversation_id,
                        &record.payload_json,
                        &record.created_at,
                        now_ms,
                        record.attempt_count.max(0)
                    ],
                )
                .map_err(|error| format!("enqueue offline pending send failed: {error}"))?;
            if changed == 0 {
                return Err(
                    "offline pending send is currently owned by an active flush claim".into(),
                );
            }
            Ok(())
        })
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_list_pending_sends(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    limit: Option<usize>,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    validate_scope(&scope)?;
    let limit = normalize_limit(limit);
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT client_msg_id, conversation_id, payload_json, created_at, attempt_count
                FROM offline_pending_sends
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND queue_status = 'pending'
                  AND (flush_claim_id IS NULL OR flush_claim_expires_at_ms <= ?5)
                ORDER BY created_at_ms ASC, client_msg_id ASC
                LIMIT ?6
                "#,
            )
            .map_err(|error| format!("prepare offline pending send list failed: {error}"))?;
        let rows = statement
            .query_map(
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    now_ms,
                    limit
                ],
                |row| map_pending_send_row(row, &scope),
            )
            .map_err(|error| format!("query offline pending sends failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline pending sends failed: {error}"))
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_claim_pending_sends(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    claim_id: String,
    limit: Option<usize>,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    let now_ms = unix_epoch_millis()?;
    let limit = normalize_limit(limit);
    with_connection_blocking(app, move |connection| {
        claim_pending_sends(
            connection,
            &scope,
            claim_id.as_str(),
            now_ms,
            PENDING_SEND_CLAIM_LEASE_MS,
            limit,
        )
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_release_pending_send_claim(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    client_msg_id: String,
    claim_id: String,
) -> Result<bool, String> {
    validate_scope(&scope)?;
    validate_identifier("clientMsgId", client_msg_id.as_str())?;
    validate_identifier("claimId", claim_id.as_str())?;
    with_connection_blocking(app, move |connection| {
        let released = connection
            .execute(
                r#"
                UPDATE offline_pending_sends
                SET flush_claim_id = NULL,
                    flush_claimed_at_ms = NULL,
                    flush_claim_expires_at_ms = NULL
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND client_msg_id = ?5
                  AND flush_claim_id = ?6
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    client_msg_id,
                    claim_id
                ],
            )
            .map_err(|error| format!("release offline pending send claim failed: {error}"))?;
        Ok(released > 0)
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_delete_pending_send(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    client_msg_id: String,
    claim_id: String,
) -> Result<bool, String> {
    with_connection_blocking(app, move |connection| {
        acknowledge_pending_send(
            connection,
            &scope,
            client_msg_id.as_str(),
            claim_id.as_str(),
        )
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_quarantine_pending_send(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    client_msg_id: String,
    claim_id: String,
    reason: String,
) -> Result<bool, String> {
    let now_ms = unix_epoch_millis()?;
    with_connection_blocking(app, move |connection| {
        quarantine_pending_send(
            connection,
            &scope,
            client_msg_id.as_str(),
            claim_id.as_str(),
            reason.as_str(),
            now_ms,
        )
    })
    .await
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_purge_principal_cache(
    app: AppHandle,
    scope: OfflinePrincipalScope,
) -> Result<usize, String> {
    with_connection_blocking(app, move |connection| {
        purge_principal_cache(connection, &scope)
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("sdkwork-im-offline-test-{nanos}.sqlite"))
    }

    fn cleanup_db(path: &Path) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(format!("{}-wal", path.display()));
        let _ = fs::remove_file(format!("{}-shm", path.display()));
    }

    fn principal_scope(principal_id: &str) -> OfflinePrincipalScope {
        OfflinePrincipalScope {
            tenant_id: "100001".into(),
            organization_id: "org-a".into(),
            principal_kind: "user".into(),
            principal_id: principal_id.into(),
        }
    }

    fn insert_pending_send_for_test(
        connection: &Connection,
        scope: &OfflinePrincipalScope,
        client_msg_id: &str,
        claim: Option<(&str, i64, i64)>,
    ) {
        let (claim_id, claimed_at_ms, claim_expires_at_ms) = claim
            .map(|(id, claimed_at, expires_at)| (Some(id), Some(claimed_at), Some(expires_at)))
            .unwrap_or((None, None, None));
        connection
            .execute(
                r#"
                INSERT INTO offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_json, created_at, created_at_ms,
                    attempt_count, flush_claim_id, flush_claimed_at_ms, flush_claim_expires_at_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, 'conversation', '{}', '2026-07-10T00:00:00Z', 1, 0, ?6, ?7, ?8)
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    client_msg_id,
                    claim_id,
                    claimed_at_ms,
                    claim_expires_at_ms
                ],
            )
            .expect("insert pending send fixture");
    }

    #[test]
    fn offline_store_configures_versioned_safe_sqlite_profile() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("read user version");
        let foreign_keys: i64 = connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("read foreign key setting");
        assert_eq!(version, OFFLINE_SCHEMA_VERSION);
        assert_eq!(foreign_keys, 1);
        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn schema_v2_to_v3_migration_preserves_pending_sends() {
        let path = temp_db_path();
        let legacy = Connection::open(path.as_path()).expect("open v2 offline db");
        legacy
            .execute_batch(
                r#"
                CREATE TABLE offline_pending_sends (
                    tenant_id TEXT NOT NULL,
                    organization_id TEXT NOT NULL,
                    principal_kind TEXT NOT NULL,
                    principal_id TEXT NOT NULL,
                    client_msg_id TEXT NOT NULL,
                    conversation_id TEXT NOT NULL,
                    payload_json TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    created_at_ms INTEGER NOT NULL,
                    attempt_count INTEGER NOT NULL DEFAULT 0,
                    flush_claim_id TEXT,
                    flush_claimed_at_ms INTEGER,
                    flush_claim_expires_at_ms INTEGER,
                    PRIMARY KEY (
                        tenant_id, organization_id, principal_kind, principal_id, client_msg_id
                    )
                );
                INSERT INTO offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_json, created_at, created_at_ms,
                    attempt_count
                ) VALUES (
                    '100001', 'org-a', 'user', 'user-1',
                    'pending-v2', 'conversation', '{"content":"preserve"}',
                    '2026-07-10T00:00:00Z', 1, 0
                );
                PRAGMA user_version = 2;
                "#,
            )
            .expect("create v2 offline fixture");
        drop(legacy);

        let connection = open_connection(path.as_path()).expect("migrate v2 offline db");
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("read migrated version");
        let migrated: (String, String, String, String) = connection
            .query_row(
                "SELECT principal_id, client_msg_id, payload_json, queue_status FROM offline_pending_sends",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .expect("read migrated pending send");
        assert_eq!(version, OFFLINE_SCHEMA_VERSION);
        assert_eq!(migrated.0, "user-1");
        assert_eq!(migrated.1, "pending-v2");
        assert_eq!(migrated.2, r#"{"content":"preserve"}"#);
        assert_eq!(migrated.3, "pending");

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn offline_store_isolates_conversations_messages_cursors_and_pending_sends_by_principal() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let first = principal_scope("user-1");
        let second = principal_scope("user-2");

        for scope in [&first, &second] {
            connection.execute(
                "INSERT INTO offline_conversations (tenant_id, organization_id, principal_kind, principal_id, conversation_id, payload_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'shared', '{}', '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id],
            ).expect("insert conversation");
            connection.execute(
                "INSERT INTO offline_messages (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'shared', 1, 'message', '{}', '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id],
            ).expect("insert message");
            connection.execute(
                "INSERT INTO offline_sync_cursors (tenant_id, organization_id, principal_kind, principal_id, cursor_scope, cursor_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'inbox', '{}', '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id],
            ).expect("insert cursor");
            insert_pending_send_for_test(&connection, scope, "shared-client-message", None);
        }

        for table in [
            "offline_conversations",
            "offline_messages",
            "offline_sync_cursors",
            "offline_pending_sends",
        ] {
            let count: i64 = connection.query_row(
                format!("SELECT COUNT(*) FROM {table} WHERE tenant_id = ?1 AND organization_id = ?2 AND principal_kind = ?3 AND principal_id = ?4").as_str(),
                params![&first.tenant_id, &first.organization_id, &first.principal_kind, &first.principal_id],
                |row| row.get(0),
            ).expect("count principal rows");
            assert_eq!(count, 1, "{table} must isolate the first principal");
        }
        let first_messages = list_messages_for_scope(&connection, &first, "shared", None, 20)
            .expect("list first principal messages");
        assert_eq!(first_messages.len(), 1);
        assert_eq!(first_messages[0].scope, first);

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn backward_message_pages_return_chronological_windows_from_latest() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        for seq in 1..=4 {
            connection.execute(
                "INSERT INTO offline_messages (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', ?5, ?6, '{}', '2026-07-10T00:00:00Z', ?5)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, seq, format!("message-{seq}")],
            ).expect("insert message");
        }
        let latest = list_messages_for_scope(&connection, &scope, "conversation", None, 2)
            .expect("latest page");
        assert_eq!(
            latest
                .iter()
                .map(|item| item.message_seq)
                .collect::<Vec<_>>(),
            vec![3, 4]
        );
        let older = list_messages_for_scope(&connection, &scope, "conversation", Some(3), 2)
            .expect("older page");
        assert_eq!(
            older
                .iter()
                .map(|item| item.message_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn expired_pending_send_claim_is_recovered_without_stealing_a_live_claim() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        insert_pending_send_for_test(&connection, &scope, "expired", Some(("old", 100, 200)));
        insert_pending_send_for_test(&connection, &scope, "live", Some(("live", 900, 1_100)));
        let claimed = claim_pending_sends(&connection, &scope, "replacement", 1_000, 60_000, 10)
            .expect("claim pending sends");
        assert_eq!(
            claimed
                .iter()
                .map(|record| record.client_msg_id.as_str())
                .collect::<Vec<_>>(),
            vec!["expired"]
        );
        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn stale_pending_send_claim_cannot_acknowledge_a_reclaimed_row() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        insert_pending_send_for_test(&connection, &scope, "message", None);
        claim_pending_sends(&connection, &scope, "old-claim", 100, 100, 1).expect("old claim");
        claim_pending_sends(&connection, &scope, "new-claim", 201, 100, 1).expect("new claim");

        assert!(
            !acknowledge_pending_send(&connection, &scope, "message", "old-claim")
                .expect("stale acknowledgement")
        );
        assert!(
            acknowledge_pending_send(&connection, &scope, "message", "new-claim")
                .expect("current acknowledgement")
        );

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn only_current_claim_can_quarantine_a_corrupt_pending_send() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        insert_pending_send_for_test(&connection, &scope, "message", None);
        claim_pending_sends(&connection, &scope, "old-claim", 100, 100, 1).expect("old claim");
        claim_pending_sends(&connection, &scope, "new-claim", 201, 100, 1).expect("new claim");

        assert!(!quarantine_pending_send(
            &connection,
            &scope,
            "message",
            "old-claim",
            "invalid pending send payload",
            202,
        )
        .expect("stale quarantine"));
        assert!(quarantine_pending_send(
            &connection,
            &scope,
            "message",
            "new-claim",
            "invalid pending send payload",
            202,
        )
        .expect("current quarantine"));
        assert!(
            claim_pending_sends(&connection, &scope, "third-claim", 203, 100, 1)
                .expect("claim after quarantine")
                .is_empty()
        );
        let status: (String, String) = connection
            .query_row(
                "SELECT queue_status, quarantine_reason FROM offline_pending_sends WHERE client_msg_id = 'message'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("read quarantine state");
        assert_eq!(status.0, "quarantined");
        assert_eq!(status.1, "invalid pending send payload");

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn quarantine_policy_is_bounded_without_deleting_pending_sends() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        insert_pending_send_for_test(&connection, &scope, "pending", None);
        for (client_msg_id, quarantined_at_ms) in [
            ("quarantined-1", 1),
            ("quarantined-2", 2),
            ("quarantined-3", 3),
        ] {
            insert_pending_send_for_test(&connection, &scope, client_msg_id, None);
            connection
                .execute(
                    "UPDATE offline_pending_sends SET queue_status = 'quarantined', quarantine_reason = 'invalid', quarantined_at_ms = ?1 WHERE client_msg_id = ?2",
                    params![quarantined_at_ms, client_msg_id],
                )
                .expect("quarantine fixture");
        }

        enforce_pending_send_quarantine_policy(
            &connection,
            &scope,
            50,
            PendingSendQuarantinePolicy {
                retention_ms: 1_000,
                row_limit: 2,
                byte_budget: 1_000,
            },
        )
        .expect("enforce quarantine policy");

        let remaining: Vec<String> = {
            let mut statement = connection
                .prepare(
                    "SELECT client_msg_id FROM offline_pending_sends WHERE queue_status = 'quarantined' ORDER BY quarantined_at_ms",
                )
                .expect("prepare quarantine query");
            statement
                .query_map([], |row| row.get(0))
                .expect("query quarantine rows")
                .collect::<Result<Vec<_>, _>>()
                .expect("collect quarantine rows")
        };
        assert_eq!(remaining, vec!["quarantined-2", "quarantined-3"]);
        let pending_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM offline_pending_sends WHERE client_msg_id = 'pending' AND queue_status = 'pending'",
                [],
                |row| row.get(0),
            )
            .expect("count pending row");
        assert_eq!(pending_count, 1);

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn principal_cache_purge_preserves_unsent_and_other_principal_rows() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let first = principal_scope("user-1");
        let second = principal_scope("user-2");
        for scope in [&first, &second] {
            connection.execute(
                "INSERT INTO offline_conversations (tenant_id, organization_id, principal_kind, principal_id, conversation_id, payload_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', '{}', '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id],
            ).expect("insert conversation");
        }
        insert_pending_send_for_test(&connection, &first, "unsent", None);

        assert_eq!(
            purge_principal_cache(&connection, &first).expect("purge cache"),
            1
        );
        let first_cache: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM offline_conversations WHERE principal_id = 'user-1'",
                [],
                |row| row.get(0),
            )
            .expect("first cache count");
        let second_cache: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM offline_conversations WHERE principal_id = 'user-2'",
                [],
                |row| row.get(0),
            )
            .expect("second cache count");
        let pending: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM offline_pending_sends WHERE principal_id = 'user-1'",
                [],
                |row| row.get(0),
            )
            .expect("pending count");
        assert_eq!(first_cache, 0);
        assert_eq!(second_cache, 1);
        assert_eq!(pending, 1);

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn pending_send_capacity_treats_zero_byte_payload_update_as_existing_row() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        connection
            .execute(
                r#"
                WITH RECURSIVE sequence(value) AS (
                    SELECT 1
                    UNION ALL
                    SELECT value + 1 FROM sequence WHERE value < ?5
                )
                INSERT INTO offline_pending_sends (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_json, created_at, created_at_ms,
                    attempt_count
                )
                SELECT ?1, ?2, ?3, ?4, printf('message-%05d', value),
                       'conversation', '', '2026-07-10T00:00:00Z', value, 0
                FROM sequence
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    MAX_PENDING_SEND_ROWS_PER_SCOPE
                ],
            )
            .expect("fill pending send capacity");
        let existing = OfflinePendingSendRecord {
            scope,
            client_msg_id: "message-00001".into(),
            conversation_id: "conversation".into(),
            payload_json: String::new(),
            created_at: "2026-07-10T00:00:00Z".into(),
            attempt_count: 0,
        };

        ensure_pending_send_capacity(&connection, &existing)
            .expect("updating a zero-byte payload must not consume another row");

        drop(connection);
        cleanup_db(path.as_path());
    }

    #[test]
    fn cache_policy_evicts_old_cache_rows_but_preserves_unsent_rows() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        let scope = principal_scope("user-1");
        for seq in 1..=3 {
            connection.execute(
                "INSERT INTO offline_messages (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_json, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', ?5, ?6, '1234567890', '2026-07-10T00:00:00Z', ?5)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, seq, format!("message-{seq}")],
            ).expect("insert message");
        }
        insert_pending_send_for_test(&connection, &scope, "unsent", None);
        let policy = CachePolicy {
            retention_ms: 100,
            conversation_row_limit: 10,
            conversation_byte_budget: 1_000,
            message_row_limit: 2,
            message_byte_budget: 1_000,
            cursor_row_limit: 10,
            cursor_byte_budget: 1_000,
        };
        enforce_cache_policy(&connection, &scope, 50, policy).expect("enforce cache policy");
        let remaining: Vec<i64> = {
            let mut statement = connection
                .prepare("SELECT message_seq FROM offline_messages ORDER BY message_seq")
                .expect("prepare");
            statement
                .query_map([], |row| row.get(0))
                .expect("query")
                .collect::<Result<Vec<_>, _>>()
                .expect("collect")
        };
        assert_eq!(remaining, vec![2, 3]);
        let pending_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM offline_pending_sends", [], |row| {
                row.get(0)
            })
            .expect("pending count");
        assert_eq!(pending_count, 1);
        drop(connection);
        cleanup_db(path.as_path());
    }
}
