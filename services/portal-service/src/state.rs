use std::sync::Arc;

use audit_service::AuditRuntime;
use ops_service::OpsRuntime;

#[derive(Clone)]
pub struct PortalRuntime {
    pub ops: Arc<OpsRuntime>,
    pub audit: Arc<AuditRuntime>,
}

impl PortalRuntime {
    pub fn new(ops: Arc<OpsRuntime>, audit: Arc<AuditRuntime>) -> Self {
        Self { ops, audit }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub runtime: Arc<PortalRuntime>,
}

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<tokio::sync::Semaphore>,
}
