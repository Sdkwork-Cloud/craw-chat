use axum::response::{IntoResponse, Response};
use sdkwork_im_contract_core::ContractError;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};

#[derive(Debug)]
pub struct CallingError {
    pub(crate) status: axum::http::StatusCode,
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

impl CallingError {
    pub(crate) fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    pub fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub(crate) fn conflict(rtc_session_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "call_session_conflict",
            message: format!(
                "call session request conflicts with existing session idempotency key: {rtc_session_id}"
            ),
        }
    }

    pub(crate) fn payload_too_large(
        field: &'static str,
        max_bytes: usize,
        actual_bytes: usize,
    ) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub(crate) fn state_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "call_state_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "call_state_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "call_state_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_state_store_invalid",
                message,
            },
        }
    }
}

/// Map [`CallingError::status`] to the canonical [`WebFrameworkErrorKind`].
fn calling_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
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

impl From<CallingError> for ApiProblem {
    fn from(error: CallingError) -> Self {
        let framework_error = WebFrameworkError {
            kind: calling_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

/// Fallback `IntoResponse` for contexts where [`WebRequestContext`] is not
/// available (e.g. middleware without context injection). Produces a
/// Problem+json response without `traceId`.
impl IntoResponse for CallingError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: calling_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}
