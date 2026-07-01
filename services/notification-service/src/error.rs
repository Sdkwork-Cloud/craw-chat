use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use projection_service::ProjectionAccessError;
use sdkwork_im_contract_core::ContractError;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, ProblemCorrelation, problem_response,
};

#[derive(Debug)]
pub struct NotificationError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl NotificationError {
    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn not_found(notification_id: &str) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "notification_not_found",
            message: format!("notification not found: {notification_id}"),
        }
    }

    pub(crate) fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn conflict(notification_id: &str) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "notification_conflict",
            message: format!(
                "notification request conflicts with existing notification idempotency key: {notification_id}"
            ),
        }
    }

    pub(crate) fn payload_too_large(
        field: &'static str,
        max_bytes: usize,
        actual_bytes: usize,
    ) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub(crate) fn notification_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "notification_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "notification_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::NOT_IMPLEMENTED,
                code: "notification_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "notification_store_invalid",
                message,
            },
        }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

/// Map [`NotificationError::status`] to the canonical [`WebFrameworkErrorKind`].
fn notification_error_kind(status: &StatusCode) -> WebFrameworkErrorKind {
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

impl From<NotificationError> for ApiProblem {
    fn from(error: NotificationError) -> Self {
        let framework_error = WebFrameworkError {
            kind: notification_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for NotificationError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: notification_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

impl From<ContractError> for NotificationError {
    fn from(_value: ContractError) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "journal_unavailable",
            message: "commit journal unavailable".into(),
        }
    }
}

impl From<ProjectionAccessError> for NotificationError {
    fn from(value: ProjectionAccessError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}
