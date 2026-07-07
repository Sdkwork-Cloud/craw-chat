use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::{Extension, Json};
use im_app_context::AppContext;
use im_domain_core::realtime::{RealtimeAckState, RealtimeSubscriptionSnapshot};
use sdkwork_utils_rust::http_api::{SDKWORK_TRACE_ID_HEADER, SdkWorkApiResponse};

use crate::api_error::ApiError;
use crate::realtime::{
    self, AckRealtimeEventsRequest, ListRealtimeEventsQuery, RealtimeEventWindowQuery,
    SyncRealtimeSubscriptionsRequest,
};
use crate::realtime_list_page::{RealtimeEventsListData, realtime_events_list_from_window};
use crate::{AppState, resolve_request_app_context, resolve_requested_device_id};

/// Converts a `tokio::task::JoinError` (panic or cancellation of a
/// `spawn_blocking` task) into an `ApiError` so callers surface a clean 500
/// instead of a runtime-level panic propagating into the axum handler.
fn join_error_to_api_error(join_error: tokio::task::JoinError) -> ApiError {
    ApiError::internal("realtime_blocking_join_failed", join_error.to_string())
}

pub async fn sync_realtime_subscriptions(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    // Pure validation (no IO) — safe on the async worker thread.
    state
        .realtime_runtime
        .validate_subscriptions_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            &request.items,
        )?;

    // The prepare + sync chain performs blocking Redis/Postgres IO. Run it on
    // the dedicated blocking thread pool so tokio async workers are not
    // starved by synchronous database calls (tokio docs: "Use spawn_blocking
    // to run blocking operations").
    let blocking_state = state.clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.clone();
    let items = request.items;
    let snapshot =
        tokio::task::spawn_blocking(move || -> Result<RealtimeSubscriptionSnapshot, ApiError> {
            blocking_state.prepare_active_client_route(
                &blocking_auth,
                blocking_device_id.as_str(),
                "http",
                false,
            )?;
            blocking_state
                .realtime_runtime
                .sync_subscriptions_for_principal_kind(
                    blocking_auth.tenant_id.as_str(),
                    blocking_auth.organization_id.as_str(),
                    blocking_auth.actor_id.as_str(),
                    blocking_auth.actor_kind.as_str(),
                    blocking_device_id.as_str(),
                    items,
                )
                .map_err(ApiError::from)
        })
        .await
        .map_err(join_error_to_api_error)??;

    Ok(Json(snapshot))
}

fn resolve_trace_id(headers: &HeaderMap) -> String {
    headers
        .get(SDKWORK_TRACE_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("sg-{}", chrono::Utc::now().timestamp_millis()))
}

pub async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SdkWorkApiResponse<RealtimeEventsListData>>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    let limit = realtime::resolve_realtime_event_limit(&query.paging)?;

    let blocking_state = state.clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.clone();
    let after_seq = query.paging.after_seq.unwrap_or_default();
    let window = tokio::task::spawn_blocking(
        move || -> Result<im_domain_core::realtime::RealtimeEventWindow, ApiError> {
            blocking_state.prepare_active_client_route(
                &blocking_auth,
                blocking_device_id.as_str(),
                "http_poll",
                false,
            )?;
            blocking_state
                .realtime_runtime
                .list_events_for_principal_kind(RealtimeEventWindowQuery {
                    tenant_id: blocking_auth.tenant_id.as_str(),
                    organization_id: blocking_auth.organization_id.as_str(),
                    principal_id: blocking_auth.actor_id.as_str(),
                    principal_kind: blocking_auth.actor_kind.as_str(),
                    device_id: blocking_device_id.as_str(),
                    after_seq,
                    limit,
                })
                .map_err(ApiError::from)
        },
    )
    .await
    .map_err(join_error_to_api_error)??;

    Ok(Json(SdkWorkApiResponse::success(
        realtime_events_list_from_window(window, limit),
        resolve_trace_id(&headers),
    )))
}

pub async fn ack_realtime_events(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<RealtimeAckState>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    let acked_seq = request.acked_seq;

    // The ack flow reads route state, binds a route, performs blocking
    // Postgres load/trim/persist, and on failure restores or releases the
    // route. The entire chain is synchronous and IO-bound, so it runs on the
    // blocking thread pool.
    let blocking_state = state.clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.clone();
    let ack = tokio::task::spawn_blocking(move || -> Result<RealtimeAckState, ApiError> {
        let previous_route =
            blocking_state.current_active_client_route(&blocking_auth, blocking_device_id.as_str());
        blocking_state.prepare_active_client_route(
            &blocking_auth,
            blocking_device_id.as_str(),
            "http",
            false,
        )?;
        let bound_route =
            blocking_state.current_active_client_route(&blocking_auth, blocking_device_id.as_str());
        let ack_result = blocking_state
            .realtime_runtime
            .ack_events_for_principal_kind(
                blocking_auth.tenant_id.as_str(),
                blocking_auth.organization_id.as_str(),
                blocking_auth.actor_id.as_str(),
                blocking_auth.actor_kind.as_str(),
                blocking_device_id.as_str(),
                acked_seq,
            );
        match ack_result {
            Ok(ack) => Ok(ack),
            Err(error) => {
                match (previous_route, bound_route) {
                    (Some(previous_route), Some(bound_route)) => {
                        blocking_state
                            .restore_active_client_route_if_current(&bound_route, previous_route);
                    }
                    (None, _) => {
                        blocking_state.release_active_client_route_if_current_session(
                            &blocking_auth,
                            blocking_device_id.as_str(),
                        );
                    }
                    _ => {}
                }
                Err(ApiError::from(error))
            }
        }
    })
    .await
    .map_err(join_error_to_api_error)??;

    Ok(Json(ack))
}
