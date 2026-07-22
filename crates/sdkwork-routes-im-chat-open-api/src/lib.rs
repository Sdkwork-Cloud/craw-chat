mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{API_SURFACE, route_manifest};
pub use paths::PREFIX;
pub use routes::{build_api_router, build_api_router_with_query_service};

use axum::Router;
use conversation_runtime::http::{
    PrincipalDirectory, app_state_with_principal_directory, apply_public_http_guardrails,
    default_app_state,
};
use std::sync::Arc;

pub fn build_public_app() -> Router {
    web_bootstrap::wrap_router(apply_public_http_guardrails(routes::build_api_router(
        default_app_state(),
    )))
}

pub async fn build_public_app_from_env() -> Router {
    web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(routes::build_api_router(
        default_app_state(),
    )))
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
    web_bootstrap::wrap_router(apply_public_http_guardrails(routes::build_api_router(
        app_state_with_principal_directory(principal_directory),
    )))
}

pub async fn build_public_app_with_principal_directory_from_env(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(routes::build_api_router(
        app_state_with_principal_directory(principal_directory),
    )))
    .await
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub async fn gateway_mount() -> Result<axum::Router, String> {
    let state = conversation_runtime::http::bootstrap_conversation_app_state_from_env()?;
    gateway_mount_with_state(state).await
}

/// Mount with the state created by the application assembly. This avoids
/// split in-memory Conversation state when another route crate exposes a
/// companion capability for the same aggregate.
pub async fn gateway_mount_with_state(
    state: conversation_runtime::http::AppState,
) -> Result<axum::Router, String> {
    state
        .ensure_group_knowledgebase_outbox_relay_started()
        .await
        .map_err(|error| {
            format!(
                "conversation chat open-api group knowledgebase relay readiness failed: {error}"
            )
        })?;
    Ok(
        web_bootstrap::wrap_router_from_env(apply_public_http_guardrails(
            routes::build_api_router(state),
        ))
        .await,
    )
}
