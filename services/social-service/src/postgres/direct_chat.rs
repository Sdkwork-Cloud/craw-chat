//! Direct chat API handlers.

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

use im_adapters_social_postgres::direct_chat_store::DirectChatRecord;

use crate::api_payload::{bounded_sql_list_page, resource_item};
use crate::postgres::access::{ensure_direct_chat_participant, social_principal_user_id};
use crate::postgres::http::PostgresAppState;
use crate::postgres::list_query::{resolve_list_page, sql_fetch_limit, sql_fetch_offset, ListQuery};

#[derive(Debug, Deserialize)]
pub struct CreateDirectChatRequest {
    pub target_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct DirectChatResponse {
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub status: String,
    pub conversation_id: Option<String>,
    pub created_at: String,
}

impl From<DirectChatRecord> for DirectChatResponse {
    fn from(record: DirectChatRecord) -> Self {
        Self {
            direct_chat_id: record.direct_chat_id.to_string(),
            left_actor_id: record.left_actor_id,
            right_actor_id: record.right_actor_id,
            status: record.status,
            conversation_id: record.conversation_id,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateDirectChatRequest {
    pub status: Option<String>,
}

pub async fn create_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<PostgresAppState>,
    Json(_request): Json<CreateDirectChatRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<DirectChatResponse>> =
        Err(crate::postgres::mutation_policy::supplemental_social_mutation_forbidden());
    finish_api_json(&ctx, result)
}

pub async fn list_direct_chats(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<DirectChatResponse>> = (|| {
        let paging = resolve_list_page(&query)?;
        let records = state
            .direct_chat_store
            .list_by_actor(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                social_principal_user_id(&auth),
                "active",
                sql_fetch_limit(paging),
                sql_fetch_offset(paging),
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to list direct chats"))?;
        let items = records.into_iter().map(DirectChatResponse::from).collect();
        Ok(bounded_sql_list_page(items, paging.page_size, paging.offset))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(direct_chat_id): Path<String>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<DirectChatResponse>> = (|| {
        let dcid: i64 = direct_chat_id.parse().unwrap_or(0);
        let record = state
            .direct_chat_store
            .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), dcid)
            .map_err(|_| ApiProblem::internal_server_error("failed to read direct chat"))?
            .ok_or_else(|| ApiProblem::not_found("direct chat not found"))?;
        ensure_direct_chat_participant(&auth, &record)?;
        Ok(resource_item(DirectChatResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<PostgresAppState>,
    Path(_direct_chat_id): Path<String>,
    Json(_request): Json<UpdateDirectChatRequest>,
) -> Response {
    let result: Result<Response, ApiProblem> =
        Err(crate::postgres::mutation_policy::supplemental_social_mutation_forbidden());
    finish_api_response(&ctx, result)
}
