//! Typed automation service errors and HTTP boundary mapping.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sdkwork_im_contract_core::ContractError;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, problem_response, ProblemCorrelation,
};

#[derive(Debug)]
pub struct AutomationError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl AutomationError {
    pub(crate) fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn not_found(execution_id: &str) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "automation_execution_not_found",
            message: format!("automation execution not found: {execution_id}"),
        }
    }

    pub(crate) fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }

    pub(crate) fn conflict(execution_id: &str) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "automation_execution_conflict",
            message: format!("automation execution conflict: {execution_id}"),
        }
    }

    pub(crate) fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub(crate) fn automation_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "automation_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "automation_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::NOT_IMPLEMENTED,
                code: "automation_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "automation_store_invalid",
                message,
            },
        }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}

/// Map [`AutomationError::status`] to the canonical [`WebFrameworkErrorKind`].
fn automation_error_kind(status: &StatusCode) -> WebFrameworkErrorKind {
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

impl From<AutomationError> for ApiProblem {
    fn from(error: AutomationError) -> Self {
        let framework_error = WebFrameworkError {
            kind: automation_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for AutomationError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: automation_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

impl From<ContractError> for AutomationError {
    fn from(_value: ContractError) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "journal_unavailable",
            message: "commit journal unavailable".into(),
        }
    }
}
