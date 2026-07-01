use im_app_context::AppContext;

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
