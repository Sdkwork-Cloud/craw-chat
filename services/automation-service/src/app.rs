//! App builders, router composition, and HTTP guardrail middleware.

use std::sync::Arc;

use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{Request, header::CONTENT_TYPE};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use tokio::sync::Semaphore;

use sdkwork_im_web_bootstrap::{
    im_service_http_metrics, im_service_router_config, mount_im_infra_routes,
};
use sdkwork_web_core::WebRequestContext;

use crate::error::AutomationError;
use crate::handlers::*;
use crate::helpers::{resolve_max_http_request_body_bytes, resolve_max_in_flight_requests};
use crate::runtime::AutomationRuntime;
use crate::state::{AppState, PublicAppGuardrails};

pub fn build_default_app() -> Router {
    build_app(crate::bootstrap::default_automation_runtime())
}

pub fn build_domain_api_router(state: AppState) -> Router {
    build_app_api_router(state.clone()).merge(build_backend_api_router(state))
}

pub fn build_app_api_router(state: AppState) -> Router {
    Router::new()
        .route("/app/v3/api/automation/executions", post(request_execution))
        .route(
            "/app/v3/api/automation/agent_responses",
            post(start_agent_response),
        )
        .route(
            "/app/v3/api/automation/agent_responses/{stream_id}/frames",
            post(append_agent_response_delta),
        )
        .route(
            "/app/v3/api/automation/agent_responses/{stream_id}/complete",
            post(complete_agent_response),
        )
        .route(
            "/app/v3/api/automation/agent_tool_calls",
            post(request_agent_tool_call),
        )
        .route(
            "/app/v3/api/automation/executions/{execution_id}/agent_tool_calls/{tool_call_id}/complete",
            post(complete_agent_tool_call),
        )
        .route(
            "/app/v3/api/automation/executions/{execution_id}",
            get(get_execution),
        )
        .with_state(state)
}

pub fn build_backend_api_router(state: AppState) -> Router {
    Router::new()
        .route("/backend/v3/api/automation/governance", get(get_governance))
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
    let runtime = crate::bootstrap::default_automation_runtime();
    let api_router =
        apply_public_http_guardrails(build_domain_api_router(AppState::new(runtime.clone())));
    build_public_app_from_api_router(runtime, api_router)
}

pub fn build_app(runtime: Arc<AutomationRuntime>) -> Router {
    let api_router = build_domain_api_router(AppState::new(runtime.clone()));
    build_public_app_from_api_router(runtime, api_router)
}

pub fn build_business_router(runtime: Arc<AutomationRuntime>) -> Router {
    build_service_router(
        runtime.clone(),
        build_domain_api_router(AppState::new(runtime)),
    )
}

pub fn build_public_app_from_api_router(
    runtime: Arc<AutomationRuntime>,
    api_router: Router,
) -> Router {
    mount_automation_infra_routes(build_service_router(runtime, api_router))
}

fn build_service_router(runtime: Arc<AutomationRuntime>, api_router: Router) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route(
            "/metrics",
            get(move || {
                let runtime = runtime.clone();
                async move {
                    let mut body = im_service_http_metrics().render_prometheus();
                    body.push_str(&runtime.render_runtime_metrics_prometheus());
                    (
                        [(CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
                        body,
                    )
                }
            }),
        )
        .merge(api_router)
}

fn mount_automation_infra_routes(router: Router) -> Router {
    mount_im_infra_routes(router, im_service_router_config().skip_metrics())
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
            let problem = sdkwork_routes_web_framework_backend_api::response::ApiProblem::dependency_unavailable(
                "server is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return AutomationError {
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
