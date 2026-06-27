use std::time::Duration;

use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket};
use futures_util::StreamExt;
use im_app_context::AppContext;
use tokio::time::Instant;

use crate::frame::{
    AUTH_INIT_MAX_FRAME_BYTES, AUTH_INIT_TIMEOUT_SECONDS, WebsocketAuthInitFrame, auth_error_payload,
    auth_ok_payload_from_context,
};

pub async fn read_websocket_auth_init_frame(socket: &mut WebSocket) -> Option<WebsocketAuthInitFrame> {
    let deadline = Instant::now() + Duration::from_secs(AUTH_INIT_TIMEOUT_SECONDS);
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return None;
        }
        let next_message = tokio::time::timeout(remaining, socket.next()).await.ok()??;
        let Ok(message) = next_message else {
            return None;
        };
        match message {
            Message::Text(text) => {
                if text.len() > AUTH_INIT_MAX_FRAME_BYTES {
                    return None;
                }
                return serde_json::from_str::<WebsocketAuthInitFrame>(text.as_str()).ok();
            }
            Message::Binary(bytes) => {
                let text = String::from_utf8(bytes.to_vec()).ok()?;
                if text.len() > AUTH_INIT_MAX_FRAME_BYTES {
                    return None;
                }
                return serde_json::from_str::<WebsocketAuthInitFrame>(text.as_str()).ok();
            }
            Message::Close(_) => return None,
            Message::Ping(payload) => {
                let _ = socket.send(Message::Pong(payload)).await;
            }
            Message::Pong(_) => {}
        }
    }
}

pub async fn send_websocket_auth_ok(
    socket: &mut WebSocket,
    request_id: Option<&str>,
    auth: &AppContext,
    device_id: &str,
) -> Result<(), ()> {
    socket
        .send(Message::Text(
            auth_ok_payload_from_context(request_id, auth, device_id).into(),
        ))
        .await
        .map_err(|_| ())
}

pub async fn close_websocket_with_auth_error(
    socket: &mut WebSocket,
    request_id: Option<&str>,
    code: &str,
    message: &str,
) {
    let _ = socket
        .send(Message::Text(
            auth_error_payload(request_id, code, message).into(),
        ))
        .await;
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::POLICY,
            reason: Utf8Bytes::from(code.to_owned()),
        })))
        .await;
}
