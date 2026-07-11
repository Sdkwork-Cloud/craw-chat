//! Live PostgreSQL append repro for journal binding issues.
//! Run with:
//! SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p im-adapters-postgres-journal --test append_live_integration_test -- --ignored --nocapture

use im_adapters_postgres_journal::PostgresJournalConfig;
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{CommitJournal, ContractError};
use serde_json::json;

static LIVE_POSTGRES_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn sample_envelope(event_id: &str, conversation_id: &str) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        event_type: "conversation.created".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key("100001", conversation_id),
        ordering_seq: 0,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: "1".into(),
            actor_kind: "user".into(),
            actor_session_id: None,
        },
        occurred_at: "2026-06-25T10:00:00.000Z".into(),
        committed_at: "2026-06-25T10:00:00.000Z".into(),
        payload_schema: Some("conversation.created.v1".into()),
        payload: json!({
            "conversationId": conversation_id,
            "conversationType": "agent_dialog",
            "agentDialog": { "agentId": "agent.demo" }
        })
        .to_string(),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn append_agent_dialog_envelope_live() {
    let _test_guard = LIVE_POSTGRES_TEST_LOCK.lock().await;
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let journal = PostgresJournalConfig::new(database_url)
        .connect()
        .expect("postgres journal should connect");

    let conversation_id = format!("c_agent_dialog_live_{}", uuid_like_suffix());
    let event_id = format!("evt_{conversation_id}_created");
    let envelope = sample_envelope(event_id.as_str(), conversation_id.as_str());

    journal
        .append(envelope)
        .expect("append should succeed against live postgres");

    let pool = journal.pool().clone();
    tokio::task::spawn_blocking(move || {
        pool.get()
            .expect("cleanup connection should be available")
            .execute(
                "delete from im_commit_journal where event_id = $1",
                &[&event_id],
            )
            .expect("live append test row should be cleaned up");
    })
    .await
    .expect("cleanup task should not panic");
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn append_and_batch_replays_validate_the_immutable_event_fingerprint() {
    let _test_guard = LIVE_POSTGRES_TEST_LOCK.lock().await;
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let journal = PostgresJournalConfig::new(database_url)
        .connect()
        .expect("postgres journal should connect");
    let suffix = uuid_like_suffix();

    let append_conversation_id = format!("c_append_replay_{suffix}");
    let append_event_id = format!("evt_append_replay_{suffix}");
    let append_envelope =
        sample_envelope(append_event_id.as_str(), append_conversation_id.as_str());
    let append_position = journal
        .append(append_envelope.clone())
        .expect("initial append should commit");
    let exact_append_replay = journal.append(append_envelope.clone());
    let mut conflicting_append_replay = append_envelope.clone();
    conflicting_append_replay.payload = json!({
        "conversationId": append_conversation_id,
        "replayVariant": "different-payload"
    })
    .to_string();
    let conflicting_append_result = journal.append(conflicting_append_replay);
    let mut conflicting_occurred_at_replay = append_envelope.clone();
    conflicting_occurred_at_replay.occurred_at = "2026-06-25T10:00:01.000Z".into();
    let conflicting_occurred_at_result = journal.append(conflicting_occurred_at_replay);
    let mut conflicting_retention_replay = append_envelope;
    conflicting_retention_replay.retention_class = "ephemeral".into();
    let conflicting_retention_result = journal.append(conflicting_retention_replay);

    let batch_conversation_id = format!("c_batch_replay_{suffix}");
    let batch_event_id = format!("evt_batch_replay_{suffix}");
    let batch_envelope = sample_envelope(batch_event_id.as_str(), batch_conversation_id.as_str());
    let batch_positions = journal
        .append_batch(vec![batch_envelope.clone()])
        .expect("initial batch append should commit");
    let exact_batch_replay = journal.append_batch(vec![batch_envelope.clone()]);
    let mut conflicting_batch_replay = batch_envelope;
    conflicting_batch_replay.tenant_id = format!("different-tenant-{suffix}");
    conflicting_batch_replay.organization_id = format!("different-organization-{suffix}");
    conflicting_batch_replay.aggregate_id = format!("different-aggregate-{suffix}");
    conflicting_batch_replay.ordering_key = format!("different-ordering-key-{suffix}");
    let batch_rollback_conversation_id = format!("c_batch_rollback_{suffix}");
    let batch_rollback_event_id = format!("evt_batch_rollback_{suffix}");
    let batch_rollback_envelope = sample_envelope(
        batch_rollback_event_id.as_str(),
        batch_rollback_conversation_id.as_str(),
    );
    let conflicting_batch_result =
        journal.append_batch(vec![batch_rollback_envelope, conflicting_batch_replay]);

    let pool = journal.pool().clone();
    let rolled_back_batch_rows = tokio::task::spawn_blocking(move || {
        let mut client = pool.get().expect("cleanup connection should be available");
        let rolled_back_batch_rows: i64 = client
            .query_one(
                "select count(*) from im_commit_journal where event_id = $1",
                &[&batch_rollback_event_id],
            )
            .expect("rolled-back batch row should be countable")
            .get(0);
        for event_id in [&append_event_id, &batch_event_id, &batch_rollback_event_id] {
            client
                .execute(
                    "delete from im_commit_journal where event_id = $1",
                    &[event_id],
                )
                .expect("live replay test row should be cleaned up");
        }
        rolled_back_batch_rows
    })
    .await
    .expect("cleanup task should not panic");

    assert_eq!(
        exact_append_replay.expect("exact append replay should be idempotent"),
        append_position
    );
    assert!(matches!(
        conflicting_append_result,
        Err(ContractError::Conflict(_))
    ));
    assert!(matches!(
        conflicting_occurred_at_result,
        Err(ContractError::Conflict(_))
    ));
    assert!(matches!(
        conflicting_retention_result,
        Err(ContractError::Conflict(_))
    ));
    assert_eq!(
        exact_batch_replay.expect("exact batch replay should be idempotent"),
        batch_positions
    );
    assert!(matches!(
        conflicting_batch_result,
        Err(ContractError::Conflict(_))
    ));
    assert_eq!(
        rolled_back_batch_rows, 0,
        "a later conflicting replay must roll back earlier inserts in the same batch"
    );
}

fn uuid_like_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".into())
}
