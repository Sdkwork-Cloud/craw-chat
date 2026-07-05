//! Backend control-plane permission guards for social operator surfaces.

use im_app_context::AppContext;

use crate::friendship::SocialServiceError;

pub(crate) fn ensure_control_write_access(auth: &AppContext) -> Result<(), SocialServiceError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(SocialServiceError::forbidden(
        "control_write_required",
        "control.write permission is required",
    ))
}

pub(crate) fn ensure_control_read_access(auth: &AppContext) -> Result<(), SocialServiceError> {
    if auth.has_permission("control.read") || auth.has_permission("control.write") {
        return Ok(());
    }

    Err(SocialServiceError::forbidden(
        "control_read_required",
        "control.read or control.write permission is required",
    ))
}
