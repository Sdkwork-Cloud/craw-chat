//! White-box unit tests for projection observability.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "observability_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use std::panic::{self, AssertUnwindSafe};

fn poison_mutex<T>(mutex: &std::sync::Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

#[test]
fn test_projection_plane_observability_recovers_from_poisoned_lock() {
    let projection = TimelineProjectionService::default();
    poison_mutex(&projection.observability);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.projection_plane_observability()
    }));
    assert!(
        result.is_ok(),
        "projection_plane_observability should not panic when observability lock is poisoned"
    );
}

#[test]
fn test_record_projection_update_delay_recovers_from_poisoned_lock() {
    let projection = TimelineProjectionService::default();
    poison_mutex(&projection.observability);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.record_projection_update_delay("message.posted", "100001:c_demo", 10, 20)
    }));
    assert!(
        result.is_ok(),
        "record_projection_update_delay should not panic when observability lock is poisoned"
    );
}
