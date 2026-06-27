//! White-box unit tests for session-gateway presence runtime.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "presence_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use im_domain_core::presence::{PresenceClientView, PresenceStatus};

fn presence_record(device_id: &str, last_seen_at: &str) -> PresenceStateRecord {
    PresenceStateRecord {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: device_id.into(),
        presence: PresenceClientView {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            device_id: device_id.into(),
            platform: None,
            session_id: Some(format!("s_{device_id}")),
            status: PresenceStatus::Online,
            last_sync_seq: 0,
            last_resume_at: Some(last_seen_at.into()),
            last_seen_at: Some(last_seen_at.into()),
        },
        resume_required: false,
        updated_at: last_seen_at.into(),
    }
}

#[test]
fn test_presence_state_store_load_recovers_from_poisoned_lock() {
    let store = RuntimeMemoryPresenceStateStore::default();
    let _ = std::panic::catch_unwind({
        let state = store.state.clone();
        move || {
            let _guard = state.lock().expect("presence state store should lock");
            panic!("poison presence state store lock");
        }
    });

    let restored = store
        .load_state("100001", "default", "user", "1", "d_poison")
        .expect("poisoned lock should be recovered");
    assert!(restored.is_none());
}

#[test]
fn test_presence_state_store_seen_at_cutoff_compares_rfc3339_by_instant() {
    let store = RuntimeMemoryPresenceStateStore::default();
    store
        .save_state(presence_record(
            "d_later_fraction",
            "2026-05-06T00:00:00.100Z",
        ))
        .expect("later presence save should succeed");
    store
        .save_state(presence_record("d_whole_second", "2026-05-06T00:00:00Z"))
        .expect("whole-second presence save should succeed");

    let stale = store
        .list_online_states_seen_at_or_before("2026-05-06T00:00:00Z", 10)
        .expect("stale online list should succeed");

    assert_eq!(
        stale
            .iter()
            .map(|record| record.device_id.as_str())
            .collect::<Vec<_>>(),
        vec!["d_whole_second"]
    );
}

#[test]
fn test_presence_state_store_isolates_organizations_for_same_principal_and_device() {
    let store = RuntimeMemoryPresenceStateStore::default();
    let mut org_a = presence_record("d_pad", "2026-05-06T00:00:00.000Z");
    org_a.organization_id = "org_a".into();
    let mut org_b = presence_record("d_pad", "2026-05-06T00:00:01.000Z");
    org_b.organization_id = "org_b".into();
    let mut org_b_phone = presence_record("d_phone", "2026-05-06T00:00:02.000Z");
    org_b_phone.organization_id = "org_b".into();

    store
        .save_state(org_a)
        .expect("org_a presence save should succeed");
    store
        .save_state(org_b)
        .expect("org_b presence save should succeed");
    store
        .save_state(org_b_phone)
        .expect("org_b phone presence save should succeed");

    let listed_a = store
        .list_states_for_principal("100001", "org_a", "user", "1")
        .expect("org_a principal list should succeed");
    assert_eq!(listed_a.len(), 1);
    assert_eq!(listed_a[0].organization_id, "org_a");
    assert_eq!(
        listed_a[0].presence.last_seen_at.as_deref(),
        Some("2026-05-06T00:00:00.000Z")
    );

    let listed_b = store
        .list_states_for_principal("100001", "org_b", "user", "1")
        .expect("org_b principal list should succeed");
    assert_eq!(listed_b.len(), 2);
    assert!(listed_b.iter().all(|record| record.organization_id == "org_b"));

    assert!(
        store
            .load_state("100001", "org_a", "user", "1", "d_phone")
            .expect("org_a missing device load should succeed")
            .is_none(),
        "org_a scope must not expose devices only saved under org_b"
    );
}

#[test]
fn test_memory_presence_stores_share_principal_scope_key_shape() {
    use im_adapters_local_memory::MemoryPresenceStateStore;

    let runtime_store = RuntimeMemoryPresenceStateStore::default();
    let memory_store = MemoryPresenceStateStore::default();
    let record = presence_record("d_pad", "2026-05-06T00:00:00.000Z");

    runtime_store
        .save_state(record.clone())
        .expect("runtime presence save should succeed");
    memory_store
        .save_state(record)
        .expect("memory adapter presence save should succeed");

    let runtime_list = runtime_store
        .list_states_for_principal("100001", "default", "user", "1")
        .expect("runtime principal list should succeed");
    let memory_list = memory_store
        .list_states_for_principal("100001", "default", "user", "1")
        .expect("memory adapter principal list should succeed");

    assert_eq!(runtime_list.len(), 1);
    assert_eq!(memory_list.len(), 1);
    assert_eq!(runtime_list[0].device_id, memory_list[0].device_id);
}
