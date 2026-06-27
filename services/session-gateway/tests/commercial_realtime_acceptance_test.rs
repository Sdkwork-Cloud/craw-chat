use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use im_adapters_local_memory::{
    MemoryRealtimeCheckpointStore, MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput,
};

const TENANT_ID: &str = "t_commercial";
const PRINCIPAL_ID: &str = "1133";
const PRINCIPAL_KIND: &str = "user";
const CONVERSATION_ID: &str = "c_commercial";
const EVENT_TYPE: &str = "message.posted";

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("realtime commercial acceptance operation should succeed")
}

fn message_payload(index: usize) -> String {
    format!(r#"{{"messageId":"msg_commercial_{index:04}","index":{index}}}"#)
}

fn subscription(scope_id: &str) -> Vec<RealtimeSubscriptionItemInput> {
    vec![RealtimeSubscriptionItemInput {
        scope_type: "conversation".into(),
        scope_id: scope_id.into(),
        event_types: vec![EVENT_TYPE.into()],
    }]
}

fn publish_message(
    runtime: &RealtimeDeliveryRuntime,
    index: usize,
    candidate_device_ids: Vec<String>,
) -> usize {
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(index),
        candidate_device_ids,
    ))
}

#[test]
fn test_commercial_realtime_core_survives_multi_client_route_pressure_trim_restore_and_compensation()
 {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );

    for device_id in ["d_primary", "d_mobile", "d_tablet"] {
        expect_ok(runtime.sync_subscriptions_for_principal_kind(
            TENANT_ID,
        "default",
        PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
            subscription(CONVERSATION_ID),
        ));
    }
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_other_conversation",
        subscription("c_other"),
    ));

    let candidate_devices = vec![
        "d_primary".to_owned(),
        "d_mobile".to_owned(),
        "d_tablet".to_owned(),
        "d_other_conversation".to_owned(),
        "d_missing".to_owned(),
    ];
    for index in 1..=1_050 {
        assert_eq!(
            publish_message(&runtime, index, candidate_devices.clone()),
            3,
            "fanout should deliver only to subscribed devices at message {index}"
        );
    }

    for device_id in ["d_primary", "d_mobile", "d_tablet"] {
        let checkpoint = expect_ok(runtime.window_checkpoint_for_principal_kind(
            TENANT_ID,
            "default",
            PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
        ));
        assert_eq!(checkpoint.latest_realtime_seq, 1_050);
        assert_eq!(checkpoint.acked_through_seq, 0);
        assert_eq!(checkpoint.trimmed_through_seq, 50);

        let persisted_checkpoint = checkpoint_store
            .checkpoint(TENANT_ID, "default", PRINCIPAL_KIND, PRINCIPAL_ID, device_id)
            .expect("trimmed checkpoint should persist commercial diagnostics metadata");
        assert_eq!(persisted_checkpoint.capacity_trimmed_event_count, 50);
        assert_eq!(persisted_checkpoint.capacity_trimmed_through_seq, 50);
        assert!(
            persisted_checkpoint.last_capacity_trimmed_at.is_some(),
            "capacity trimming should persist an operational timestamp"
        );

        let window = expect_ok(runtime.list_events_for_principal_kind(
            TENANT_ID,
        "default",
        PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
            0,
            1_000,
        ));
        assert_eq!(window.items.len(), 1_000);
        assert_eq!(window.items[0].realtime_seq, 51);
        assert_eq!(window.items[999].realtime_seq, 1_050);
        assert_eq!(window.trimmed_through_seq, 50);
        assert!(!window.has_more);
    }

    let unmatched_window = expect_ok(runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_other_conversation",
        0,
        10,
    ));
    assert_eq!(
        unmatched_window.items.len(),
        0,
        "fanout must not leak events into devices subscribed to another conversation"
    );

    let ack = expect_ok(runtime.ack_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        800,
    ));
    assert_eq!(ack.acked_through_seq, 800);
    assert_eq!(ack.trimmed_through_seq, 800);
    assert_eq!(ack.retained_event_count, 250);

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );
    let restored_primary = expect_ok(rebuilt_runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        0,
        1_000,
    ));
    assert_eq!(restored_primary.items.len(), 250);
    assert_eq!(restored_primary.items[0].realtime_seq, 801);
    assert_eq!(restored_primary.items[249].realtime_seq, 1_050);
    assert_eq!(restored_primary.acked_through_seq, 800);
    assert_eq!(restored_primary.trimmed_through_seq, 800);

    let restored_mobile = expect_ok(rebuilt_runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_mobile",
        0,
        1_000,
    ));
    assert_eq!(restored_mobile.items.len(), 1_000);
    assert_eq!(restored_mobile.items[0].realtime_seq, 51);
    assert_eq!(restored_mobile.items[999].realtime_seq, 1_050);

    let diagnostics = expect_ok(rebuilt_runtime.realtime_inbox_diagnostics());
    assert_eq!(diagnostics.status, "critical");
    assert_eq!(diagnostics.client_route_window_count, 3);
    assert_eq!(diagnostics.pending_event_count, 2_250);
    assert_eq!(diagnostics.max_client_route_window_event_count, 1_000);
    assert_eq!(diagnostics.max_client_route_window_usage_permille, 1_000);
    assert_eq!(diagnostics.capacity_trimmed_event_count, 150);
    assert_eq!(diagnostics.max_capacity_trimmed_through_seq, 50);
    assert_eq!(diagnostics.max_trimmed_through_seq, 800);
    assert_eq!(diagnostics.high_risk_windows.len(), 3);
    assert!(
        diagnostics
            .high_risk_windows
            .iter()
            .any(|window| window.device_id == "d_mobile" && window.usage_permille == 1_000),
        "diagnostics should identify saturated client route windows"
    );

    let failing_checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::with_seed(
        checkpoint_store.clone(),
    ));
    let failure_runtime = RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
        failing_checkpoint_store.clone(),
        subscription_store,
        event_window_store,
        Arc::new(session_gateway::StandaloneRealtimeScopeAccessPolicy),
    );
    failing_checkpoint_store.fail_saves();
    let error = failure_runtime
        .ack_events_for_principal_kind(TENANT_ID, "default", PRINCIPAL_ID, PRINCIPAL_KIND, "d_mobile", 900)
        .expect_err("failed checkpoint persistence should reject ack");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    let mobile_after_failed_ack = expect_ok(failure_runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_mobile",
        0,
        1_000,
    ));
    assert_eq!(
        mobile_after_failed_ack.items.len(),
        1_000,
        "checkpoint failure must roll durable window trim back"
    );
    assert_eq!(mobile_after_failed_ack.items[0].realtime_seq, 51);
    assert_eq!(mobile_after_failed_ack.acked_through_seq, 0);
    assert_eq!(mobile_after_failed_ack.trimmed_through_seq, 50);
}

#[test]
fn test_commercial_realtime_cluster_handoff_preserves_checkpoint_and_pending_window() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        subscription(CONVERSATION_ID),
    ));
    cluster
        .bind_client_route_for_principal_kind(
            TENANT_ID,
            "default",
            PRINCIPAL_ID,
            PRINCIPAL_KIND,
            "d_primary",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial websocket route should bind");

    for index in 1..=24 {
        assert_eq!(
            publish_message(&runtime_a, index, vec!["d_primary".to_owned()]),
            1
        );
    }
    let ack = expect_ok(runtime_a.ack_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        12,
    ));
    assert_eq!(ack.acked_through_seq, 12);
    assert_eq!(ack.retained_event_count, 12);

    cluster
        .mark_node_draining("node_a")
        .expect("node should drain");
    cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("draining node should migrate routes");

    let route = cluster
        .resolve_client_route_for_principal_kind(
            TENANT_ID,
            "default",
            PRINCIPAL_ID,
            PRINCIPAL_KIND,
            "d_primary",
        )
        .expect("migrated route should resolve");
    assert_eq!(route.owner_node_id, "node_b");
    assert_eq!(route.session_id.as_deref(), Some("s_old"));
    assert_eq!(route.connection_kind, "websocket");

    let source_after_migration = expect_ok(runtime_a.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        0,
        100,
    ));
    assert_eq!(
        source_after_migration.items.len(),
        0,
        "source node should no longer retain the migrated pending window"
    );

    let target_window = expect_ok(runtime_b.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        0,
        100,
    ));
    assert_eq!(target_window.items.len(), 12);
    assert_eq!(target_window.items[0].realtime_seq, 13);
    assert_eq!(target_window.items[11].realtime_seq, 24);
    assert_eq!(target_window.acked_through_seq, 12);
    assert_eq!(target_window.trimmed_through_seq, 12);

    let publish_after_migration = cluster.publish_client_route_event_for_principal_kind(
        "node_a",
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(25),
    );
    assert_eq!(publish_after_migration.route_state, "resolved");
    assert_eq!(publish_after_migration.target_node_id, "node_b");
    assert_eq!(publish_after_migration.delivered, 1);

    let target_after_publish = expect_ok(runtime_b.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        12,
        100,
    ));
    assert_eq!(target_after_publish.items.len(), 13);
    assert_eq!(target_after_publish.items[12].realtime_seq, 25);
}

#[derive(Clone)]
struct ToggleRealtimeCheckpointStore {
    seed: Arc<MemoryRealtimeCheckpointStore>,
    fail_saves: Arc<AtomicBool>,
}

impl ToggleRealtimeCheckpointStore {
    fn with_seed(seed: Arc<MemoryRealtimeCheckpointStore>) -> Self {
        Self {
            seed,
            fail_saves: Arc::new(AtomicBool::new(false)),
        }
    }

    fn fail_saves(&self) {
        self.fail_saves.store(true, Ordering::SeqCst);
    }
}

impl RealtimeCheckpointStore for ToggleRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        _organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        self.seed
            .load_checkpoint(tenant_id, "default", principal_kind, principal_id, device_id)
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.fail_saves.load(Ordering::SeqCst) {
            return Err(ContractError::Unavailable(
                "synthetic commercial checkpoint save failure".into(),
            ));
        }
        self.seed.save_checkpoints(records)
    }
}
