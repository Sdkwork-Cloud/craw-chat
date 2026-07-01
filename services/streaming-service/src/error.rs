use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sdkwork_im_contract_core::ContractError;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, ProblemCorrelation, problem_response,
};

#[derive(Debug)]
pub struct StreamingError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl StreamingError {
    pub(crate) fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub(crate) fn conflict(stream_id: &str) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "stream_conflict",
            message: format!(
                "stream open request conflicts with existing stream idempotency key: {stream_id}"
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

    pub(crate) fn stream_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "stream_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "stream_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::NOT_IMPLEMENTED,
                code: "stream_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "stream_store_invalid",
                message,
            },
        }
    }
}

/// Map [`StreamingError::status`] to the canonical [`WebFrameworkErrorKind`].
fn streaming_error_kind(status: &StatusCode) -> WebFrameworkErrorKind {
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

impl From<StreamingError> for ApiProblem {
    fn from(error: StreamingError) -> Self {
        let framework_error = WebFrameworkError {
            kind: streaming_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for StreamingError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: streaming_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}
