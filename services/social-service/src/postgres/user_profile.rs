//! User profile API handlers.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_profile_store::UserProfileRecord;

use crate::postgres::http::PostgresAppState;
use crate::postgres::service_http::require_request_scope;

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub user_id: String,
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
    pub im_online_status: String,
    pub last_active_at: Option<String>,
}

impl From<UserProfileRecord> for UserProfileResponse {
    fn from(record: UserProfileRecord) -> Self {
        Self {
            user_id: record.user_id,
            im_nickname: record.im_nickname,
            im_avatar_url: record.im_avatar_url,
            im_status_message: record.im_status_message,
            im_online_status: record.im_online_status,
            last_active_at: record.last_active_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
}

pub async fn get_user_profile(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;

    match state.user_profile_store.get_by_user_id(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        user_id.as_str(),
    ) {
        Ok(Some(record)) => Ok(Json(UserProfileResponse::from(record))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user_profile(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserProfileRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    if scope.user_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();
    let record = UserProfileRecord {
        tenant_id: scope.tenant_id,
        organization_id: scope.organization_id,
        user_id,
        im_nickname: request.im_nickname,
        im_avatar_url: request.im_avatar_url,
        im_status_message: request.im_status_message,
        im_online_status: "online".to_string(),
        last_active_at: Some(now.clone()),
        created_at: now.clone(),
        updated_at: now,
    };

    match state.user_profile_store.upsert_profile(&record) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
