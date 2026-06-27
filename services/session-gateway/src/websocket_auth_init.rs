use im_app_context::coalesce_websocket_device_id;
use sdkwork_im_websocket_auth_gate::{
    close_websocket_with_auth_error, dual_token_headers_from_auth_init_frame,
    read_websocket_auth_init_frame, resolve_websocket_device_binding, send_websocket_auth_ok,
};

use crate::AppState;
use crate::websocket_upgrade::{prepare_realtime_websocket_upgrade, serve_realtime_websocket_upgrade};

pub(crate) async fn realtime_websocket_after_auth_init_frame(
    mut socket: axum::extract::ws::WebSocket,
    state: AppState,
    selected_protocol: Option<String>,
    query_device_id: Option<String>,
    semaphore_permit: tokio::sync::OwnedSemaphorePermit,
) {
    let Some(auth_init) = read_websocket_auth_init_frame(&mut socket).await else {
        close_websocket_with_auth_error(
            &mut socket,
            None,
            "websocket_auth_required",
            "auth.init frame is required before realtime websocket frames",
        )
        .await;
        return;
    };

    let auth_headers = match dual_token_headers_from_auth_init_frame(&auth_init) {
        Ok(headers) => headers,
        Err(error) => {
            close_websocket_with_auth_error(
                &mut socket,
                auth_init.request_id.as_deref(),
                error.error_code(),
                error.message(),
            )
            .await;
            return;
        }
    };

    let auth = match state.auth_resolver.resolve_from_headers(&auth_headers).await {
        Ok(context) => context,
        Err(_) => {
            close_websocket_with_auth_error(
                &mut socket,
                auth_init.request_id.as_deref(),
                "websocket_auth_failed",
                "websocket auth.init token context validation failed",
            )
            .await;
            return;
        }
    };

    let requested_device_id =
        coalesce_websocket_device_id(auth_init.device_id.clone(), query_device_id);
    let device_id = match resolve_websocket_device_binding(&auth, requested_device_id) {
        Ok(device_id) => device_id,
        Err(error) => {
            close_websocket_with_auth_error(
                &mut socket,
                auth_init.request_id.as_deref(),
                error.code,
                error.message.as_str(),
            )
            .await;
            return;
        }
    };

    if let Err(error) = state.prepare_active_client_route(&auth, device_id.as_str(), "websocket", false)
    {
        close_websocket_with_auth_error(
            &mut socket,
            auth_init.request_id.as_deref(),
            error.code,
            error.message.as_str(),
        )
        .await;
        return;
    }

    let _ = send_websocket_auth_ok(
        &mut socket,
        auth_init.request_id.as_deref(),
        &auth,
        device_id.as_str(),
    )
    .await;

    let upgrade = prepare_realtime_websocket_upgrade(
        selected_protocol.as_deref(),
        auth,
        device_id,
        state.realtime_runtime.clone(),
        state.client_route_registration.clone(),
    );
    upgrade
        .execute(socket, move |socket, context, mode| {
            serve_realtime_websocket_upgrade(socket, context, mode, semaphore_permit)
        })
        .await;
}
