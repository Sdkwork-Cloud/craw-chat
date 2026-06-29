use axum::http::{StatusCode, header::CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use axum::Json;
use im_app_context::AppContextError;
use im_platform_contracts::ContractError;
use session_gateway::RealtimeClusterError;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ControlPlaneErrorStatus {
    Unauthorized,
    Forbidden,
    Invalid,
    Conflict,
    NotFound,
    Unavailable,
}

#[derive(Debug)]
pub(crate) struct ControlPlaneError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<serde_json::Value>,
}

impl From<RealtimeClusterError> for ControlPlaneError {
    fn from(value: RealtimeClusterError) -> Self {
        let status = match value.code {
            "node_not_found" | "target_node_not_found" | "node_runtime_missing" => {
                StatusCode::NOT_FOUND
            }
            "same_node_migration"
            | "node_not_draining"
            | "target_node_unavailable"
            | "node_draining" => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
            details: None,
        }
    }
}

impl From<AppContextError> for ControlPlaneError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
            details: None,
        }
    }
}

impl From<ContractError> for ControlPlaneError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_provider_policy",
                message,
                details: None,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "provider_policy_conflict",
                message,
                details: None,
            },
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_policy_unavailable",
                message,
                details: None,
            },
            ContractError::Invalid(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "provider_policy_invalid",
                message,
                details: None,
            },
        }
    }
}

impl ControlPlaneError {
    pub(crate) fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
            details: None,
        }
    }

    pub(crate) fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
            details: None,
        }
    }

    pub(crate) fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
            details: None,
        }
    }

    pub(crate) fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn response_status(status: StatusCode) -> ControlPlaneErrorStatus {
        match status {
            StatusCode::UNAUTHORIZED => ControlPlaneErrorStatus::Unauthorized,
            StatusCode::FORBIDDEN => ControlPlaneErrorStatus::Forbidden,
            StatusCode::CONFLICT => ControlPlaneErrorStatus::Conflict,
            StatusCode::NOT_FOUND => ControlPlaneErrorStatus::NotFound,
            StatusCode::SERVICE_UNAVAILABLE => ControlPlaneErrorStatus::Unavailable,
            _ => ControlPlaneErrorStatus::Invalid,
        }
    }
}

impl IntoResponse for ControlPlaneError {
    fn into_response(self) -> Response {
        let status = self.status;
        let response_status = Self::response_status(status);
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        let mut body = serde_json::json!({
            "type": "about:blank",
            "title": title,
            "status": status.as_u16(),
            "detail": detail,
            "code": self.code,
            "message": message,
            "errorStatus": response_status
        });
        if let Some(details) = self.details {
            body["details"] = details;
        }
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(body),
        )
            .into_response()
    }
}
