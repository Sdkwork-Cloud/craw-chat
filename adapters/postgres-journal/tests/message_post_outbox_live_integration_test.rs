//! Live PostgreSQL coverage for atomic message post and outbox persistence.
//!
//! Run with `SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p
//! im-adapters-postgres-journal --test message_post_outbox_live_integration_test --
//! --ignored --nocapture`.

use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Utc};
use im_adapters_postgres_journal::{
    PostgresDurableMessagePostWriter, PostgresJournalConfig, PostgresJournalPool,
    PostgresOutboxStore,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE,
    AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA, ContractError, OutboxEventRecord, OutboxPublishStatus,
    OutboxStore, StoredMessageRecord,
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
    journal_payload_json: serde_json::Value,
    journal_payload_hash: String,
    message_tenant_id: String,
    message_organization_id: String,
    message_conversation_id: String,
    message_id: i64,
    message_seq: i64,
    message_sender_principal_kind: String,
    message_sender_principal_id: String,
    message_sender_device_id: Option<String>,
    message_client_msg_id: Option<String>,
    message_type: String,
    message_payload_json: serde_json::Value,
    message_payload_hash: String,
    message_created_at: DateTime<Utc>,
    outbox_tenant_id: String,
    outbox_organization_id: String,
    outbox_id: String,
    outbox_aggregate_type: String,
    outbox_aggregate_id: String,
    outbox_event_id: String,
    outbox_event_type: String,
    outbox_payload_json: serde_json::Value,
    outbox_payload_hash: String,
    outbox_created_at: DateTime<Utc>,
}

const SAFE_REPLAY_CONFLICT: &str = "message post replay conflicts with existing durable state";

async fn bootstrap_live_database_pools() {
    static BOOTSTRAP_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    let _guard = BOOTSTRAP_LOCK.lock().await;
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
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
    let journal_event_id = format!("evt_journal_atomic_{scenario}_{suffix}");
    let outbox_event_id = format!("evt_outbox_atomic_{scenario}_{suffix}");
    let now = chrono::Utc::now().to_rfc3339();
    let message_payload = json!({
        "conversationId": conversation_id,
        "messageId": message_id.to_string(),
        "messageSeq": "1",
        "text": "atomic message post integration test"
    })
    .to_string();
    let journal_payload = json!({
        "eventId": journal_event_id,
        "conversationId": conversation_id,
        "messageId": message_id.to_string()
    })
    .to_string();
    let outbox_payload = json!({
        "eventId": outbox_event_id,
        "conversationId": conversation_id,
        "messageId": message_id.to_string()
    })
    .to_string();

    MessagePostFixture {
        envelope: CommitEnvelope {
            event_id: journal_event_id,
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
            payload: journal_payload,
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
            reactions: Vec::new(),
            pin: None,
        },
        outbox: OutboxEventRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            outbox_id: format!("outbox_atomic_{scenario}_{suffix}"),
            aggregate_type: "conversation".to_owned(),
            aggregate_id: conversation_id,
            event_id: outbox_event_id,
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

fn semantically_equivalent_json(payload: &str) -> String {
    let payload = serde_json::from_str::<serde_json::Value>(payload)
        .expect("fixture payload should be valid JSON");
    serde_json::to_string_pretty(&payload).expect("fixture payload should format as JSON")
}

fn same_instant_with_eight_hour_offset(value: &str) -> String {
    let instant = DateTime::parse_from_rfc3339(value).expect("fixture timestamp should be RFC3339");
    let offset = FixedOffset::east_opt(8 * 60 * 60).expect("eight-hour offset should be valid");
    instant.with_timezone(&offset).to_rfc3339()
}

fn assert_safe_replay_conflict<T: Debug>(result: Result<T, ContractError>, scenario: &str) {
    match result {
        Err(ContractError::Conflict(message)) => assert_eq!(
            message, SAFE_REPLAY_CONFLICT,
            "{scenario} must use the non-sensitive replay conflict"
        ),
        other => panic!("{scenario} must fail closed with Conflict: {other:?}"),
    }
}

async fn persisted_row_counts(
    pool: PostgresJournalPool,
    fixture: &MessagePostFixture,
) -> PersistedRowCounts {
    let tenant_id = fixture.message.tenant_id.clone();
    let organization_id = fixture.message.organization_id.clone();
    let journal_event_id = fixture.envelope.event_id.clone();
    let conversation_id = fixture.message.conversation_id.clone();
    let message_id = fixture.message.message_id;
    let outbox_id = fixture.outbox.outbox_id.clone();
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
        where tenant_id = $1 and organization_id = $2
          and conversation_id = $4 and message_id = $5),
    (select count(*) from im_outbox_events
        where tenant_id = $1 and organization_id = $2 and outbox_id = $6)
"#,
                &[
                    &tenant_id,
                    &organization_id,
                    &journal_event_id,
                    &conversation_id,
                    &message_id,
                    &outbox_id,
                ],
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
    let journal_event_id = fixture.envelope.event_id.clone();
    let tenant_id = fixture.message.tenant_id.clone();
    let organization_id = fixture.message.organization_id.clone();
    let conversation_id = fixture.message.conversation_id.clone();
    let message_id = fixture.message.message_id;
    let outbox_id = fixture.outbox.outbox_id.clone();
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
    journal.payload_json,
    journal.payload_hash,
    message.tenant_id,
    message.organization_id,
    message.conversation_id,
    message.message_id,
    message.message_seq,
    message.sender_principal_kind,
    message.sender_principal_id,
    message.sender_device_id,
    message.client_msg_id,
    message.message_type,
    message.payload_json,
    message.payload_hash,
    message.created_at,
    outbox.tenant_id,
    outbox.organization_id,
    outbox.outbox_id,
    outbox.aggregate_type,
    outbox.aggregate_id,
    outbox.event_id,
    outbox.event_type,
    outbox.payload_json,
    outbox.payload_hash,
    outbox.created_at
from im_commit_journal journal
cross join im_conversation_messages message
cross join im_outbox_events outbox
where journal.event_id = $1
  and journal.tenant_id = $2
  and journal.organization_id = $3
  and message.tenant_id = $2
  and message.organization_id = $3
  and message.conversation_id = $4
  and message.message_id = $5
  and outbox.tenant_id = $2
  and outbox.organization_id = $3
  and outbox.outbox_id = $6
"#,
                &[
                    &journal_event_id,
                    &tenant_id,
                    &organization_id,
                    &conversation_id,
                    &message_id,
                    &outbox_id,
                ],
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
            journal_payload_json: row.get(8),
            journal_payload_hash: row.get(9),
            message_tenant_id: row.get(10),
            message_organization_id: row.get(11),
            message_conversation_id: row.get(12),
            message_id: row.get(13),
            message_seq: row.get(14),
            message_sender_principal_kind: row.get(15),
            message_sender_principal_id: row.get(16),
            message_sender_device_id: row.get(17),
            message_client_msg_id: row.get(18),
            message_type: row.get(19),
            message_payload_json: row.get(20),
            message_payload_hash: row.get(21),
            message_created_at: row.get(22),
            outbox_tenant_id: row.get(23),
            outbox_organization_id: row.get(24),
            outbox_id: row.get(25),
            outbox_aggregate_type: row.get(26),
            outbox_aggregate_id: row.get(27),
            outbox_event_id: row.get(28),
            outbox_event_type: row.get(29),
            outbox_payload_json: row.get(30),
            outbox_payload_hash: row.get(31),
            outbox_created_at: row.get(32),
        }
    })
    .await
    .expect("immutable-row task should not panic")
}

async fn delete_fixture_message(pool: PostgresJournalPool, fixture: &MessagePostFixture) {
    let tenant_id = fixture.message.tenant_id.clone();
    let organization_id = fixture.message.organization_id.clone();
    let conversation_id = fixture.message.conversation_id.clone();
    let message_id = fixture.message.message_id;
    tokio::task::spawn_blocking(move || {
        let mut client = pool
            .get()
            .expect("message-deletion connection should be available");
        let deleted = client
            .execute(
                r#"
delete from im_conversation_messages
where tenant_id = $1 and organization_id = $2
  and conversation_id = $3 and message_id = $4
"#,
                &[&tenant_id, &organization_id, &conversation_id, &message_id],
            )
            .expect("fixture message should be deletable");
        assert_eq!(deleted, 1, "exactly one fixture message should be deleted");
    })
    .await
    .expect("message-deletion task should not panic");
}

async fn delete_fixture_outbox(pool: PostgresJournalPool, fixture: &MessagePostFixture) {
    let tenant_id = fixture.outbox.tenant_id.clone();
    let organization_id = fixture.outbox.organization_id.clone();
    let outbox_id = fixture.outbox.outbox_id.clone();
    tokio::task::spawn_blocking(move || {
        let mut client = pool
            .get()
            .expect("outbox-deletion connection should be available");
        let deleted = client
            .execute(
                r#"
delete from im_outbox_events
where tenant_id = $1 and organization_id = $2 and outbox_id = $3
"#,
                &[&tenant_id, &organization_id, &outbox_id],
            )
            .expect("fixture outbox row should be deletable");
        assert_eq!(
            deleted, 1,
            "exactly one fixture outbox row should be deleted"
        );
    })
    .await
    .expect("outbox-deletion task should not panic");
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
    bootstrap_live_database_pools().await;
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
    let missing_message_fixture = fixture(
        tenant_id.as_str(),
        "missing_message",
        suffix.as_str(),
        base_message_id + 3,
    );
    let missing_outbox_fixture = fixture(
        tenant_id.as_str(),
        "missing_outbox",
        suffix.as_str(),
        base_message_id + 4,
    );
    let omitted_outbox_replay_fixture = fixture(
        tenant_id.as_str(),
        "omitted_outbox_replay",
        suffix.as_str(),
        base_message_id + 5,
    );
    let added_outbox_replay_fixture = fixture(
        tenant_id.as_str(),
        "added_outbox_replay",
        suffix.as_str(),
        base_message_id + 6,
    );

    for fixture in [
        &outbox_id_conflict_fixture,
        &event_id_conflict_fixture,
        &commit_fixture,
        &missing_message_fixture,
        &missing_outbox_fixture,
        &omitted_outbox_replay_fixture,
        &added_outbox_replay_fixture,
    ] {
        assert_ne!(
            fixture.envelope.event_id, fixture.outbox.event_id,
            "journal and outbox event identities must remain distinct in production behavior"
        );
    }

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
            outbox: 0,
        },
        "the pre-existing event-id conflict must not use the requested outbox identity"
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

    let mut semantically_equivalent_message = commit_fixture.message.clone();
    semantically_equivalent_message.payload_json =
        semantically_equivalent_json(commit_fixture.message.payload_json.as_str());
    semantically_equivalent_message.created_at =
        same_instant_with_eight_hour_offset(commit_fixture.message.created_at.as_str());
    semantically_equivalent_message.updated_at = "2035-01-01T00:00:00Z".to_owned();
    semantically_equivalent_message.deleted_at = Some("2035-01-02T00:00:00Z".to_owned());
    semantically_equivalent_message.retention_until = Some("2035-02-01T00:00:00Z".to_owned());
    let mut semantically_equivalent_outbox = commit_fixture.outbox.clone();
    semantically_equivalent_outbox.payload_json =
        semantically_equivalent_json(commit_fixture.outbox.payload_json.as_str());
    semantically_equivalent_outbox.created_at =
        same_instant_with_eight_hour_offset(commit_fixture.outbox.created_at.as_str());
    semantically_equivalent_outbox.publish_status = OutboxPublishStatus::Published;
    semantically_equivalent_outbox.attempt_count = 7;
    semantically_equivalent_outbox.available_at = "2035-01-01T00:00:00Z".to_owned();
    semantically_equivalent_outbox.published_at = Some("2035-01-01T00:00:01Z".to_owned());
    semantically_equivalent_outbox.updated_at = "2035-01-01T00:00:02Z".to_owned();
    let semantic_replay_position = writer.persist_message_post(
        commit_fixture.envelope.clone(),
        semantically_equivalent_message,
        Some(semantically_equivalent_outbox),
    );
    let semantic_replay_counts = persisted_row_counts(pool.clone(), &commit_fixture).await;

    let immutable_rows_before_conflict =
        persisted_immutable_rows(pool.clone(), &commit_fixture).await;
    let mut conflicting_message = commit_fixture.message.clone();
    conflicting_message.payload_json = json!({
        "conversationId": conflicting_message.conversation_id,
        "messageId": conflicting_message.message_id.to_string(),
        "messageSeq": conflicting_message.message_seq.to_string(),
        "text": "different message payload for the same event id"
    })
    .to_string();
    assert_ne!(
        serde_json::from_str::<serde_json::Value>(conflicting_message.payload_json.as_str())
            .expect("conflicting message payload should be JSON"),
        serde_json::from_str::<serde_json::Value>(commit_fixture.message.payload_json.as_str())
            .expect("original message payload should be JSON")
    );
    let message_conflicting_replay_result = writer.persist_message_post(
        commit_fixture.envelope.clone(),
        conflicting_message,
        Some(commit_fixture.outbox.clone()),
    );
    let message_conflicting_replay_counts =
        persisted_row_counts(pool.clone(), &commit_fixture).await;
    let immutable_rows_after_message_conflict =
        persisted_immutable_rows(pool.clone(), &commit_fixture).await;

    let mut conflicting_outbox = commit_fixture.outbox.clone();
    conflicting_outbox.payload_json = json!({
        "eventId": conflicting_outbox.event_id,
        "conversationId": conflicting_outbox.aggregate_id,
        "messageId": commit_fixture.message.message_id.to_string(),
        "replayVariant": "different-outbox-payload"
    })
    .to_string();
    assert_ne!(
        serde_json::from_str::<serde_json::Value>(conflicting_outbox.payload_json.as_str())
            .expect("conflicting outbox payload should be JSON"),
        serde_json::from_str::<serde_json::Value>(commit_fixture.outbox.payload_json.as_str())
            .expect("original outbox payload should be JSON")
    );
    let outbox_conflicting_replay_result = writer.persist_message_post(
        commit_fixture.envelope.clone(),
        commit_fixture.message.clone(),
        Some(conflicting_outbox),
    );
    let outbox_conflicting_replay_counts =
        persisted_row_counts(pool.clone(), &commit_fixture).await;
    let immutable_rows_after_outbox_conflict =
        persisted_immutable_rows(pool.clone(), &commit_fixture).await;

    writer
        .persist_message_post(
            missing_message_fixture.envelope.clone(),
            missing_message_fixture.message.clone(),
            Some(missing_message_fixture.outbox.clone()),
        )
        .expect("missing-message fixture should initially persist");
    delete_fixture_message(pool.clone(), &missing_message_fixture).await;
    let missing_message_replay_result = writer.persist_message_post(
        missing_message_fixture.envelope.clone(),
        missing_message_fixture.message.clone(),
        Some(missing_message_fixture.outbox.clone()),
    );
    let missing_message_replay_counts =
        persisted_row_counts(pool.clone(), &missing_message_fixture).await;

    writer
        .persist_message_post(
            missing_outbox_fixture.envelope.clone(),
            missing_outbox_fixture.message.clone(),
            Some(missing_outbox_fixture.outbox.clone()),
        )
        .expect("missing-outbox fixture should initially persist");
    delete_fixture_outbox(pool.clone(), &missing_outbox_fixture).await;
    let missing_outbox_replay_result = writer.persist_message_post(
        missing_outbox_fixture.envelope.clone(),
        missing_outbox_fixture.message.clone(),
        Some(missing_outbox_fixture.outbox.clone()),
    );
    let missing_outbox_replay_counts =
        persisted_row_counts(pool.clone(), &missing_outbox_fixture).await;

    writer
        .persist_message_post(
            omitted_outbox_replay_fixture.envelope.clone(),
            omitted_outbox_replay_fixture.message.clone(),
            Some(omitted_outbox_replay_fixture.outbox.clone()),
        )
        .expect("omitted-outbox replay fixture should initially persist with outbox");
    let omitted_outbox_replay_result = writer.persist_message_post(
        omitted_outbox_replay_fixture.envelope.clone(),
        omitted_outbox_replay_fixture.message.clone(),
        None,
    );
    let omitted_outbox_replay_counts =
        persisted_row_counts(pool.clone(), &omitted_outbox_replay_fixture).await;

    let no_outbox_position = writer
        .persist_message_post(
            added_outbox_replay_fixture.envelope.clone(),
            added_outbox_replay_fixture.message.clone(),
            None,
        )
        .expect("no-outbox fixture should initially persist");
    let no_outbox_replay_position = writer
        .persist_message_post(
            added_outbox_replay_fixture.envelope.clone(),
            added_outbox_replay_fixture.message.clone(),
            None,
        )
        .expect("matching no-outbox replay should remain idempotent");
    let added_outbox_replay_result = writer.persist_message_post(
        added_outbox_replay_fixture.envelope.clone(),
        added_outbox_replay_fixture.message.clone(),
        Some(added_outbox_replay_fixture.outbox.clone()),
    );
    let added_outbox_replay_counts =
        persisted_row_counts(pool.clone(), &added_outbox_replay_fixture).await;

    cleanup_tenant(pool, tenant_id).await;

    assert!(
        matches!(outbox_id_conflict_result, Err(ContractError::Conflict(_)))
            && outbox_id_conflict_counts
                == PersistedRowCounts {
                    journal: 0,
                    message: 0,
                    outbox: 1,
                }
            && matches!(event_id_conflict_result, Err(ContractError::Conflict(_)))
            && event_id_conflict_counts
                == PersistedRowCounts {
                    journal: 0,
                    message: 0,
                    outbox: 0,
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
    assert_eq!(
        semantic_replay_position.expect(
            "semantic JSON equality, normalized instants, and mutable lifecycle fields must replay",
        ),
        commit_position
    );
    assert_eq!(
        semantic_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "semantic replay must not duplicate durable rows"
    );
    assert_safe_replay_conflict(
        message_conflicting_replay_result,
        "message fingerprint mismatch",
    );
    assert_eq!(
        message_conflicting_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "message-conflicting replay must not add durable rows"
    );
    assert_eq!(
        immutable_rows_after_message_conflict, immutable_rows_before_conflict,
        "message-conflicting replay must leave all original immutable rows unchanged"
    );
    assert_safe_replay_conflict(
        outbox_conflicting_replay_result,
        "outbox fingerprint mismatch",
    );
    assert_eq!(
        outbox_conflicting_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "outbox-conflicting replay must not add durable rows"
    );
    assert_eq!(
        immutable_rows_after_outbox_conflict, immutable_rows_before_conflict,
        "outbox-conflicting replay must leave all original immutable rows unchanged"
    );
    assert_safe_replay_conflict(missing_message_replay_result, "missing durable message");
    assert_eq!(
        missing_message_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 0,
            outbox: 1,
        },
        "missing-message replay must not repair or duplicate durable rows"
    );
    assert_safe_replay_conflict(missing_outbox_replay_result, "missing durable outbox row");
    assert_eq!(
        missing_outbox_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 0,
        },
        "missing-outbox replay must not repair or duplicate durable rows"
    );
    assert_safe_replay_conflict(
        omitted_outbox_replay_result,
        "replay omitted an originally persisted outbox row",
    );
    assert_eq!(
        omitted_outbox_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 1,
        },
        "omitted-outbox replay must preserve the original durable rows"
    );
    assert_eq!(
        no_outbox_replay_position, no_outbox_position,
        "matching no-outbox replay must return the original commit position"
    );
    assert_safe_replay_conflict(
        added_outbox_replay_result,
        "replay added an outbox row that was absent from the original write",
    );
    assert_eq!(
        added_outbox_replay_counts,
        PersistedRowCounts {
            journal: 1,
            message: 1,
            outbox: 0,
        },
        "added-outbox replay must not mutate the original no-outbox durable state"
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn message_post_dispatch_event_and_two_outboxes_are_atomic_and_idempotent() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    bootstrap_live_database_pools().await;
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let writer = PostgresDurableMessagePostWriter::new(pool.clone(), Arc::from(""));
    let suffix = test_suffix();
    let tenant_id = format!("message-agent-dispatch-test-{suffix}");
    let message_id = suffix
        .parse::<u128>()
        .expect("test suffix should be numeric")
        .checked_div(1_000)
        .and_then(|value| i64::try_from(value).ok())
        .expect("test message id should fit in i64");
    let fixture = fixture(
        tenant_id.as_str(),
        "agent_dispatch_atomic",
        suffix.as_str(),
        message_id,
    );
    let mut dispatch_envelope = fixture.envelope.clone();
    dispatch_envelope.event_id = format!("evt_agent_dispatch_{suffix}");
    dispatch_envelope.event_type = AGENT_MENTION_DISPATCH_EVENT_TYPE.into();
    dispatch_envelope.ordering_seq = 1;
    dispatch_envelope.payload_schema = Some(AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA.into());
    dispatch_envelope.causation_id = Some(fixture.envelope.event_id.clone());
    dispatch_envelope.payload = json!({
        "schemaVersion": 1,
        "tenantId": tenant_id,
        "organizationId": "0",
        "conversationId": fixture.message.conversation_id,
        "messageId": fixture.message.message_id.to_string(),
        "messageSeq": fixture.message.message_seq,
        "causationEventId": fixture.envelope.event_id,
        "senderPrincipalId": "1",
        "senderPrincipalKind": "user",
        "assignmentGeneration": 2,
        "targets": [{
            "dispatchId": "amd_live_dispatch",
            "agentId": "agent.im.writer",
            "revisionId": "revision.im.writer.1"
        }],
        "body": {"summary": "hello", "parts": [], "renderHints": {}},
        "requestedAt": fixture.message.created_at
    })
    .to_string();
    let mut dispatch_outbox = fixture.outbox.clone();
    dispatch_outbox.outbox_id = format!("outbox_agent_dispatch_{suffix}");
    dispatch_outbox.aggregate_type = AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE.into();
    dispatch_outbox.event_id = format!("agent-dispatch:evt_agent_dispatch_{suffix}");
    dispatch_outbox.event_type = AGENT_MENTION_DISPATCH_EVENT_TYPE.into();
    dispatch_outbox.payload_json = dispatch_envelope.payload.clone();
    dispatch_outbox.payload_hash =
        sdkwork_utils_rust::sha256_hash(dispatch_outbox.payload_json.as_bytes());

    let outboxes = vec![fixture.outbox.clone(), dispatch_outbox.clone()];
    let envelopes = vec![fixture.envelope.clone(), dispatch_envelope.clone()];
    let positions = writer
        .persist_message_post_batch(envelopes.clone(), fixture.message.clone(), outboxes.clone())
        .expect("message, dispatch event, and both outboxes should commit atomically");
    assert_eq!(positions.len(), 2);
    let replay_positions = writer
        .persist_message_post_batch(envelopes, fixture.message.clone(), outboxes)
        .expect("the exact batch should replay idempotently");
    assert_eq!(replay_positions, positions);

    let tenant_for_query = tenant_id.clone();
    let conversation_id = fixture.message.conversation_id.clone();
    let message_id_for_query = fixture.message.message_id;
    let dispatch_event_id = dispatch_envelope.event_id.clone();
    let dispatch_outbox_id = dispatch_outbox.outbox_id.clone();
    let count_pool = pool.clone();
    let counts = tokio::task::spawn_blocking(move || {
        let mut client = count_pool
            .get()
            .expect("count connection should be available");
        let row = client
            .query_one(
                r#"
select
  (select count(*) from im_commit_journal where tenant_id = $1 and organization_id = '0' and event_id in ($2, $3)),
  (select count(*) from im_conversation_messages where tenant_id = $1 and organization_id = '0' and conversation_id = $4 and message_id = $5),
  (select count(*) from im_outbox_events where tenant_id = $1 and organization_id = '0' and outbox_id in ($6, $7)),
  (select count(*) from im_commit_journal where tenant_id = $1 and organization_id = '0' and event_id = $3)
"#,
                &[
                    &tenant_for_query,
                    &fixture.envelope.event_id,
                    &dispatch_event_id,
                    &conversation_id,
                    &message_id_for_query,
                    &fixture.outbox.outbox_id,
                    &dispatch_outbox_id,
                ],
            )
            .expect("batch rows should be countable");
        (row.get::<_, i64>(0), row.get::<_, i64>(1), row.get::<_, i64>(2), row.get::<_, i64>(3))
    })
    .await
    .expect("count task should not panic");
    assert_eq!(counts, (2, 1, 2, 1));

    cleanup_tenant(pool, tenant_id).await;
}
