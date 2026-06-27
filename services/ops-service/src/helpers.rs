use axum::extract::Extension;
use axum::http::HeaderMap;
use im_app_context::{AppContext, resolve_app_context};

use crate::error::OpsError;

pub(crate) fn ensure_ops_read_access(auth: &AppContext) -> Result<(), OpsError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(OpsError::forbidden("ops.read"))
}

pub(crate) fn ensure_ops_write_access(auth: &AppContext) -> Result<(), OpsError> {
    if auth.has_permission("ops.write") {
        return Ok(());
    }

    Err(OpsError::forbidden("ops.write"))
}

pub(crate) fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, OpsError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(OpsError::from),
    }
}
