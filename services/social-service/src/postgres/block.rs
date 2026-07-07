//! Block API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_block_store::UserBlockRecord;

use crate::api_payload::{keyset_list_page, resource_item};
use crate::postgres::access::{ensure_block_owner, social_principal_user_id};
use crate::postgres::http::PostgresAppState;
use crate::postgres::list_query::{ListQuery, resolve_keyset_page};

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

pub async fn block_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<PostgresAppState>,
    Json(_request): Json<BlockUserRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<BlockResponse>> =
        Err(crate::postgres::mutation_policy::supplemental_social_mutation_forbidden());
    finish_api_json(&ctx, result)
}

pub async fn list_blocks(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<BlockResponse>> =
        crate::postgres::http::run_blocking_postgres_call(state, move |state| {
            let paging = resolve_keyset_page(&query)?;
            let records = state
                .user_block_store
                .list_by_blocker(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    social_principal_user_id(&auth),
                    paging.cursor_created_at.as_deref(),
                    paging.cursor_block_id,
                    paging.fetch_limit(),
                )
                .map_err(|_| ApiProblem::internal_server_error("failed to list block records"))?;
            let items: Vec<BlockResponse> = records.into_iter().map(BlockResponse::from).collect();
            Ok(keyset_list_page(
                items,
                paging.page_size,
                |item: &BlockResponse| {
                    (item.created_at.clone(), item.block_id.parse().unwrap_or(0))
                },
            ))
        })
        .await;
    finish_api_json(&ctx, result)
}

pub async fn get_block(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(block_id): Path<String>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<BlockResponse>> =
        crate::postgres::http::run_blocking_postgres_call(state, move |state| {
            let bid: i64 = block_id.parse().unwrap_or(0);
            let record = state
                .user_block_store
                .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), bid)
                .map_err(|_| ApiProblem::internal_server_error("failed to read block record"))?
                .ok_or_else(|| ApiProblem::not_found("block record not found"))?;
            ensure_block_owner(&auth, &record)?;
            Ok(resource_item(BlockResponse::from(record)))
        })
        .await;
    finish_api_json(&ctx, result)
}

pub async fn unblock_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<PostgresAppState>,
    Path(_block_id): Path<String>,
) -> Response {
    let result: Result<Response, ApiProblem> =
        Err(crate::postgres::mutation_policy::supplemental_social_mutation_forbidden());
    finish_api_response(&ctx, result)
}
