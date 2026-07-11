//! Live PostgreSQL coverage for atomic message post and outbox persistence.
//!
//! Run with `SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p
//! im-adapters-postgres-journal --test message_post_outbox_live_integration_test --
//! --ignored --nocapture`.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{
    PostgresDurableMessagePostWriter, PostgresJournalConfig, PostgresJournalPool,
    PostgresOutboxStore,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    ContractError, OutboxEventRecord, OutboxPublishStatus, OutboxStore, StoredMessageRecord,
};
use serde_json::json;

struct MessagePostFixture {
    envelope: CommitEnvelope,
    message: StoredMessageRecord,
    outbox: OutboxEventRecord,
}

#[derive(Debug, PartialEq, Eq)]
struct PersistedRowCounts {
    journal: i64,
    message: i64,
    outbox: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct PersistedImmutableRows {
    journal_partition: String,
    journal_offset: i64,
    journal_tenant_id: String,
    journal_organization_id: String,
    journal_aggregate_type: String,
    journal_aggregate_id: String,
    journal_aggregate_seq: i64,
    journal_event_type: String,
    journal_payload_hash: String,
    message_id: i64,
    message_payload_hash: String,
    outbox_id: String,
    outbox_payload_hash: String,
}

fn test_suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after the Unix epoch")
        .as_nanos()
        .to_string()
}

fn fixture(tenant_id: &str, scenario: &str, suffix: &str, message_id: i64) -> MessagePostFixture {
    let organization_id = "0";
    let conversation_id = format!("c_atomic_{scenario}_{suffix}");
    let event_id = format!("evt_atomic_{scenario}_{suffix}");
    let now = chrono::Utc::now().to_rfc3339();
    let message_payload = json!({
        "conversationId": conversation_id,
        "messageId": message_id.to_string(),
        "messageSeq": "1",
        "text": "atomic message post integration test"
    })
    .to_string();
    let outbox_payload = json!({
        "eventId": event_id,
        "conversationId": conversation_id,
        "messageId": message_id.to_string()
    })
    .to_string();

    MessagePostFixture {
        envelope: CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            event_type: "message.posted".to_owned(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: conversation_id.clone(),
            scope_type: "conversation".to_owned(),
            scope_id: conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id.as_str()),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: Some(format!("idem_atomic_{scenario}_{suffix}")),
            actor: EventActor {
                actor_id: "1".to_owned(),
                actor_kind: "user".to_owned(),
                actor_session_id: None,
            },
            occurred_at: now.clone(),
            committed_at: now.clone(),
            payload_schema: Some("message.posted.v1".to_owned()),
            payload: outbox_payload.clone(),
            retention_class: "standard".to_owned(),
            audit_class: "default".to_owned(),
        },
        message: StoredMessageRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.clone(),
            message_id,
            message_seq: 1,
            sender_principal_kind: "user".to_owned(),
            sender_principal_id: "1".to_owned(),
            sender_device_id: Some("device-live-test".to_owned()),
            client_msg_id: Some(format!("client_atomic_{scenario}_{suffix}")),
            message_type: "text".to_owned(),
            payload_hash: sdkwork_utils_rust::sha256_hash(message_payload.as_bytes()),
            payload_json: message_payload,
            created_at: now.clone(),
            updated_at: now.clone(),
            deleted_at: None,
            retention_until: None,
        },
        outbox: OutboxEventRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            outbox_id: format!("outbox_atomic_{scenario}_{suffix}"),
            aggregate_type: "conversation".to_owned(),
            aggregate_id: conversation_id,
            event_id,
            event_type: "message.posted".to_owned(),
            payload_hash: sdkwork_utils_rust::sha256_hash(outbox_payload.as_bytes()),
            payload_json: outbox_payload,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        },
    }
}

async fn persisted_row_counts(
    pool: PostgresJournalPool,
    fixture: &MessagePostFixture,
) -> PersistedRowCounts {
    let tenant_id = fixture.message.tenant_id.clone();
    let organization_id = fixture.message.organization_id.clone();
    let event_id = fixture.envelope.event_id.clone();
    let message_id = fixture.message.message_id;
    tokio::task::spawn_blocking(move || {
        let mut client = pool
            .get()
            .expect("row-count connection should be available");
        let row = client
            .query_one(
                r#"
select
    (select count(*) from im_commit_journal
        where tenant_id = $1 and organization_id = $2 and event_id = $3),
    (select count(*) from im_conversation_messages
        where tenant_id = $1 and organization_id = $2 and message_id = $4),
    (select count(*) from im_outbox_events
        where tenant_id = $1 and organization_id = $2 and event_id = $3)
"#,
                &[&tenant_id, &organization_id, &event_id, &message_id],
            )
            .expect("atomic message post rows should be countable");
        PersistedRowCounts {
            journal: row.get(0),
            message: row.get(1),
            outbox: row.get(2),
        }
    })
    .await
    .expect("row-count task should not panic")
}

async fn persisted_immutable_rows(
    pool: PostgresJournalPool,
    fixture: &MessagePostFixture,
) -> PersistedImmutableRows {
    let event_id = fixture.envelope.event_id.clone();
    let message_id = fixture.message.message_id;
    tokio::task::spawn_blocking(move || {
        let mut client = pool
            .get()
            .expect("immutable-row connection should be available");
        let row = client
            .query_one(
                r#"
select
    journal.partition_key,
    journal.commit_offset,
    journal.tenant_id,
    journal.organization_id,
    journal.aggregate_type,
    journal.aggregate_id,
    journal.aggregate_seq,
    journal.event_type,
    journal.payload_hash,
    message.message_id,
    message.payload_hash,
    outbox.outbox_id,
    outbox.payload_hash
from im_commit_journal journal
join im_conversation_messages message
  on message.tenant_id = journal.tenant_id
 and message.organization_id = journal.organization_id
 and message.message_id = $2
join im_outbox_events outbox
  on outbox.tenant_id = journal.tenant_id
 and outbox.organization_id = journal.organization_id
 and outbox.event_id = journal.event_id
where journal.event_id = $1
"#,
                &[&event_id, &message_id],
            )
            .expect("committed immutable rows should be readable");
        PersistedImmutableRows {
            journal_partition: row.get(0),
            journal_offset: row.get(1),
            journal_tenant_id: row.get(2),
            journal_organization_id: row.get(3),
            journal_aggregate_type: row.get(4),
            journal_aggregate_id: row.get(5),
            journal_aggregate_seq: row.get(6),
            journal_event_type: row.get(7),
            journal_payload_hash: row.get(8),
            message_id: row.get(9),
            message_payload_hash: row.get(10),
            outbox_id: row.get(11),
            outbox_payload_hash: row.get(12),
        }
    })
    .await
    .expect("immutable-row task should not panic")
}

async fn cleanup_tenant(pool: PostgresJournalPool, tenant_id: String) {
    tokio::task::spawn_blocking(move || {
        let mut client = pool.get().expect("cleanup connection should be available");
        let mut transaction = client
            .transaction()
            .expect("cleanup transaction should begin");
        transaction
            .execute(
                "delete from im_outbox_events where tenant_id = $1 and organization_id = $2",
                &[&tenant_id, &"0"],
            )
            .expect("test outbox rows should be cleaned up");
        transaction
            .execute(
                "delete from im_conversation_messages where tenant_id = $1 and organization_id = $2",
                &[&tenant_id, &"0"],
            )
            .expect("test message rows should be cleaned up");
        transaction
            .execute(
                "delete from im_commit_journal where tenant_id = $1 and organization_id = $2",
                &[&tenant_id, &"0"],
            )
            .expect("test journal rows should be cleaned up");
        transaction
            .commit()
            .expect("cleanup transaction should commit");
    })
    .await
    .expect("cleanup task should not panic");
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn message_post_and_outbox_are_committed_or_rolled_back_together() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let writer = PostgresDurableMessagePostWriter::new(pool.clone(), Arc::from(""));
    let outbox_store = PostgresOutboxStore::from_pool(pool.clone());
    let suffix = test_suffix();
    let tenant_id = format!("message-post-outbox-test-{suffix}");
    let base_message_id = suffix
        .parse::<u128>()
        .expect("test suffix should be numeric")
        .checked_div(1_000)
        .and_then(|value| i64::try_from(value).ok())
        .expect("test message id should fit in i64");
    let outbox_id_conflict_fixture = fixture(
        tenant_id.as_str(),
        "outbox_id_conflict",
        suffix.as_str(),
        base_message_id,
    );
    let event_id_conflict_fixture = fixture(
        tenant_id.as_str(),
        "event_id_conflict",
        suffix.as_str(),
        base_message_id + 1,
    );
    let commit_fixture = fixture(
        tenant_id.as_str(),
        "commit",
        suffix.as_str(),
        base_message_id + 2,
    );

    let mut conflicting_outbox_id_row = outbox_id_conflict_fixture.outbox.clone();
    conflicting_outbox_id_row.event_id = format!("evt_existing_outbox_id_{suffix}");
    outbox_store
        .enqueue(conflicting_outbox_id_row)
        .expect("outbox-id conflict fixture should enqueue");

    let mut conflicting_event_id_row = event_id_conflict_fixture.outbox.clone();
    conflicting_event_id_row.outbox_id = format!("outbox_existing_event_id_{suffix}");
    conflicting_event_id_row.payload_json = json!({
        "eventId": conflicting_event_id_row.event_id,
        "source": "pre-existing-mismatched-outbox-row"
    })
    .to_string();
    conflicting_event_id_row.payload_hash =
        sdkwork_utils_rust::sha256_hash(conflicting_event_id_row.payload_json.as_bytes());
    assert_ne!(
        conflicting_event_id_row.outbox_id,
        event_id_conflict_fixture.outbox.outbox_id
    );
    assert_ne!(
        conflicting_event_id_row.payload_json,
        event_id_conflict_fixture.outbox.payload_json
    );
    outbox_store
        .enqueue(conflicting_event_id_row)
        .expect("event-id conflict fixture should enqueue");

    let event_id_conflict_before_counts =
        persisted_row_counts(pool.clone(), &event_id_conflict_fixture).await;
    assert_eq!(
        event_id_conflict_before_counts,
        PersistedRowCounts {
            journal: 0,
            message: 0,
            outbox: 1,
        },
        "the pre-existing mismatched event must not have a matching journal or message row"
    );

    let outbox_id_conflict_result = writer.persist_message_post(
        outbox_id_conflict_fixture.envelope.clone(),
        outbox_id_conflict_fixture.message.clone(),
        Some(outbox_id_conflict_fixture.outbox.clone()),
    );
    let outbox_id_conflict_counts =
        persisted_row_counts(pool.clone(), &outbox_id_conflict_fixture).await;

    let event_id_conflict_result = writer.persist_message_post(
        event_id_conflict_fixture.envelope.clone(),
        event_id_conflict_fixture.message.clone(),
        Some(event_id_conflict_fixture.outbox.clone()),
    );
    let event_id_conflict_counts =
        persisted_row_counts(pool.clone(), &event_id_conflict_fixture).await;

    let commit_position = writer.persist_message_post(
        commit_fixture.envelope.clone(),
        commit_fixture.message.clone(),
        Some(commit_fixture.outbox.clone()),
    );
    let commit_counts = persisted_row_counts(pool.clone(), &commit_fixture).await;
    let replay_position = writer.persist_message_post(
        commit_fixture.envelope.clone(),
        commit_fixture.message.clone(),
        Some(commit_fixture.outbox.clone()),
    );
    let replay_counts = persisted_row_counts(pool.clone(), &commit_fixture).await;

    let immutable_rows_before_conflict =
        persisted_immutable_rows(pool.clone(), &commit_fixture).await;
    let mut conflicting_envelope = commit_fixture.envelope.clone();
    conflicting_envelope.payload = json!({
        "eventId": conflicting_envelope.event_id,
        "conversationId": conflicting_envelope.aggregate_id,
        "messageId": commit_fixture.message.message_id.to_string(),
        "replayVariant": "different-journal-payload"
    })
    .to_string();
    let mut conflicting_message = commit_fixture.message.clone();
    conflicting_message.payload_json = json!({
        "conversationId": conflicting_message.conversation_id,
        "messageId": conflicting_message.message_id.to_string(),
        "messageSeq": conflicting_message.message_seq.to_string(),
        "text": "different message payload for the same event id"
    })
    .to_string();
    conflicting_message.payload_hash =
        sdkwork_utils_rust::sha256_hash(conflicting_message.payload_json.as_bytes());
    let mut conflicting_outbox = commit_fixture.outbox.clone();
    conflicting_outbox.payload_json = json!({
        "eventId": conflicting_outbox.event_id,
        "conversationId": conflicting_outbox.aggregate_id,
        "messageId": commit_fixture.message.message_id.to_string(),
        "replayVariant": "different-outbox-payload"
    })
    .to_string();
    conflicting_outbox.payload_hash =
        sdkwork_utils_rust::sha256_hash(conflicting_outbox.payload_json.as_bytes());
    let conflicting_replay_result = writer.persist_message_post(
        conflicting_envelope,
        conflicting_message,
        Some(conflicting_outbox),
    );
    let conflicting_replay_counts = persisted_row_counts(pool.clone(), &commit_fixture).await;
    let immutable_rows_after_conflict =
        persisted_immutable_rows(pool.clone(), &commit_fixture).await;

    cleanup_tenant(pool, tenant_id).await;

    assert!(
        matches!(outbox_id_conflict_result, Err(ContractError::Conflict(_)))
            && outbox_id_conflict_counts
                == PersistedRowCounts {
                    journal: 0,
                    message: 0,
                    outbox: 0,
                }
            && matches!(event_id_conflict_result, Err(ContractError::Conflict(_)))
            && event_id_conflict_counts
                == PersistedRowCounts {
                    journal: 0,
                    message: 0,
                    outbox: 1,
                },
        "outbox unique conflicts must be Conflict and roll back journal/message rows: \
         outbox_id_result={outbox_id_conflict_result:?}, \
         outbox_id_counts={outbox_id_conflict_counts:?}, \
         event_id_result={event_id_conflict_result:?}, \
         event_id_counts={event_id_conflict_counts:?}"
    );
    let commit_position =
        commit_position.expect("valid journal, message, and outbox rows should commit atomically");
    assert_eq!(
        commit_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        }
    );
    assert_eq!(
        replay_position.expect("the same journal event should replay idempotently"),
        commit_position
    );
    assert_eq!(
        replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "journal replay must not duplicate message or outbox rows"
    );
    assert!(
        matches!(conflicting_replay_result, Err(ContractError::Conflict(_))),
        "the same event_id with different immutable payloads must conflict: {conflicting_replay_result:?}"
    );
    assert_eq!(
        conflicting_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "conflicting replay must not add journal, message, or outbox rows"
    );
    assert_eq!(
        immutable_rows_after_conflict, immutable_rows_before_conflict,
        "conflicting replay must leave the original immutable rows unchanged"
    );
}
