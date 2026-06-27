//! White-box unit tests for session-gateway realtime delivery runtime.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "realtime_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use std::sync::Arc;

use im_adapters_local_memory::MemoryRealtimeCheckpointStore;

#[test]
fn test_postgres_realtime_sql_contracts_are_compiled_with_runtime_module() {
    assert_eq!(realtime_postgres_sql_contracts().len(), 21);
    assert_eq!(realtime_postgres_sql_contract_specs().len(), 21);
    assert_eq!(realtime_postgres_transaction_plans().len(), 6);
    assert_eq!(realtime_postgres_adapter_plan().method_plans.len(), 21);
    assert!(
        realtime_postgres_sql_contracts()
            .iter()
            .all(|sql| !sql.trim().is_empty())
    );
    assert!(
        realtime_postgres_sql_contract_specs()
            .iter()
            .all(|spec| !spec.name.trim().is_empty() && !spec.sql.trim().is_empty())
    );
    assert!(
        realtime_postgres_transaction_plans()
            .iter()
            .all(|plan| !plan.trim().is_empty())
    );
}

#[test]
fn test_collect_matched_delivery_targets_filters_to_registered_matching_devices() {
    let mut subscriptions = HashMap::new();
    subscriptions.insert(
        client_route_scope_key("100001", "1", "user", "d_match"),
        RealtimeClientRouteSubscriptions::from_items(vec![subscription(
            "conversation",
            "c_demo",
            vec!["message.posted"],
        )]),
    );
    subscriptions.insert(
        client_route_scope_key("100001", "1", "user", "d_other_scope"),
        RealtimeClientRouteSubscriptions::from_items(vec![subscription(
            "conversation",
            "c_other",
            vec!["message.posted"],
        )]),
    );
    subscriptions.insert(
        client_route_scope_key("100001", "1", "user", "d_other_event"),
        RealtimeClientRouteSubscriptions::from_items(vec![subscription(
            "conversation",
            "c_demo",
            vec!["message.read"],
        )]),
    );
    let subscription_scope_index = subscription_scope_index_from_subscriptions(&[
        (
            "d_match",
            subscriptions
                .get(client_route_scope_key("100001", "1", "user", "d_match").as_str())
                .expect("matching subscription should exist"),
        ),
        (
            "d_other_scope",
            subscriptions
                .get(client_route_scope_key("100001", "1", "user", "d_other_scope").as_str())
                .expect("other scope subscription should exist"),
        ),
        (
            "d_other_event",
            subscriptions
                .get(client_route_scope_key("100001", "1", "user", "d_other_event").as_str())
                .expect("other event subscription should exist"),
        ),
    ]);

    let matched = collect_matched_delivery_targets(
        &subscription_scope_index,
        "100001",
        "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        vec![
            "d_other_scope".into(),
            "d_match".into(),
            "d_other_event".into(),
            "d_match".into(),
            "d_missing".into(),
        ],
    );

    assert_eq!(
        matched,
        vec![(
            client_route_scope_key("100001", "1", "user", "d_match"),
            "d_match".into()
        )]
    );
}

#[test]
fn test_collect_matched_delivery_targets_accepts_wildcard_event_subscriptions() {
    let mut subscriptions = HashMap::new();
    subscriptions.insert(
        client_route_scope_key("100001", "1", "user", "d_wildcard"),
        RealtimeClientRouteSubscriptions::from_items(vec![subscription(
            "conversation",
            "c_demo",
            vec![],
        )]),
    );
    let subscription_scope_index = subscription_scope_index_from_subscriptions(&[(
        "d_wildcard",
        subscriptions
            .get(client_route_scope_key("100001", "1", "user", "d_wildcard").as_str())
            .expect("wildcard subscription should exist"),
    )]);

    let matched = collect_matched_delivery_targets(
        &subscription_scope_index,
        "100001",
        "1",
        "user",
        "conversation",
        "c_demo",
        "message.edited",
        vec!["d_wildcard".into()],
    );

    assert_eq!(
        matched,
        vec![(
            client_route_scope_key("100001", "1", "user", "d_wildcard"),
            "d_wildcard".into()
        )]
    );
}

#[test]
fn test_persist_checkpoint_normalizes_transient_inconsistent_sequence_state() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
    let scope_key = client_route_scope_key("100001", "1", "user", "d_pad");

    runtime
        .latest_sequences
        .lock()
        .expect("realtime sequence store should lock")
        .insert(scope_key.clone(), 3);
    runtime
        .acked_sequences
        .lock()
        .expect("realtime ack store should lock")
        .insert(scope_key.clone(), 9);
    runtime
        .trimmed_sequences
        .lock()
        .expect("realtime trim store should lock")
        .insert(scope_key, 11);

    runtime
        .persist_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("checkpoint persist should succeed");

    let persisted = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint should be persisted");
    assert_eq!(persisted.latest_realtime_seq, 3);
    assert_eq!(persisted.acked_through_seq, 3);
    assert_eq!(persisted.trimmed_through_seq, 3);
}

#[test]
fn test_window_checkpoint_normalizes_transient_inconsistent_sequence_state() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    let scope_key = client_route_scope_key("100001", "1", "user", "d_pad");
    runtime
        .latest_sequences
        .lock()
        .expect("realtime sequence store should lock")
        .insert(scope_key.clone(), 3);
    runtime
        .acked_sequences
        .lock()
        .expect("realtime ack store should lock")
        .insert(scope_key.clone(), 9);
    runtime
        .trimmed_sequences
        .lock()
        .expect("realtime trim store should lock")
        .insert(scope_key, 11);

    let checkpoint = runtime
        .window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("window checkpoint should be readable");

    assert_eq!(checkpoint.latest_realtime_seq, 3);
    assert_eq!(checkpoint.acked_through_seq, 3);
    assert_eq!(checkpoint.trimmed_through_seq, 3);
}

#[test]
fn test_ensure_client_route_state_recovers_from_poisoned_sequence_store_lock() {
    let runtime = RealtimeDeliveryRuntime::default();
    let _ = std::panic::catch_unwind({
        let latest_sequences = runtime.latest_sequences.clone();
        move || {
            let _guard = latest_sequences
                .lock()
                .expect("realtime sequence store should lock");
            panic!("poison realtime sequence store lock");
        }
    });

    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_poison")
        .expect("poisoned lock should be recovered");
    let checkpoint = runtime
        .window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_poison")
        .expect("window checkpoint should still be available");
    assert_eq!(checkpoint.latest_realtime_seq, 0);
    assert_eq!(checkpoint.acked_through_seq, 0);
    assert_eq!(checkpoint.trimmed_through_seq, 0);
}

#[test]
fn test_sync_subscriptions_rejects_archived_conversation_scope_when_policy_denies() {
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(ArchivedConversationPolicy),
    );

    let error = runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_archived".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect_err("archived conversation subscription should be rejected");

    assert_eq!(error.code, "conversation_archived");
}

#[test]
fn test_list_events_filters_hidden_conversation_scopes_and_advances_cursor() {
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(ArchivedConversationPolicy),
    );
    let scope_key = client_route_scope_key("100001", "1", "user", "d_pad");
    lock_realtime_mutex(&runtime.windows, "realtime window store").insert(
        scope_key.clone(),
        [
            (
                1,
                RealtimeEvent {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_pad".into(),
                    realtime_seq: 1,
                    scope_type: "conversation".into(),
                    scope_id: "c_visible".into(),
                    event_type: "message.posted".into(),
                    delivery_class: "ephemeral".into(),
                    payload: "{}".into(),
                    occurred_at: "2026-04-15T10:00:00Z".into(),
                },
            ),
            (
                2,
                RealtimeEvent {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_pad".into(),
                    realtime_seq: 2,
                    scope_type: "conversation".into(),
                    scope_id: "c_archived".into(),
                    event_type: "message.posted".into(),
                    delivery_class: "ephemeral".into(),
                    payload: "{}".into(),
                    occurred_at: "2026-04-15T10:00:01Z".into(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    );
    lock_realtime_mutex(&runtime.latest_sequences, "realtime sequence store").insert(scope_key, 2);

    let window = runtime
        .list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10)
        .expect("filtered realtime window should be readable");

    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].scope_id, "c_visible");
    assert_eq!(window.next_after_seq, Some(2));
    assert!(!window.has_more);
}

#[test]
fn test_list_events_never_returns_events_at_or_below_trim_boundary() {
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(StandaloneRealtimeScopeAccessPolicy),
    );
    let scope_key = client_route_scope_key("100001", "1", "user", "d_pad");
    lock_realtime_mutex(&runtime.windows, "realtime window store").insert(
        scope_key.clone(),
        [
            (
                1,
                RealtimeEvent {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_pad".into(),
                    realtime_seq: 1,
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_type: "message.posted".into(),
                    delivery_class: "ephemeral".into(),
                    payload: r#"{"messageId":"trimmed"}"#.into(),
                    occurred_at: "2026-04-15T10:00:00Z".into(),
                },
            ),
            (
                2,
                RealtimeEvent {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_pad".into(),
                    realtime_seq: 2,
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_type: "message.posted".into(),
                    delivery_class: "ephemeral".into(),
                    payload: r#"{"messageId":"visible"}"#.into(),
                    occurred_at: "2026-04-15T10:00:01Z".into(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    );
    lock_realtime_mutex(&runtime.latest_sequences, "realtime sequence store")
        .insert(scope_key.clone(), 2);
    lock_realtime_mutex(&runtime.trimmed_sequences, "realtime trim store").insert(scope_key, 1);

    let window = runtime
        .list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10)
        .expect("trimmed realtime window should be readable");

    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].realtime_seq, 2);
    assert_eq!(window.items[0].payload, r#"{"messageId":"visible"}"#);
    assert_eq!(window.next_after_seq, Some(2));
    assert_eq!(window.trimmed_through_seq, 1);
}

struct ArchivedConversationPolicy;

impl RealtimeScopeAccessPolicy for ArchivedConversationPolicy {
    fn validate_subscription_scope(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        if scope_type == "conversation" && scope_id == "c_archived" {
            return Err(RealtimeRuntimeError {
                code: "conversation_archived",
                message: format!("direct chat conversation is archived: {scope_id}"),
            });
        }

        Ok(())
    }

    fn is_event_visible(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        event: &RealtimeEvent,
    ) -> bool {
        event.scope_type != "conversation" || event.scope_id != "c_archived"
    }
}

fn subscription(scope_type: &str, scope_id: &str, event_types: Vec<&str>) -> RealtimeSubscription {
    RealtimeSubscription {
        scope_type: scope_type.into(),
        scope_id: scope_id.into(),
        event_types: event_types.into_iter().map(|item| item.into()).collect(),
        subscribed_at: "2026-04-05T10:10:00Z".into(),
    }
}

fn subscription_scope_index_from_subscriptions(
    subscriptions: &[(&str, &RealtimeClientRouteSubscriptions)],
) -> HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>> {
    let mut index: HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>> =
        HashMap::new();
    for (device_id, client_route_subscriptions) in subscriptions {
        for subscription_scope in &client_route_subscriptions.scope_order {
            let subscription = client_route_subscriptions
                .by_scope
                .get(subscription_scope)
                .expect("test subscription should exist for scope");
            index
                .entry(RealtimePrincipalScopeKey::new(
                    "100001",
                    "user",
                    "1",
                    subscription_scope.scope_type.as_str(),
                    subscription_scope.scope_id.as_str(),
                ))
                .or_default()
                .insert((*device_id).into(), subscription.clone());
        }
    }
    index
}
