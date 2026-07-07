//! Space API handlers.

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

use im_adapters_social_postgres::governance_store::SpaceMemberRecord;
use im_adapters_social_postgres::organization_store::SpaceRecord;

use crate::api_payload::{keyset_list_page, resource_item};
use crate::list_query::{ListQuery, resolve_keyset_page};

use crate::http::AppState;
use crate::id::next_entity_id;
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, load_space, parse_space_id,
};
use crate::write_authority::{persist_space_created, persist_space_deleted, persist_space_updated};

#[derive(Debug, Deserialize)]
pub struct CreateSpaceRequest {
    pub space_name: String,
    pub space_type: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SpaceResponse {
    pub space_id: String,
    pub space_name: String,
    pub space_type: String,
    pub owner_user_id: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: i32,
    pub created_at: String,
}

impl From<SpaceRecord> for SpaceResponse {
    fn from(record: SpaceRecord) -> Self {
        Self {
            space_id: record.space_id.to_string(),
            space_name: record.space_name,
            space_type: record.space_type,
            owner_user_id: record.owner_user_id,
            description: record.description,
            avatar_url: record.avatar_url,
            max_members: record.max_members,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpaceRequest {
    pub space_name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: Option<i32>,
}

pub async fn create_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateSpaceRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<SpaceResponse>> = (|| {
        // Validate max_members if provided
        let max_members = request.max_members.unwrap_or(10000);
        if !(2..=10000).contains(&max_members) {
            tracing::warn!(max_members, "max_members out of valid range");
            return Err(ApiProblem::bad_request(
                "validation failed: max_members out of range",
            ));
        }

        let space_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();

        let record = SpaceRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            space_id,
            space_name: request.space_name,
            space_type: request
                .space_type
                .unwrap_or_else(|| "organization".to_string()),
            owner_user_id: auth.actor_id.clone(),
            description: request.description,
            avatar_url: request.avatar_url,
            max_members,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let owner_member = SpaceMemberRecord {
            tenant_id: record.tenant_id.clone(),
            organization_id: record.organization_id.clone(),
            space_id: record.space_id,
            user_id: record.owner_user_id.clone(),
            role: "owner".to_owned(),
            nickname: None,
            joined_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        };

        match persist_space_created(&state, &auth, &record, &owner_member) {
            Ok(()) => Ok(resource_item(SpaceResponse::from(record))),
            Err(error) => Err(error),
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_spaces(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<SpaceResponse>> = (|| {
        let paging = resolve_keyset_page(&query)?;
        let cursor_space_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());
        let records = match state.space_store.list_accessible_by_user(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            paging.cursor_sort_value.as_deref(),
            cursor_space_id,
            paging.fetch_limit(),
        ) {
            Ok(records) => records,
            Err(error) => {
                tracing::error!(error = ?error, "failed to list accessible spaces");
                return Err(ApiProblem::internal_server_error("failed to list spaces"));
            }
        };
        Ok(keyset_list_page(
            records.into_iter().map(SpaceResponse::from).collect(),
            paging.page_size,
            |item: &SpaceResponse| (item.created_at.clone(), item.space_id.clone()),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<SpaceResponse>> = (|| {
        let sid = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, sid)?;
        actor_can_read_space(&state, &auth, &space)?;
        Ok(resource_item(SpaceResponse::from(space)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<UpdateSpaceRequest>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let sid = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, sid)?;
        actor_can_manage_space(&state, &auth, &space)?;

        // Validate max_members if provided
        if let Some(max) = request.max_members
            && !(2..=10000).contains(&max)
        {
            tracing::warn!(max_members = max, "max_members out of valid range");
            return Err(ApiProblem::bad_request(
                "validation failed: max_members out of range",
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();
        let mut record = space;
        if let Some(name) = request.space_name {
            record.space_name = name;
        }
        if let Some(desc) = request.description {
            record.description = Some(desc);
        }
        if let Some(url) = request.avatar_url {
            record.avatar_url = Some(url);
        }
        if let Some(max) = request.max_members {
            record.max_members = max;
        }
        record.updated_at = now;

        persist_space_updated(&state, &auth, &record)?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn delete_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;

        // IDOR fix (SECURITY_SPEC §4.2): fetch the record first to verify
        // ownership before deleting. Without this check, any authenticated
        // tenant member could delete any space by ID.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(record)) => {
                if record.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %record.owner_user_id,
                        space_id = sid,
                        "space ownership check failed for delete_space"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
                let deleted_at = chrono::Utc::now().to_rfc3339();
                persist_space_deleted(&state, &auth, sid, deleted_at.as_str())?;
                Ok(())
            }
            Ok(None) => Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for delete");
                Err(ApiProblem::internal_server_error("failed to get space"))
            }
        }
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
