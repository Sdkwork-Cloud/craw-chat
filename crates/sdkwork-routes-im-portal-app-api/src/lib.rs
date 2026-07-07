mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{API_SURFACE, route_manifest};
pub use paths::PREFIX;

use axum::Router;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(portal_service::apply_public_http_guardrails(
        routes::build_api_router(),
    ))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> Router {
    build_public_app()
}
