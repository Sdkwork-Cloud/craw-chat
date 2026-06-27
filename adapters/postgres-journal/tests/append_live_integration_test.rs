//! Live PostgreSQL append repro for journal binding issues.
//! Run with:
//! SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p im-adapters-postgres-journal --test append_live_integration_test -- --ignored --nocapture

use im_adapters_postgres_journal::PostgresJournalConfig;
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::CommitJournal;
use serde_json::json;

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

#[test]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
fn append_agent_dialog_envelope_live() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    let journal = PostgresJournalConfig::new(database_url)
        .connect()
        .expect("postgres journal should connect");

    let conversation_id = format!("c_agent_dialog_live_{}", uuid_like_suffix());
    let event_id = format!("evt_{conversation_id}_created");
    let envelope = sample_envelope(event_id.as_str(), conversation_id.as_str());

    let result = journal.append(envelope);
    match &result {
        Ok(position) => {
            eprintln!("append ok: partition={} offset={}", position.partition, position.offset);
        }
        Err(error) => {
            eprintln!("append failed: {error:?}");
        }
    }
    result.expect("append should succeed against live postgres");
}

fn uuid_like_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".into())
}
