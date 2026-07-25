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

use im_adapters_social_postgres::{
    direct_chat_store::{DirectChatActorListQuery, DirectChatRecord},
    wire_id::parse_social_entity_id,
};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::postgres::access::{ensure_direct_chat_participant, social_principal_user_id};
use crate::postgres::http::PostgresAppState;
use crate::postgres::list_query::{ListQuery, resolve_keyset_page};

#[derive(Debug, Deserialize)]
pub struct CreateDirectChatRequest {
    pub target_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct DirectChatResponse {
    pub direct_chat_id: String,
    #[serde(skip)]
    sort_id: i64,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub status: String,
    pub conversation_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<DirectChatRecord> for DirectChatResponse {
    fn from(record: DirectChatRecord) -> Self {
        Self {
            direct_chat_id: record.direct_chat_id.to_string(),
            sort_id: record.direct_chat_id,
            left_actor_id: record.left_actor_id,
            right_actor_id: record.right_actor_id,
            status: record.status,
            conversation_id: record.conversation_id,
            created_at: record.created_at,
            updated_at: record.updated_at,
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
    let result: ApiResult<SdkWorkPageData<DirectChatResponse>> =
        crate::postgres::http::run_blocking_postgres_call(state, move |state| {
            let paging = resolve_keyset_page(&query)?;
            let records = state
                .direct_chat_store
                .list_by_actor(DirectChatActorListQuery {
                    tenant_id: auth.tenant_id.as_str(),
                    organization_id: auth.organization_id.as_str(),
                    actor_id: social_principal_user_id(&auth),
                    status: "active",
                    cursor_updated_at: paging.cursor_created_at.as_deref(),
                    cursor_direct_chat_id: paging.cursor_direct_chat_id,
                    limit: paging.fetch_limit(),
                })
                .map_err(|_| ApiProblem::internal_server_error("failed to list direct chats"))?;
            let items: Vec<DirectChatResponse> =
                records.into_iter().map(DirectChatResponse::from).collect();
            Ok(keyset_list_page(
                items,
                paging.page_size,
                |item: &DirectChatResponse| (item.updated_at.clone(), item.sort_id),
            ))
        })
        .await;
    finish_api_json(&ctx, result)
}

pub async fn get_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(direct_chat_id): Path<String>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<DirectChatResponse>> =
        crate::postgres::http::run_blocking_postgres_call(state, move |state| {
            let dcid = parse_social_entity_id(direct_chat_id.as_str()).map_err(|_| {
                ApiProblem::bad_request(
                    "direct_chat_id must be a canonical positive signed int64 string",
                )
            })?;
            let record = state
                .direct_chat_store
                .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), dcid)
                .map_err(|_| ApiProblem::internal_server_error("failed to read direct chat"))?
                .ok_or_else(|| ApiProblem::not_found("direct chat not found"))?;
            ensure_direct_chat_participant(&auth, &record)?;
            Ok(resource_item(DirectChatResponse::from(record)))
        })
        .await;
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
