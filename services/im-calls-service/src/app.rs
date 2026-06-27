use std::sync::Arc;

use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use tokio::sync::Semaphore;

use crate::error::CallingError;
use crate::handlers::{
    accept_call_session, create_call_session, end_call_session, invite_call_session,
    issue_participant_credential, post_call_signal, reject_call_session, retrieve_call_session,
};
use crate::helpers::{resolve_max_http_request_body_bytes, resolve_max_in_flight_requests};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, CallingRuntime};

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(CallingRuntime::default()))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/im/v3/api/calls/sessions", post(create_call_session))
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}",
            get(retrieve_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/invite",
            post(invite_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/accept",
            post(accept_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/reject",
            post(reject_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/end",
            post(end_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/signals",
            post(post_call_signal),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/credentials",
            post(issue_participant_credential),
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
            CallingRuntime::default(),
        ))),
        im_service_router_config(),
    )
}

pub fn build_app(runtime: Arc<CallingRuntime>) -> Router {
    mount_im_infra_routes(
        build_business_router(runtime),
        im_service_router_config(),
    )
}

fn build_business_router(runtime: Arc<CallingRuntime>) -> Router {
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
            return CallingError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message:
                    "server is at maximum in-flight request capacity, please retry later".to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}