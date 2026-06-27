use im_domain_events::{AggregateType, CommitEnvelope, EventActor};

#[test]
fn test_commit_envelope_normalizes_organization_id() {
    let envelope = CommitEnvelope::minimal("evt_demo", "100001", "message.posted", "conversation", "c_demo", 1)
        .with_organization_id("org_a");
    assert_eq!(envelope.normalized_organization_id(), "org_a");
    assert_eq!(
        CommitEnvelope::minimal("evt_demo", "100001", "message.posted", "conversation", "c_demo", 1)
            .with_organization_id("")
            .normalized_organization_id(),
        "default"
    );
}

#[test]
fn test_commit_envelope_builds_stable_ordering_key() {
    let envelope = CommitEnvelope {
        event_id: "evt_demo".into(),
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        event_type: "message.posted".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: "c_demo".into(),
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        ordering_key: CommitEnvelope::ordering_key("100001", "c_demo"),
        ordering_seq: 1,
        causation_id: Some("cmd_demo".into()),
        correlation_id: Some("corr_demo".into()),
        idempotency_key: Some("ik_demo".into()),
        actor: EventActor {
            actor_id: "1".into(),
            actor_kind: "user".into(),
            actor_session_id: Some("s_demo".into()),
        },
        occurred_at: "2026-04-05T10:00:00Z".into(),
        committed_at: "2026-04-05T10:00:01Z".into(),
        payload_schema: Some("message.posted.v1".into()),
        payload: "{}".into(),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    };

    assert_eq!(envelope.aggregate_type.as_wire_value(), "conversation");
    assert_eq!(envelope.ordering_key, "6#1000016#c_demo");
}

#[test]
fn test_commit_envelope_ordering_key_is_segment_safe() {
    assert_eq!(
        CommitEnvelope::ordering_key("tenant:a", "b"),
        "8#tenant:a1#b"
    );
    assert_eq!(
        CommitEnvelope::ordering_key("tenant", "a:b"),
        "6#tenant3#a:b"
    );
    assert_ne!(
        CommitEnvelope::ordering_key("tenant:a", "b"),
        CommitEnvelope::ordering_key("tenant", "a:b"),
        "ordering keys must not collide when tenant or scope ids contain delimiter characters"
    );
}

#[test]
fn test_aggregate_types_do_not_include_app_local_media_asset_lifecycle() {
    let source = include_str!("../src/lib.rs");

    assert!(
        !source.contains("MediaAsset"),
        "domain events must not model app-local MediaAsset lifecycle aggregates"
    );
    assert!(
        !source.contains("media_asset"),
        "domain events must not expose legacy media_asset aggregate wire value"
    );
}
