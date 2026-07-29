use std::sync::Arc;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use sdkwork_web_core::WebRequestContext;
use tokio::sync::Semaphore;

use crate::bootstrap::default_portal_runtime;
use crate::error::PortalError;
use crate::handlers::{get_portal_access_snapshot, get_portal_snapshot, get_portal_workspace};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, PortalRuntime, PublicAppGuardrails};

const PORTAL_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_PORTAL_MAX_IN_FLIGHT_REQUESTS";
const PORTAL_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 64;
const PORTAL_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 256;

pub fn default_app_state() -> AppState {
    crate::bootstrap::default_app_state()
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/app/v3/api/portal/access", get(get_portal_access_snapshot))
        .route("/app/v3/api/portal/automation", get(get_portal_snapshot))
        .route("/app/v3/api/portal/conversations", get(get_portal_snapshot))
        .route("/app/v3/api/portal/dashboard", get(get_portal_snapshot))
        .route("/app/v3/api/portal/governance", get(get_portal_snapshot))
        .route("/app/v3/api/portal/home", get(get_portal_snapshot))
        .route("/app/v3/api/portal/media", get(get_portal_snapshot))
        .route("/app/v3/api/portal/realtime", get(get_portal_snapshot))
        .route("/app/v3/api/portal/workspace", get(get_portal_workspace))
        .with_state(state)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(PORTAL_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(PORTAL_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(PORTAL_MAX_IN_FLIGHT_REQUESTS_MAX)
}

pub fn build_public_app() -> Router {
    let runtime = default_portal_runtime();
    build_public_app_from_api_router(apply_public_http_guardrails(build_domain_api_router(
        AppState::new(runtime),
    )))
}

pub fn build_app(runtime: Arc<PortalRuntime>) -> Router {
    build_public_app_from_api_router(build_domain_api_router(AppState::new(runtime)))
}

pub fn build_default_app() -> Router {
    build_app(default_portal_runtime())
}

pub fn build_public_app_from_api_router(api_router: Router) -> Router {
    mount_im_infra_routes(build_service_router(api_router), im_service_router_config())
}

fn build_service_router(api_router: Router) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(api_router)
}

async fn enforce_in_flight_gate(
    axum::extract::State(guardrails): axum::extract::State<PublicAppGuardrails>,
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
            let problem = sdkwork_routes_web_framework_backend_api::response::ApiProblem::dependency_unavailable(
                "portal service is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return PortalError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message:
                    "portal service is at maximum in-flight request capacity, please retry later"
                        .to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}
