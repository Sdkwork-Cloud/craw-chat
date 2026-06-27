use std::sync::Arc;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use tokio::sync::Semaphore;

use crate::error::NotificationError;
use crate::handlers::{get_notification, list_notifications, request_notification};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, NotificationRuntime, PublicAppGuardrails};

const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_NOTIFICATION_MAX_IN_FLIGHT_REQUESTS";
const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_NOTIFICATION_MAX_REQUEST_BODY_BYTES";
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(NotificationRuntime::default()),
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(NotificationRuntime::default()))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/app/v3/api/notifications/requests",
            post(request_notification),
        )
        .route("/app/v3/api/notifications", get(list_notifications))
        .route(
            "/app/v3/api/notifications/{notification_id}",
            get(get_notification),
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
        apply_public_http_guardrails(build_business_router(Arc::new(
            NotificationRuntime::default(),
        ))),
        im_service_router_config(),
    )
}

pub fn build_app(runtime: Arc<NotificationRuntime>) -> Router {
    mount_im_infra_routes(build_business_router(runtime), im_service_router_config())
}

fn build_business_router(runtime: Arc<NotificationRuntime>) -> Router {
    let state = AppState { runtime };
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state))
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
            return NotificationError {
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
    std::env::var(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(NOTIFICATION_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(NOTIFICATION_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(NOTIFICATION_MAX_REQUEST_BODY_BYTES_MAX)
}
