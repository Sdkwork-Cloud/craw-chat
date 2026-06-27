use axum::extract::{Extension, State};
use axum::http::HeaderMap;
use axum::Json;
use im_app_context::AppContext;

use crate::client_route_state::ClientRouteStateSnapshot;
use crate::{resolve_request_app_context, resolve_requested_device_id, ApiError, AppState, PresenceHeartbeatRequest};

fn client_route_state_snapshot(
    state: &AppState,
    auth: &AppContext,
    requested_device_id: Option<&str>,
) -> Result<ClientRouteStateSnapshot, ApiError> {
    state.client_route_state_snapshot(auth, requested_device_id)
}

/// Converts a `tokio::task::JoinError` into an `ApiError` so presence
/// handlers surface a clean 500 instead of a runtime-level panic.
fn join_error_to_api_error(join_error: tokio::task::JoinError) -> ApiError {
    ApiError::internal(
        "presence_blocking_join_failed",
        join_error.to_string(),
    )
}

pub(crate) async fn get_presence_me(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::presence::PresenceSnapshotView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let requested_device_id = auth.device_id.clone();

    // Route state snapshot + presence snapshot both perform blocking IO on the
    // route store and presence runtime. Run on the blocking thread pool.
    let blocking_state = state.clone();
    let blocking_auth = auth.clone();
    let snapshot = tokio::task::spawn_blocking(
        move || -> Result<im_domain_core::presence::PresenceSnapshotView, ApiError> {
            let sync_state = client_route_state_snapshot(
                &blocking_state,
                &blocking_auth,
                requested_device_id.as_deref(),
            )?;
            blocking_state
                .presence_runtime
                .presence_snapshot(
                    &blocking_auth,
                    blocking_auth.device_id.clone(),
                    sync_state.registered_route_keys,
                )
                .map_err(ApiError::from)
        },
    )
    .await
    .map_err(join_error_to_api_error)??;

    Ok(Json(snapshot))
}

pub(crate) async fn heartbeat_presence(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceHeartbeatRequest>,
) -> Result<Json<im_domain_core::presence::PresenceSnapshotView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;

    let blocking_state = state.clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.clone();
    let snapshot = tokio::task::spawn_blocking(
        move || -> Result<im_domain_core::presence::PresenceSnapshotView, ApiError> {
            blocking_state.prepare_active_client_route(
                &blocking_auth,
                blocking_device_id.as_str(),
                "http",
                false,
            )?;
            let sync_state = client_route_state_snapshot(
                &blocking_state,
                &blocking_auth,
                Some(blocking_device_id.as_str()),
            )?;
            blocking_state
                .presence_runtime
                .heartbeat(
                    &blocking_auth,
                    blocking_device_id,
                    sync_state.latest_sync_seq,
                    sync_state.registered_route_keys,
                )
                .map_err(ApiError::from)
        },
    )
    .await
    .map_err(join_error_to_api_error)??;

    Ok(Json(snapshot))
}
