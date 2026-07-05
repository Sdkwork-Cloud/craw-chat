//! Group member API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::organization_store::{GroupMemberRecord, GroupRecord};
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;

use crate::space_access::ensure_user_not_banned_in_space;
use crate::api_payload::{bounded_sql_list_page, resource_item};
use crate::group_conversation_binder::SyncSpaceGroupMemberInput;
use crate::http::AppState;
use crate::list_query::{resolve_list_page, sql_fetch_limit, sql_fetch_offset, ListQuery};
use crate::write_authority::{
    persist_group_member_joined, persist_group_member_removed, persist_group_member_updated,
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
    pub mute_until: Option<String>,
    pub joined_at: String,
}

impl From<GroupMemberRecord> for MemberResponse {
    fn from(record: GroupMemberRecord) -> Self {
        Self {
            user_id: record.user_id,
            role: record.role,
            nickname: record.nickname,
            mute_until: record.mute_until,
            joined_at: record.joined_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemberRequest {
    pub role: Option<String>,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
}

fn parse_space_id(space_id: &str) -> Result<i64, ApiProblem> {
    space_id.parse().map_err(|_| {
        tracing::warn!("invalid space_id path parameter: {space_id}");
        ApiProblem::bad_request("invalid space_id path parameter")
    })
}

fn parse_group_id(group_id: &str) -> Result<i64, ApiProblem> {
    group_id.parse().map_err(|_| {
        tracing::warn!("invalid group_id path parameter: {group_id}");
        ApiProblem::bad_request("invalid group_id path parameter")
    })
}

fn normalize_member_role(role: Option<&str>, allow_owner: bool) -> Result<String, ApiProblem> {
    match role.unwrap_or("member") {
        "owner" if allow_owner => Ok("owner".to_owned()),
        "owner" => Err(ApiProblem::bad_request("owner role cannot be assigned directly")),
        "admin" => Ok("admin".to_owned()),
        "member" => Ok("member".to_owned()),
        "muted" => Ok("muted".to_owned()),
        other => {
            tracing::warn!(role = other, "invalid group member role");
            Err(ApiProblem::bad_request("invalid group member role"))
        }
    }
}

fn load_group_in_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    group_id: i64,
) -> Result<GroupRecord, ApiProblem> {
    let group = state
        .group_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            group_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, group_id, "failed to load group");
            ApiProblem::internal_server_error("failed to load group")
        })?
        .ok_or_else(|| ApiProblem::not_found("group not found"))?;

    if group.space_id != Some(space_id) {
        tracing::warn!(
            group_id,
            space_id,
            actual_space_id = ?group.space_id,
            "group does not belong to requested space"
        );
        return Err(ApiProblem::not_found("group not found"));
    }
    Ok(group)
}

fn actor_can_manage_group(
    state: &AppState,
    auth: &AppContext,
    group: &GroupRecord,
) -> Result<(), ApiProblem> {
    if group.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.group_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        group.group_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(member)) if member.role == "admin" => Ok(()),
        Ok(Some(_)) => Err(ApiProblem::forbidden("group admin permission required")),
        Ok(None) => Err(ApiProblem::forbidden("group admin permission required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve group admin membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve group admin membership",
            ))
        }
    }
}

fn actor_can_read_group_members(
    state: &AppState,
    auth: &AppContext,
    group: &GroupRecord,
) -> Result<(), ApiProblem> {
    if group.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.group_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        group.group_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(ApiProblem::forbidden("group membership required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve group membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve group membership",
            ))
        }
    }
}

fn sync_conversation_add_member(
    state: &AppState,
    auth: &AppContext,
    group: &GroupRecord,
    user_id: &str,
    role: &str,
    mute_until: Option<String>,
) -> Result<(), ApiProblem> {
    let Some(conversation_id) = group.conversation_id.as_deref() else {
        return Ok(());
    };
    let Some(binder) = state.group_conversation_binder.as_ref() else {
        if crate::runtime_env::is_production_like_im_environment() {
            return Err(ApiProblem::dependency_unavailable(
                "group conversation binder is required when group is linked to a conversation",
            ));
        }
        tracing::warn!(
            group_id = group.group_id,
            conversation_id,
            "group conversation binder is not configured; conversation membership will not sync"
        );
        return Ok(());
    };
    binder
        .add_group_member(SyncSpaceGroupMemberInput {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            conversation_id: conversation_id.to_owned(),
            user_id: user_id.to_owned(),
            role: role.to_owned(),
            actor_user_id: auth.actor_id.clone(),
            mute_until,
        })
        .map_err(|error| {
            tracing::error!(error = %error, "failed to sync group member into conversation");
            ApiProblem::internal_server_error("failed to sync group member into conversation")
        })
}

fn sync_conversation_remove_member(
    state: &AppState,
    auth: &AppContext,
    group: &GroupRecord,
    user_id: &str,
    role: &str,
) -> Result<(), ApiProblem> {
    let Some(conversation_id) = group.conversation_id.as_deref() else {
        return Ok(());
    };
    let Some(binder) = state.group_conversation_binder.as_ref() else {
        if crate::runtime_env::is_production_like_im_environment() {
            return Err(ApiProblem::dependency_unavailable(
                "group conversation binder is required when group is linked to a conversation",
            ));
        }
        tracing::warn!(
            group_id = group.group_id,
            conversation_id,
            "group conversation binder is not configured; conversation membership will not sync"
        );
        return Ok(());
    };
    binder
        .remove_group_member(SyncSpaceGroupMemberInput {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            conversation_id: conversation_id.to_owned(),
            user_id: user_id.to_owned(),
            role: role.to_owned(),
            actor_user_id: auth.actor_id.clone(),
            mute_until: None,
        })
        .map_err(|error| {
            tracing::error!(error = %error, "failed to remove group member from conversation");
            ApiProblem::internal_server_error("failed to remove group member from conversation")
        })
}

pub async fn add_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
    Json(request): Json<AddMemberRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;
        actor_can_manage_group(&state, &auth, &group)?;

        if request.user_id.trim().is_empty() {
            return Err(ApiProblem::bad_request("user_id is required"));
        }
        ensure_user_not_banned_in_space(&state, &auth, space_id, request.user_id.as_str())?;
        if request.user_id == group.owner_user_id {
            return Err(ApiProblem::bad_request("group owner is already a member"));
        }

        let role = normalize_member_role(request.role.as_deref(), false)?;

        let now = chrono::Utc::now().to_rfc3339();
        let record = GroupMemberRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            group_id,
            user_id: request.user_id.clone(),
            role: role.clone(),
            nickname: request.nickname,
            mute_until: None,
            joined_at: now.clone(),
            updated_at: now,
        };

        sync_conversation_add_member(
            &state,
            &auth,
            &group,
            record.user_id.as_str(),
            role.as_str(),
            record.mute_until.clone(),
        )?;

        if let Err(error) = persist_group_member_joined(&state, &auth, &record, group.max_members) {
            if sync_conversation_remove_member(
                &state,
                &auth,
                &group,
                record.user_id.as_str(),
                role.as_str(),
            )
            .is_err()
            {
                tracing::error!(
                    group_id = group.group_id,
                    user_id = record.user_id.as_str(),
                    "failed to compensate conversation membership after group member persist failure"
                );
            }
            return Err(error);
        }
        Ok(resource_item(MemberResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_group_members(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;
        actor_can_read_group_members(&state, &auth, &group)?;
        let paging = resolve_list_page(&query)?;

        let records = state
            .group_member_store
            .list_by_group(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
                sql_fetch_limit(paging),
                sql_fetch_offset(paging),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to list group members");
                ApiProblem::internal_server_error("failed to list group members")
            })?;

        let items = records.into_iter().map(MemberResponse::from).collect();
        Ok(bounded_sql_list_page(items, paging.page_size, paging.offset))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id, user_id)): Path<(String, String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<MemberResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;
        actor_can_read_group_members(&state, &auth, &group)?;

        state
            .group_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to get group member");
                ApiProblem::internal_server_error("failed to get group member")
            })?
            .map(MemberResponse::from)
            .map(resource_item)
            .ok_or_else(|| ApiProblem::not_found("group member not found"))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id, user_id)): Path<(String, String, String)>,
    Json(request): Json<UpdateMemberRequest>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;
        actor_can_manage_group(&state, &auth, &group)?;

        let mut record = state
            .group_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load group member for update");
                ApiProblem::internal_server_error("failed to load group member")
            })?
            .ok_or_else(|| ApiProblem::not_found("group member not found"))?;

        if record.role == "owner" || user_id == group.owner_user_id {
            return Err(ApiProblem::forbidden("group owner membership cannot be modified"));
        }

        if let Some(role) = request.role.as_deref() {
            record.role = normalize_member_role(Some(role), false)?;
        }
        if let Some(nickname) = request.nickname {
            record.nickname = Some(nickname);
        }
        if let Some(mute_until) = request.mute_until {
            record.mute_until = Some(mute_until);
        }
        record.updated_at = chrono::Utc::now().to_rfc3339();

        persist_group_member_updated(&state, &auth, &record)?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn remove_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id, user_id)): Path<(String, String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;
        let is_self = auth.actor_id == user_id;
        if is_self {
            if user_id == group.owner_user_id {
                return Err(ApiProblem::forbidden(
                    "group owner must transfer ownership before leaving",
                ));
            }
            actor_can_read_group_members(&state, &auth, &group)?;
        } else {
            actor_can_manage_group(&state, &auth, &group)?;
            if user_id == group.owner_user_id {
                return Err(ApiProblem::forbidden("group owner cannot be removed"));
            }
        }

        let member = state
            .group_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load group member for removal");
                ApiProblem::internal_server_error("failed to load group member")
            })?
            .ok_or_else(|| ApiProblem::not_found("group member not found"))?;

        let removed_at = chrono::Utc::now().to_rfc3339();
        persist_group_member_removed(
            &state,
            &auth,
            group_id,
            member.user_id.as_str(),
            removed_at.as_str(),
        )?;

        sync_conversation_remove_member(
            &state,
            &auth,
            &group,
            member.user_id.as_str(),
            member.role.as_str(),
        )?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
