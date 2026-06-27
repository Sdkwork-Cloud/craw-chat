mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;

pub fn build_public_app(state: space_service::http::AppState) -> Router {
    web_bootstrap::wrap_router(routes::build_api_router(state))
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount(state: space_service::http::AppState) -> axum::Router {
    build_public_app(state)
}
