//! Ban API handlers.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct BanUserRequest {
    pub user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BanResponse {
    pub ban_id: String,
    pub banned_user_id: String,
    pub banned_by_user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn ban_user(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<BanUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "banned"})),
    ))
}

pub async fn list_bans(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<BanResponse>::new()))
}

pub async fn get_ban(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn unban_user(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
