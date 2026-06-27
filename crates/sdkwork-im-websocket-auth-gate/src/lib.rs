//! Shared WebSocket `auth.init` gate for browser clients and gateway proxy paths.

mod axum_gate;
mod device;
mod frame;
mod policy;

pub use axum_gate::{
    close_websocket_with_auth_error, read_websocket_auth_init_frame, send_websocket_auth_ok,
};
pub use device::{resolve_websocket_device_binding, WebsocketDeviceBindingError};
pub use frame::{
    AUTH_INIT_FRAME_TYPE, AUTH_INIT_MAX_FRAME_BYTES, AUTH_INIT_TIMEOUT_SECONDS,
    AuthInitValidationError, WebsocketAuthInitFrame, auth_ok_payload, auth_ok_payload_from_context,
    dual_token_headers_from_auth_init_frame, normalize_websocket_auth_token,
};
pub use policy::{
    SENSITIVE_WEBSOCKET_QUERY_KEYS, is_sensitive_websocket_query_key,
    sanitized_realtime_websocket_path_and_query, should_require_auth_init_frame,
};
