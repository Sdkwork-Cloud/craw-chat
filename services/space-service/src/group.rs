//! Group API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::organization_store::GroupRecord;

use crate::http::AppState;
use crate::id::next_entity_id;
use crate::service_http::require_request_scope;

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
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;

    let group_id = next_entity_id(&state.id_generator)?;
    let now = chrono::Utc::now().to_rfc3339();

    let record = GroupRecord {
        tenant_id: scope.tenant_id,
        organization_id: scope.organization_id,
        group_id,
        space_id: space_id.parse().ok(),
        group_name: request.group_name,
        group_type: request.group_type.unwrap_or_else(|| "normal".to_string()),
        owner_user_id: scope.user_id,
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
        Ok(()) => {
            let response = GroupResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_groups(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let sid: i64 = space_id.parse().map_err(|_| {
        tracing::warn!("invalid space_id path parameter: {space_id}");
        StatusCode::BAD_REQUEST
    })?;
    let limit = query.limit.unwrap_or(20);

    match state.group_store.list_by_space(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        sid,
        limit,
    ) {
        Ok(records) => {
            let response: Vec<GroupResponse> =
                records.into_iter().map(GroupResponse::from).collect();
            Ok(Json(response))
        }
        Err(error) => {
            tracing::error!(error = ?error, "failed to list groups for space {sid}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_group(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path((_space_id, group_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let gid: i64 = group_id.parse().map_err(|_| {
        tracing::warn!("invalid group_id path parameter: {group_id}");
        StatusCode::BAD_REQUEST
    })?;

    match state.group_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        gid,
    ) {
        Ok(Some(record)) => Ok(Json(GroupResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            tracing::error!(error = ?error, "failed to get group {gid}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_group(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path((_space_id, group_id)): Path<(String, String)>,
    Json(request): Json<UpdateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let gid: i64 = group_id.parse().map_err(|_| {
        tracing::warn!("invalid group_id path parameter: {group_id}");
        StatusCode::BAD_REQUEST
    })?;
    let now = chrono::Utc::now().to_rfc3339();

    match state.group_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        gid,
    ) {
        Ok(Some(mut record)) => {
            if record.owner_user_id != scope.user_id {
                tracing::warn!(
                    user_id = %scope.user_id,
                    group_id = %gid,
                    owner_user_id = %record.owner_user_id,
                    "ownership check failed for update_group"
                );
                return Err(StatusCode::FORBIDDEN);
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
                Ok(()) => Ok(StatusCode::NO_CONTENT),
                Err(error) => {
                    tracing::error!(error = ?error, "failed to update group {gid}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            tracing::error!(error = ?error, "failed to get group {gid} for update");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_group(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path((_space_id, group_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let gid: i64 = group_id.parse().map_err(|_| {
        tracing::warn!("invalid group_id path parameter: {group_id}");
        StatusCode::BAD_REQUEST
    })?;

    // 先获取记录验证所有权，防止 IDOR 越权删除
    match state.group_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        gid,
    ) {
        Ok(Some(record)) => {
            if record.owner_user_id != scope.user_id {
                tracing::warn!(
                    user_id = %scope.user_id,
                    group_id = %gid,
                    owner_user_id = %record.owner_user_id,
                    "ownership check failed for delete_group"
                );
                return Err(StatusCode::FORBIDDEN);
            }
            match state.group_store.delete(
                scope.tenant_id.as_str(),
                scope.organization_id.as_str(),
                gid,
            ) {
                Ok(()) => Ok(StatusCode::NO_CONTENT),
                Err(error) => {
                    tracing::error!(error = ?error, "failed to delete group {gid}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            tracing::error!(error = ?error, "failed to get group {gid} for delete");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
