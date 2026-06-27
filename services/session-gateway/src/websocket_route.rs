use std::sync::Arc;

use axum::extract::Extension;
use axum::http::HeaderMap;
use im_app_context::AppContext;

use crate::client_route_registration::ClientRouteRegistration;
use crate::{ApiError, AppState, RealtimeDeliveryRuntime, resolve_requested_device_id};

pub(crate) struct RealtimeWebsocketRouteContext {
    pub auth: AppContext,
    pub device_id: String,
    pub runtime: Arc<RealtimeDeliveryRuntime>,
    pub route_owner: ClientRouteRegistration,
}

pub(crate) async fn prepare_realtime_websocket_route(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
    state: &AppState,
    query_device_id: Option<String>,
) -> Result<RealtimeWebsocketRouteContext, ApiError> {
    let auth = crate::resolve_request_app_context(auth, headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, query_device_id)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "websocket", false)?;
    Ok(RealtimeWebsocketRouteContext {
        auth,
        device_id,
        runtime: state.realtime_runtime.clone(),
        route_owner: state.client_route_registration.clone(),
    })
}
