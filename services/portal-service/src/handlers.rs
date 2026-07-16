use audit_service::AuditRecordSample;
use axum::extract::{Extension, OriginalUri, State};
use axum::response::Response;
use im_app_context::AppContext;
use im_portal_snapshots::{
    PortalSnapshot, build_portal_snapshot_for_section, build_portal_workspace_view,
};
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_utils_rust::SdkWorkResourceData;
use sdkwork_web_core::WebRequestContext;

use crate::error::PortalError;
use crate::state::AppState;

const PORTAL_AUDIT_PAGE_SIZE: usize = 20;

pub(crate) async fn get_portal_workspace(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<im_portal_snapshots::PortalWorkspaceView>> = (|| {
        ensure_authenticated(&auth)?;
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
    OriginalUri(uri): OriginalUri,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<PortalSnapshot>> = (|| {
        let section = uri
            .path()
            .rsplit('/')
            .next()
            .map(str::trim)
            .unwrap_or_default();
        if section == "access" {
            return Err(PortalError::not_found(
                "use /app/v3/api/portal/access for the access snapshot",
            )
            .into());
        }
        ensure_authenticated(&auth)?;
        let audit_sample = load_audit_sample_for_section(section, &auth, &state)?;
        let snapshot = build_portal_snapshot_for_section(
            section,
            state.runtime.ops.clone(),
            Some(&auth),
            audit_sample.as_ref(),
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
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<PortalSnapshot>> = (|| {
        ensure_authenticated(&auth)?;
        let audit_sample = load_audit_sample_for_section("access", &auth, &state)?;
        let snapshot = build_portal_snapshot_for_section(
            "access",
            state.runtime.ops.clone(),
            Some(&auth),
            audit_sample.as_ref(),
        )
        .ok_or_else(|| PortalError::not_found("portal access snapshot is unavailable"))?;
        Ok(SdkWorkResourceData { item: snapshot })
    })();
    finish_api_json(&ctx, result)
}

fn load_audit_sample_for_section(
    section: &str,
    auth: &AppContext,
    state: &AppState,
) -> Result<Option<AuditRecordSample>, audit_service::AuditError> {
    if !matches!(section, "access" | "governance") {
        return Ok(None);
    }

    state
        .runtime
        .audit
        .recent_records(auth, PORTAL_AUDIT_PAGE_SIZE)
        .map(Some)
}

fn ensure_authenticated(auth: &AppContext) -> Result<(), PortalError> {
    if auth.tenant_id.trim().is_empty() || auth.user_id.trim().is_empty() {
        return Err(PortalError::unauthorized(
            "portal snapshot requires an authenticated app session",
        ));
    }
    Ok(())
}
