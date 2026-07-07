use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_web_core::{
    ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind, problem_response,
};

#[derive(Debug, Clone)]
pub struct PortalError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl PortalError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "portal_snapshot_not_found",
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "portal_unauthorized",
            message: message.into(),
        }
    }

    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }
}

fn portal_error_kind(status: &StatusCode) -> WebFrameworkErrorKind {
    match *status {
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<PortalError> for ApiProblem {
    fn from(error: PortalError) -> Self {
        ApiProblem::from_web_framework(WebFrameworkError {
            kind: portal_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        })
    }
}

impl IntoResponse for PortalError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: portal_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}
