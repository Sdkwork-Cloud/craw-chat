mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{API_SURFACE, route_manifest};
pub use paths::PREFIX;

use axum::Router;
use conversation_runtime::http::{
    AppState, apply_public_http_guardrails, bootstrap_conversation_app_state_from_env,
    default_app_state,
};

pub async fn build_public_app() -> Router {
    gateway_mount_with_state(default_app_state())
        .await
        .expect("development knowledgebase app-api state should mount")
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub async fn gateway_mount() -> Result<Router, String> {
    let state = bootstrap_conversation_app_state_from_env()?;
    gateway_mount_with_state(state).await
}

/// Mount with the state created by the application assembly. This keeps the
/// Chat Open API and the group knowledgebase App API on one authoritative
/// Conversation runtime instance in embedded deployments.
pub async fn gateway_mount_with_state(state: AppState) -> Result<Router, String> {
    state
        .ensure_group_knowledgebase_outbox_relay_started()
        .await
        .map_err(|error| {
            format!(
                "conversation knowledgebase app-api group knowledgebase relay readiness failed: {error}"
            )
        })?;
    Ok(web_bootstrap::wrap_router(apply_public_http_guardrails(
        routes::build_api_router(state),
    )))
}
