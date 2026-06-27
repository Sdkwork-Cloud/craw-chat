//! White-box unit tests for realtime durable stores.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "storage_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use im_domain_core::realtime::RealtimeSubscription;

#[test]
fn test_runtime_checkpoint_store_load_recovers_from_poisoned_lock() {
    let store = RuntimeMemoryCheckpointStore::default();
    let _ = std::panic::catch_unwind({
        let checkpoints = store.checkpoints.clone();
        move || {
            let _guard = checkpoints
                .lock()
                .expect("runtime checkpoint store should lock");
            panic!("poison runtime checkpoint store lock");
        }
    });

    let checkpoint = store
        .load_checkpoint("100001", "default", "user", "1", "d_poison")
        .expect("poisoned lock should be recovered");
    assert!(checkpoint.is_none());
}

#[test]
fn test_runtime_checkpoint_store_rejects_stale_regression_writes() {
    let store = RuntimeMemoryCheckpointStore::default();
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 9,
            acked_through_seq: 7,
            trimmed_through_seq: 6,
            capacity_trimmed_event_count: 3,
            capacity_trimmed_through_seq: 6,
            last_capacity_trimmed_at: Some("2026-05-06T00:00:02.000Z".into()),
            updated_at: "2026-05-06T00:00:02.000Z".into(),
        })
        .expect("new checkpoint save should succeed");
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 5,
            acked_through_seq: 4,
            trimmed_through_seq: 4,
            capacity_trimmed_event_count: 2,
            capacity_trimmed_through_seq: 4,
            last_capacity_trimmed_at: Some("2026-05-06T00:00:01.000Z".into()),
            updated_at: "2026-05-06T00:00:01.000Z".into(),
        })
        .expect("stale checkpoint save should not fail the caller");

    let checkpoint = store
        .load_checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint load should succeed")
        .expect("checkpoint should be present");
    assert_eq!(checkpoint.latest_realtime_seq, 9);
    assert_eq!(checkpoint.acked_through_seq, 7);
    assert_eq!(checkpoint.trimmed_through_seq, 6);
    assert_eq!(checkpoint.capacity_trimmed_event_count, 3);
    assert_eq!(checkpoint.capacity_trimmed_through_seq, 6);
    assert_eq!(
        checkpoint.last_capacity_trimmed_at.as_deref(),
        Some("2026-05-06T00:00:02.000Z")
    );
    assert_eq!(checkpoint.updated_at, "2026-05-06T00:00:02.000Z");
}

#[test]
fn test_runtime_subscription_store_does_not_clear_newer_subscription() {
    let store = RuntimeMemorySubscriptionStore::default();
    store
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: Vec::new(),
                subscribed_at: "2026-05-06T00:00:02.000Z".into(),
            }],
            synced_at: "2026-05-06T00:00:02.000Z".into(),
        })
        .expect("subscription save should succeed");

    let cleared = store
        .clear_subscriptions_synced_at_or_before(
            "100001",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:01.000Z",
        )
        .expect("conditional clear should succeed");

    assert!(!cleared);
    assert!(
        store
            .load_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("subscription load should succeed")
            .is_some(),
        "newer subscription must not be deleted by an older disconnect cleanup"
    );
}

#[test]
fn test_runtime_subscription_store_compares_synced_at_by_rfc3339_instant() {
    let store = RuntimeMemorySubscriptionStore::default();
    store
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: Vec::new(),
                subscribed_at: "2026-05-06T00:00:00.100Z".into(),
            }],
            synced_at: "2026-05-06T00:00:00.100Z".into(),
        })
        .expect("subscription save should succeed");

    let cleared = store
        .clear_subscriptions_synced_at_or_before(
            "100001",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:00Z",
        )
        .expect("conditional clear should succeed");

    assert!(
        !cleared,
        "a later fractional timestamp must not be cleared by an earlier whole-second cutoff"
    );
    assert!(
        store
            .load_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("subscription load should succeed")
            .is_some(),
        "subscription must remain after an earlier cutoff"
    );
}
