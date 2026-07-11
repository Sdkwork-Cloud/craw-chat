//! Live PostgreSQL coverage for backward keyset message history pagination.
//!
//! Run with `SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p
//! im-adapters-postgres-journal --test message_history_live_integration_test --
//! --ignored --nocapture`.

use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresMessageStore};
use im_platform_contracts::{MessageStore, StoredMessageRecord};

fn test_suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after the Unix epoch")
        .as_nanos()
        .to_string()
}

fn message(tenant_id: &str, conversation_id: &str, message_seq: u64) -> StoredMessageRecord {
    let now = chrono::Utc::now().to_rfc3339();
    StoredMessageRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: "0".to_owned(),
        conversation_id: conversation_id.to_owned(),
        message_id: message_seq as i64,
        message_seq,
        sender_principal_kind: "user".to_owned(),
        sender_principal_id: "1".to_owned(),
        sender_device_id: None,
        client_msg_id: Some(format!("client_{message_seq}")),
        message_type: "standard".to_owned(),
        payload_json: "{}".to_owned(),
        payload_hash: format!("hash_{message_seq}"),
        created_at: now.clone(),
        updated_at: now,
        deleted_at: None,
        retention_until: None,
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn backward_history_is_stable_when_new_messages_arrive_between_pages() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let store = PostgresMessageStore::from_pool(pool.clone());
    let suffix = test_suffix();
    let tenant_id = format!("message-history-test-{suffix}");
    let conversation_id = format!("c_message_history_{suffix}");

    for message_seq in 1..=10 {
        store
            .insert_message(message(&tenant_id, &conversation_id, message_seq))
            .expect("test message should insert");
    }

    let latest = store
        .read_history_window(&tenant_id, "0", &conversation_id, None, 3)
        .expect("latest history page should load");
    store
        .insert_message(message(&tenant_id, &conversation_id, 11))
        .expect("concurrent new message should insert");
    let older = store
        .read_history_window(&tenant_id, "0", &conversation_id, latest.next_before_seq, 3)
        .expect("older history page should load");

    let cleanup_pool = pool.clone();
    let cleanup_tenant_id = tenant_id.clone();
    tokio::task::spawn_blocking(move || {
        cleanup_pool
            .get()
            .expect("cleanup connection should be available")
            .execute(
                "delete from im_conversation_messages where tenant_id = $1 and organization_id = $2",
                &[&cleanup_tenant_id, &"0"],
            )
            .expect("test message rows should be cleaned up");
    })
    .await
    .expect("cleanup task should not panic");

    assert_eq!(
        latest
            .items
            .iter()
            .map(|item| item.message_seq)
            .collect::<Vec<_>>(),
        vec![8, 9, 10]
    );
    assert_eq!(latest.next_before_seq, Some(8));
    assert!(latest.has_more);
    assert_eq!(
        older
            .items
            .iter()
            .map(|item| item.message_seq)
            .collect::<Vec<_>>(),
        vec![5, 6, 7]
    );
    assert_eq!(older.next_before_seq, Some(5));
    assert!(older.has_more);
}
