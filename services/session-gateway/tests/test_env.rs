use std::sync::Mutex;

static TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

/// Pins `SDKWORK_IM_ENVIRONMENT=dev` for integration tests that use dual-token
/// header fallback without an IAM database pool.
///
/// Call before building the app or spawning servers in session-gateway integration
/// tests that authenticate over HTTP/WebSocket without an IAM database pool.
pub struct DevTestEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

pub fn dev_test_environment() -> DevTestEnvironment {
    let guard = TEST_ENV_LOCK.lock().expect("session-gateway test env lock");
    // SAFETY: integration tests run serially under the mutex guard.
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
    }
    DevTestEnvironment { _guard: guard }
}
