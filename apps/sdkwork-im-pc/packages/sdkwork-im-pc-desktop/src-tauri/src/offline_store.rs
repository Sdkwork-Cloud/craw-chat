//! Scope-bound, encrypted desktop cache and resumable pending-send queue.

mod crypto;
mod database;

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crypto::{payload_hash, PayloadCipher};
#[cfg(test)]
use database::OFFLINE_SCHEMA_VERSION;
use database::{
    open_scoped_database, remove_legacy_unscoped_database, scope_fingerprint, OfflineDatabase,
};

const DEFAULT_PAGE_LIMIT: usize = 20;
const MAX_PAGE_LIMIT: usize = 200;
const MAX_WRITE_BATCH: usize = 200;
const MAX_BATCH_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECORD_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;
const MAX_CURSOR_BYTES: usize = 64 * 1024;
const MAX_TIMESTAMP_BYTES: usize = 64;
const PENDING_SEND_CLAIM_LEASE_MS: i64 = 60_000;
const MAX_PENDING_SEND_ATTEMPTS: i64 = 20;
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

#[derive(Clone, Copy)]
struct CacheTablePolicy {
    table: &'static str,
    payload_column: &'static str,
    identity_columns: &'static str,
    newest_order: &'static str,
    row_limit: i64,
    byte_budget: i64,
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

static OFFLINE_DB: Mutex<Option<OfflineDatabase>> = Mutex::new(None);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePrincipalScope {
    pub environment: String,
    pub deployment_profile: String,
    pub deployment_mode: String,
    pub api_origin: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub account_id: String,
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
    scope_fingerprint(scope).map(|_| ())
}

fn validate_payload(field: &str, value: &str, max_bytes: usize) -> Result<(), String> {
    if value.len() > max_bytes {
        return Err(format!("{field} exceeds the {max_bytes} byte limit"));
    }
    Ok(())
}

fn validate_timestamp(field: &str, value: &str) -> Result<(), String> {
    validate_payload(field, value, MAX_TIMESTAMP_BYTES)?;
    chrono::DateTime::parse_from_rfc3339(value)
        .map_err(|_| format!("{field} must be an RFC 3339 timestamp"))?;
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
    scope: &OfflinePrincipalScope,
    operation: impl FnOnce(&OfflineDatabase) -> Result<R, String>,
) -> Result<R, String> {
    validate_scope(scope)?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("resolve app data dir failed: {error}"))?;
    let fingerprint = scope_fingerprint(scope)?;
    let mut guard = OFFLINE_DB
        .lock()
        .map_err(|_| "client-local database mutex poisoned".to_owned())?;
    let requires_open = guard
        .as_ref()
        .is_none_or(|database| database.scope_fingerprint != fingerprint);
    if requires_open {
        *guard = None;
        remove_legacy_unscoped_database(app_data_dir.as_path())?;
        *guard = Some(open_scoped_database(app_data_dir.as_path(), scope)?);
    }
    let database = guard
        .as_ref()
        .ok_or_else(|| "client-local database connection unavailable".to_owned())?;
    operation(database)
}

async fn with_scoped_connection_blocking<R, F>(
    app: AppHandle,
    scope: OfflinePrincipalScope,
    operation: F,
) -> Result<R, String>
where
    R: Send + 'static,
    F: FnOnce(&OfflineDatabase) -> Result<R, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(move || with_connection(&app, &scope, operation))
        .await
        .map_err(|error| format!("client-local sqlite blocking task failed: {error}"))?
}

fn enforce_cache_policy(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    now_ms: i64,
    policy: CachePolicy,
) -> Result<(), String> {
    let cutoff_ms = now_ms.saturating_sub(policy.retention_ms);
    for table in [
        "im_local_message_cache",
        "im_local_conversation_cache",
        "im_local_cache_cursor",
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
        CacheTablePolicy {
            table: "im_local_message_cache",
            payload_column: "payload_ciphertext",
            identity_columns: "tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq",
            newest_order: "cached_at_ms DESC, message_seq DESC",
            row_limit: policy.message_row_limit,
            byte_budget: policy.message_byte_budget,
        },
    )?;
    trim_cache_table(
        connection,
        scope,
        CacheTablePolicy {
            table: "im_local_conversation_cache",
            payload_column: "payload_ciphertext",
            identity_columns:
                "tenant_id, organization_id, principal_kind, principal_id, conversation_id",
            newest_order: "cached_at_ms DESC, conversation_id DESC",
            row_limit: policy.conversation_row_limit,
            byte_budget: policy.conversation_byte_budget,
        },
    )?;
    trim_cache_table(
        connection,
        scope,
        CacheTablePolicy {
            table: "im_local_cache_cursor",
            payload_column: "cursor_ciphertext",
            identity_columns:
                "tenant_id, organization_id, principal_kind, principal_id, cursor_scope",
            newest_order: "cached_at_ms DESC, cursor_scope DESC",
            row_limit: policy.cursor_row_limit,
            byte_budget: policy.cursor_byte_budget,
        },
    )?;
    Ok(())
}

fn trim_cache_table(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    policy: CacheTablePolicy,
) -> Result<(), String> {
    let CacheTablePolicy {
        table,
        payload_column,
        identity_columns,
        newest_order,
        row_limit,
        byte_budget,
    } = policy;
    let sql = format!(
        r#"
        WITH ranked AS (
            SELECT {identity_columns},
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
        WHERE ({identity_columns}) IN (
            SELECT {identity_columns} FROM ranked
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
    cipher: &PayloadCipher,
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
            SELECT conversation_id, message_seq, message_id, payload_ciphertext, updated_at
            FROM im_local_message_cache
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
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .map_err(|error| format!("query offline messages failed: {error}"))?;
    let encrypted_items = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("collect offline messages failed: {error}"))?;
    let mut items = encrypted_items
        .into_iter()
        .map(
            |(conversation_id, message_seq, message_id, ciphertext, updated_at)| {
                let record_key = format!("{conversation_id}:{message_seq}");
                Ok(OfflineMessageRecord {
                    scope: scope.clone(),
                    conversation_id,
                    message_seq,
                    message_id,
                    payload_json: cipher.decrypt_json(
                        "message-cache",
                        record_key.as_str(),
                        ciphertext.as_str(),
                    )?,
                    updated_at,
                })
            },
        )
        .collect::<Result<Vec<_>, String>>()?;
    items.reverse();
    Ok(items)
}

fn map_pending_send_row(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<(String, String, String, String, i64)> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}

fn decrypt_pending_send_rows(
    cipher: &PayloadCipher,
    scope: &OfflinePrincipalScope,
    rows: Vec<(String, String, String, String, i64)>,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    rows.into_iter()
        .map(
            |(client_msg_id, conversation_id, ciphertext, created_at, attempt_count)| {
                Ok(OfflinePendingSendRecord {
                    scope: scope.clone(),
                    payload_json: cipher.decrypt_json(
                        "pending-send",
                        client_msg_id.as_str(),
                        ciphertext.as_str(),
                    )?,
                    client_msg_id,
                    conversation_id,
                    created_at,
                    attempt_count,
                })
            },
        )
        .collect()
}

fn claim_pending_sends(
    connection: &Connection,
    cipher: &PayloadCipher,
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
                UPDATE im_local_pending_send
                SET queue_status = 'quarantined',
                    quarantine_reason = 'retry budget exhausted',
                    quarantined_at_ms = ?5,
                    flush_claim_id = NULL,
                    flush_claimed_at_ms = NULL,
                    flush_claim_expires_at_ms = NULL
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND queue_status = 'pending'
                  AND attempt_count >= ?6
                  AND (flush_claim_id IS NULL OR flush_claim_expires_at_ms <= ?5)
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    now_ms,
                    MAX_PENDING_SEND_ATTEMPTS
                ],
            )
            .map_err(|error| {
                format!("quarantine exhausted offline pending sends failed: {error}")
            })?;
        enforce_pending_send_quarantine_policy(
            connection,
            scope,
            now_ms,
            PENDING_SEND_QUARANTINE_POLICY,
        )?;
        connection
            .execute(
                r#"
                UPDATE im_local_pending_send
                SET flush_claim_id = ?5,
                    flush_claimed_at_ms = ?6,
                    flush_claim_expires_at_ms = ?7,
                    attempt_count = attempt_count + 1
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND client_msg_id IN (
                    SELECT client_msg_id
                    FROM im_local_pending_send
                    WHERE tenant_id = ?1
                      AND organization_id = ?2
                      AND principal_kind = ?3
                      AND principal_id = ?4
                      AND queue_status = 'pending'
                      AND attempt_count < ?9
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
                    limit,
                    MAX_PENDING_SEND_ATTEMPTS
                ],
            )
            .map_err(|error| format!("claim offline pending sends failed: {error}"))?;

        let mut statement = connection
            .prepare(
                r#"
                SELECT client_msg_id, conversation_id, payload_ciphertext, created_at, attempt_count
                FROM im_local_pending_send
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
                map_pending_send_row,
            )
            .map_err(|error| format!("query claimed offline pending sends failed: {error}"))?;
        let encrypted = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect claimed offline pending sends failed: {error}"))?;
        decrypt_pending_send_rows(cipher, scope, encrypted)
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
            DELETE FROM im_local_pending_send
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
                UPDATE im_local_pending_send
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
            DELETE FROM im_local_pending_send
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
                SELECT tenant_id, organization_id, principal_kind, principal_id,
                       client_msg_id,
                       ROW_NUMBER() OVER (
                           ORDER BY quarantined_at_ms DESC, client_msg_id DESC
                       ) AS row_number,
                       SUM(
                           LENGTH(CAST(payload_ciphertext AS BLOB))
                           + LENGTH(CAST(quarantine_reason AS BLOB))
                       ) OVER (
                           ORDER BY quarantined_at_ms DESC, client_msg_id DESC
                       ) AS cumulative_bytes
                FROM im_local_pending_send
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND queue_status = 'quarantined'
            )
            DELETE FROM im_local_pending_send
            WHERE (
                tenant_id, organization_id, principal_kind, principal_id, client_msg_id
            ) IN (
                SELECT tenant_id, organization_id, principal_kind, principal_id, client_msg_id
                FROM ranked
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
            "im_local_cache_cursor",
            "im_local_message_cache",
            "im_local_conversation_cache",
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

fn validate_batch_scope<T>(
    records: &[T],
    resolve_scope: impl Fn(&T) -> &OfflinePrincipalScope,
) -> Result<Option<OfflinePrincipalScope>, String> {
    let Some(first) = records.first() else {
        return Ok(None);
    };
    let scope = resolve_scope(first);
    validate_scope(scope)?;
    if records.iter().any(|record| resolve_scope(record) != scope) {
        return Err("offline write batch must contain exactly one complete scope".to_owned());
    }
    Ok(Some(scope.clone()))
}

#[tauri::command]
pub async fn sdkwork_im_pc_offline_init(
    app: AppHandle,
    scope: OfflinePrincipalScope,
) -> Result<(), String> {
    with_scoped_connection_blocking(app, scope, |_| Ok(())).await
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
    let Some(scope) = validate_batch_scope(records.as_slice(), |record| &record.scope)? else {
        return Ok(0);
    };
    let now_ms = unix_epoch_millis()?;
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        with_immediate_transaction(&database.connection, |connection| {
            for record in &records {
                validate_identifier("conversationId", record.conversation_id.as_str())?;
                validate_payload(
                    "conversation payload",
                    record.payload_json.as_str(),
                    MAX_RECORD_PAYLOAD_BYTES,
                )?;
                validate_timestamp("updatedAt", record.updated_at.as_str())?;
                let ciphertext = database.cipher.encrypt_json(
                    "conversation-cache",
                    record.conversation_id.as_str(),
                    record.payload_json.as_str(),
                )?;
                connection
                    .execute(
                        r#"
                        INSERT INTO im_local_conversation_cache (
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, payload_ciphertext, updated_at, cached_at_ms
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                        ON CONFLICT(
                            tenant_id, organization_id, principal_kind, principal_id, conversation_id
                        ) DO UPDATE SET
                            payload_ciphertext = excluded.payload_ciphertext,
                            updated_at = excluded.updated_at,
                            cached_at_ms = excluded.cached_at_ms
                        "#,
                        params![
                            &scope.tenant_id,
                            &scope.organization_id,
                            &scope.principal_kind,
                            &scope.principal_id,
                            &record.conversation_id,
                            ciphertext,
                            &record.updated_at,
                            now_ms
                        ],
                    )
                    .map_err(|error| format!("upsert offline conversation failed: {error}"))?;
            }
            enforce_cache_policy(connection, &scope, now_ms, OFFLINE_CACHE_POLICY)?;
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        let mut statement = database
            .connection
            .prepare(
                r#"
                SELECT conversation_id, payload_ciphertext, updated_at
                FROM im_local_conversation_cache
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
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .map_err(|error| format!("query offline conversations failed: {error}"))?;
        let encrypted = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline conversations failed: {error}"))?;
        encrypted
            .into_iter()
            .map(|(conversation_id, ciphertext, updated_at)| {
                Ok(OfflineConversationRecord {
                    scope: scope.clone(),
                    payload_json: database.cipher.decrypt_json(
                        "conversation-cache",
                        conversation_id.as_str(),
                        ciphertext.as_str(),
                    )?,
                    conversation_id,
                    updated_at,
                })
            })
            .collect()
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
    let Some(scope) = validate_batch_scope(records.as_slice(), |record| &record.scope)? else {
        return Ok(0);
    };
    let now_ms = unix_epoch_millis()?;
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        with_immediate_transaction(&database.connection, |connection| {
            for record in &records {
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
                validate_timestamp("updatedAt", record.updated_at.as_str())?;
                let record_key = format!("{}:{}", record.conversation_id, record.message_seq);
                let ciphertext = database.cipher.encrypt_json(
                    "message-cache",
                    record_key.as_str(),
                    record.payload_json.as_str(),
                )?;
                connection
                    .execute(
                        r#"
                        INSERT INTO im_local_message_cache (
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, message_seq, message_id, payload_ciphertext,
                            updated_at, cached_at_ms
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                        ON CONFLICT(
                            tenant_id, organization_id, principal_kind, principal_id,
                            conversation_id, message_seq
                        ) DO UPDATE SET
                            message_id = excluded.message_id,
                            payload_ciphertext = excluded.payload_ciphertext,
                            updated_at = excluded.updated_at,
                            cached_at_ms = excluded.cached_at_ms
                        "#,
                        params![
                            &scope.tenant_id,
                            &scope.organization_id,
                            &scope.principal_kind,
                            &scope.principal_id,
                            &record.conversation_id,
                            record.message_seq,
                            &record.message_id,
                            ciphertext,
                            &record.updated_at,
                            now_ms
                        ],
                    )
                    .map_err(|error| format!("upsert offline message failed: {error}"))?;
            }
            enforce_cache_policy(connection, &scope, now_ms, OFFLINE_CACHE_POLICY)?;
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        list_messages_for_scope(
            &database.connection,
            &database.cipher,
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        let ciphertext = database
            .connection
            .query_row(
                r#"
                SELECT cursor_ciphertext
                FROM im_local_cache_cursor
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
            .map_err(|error| format!("read offline sync cursor failed: {error}"))?;
        ciphertext
            .map(|ciphertext| {
                database.cipher.decrypt_json(
                    "cache-cursor",
                    cursor_scope.as_str(),
                    ciphertext.as_str(),
                )
            })
            .transpose()
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
    validate_timestamp("updatedAt", updated_at.as_str())?;
    let now_ms = unix_epoch_millis()?;
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        let ciphertext = database.cipher.encrypt_json(
            "cache-cursor",
            cursor_scope.as_str(),
            cursor_json.as_str(),
        )?;
        with_immediate_transaction(&database.connection, |connection| {
            connection
                .execute(
                    r#"
                    INSERT INTO im_local_cache_cursor (
                        tenant_id, organization_id, principal_kind, principal_id,
                        cursor_scope, cursor_ciphertext, updated_at, cached_at_ms
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                    ON CONFLICT(
                        tenant_id, organization_id, principal_kind, principal_id, cursor_scope
                    ) DO UPDATE SET
                        cursor_ciphertext = excluded.cursor_ciphertext,
                        updated_at = excluded.updated_at,
                        cached_at_ms = excluded.cached_at_ms
                    "#,
                    params![
                        &scope.tenant_id,
                        &scope.organization_id,
                        &scope.principal_kind,
                        &scope.principal_id,
                        &cursor_scope,
                        ciphertext,
                        &updated_at,
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
    scope: &OfflinePrincipalScope,
    encrypted_payload_bytes: usize,
) -> Result<(), String> {
    let (row_count, payload_bytes): (i64, i64) = connection
        .query_row(
            r#"
            SELECT COUNT(*),
                   COALESCE(SUM(LENGTH(CAST(payload_ciphertext AS BLOB))), 0)
            FROM im_local_pending_send
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND principal_kind = ?3
              AND principal_id = ?4
              AND queue_status = 'pending'
            "#,
            params![
                &scope.tenant_id,
                &scope.organization_id,
                &scope.principal_kind,
                &scope.principal_id
            ],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| format!("read offline pending send capacity failed: {error}"))?;
    let next_rows = row_count.saturating_add(1);
    let next_bytes = payload_bytes.saturating_add(encrypted_payload_bytes as i64);
    if next_rows > MAX_PENDING_SEND_ROWS_PER_SCOPE || next_bytes > MAX_PENDING_SEND_BYTES_PER_SCOPE
    {
        return Err(
            "offline pending send queue capacity exceeded; reconnect before sending more messages"
                .into(),
        );
    }
    Ok(())
}

fn enqueue_pending_send(
    connection: &Connection,
    cipher: &PayloadCipher,
    record: &OfflinePendingSendRecord,
    now_ms: i64,
) -> Result<(), String> {
    validate_scope(&record.scope)?;
    validate_identifier("clientMsgId", record.client_msg_id.as_str())?;
    validate_identifier("conversationId", record.conversation_id.as_str())?;
    validate_payload(
        "pending send payload",
        record.payload_json.as_str(),
        MAX_RECORD_PAYLOAD_BYTES,
    )?;
    validate_timestamp("createdAt", record.created_at.as_str())?;
    let hash = payload_hash(record.payload_json.as_str());
    let ciphertext = cipher.encrypt_json(
        "pending-send",
        record.client_msg_id.as_str(),
        record.payload_json.as_str(),
    )?;
    with_immediate_transaction(connection, |connection| {
        let existing = connection
            .query_row(
                r#"
                SELECT conversation_id, payload_hash
                FROM im_local_pending_send
                WHERE tenant_id = ?1
                  AND organization_id = ?2
                  AND principal_kind = ?3
                  AND principal_id = ?4
                  AND client_msg_id = ?5
                "#,
                params![
                    &record.scope.tenant_id,
                    &record.scope.organization_id,
                    &record.scope.principal_kind,
                    &record.scope.principal_id,
                    &record.client_msg_id
                ],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
            .map_err(|error| {
                format!("read offline pending send idempotency state failed: {error}")
            })?;
        if let Some((existing_conversation_id, existing_hash)) = existing {
            if existing_conversation_id == record.conversation_id && existing_hash == hash {
                return Ok(());
            }
            return Err(format!(
                "offline pending send idempotency conflict for clientMsgId {}",
                record.client_msg_id
            ));
        }
        ensure_pending_send_capacity(connection, &record.scope, ciphertext.len())?;
        connection
            .execute(
                r#"
                INSERT INTO im_local_pending_send (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_ciphertext, payload_hash,
                    created_at, created_at_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                "#,
                params![
                    &record.scope.tenant_id,
                    &record.scope.organization_id,
                    &record.scope.principal_kind,
                    &record.scope.principal_id,
                    &record.client_msg_id,
                    &record.conversation_id,
                    ciphertext,
                    hash,
                    &record.created_at,
                    now_ms
                ],
            )
            .map_err(|error| format!("enqueue offline pending send failed: {error}"))?;
        Ok(())
    })
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
    let scope = record.scope.clone();
    with_scoped_connection_blocking(app, scope, move |database| {
        enqueue_pending_send(&database.connection, &database.cipher, &record, now_ms)
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        let mut statement = database
            .connection
            .prepare(
                r#"
                SELECT client_msg_id, conversation_id, payload_ciphertext, created_at, attempt_count
                FROM im_local_pending_send
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
                map_pending_send_row,
            )
            .map_err(|error| format!("query offline pending sends failed: {error}"))?;
        let encrypted = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline pending sends failed: {error}"))?;
        decrypt_pending_send_rows(&database.cipher, &scope, encrypted)
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        claim_pending_sends(
            &database.connection,
            &database.cipher,
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        let released = database
            .connection
            .execute(
                r#"
                UPDATE im_local_pending_send
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        acknowledge_pending_send(
            &database.connection,
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        quarantine_pending_send(
            &database.connection,
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
    with_scoped_connection_blocking(app, scope.clone(), move |database| {
        purge_principal_cache(&database.connection, &scope)
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("open test database");
        connection
            .execute_batch(include_str!(
                "../database/migrations/sqlite/0004_create_im_pc_client_local_store.up.sql"
            ))
            .expect("apply client-local migration");
        connection
    }

    fn principal_scope(principal_id: &str) -> OfflinePrincipalScope {
        OfflinePrincipalScope {
            environment: "development".into(),
            deployment_profile: "standalone".into(),
            deployment_mode: "local".into(),
            api_origin: "http://127.0.0.1:18079".into(),
            tenant_id: "100001".into(),
            organization_id: "org-a".into(),
            account_id: principal_id.into(),
            principal_kind: "user".into(),
            principal_id: principal_id.into(),
        }
    }

    fn test_cipher(scope: &OfflinePrincipalScope) -> PayloadCipher {
        PayloadCipher::for_test(&scope_fingerprint(scope).expect("scope fingerprint"))
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
        let cipher = test_cipher(scope);
        let payload = format!(r#"{{"clientMsgId":"{client_msg_id}"}}"#);
        let ciphertext = cipher
            .encrypt_json("pending-send", client_msg_id, payload.as_str())
            .expect("encrypt pending send fixture");
        connection
            .execute(
                r#"
                INSERT INTO im_local_pending_send (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_ciphertext, payload_hash,
                    created_at, created_at_ms, attempt_count, flush_claim_id,
                    flush_claimed_at_ms, flush_claim_expires_at_ms
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, 'conversation', ?6, ?7,
                    '2026-07-10T00:00:00Z', 1, 0, ?8, ?9, ?10
                )
                "#,
                params![
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.principal_kind,
                    &scope.principal_id,
                    client_msg_id,
                    ciphertext,
                    payload_hash(payload.as_str()),
                    claim_id,
                    claimed_at_ms,
                    claim_expires_at_ms
                ],
            )
            .expect("insert pending send fixture");
    }

    #[test]
    fn offline_store_configures_versioned_safe_sqlite_profile() {
        let connection = test_connection();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("read user version");
        let foreign_keys: i64 = connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("read foreign key setting");
        assert_eq!(version, OFFLINE_SCHEMA_VERSION);
        assert_eq!(foreign_keys, 1);
    }

    #[test]
    fn write_batches_reject_mixed_complete_scopes() {
        let first = OfflineConversationRecord {
            scope: principal_scope("user-1"),
            conversation_id: "conversation-1".into(),
            payload_json: "{}".into(),
            updated_at: "2026-07-10T00:00:00Z".into(),
        };
        let second = OfflineConversationRecord {
            scope: principal_scope("user-2"),
            conversation_id: "conversation-2".into(),
            payload_json: "{}".into(),
            updated_at: "2026-07-10T00:00:00Z".into(),
        };
        assert!(validate_batch_scope(&[first, second], |record| &record.scope).is_err());
    }

    #[test]
    fn offline_store_isolates_conversations_messages_cursors_and_pending_sends_by_principal() {
        let connection = test_connection();
        let first = principal_scope("user-1");
        let second = principal_scope("user-2");

        for scope in [&first, &second] {
            let cipher = test_cipher(scope);
            connection.execute(
                "INSERT INTO im_local_conversation_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, payload_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'shared', ?5, '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, cipher.encrypt_json("conversation-cache", "shared", "{}").expect("encrypt conversation")],
            ).expect("insert conversation");
            connection.execute(
                "INSERT INTO im_local_message_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'shared', 1, 'message', ?5, '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, cipher.encrypt_json("message-cache", "shared:1", "{}").expect("encrypt message")],
            ).expect("insert message");
            connection.execute(
                "INSERT INTO im_local_cache_cursor (tenant_id, organization_id, principal_kind, principal_id, cursor_scope, cursor_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'inbox', ?5, '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, cipher.encrypt_json("cache-cursor", "inbox", "{}").expect("encrypt cursor")],
            ).expect("insert cursor");
            insert_pending_send_for_test(&connection, scope, "shared-client-message", None);
        }

        for table in [
            "im_local_conversation_cache",
            "im_local_message_cache",
            "im_local_cache_cursor",
            "im_local_pending_send",
        ] {
            let count: i64 = connection.query_row(
                format!("SELECT COUNT(*) FROM {table} WHERE tenant_id = ?1 AND organization_id = ?2 AND principal_kind = ?3 AND principal_id = ?4").as_str(),
                params![&first.tenant_id, &first.organization_id, &first.principal_kind, &first.principal_id],
                |row| row.get(0),
            ).expect("count principal rows");
            assert_eq!(count, 1, "{table} must isolate the first principal");
        }
        let first_messages = list_messages_for_scope(
            &connection,
            &test_cipher(&first),
            &first,
            "shared",
            None,
            20,
        )
        .expect("list first principal messages");
        assert_eq!(first_messages.len(), 1);
        assert_eq!(first_messages[0].scope, first);
    }

    #[test]
    fn backward_message_pages_return_chronological_windows_from_latest() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        for seq in 1..=4 {
            let record_key = format!("conversation:{seq}");
            connection.execute(
                "INSERT INTO im_local_message_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', ?5, ?6, ?7, '2026-07-10T00:00:00Z', ?5)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, seq, format!("message-{seq}"), cipher.encrypt_json("message-cache", record_key.as_str(), "{}").expect("encrypt message")],
            ).expect("insert message");
        }
        let latest = list_messages_for_scope(&connection, &cipher, &scope, "conversation", None, 2)
            .expect("latest page");
        assert_eq!(
            latest
                .iter()
                .map(|item| item.message_seq)
                .collect::<Vec<_>>(),
            vec![3, 4]
        );
        let older =
            list_messages_for_scope(&connection, &cipher, &scope, "conversation", Some(3), 2)
                .expect("older page");
        assert_eq!(
            older
                .iter()
                .map(|item| item.message_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn expired_pending_send_claim_is_recovered_without_stealing_a_live_claim() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        insert_pending_send_for_test(&connection, &scope, "expired", Some(("old", 100, 200)));
        insert_pending_send_for_test(&connection, &scope, "live", Some(("live", 900, 1_100)));
        let claimed = claim_pending_sends(
            &connection,
            &cipher,
            &scope,
            "replacement",
            1_000,
            60_000,
            10,
        )
        .expect("claim pending sends");
        assert_eq!(
            claimed
                .iter()
                .map(|record| record.client_msg_id.as_str())
                .collect::<Vec<_>>(),
            vec!["expired"]
        );
    }

    #[test]
    fn stale_pending_send_claim_cannot_acknowledge_a_reclaimed_row() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        insert_pending_send_for_test(&connection, &scope, "message", None);
        claim_pending_sends(&connection, &cipher, &scope, "old-claim", 100, 100, 1)
            .expect("old claim");
        claim_pending_sends(&connection, &cipher, &scope, "new-claim", 201, 100, 1)
            .expect("new claim");

        assert!(
            !acknowledge_pending_send(&connection, &scope, "message", "old-claim")
                .expect("stale acknowledgement")
        );
        assert!(
            acknowledge_pending_send(&connection, &scope, "message", "new-claim")
                .expect("current acknowledgement")
        );
    }

    #[test]
    fn only_current_claim_can_quarantine_a_corrupt_pending_send() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        insert_pending_send_for_test(&connection, &scope, "message", None);
        claim_pending_sends(&connection, &cipher, &scope, "old-claim", 100, 100, 1)
            .expect("old claim");
        claim_pending_sends(&connection, &cipher, &scope, "new-claim", 201, 100, 1)
            .expect("new claim");

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
            claim_pending_sends(&connection, &cipher, &scope, "third-claim", 203, 100, 1)
                .expect("claim after quarantine")
                .is_empty()
        );
        let status: (String, String) = connection
            .query_row(
                "SELECT queue_status, quarantine_reason FROM im_local_pending_send WHERE client_msg_id = 'message'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("read quarantine state");
        assert_eq!(status.0, "quarantined");
        assert_eq!(status.1, "invalid pending send payload");
    }

    #[test]
    fn quarantine_policy_is_bounded_without_deleting_pending_sends() {
        let connection = test_connection();
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
                    "UPDATE im_local_pending_send SET queue_status = 'quarantined', quarantine_reason = 'invalid', quarantined_at_ms = ?1 WHERE client_msg_id = ?2",
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
                    "SELECT client_msg_id FROM im_local_pending_send WHERE queue_status = 'quarantined' ORDER BY quarantined_at_ms",
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
                "SELECT COUNT(*) FROM im_local_pending_send WHERE client_msg_id = 'pending' AND queue_status = 'pending'",
                [],
                |row| row.get(0),
            )
            .expect("count pending row");
        assert_eq!(pending_count, 1);
    }

    #[test]
    fn principal_cache_purge_preserves_unsent_and_other_principal_rows() {
        let connection = test_connection();
        let first = principal_scope("user-1");
        let second = principal_scope("user-2");
        for scope in [&first, &second] {
            let ciphertext = test_cipher(scope)
                .encrypt_json("conversation-cache", "conversation", "{}")
                .expect("encrypt conversation");
            connection.execute(
                "INSERT INTO im_local_conversation_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, payload_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', ?5, '2026-07-10T00:00:00Z', 1)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, ciphertext],
            ).expect("insert conversation");
        }
        insert_pending_send_for_test(&connection, &first, "unsent", None);

        assert_eq!(
            purge_principal_cache(&connection, &first).expect("purge cache"),
            1
        );
        let first_cache: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM im_local_conversation_cache WHERE principal_id = 'user-1'",
                [],
                |row| row.get(0),
            )
            .expect("first cache count");
        let second_cache: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM im_local_conversation_cache WHERE principal_id = 'user-2'",
                [],
                |row| row.get(0),
            )
            .expect("second cache count");
        let pending: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM im_local_pending_send WHERE principal_id = 'user-1'",
                [],
                |row| row.get(0),
            )
            .expect("pending count");
        assert_eq!(first_cache, 0);
        assert_eq!(second_cache, 1);
        assert_eq!(pending, 1);
    }

    #[test]
    fn plaintext_payload_never_enters_the_sqlite_file() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "sdkwork-im-pc-client-local-plaintext-{nonce}.sqlite"
        ));
        let connection = Connection::open(path.as_path()).expect("open file database");
        connection
            .execute_batch(include_str!(
                "../database/migrations/sqlite/0004_create_im_pc_client_local_store.up.sql"
            ))
            .expect("apply client-local migration");
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        let marker = "plaintext-must-not-reach-sqlite";
        let record = OfflinePendingSendRecord {
            scope,
            client_msg_id: "client-message".into(),
            conversation_id: "conversation".into(),
            payload_json: format!(r#"{{"content":"{marker}"}}"#),
            created_at: "2026-07-10T00:00:00Z".into(),
            attempt_count: 0,
        };
        enqueue_pending_send(&connection, &cipher, &record, 1).expect("enqueue encrypted row");
        drop(connection);
        let bytes = std::fs::read(path.as_path()).expect("read sqlite file");
        assert!(!bytes
            .windows(marker.len())
            .any(|window| window == marker.as_bytes()));
        std::fs::remove_file(path.as_path()).expect("remove test database");
    }

    #[test]
    fn pending_send_exact_replay_is_idempotent_and_conflicts_do_not_overwrite() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        let mut record = OfflinePendingSendRecord {
            scope,
            client_msg_id: "client-message".into(),
            conversation_id: "conversation".into(),
            payload_json: r#"{"content":"plaintext-marker"}"#.into(),
            created_at: "2026-07-10T00:00:00Z".into(),
            attempt_count: 0,
        };
        enqueue_pending_send(&connection, &cipher, &record, 1).expect("first enqueue");
        enqueue_pending_send(&connection, &cipher, &record, 2).expect("exact replay");
        let (count, ciphertext): (i64, String) = connection
            .query_row(
                "SELECT COUNT(*), MAX(payload_ciphertext) FROM im_local_pending_send",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("read encrypted row");
        assert_eq!(count, 1);
        assert!(!ciphertext.contains("plaintext-marker"));

        record.payload_json = r#"{"content":"different"}"#.into();
        assert!(enqueue_pending_send(&connection, &cipher, &record, 3).is_err());
        let persisted_hash: String = connection
            .query_row(
                "SELECT payload_hash FROM im_local_pending_send WHERE client_msg_id = 'client-message'",
                [],
                |row| row.get(0),
            )
            .expect("read persisted hash");
        assert_eq!(
            persisted_hash,
            payload_hash(r#"{"content":"plaintext-marker"}"#)
        );
    }

    #[test]
    fn exhausted_pending_send_is_quarantined_before_another_claim() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        insert_pending_send_for_test(&connection, &scope, "exhausted", None);
        connection
            .execute(
                "UPDATE im_local_pending_send SET attempt_count = ?1 WHERE client_msg_id = 'exhausted'",
                [MAX_PENDING_SEND_ATTEMPTS],
            )
            .expect("set retry count");
        assert!(
            claim_pending_sends(&connection, &cipher, &scope, "claim", 1_000, 60_000, 1,)
                .expect("claim")
                .is_empty()
        );
        let status: (String, String) = connection
            .query_row(
                "SELECT queue_status, quarantine_reason FROM im_local_pending_send WHERE client_msg_id = 'exhausted'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("read quarantine state");
        assert_eq!(
            status,
            ("quarantined".into(), "retry budget exhausted".into())
        );
    }

    #[test]
    fn pending_send_capacity_rejects_a_row_beyond_the_bound() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        connection
            .execute(
                r#"
                WITH RECURSIVE sequence(value) AS (
                    SELECT 1
                    UNION ALL
                    SELECT value + 1 FROM sequence WHERE value < ?5
                )
                INSERT INTO im_local_pending_send (
                    tenant_id, organization_id, principal_kind, principal_id,
                    client_msg_id, conversation_id, payload_ciphertext, payload_hash,
                    created_at, created_at_ms, attempt_count
                )
                SELECT ?1, ?2, ?3, ?4, printf('message-%05d', value),
                       'conversation', 'enc-v1:fixture',
                       '0000000000000000000000000000000000000000000000000000000000000000',
                       '2026-07-10T00:00:00Z', value, 0
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
        assert!(ensure_pending_send_capacity(&connection, &scope, 64).is_err());
    }

    #[test]
    fn cache_policy_evicts_old_cache_rows_but_preserves_unsent_rows() {
        let connection = test_connection();
        let scope = principal_scope("user-1");
        let cipher = test_cipher(&scope);
        for seq in 1..=3 {
            let record_key = format!("conversation:{seq}");
            connection.execute(
                "INSERT INTO im_local_message_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, message_seq, message_id, payload_ciphertext, updated_at, cached_at_ms) VALUES (?1, ?2, ?3, ?4, 'conversation', ?5, ?6, ?7, '2026-07-10T00:00:00Z', ?5)",
                params![&scope.tenant_id, &scope.organization_id, &scope.principal_kind, &scope.principal_id, seq, format!("message-{seq}"), cipher.encrypt_json("message-cache", record_key.as_str(), r#"{"value":"1234567890"}"#).expect("encrypt message")],
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
                .prepare("SELECT message_seq FROM im_local_message_cache ORDER BY message_seq")
                .expect("prepare");
            statement
                .query_map([], |row| row.get(0))
                .expect("query")
                .collect::<Result<Vec<_>, _>>()
                .expect("collect")
        };
        assert_eq!(remaining, vec![2, 3]);
        let pending_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM im_local_pending_send", [], |row| {
                row.get(0)
            })
            .expect("pending count");
        assert_eq!(pending_count, 1);
    }
}
