use std::sync::Arc;

use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use tokio::sync::Semaphore;

use crate::error::OpsError;
use crate::handlers::{
    get_cluster, get_diagnostics, get_lag, get_ops_health, get_provider_binding_drift,
    get_provider_bindings, get_replay_status, get_runtime_dir, post_retention_purge,
};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, OpsRuntime, PublicAppGuardrails};

const OPS_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_OPS_MAX_IN_FLIGHT_REQUESTS";
const OPS_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const OPS_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const OPS_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_OPS_MAX_REQUEST_BODY_BYTES";
const OPS_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const OPS_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(OpsRuntime::default()),
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(OpsRuntime::default()))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/backend/v3/api/ops/health", get(get_ops_health))
        .route("/backend/v3/api/ops/cluster", get(get_cluster))
        .route("/backend/v3/api/ops/lag", get(get_lag))
        .route("/backend/v3/api/ops/replay_status", get(get_replay_status))
        .route("/backend/v3/api/ops/runtime_dir", get(get_runtime_dir))
        .route(
            "/backend/v3/api/ops/provider_bindings",
            get(get_provider_bindings),
        )
        .route(
            "/backend/v3/api/ops/provider_bindings/drift",
            get(get_provider_binding_drift),
        )
        .route("/backend/v3/api/ops/diagnostics", get(get_diagnostics))
        .route(
            "/backend/v3/api/ops/retention/purge",
            post(post_retention_purge),
        )
        .with_state(state)
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

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(Arc::new(OpsRuntime::default()))),
        im_service_router_config(),
    )
}

pub fn build_app(runtime: Arc<OpsRuntime>) -> Router {
    mount_im_infra_routes(build_business_router(runtime), im_service_router_config())
}

fn build_business_router(runtime: Arc<OpsRuntime>) -> Router {
    let state = AppState { runtime };
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state))
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if matches!(
        request.uri().path(),
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json" | "/docs"
    ) {
        return next.run(request).await;
    }
    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return OpsError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message: "server is at maximum in-flight request capacity, please retry later"
                    .to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(OPS_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(OPS_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(OPS_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(OPS_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(OPS_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(OPS_MAX_REQUEST_BODY_BYTES_MAX)
}
