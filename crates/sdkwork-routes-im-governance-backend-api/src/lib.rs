mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{API_SURFACE, route_manifest};
pub use paths::PREFIX;

use std::sync::Arc;

use axum::Router;

pub fn build_public_app() -> Router {
    build_public_app_with_governance_sinks(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        Arc::new(ops_service::OpsRuntime::from_env()),
        Arc::new(audit_service::AuditRuntime::from_env()),
    )
}

pub fn build_public_app_with_governance_sinks(
    realtime_cluster: Arc<session_gateway::RealtimeClusterBridge>,
    ops_runtime: Arc<ops_service::OpsRuntime>,
    audit_runtime: Arc<audit_service::AuditRuntime>,
) -> Router {
    governance_service::build_public_app_from_api_router(
        build_gateway_router_with_governance_sinks(realtime_cluster, ops_runtime, audit_runtime),
    )
}

fn build_gateway_router_with_governance_sinks(
    realtime_cluster: Arc<session_gateway::RealtimeClusterBridge>,
    ops_runtime: Arc<ops_service::OpsRuntime>,
    audit_runtime: Arc<audit_service::AuditRuntime>,
) -> Router {
    web_bootstrap::wrap_router(governance_service::apply_public_http_guardrails(
        routes::build_api_router_with_governance_sinks(
            realtime_cluster,
            ops_runtime,
            audit_runtime,
        ),
    ))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> axum::Router {
    gateway_mount_with_governance_sinks(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        Arc::new(ops_service::OpsRuntime::from_env()),
        Arc::new(audit_service::AuditRuntime::from_env()),
    )
}

pub fn gateway_mount_with_governance_sinks(
    realtime_cluster: Arc<session_gateway::RealtimeClusterBridge>,
    ops_runtime: Arc<ops_service::OpsRuntime>,
    audit_runtime: Arc<audit_service::AuditRuntime>,
) -> axum::Router {
    build_gateway_router_with_governance_sinks(realtime_cluster, ops_runtime, audit_runtime)
}
