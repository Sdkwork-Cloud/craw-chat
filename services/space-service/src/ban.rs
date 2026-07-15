//! Ban API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::governance_store::{BanRecord, BanTargetListQuery};
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::http::AppState;
use crate::id::next_entity_id;
use crate::list_query::{ListQuery, resolve_keyset_page};
use crate::space_access::{actor_can_manage_space, load_space, parse_space_id};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanUserRequest {
    pub user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BanResponse {
    pub ban_id: String,
    pub banned_user_id: String,
    pub banned_by_user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
}

impl From<BanRecord> for BanResponse {
    fn from(record: BanRecord) -> Self {
        Self {
            ban_id: record.ban_id.to_string(),
            banned_user_id: record.banned_user_id,
            banned_by_user_id: record.banned_by_user_id,
            reason: record.reason,
            expires_at: record.expires_at,
            created_at: record.created_at,
        }
    }
}

pub async fn ban_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<BanUserRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<BanResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        if request.user_id.trim().is_empty() {
            return Err(ApiProblem::bad_request("user_id is required"));
        }
        if request.user_id == space.owner_user_id {
            return Err(ApiProblem::bad_request("space owner cannot be banned"));
        }

        if state
            .ban_store
            .get_active_by_user(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                "space",
                space_id,
                request.user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to check existing ban");
                ApiProblem::internal_server_error("failed to check existing ban")
            })?
            .is_some()
        {
            return Err(ApiProblem::bad_request("user is already banned"));
        }

        let now = chrono::Utc::now().to_rfc3339();
        let record = BanRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            ban_id: next_entity_id(&state.id_generator)?,
            target_type: "space".to_owned(),
            target_id: space_id,
            banned_user_id: request.user_id,
            banned_by_user_id: auth.actor_id.clone(),
            reason: request.reason,
            expires_at: request.expires_at,
            unbanned_at: None,
            unbanned_by_user_id: None,
            created_at: now.clone(),
            updated_at: now,
        };

        state.ban_store.insert(&record).map_err(|error| {
            tracing::error!(error = ?error, "failed to insert ban record");
            ApiProblem::internal_server_error("failed to ban user")
        })?;
        Ok(resource_item(BanResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_bans(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<BanResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        let paging = resolve_keyset_page(&query)?;
        let cursor_ban_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());

        let records = state
            .ban_store
            .list_active_by_target(BanTargetListQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: auth.organization_id.as_str(),
                target_type: "space",
                target_id: space_id,
                cursor_created_at: paging.cursor_sort_value.as_deref(),
                cursor_ban_id,
                limit: paging.fetch_limit(),
            })
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to list bans");
                ApiProblem::internal_server_error("failed to list bans")
            })?;

        let items = records.into_iter().map(BanResponse::from).collect();
        Ok(keyset_list_page(
            items,
            paging.page_size,
            |item: &BanResponse| (item.created_at.clone(), item.ban_id.clone()),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_ban(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<BanResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        state
            .ban_store
            .get_active_by_user(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                "space",
                space_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to get ban");
                ApiProblem::internal_server_error("failed to get ban")
            })?
            .map(BanResponse::from)
            .map(resource_item)
            .ok_or_else(|| ApiProblem::not_found("ban not found"))
    })();
    finish_api_json(&ctx, result)
}

pub async fn unban_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        let mut record = state
            .ban_store
            .get_active_by_user(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                "space",
                space_id,
                user_id.as_str(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to load ban for unban");
                ApiProblem::internal_server_error("failed to load ban")
            })?
            .ok_or_else(|| ApiProblem::not_found("ban not found"))?;

        let now = chrono::Utc::now().to_rfc3339();
        record.unbanned_at = Some(now.clone());
        record.unbanned_by_user_id = Some(auth.actor_id.clone());
        record.updated_at = now;

        state.ban_store.update(&record).map_err(|error| {
            tracing::error!(error = ?error, "failed to unban user");
            ApiProblem::internal_server_error("failed to unban user")
        })?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
