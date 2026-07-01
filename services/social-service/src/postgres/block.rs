//! Block API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_block_store::UserBlockRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::id::next_entity_id;

#[derive(Debug, Deserialize)]
pub struct BlockUserRequest {
    pub blocked_user_id: String,
    pub scope: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlockResponse {
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: String,
    pub created_at: String,
}

impl From<UserBlockRecord> for BlockResponse {
    fn from(record: UserBlockRecord) -> Self {
        Self {
            block_id: record.block_id.to_string(),
            blocker_user_id: record.blocker_user_id,
            blocked_user_id: record.blocked_user_id,
            scope: record.scope,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn block_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Json(request): Json<BlockUserRequest>,
) -> Response {
    let result: ApiResult<BlockResponse> = (|| {
        let block_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();

        let record = UserBlockRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            block_id,
            blocker_user_id: auth.actor_id,
            blocked_user_id: request.blocked_user_id,
            scope: request.scope.unwrap_or_else(|| "all".to_string()),
            direct_chat_id: None,
            reason: request.reason,
            expires_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        state
            .user_block_store
            .insert(&record)
            .map_err(|_| ApiProblem::internal_server_error("failed to insert block record"))?;
        Ok(BlockResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_blocks(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<BlockResponse>> = (|| {
        let limit = query.limit.unwrap_or(20);
        let records = state
            .user_block_store
            .list_by_blocker(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                limit,
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to list block records"))?;
        Ok(records.into_iter().map(BlockResponse::from).collect())
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_block(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(block_id): Path<String>,
) -> Response {
    let result: ApiResult<BlockResponse> = (|| {
        let bid: i64 = block_id.parse().unwrap_or(0);
        let record = state
            .user_block_store
            .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), bid)
            .map_err(|_| ApiProblem::internal_server_error("failed to read block record"))?
            .ok_or_else(|| ApiProblem::not_found("block record not found"))?;
        Ok(BlockResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn unblock_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(block_id): Path<String>,
) -> Response {
    let result: Result<Response, ApiProblem> = (|| {
        let bid: i64 = block_id
            .parse()
            .map_err(|_| ApiProblem::bad_request("invalid block id"))?;
        match state.user_block_store.delete_by_blocker(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            bid,
            auth.actor_id.as_str(),
        ) {
            Ok(true) => no_content(&ctx),
            Ok(false) => Err(ApiProblem::not_found("block record not found")),
            Err(_) => Err(ApiProblem::internal_server_error("failed to delete block record")),
        }
    })();
    finish_api_response(&ctx, result)
}
