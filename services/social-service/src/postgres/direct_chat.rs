//! Direct chat API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::direct_chat_store::DirectChatRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::id::next_entity_id;
use crate::postgres::service_http::require_request_scope;

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
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Json(request): Json<CreateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let direct_chat_id = next_entity_id(&state.id_generator)?;
    let now = chrono::Utc::now().to_rfc3339();
    let pair_hash = normalize_pair_hash(scope.user_id.as_str(), &request.target_user_id);

    let record = DirectChatRecord {
        tenant_id: scope.tenant_id,
        organization_id: scope.organization_id,
        direct_chat_id,
        left_actor_kind: "user".to_string(),
        left_actor_id: scope.user_id.clone(),
        right_actor_kind: "user".to_string(),
        right_actor_id: request.target_user_id,
        pair_hash,
        status: "active".to_string(),
        conversation_id: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match state.direct_chat_store.insert(&record) {
        Ok(()) => {
            let response = DirectChatResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_direct_chats(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let limit = query.limit.unwrap_or(20);

    match state.direct_chat_store.list_by_actor(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.user_id.as_str(),
        "active",
        limit,
    ) {
        Ok(records) => {
            let response: Vec<DirectChatResponse> =
                records.into_iter().map(DirectChatResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_direct_chat(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(direct_chat_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let dcid: i64 = direct_chat_id.parse().unwrap_or(0);

    match state.direct_chat_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        dcid,
    ) {
        Ok(Some(record)) => Ok(Json(DirectChatResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_direct_chat(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(direct_chat_id): Path<String>,
    Json(request): Json<UpdateDirectChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let dcid: i64 = direct_chat_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();
    let status = request.status.as_deref().unwrap_or("active");

    match state.direct_chat_store.update_status(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        dcid,
        status,
        &now,
    ) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
