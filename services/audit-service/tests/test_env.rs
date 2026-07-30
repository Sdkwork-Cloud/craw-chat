use std::sync::Mutex;

static TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

/// Pins `SDKWORK_IM_ENVIRONMENT=dev` for integration tests that exercise
/// `AuditRuntime::from_env()` without a live PostgreSQL pool.
///
/// `audit-service` fails closed in production without
/// `SDKWORK_DATABASE_URL` (see `lib.rs` `resolve_audit_backend`). Call
/// before building the app so the in-memory audit ledger is selected.
pub struct DevTestEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

pub fn dev_test_environment() -> DevTestEnvironment {
    let guard = TEST_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    // SAFETY: integration tests run serially under the mutex guard.
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
    }
    DevTestEnvironment { _guard: guard }
}
