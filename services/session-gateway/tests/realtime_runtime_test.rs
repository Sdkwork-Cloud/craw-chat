use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use im_adapters_local_memory::{
    MemoryRealtimeCheckpointStore, MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscription};
use im_platform_contracts::{
    ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeMatchingSubscriptionQuery, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
use session_gateway::{
    RealtimeDeliveryRuntime, RealtimeRuntimeError, RealtimeSubscriptionItemInput,
};

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("realtime runtime operation should succeed")
}

#[test]
fn test_default_runtime_rejects_scope_subscriptions_without_access_policy() {
    let runtime = RealtimeDeliveryRuntime::default();

    let error = runtime
        .sync_subscriptions_for_principal_kind(
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
        )
        .expect_err("default realtime runtime should fail closed without scope access policy");

    assert_eq!(error.code, "realtime_scope_access_denied");
}

#[test]
fn test_ack_events_trims_window_and_tracks_checkpoint() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    let delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);

    let ack =
        expect_ok(runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 1));
    assert_eq!(ack.acked_through_seq, 1);
    assert_eq!(ack.trimmed_through_seq, 1);
    assert_eq!(ack.retained_event_count, 0);

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 0);
    assert_eq!(window.acked_through_seq, 1);
    assert_eq!(window.trimmed_through_seq, 1);
    assert!(!window.has_more);
}

#[test]
fn test_ack_events_is_monotonic_and_clamped_to_latest_sequence() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));

    let first_ack =
        expect_ok(runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 99));
    assert_eq!(first_ack.acked_through_seq, 1);
    assert_eq!(first_ack.trimmed_through_seq, 1);

    let second_ack =
        expect_ok(runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0));
    assert_eq!(second_ack.acked_through_seq, 1);
    assert_eq!(second_ack.trimmed_through_seq, 1);
}

#[test]
fn test_failed_ack_checkpoint_persistence_does_not_commit_runtime_ack_state() {
    let checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::new(false));
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_failed_ack"}"#.into(),
        vec!["d_pad".into()],
    ));

    checkpoint_store.fail_next_saves();
    let error = runtime
        .ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 1)
        .expect_err("checkpoint save failure should reject ack");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(
        window.items.len(),
        1,
        "failed ack persistence must not trim the in-memory delivery window"
    );
    assert_eq!(window.items[0].payload, r#"{"messageId":"msg_failed_ack"}"#);
    assert_eq!(window.acked_through_seq, 0);
    assert_eq!(window.trimmed_through_seq, 0);
}

#[test]
fn test_runtime_persists_checkpoint_and_recovers_on_rebuild() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    expect_ok(runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 1));

    let persisted = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint should be persisted");
    assert_eq!(persisted.latest_realtime_seq, 1);
    assert_eq!(persisted.acked_through_seq, 1);
    assert_eq!(persisted.trimmed_through_seq, 1);

    let rebuilt_runtime =
        RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(checkpoint_store);
    let restored = expect_ok(
        rebuilt_runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(restored.latest_realtime_seq, 1);
    assert_eq!(restored.acked_through_seq, 1);
    assert_eq!(restored.trimmed_through_seq, 1);

    expect_ok(rebuilt_runtime.sync_subscriptions_for_principal_kind(
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
    let delivered = expect_ok(rebuilt_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);

    let after_rebuild = expect_ok(
        rebuilt_runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(after_rebuild.latest_realtime_seq, 2);
}

#[test]
fn test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store.clone(),
        subscription_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store,
        subscription_store,
    );
    expect_ok(
        rebuilt_runtime
            .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );

    let delivered = expect_ok(rebuilt_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);
}

#[test]
fn test_runtime_restores_unacked_events_from_durable_window_store_after_rebuild() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_durable"}"#.into(),
        vec!["d_pad".into()],
    ));

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store,
        subscription_store,
        event_window_store.clone(),
    );
    let restored = expect_ok(
        rebuilt_runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );

    assert_eq!(restored.items.len(), 1);
    assert_eq!(restored.items[0].realtime_seq, 1);
    assert_eq!(restored.items[0].payload, r#"{"messageId":"msg_durable"}"#);
    assert_eq!(restored.acked_through_seq, 0);
    assert_eq!(restored.trimmed_through_seq, 0);

    expect_ok(
        rebuilt_runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 1),
    );
    let persisted_after_ack = event_window_store
        .window("100001", "default", "user", "1", "d_pad")
        .expect("durable window record should remain after ack for trim metadata");
    assert_eq!(persisted_after_ack.trimmed_through_seq, 1);
    assert!(
        persisted_after_ack.events.is_empty(),
        "acked realtime events should be trimmed from durable window"
    );
}

#[test]
fn test_publish_restores_persisted_subscriptions_for_registered_client_routes_after_rebuild() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store.clone(),
        subscription_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store,
        subscription_store,
    );

    let delivered = expect_ok(rebuilt_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_after_rebuild"}"#.into(),
        vec!["d_pad".into(), "d_pad".into()],
    ));
    assert_eq!(
        delivered, 1,
        "publish must not silently drop a registered client route whose durable subscription has not been lazily restored yet"
    );

    let window = expect_ok(
        rebuilt_runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 1);
    assert_eq!(
        window.items[0].payload,
        r#"{"messageId":"msg_after_rebuild"}"#
    );
}

#[test]
fn test_publish_does_not_restore_unmatched_registered_client_routes() {
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let seed_runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        subscription_store.clone(),
    );
    expect_ok(seed_runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_match",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(FailingLoadForDeviceCheckpointStore::new("d_unmatched")),
        subscription_store,
    );

    let delivered = expect_ok(rebuilt_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_matched_only"}"#.into(),
        vec!["d_match".into(), "d_unmatched".into()],
    ));
    assert_eq!(
        delivered, 1,
        "publish should not restore or fail on registered client routes that have no matching subscription"
    );

    let window = expect_ok(
        rebuilt_runtime
            .list_events_for_principal_kind("100001", "default", "1", "user", "d_match", 0, 10),
    );
    assert_eq!(window.items.len(), 1);
    assert_eq!(
        window.items[0].payload,
        r#"{"messageId":"msg_matched_only"}"#
    );
}

#[derive(Clone, Default)]
struct ToggleRealtimeCheckpointStore {
    fail_saves: Arc<AtomicBool>,
}

impl ToggleRealtimeCheckpointStore {
    fn new(fail_saves: bool) -> Self {
        Self {
            fail_saves: Arc::new(AtomicBool::new(fail_saves)),
        }
    }

    fn fail_next_saves(&self) {
        self.fail_saves.store(true, Ordering::SeqCst);
    }
}

impl RealtimeCheckpointStore for ToggleRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(None)
    }

    fn save_checkpoints(
        &self,
        _records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.fail_saves.load(Ordering::SeqCst) {
            Err(ContractError::Unavailable(
                "synthetic checkpoint save failure".into(),
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Default)]
struct FailAfterRealtimeCheckpointStore {
    remaining_successful_saves: Arc<AtomicUsize>,
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl FailAfterRealtimeCheckpointStore {
    fn new(successful_saves_before_failure: usize) -> Self {
        Self {
            remaining_successful_saves: Arc::new(AtomicUsize::new(successful_saves_before_failure)),
            checkpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn allow_successful_saves(&self, successful_saves_before_failure: usize) {
        self.remaining_successful_saves
            .store(successful_saves_before_failure, Ordering::SeqCst);
    }

    fn insert_checkpoint(&self, checkpoint: RealtimeCheckpointRecord) {
        self.checkpoints
            .lock()
            .expect("test checkpoint store should lock")
            .insert(checkpoint.device_id.clone(), checkpoint);
    }

    fn checkpoint(&self, device_id: &str) -> Option<RealtimeCheckpointRecord> {
        self.checkpoints
            .lock()
            .expect("test checkpoint store should lock")
            .get(device_id)
            .cloned()
    }
}

#[derive(Clone, Default)]
struct BlockingFirstDeviceCheckpointStore {
    gate: Arc<Mutex<Option<mpsc::Receiver<()>>>>,
}

impl BlockingFirstDeviceCheckpointStore {
    fn new(gate: mpsc::Receiver<()>) -> Self {
        Self {
            gate: Arc::new(Mutex::new(Some(gate))),
        }
    }
}

impl RealtimeCheckpointStore for BlockingFirstDeviceCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(None)
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if records.iter().any(|record| record.device_id == "d_blocked")
            && let Some(receiver) = self
                .gate
                .lock()
                .expect("test checkpoint gate should lock")
                .take()
        {
            receiver
                .recv()
                .expect("test should release blocked checkpoint save");
        }
        Ok(())
    }
}

#[derive(Clone)]
struct FailingLoadForDeviceCheckpointStore {
    failing_device_id: String,
}

impl FailingLoadForDeviceCheckpointStore {
    fn new(failing_device_id: &str) -> Self {
        Self {
            failing_device_id: failing_device_id.into(),
        }
    }
}

impl RealtimeCheckpointStore for FailingLoadForDeviceCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        if device_id == self.failing_device_id {
            return Err(ContractError::Unavailable(
                "synthetic unrelated checkpoint load failure".into(),
            ));
        }
        Ok(None)
    }

    fn save_checkpoints(
        &self,
        _records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        Ok(())
    }
}

impl RealtimeCheckpointStore for FailAfterRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(self.checkpoint(device_id))
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.remaining_successful_saves.load(Ordering::SeqCst) < records.len() {
            return Err(ContractError::Unavailable(
                "synthetic checkpoint save failure".into(),
            ));
        }
        self.remaining_successful_saves
            .fetch_sub(records.len(), Ordering::SeqCst);
        let mut checkpoints = self
            .checkpoints
            .lock()
            .expect("test checkpoint store should lock");
        for record in records {
            checkpoints.insert(record.device_id.clone(), record);
        }
        Ok(())
    }
}

struct ToggleRealtimeSubscriptionStore {
    fail_saves: Arc<AtomicBool>,
}

impl ToggleRealtimeSubscriptionStore {
    fn new(fail_saves: bool) -> Self {
        Self {
            fail_saves: Arc::new(AtomicBool::new(fail_saves)),
        }
    }

    fn fail_next_saves(&self) {
        self.fail_saves.store(true, Ordering::SeqCst);
    }
}

impl RealtimeSubscriptionStore for ToggleRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(None)
    }

    fn load_matching_subscriptions(
        &self,
        _query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn save_subscriptions(&self, _record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        if self.fail_saves.load(Ordering::SeqCst) {
            Err(ContractError::Unavailable(
                "synthetic subscription save failure".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn clear_subscriptions(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<bool, ContractError> {
        Err(ContractError::Unavailable(
            "synthetic subscription clear failure".into(),
        ))
    }

    fn clear_subscriptions_synced_at_or_before(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
        _cutoff_synced_at: &str,
    ) -> Result<bool, ContractError> {
        Err(ContractError::Unavailable(
            "synthetic subscription conditional clear failure".into(),
        ))
    }
}

#[test]
fn test_failed_subscription_persistence_does_not_install_runtime_subscription() {
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(ToggleRealtimeSubscriptionStore::new(true)),
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
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect_err("subscription store failure should reject the sync");
    assert_eq!(error.code, "subscription_store_unavailable");

    let delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_after_failed_sync"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(
        delivered, 0,
        "failed subscription persistence must not leave an in-memory fanout subscription"
    );
}

#[test]
fn test_failed_subscription_persistence_restores_previous_runtime_subscription() {
    let subscription_store = Arc::new(ToggleRealtimeSubscriptionStore::new(false));
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        subscription_store.clone(),
    );

    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_old".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    subscription_store.fail_next_saves();
    let error = runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_new".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect_err("replacement subscription persistence should fail");
    assert_eq!(error.code, "subscription_store_unavailable");

    let old_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_old",
        "message.posted",
        r#"{"messageId":"msg_old"}"#.into(),
        vec!["d_pad".into()],
    ));
    let new_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_new",
        "message.posted",
        r#"{"messageId":"msg_new"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(old_delivery, 1);
    assert_eq!(new_delivery, 0);
}

#[test]
fn test_failed_subscription_clear_preserves_runtime_subscription() {
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(ToggleRealtimeSubscriptionStore::new(false)),
    );

    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    let error = runtime
        .clear_client_route_subscriptions_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect_err("subscription store clear failure should reject the clear request");
    assert_eq!(error.code, "subscription_store_unavailable");

    let delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_after_failed_clear"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(
        delivered, 1,
        "failed subscription clear must preserve the in-memory fanout subscription"
    );
}

#[test]
fn test_failed_checkpoint_persistence_rolls_back_published_runtime_event() {
    let checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::new(false));
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store.clone(),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
    );

    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    checkpoint_store.fail_next_saves();
    let error = runtime
        .publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_failed_checkpoint"}"#.into(),
            vec!["d_pad".into()],
        )
        .expect_err("checkpoint save failure should reject publish");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(
        window.items.len(),
        0,
        "failed checkpoint persistence must not leave a visible in-memory event"
    );
    let checkpoint = expect_ok(
        runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(checkpoint.latest_realtime_seq, 0);
}

#[test]
fn test_runtime_realtime_inbox_diagnostics_tracks_unacked_and_trimmed_windows() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_diagnostics"}"#.into(),
        vec!["d_pad".into()],
    ));

    let diagnostics = expect_ok(runtime.realtime_inbox_diagnostics());
    assert_eq!(diagnostics.status, "degraded");
    assert_eq!(diagnostics.client_route_window_count, 1);
    assert_eq!(diagnostics.pending_event_count, 1);
    assert_eq!(diagnostics.max_client_route_window_event_count, 1);
    assert_eq!(diagnostics.client_route_window_capacity, 1000);
    assert_eq!(diagnostics.max_client_route_window_usage_permille, 1);
    assert_eq!(diagnostics.max_trimmed_through_seq, 0);
    assert_eq!(diagnostics.high_risk_windows.len(), 1);
    assert_eq!(diagnostics.high_risk_windows[0].tenant_id, "100001");
    assert_eq!(diagnostics.high_risk_windows[0].principal_kind, "user");
    assert_eq!(diagnostics.high_risk_windows[0].principal_id, "1");
    assert_eq!(diagnostics.high_risk_windows[0].device_id, "d_pad");
    assert_eq!(diagnostics.high_risk_windows[0].pending_event_count, 1);
    assert_eq!(diagnostics.high_risk_windows[0].usage_permille, 1);
    assert!(
        diagnostics.high_risk_windows[0]
            .oldest_pending_occurred_at
            .is_some(),
        "high-risk window should expose oldest pending timestamp without exposing event payload"
    );
    assert!(
        diagnostics.oldest_pending_occurred_at.is_some(),
        "oldest pending timestamp should be present while realtime inbox has backlog"
    );

    let ack =
        expect_ok(runtime.ack_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 1));
    assert_eq!(ack.acked_through_seq, 1);
    assert_eq!(ack.trimmed_through_seq, 1);

    let diagnostics_after_ack = expect_ok(runtime.realtime_inbox_diagnostics());
    assert_eq!(diagnostics_after_ack.status, "ok");
    assert_eq!(diagnostics_after_ack.client_route_window_count, 1);
    assert_eq!(diagnostics_after_ack.pending_event_count, 0);
    assert_eq!(diagnostics_after_ack.max_client_route_window_event_count, 0);
    assert_eq!(diagnostics_after_ack.client_route_window_capacity, 1000);
    assert_eq!(
        diagnostics_after_ack.max_client_route_window_usage_permille,
        0
    );
    assert_eq!(diagnostics_after_ack.max_trimmed_through_seq, 1);
    assert_eq!(diagnostics_after_ack.high_risk_windows.len(), 0);
    assert_eq!(diagnostics_after_ack.oldest_pending_occurred_at, None);
}

#[test]
fn test_failed_publish_checkpoint_persistence_rolls_back_durable_event_window() {
    let checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::new(false));
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
        event_window_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    checkpoint_store.fail_next_saves();
    let error = runtime
        .publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_failed"}"#.into(),
            vec!["d_pad".into()],
        )
        .expect_err("checkpoint store failure should reject publish");

    assert_eq!(error.code, "checkpoint_store_unavailable");
    let durable_window = event_window_store
        .window("100001", "default", "user", "1", "d_pad")
        .expect("durable window rollback should materialize an empty window");
    assert!(
        durable_window.events.is_empty(),
        "failed checkpoint persist must not leave durable events behind"
    );
}

#[test]
fn test_failed_multi_client_route_checkpoint_persistence_does_not_partially_commit_any_route() {
    let checkpoint_store = Arc::new(FailAfterRealtimeCheckpointStore::new(1));
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store.clone(),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
    );

    for device_id in ["d_pad", "d_phone"] {
        expect_ok(runtime.sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            device_id,
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));
    }

    let error = runtime
        .publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_failed_multi_checkpoint"}"#.into(),
            vec!["d_pad".into(), "d_phone".into()],
        )
        .expect_err("partial checkpoint persistence should reject the whole publish");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    assert!(
        checkpoint_store.checkpoint("d_pad").is_none(),
        "publish must not durably commit one device checkpoint when another device checkpoint fails"
    );
    assert!(
        checkpoint_store.checkpoint("d_phone").is_none(),
        "failed publish must not commit the failing device checkpoint"
    );

    for device_id in ["d_pad", "d_phone"] {
        let window = expect_ok(
            runtime.list_events_for_principal_kind("100001", "default", "1", "user", device_id, 0, 10),
        );
        assert_eq!(window.items.len(), 0);
    }
}

#[test]
fn test_unrelated_publish_is_not_blocked_by_checkpoint_save_for_different_device_scope() {
    let (release_blocked_save, blocked_save) = mpsc::channel();
    let runtime = Arc::new(RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(BlockingFirstDeviceCheckpointStore::new(blocked_save)),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
    ));

    for (principal_id, device_id, conversation_id) in [
        ("1136", "d_blocked", "c_blocked"),
        ("1140", "d_fast", "c_fast"),
    ] {
        expect_ok(runtime.sync_subscriptions_for_principal_kind(
            "100001",
            "default",
            principal_id,
            "user",
            device_id,
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: conversation_id.into(),
                event_types: vec!["message.posted".into()],
            }],
        ));
    }

    let blocked_runtime = runtime.clone();
    let blocked_publish = std::thread::spawn(move || {
        blocked_runtime.publish_scope_event_for_principal_kind(
            "100001",
            "default",
            "1136",
            "user",
            "conversation",
            "c_blocked",
            "message.posted",
            r#"{"messageId":"msg_blocked"}"#.into(),
            vec!["d_blocked".into()],
        )
    });

    sleep(Duration::from_millis(25));
    let fast_runtime = runtime.clone();
    let fast_publish = std::thread::spawn(move || {
        fast_runtime.publish_scope_event_for_principal_kind(
            "100001",
            "default",
            "1140",
            "user",
            "conversation",
            "c_fast",
            "message.posted",
            r#"{"messageId":"msg_fast"}"#.into(),
            vec!["d_fast".into()],
        )
    });
    let fast_result = fast_publish
        .join()
        .expect("fast publish thread should not panic");
    assert_eq!(
        expect_ok(fast_result),
        1,
        "checkpoint persistence for one device scope must not block unrelated realtime publish"
    );

    release_blocked_save
        .send(())
        .expect("blocked checkpoint save should still wait for release");
    assert_eq!(
        expect_ok(
            blocked_publish
                .join()
                .expect("blocked publish thread should not panic")
        ),
        1
    );
}

#[test]
fn test_sync_subscriptions_rejects_oversized_event_types_payload() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    let oversized_event_types = (0..300)
        .map(|index| format!("evt_{index:03}_{}", "x".repeat(64)))
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: oversized_event_types,
            }],
        )
        .expect_err("oversized eventTypes payload should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("eventTypes"),
        "error should point to eventTypes payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_rejects_too_many_subscription_items() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    let oversized_items = (0..300)
        .map(|index| RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: format!("c_{index:03}"),
            event_types: Vec::new(),
        })
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions_for_principal_kind("100001", "default", "1", "user", "d_pad", oversized_items)
        .expect_err("too many subscription items should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("items"),
        "error should point to items payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_rejects_oversized_total_subscription_payload() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    let oversized_items = (0..40)
        .map(|index| RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: format!("c_{index:03}_{}", "x".repeat(480)),
            event_types: (0..120)
                .map(|event_index| format!("evt_{event_index:02}_{}", "y".repeat(120)))
                .collect(),
        })
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions_for_principal_kind("100001", "default", "1", "user", "d_pad", oversized_items)
        .expect_err("oversized total subscription payload should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("items"),
        "error should point to total items payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_rejects_duplicate_scope_entries() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    let error = runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![
                RealtimeSubscriptionItemInput {
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_types: vec!["message.posted".into()],
                },
                RealtimeSubscriptionItemInput {
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_types: vec!["message.read".into()],
                },
            ],
        )
        .expect_err("duplicate scope subscriptions should be rejected");

    assert_eq!(error.code, "subscription_scope_duplicate");
    assert!(
        error.message.contains("12#conversation6#c_demo"),
        "duplicate scope error should identify the scope, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_does_not_collapse_delimiter_shaped_scope_segments() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    let snapshot = expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![
            RealtimeSubscriptionItemInput {
                scope_type: "conversation:a".into(),
                scope_id: "b".into(),
                event_types: vec!["message.left".into()],
            },
            RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "a:b".into(),
                event_types: vec!["message.right".into()],
            },
        ],
    ));
    assert_eq!(snapshot.items.len(), 2);

    let left_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation:a",
        "b",
        "message.left",
        r#"{"messageId":"msg_left"}"#.into(),
        vec!["d_pad".into()],
    ));
    let right_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "a:b",
        "message.right",
        r#"{"messageId":"msg_right"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(left_delivered, 1);
    assert_eq!(right_delivered, 1);

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 2);
    assert_eq!(window.items[0].scope_type, "conversation:a");
    assert_eq!(window.items[0].scope_id, "b");
    assert_eq!(window.items[1].scope_type, "conversation");
    assert_eq!(window.items[1].scope_id, "a:b");
}

#[test]
fn test_runtime_isolates_same_actor_id_across_principal_kinds() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_user".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "agent",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_agent".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let user_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_user",
        "message.posted",
        r#"{"messageId":"msg_user"}"#.into(),
        vec!["d_pad".into()],
    ));
    let agent_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "agent",
        "conversation",
        "c_agent",
        "message.posted",
        r#"{"messageId":"msg_agent"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(user_delivered, 1);
    assert_eq!(agent_delivered, 1);

    let user_window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    let agent_window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "agent", "d_pad", 0, 10),
    );

    assert_eq!(user_window.items.len(), 1);
    assert_eq!(user_window.items[0].scope_id, "c_user");
    assert_eq!(user_window.items[0].payload, r#"{"messageId":"msg_user"}"#);
    assert_eq!(agent_window.items.len(), 1);
    assert_eq!(agent_window.items[0].scope_id, "c_agent");
    assert_eq!(
        agent_window.items[0].payload,
        r#"{"messageId":"msg_agent"}"#
    );
}

#[test]
fn test_runtime_clamps_invalid_checkpoint_invariants_on_restore() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    checkpoint_store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 3,
            acked_through_seq: 9,
            trimmed_through_seq: 11,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-05T12:30:00Z".into(),
        })
        .expect("invalid checkpoint fixture should save");

    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );
    let restored = expect_ok(
        runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(restored.latest_realtime_seq, 3);
    assert_eq!(restored.acked_through_seq, 3);
    assert_eq!(restored.trimmed_through_seq, 3);

    let persisted = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint should exist after normalization");
    assert_eq!(persisted.latest_realtime_seq, 3);
    assert_eq!(persisted.acked_through_seq, 3);
    assert_eq!(persisted.trimmed_through_seq, 3);
}

#[test]
fn test_failed_checkpoint_normalization_restore_retries_durable_repair() {
    let checkpoint_store = Arc::new(FailAfterRealtimeCheckpointStore::new(0));
    checkpoint_store.insert_checkpoint(RealtimeCheckpointRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        latest_realtime_seq: 3,
        acked_through_seq: 9,
        trimmed_through_seq: 11,
        capacity_trimmed_event_count: 0,
        capacity_trimmed_through_seq: 0,
        last_capacity_trimmed_at: None,
        updated_at: "2026-04-05T12:30:00Z".into(),
    });

    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );

    let error = runtime
        .window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect_err("failed normalization persistence should reject restore");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    checkpoint_store.allow_successful_saves(1);
    let restored = expect_ok(
        runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(restored.latest_realtime_seq, 3);
    assert_eq!(restored.acked_through_seq, 3);
    assert_eq!(restored.trimmed_through_seq, 3);

    let persisted = checkpoint_store
        .checkpoint("d_pad")
        .expect("checkpoint should be repaired after retry");
    assert_eq!(persisted.latest_realtime_seq, 3);
    assert_eq!(
        persisted.acked_through_seq, 3,
        "failed normalization must not mark the device as restored before durable repair succeeds"
    );
    assert_eq!(persisted.trimmed_through_seq, 3);
}

#[test]
fn test_restore_client_route_state_clamps_invalid_checkpoint_fields_before_persist() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );

    expect_ok(runtime.restore_client_route_state(
        session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![],
            latest_realtime_seq: 4,
            acked_through_seq: 8,
            trimmed_through_seq: 12,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        },
    ));

    let restored = expect_ok(
        runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(restored.latest_realtime_seq, 4);
    assert_eq!(restored.acked_through_seq, 4);
    assert_eq!(restored.trimmed_through_seq, 4);

    let persisted = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint should exist after restore");
    assert_eq!(persisted.latest_realtime_seq, 4);
    assert_eq!(persisted.acked_through_seq, 4);
    assert_eq!(persisted.trimmed_through_seq, 4);
}

#[test]
fn test_restore_client_route_state_checkpoint_failure_does_not_install_runtime_state() {
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(ToggleRealtimeCheckpointStore::new(true)),
        subscription_store.clone(),
    );

    let error = runtime
        .restore_client_route_state(session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
                subscribed_at: "2026-04-05T12:30:00Z".into(),
            }],
            events: vec![realtime_event(
                "100001",
                "1",
                "d_pad",
                1,
                "msg_restored",
            )],
            latest_realtime_seq: 1,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        })
        .expect_err("checkpoint store failure should reject restore");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    let delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_after_failed_restore"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(
        delivered, 0,
        "failed restore must not install an in-memory fanout subscription"
    );

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(
        window.items.len(),
        0,
        "failed restore must not install in-memory realtime events"
    );
    assert_eq!(window.acked_through_seq, 0);
    assert_eq!(window.trimmed_through_seq, 0);
    assert!(
        subscription_store
            .subscriptions("100001", "default", "user", "1", "d_pad")
            .is_none(),
        "failed restore must not leave a durable subscription record for a state that was never installed"
    );
}

#[test]
fn test_restore_client_route_state_reports_subscription_compensation_failure() {
    let checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::new(true));
    let subscription_store = Arc::new(ToggleRealtimeSubscriptionStore::new(false));
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        checkpoint_store,
        subscription_store,
    );

    let error = runtime
        .restore_client_route_state(session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
                subscribed_at: "2026-04-05T12:30:00Z".into(),
            }],
            events: vec![],
            latest_realtime_seq: 0,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        })
        .expect_err("checkpoint failure plus subscription rollback failure should be explicit");
    assert_eq!(error.code, "realtime_state_compensation_failed");
    assert!(
        error.message.contains("checkpoint persist failed"),
        "message should include checkpoint failure: {}",
        error.message
    );
    assert!(
        error.message.contains("subscription compensation failed"),
        "message should include subscription rollback failure: {}",
        error.message
    );
}

#[test]
fn test_restore_client_route_state_checkpoint_failure_restores_previous_durable_subscription() {
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(ToggleRealtimeCheckpointStore::new(false)),
        subscription_store.clone(),
    );

    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_old".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    let failing_runtime = RealtimeDeliveryRuntime::with_stores_permissive_for_tests(
        Arc::new(ToggleRealtimeCheckpointStore::new(true)),
        subscription_store.clone(),
    );

    let error = failing_runtime
        .restore_client_route_state(session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_new".into(),
                event_types: vec!["message.posted".into()],
                subscribed_at: "2026-04-05T12:30:00Z".into(),
            }],
            events: vec![],
            latest_realtime_seq: 0,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        })
        .expect_err("checkpoint store failure should reject restore");
    assert_eq!(error.code, "checkpoint_store_unavailable");

    let durable_subscription = subscription_store
        .subscriptions("100001", "default", "user", "1", "d_pad")
        .expect("previous durable subscription should be restored after failed restore");
    assert_eq!(durable_subscription.items.len(), 1);
    assert_eq!(durable_subscription.items[0].scope_id, "c_old");
}

#[test]
fn test_restore_client_route_state_normalizes_event_order_for_monotonic_pagination() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    expect_ok(runtime.restore_client_route_state(
        session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("100001", "1", "d_pad", 3, "msg_3"),
                realtime_event("100001", "1", "d_pad", 1, "msg_1"),
                realtime_event("100001", "1", "d_pad", 2, "msg_2"),
            ],
            latest_realtime_seq: 3,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        },
    ));

    let first_page = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 2),
    );
    assert_eq!(
        first_page
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(first_page.next_after_seq, Some(2));
    assert!(first_page.has_more);

    let second_page = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 2, 2),
    );
    assert_eq!(
        second_page
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![3]
    );
    assert_eq!(second_page.next_after_seq, Some(3));
    assert!(!second_page.has_more);
}

#[test]
fn test_list_events_rejects_zero_limit_at_runtime_boundary() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(
        runtime.ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );

    let error = runtime
        .list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 0)
        .expect_err("zero realtime event window limit should be rejected");
    assert_eq!(error.code, "limit_invalid");
}

#[test]
fn test_restore_client_route_state_deduplicates_realtime_sequences() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    expect_ok(runtime.restore_client_route_state(
        session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("100001", "1", "d_pad", 1, "msg_1_first"),
                realtime_event("100001", "1", "d_pad", 1, "msg_1_duplicate"),
                realtime_event("100001", "1", "d_pad", 2, "msg_2"),
            ],
            latest_realtime_seq: 2,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        },
    ));

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(
        window
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(window.items.len(), 2);
}

#[test]
fn test_restore_client_route_state_discards_events_at_or_below_trimmed_boundary() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    expect_ok(runtime.restore_client_route_state(
        session_gateway::RealtimeClientRouteStateSnapshot {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("100001", "1", "d_pad", 1, "msg_1"),
                realtime_event("100001", "1", "d_pad", 2, "msg_2"),
                realtime_event("100001", "1", "d_pad", 3, "msg_3"),
            ],
            latest_realtime_seq: 3,
            acked_through_seq: 2,
            trimmed_through_seq: 2,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            disconnect_generation: 0,
        },
    ));

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(
        window
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![3]
    );
    assert_eq!(window.acked_through_seq, 2);
    assert_eq!(window.trimmed_through_seq, 2);
}

#[test]
fn test_take_restore_client_route_state_transfers_disconnect_generation() {
    let source_runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(
        source_runtime
            .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    expect_ok(
        source_runtime
            .signal_device_disconnect_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    expect_ok(
        source_runtime
            .signal_device_disconnect_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(
        expect_ok(
            source_runtime
                .disconnect_generation_for_principal_kind("100001", "default", "1", "user", "d_pad")
        ),
        2
    );

    let snapshot = expect_ok(
        source_runtime
            .take_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(
        expect_ok(
            source_runtime
                .disconnect_generation_for_principal_kind("100001", "default", "1", "user", "d_pad")
        ),
        0,
        "taking client route state must remove stale source disconnect generation state"
    );

    let target_runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(target_runtime.restore_client_route_state(snapshot));
    assert_eq!(
        expect_ok(
            target_runtime
                .disconnect_generation_for_principal_kind("100001", "default", "1", "user", "d_pad")
        ),
        2,
        "restored client route state must preserve disconnect signal generation"
    );
}

#[test]
fn test_sync_subscriptions_advances_sync_timestamps_between_calls() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();

    let first = expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_first".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    sleep(Duration::from_millis(5));

    let second = expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_second".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    assert_ne!(
        first.synced_at, second.synced_at,
        "separate subscription sync calls must not reuse a fixed synced_at timestamp"
    );
    assert_ne!(
        first.items[0].subscribed_at, second.items[0].subscribed_at,
        "separate subscription sync calls must not reuse a fixed subscribed_at timestamp"
    );
}

#[test]
fn test_clearing_client_route_subscriptions_stops_future_realtime_delivery() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    let first_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(first_delivery, 1);

    expect_ok(
        runtime.clear_client_route_subscriptions_for_principal_kind(
            "100001", "default", "1", "user", "d_pad",
        ),
    );

    let second_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(second_delivery, 0);

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].realtime_seq, 1);
    assert_eq!(window.items[0].payload, r#"{"messageId":"msg_demo_1"}"#);
}

#[test]
fn test_resyncing_client_route_subscriptions_removes_stale_scope_fanout_index() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_old".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_new".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let old_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_old",
        "message.posted",
        r#"{"messageId":"msg_old"}"#.into(),
        vec!["d_pad".into()],
    ));
    let new_delivery = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_new",
        "message.posted",
        r#"{"messageId":"msg_new"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(old_delivery, 0);
    assert_eq!(new_delivery, 1);
    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].scope_id, "c_new");
}

#[test]
fn test_restored_client_route_state_rebuilds_scope_fanout_index() {
    let source_runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(source_runtime.sync_subscriptions_for_principal_kind(
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

    let snapshot = expect_ok(
        source_runtime
            .take_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    let target_runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(target_runtime.restore_client_route_state(snapshot));

    let delivered = expect_ok(target_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_restored"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(delivered, 1);
    let window = expect_ok(
        target_runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].payload, r#"{"messageId":"msg_restored"}"#);
}

#[test]
fn test_take_client_route_state_clears_source_durable_subscriptions_before_lazy_restore() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let source_runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );
    let target_runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store,
        subscription_store.clone(),
        event_window_store,
    );

    expect_ok(source_runtime.sync_subscriptions_for_principal_kind(
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
    let snapshot = expect_ok(
        source_runtime
            .take_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    expect_ok(target_runtime.restore_client_route_state(snapshot));

    expect_ok(
        source_runtime
            .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    let source_delivery = expect_ok(source_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_source_after_take"}"#.into(),
        vec!["d_pad".into()],
    ));
    let target_delivery = expect_ok(target_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_target_after_restore"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(source_delivery, 0);
    assert_eq!(target_delivery, 1);
    assert!(
        subscription_store
            .load_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("durable subscription load should succeed")
            .is_some(),
        "target restore should own the durable subscription after migration"
    );
}

#[test]
fn test_sync_subscriptions_after_take_client_route_state_reclaims_migrated_out_scope() {
    let source_runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(source_runtime.sync_subscriptions_for_principal_kind(
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
    let _snapshot = expect_ok(
        source_runtime
            .take_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );

    expect_ok(source_runtime.sync_subscriptions_for_principal_kind(
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
    let delivered = expect_ok(source_runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_reclaimed"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(delivered, 1);
}

#[test]
fn test_publish_scope_event_advances_occurred_at_between_events() {
    let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));

    sleep(Duration::from_millis(5));

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
    );
    assert_eq!(window.items.len(), 2);
    assert_ne!(
        window.items[0].occurred_at, window.items[1].occurred_at,
        "separate realtime events must not reuse a fixed occurred_at timestamp"
    );
}

#[test]
fn test_checkpoint_updated_at_advances_after_new_persisted_mutation() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    let first = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("first checkpoint should be persisted");

    sleep(Duration::from_millis(5));

    expect_ok(runtime.publish_scope_event_for_principal_kind(
        "100001",
                "default",
                "1",
        "user",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    let second = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("second checkpoint should be persisted");

    assert_ne!(
        first.updated_at, second.updated_at,
        "separate persisted checkpoint writes must not reuse a fixed updated_at timestamp"
    );
}

#[test]
fn test_publish_scope_event_enforces_bounded_client_route_window_and_persists_trim_checkpoint() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(
        checkpoint_store.clone(),
    );
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
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

    for index in 1..=1100 {
        expect_ok(runtime.publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            format!(r#"{{"messageId":"msg_demo_{index}"}}"#),
            vec!["d_pad".into()],
        ));
    }

    let window = expect_ok(
        runtime.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 1000),
    );
    assert_eq!(window.items.len(), 1000);
    assert_eq!(window.items[0].realtime_seq, 101);
    assert_eq!(window.items[999].realtime_seq, 1100);
    assert_eq!(window.next_after_seq, Some(1100));
    assert_eq!(window.acked_through_seq, 0);
    assert_eq!(window.trimmed_through_seq, 100);
    assert!(!window.has_more);

    let checkpoint = expect_ok(
        runtime.window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad"),
    );
    assert_eq!(checkpoint.latest_realtime_seq, 1100);
    assert_eq!(checkpoint.acked_through_seq, 0);
    assert_eq!(checkpoint.trimmed_through_seq, 100);

    let persisted = checkpoint_store
        .checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("trimmed checkpoint should be persisted");
    assert_eq!(persisted.latest_realtime_seq, 1100);
    assert_eq!(persisted.acked_through_seq, 0);
    assert_eq!(persisted.trimmed_through_seq, 100);
    assert_eq!(persisted.capacity_trimmed_event_count, 100);
    assert_eq!(persisted.capacity_trimmed_through_seq, 100);
    assert!(
        persisted.last_capacity_trimmed_at.is_some(),
        "capacity-trimmed checkpoint should preserve its diagnostic timestamp"
    );

    let diagnostics = expect_ok(runtime.realtime_inbox_diagnostics());
    assert_eq!(diagnostics.status, "critical");
    assert_eq!(diagnostics.max_client_route_window_event_count, 1000);
    assert_eq!(diagnostics.client_route_window_capacity, 1000);
    assert_eq!(diagnostics.max_client_route_window_usage_permille, 1000);
    assert_eq!(diagnostics.capacity_trimmed_event_count, 100);
    assert_eq!(diagnostics.max_capacity_trimmed_through_seq, 100);
    assert_eq!(diagnostics.high_risk_windows.len(), 1);
    assert_eq!(
        diagnostics.high_risk_windows[0].capacity_trimmed_event_count,
        100
    );
    assert_eq!(
        diagnostics.high_risk_windows[0].capacity_trimmed_through_seq,
        100
    );
}

fn realtime_event(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    realtime_seq: u64,
    message_id: &str,
) -> RealtimeEvent {
    RealtimeEvent {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        realtime_seq,
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        event_type: "message.posted".into(),
        delivery_class: "ephemeral".into(),
        payload: format!(r#"{{"messageId":"{message_id}"}}"#),
        occurred_at: "2026-04-05T12:30:00Z".into(),
    }
}
