//! Group API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;

use im_adapters_social_postgres::organization_store::GroupRecord;

use crate::http::AppState;
use crate::id::next_entity_id;

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

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateGroupRequest>,
) -> Response {
    let result: ApiResult<GroupResponse> = (|| {
        let group_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();

        let record = GroupRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            group_id,
            space_id: space_id.parse().ok(),
            group_name: request.group_name,
            group_type: request.group_type.unwrap_or_else(|| "normal".to_string()),
            owner_user_id: auth.actor_id,
            conversation_id: None,
            max_members: request.max_members.unwrap_or(500),
            description: request.description,
            avatar_url: request.avatar_url,
            announcement: None,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now,
        };

        match state.group_store.insert(&record) {
            Ok(()) => Ok(GroupResponse::from(record)),
            Err(error) => {
                tracing::error!(error = ?error, "failed to insert group record");
                Err(ApiProblem::internal_server_error("failed to insert group"))
            }
        }
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
    let result: ApiResult<Vec<GroupResponse>> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;
        let limit = query.limit.unwrap_or(20);

        match state.group_store.list_by_space(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
            limit,
        ) {
            Ok(records) => {
                Ok(records.into_iter().map(GroupResponse::from).collect())
            }
            Err(error) => {
                tracing::error!(error = ?error, "failed to list groups for space {sid}");
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
    Path((_space_id, group_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<GroupResponse> = (|| {
        let gid: i64 = group_id.parse().map_err(|_| {
            tracing::warn!("invalid group_id path parameter: {group_id}");
            ApiProblem::bad_request("invalid group_id path parameter")
        })?;

        match state.group_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            gid,
        ) {
            Ok(Some(record)) => Ok(GroupResponse::from(record)),
            Ok(None) => Err(ApiProblem::not_found("group not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get group {gid}");
                Err(ApiProblem::internal_server_error("failed to get group"))
            }
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((_space_id, group_id)): Path<(String, String)>,
    Json(request): Json<UpdateGroupRequest>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let gid: i64 = group_id.parse().map_err(|_| {
            tracing::warn!("invalid group_id path parameter: {group_id}");
            ApiProblem::bad_request("invalid group_id path parameter")
        })?;
        let now = chrono::Utc::now().to_rfc3339();

        match state.group_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            gid,
        ) {
            Ok(Some(mut record)) => {
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

                match state.group_store.update(&record) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to update group {gid}");
                        Err(ApiProblem::internal_server_error("failed to update group"))
                    }
                }
            }
            Ok(None) => Err(ApiProblem::not_found("group not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get group {gid} for update");
                Err(ApiProblem::internal_server_error("failed to get group"))
            }
        }
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn delete_group(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((_space_id, group_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let gid: i64 = group_id.parse().map_err(|_| {
            tracing::warn!("invalid group_id path parameter: {group_id}");
            ApiProblem::bad_request("invalid group_id path parameter")
        })?;

        // 先获取记录验证所有权，防止 IDOR 越权删除
        match state.group_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            gid,
        ) {
            Ok(Some(record)) => {
                if record.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        group_id = %gid,
                        owner_user_id = %record.owner_user_id,
                        "ownership check failed for delete_group"
                    );
                    return Err(ApiProblem::forbidden("group ownership check failed"));
                }
                match state.group_store.delete(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    gid,
                ) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to delete group {gid}");
                        Err(ApiProblem::internal_server_error("failed to delete group"))
                    }
                }
            }
            Ok(None) => Err(ApiProblem::not_found("group not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get group {gid} for delete");
                Err(ApiProblem::internal_server_error("failed to get group"))
            }
        }
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
