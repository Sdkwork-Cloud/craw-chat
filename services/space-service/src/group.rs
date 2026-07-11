//! Group API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::organization_store::{GroupMemberRecord, GroupRecord};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::group_conversation_binder::{
    CreateSpaceGroupConversationInput, TransferSpaceGroupOwnerInput,
};
use crate::http::AppState;
use crate::id::next_entity_id;
use crate::list_query::{ListQuery, resolve_keyset_page};
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, load_space, parse_space_id,
};
use crate::write_authority::{
    persist_group_created, persist_group_deleted, persist_group_owner_transferred,
    persist_group_updated,
};

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub group_name: String,
    pub group_type: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub group_id: String,
    pub space_id: Option<String>,
    pub group_name: String,
    pub group_type: String,
    pub owner_user_id: String,
    pub conversation_id: Option<String>,
    pub max_members: i32,
    pub created_at: String,
}

impl From<GroupRecord> for GroupResponse {
    fn from(record: GroupRecord) -> Self {
        Self {
            group_id: record.group_id.to_string(),
            space_id: record.space_id.map(|s| s.to_string()),
            group_name: record.group_name,
            group_type: record.group_type,
            owner_user_id: record.owner_user_id,
            conversation_id: record.conversation_id,
            max_members: record.max_members,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub announcement: Option<String>,
}

fn parse_group_id(group_id: &str) -> Result<i64, ApiProblem> {
    group_id.parse().map_err(|_| {
        tracing::warn!("invalid group_id path parameter: {group_id}");
        ApiProblem::bad_request("invalid group_id path parameter")
    })
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

pub async fn create_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateGroupRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<GroupResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        let group_id = next_entity_id(&state.id_generator)?;
        let conversation_id = next_entity_id(&state.id_generator)?.to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let max_members =
            im_domain_core::space::normalize_chat_group_max_members(request.max_members);

        let record = GroupRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            group_id,
            space_id: Some(space_id),
            group_name: request.group_name,
            group_type: request.group_type.unwrap_or_else(|| "normal".to_string()),
            owner_user_id: auth.actor_id.clone(),
            conversation_id: Some(conversation_id.clone()),
            max_members,
            description: request.description,
            avatar_url: request.avatar_url,
            announcement: None,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let owner_member = GroupMemberRecord {
            tenant_id: record.tenant_id.clone(),
            organization_id: record.organization_id.clone(),
            group_id: record.group_id,
            user_id: record.owner_user_id.clone(),
            role: "owner".to_owned(),
            nickname: None,
            mute_until: None,
            joined_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        };

        persist_group_created(&state, &auth, &record, &owner_member)?;

        if let Some(binder) = state.group_conversation_binder.as_ref()
            && let Err(error) =
                binder.create_group_conversation(CreateSpaceGroupConversationInput {
                    tenant_id: auth.tenant_id.clone(),
                    organization_id: auth.organization_id.clone(),
                    conversation_id: conversation_id.clone(),
                    group_name: record.group_name.clone(),
                    creator_user_id: auth.actor_id.clone(),
                    max_members,
                })
        {
            tracing::error!(error = %error, group_id, "failed to bind group conversation");
            if let Err(compensate_error) =
                persist_group_deleted(&state, &auth, group_id, now.as_str())
            {
                tracing::error!(
                    error = ?compensate_error,
                    group_id,
                    "failed to compensate group record after conversation bind failure"
                );
            }
            return Err(ApiProblem::internal_server_error(
                "failed to bind group conversation",
            ));
        }

        Ok(resource_item(GroupResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_groups(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<GroupResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        let paging = resolve_keyset_page(&query)?;
        let cursor_group_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());

        match state.group_store.list_by_space(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            space_id,
            paging.cursor_sort_value.as_deref(),
            cursor_group_id,
            paging.fetch_limit(),
        ) {
            Ok(records) => {
                let items = records.into_iter().map(GroupResponse::from).collect();
                Ok(keyset_list_page(
                    items,
                    paging.page_size,
                    |item: &GroupResponse| (item.created_at.clone(), item.group_id.clone()),
                ))
            }
            Err(error) => {
                tracing::error!(error = ?error, "failed to list groups for space {space_id}");
                Err(ApiProblem::internal_server_error("failed to list groups"))
            }
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<GroupResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        let record = load_group_in_space(&state, &auth, space_id, group_id)?;
        Ok(resource_item(GroupResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
    Json(request): Json<UpdateGroupRequest>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let gid = parse_group_id(group_id.as_str())?;
        let now = chrono::Utc::now().to_rfc3339();
        let mut record = load_group_in_space(&state, &auth, space_id, gid)?;
        if record.owner_user_id != auth.actor_id {
            tracing::warn!(
                user_id = %auth.actor_id,
                group_id = %gid,
                owner_user_id = %record.owner_user_id,
                "ownership check failed for update_group"
            );
            return Err(ApiProblem::forbidden("group ownership check failed"));
        }
        if let Some(name) = request.group_name {
            record.group_name = name;
        }
        if let Some(desc) = request.description {
            record.description = Some(desc);
        }
        if let Some(url) = request.avatar_url {
            record.avatar_url = Some(url);
        }
        if let Some(ann) = request.announcement {
            record.announcement = Some(ann);
        }
        record.updated_at = now;
        persist_group_updated(&state, &auth, &record)?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferGroupOwnerRequest {
    pub new_owner_user_id: String,
}

fn sync_conversation_transfer_group_owner(
    state: &AppState,
    auth: &AppContext,
    group: &GroupRecord,
    new_owner_user_id: &str,
) -> Result<(), ApiProblem> {
    let Some(conversation_id) = group.conversation_id.as_deref() else {
        return Ok(());
    };
    let Some(binder) = state.group_conversation_binder.as_ref() else {
        return Ok(());
    };
    binder
        .transfer_group_owner(TransferSpaceGroupOwnerInput {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            conversation_id: conversation_id.to_owned(),
            current_owner_user_id: group.owner_user_id.clone(),
            new_owner_user_id: new_owner_user_id.to_owned(),
            actor_user_id: auth.actor_id.clone(),
        })
        .map_err(|error| {
            tracing::error!(error = %error, "failed to transfer group owner in conversation");
            ApiProblem::internal_server_error("failed to transfer group owner in conversation")
        })
}

pub async fn transfer_group_owner(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
    Json(request): Json<TransferGroupOwnerRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<GroupResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let group_id = parse_group_id(group_id.as_str())?;
        let group = load_group_in_space(&state, &auth, space_id, group_id)?;

        if auth.actor_id != group.owner_user_id {
            return Err(ApiProblem::forbidden(
                "only the group owner can transfer ownership",
            ));
        }

        let new_owner_user_id = request.new_owner_user_id.trim();
        if new_owner_user_id.is_empty() {
            return Err(ApiProblem::bad_request("newOwnerUserId is required"));
        }
        if new_owner_user_id == group.owner_user_id {
            return Err(ApiProblem::bad_request(
                "new owner must differ from the current owner",
            ));
        }

        state
            .group_member_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
                new_owner_user_id,
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load prospective group owner member");
                ApiProblem::internal_server_error("failed to load prospective group owner member")
            })?
            .ok_or_else(|| ApiProblem::bad_request("new owner must be an existing group member"))?;

        let updated_at = chrono::Utc::now().to_rfc3339();
        persist_group_owner_transferred(
            &state,
            &auth,
            group_id,
            group.owner_user_id.as_str(),
            new_owner_user_id,
            updated_at.as_str(),
        )?;
        let updated_group = state
            .group_store
            .get_by_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                group_id,
            )
            .map_err(|error| {
                tracing::error!(error = ?error, group_id, "failed to reload group after owner transfer");
                ApiProblem::internal_server_error("failed to transfer group owner")
            })?
            .ok_or_else(|| ApiProblem::not_found("group not found"))?;

        sync_conversation_transfer_group_owner(&state, &auth, &group, new_owner_user_id)?;
        Ok(resource_item(GroupResponse::from(updated_group)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn delete_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, group_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let gid = parse_group_id(group_id.as_str())?;
        let record = load_group_in_space(&state, &auth, space_id, gid)?;
        if record.owner_user_id != auth.actor_id {
            tracing::warn!(
                user_id = %auth.actor_id,
                group_id = %gid,
                owner_user_id = %record.owner_user_id,
                "ownership check failed for delete_group"
            );
            return Err(ApiProblem::forbidden("group ownership check failed"));
        }
        let deleted_at = chrono::Utc::now().to_rfc3339();
        persist_group_deleted(&state, &auth, gid, deleted_at.as_str())?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
