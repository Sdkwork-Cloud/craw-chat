use rusqlite::Connection;
use serde_json::Value;

const MANIFEST: &str = include_str!("../database/database.manifest.json");
const MIGRATION: &str =
    include_str!("../database/migrations/sqlite/0004_create_im_pc_client_local_store.up.sql");
const BASELINE: &str =
    include_str!("../database/ddl/baseline/sqlite/0004_im_pc_client_local_store.sql");
const LOCAL_DATA_POLICY: &str = include_str!("../database/local-data-policy.yaml");

fn schema_inventory(sql: &str) -> Vec<String> {
    let connection = Connection::open_in_memory().expect("open contract database");
    connection.execute_batch(sql).expect("apply schema asset");
    let mut statement = connection
        .prepare(
            "SELECT type || ':' || name FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' ORDER BY 1",
        )
        .expect("prepare inventory");
    statement
        .query_map([], |row| row.get(0))
        .expect("query inventory")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect inventory")
}

#[test]
fn manifest_declares_an_isolated_sqlite_cache() {
    let manifest: Value = serde_json::from_str(MANIFEST).expect("parse database manifest");
    assert_eq!(manifest["databaseRole"], "client-local");
    assert_eq!(manifest["defaultEngine"], "sqlite");
    assert_eq!(manifest["tablePrefix"], "im_local_");
    assert_eq!(manifest["clientLocal"]["mode"], "cache");
    assert_eq!(
        manifest["clientLocal"]["authoritativeSource"],
        "sdkwork-im-app-api"
    );
    assert_eq!(manifest["engines"], serde_json::json!(["sqlite"]));
}

#[test]
fn migration_and_baseline_define_exactly_the_five_client_local_tables() {
    let migration_inventory = schema_inventory(MIGRATION);
    let baseline_inventory = schema_inventory(BASELINE);
    assert_eq!(migration_inventory, baseline_inventory);
    let tables = migration_inventory
        .iter()
        .filter_map(|item| item.strip_prefix("table:"))
        .collect::<Vec<_>>();
    assert_eq!(
        tables,
        vec![
            "im_local_cache_cursor",
            "im_local_conversation_cache",
            "im_local_installation",
            "im_local_message_cache",
            "im_local_pending_send",
        ]
    );
    for retired in [
        "offline_conversations",
        "offline_messages",
        "offline_pending_sends",
        "offline_sync_cursors",
    ] {
        assert!(!MIGRATION.contains(retired));
        assert!(!BASELINE.contains(retired));
    }
    assert!(!MIGRATION.contains("payload_json"));
    assert!(!MIGRATION.contains("cursor_json"));
}

#[test]
fn local_data_policy_is_cache_only_and_credential_free() {
    assert!(LOCAL_DATA_POLICY.contains("mode: cache"));
    assert!(LOCAL_DATA_POLICY.contains("key_store: OS credential vault through keyring"));
    assert!(LOCAL_DATA_POLICY.contains("logout: purge fetched conversation/message/cursor cache"));
    assert!(LOCAL_DATA_POLICY.contains("projection_rebuild: forbidden"));
}
