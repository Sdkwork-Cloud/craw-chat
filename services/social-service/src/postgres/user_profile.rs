//! User profile API handlers.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::user_profile_store::UserProfileRecord;

use crate::postgres::http::PostgresAppState;

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
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(user_id): Path<String>,
) -> Response {
    let result: ApiResult<UserProfileResponse> = (|| {
        let record = state
            .user_profile_store
            .get_by_user_id(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                user_id.as_str(),
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to read user profile"))?
            .ok_or_else(|| ApiProblem::not_found("user profile not found"))?;
        Ok(UserProfileResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_user_profile(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserProfileRequest>,
) -> Response {
    let result: Result<Response, ApiProblem> = (|| {
        if auth.actor_id != user_id {
            return Err(ApiProblem::forbidden("user can only update own profile"));
        }

        let now = chrono::Utc::now().to_rfc3339();
        let record = UserProfileRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            user_id,
            im_nickname: request.im_nickname,
            im_avatar_url: request.im_avatar_url,
            im_status_message: request.im_status_message,
            im_online_status: "online".to_string(),
            last_active_at: Some(now.clone()),
            created_at: now.clone(),
            updated_at: now,
        };

        state
            .user_profile_store
            .upsert_profile(&record)
            .map_err(|_| ApiProblem::internal_server_error("failed to update user profile"))?;
        no_content(&ctx)
    })();
    finish_api_response(&ctx, result)
}
