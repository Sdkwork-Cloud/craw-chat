//! Friendship API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::friendship_store::FriendshipRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::service_http::require_request_scope;

#[derive(Debug, Serialize)]
pub struct FriendshipResponse {
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub status: String,
    pub established_at: Option<String>,
}

impl From<FriendshipRecord> for FriendshipResponse {
    fn from(record: FriendshipRecord) -> Self {
        Self {
            friendship_id: record.friendship_id.to_string(),
            user_low_id: record.user_low_id,
            user_high_id: record.user_high_id,
            status: record.status,
            established_at: record.established_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn list_friends(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let limit = query.limit.unwrap_or(20);

    match state.friendship_store.list_by_user(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.user_id.as_str(),
        "active",
        limit,
    ) {
        Ok(records) => {
            let response: Vec<FriendshipResponse> =
                records.into_iter().map(FriendshipResponse::from).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_friendship(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(friendship_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let fid: i64 = friendship_id.parse().unwrap_or(0);

    match state.friendship_store.get_by_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        fid,
    ) {
        Ok(Some(record)) => Ok(Json(FriendshipResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove_friendship(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(friendship_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let fid: i64 = friendship_id.parse().unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    match state.friendship_store.update_status(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        fid,
        "removed",
        &now,
    ) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
