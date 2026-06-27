mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;
use conversation_runtime::http::{
    app_state_with_principal_directory, apply_public_http_guardrails, default_app_state,
    PrincipalDirectory,
};
use std::sync::Arc;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_api_router(default_app_state()),
    ))
}

pub async fn build_public_app_from_env() -> Router {
    web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(
        routes::build_api_router(default_app_state()),
    ))
    .await
}

pub fn build_public_app_with_allow_all_principals() -> Router {
    build_public_app()
}

pub async fn build_public_app_with_allow_all_principals_from_env() -> Router {
    build_public_app_from_env().await
}

pub fn build_public_app_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_api_router(app_state_with_principal_directory(principal_directory)),
    ))
}

pub async fn build_public_app_with_principal_directory_from_env(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(
        routes::build_api_router(app_state_with_principal_directory(principal_directory)),
    ))
    .await
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub async fn gateway_mount() -> axum::Router {
    build_public_app_from_env().await
}
