use axum::response::{IntoResponse, Response};
use im_platform_contracts::ContractError;
use session_gateway::RealtimeClusterError;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, problem_response, ProblemCorrelation,
};

#[derive(Debug)]
pub struct ControlPlaneError {
    pub status: axum::http::StatusCode,
    pub code: &'static str,
    pub message: String,
}

fn control_plane_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
    use axum::http::StatusCode;
    match *status {
        StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkErrorKind::PayloadTooLarge,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        StatusCode::NOT_IMPLEMENTED => WebFrameworkErrorKind::NotImplemented,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<RealtimeClusterError> for ControlPlaneError {
    fn from(value: RealtimeClusterError) -> Self {
        let status = match value.code {
            "node_not_found" | "target_node_not_found" | "node_runtime_missing" => {
                axum::http::StatusCode::NOT_FOUND
            }
            "same_node_migration"
            | "node_not_draining"
            | "target_node_unavailable"
            | "node_draining" => axum::http::StatusCode::CONFLICT,
            _ => axum::http::StatusCode::BAD_REQUEST,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl From<ContractError> for ControlPlaneError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_provider_policy",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "provider_policy_conflict",
                message,
            },
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_policy_unavailable",
                message,
            },
            ContractError::Invalid(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "provider_policy_invalid",
                message,
            },
        }
    }
}

impl From<ControlPlaneError> for ApiProblem {
    fn from(error: ControlPlaneError) -> Self {
        let framework_error = WebFrameworkError {
            kind: control_plane_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for ControlPlaneError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: control_plane_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

impl ControlPlaneError {
    pub(crate) fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }

    pub(crate) fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub(crate) fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
        }
    }
}
