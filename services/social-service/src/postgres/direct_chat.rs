//! Direct chat API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::sha256_hash;
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::direct_chat_store::DirectChatRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::id::next_entity_id;

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

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

fn normalize_pair_hash(left: &str, right: &str) -> String {
    let (low, high) = if left < right {
        (left, right)
    } else {
        (right, left)
    };
    sha256_hash(format!("{low}:{high}").as_bytes())
}

pub async fn create_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Json(request): Json<CreateDirectChatRequest>,
) -> Response {
    let result: ApiResult<DirectChatResponse> = (|| {
        let direct_chat_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();
        let pair_hash = normalize_pair_hash(auth.actor_id.as_str(), &request.target_user_id);

        let record = DirectChatRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            direct_chat_id,
            left_actor_kind: "user".to_string(),
            left_actor_id: auth.actor_id,
            right_actor_kind: "user".to_string(),
            right_actor_id: request.target_user_id,
            pair_hash,
            status: "active".to_string(),
            conversation_id: None,
            created_at: now.clone(),
            updated_at: now,
        };

        state
            .direct_chat_store
            .insert(&record)
            .map_err(|_| ApiProblem::internal_server_error("failed to insert direct chat"))?;
        Ok(DirectChatResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_direct_chats(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<DirectChatResponse>> = (|| {
        let limit = query.limit.unwrap_or(20);
        let records = state
            .direct_chat_store
            .list_by_actor(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                "active",
                limit,
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to list direct chats"))?;
        Ok(records.into_iter().map(DirectChatResponse::from).collect())
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(direct_chat_id): Path<String>,
) -> Response {
    let result: ApiResult<DirectChatResponse> = (|| {
        let dcid: i64 = direct_chat_id.parse().unwrap_or(0);
        let record = state
            .direct_chat_store
            .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), dcid)
            .map_err(|_| ApiProblem::internal_server_error("failed to read direct chat"))?
            .ok_or_else(|| ApiProblem::not_found("direct chat not found"))?;
        Ok(DirectChatResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_direct_chat(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(direct_chat_id): Path<String>,
    Json(request): Json<UpdateDirectChatRequest>,
) -> Response {
    let result: Result<Response, ApiProblem> = (|| {
        let dcid: i64 = direct_chat_id.parse().unwrap_or(0);
        let now = chrono::Utc::now().to_rfc3339();
        let status = request.status.as_deref().unwrap_or("active");
        state
            .direct_chat_store
            .update_status(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                dcid,
                status,
                &now,
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to update direct chat"))?;
        no_content(&ctx)
    })();
    finish_api_response(&ctx, result)
}
