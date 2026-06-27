//! White-box unit tests for session-gateway cluster bridge.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "cluster_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
use im_platform_contracts::{
    ContractError, RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
};

use super::*;
use crate::RealtimeSubscriptionItemInput;

fn expect_ok<T>(result: Result<T, crate::realtime::RealtimeRuntimeError>) -> T {
    result.expect("realtime runtime operation should succeed")
}

fn poison_mutex<T>(mutex: &Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

#[test]
fn test_bind_node_runtime_recovers_from_poisoned_runtime_registry_lock() {
    let cluster = RealtimeClusterBridge::default();
    poison_mutex(&cluster.node_runtimes);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        cluster.bind_node_runtime(
            "node_a",
            Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
        );
    }));
    assert!(
        result.is_ok(),
        "bind_node_runtime should not panic when runtime registry mutex is poisoned"
    );
    assert!(cluster.node_lifecycle("node_a").is_some());
}

#[test]
fn test_route_rebind_recovers_from_poisoned_runtime_registry_lock() {
    let cluster = RealtimeClusterBridge::default();
    cluster.bind_node_runtime(
        "node_a",
        Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
    );
    cluster.bind_node_runtime(
        "node_b",
        Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
    );
    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");

    poison_mutex(&cluster.node_runtimes);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        cluster.bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_b",
            Some("s_new"),
            "http",
        )
    }));
    assert!(
        result.is_ok(),
        "route rebind should not panic when runtime registry mutex is poisoned"
    );
    let bind_result = result.expect("panic status should be captured");
    assert!(
        bind_result.is_ok(),
        "route rebind should recover from poisoned runtime registry lock"
    );
}

#[test]
fn test_publish_recovers_from_poisoned_runtime_registry_lock() {
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    poison_mutex(&cluster.node_runtimes);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        cluster.publish_client_route_event_for_principal_kind(
            "node_a",
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_poison"}"#.into(),
        )
    }));
    assert!(
        result.is_ok(),
        "publish should not panic when runtime registry mutex is poisoned"
    );
    let publish_result = result.expect("panic status should be captured");
    assert_eq!(publish_result.target_node_id, "node_a");
    assert_eq!(publish_result.route_state, "local_fallback");
    assert_eq!(publish_result.delivered, 1);
}

#[test]
fn test_publish_does_not_fallback_to_origin_when_route_points_to_missing_target_runtime() {
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b);

    expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_b",
            None,
            "websocket",
        )
        .expect("route bind should succeed");

    cluster
        .node_runtimes
        .lock()
        .expect("realtime cluster runtime registry should lock")
        .remove("node_b");

    let result = cluster.publish_client_route_event_for_principal_kind(
        "node_a",
        "100001",
        "default",
        "1",
        "user",
        "d_pad",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
    );

    assert_eq!(result.target_node_id, "node_b");
    assert_eq!(result.route_state, "target_runtime_missing");
    assert_eq!(result.delivered, 0);

    let origin_window = expect_ok(
        runtime_a.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(origin_window.items.len(), 0);
}

#[test]
fn test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing() {
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    cluster
        .mark_node_draining("node_a")
        .expect("source drain should succeed");

    cluster
        .node_runtimes
        .lock()
        .expect("realtime cluster runtime registry should lock")
        .remove("node_a");

    let rebound = cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_b",
            Some("s_new"),
            "http",
        )
        .expect("stale route should not block takeover when previous runtime is missing");
    assert_eq!(rebound.owner_node_id, "node_b");
    assert_eq!(rebound.connection_kind, "http");
    assert_eq!(rebound.session_id.as_deref(), Some("s_new"));

    let source_lifecycle = cluster
        .node_lifecycle("node_a")
        .expect("source lifecycle should remain observable");
    assert_eq!(source_lifecycle.drain_status, "drained");
    assert_eq!(source_lifecycle.rebalance_state, "stable");
    assert_eq!(source_lifecycle.owned_route_count, 0);

    expect_ok(runtime_b.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let publish = cluster.publish_client_route_event_for_principal_kind(
        "node_a",
        "100001",
        "default",
        "1",
        "user",
        "d_pad",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_after_stale_takeover"}"#.into(),
    );

    assert_eq!(publish.target_node_id, "node_b");
    assert_eq!(publish.route_state, "resolved");
    assert_eq!(publish.delivered, 1);

    let target_window = expect_ok(
        runtime_b.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(target_window.items.len(), 1);
    assert_eq!(target_window.items[0].event_type, "message.posted");
}

#[test]
fn test_route_session_fence_rejects_stale_session_after_takeover() {
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_b",
            Some("s_new"),
            "http",
        )
        .expect("takeover route bind should succeed");

    let stale_error = cluster
        .ensure_route_session_current_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            Some("s_old"),
        )
        .expect_err("stale session should be rejected after takeover");
    assert_eq!(stale_error.code, "stale_session");
    assert_eq!(stale_error.node_id, "node_b");

    cluster
        .ensure_route_session_current_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            Some("s_new"),
        )
        .expect("current session should remain valid");
}

#[test]
fn test_route_session_fence_requires_session_id_once_route_is_bound_to_session() {
    let cluster = RealtimeClusterBridge::default();
    let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime);

    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "node_a",
            Some("s_live"),
            "websocket",
        )
        .expect("initial route bind should succeed");

    let error = cluster
        .ensure_route_session_current_for_principal_kind("100001", "default", "1", "user", "d_pad", None)
        .expect_err("missing session id should be rejected once route has current session");
    assert_eq!(error.code, "session_id_required");
    assert_eq!(error.node_id, "node_a");
}

#[test]
fn test_disconnect_fence_requires_resume_until_cleared() {
    let cluster = RealtimeClusterBridge::default();
    let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime);

    cluster
        .mark_client_route_disconnected_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            Some("s_old"),
            "node_a",
        )
        .expect("disconnect fence should persist");

    let error = cluster
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect_err("disconnect fence should require an explicit resume");
    assert_eq!(error.code, "reconnect_required");
    assert_eq!(error.node_id, "node_a");
    assert!(
        cluster
            .disconnect_fence_matches_client_route_session_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old")
            )
            .expect("session match should load")
    );
    assert!(
        !cluster
            .disconnect_fence_matches_client_route_session_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_other")
            )
            .expect("session mismatch should load")
    );

    assert!(
        cluster
            .clear_client_route_disconnect_fence_for_principal_kind(
                "100001", "1", "user", "d_pad"
            )
            .expect("disconnect fence clear should succeed")
    );
    cluster
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect("fresh resume should clear the disconnect fence");
}

#[test]
fn test_disconnect_fence_survives_bridge_rebuild_with_shared_store() {
    let store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
    let cluster_a = RealtimeClusterBridge::with_disconnect_fence_store(store.clone());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster_a.bind_node_runtime("node_a", runtime_a);
    cluster_a
        .mark_client_route_disconnected_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            Some("s_old"),
            "node_a",
        )
        .expect("disconnect fence should persist");

    let cluster_b = RealtimeClusterBridge::with_disconnect_fence_store(store);
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster_b.bind_node_runtime("node_b", runtime_b);

    let error = cluster_b
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect_err("persisted disconnect fence should still require a fresh resume");
    assert_eq!(error.code, "reconnect_required");
    assert!(
        cluster_b
            .disconnect_fence_matches_client_route_session_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old")
            )
            .expect("restored session match should load")
    );

    assert!(
        cluster_b
            .clear_client_route_disconnect_fence_for_principal_kind(
                "100001", "1", "user", "d_pad"
            )
            .expect("restored fence clear should succeed")
    );
    cluster_b
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect("clearing the restored fence should allow traffic again");
}

#[test]
fn test_disconnect_fence_clear_for_current_session_does_not_delete_new_disconnect_fence() {
    let store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
    store
        .save_fence(RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_new".into()),
            owner_node_id: "node_b".into(),
            disconnected_at: "2026-05-06T00:00:02.000Z".into(),
            fence_token: "fence:100001:user:1:d_pad:s_new:node_b:2026-05-06T00:00:02.000Z"
                .into(),
        })
        .expect("new disconnect fence should persist");
    let cluster = RealtimeClusterBridge::with_disconnect_fence_store(store.clone());

    let cleared = cluster
        .clear_client_route_disconnect_fence_for_current_session(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_new"),
        )
        .expect("protected fence clear should succeed");

    assert!(!cleared);
    let error = cluster
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect_err("current session disconnect fence must still require a fresh resume");
    assert_eq!(error.code, "reconnect_required");
}

#[derive(Clone, Default)]
struct FailingDisconnectFenceStore;

impl RealtimeDisconnectFenceStore for FailingDisconnectFenceStore {
    fn load_fence(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Err(ContractError::Unavailable(
            "disconnect fence store load failed".into(),
        ))
    }

    fn save_fence(&self, _record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        Err(ContractError::Unavailable(
            "disconnect fence store save failed".into(),
        ))
    }

    fn clear_fence(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<bool, ContractError> {
        Err(ContractError::Unavailable(
            "disconnect fence store clear failed".into(),
        ))
    }

    fn clear_fence_disconnected_at_or_before(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
        _cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError> {
        Err(ContractError::Unavailable(
            "disconnect fence store clear failed".into(),
        ))
    }

    fn clear_fence_if_matches(
        &self,
        _expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        Err(ContractError::Unavailable(
            "disconnect fence store clear failed".into(),
        ))
    }
}

#[test]
fn test_disconnect_fence_store_failures_surface_as_controlled_cluster_errors() {
    let cluster =
        RealtimeClusterBridge::with_disconnect_fence_store(Arc::new(FailingDisconnectFenceStore));
    let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime);

    let save_error = cluster
        .mark_client_route_disconnected_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            Some("s_old"),
            "node_a",
        )
        .expect_err("save failure should not panic");
    assert_eq!(save_error.code, "disconnect_fence_store_unavailable");

    let load_error = cluster
        .ensure_client_route_resume_not_required_for_principal_kind(
            "100001", "1", "user", "d_pad",
        )
        .expect_err("load failure should surface as a controlled error");
    assert_eq!(load_error.code, "disconnect_fence_store_unavailable");

    let clear_error = cluster
        .clear_client_route_disconnect_fence_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect_err("clear failure should not panic");
    assert_eq!(clear_error.code, "disconnect_fence_store_unavailable");
}
