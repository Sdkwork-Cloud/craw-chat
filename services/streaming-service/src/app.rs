use std::sync::Arc;

use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::Request;
use axum::http::header::CONTENT_TYPE;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sdkwork_im_web_bootstrap::{
    im_service_http_metrics, im_service_router_config, mount_im_infra_routes,
};
use sdkwork_web_bootstrap::{ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::WebRequestContext;
use tokio::sync::Semaphore;

use crate::bootstrap::default_streaming_runtime;

use crate::error::StreamingError;
use crate::handlers::{
    abort_stream, append_stream_frame, checkpoint_stream, complete_stream, list_stream_frames,
    open_stream,
};
use crate::helpers::{resolve_max_http_request_body_bytes, resolve_max_in_flight_requests};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, StreamingRuntime};

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Clone)]
struct StreamStoreReadiness {
    runtime: Arc<StreamingRuntime>,
}

impl ReadinessCheck for StreamStoreReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        let runtime = self.runtime.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || runtime.check_store_ready())
                .await
                .map_err(|error| format!("stream readiness task failed: {error}"))?
        })
    }
}

pub fn build_default_app() -> Router {
    build_app(default_streaming_runtime())
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/im/v3/api/streams", post(open_stream))
        .route(
            "/im/v3/api/streams/{stream_id}/frames",
            post(append_stream_frame).get(list_stream_frames),
        )
        .route(
            "/im/v3/api/streams/{stream_id}/checkpoint",
            post(checkpoint_stream),
        )
        .route(
            "/im/v3/api/streams/{stream_id}/complete",
            post(complete_stream),
        )
        .route("/im/v3/api/streams/{stream_id}/abort", post(abort_stream))
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
    let runtime = default_streaming_runtime();
    mount_stream_infra_routes(
        apply_public_http_guardrails(build_business_router(runtime.clone())),
        runtime,
    )
}

pub fn build_app(runtime: Arc<StreamingRuntime>) -> Router {
    mount_stream_infra_routes(build_business_router(runtime.clone()), runtime)
}

fn build_business_router(runtime: Arc<StreamingRuntime>) -> Router {
    let state = AppState { runtime };
    let metrics_runtime = state.runtime.clone();
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route(
            "/metrics",
            get(move || {
                let runtime = metrics_runtime.clone();
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
        .merge(build_domain_api_router(state))
}

fn mount_stream_infra_routes(router: Router, runtime: Arc<StreamingRuntime>) -> Router {
    let mut config = im_service_router_config();
    let mut readiness_checks = Vec::new();
    if let Some(environment_readiness) = config.readiness.take() {
        readiness_checks.push(environment_readiness);
    }
    readiness_checks.push(Arc::new(StreamStoreReadiness { runtime }));
    mount_im_infra_routes(
        router,
        config
            .skip_metrics()
            .with_composite_readiness(readiness_checks),
    )
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
            return StreamingError {
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
