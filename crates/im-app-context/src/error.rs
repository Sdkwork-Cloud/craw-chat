use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppContextError {
    code: &'static str,
    message: String,
}

impl AppContextError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn missing(message: impl Into<String>) -> Self {
        Self {
            code: "app_context_missing",
            message: message.into(),
        }
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            code: "app_context_invalid",
            message: message.into(),
        }
    }

    pub fn auth_token_missing() -> Self {
        Self {
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        }
    }

    pub fn access_token_missing() -> Self {
        Self {
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
        }
    }
}

impl std::fmt::Display for AppContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppContextError {}

pub(crate) fn app_context_error_response(error: AppContextError) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(json!({
            "code": error.code(),
            "message": error.message(),
        })),
    )
        .into_response()
}
