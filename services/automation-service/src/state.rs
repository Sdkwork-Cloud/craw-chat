//! Application state wrappers for the automation service.

use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::runtime::AutomationRuntime;

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<AutomationRuntime>,
}

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<Semaphore>,
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(AutomationRuntime::default()),
    }
}
