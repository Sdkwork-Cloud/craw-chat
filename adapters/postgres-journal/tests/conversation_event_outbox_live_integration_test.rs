//! Live PostgreSQL coverage for atomic conversation journal + outbox writes.
//!
//! Run with:
//! SDKWORK_DATABASE_URL=postgresql://... cargo test -p im-adapters-postgres-journal --test conversation_event_outbox_live_integration_test -- --ignored --nocapture

use im_adapters_postgres_journal::{
    PostgresDurableConversationEventWriter, PostgresJournalConfig, PostgresJournalPool,
    PostgresOutboxStore,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{ContractError, OutboxEventRecord, OutboxPublishStatus, OutboxStore};
use serde_json::json;

fn fixture(suffix: &str, scenario: &str) -> (CommitEnvelope, OutboxEventRecord) {
    let tenant_id = format!("conversation-event-test-{suffix}");
    let organization_id = "0";
    let conversation_id = format!("group-{scenario}-{suffix}");
    let event_id = format!("evt-{scenario}-{suffix}");
    let event_type = "conversation.agents_replaced";
    let occurred_at = "2026-07-12T10:00:00.000Z";
    let payload = json!({
        "conversationId": conversation_id,
        "previousGeneration": 1,
        "agentAssignments": {
            "generation": 2,
            "source": "conversation_override",
            "agents": [{
                "agentId": "agent.im.writer",
                "revisionId": "revision.im.writer.1"
            }]
        },
        "replacedAt": occurred_at
    })
    .to_string();
    let payload_hash = sdkwork_utils_rust::sha256_hash(payload.as_bytes());
    let envelope = CommitEnvelope {
        event_id: event_id.clone(),
        tenant_id: tenant_id.clone(),
        organization_id: organization_id.into(),
        event_type: event_type.into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id.as_str(), conversation_id.as_str()),
        ordering_seq: 2,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: "user-1".into(),
            actor_kind: "user".into(),
            actor_session_id: None,
        },
        occurred_at: occurred_at.into(),
        committed_at: occurred_at.into(),
        payload_schema: Some("conversation.agents_replaced.v1".into()),
        payload: payload.clone(),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    };
    let outbox = OutboxEventRecord {
        tenant_id,
        organization_id: organization_id.into(),
        outbox_id: format!("conv-ob-{scenario}-{suffix}"),
        aggregate_type: "conversation".into(),
        aggregate_id: conversation_id,
        event_id: format!("conversation:{event_type}:{event_id}"),
        event_type: event_type.into(),
        payload_json: payload,
        payload_hash,
        publish_status: OutboxPublishStatus::Pending,
        attempt_count: 0,
        available_at: occurred_at.into(),
        published_at: None,
        created_at: occurred_at.into(),
        updated_at: occurred_at.into(),
    };
    (envelope, outbox)
}

async fn row_counts(
    pool: PostgresJournalPool,
    envelope: &CommitEnvelope,
    outbox: &OutboxEventRecord,
) -> (i64, i64) {
    let event_id = envelope.event_id.clone();
    let tenant_id = outbox.tenant_id.clone();
    let organization_id = outbox.organization_id.clone();
    let outbox_id = outbox.outbox_id.clone();
    tokio::task::spawn_blocking(move || {
        let mut client = pool.get().expect("count connection should be available");
        let row = client
            .query_one(
                r#"
select
  (select count(*) from im_commit_journal where event_id = $1),
  (select count(*) from im_outbox_events where tenant_id = $2 and organization_id = $3 and outbox_id = $4)
"#,
                &[&event_id, &tenant_id, &organization_id, &outbox_id],
            )
            .expect("durable rows should be countable");
        (row.get(0), row.get(1))
    })
    .await
    .expect("count task should not panic")
}

async fn execute(pool: PostgresJournalPool, sql: &'static str, params: Vec<String>) -> u64 {
    tokio::task::spawn_blocking(move || {
        let mut client = pool.get().expect("database connection should be available");
        let references = params
            .iter()
            .map(|value| value as &(dyn postgres::types::ToSql + Sync))
            .collect::<Vec<_>>();
        client
            .execute(sql, references.as_slice())
            .expect("live integration SQL should succeed")
    })
    .await
    .expect("database task should not panic")
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn conversation_event_outbox_is_atomic_idempotent_and_self_healing() {
    let database_url = std::env::var("SDKWORK_DATABASE_URL")
        .expect("SDKWORK_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let journal = PostgresJournalConfig::new(database_url)
        .connect()
        .expect("postgres journal should connect");
    let pool = journal.pool().clone();
    let writer = PostgresDurableConversationEventWriter::from_journal(&journal);
    let suffix = unique_suffix();

    let (envelope, outbox) = fixture(suffix.as_str(), "commit");
    let position = writer
        .persist_conversation_event(envelope.clone(), outbox.clone())
        .expect("journal and outbox should commit atomically");
    assert_eq!(row_counts(pool.clone(), &envelope, &outbox).await, (1, 1));
    assert_eq!(
        writer
            .persist_conversation_event(envelope.clone(), outbox.clone())
            .expect("exact replay should be idempotent"),
        position
    );

    execute(
        pool.clone(),
        "delete from im_outbox_events where tenant_id = $1 and organization_id = $2 and outbox_id = $3",
        vec![
            outbox.tenant_id.clone(),
            outbox.organization_id.clone(),
            outbox.outbox_id.clone(),
        ],
    )
    .await;
    assert_eq!(row_counts(pool.clone(), &envelope, &outbox).await, (1, 0));
    assert_eq!(
        writer
            .persist_conversation_event(envelope.clone(), outbox.clone())
            .expect("journal replay should repair its missing deterministic outbox"),
        position
    );
    assert_eq!(row_counts(pool.clone(), &envelope, &outbox).await, (1, 1));

    execute(
        pool.clone(),
        "update im_outbox_events set payload_hash = $4 where tenant_id = $1 and organization_id = $2 and outbox_id = $3",
        vec![
            outbox.tenant_id.clone(),
            outbox.organization_id.clone(),
            outbox.outbox_id.clone(),
            "tampered-producer-hash".into(),
        ],
    )
    .await;
    assert!(matches!(
        writer.persist_conversation_event(envelope.clone(), outbox.clone()),
        Err(ContractError::Conflict(_))
    ));

    let (rollback_envelope, rollback_outbox) = fixture(suffix.as_str(), "rollback");
    let mut occupying_outbox = rollback_outbox.clone();
    occupying_outbox.event_id = format!("different-{}", occupying_outbox.event_id);
    PostgresOutboxStore::from_pool(pool.clone())
        .enqueue(occupying_outbox)
        .expect("conflicting outbox fixture should be inserted");
    assert!(matches!(
        writer.persist_conversation_event(rollback_envelope.clone(), rollback_outbox.clone()),
        Err(ContractError::Conflict(_))
    ));
    assert_eq!(
        row_counts(pool.clone(), &rollback_envelope, &rollback_outbox).await,
        (0, 1),
        "outbox identity conflict must roll back the newly inserted journal row"
    );

    execute(
        pool.clone(),
        "delete from im_outbox_events where tenant_id = $1 and organization_id = $2",
        vec![outbox.tenant_id.clone(), outbox.organization_id.clone()],
    )
    .await;
    execute(
        pool,
        "delete from im_commit_journal where tenant_id = $1 and organization_id = $2",
        vec![outbox.tenant_id.clone(), outbox.organization_id.clone()],
    )
    .await;
}

fn unique_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".into())
}
