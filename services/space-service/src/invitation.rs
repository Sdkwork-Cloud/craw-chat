//! Invitation API handlers.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateInvitationRequest {
    pub invitee_user_id: Option<String>,
    pub invitee_email: Option<String>,
    pub invitee_phone: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub role: Option<String>,
    pub message: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    pub invitation_id: String,
    pub inviter_user_id: String,
    pub invitee_user_id: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
}

pub async fn create_invitation(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<CreateInvitationRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "created"})),
    ))
}

pub async fn list_invitations(
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<InvitationResponse>::new()))
}

pub async fn get_invitation(
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Err::<(), StatusCode>(StatusCode::NOT_FOUND)
}

pub async fn revoke_invitation(
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

pub async fn accept_invitation(
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
