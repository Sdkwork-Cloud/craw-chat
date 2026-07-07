use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use im_portal_snapshots::{build_portal_snapshot_for_section, build_portal_workspace_view};
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_utils_rust::SdkWorkResourceData;
use sdkwork_web_core::WebRequestContext;

use crate::error::PortalError;
use crate::state::AppState;

const PORTAL_READ_PERMISSION: &str = "portal.read";

pub(crate) async fn get_portal_workspace(Extension(ctx): Extension<WebRequestContext>) -> Response {
    let result: ApiResult<SdkWorkResourceData<im_portal_snapshots::PortalWorkspaceView>> = (|| {
        Ok(SdkWorkResourceData {
            item: build_portal_workspace_view(),
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_portal_snapshot(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(section): Path<String>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<serde_json::Value>> = (|| {
        let section = section.trim();
        if section == "access" {
            return Err(PortalError::not_found(
                "use /app/v3/api/portal/access for the access snapshot",
            )
            .into());
        }
        ensure_authenticated(&auth)?;
        let snapshot = build_portal_snapshot_for_section(
            section,
            state.runtime.ops.clone(),
            Some(&auth),
            Some(state.runtime.audit.clone()),
        )
        .ok_or_else(|| {
            PortalError::not_found(format!("portal section `{section}` is unavailable"))
        })?;
        Ok(SdkWorkResourceData { item: snapshot })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_portal_access_snapshot(
    Extension(ctx): Extension<WebRequestContext>,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<serde_json::Value>> = (|| {
        let auth = auth.as_ref().map(|Extension(auth)| auth);
        let audit = auth.is_some().then(|| state.runtime.audit.clone());
        let snapshot =
            build_portal_snapshot_for_section("access", state.runtime.ops.clone(), auth, audit)
                .ok_or_else(|| PortalError::not_found("portal access snapshot is unavailable"))?;
        Ok(SdkWorkResourceData { item: snapshot })
    })();
    finish_api_json(&ctx, result)
}

fn ensure_authenticated(auth: &AppContext) -> Result<(), PortalError> {
    if auth.tenant_id.trim().is_empty()
        || auth.user_id.trim().is_empty()
        || !auth.has_permission(PORTAL_READ_PERMISSION)
    {
        return Err(PortalError::unauthorized(
            "portal snapshot requires an authenticated app session",
        ));
    }
    Ok(())
}
