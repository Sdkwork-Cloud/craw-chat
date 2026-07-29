use std::sync::Arc;

use axum::Router;

pub fn build_api_router_with_governance_sinks(
    realtime_cluster: Arc<session_gateway::RealtimeClusterBridge>,
    ops_runtime: Arc<ops_service::OpsRuntime>,
    audit_runtime: Arc<audit_service::AuditRuntime>,
) -> Router {
    governance_service::build_control_surface_with_cluster_and_governance_sinks(
        realtime_cluster,
        ops_runtime,
        audit_runtime,
    )
}
