//! Channel API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::organization_store::ChannelRecord;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, created_json, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::channel_conversation_binder::CreateSpaceChannelConversationInput;
use crate::http::AppState;
use crate::id::next_entity_id;
use crate::list_query::{ListQuery, resolve_keyset_page};
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, enforce_channel_permission,
    load_channel_in_space, load_space, parse_entity_id, parse_space_id,
};

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub channel_name: String,
    pub channel_type: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
    pub channel_id: String,
    pub space_id: String,
    pub channel_name: String,
    pub channel_type: String,
    pub description: Option<String>,
    pub conversation_id: Option<String>,
    pub position: i32,
    pub topic: Option<String>,
    pub created_at: String,
}

impl From<ChannelRecord> for ChannelResponse {
    fn from(record: ChannelRecord) -> Self {
        Self {
            channel_id: record.channel_id.to_string(),
            space_id: record.space_id.to_string(),
            channel_name: record.channel_name,
            channel_type: record.channel_type,
            description: record.description,
            conversation_id: record.conversation_id,
            position: record.position,
            topic: record.topic,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub channel_name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<String>,
}

pub async fn create_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateChannelRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<ChannelResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;

        let channel_id = next_entity_id(&state.id_generator)?;
        let conversation_id = next_entity_id(&state.id_generator)?.to_string();
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(binder) = state.channel_conversation_binder.as_ref() {
            binder
                .create_channel_conversation(CreateSpaceChannelConversationInput {
                    tenant_id: auth.tenant_id.clone(),
                    organization_id: auth.organization_id.clone(),
                    conversation_id: conversation_id.clone(),
                    creator_user_id: auth.actor_id.clone(),
                })
                .map_err(|error| {
                    tracing::error!(error = %error, "failed to bind channel conversation");
                    ApiProblem::internal_server_error("failed to bind channel conversation")
                })?;
        }

        let record = ChannelRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            channel_id,
            space_id,
            channel_name: request.channel_name,
            channel_type: request.channel_type.unwrap_or_else(|| "text".to_string()),
            description: request.description,
            conversation_id: Some(conversation_id),
            position: request.position.unwrap_or(0),
            is_nsfw: false,
            is_pinned: false,
            topic: request.topic,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now,
        };

        state.channel_store.insert(&record).map_err(|error| {
            tracing::error!(error = ?error, space_id, "failed to insert channel");
            ApiProblem::internal_server_error("failed to insert channel")
        })?;
        Ok(resource_item(ChannelResponse::from(record)))
    })();
    finish_api_response(&ctx, result.and_then(|data| created_json(&ctx, data)))
}

pub async fn list_channels(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<ChannelResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        let paging = resolve_keyset_page(&query)?;
        let cursor_channel_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());

        let records = state
            .channel_store
            .list_by_space(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                space_id,
                paging.cursor_sort_value.as_deref(),
                cursor_channel_id,
                paging.fetch_limit(),
            )
            .map_err(|error| {
                tracing::error!(error = ?error, space_id, "failed to list channels");
                ApiProblem::internal_server_error("failed to list channels")
            })?;

        let items = records.into_iter().map(ChannelResponse::from).collect();
        Ok(keyset_list_page(
            items,
            paging.page_size,
            |item: &ChannelResponse| (item.created_at.clone(), item.channel_id.clone()),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<ChannelResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &space)?;
        let record = load_channel_in_space(&state, &auth, space_id, channel_id)?;
        enforce_channel_permission(&state, &auth, &space, channel_id, "view")?;
        Ok(resource_item(ChannelResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
    Json(request): Json<UpdateChannelRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<ChannelResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        let mut record = load_channel_in_space(&state, &auth, space_id, channel_id)?;
        enforce_channel_permission(&state, &auth, &space, channel_id, "manage")?;
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(name) = request.channel_name {
            record.channel_name = name;
        }
        if let Some(desc) = request.description {
            record.description = Some(desc);
        }
        if let Some(pos) = request.position {
            record.position = pos;
        }
        if let Some(topic) = request.topic {
            record.topic = Some(topic);
        }
        record.updated_at = now;

        state.channel_store.update(&record).map_err(|error| {
            tracing::error!(error = ?error, channel_id, "failed to update channel");
            ApiProblem::internal_server_error("failed to update channel")
        })?;
        Ok(resource_item(ChannelResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn delete_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let channel_id = parse_entity_id(channel_id.as_str(), "channel_id")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        let _record = load_channel_in_space(&state, &auth, space_id, channel_id)?;
        enforce_channel_permission(&state, &auth, &space, channel_id, "manage")?;

        state
            .channel_store
            .delete(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                channel_id,
            )
            .map_err(|error| {
                tracing::error!(error = ?error, channel_id, "failed to delete channel");
                ApiProblem::internal_server_error("failed to delete channel")
            })?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
