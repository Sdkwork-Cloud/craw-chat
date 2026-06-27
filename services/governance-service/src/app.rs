use std::sync::Arc;

use audit_service::AuditRuntime;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use im_platform_contracts::{ProviderRegistry, RuntimeProviderRegistry};
use ops_service::OpsRuntime;
use sdkwork_im_ccp_registry::CcpRegistry;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use session_gateway::RealtimeClusterBridge;
use tokio::sync::Semaphore;

use crate::error::ControlPlaneError;
use crate::handlers::{
    activate_node, drain_node, docs, migrate_node_routes, openapi_document,
    protocol_governance_snapshot, protocol_registry_snapshot, provider_bindings_snapshot,
    provider_policy_diff, provider_policy_history, provider_policy_preview,
    provider_registry_snapshot, rollback_provider_policy, upsert_provider_binding_policy,
};
use crate::state::{
    AppState, GovernanceLoop, PublicAppGuardrails, CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT,
    CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV, CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX,
    CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT, CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV,
    CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX,
};

pub fn build_app() -> Router {
    build_app_with_cluster(Arc::new(RealtimeClusterBridge::default()))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router_with_cluster(Arc::new(
            RealtimeClusterBridge::default(),
        ))),
        im_service_router_config(),
    )
}

pub fn default_control_state() -> AppState {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    AppState {
        realtime_cluster: Arc::new(RealtimeClusterBridge::default()),
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    }
}

pub fn build_domain_api_router(state: AppState) -> Router {
    build_control_surface_with_state(state)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_runtime_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_control_surface_with_cluster_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_app_with_cluster_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

fn build_business_router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_document))
        .route(
            "/backend/v3/api/control/openapi.json",
            get(openapi_document),
        )
        .route("/docs", get(docs))
        .merge(build_control_surface_with_state(state))
}

fn build_business_router_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_business_router_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    })
}

fn build_app_with_state(state: AppState) -> Router {
    mount_im_infra_routes(
        build_business_router_with_state(state),
        im_service_router_config(),
    )
}

fn build_control_surface_with_state(state: AppState) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/control/protocol_registry",
            get(protocol_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/protocol_governance",
            get(protocol_governance_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_registry",
            get(provider_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_bindings",
            get(provider_bindings_snapshot).post(upsert_provider_binding_policy),
        )
        .route(
            "/backend/v3/api/control/provider_policies",
            get(provider_policy_history),
        )
        .route(
            "/backend/v3/api/control/provider_policies/diff",
            get(provider_policy_diff),
        )
        .route(
            "/backend/v3/api/control/provider_policies/preview",
            post(provider_policy_preview),
        )
        .route(
            "/backend/v3/api/control/provider_policies/rollback",
            post(rollback_provider_policy),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/drain",
            post(drain_node),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/activate",
            post(activate_node),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/routes/migrate",
            post(migrate_node_routes),
        )
        .with_state(state)
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json"
            | "/backend/v3/api/control/openapi.json" | "/docs" => {
            next.run(request).await
        }
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return ControlPlaneError::service_unavailable(
                        "http_overloaded",
                        "server is at maximum in-flight request capacity, please retry later",
                    )
                    .into_response();
                }
            };
            let response = next.run(request).await;
            drop(permit);
            response
        }
    }
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX)
}
