//! Space member API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::governance_store::SpaceMemberRecord;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;

use crate::api_payload::{bounded_sql_list_page, resource_item};
use crate::http::AppState;
use crate::list_query::{resolve_list_page, sql_fetch_limit, sql_fetch_offset, ListQuery};
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, ensure_user_not_banned_in_space, load_space,
    normalize_space_member_role, parse_space_id,
};
use crate::write_authority::{
    persist_space_member_joined, persist_space_member_removed, persist_space_member_updated,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberResponse {
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub joined_at: String,
}

impl From<SpaceMemberRecord> for MemberResponse {
    fn from(record: SpaceMemberRecord) -> Self {
        Self {
            user_id: record.user_id,
            role: record.role,
            nickname: record.nickname,
            joined_at: record.joined_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemberRequest {
    pub role: Option<String>,
    pub nickname: Option<String>,
}

pub async fn add_space_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<AddMemberRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        if request.user_id.trim().is_empty() {
            return Err(ApiProblem::bad_request("user_id is required"));
        }
        if request.user_id == space.owner_user_id {
            return Err(ApiProblem::bad_request("space owner is already a member"));
        }

        ensure_user_not_banned_in_space(&state, &auth, space_id, request.user_id.as_str())?;

        let role = normalize_space_member_role(request.role.as_deref(), false)?;

        let now = chrono::Utc::now().to_rfc3339();
        let record = SpaceMemberRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            space_id,
            user_id: request.user_id.clone(),
            role,
            nickname: request.nickname,
            joined_at: now.clone(),
            updated_at: now,
        };

        persist_space_member_joined(&state, &auth, &record, space.max_members)?;

        Ok(resource_item(MemberResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_space_members(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        let paging = resolve_list_page(&query)?;

        let records = state
            .space_member_store
            .list_by_space(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                space_id,
                sql_fetch_limit(paging),
                sql_fetch_offset(paging),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to list space members");
                ApiProblem::internal_server_error("failed to list space members")
            })?;

        let items = records.into_iter().map(MemberResponse::from).collect();
        Ok(bounded_sql_list_page(items, paging.page_size, paging.offset))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_space_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;

        state
            .space_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                space_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to get space member");
                ApiProblem::internal_server_error("failed to get space member")
            })?
            .map(MemberResponse::from)
            .map(resource_item)
            .ok_or_else(|| ApiProblem::not_found("space member not found"))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_space_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(String, String)>,
    Json(request): Json<UpdateMemberRequest>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        let mut record = state
            .space_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                space_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load space member for update");
                ApiProblem::internal_server_error("failed to load space member")
            })?
            .ok_or_else(|| ApiProblem::not_found("space member not found"))?;

        if record.role == "owner" || user_id == space.owner_user_id {
            return Err(ApiProblem::forbidden("space owner membership cannot be modified"));
        }

        if let Some(role) = request.role.as_deref() {
            record.role = normalize_space_member_role(Some(role), false)?;
        }
        if let Some(nickname) = request.nickname {
            record.nickname = Some(nickname);
        }
        record.updated_at = chrono::Utc::now().to_rfc3339();

        persist_space_member_updated(&state, &auth, &record)?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn remove_space_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        if user_id == space.owner_user_id {
            return Err(ApiProblem::forbidden("space owner cannot be removed"));
        }

        state
            .space_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                space_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load space member for removal");
                ApiProblem::internal_server_error("failed to load space member")
            })?
            .ok_or_else(|| ApiProblem::not_found("space member not found"))?;

        let removed_at = chrono::Utc::now().to_rfc3339();
        persist_space_member_removed(&state, &auth, space_id, user_id.as_str(), removed_at.as_str())?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
