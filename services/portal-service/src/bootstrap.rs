use std::sync::Arc;

use audit_service::AuditRuntime;
use ops_service::OpsRuntime;

use crate::state::{AppState, PortalRuntime};

pub fn default_portal_runtime() -> Arc<PortalRuntime> {
    Arc::new(PortalRuntime::new(
        Arc::new(OpsRuntime::from_env()),
        Arc::new(AuditRuntime::from_env()),
    ))
}

pub fn default_app_state() -> AppState {
    AppState::new(default_portal_runtime())
}
