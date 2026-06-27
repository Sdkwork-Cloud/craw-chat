//! HTTP auth helpers for contact-service handlers.

use axum::extract::Extension;
use axum::http::{HeaderMap, StatusCode};
use im_app_context::{AppContext, AppRequestScope, require_handler_request_scope};

pub(crate) fn require_request_scope(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppRequestScope, StatusCode> {
    require_handler_request_scope(auth, headers).map_err(|_| StatusCode::UNAUTHORIZED)
}
