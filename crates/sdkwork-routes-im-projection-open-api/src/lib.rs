mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use std::sync::Arc;

use axum::Router;
use projection_service::http::apply_public_http_guardrails;
use projection_service::ProjectionRuntime;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}

pub fn build_supplemental_public_app() -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_supplemental_api_router(),
    ))
}

pub async fn build_public_app_with_runtime(runtime: Arc<ProjectionRuntime>) -> Router {
    web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(
        routes::build_api_router_with_service(runtime.service()),
    ))
    .await
}

pub fn build_public_app_with_service(service: Arc<projection_service::TimelineProjectionService>) -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_api_router_with_service(service),
    ))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> axum::Router {
    build_public_app()
}
