mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(notification_service::apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> axum::Router {
    build_public_app()
}
