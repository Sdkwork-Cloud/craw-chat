mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{API_SURFACE, route_manifest};
pub use paths::PREFIX;

use std::sync::Arc;

use axum::Router;

pub fn build_public_app() -> Router {
    build_public_app_with_runtime(portal_service::default_portal_runtime())
}

pub fn build_public_app_with_runtime(runtime: Arc<portal_service::PortalRuntime>) -> Router {
    portal_service::build_public_app_from_api_router(build_gateway_router_with_runtime(runtime))
}

fn build_gateway_router_with_runtime(runtime: Arc<portal_service::PortalRuntime>) -> Router {
    web_bootstrap::wrap_router(portal_service::apply_public_http_guardrails(
        routes::build_api_router_with_runtime(runtime),
    ))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> Router {
    gateway_mount_with_runtime(portal_service::default_portal_runtime())
}

pub fn gateway_mount_with_runtime(runtime: Arc<portal_service::PortalRuntime>) -> Router {
    build_gateway_router_with_runtime(runtime)
}
