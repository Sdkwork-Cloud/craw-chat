//! Space API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;

use im_adapters_social_postgres::organization_store::SpaceRecord;

use crate::http::AppState;
use crate::id::next_entity_id;

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

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateSpaceRequest>,
) -> Response {
    let result: ApiResult<SpaceResponse> = (|| {
        // Validate max_members if provided
        let max_members = request.max_members.unwrap_or(10000);
        if max_members < 2 || max_members > 10000 {
            tracing::warn!(max_members, "max_members out of valid range");
            return Err(ApiProblem::bad_request(
                "validation failed: max_members out of range",
            ));
        }

        let space_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();

        let record = SpaceRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            space_id,
            space_name: request.space_name,
            space_type: request
                .space_type
                .unwrap_or_else(|| "organization".to_string()),
            owner_user_id: auth.actor_id,
            description: request.description,
            avatar_url: request.avatar_url,
            max_members,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now,
        };

        match state.space_store.insert(&record) {
            Ok(()) => Ok(SpaceResponse::from(record)),
            Err(error) => {
                tracing::error!(error = ?error, "failed to insert space record");
                Err(ApiProblem::internal_server_error("failed to insert space"))
            }
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
    let result: ApiResult<Vec<SpaceResponse>> = (|| {
        let limit = query.limit.unwrap_or(20);

        match state.space_store.list_by_owner(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            limit,
        ) {
            Ok(records) => {
                Ok(records.into_iter().map(SpaceResponse::from).collect())
            }
            Err(error) => {
                tracing::error!(error = ?error, "failed to list spaces");
                Err(ApiProblem::internal_server_error("failed to list spaces"))
            }
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_space(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
) -> Response {
    let result: ApiResult<SpaceResponse> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;

        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(record)) => {
                // IDOR fix (SECURITY_SPEC §4.2): only the space owner may read
                // space metadata. Without this check, any authenticated tenant
                // member could enumerate spaces by ID.
                if record.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %record.owner_user_id,
                        space_id = sid,
                        "space ownership check failed for get_space"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
                Ok(SpaceResponse::from(record))
            }
            Ok(None) => Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid}");
                Err(ApiProblem::internal_server_error("failed to get space"))
            }
        }
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
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;

        // Validate max_members if provided
        if let Some(max) = request.max_members {
            if max < 2 || max > 10000 {
                tracing::warn!(max_members = max, "max_members out of valid range");
                return Err(ApiProblem::bad_request(
                    "validation failed: max_members out of range",
                ));
            }
        }

        let now = chrono::Utc::now().to_rfc3339();

        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(mut record)) => {
                // IDOR fix (SECURITY_SPEC §4.2): only the space owner may mutate
                // space settings. Without this check, any authenticated tenant
                // member could rename or reconfigure any space by ID.
                if record.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %record.owner_user_id,
                        space_id = sid,
                        "space ownership check failed for update_space"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
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

                match state.space_store.update(&record) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to update space {sid}");
                        Err(ApiProblem::internal_server_error("failed to update space"))
                    }
                }
            }
            Ok(None) => Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for update");
                Err(ApiProblem::internal_server_error("failed to get space"))
            }
        }
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
                match state.space_store.delete(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    sid,
                ) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to delete space {sid}");
                        Err(ApiProblem::internal_server_error("failed to delete space"))
                    }
                }
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
