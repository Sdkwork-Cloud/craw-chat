use std::sync::Arc;

use audit_service::AuditRuntime;
use im_platform_contracts::{ProviderRegistry, RuntimeProviderRegistry};
use ops_service::OpsRuntime;
use sdkwork_im_ccp_registry::CcpRegistry;
use session_gateway::RealtimeClusterBridge;
use tokio::sync::Semaphore;

pub(crate) const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS";
pub(crate) const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
pub(crate) const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
pub(crate) const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_CONTROL_PLANE_MAX_REQUEST_BODY_BYTES";
pub(crate) const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
pub(crate) const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<Semaphore>,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) realtime_cluster: Arc<RealtimeClusterBridge>,
    pub(crate) protocol_registry: Arc<CcpRegistry>,
    pub(crate) provider_registry: Arc<dyn ProviderRegistry>,
    pub(crate) provider_registry_runtime: Option<Arc<RuntimeProviderRegistry>>,
    pub(crate) governance_loop: Option<GovernanceLoop>,
}

#[derive(Clone)]
pub(crate) struct GovernanceLoop {
    pub(crate) ops_runtime: Arc<OpsRuntime>,
    pub(crate) audit_runtime: Arc<AuditRuntime>,
}
