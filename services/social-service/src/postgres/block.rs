//! Block API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_block_store::UserBlockRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::id::next_entity_id;
use crate::postgres::service_http::require_request_scope;

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
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Json(request): Json<BlockUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let block_id = next_entity_id(&state.id_generator)?;
    let now = chrono::Utc::now().to_rfc3339();

    let record = UserBlockRecord {
        tenant_id: scope.tenant_id,
        organization_id: scope.organization_id,
        block_id,
        blocker_user_id: scope.user_id,
        blocked_user_id: request.blocked_user_id,
        scope: request.scope.unwrap_or_else(|| "all".to_string()),
        direct_chat_id: None,
        reason: request.reason,
        expires_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match state.user_block_store.insert(&record) {
        Ok(()) => {
            let response = BlockResponse::from(record);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_blocks(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let limit = query.limit.unwrap_or(20);

    match state.user_block_store.list_by_blocker(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.user_id.as_str(),
        limit,
    ) {
        Ok(records) => {
            let response: Vec<BlockResponse> =
                records.into_iter().map(BlockResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_block(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let bid: i64 = block_id.parse().unwrap_or(0);

    match state.user_block_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        bid,
    ) {
        Ok(Some(record)) => Ok(Json(BlockResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn unblock_user(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(block_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let bid: i64 = block_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    match state.user_block_store.delete_by_blocker(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        bid,
        scope.user_id.as_str(),
    ) {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
