//! Desktop offline SQLite cache for messages, conversations, and sync cursors.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const OFFLINE_DB_FILE: &str = "offline-im-cache.sqlite";
const DEFAULT_MESSAGE_PAGE_LIMIT: usize = 200;

static OFFLINE_DB: Mutex<Option<Connection>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineMessageRecord {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_seq: i64,
    pub message_id: String,
    pub payload_json: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineConversationRecord {
    pub tenant_id: String,
    pub conversation_id: String,
    pub payload_json: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePendingSendRecord {
    pub tenant_id: String,
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

fn open_connection(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create offline db parent dir failed: {error}"))?;
    }
    let connection = Connection::open(path)
        .map_err(|error| format!("open offline sqlite db failed: {error}"))?;
    connection
        .execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            CREATE TABLE IF NOT EXISTS offline_conversations (
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (tenant_id, conversation_id)
            );
            CREATE TABLE IF NOT EXISTS offline_messages (
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                message_seq INTEGER NOT NULL,
                message_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (tenant_id, conversation_id, message_seq)
            );
            CREATE INDEX IF NOT EXISTS idx_offline_messages_conversation_seq
                ON offline_messages (tenant_id, conversation_id, message_seq);
            CREATE TABLE IF NOT EXISTS offline_sync_cursors (
                tenant_id TEXT NOT NULL,
                scope TEXT NOT NULL,
                cursor_json TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (tenant_id, scope)
            );
            CREATE TABLE IF NOT EXISTS offline_pending_sends (
                tenant_id TEXT NOT NULL,
                client_msg_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                attempt_count INTEGER NOT NULL DEFAULT 0,
                flush_claim_id TEXT,
                PRIMARY KEY (tenant_id, client_msg_id)
            );
            CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_tenant_created
                ON offline_pending_sends (tenant_id, created_at);
            CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_tenant_claim
                ON offline_pending_sends (tenant_id, flush_claim_id);
            "#,
        )
        .map_err(|error| format!("initialize offline sqlite schema failed: {error}"))?;
    migrate_offline_schema(&connection)?;
    Ok(connection)
}

fn migrate_offline_schema(connection: &Connection) -> Result<(), String> {
    let _ = connection.execute(
        "ALTER TABLE offline_pending_sends ADD COLUMN flush_claim_id TEXT",
        [],
    );
    connection
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_offline_pending_sends_tenant_claim
             ON offline_pending_sends (tenant_id, flush_claim_id)",
            [],
        )
        .map_err(|error| format!("migrate offline pending send claim index failed: {error}"))?;
    Ok(())
}

fn with_immediate_transaction<R>(
    connection: &Connection,
    operation: impl FnOnce(&Connection) -> Result<R, String>,
) -> Result<R, String> {
    connection
        .execute("BEGIN IMMEDIATE", [])
        .map_err(|error| format!("begin offline sqlite transaction failed: {error}"))?;
    match operation(connection) {
        Ok(value) => {
            connection
                .execute("COMMIT", [])
                .map_err(|error| format!("commit offline sqlite transaction failed: {error}"))?;
            Ok(value)
        }
        Err(error) => {
            let _ = connection.execute("ROLLBACK", []);
            Err(error)
        }
    }
}

fn map_pending_send_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<OfflinePendingSendRecord> {
    Ok(OfflinePendingSendRecord {
        tenant_id: row.get(0)?,
        client_msg_id: row.get(1)?,
        conversation_id: row.get(2)?,
        payload_json: row.get(3)?,
        created_at: row.get(4)?,
        attempt_count: row.get(5)?,
    })
}

fn with_connection<R>(app: &AppHandle, operation: impl FnOnce(&Connection) -> Result<R, String>) -> Result<R, String> {
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

#[tauri::command]
pub fn sdkwork_im_pc_offline_init(app: AppHandle) -> Result<(), String> {
    with_connection(&app, |_| Ok(()))
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_upsert_conversations(
    app: AppHandle,
    records: Vec<OfflineConversationRecord>,
) -> Result<usize, String> {
    with_connection(&app, |connection| {
        with_immediate_transaction(connection, |connection| {
            let mut count = 0usize;
            for record in records {
                connection
                    .execute(
                        r#"
                        INSERT INTO offline_conversations (tenant_id, conversation_id, payload_json, updated_at)
                        VALUES (?1, ?2, ?3, ?4)
                        ON CONFLICT(tenant_id, conversation_id) DO UPDATE SET
                            payload_json = excluded.payload_json,
                            updated_at = excluded.updated_at
                        "#,
                        params![
                            record.tenant_id,
                            record.conversation_id,
                            record.payload_json,
                            record.updated_at
                        ],
                    )
                    .map_err(|error| format!("upsert offline conversation failed: {error}"))?;
                count += 1;
            }
            Ok(count)
        })
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_list_conversations(
    app: AppHandle,
    tenant_id: String,
    limit: Option<usize>,
) -> Result<Vec<OfflineConversationRecord>, String> {
    let limit = limit.unwrap_or(DEFAULT_MESSAGE_PAGE_LIMIT).max(1) as i64;
    with_connection(&app, |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT tenant_id, conversation_id, payload_json, updated_at
                FROM offline_conversations
                WHERE tenant_id = ?1
                ORDER BY updated_at DESC
                LIMIT ?2
                "#,
            )
            .map_err(|error| format!("prepare offline conversation list failed: {error}"))?;
        let rows = statement
            .query_map(params![tenant_id, limit], |row| {
                Ok(OfflineConversationRecord {
                    tenant_id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    payload_json: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            })
            .map_err(|error| format!("query offline conversations failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline conversations failed: {error}"))
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_upsert_messages(
    app: AppHandle,
    records: Vec<OfflineMessageRecord>,
) -> Result<usize, String> {
    with_connection(&app, |connection| {
        with_immediate_transaction(connection, |connection| {
            let mut count = 0usize;
            for record in records {
                connection
                    .execute(
                        r#"
                        INSERT INTO offline_messages (
                            tenant_id, conversation_id, message_seq, message_id, payload_json, updated_at
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                        ON CONFLICT(tenant_id, conversation_id, message_seq) DO UPDATE SET
                            message_id = excluded.message_id,
                            payload_json = excluded.payload_json,
                            updated_at = excluded.updated_at
                        "#,
                        params![
                            record.tenant_id,
                            record.conversation_id,
                            record.message_seq,
                            record.message_id,
                            record.payload_json,
                            record.updated_at
                        ],
                    )
                    .map_err(|error| format!("upsert offline message failed: {error}"))?;
                count += 1;
            }
            Ok(count)
        })
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_list_messages(
    app: AppHandle,
    tenant_id: String,
    conversation_id: String,
    after_seq: Option<i64>,
    limit: Option<usize>,
) -> Result<Vec<OfflineMessageRecord>, String> {
    let after_seq = after_seq.unwrap_or(0);
    let limit = limit.unwrap_or(DEFAULT_MESSAGE_PAGE_LIMIT).max(1) as i64;
    with_connection(&app, |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT tenant_id, conversation_id, message_seq, message_id, payload_json, updated_at
                FROM offline_messages
                WHERE tenant_id = ?1
                  AND conversation_id = ?2
                  AND message_seq > ?3
                ORDER BY message_seq ASC
                LIMIT ?4
                "#,
            )
            .map_err(|error| format!("prepare offline message list failed: {error}"))?;
        let rows = statement
            .query_map(params![tenant_id, conversation_id, after_seq, limit], |row| {
                Ok(OfflineMessageRecord {
                    tenant_id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    message_seq: row.get(2)?,
                    message_id: row.get(3)?,
                    payload_json: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })
            .map_err(|error| format!("query offline messages failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline messages failed: {error}"))
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_get_sync_cursor(
    app: AppHandle,
    tenant_id: String,
    scope: String,
) -> Result<Option<String>, String> {
    with_connection(&app, |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT cursor_json
                FROM offline_sync_cursors
                WHERE tenant_id = ?1 AND scope = ?2
                "#,
            )
            .map_err(|error| format!("prepare offline sync cursor read failed: {error}"))?;
        let mut rows = statement
            .query_map(params![tenant_id, scope], |row| row.get::<_, String>(0))
            .map_err(|error| format!("query offline sync cursor failed: {error}"))?;
        match rows.next() {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(error)) => Err(format!("read offline sync cursor failed: {error}")),
            None => Ok(None),
        }
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_set_sync_cursor(
    app: AppHandle,
    tenant_id: String,
    scope: String,
    cursor_json: String,
    updated_at: String,
) -> Result<(), String> {
    with_connection(&app, |connection| {
        connection
            .execute(
                r#"
                INSERT INTO offline_sync_cursors (tenant_id, scope, cursor_json, updated_at)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(tenant_id, scope) DO UPDATE SET
                    cursor_json = excluded.cursor_json,
                    updated_at = excluded.updated_at
                "#,
                params![tenant_id, scope, cursor_json, updated_at],
            )
            .map_err(|error| format!("upsert offline sync cursor failed: {error}"))?;
        Ok(())
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_enqueue_pending_send(
    app: AppHandle,
    record: OfflinePendingSendRecord,
) -> Result<(), String> {
    with_connection(&app, |connection| {
        connection
            .execute(
                r#"
                INSERT INTO offline_pending_sends (
                    tenant_id, client_msg_id, conversation_id, payload_json, created_at, attempt_count
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(tenant_id, client_msg_id) DO UPDATE SET
                    conversation_id = excluded.conversation_id,
                    payload_json = excluded.payload_json,
                    created_at = excluded.created_at,
                    attempt_count = excluded.attempt_count
                "#,
                params![
                    record.tenant_id,
                    record.client_msg_id,
                    record.conversation_id,
                    record.payload_json,
                    record.created_at,
                    record.attempt_count
                ],
            )
            .map_err(|error| format!("enqueue offline pending send failed: {error}"))?;
        Ok(())
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_list_pending_sends(
    app: AppHandle,
    tenant_id: String,
    limit: Option<usize>,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    let limit = limit.unwrap_or(DEFAULT_MESSAGE_PAGE_LIMIT).max(1) as i64;
    with_connection(&app, |connection| {
        let mut statement = connection
            .prepare(
                r#"
                SELECT tenant_id, client_msg_id, conversation_id, payload_json, created_at, attempt_count
                FROM offline_pending_sends
                WHERE tenant_id = ?1 AND flush_claim_id IS NULL
                ORDER BY created_at ASC
                LIMIT ?2
                "#,
            )
            .map_err(|error| format!("prepare offline pending send list failed: {error}"))?;
        let rows = statement
            .query_map(params![tenant_id, limit], map_pending_send_row)
            .map_err(|error| format!("query offline pending sends failed: {error}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("collect offline pending sends failed: {error}"))
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_claim_pending_sends(
    app: AppHandle,
    tenant_id: String,
    claim_id: String,
    limit: Option<usize>,
) -> Result<Vec<OfflinePendingSendRecord>, String> {
    let limit = limit.unwrap_or(DEFAULT_MESSAGE_PAGE_LIMIT).max(1) as i64;
    if claim_id.trim().is_empty() {
        return Err("offline pending send claim id must not be empty".into());
    }
    with_connection(&app, |connection| {
        with_immediate_transaction(connection, |connection| {
            connection
                .execute(
                    r#"
                    UPDATE offline_pending_sends
                    SET flush_claim_id = ?2,
                        attempt_count = attempt_count + 1
                    WHERE rowid IN (
                        SELECT rowid
                        FROM offline_pending_sends
                        WHERE tenant_id = ?1 AND flush_claim_id IS NULL
                        ORDER BY created_at ASC
                        LIMIT ?3
                    )
                    "#,
                    params![tenant_id, claim_id, limit],
                )
                .map_err(|error| format!("claim offline pending sends failed: {error}"))?;

            let mut statement = connection
                .prepare(
                    r#"
                    SELECT tenant_id, client_msg_id, conversation_id, payload_json, created_at, attempt_count
                    FROM offline_pending_sends
                    WHERE tenant_id = ?1 AND flush_claim_id = ?2
                    ORDER BY created_at ASC
                    "#,
                )
                .map_err(|error| format!("prepare claimed offline pending send list failed: {error}"))?;
            let rows = statement
                .query_map(params![tenant_id, claim_id], map_pending_send_row)
                .map_err(|error| format!("query claimed offline pending sends failed: {error}"))?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|error| format!("collect claimed offline pending sends failed: {error}"))
        })
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_release_pending_send_claim(
    app: AppHandle,
    tenant_id: String,
    client_msg_id: String,
    claim_id: String,
) -> Result<bool, String> {
    with_connection(&app, |connection| {
        let released = connection
            .execute(
                r#"
                UPDATE offline_pending_sends
                SET flush_claim_id = NULL
                WHERE tenant_id = ?1
                  AND client_msg_id = ?2
                  AND flush_claim_id = ?3
                "#,
                params![tenant_id, client_msg_id, claim_id],
            )
            .map_err(|error| format!("release offline pending send claim failed: {error}"))?;
        Ok(released > 0)
    })
}

#[tauri::command]
pub fn sdkwork_im_pc_offline_delete_pending_send(
    app: AppHandle,
    tenant_id: String,
    client_msg_id: String,
) -> Result<bool, String> {
    with_connection(&app, |connection| {
        let deleted = connection
            .execute(
                "DELETE FROM offline_pending_sends WHERE tenant_id = ?1 AND client_msg_id = ?2",
                params![tenant_id, client_msg_id],
            )
            .map_err(|error| format!("delete offline pending send failed: {error}"))?;
        Ok(deleted > 0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("sdkwork-im-offline-test-{nanos}.sqlite"))
    }

    #[test]
    fn offline_store_round_trips_messages_and_cursors() {
        let path = temp_db_path();
        let connection = open_connection(path.as_path()).expect("open offline db");
        connection
            .execute(
                r#"
                INSERT INTO offline_messages (
                    tenant_id, conversation_id, message_seq, message_id, payload_json, updated_at
                ) VALUES ('100001', 'c1', 1, 'm1', '{"text":"hi"}', '2026-07-06T00:00:00.000Z')
                "#,
                [],
            )
            .expect("insert message");
        connection
            .execute(
                r#"
                INSERT INTO offline_sync_cursors (tenant_id, scope, cursor_json, updated_at)
                VALUES ('100001', 'inbox', '{"cursor":"abc"}', '2026-07-06T00:00:00.000Z')
                "#,
                [],
            )
            .expect("insert cursor");

        let mut statement = connection
            .prepare(
                "SELECT message_id FROM offline_messages WHERE tenant_id = '100001' AND conversation_id = 'c1'",
            )
            .expect("prepare");
        let message_id: String = statement
            .query_row([], |row| row.get(0))
            .expect("message row");
        assert_eq!(message_id, "m1");

        connection
            .execute(
                r#"
                INSERT INTO offline_pending_sends (
                    tenant_id, client_msg_id, conversation_id, payload_json, created_at, attempt_count
                ) VALUES ('100001', 'pc-1', 'c1', '{"content":"hi"}', '2026-07-06T00:00:00.000Z', 0)
                "#,
                [],
            )
            .expect("insert pending send");
        let pending_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM offline_pending_sends WHERE tenant_id = '100001'",
                [],
                |row| row.get(0),
            )
            .expect("count pending");
        assert_eq!(pending_count, 1);

        let claim_id = "claim-test-1";
        let claimed = claim_pending_sends_for_test(&connection, "100001", claim_id, 10)
            .expect("claim pending sends");
        assert_eq!(claimed.len(), 1);
        assert_eq!(claimed[0].client_msg_id, "pc-1");
        assert_eq!(claimed[0].attempt_count, 1);

        let released = connection
            .execute(
                "UPDATE offline_pending_sends SET flush_claim_id = NULL WHERE tenant_id = '100001' AND client_msg_id = 'pc-1'",
                [],
            )
            .expect("release claim");
        assert_eq!(released, 1);

        let _ = fs::remove_file(path);
    }

    fn claim_pending_sends_for_test(
        connection: &Connection,
        tenant_id: &str,
        claim_id: &str,
        limit: i64,
    ) -> Result<Vec<OfflinePendingSendRecord>, String> {
        with_immediate_transaction(connection, |connection| {
            connection
                .execute(
                    r#"
                    UPDATE offline_pending_sends
                    SET flush_claim_id = ?2,
                        attempt_count = attempt_count + 1
                    WHERE rowid IN (
                        SELECT rowid
                        FROM offline_pending_sends
                        WHERE tenant_id = ?1 AND flush_claim_id IS NULL
                        ORDER BY created_at ASC
                        LIMIT ?3
                    )
                    "#,
                    params![tenant_id, claim_id, limit],
                )
                .map_err(|error| format!("claim offline pending sends failed: {error}"))?;
            let mut statement = connection
                .prepare(
                    r#"
                    SELECT tenant_id, client_msg_id, conversation_id, payload_json, created_at, attempt_count
                    FROM offline_pending_sends
                    WHERE tenant_id = ?1 AND flush_claim_id = ?2
                    ORDER BY created_at ASC
                    "#,
                )
                .map_err(|error| format!("prepare claimed offline pending send list failed: {error}"))?;
            let rows = statement
                .query_map(params![tenant_id, claim_id], map_pending_send_row)
                .map_err(|error| format!("query claimed offline pending sends failed: {error}"))?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|error| format!("collect claimed offline pending sends failed: {error}"))
        })
    }
}
