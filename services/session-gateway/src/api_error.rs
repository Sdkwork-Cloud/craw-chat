use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::{Json, response::Response};
use im_app_context::AppContextError;

use crate::cluster::RealtimeClusterError;
use crate::presence::PresenceRuntimeError;
use crate::realtime::RealtimeRuntimeError;

#[derive(Debug)]
pub struct ApiError {
    pub status: axum::http::StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code,
            message: message.into(),
        }
    }

    pub fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }
}

impl From<AppContextError> for ApiError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<RealtimeClusterError> for ApiError {
    fn from(value: RealtimeClusterError) -> Self {
        Self {
            status: if value.code == "disconnect_fence_store_unavailable"
                || value.code == "checkpoint_store_unavailable"
                || value.code == "subscription_store_unavailable"
            {
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            } else {
                axum::http::StatusCode::CONFLICT
            },
            code: value.code,
            message: value.message,
        }
    }
}

impl From<RealtimeRuntimeError> for ApiError {
    fn from(value: RealtimeRuntimeError) -> Self {
        let status = match value.code {
            "payload_too_large" => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "limit_invalid" => axum::http::StatusCode::BAD_REQUEST,
            "conversation_archived" | "conversation_blocked" | "realtime_scope_access_denied" => {
                axum::http::StatusCode::FORBIDDEN
            }
            "checkpoint_store_unavailable"
            | "subscription_store_unavailable"
            | "event_window_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "checkpoint_store_conflict"
            | "subscription_store_conflict"
            | "event_window_store_conflict" => axum::http::StatusCode::CONFLICT,
            "checkpoint_store_unsupported"
            | "subscription_store_unsupported"
            | "event_window_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl From<PresenceRuntimeError> for ApiError {
    fn from(value: PresenceRuntimeError) -> Self {
        let status = match value.code() {
            "presence_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "presence_store_conflict" | "reconnect_required" => axum::http::StatusCode::CONFLICT,
            "presence_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(serde_json::json!({
                "type": "about:blank",
                "title": title,
                "status": status.as_u16(),
                "detail": detail,
                "code": self.code,
                "message": message
            })),
        )
            .into_response()
    }
}
