//! Space member API handlers.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub joined_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub role: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn add_space_member(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "added"})),
    ))
}

pub async fn list_space_members(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<MemberResponse>::new()))
}

pub async fn get_space_member(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn update_space_member(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
    Json(_request): Json<UpdateMemberRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_space_member(
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
