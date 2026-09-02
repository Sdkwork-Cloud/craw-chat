//! Channel access rule API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::governance_store::ChannelAccessRuleRecord;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, created_json, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::http::AppState;
use crate::id::next_entity_id;
use crate::list_query::{ListQuery, resolve_keyset_page};
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, load_channel_in_space, load_space,
    parse_entity_id, parse_space_id,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccessRuleRequest {
    pub rule_type: String,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessRuleResponse {
    pub rule_id: String,
    pub channel_id: String,
    pub rule_type: String,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
    pub created_at: String,
}

impl From<ChannelAccessRuleRecord> for AccessRuleResponse {
    fn from(record: ChannelAccessRuleRecord) -> Self {
        Self {
            rule_id: record.rule_id.to_string(),
            channel_id: record.channel_id.to_string(),
            rule_type: record.rule_type,
            principal_kind: record.principal_kind,
            principal_id: record.principal_id,
            permission: record.permission,
            created_at: record.created_at,
        }
    }
}

fn normalize_rule_type(rule_type: &str) -> Result<String, ApiProblem> {
    match rule_type {
        "allow" | "deny" => Ok(rule_type.to_owned()),
        other => {
            tracing::warn!(rule_type = other, "invalid channel access rule_type");
            Err(ApiProblem::bad_request("invalid channel access rule_type"))
        }
    }
}

fn normalize_permission(permission: &str) -> Result<String, ApiProblem> {
    match permission {
        // "send" is intentionally rejected: space-service owns no channel
        // message send path, so a send rule could never be enforced. Channel
        // message access follows conversation membership.
        "view" | "manage" => Ok(permission.to_owned()),
        "send" => {
            tracing::warn!(
                permission = permission,
                "channel access send permission is not enforced"
            );
            Err(ApiProblem::bad_request(
                "channel access send permission is not supported; message access follows conversation membership",
            ))
        }
        other => {
            tracing::warn!(permission = other, "invalid channel access permission");
            Err(ApiProblem::bad_request("invalid channel access permission"))
        }
    }
}

pub async fn create_access_rule(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
    Json(request): Json<CreateAccessRuleRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<AccessRuleResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        load_channel_in_space(&state, &auth, space_id, channel_id)?;

        let rule_type = normalize_rule_type(request.rule_type.as_str())?;
        let permission = normalize_permission(request.permission.as_str())?;
        let now = chrono::Utc::now().to_rfc3339();
        let record = ChannelAccessRuleRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            rule_id: next_entity_id(&state.id_generator)?,
            channel_id,
            rule_type,
            principal_kind: request.principal_kind,
            principal_id: request.principal_id,
            permission,
            created_at: now,
        };

        state
            .channel_access_rule_store
            .insert(&record)
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to insert channel access rule");
                ApiProblem::internal_server_error("failed to create channel access rule")
            })?;
        Ok(resource_item(AccessRuleResponse::from(record)))
    })();
    finish_api_response(&ctx, result.and_then(|data| created_json(&ctx, data)))
}

pub async fn list_access_rules(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<AccessRuleResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        load_channel_in_space(&state, &auth, space_id, channel_id)?;
        let paging = resolve_keyset_page(&query)?;
        let cursor_rule_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());

        let records = state
            .channel_access_rule_store
            .list_by_channel(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                channel_id,
                paging.cursor_sort_value.as_deref(),
                cursor_rule_id,
                paging.fetch_limit(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to list channel access rules");
                ApiProblem::internal_server_error("failed to list channel access rules")
            })?;

        let items = records.into_iter().map(AccessRuleResponse::from).collect();
        Ok(keyset_list_page(
            items,
            paging.page_size,
            |item: &AccessRuleResponse| (item.created_at.clone(), item.rule_id.clone()),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub async fn delete_access_rule(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id, rule_id)): Path<(String, String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let rule_id = parse_entity_id(rule_id.as_str(), "rule_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        load_channel_in_space(&state, &auth, space_id, channel_id)?;

        state
            .channel_access_rule_store
            .delete(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                rule_id,
            )
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to delete channel access rule");
                ApiProblem::internal_server_error("failed to delete channel access rule")
            })?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
