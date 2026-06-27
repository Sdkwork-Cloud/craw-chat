//! White-box unit tests for cluster disconnect fence.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "disconnect_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use std::panic::{self, AssertUnwindSafe};

fn poison_mutex<T>(mutex: &Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

#[test]
fn test_disconnect_fence_store_load_recovers_from_poisoned_lock() {
    let store = ClusterMemoryDisconnectFenceStore::default();
    poison_mutex(&store.fences);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        store.load_fence("100001", "default", "user", "1", "d_demo")
    }));
    assert!(
        result.is_ok(),
        "disconnect fence store load should not panic when lock is poisoned"
    );
    let load_result = result.expect("panic status should be captured");
    assert!(load_result.is_ok());
}

#[test]
fn test_mark_client_route_disconnected_recovers_from_poisoned_disconnect_cache_lock() {
    let cluster = RealtimeClusterBridge::default();
    cluster.bind_node_runtime(
        "node_a",
        std::sync::Arc::new(crate::RealtimeDeliveryRuntime::default()),
    );
    poison_mutex(&cluster.disconnect_fences);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        cluster.mark_client_route_disconnected_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_demo",
            Some("s_demo"),
            "node_a",
        )
    }));
    assert!(
        result.is_ok(),
        "mark_client_route_disconnected should not panic when disconnect cache lock is poisoned"
    );
    let mark_result = result.expect("panic status should be captured");
    assert!(mark_result.is_ok());
}
