use std::sync::Arc;

use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::extract::{Extension, Query, State};
use axum::http::HeaderMap;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use im_app_context::{AppContext, has_websocket_upgrade_auth_headers};
use sdkwork_im_runtime_link::{
    LinkWebsocketMode, LinkWebsocketUpgradeHandoff, prepare_websocket_upgrade,
    supported_websocket_subprotocols,
};
use serde::Deserialize;
use tokio::sync::OwnedSemaphorePermit;
use tracing::warn;

use crate::client_route_registration::ClientRouteRegistration;
use crate::websocket::{RealtimeWebsocketMode, serve_realtime_websocket};
use crate::websocket_route;
use crate::{ApiError, AppState, RealtimeDeliveryRuntime};

const REALTIME_MAX_WEBSOCKET_MESSAGE_BYTES: usize = 512 * 1024;
const REALTIME_MAX_WEBSOCKET_FRAME_BYTES: usize = 256 * 1024;

#[derive(Debug, Deserialize)]
pub(crate) struct RealtimeWebsocketQuery {
    #[serde(rename = "deviceId")]
    device_id: Option<String>,
}

pub(crate) struct RealtimeWebsocketUpgradeContext {
    auth: AppContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: ClientRouteRegistration,
    frame_rate_limiter: crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
}

pub(crate) fn acquire_preauth_websocket_connection_permit(
    state: &AppState,
) -> Result<OwnedSemaphorePermit, ApiError> {
    state
        .preauth_websocket_connection_semaphore
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "websocket_preauth_overloaded",
            message: "server is at maximum pre-authentication websocket capacity, please retry later"
                .to_owned(),
        })
}

pub(crate) fn acquire_websocket_connection_permit(
    state: &AppState,
) -> Result<OwnedSemaphorePermit, ApiError> {
    state
        .websocket_connection_semaphore
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "websocket_overloaded",
            message: "server is at maximum websocket capacity, please retry later".to_owned(),
        })
}

pub(crate) async fn realtime_websocket(
    ws: WebSocketUpgrade,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<RealtimeWebsocketQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    let client_ip =
        crate::websocket_upgrade_rate_limit::extract_client_ip_from_headers(&headers);
    state
        .websocket_upgrade_rate_limiter
        .check_upgrade(client_ip)?;

    if auth.is_none() && !has_websocket_upgrade_auth_headers(&headers) {
        let preauth_permit = acquire_preauth_websocket_connection_permit(&state)?;
        let requested_protocol = requested_websocket_subprotocol(&headers).map(str::to_owned);
        let state = state.clone();
        return Ok(ws
            .protocols(realtime_websocket_subprotocols())
            .max_message_size(REALTIME_MAX_WEBSOCKET_MESSAGE_BYTES)
            .max_frame_size(REALTIME_MAX_WEBSOCKET_FRAME_BYTES)
            .on_upgrade(move |socket| {
                crate::websocket_auth_init::realtime_websocket_after_auth_init_frame(
                    socket,
                    state,
                    requested_protocol,
                    query.device_id,
                    preauth_permit,
                )
            })
            .into_response());
    }

    let permit = acquire_websocket_connection_permit(&state)?;

    let context =
        websocket_route::prepare_realtime_websocket_route(auth, &headers, &state, query.device_id)
            .await?;
    let requested_protocol = requested_websocket_subprotocol(&headers);
    Ok(upgrade_realtime_websocket(
        ws,
        requested_protocol,
        context.auth,
        context.device_id,
        context.runtime,
        context.route_owner,
        state.websocket_frame_rate_limiter.clone(),
        permit,
    ))
}

fn requested_websocket_subprotocol(headers: &HeaderMap) -> Option<&str> {
    let header_value = headers.get(header::SEC_WEBSOCKET_PROTOCOL)?.to_str().ok()?;
    header_value.split(',').map(str::trim).find(|candidate| {
        realtime_websocket_subprotocols()
            .iter()
            .any(|supported| supported == candidate)
    })
}

pub(crate) fn upgrade_realtime_websocket(
    ws: WebSocketUpgrade,
    requested_protocol: Option<&str>,
    auth: AppContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: ClientRouteRegistration,
    frame_rate_limiter: crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
    semaphore_permit: OwnedSemaphorePermit,
) -> Response {
    let mode = sdkwork_im_runtime_link::select_websocket_mode(requested_protocol);
    if mode == LinkWebsocketMode::LegacyJson && !crate::realtime_accepts_legacy_websocket_json() {
        return ApiError::bad_request(
            "legacy_websocket_json_rejected",
            "websocket upgrade requires sdkwork-im.ccp.ws.v1; set SDKWORK_IM_REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON=true only for deprecated plain-json clients",
        )
        .into_response();
    }
    let ws = ws
        .protocols(realtime_websocket_subprotocols())
        .max_message_size(REALTIME_MAX_WEBSOCKET_MESSAGE_BYTES)
        .max_frame_size(REALTIME_MAX_WEBSOCKET_FRAME_BYTES);
    let upgrade = prepare_realtime_websocket_upgrade(
        ws.selected_protocol()
            .and_then(|selected| selected.to_str().ok()),
        auth,
        device_id,
        runtime,
        route_owner,
        frame_rate_limiter,
    );
    ws.on_upgrade(move |socket| {
        upgrade.execute(socket, move |socket, context, mode| {
            serve_realtime_websocket_upgrade(socket, context, mode, semaphore_permit)
        })
    })
    .into_response()
}

pub(crate) fn realtime_websocket_subprotocols() -> [&'static str; 1] {
    supported_websocket_subprotocols()
}

#[cfg(test)]
pub(crate) fn select_realtime_websocket_mode(
    selected_protocol: Option<&str>,
) -> RealtimeWebsocketMode {
    map_runtime_link_websocket_mode(sdkwork_im_runtime_link::select_websocket_mode(
        selected_protocol,
    ))
}

fn map_runtime_link_websocket_mode(mode: LinkWebsocketMode) -> RealtimeWebsocketMode {
    match mode {
        LinkWebsocketMode::LegacyJson => RealtimeWebsocketMode::LegacyJson,
        LinkWebsocketMode::CcpJson => RealtimeWebsocketMode::CcpJson,
    }
}

pub(crate) fn prepare_realtime_websocket_upgrade(
    selected_protocol: Option<&str>,
    auth: AppContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: ClientRouteRegistration,
    frame_rate_limiter: crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
) -> LinkWebsocketUpgradeHandoff<RealtimeWebsocketUpgradeContext> {
    prepare_websocket_upgrade(
        selected_protocol,
        RealtimeWebsocketUpgradeContext {
            auth,
            device_id,
            runtime,
            route_owner,
            frame_rate_limiter,
        },
    )
}

pub(crate) async fn serve_realtime_websocket_upgrade(
    socket: WebSocket,
    context: RealtimeWebsocketUpgradeContext,
    mode: LinkWebsocketMode,
    _permit: OwnedSemaphorePermit,
) {
    let RealtimeWebsocketUpgradeContext {
        auth,
        device_id,
        runtime,
        route_owner,
        frame_rate_limiter,
    } = context;
    if mode == LinkWebsocketMode::LegacyJson {
        warn!(
            target: "sdkwork.im",
            event = "im.realtime.websocket.legacy_json_deprecated",
            actor_id = %auth.actor_id,
            device_id = %device_id,
            "legacy.json websocket subprotocol is deprecated; clients must negotiate sdkwork-im.ccp.ws.v1"
        );
    }
    let cleanup_auth = auth.clone();
    let cleanup_device_id = device_id.clone();
    let cleanup_runtime = runtime.clone();
    serve_realtime_websocket(
        socket,
        auth,
        device_id,
        runtime,
        route_owner.clone(),
        map_runtime_link_websocket_mode(mode),
        frame_rate_limiter,
    )
    .await;
    route_owner.finalize_active_client_route_disconnect(
        &cleanup_auth,
        cleanup_device_id.as_str(),
        cleanup_runtime.as_ref(),
    );
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use im_app_context::AppContext;
    use sdkwork_im_runtime_link::LinkWebsocketMode;

    use super::{
        prepare_realtime_websocket_upgrade, realtime_websocket_subprotocols,
        select_realtime_websocket_mode,
    };
    use crate::{RealtimeDeliveryRuntime, RealtimeWebsocketMode};

    #[test]
    fn test_realtime_websocket_upgrade_uses_runtime_link_owner_contract() {
        assert_eq!(
            realtime_websocket_subprotocols(),
            [crate::CCP_WEBSOCKET_SUBPROTOCOL]
        );
        assert_eq!(
            select_realtime_websocket_mode(Some(crate::CCP_WEBSOCKET_SUBPROTOCOL)),
            RealtimeWebsocketMode::CcpJson
        );
        assert_eq!(
            select_realtime_websocket_mode(Some("legacy.json")),
            RealtimeWebsocketMode::LegacyJson
        );
        assert_eq!(
            select_realtime_websocket_mode(None),
            RealtimeWebsocketMode::LegacyJson
        );
    }

    #[test]
    fn test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner() {
        let runtime = Arc::new(RealtimeDeliveryRuntime::default());
        let handoff = prepare_realtime_websocket_upgrade(
            Some(crate::CCP_WEBSOCKET_SUBPROTOCOL),
            AppContext {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                user_id: "1".into(),
                actor_id: "1".into(),
                actor_kind: "user".into(),
                session_id: Some("s_demo".into()),
                app_id: None,
                environment: None,
                deployment_mode: None,
                auth_level: None,
                data_scope: Default::default(),
                permission_scope: Default::default(),
                device_id: Some("d_pad".into()),
            },
            "d_pad".into(),
            runtime.clone(),
            crate::client_route_registration::ClientRouteRegistration::new(
                "node_a".into(),
                Arc::new(crate::RealtimeClusterBridge::default()),
                Arc::new(crate::PresenceRuntime::default()),
                runtime,
                crate::client_route_state::ClientRouteState::default(),
            ),
            crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter::from_env(),
        );

        assert_eq!(handoff.mode(), LinkWebsocketMode::CcpJson);
        assert_eq!(handoff.context().auth.tenant_id, "100001");
        assert_eq!(handoff.context().auth.actor_id, "1");
        assert_eq!(handoff.context().device_id, "d_pad");
    }
}
