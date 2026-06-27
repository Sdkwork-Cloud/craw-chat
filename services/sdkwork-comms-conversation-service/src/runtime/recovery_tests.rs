//! White-box unit tests for conversation-runtime recovery.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "recovery_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use std::panic::{self, AssertUnwindSafe};

fn poison_rwlock_write<T>(lock: &RwLock<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = lock.write().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

fn recovered_created_envelope() -> CommitEnvelope {
    let payload = RecoveredConversationCreatedPayload {
        conversation_type: "group".into(),
        business_type: None,
        business_id: None,
        parent_conversation_id: None,
        root_message_id: None,
        direct_chat: None,
        agent_dialog: None,
        system_channel: None,
        source: None,
        target: None,
        handoff: None,
    };
    CommitEnvelope {
        event_id: "evt_recovery_created".into(),
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        event_type: "conversation.created".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: "c_demo".into(),
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        ordering_key: CommitEnvelope::ordering_key("100001", "c_demo"),
        ordering_seq: 1,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: "1".into(),
            actor_kind: "user".into(),
            actor_session_id: None,
        },
        occurred_at: "2026-04-12T00:00:00.000Z".into(),
        committed_at: "2026-04-12T00:00:00.000Z".into(),
        payload_schema: Some("conversation.created.v1".into()),
        payload: serde_json::to_string(&payload).expect("payload should serialize"),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

#[test]
fn test_apply_recovered_conversation_created_recovers_from_poisoned_runtime_state_lock() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());
    let envelope = recovered_created_envelope();
    poison_rwlock_write(&runtime.state);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        runtime.apply_recovered_envelope(&envelope)
    }));
    assert!(
        result.is_ok(),
        "apply_recovered_envelope should not panic when runtime state lock is poisoned"
    );
    let apply_result = result.expect("panic status should be captured");
    assert!(apply_result.is_ok());
}
