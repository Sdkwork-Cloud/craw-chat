//! IM social backend-api control routes.

mod manifest;
mod routes;
mod web_bootstrap;

pub use manifest::{backend_route_manifest, backend_routes};
pub use routes::build_control_app;
pub use web_bootstrap::wrap_router;

use std::sync::Arc;

use axum::Router;
use social_service::SocialRuntime;

pub fn build_control_public_app(social_runtime: Arc<SocialRuntime>) -> Router {
    web_bootstrap::wrap_router(routes::build_control_public_router(social_runtime))
}

/// Embedded gateway assembly: domain routes only; outer gateway owns infra probes.
pub fn build_control_embedded_public_app(social_runtime: Arc<SocialRuntime>) -> Router {
    web_bootstrap::wrap_router(routes::build_control_app(social_service::friendship::AppState {
        social_runtime,
    }))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    backend_route_manifest()
}

pub fn gateway_mount(social_runtime: std::sync::Arc<social_service::SocialRuntime>) -> axum::Router {
    build_control_public_app(social_runtime)
}
