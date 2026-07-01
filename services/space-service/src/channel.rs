//! Channel API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;

use im_adapters_social_postgres::organization_store::ChannelRecord;

use crate::http::AppState;
use crate::id::next_entity_id;

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

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateChannelRequest>,
) -> Response {
    let result: ApiResult<ChannelResponse> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;

        // IDOR fix (SECURITY_SPEC §4.2): only the space owner may create
        // channels under it. Without this check, any authenticated tenant
        // member could inject channels into any space by ID.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(space)) => {
                if space.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %space.owner_user_id,
                        space_id = sid,
                        "space ownership check failed for create_channel"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
            }
            Ok(None) => return Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for create_channel");
                return Err(ApiProblem::internal_server_error("failed to get space"));
            }
        }

        let channel_id = next_entity_id(&state.id_generator)?;
        let now = chrono::Utc::now().to_rfc3339();

        let record = ChannelRecord {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            channel_id,
            space_id: sid,
            channel_name: request.channel_name,
            channel_type: request.channel_type.unwrap_or_else(|| "text".to_string()),
            description: request.description,
            conversation_id: None,
            position: request.position.unwrap_or(0),
            is_nsfw: false,
            is_pinned: false,
            topic: request.topic,
            settings_json: request.settings_json.unwrap_or_else(|| "{}".to_string()),
            created_at: now.clone(),
            updated_at: now,
        };

        match state.channel_store.insert(&record) {
            Ok(()) => Ok(ChannelResponse::from(record)),
            Err(error) => {
                tracing::error!(error = ?error, "failed to insert channel under space {sid}");
                Err(ApiProblem::internal_server_error("failed to insert channel"))
            }
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_channels(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<ChannelResponse>> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;
        let limit = query.limit.unwrap_or(20);

        // IDOR fix (SECURITY_SPEC §4.2): only the space owner may enumerate
        // channels under it. Without this check, any authenticated tenant
        // member could discover channel structure of any space by ID.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(space)) => {
                if space.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %space.owner_user_id,
                        space_id = sid,
                        "space ownership check failed for list_channels"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
            }
            Ok(None) => return Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for list_channels");
                return Err(ApiProblem::internal_server_error("failed to get space"));
            }
        }

        match state.channel_store.list_by_space(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
            limit,
        ) {
            Ok(records) => {
                Ok(records.into_iter().map(ChannelResponse::from).collect())
            }
            Err(error) => {
                tracing::error!(error = ?error, "failed to list channels under space {sid}");
                Err(ApiProblem::internal_server_error("failed to list channels"))
            }
        }
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<ChannelResponse> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;
        let cid: i64 = channel_id.parse().map_err(|_| {
            tracing::warn!("invalid channel_id path parameter: {channel_id}");
            ApiProblem::bad_request("invalid channel_id path parameter")
        })?;

        // IDOR fix (SECURITY_SPEC §4.2): only the space owner may read
        // channel metadata. ChannelRecord has no owner_user_id, so we
        // authorize via the parent SpaceRecord.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(space)) => {
                if space.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %space.owner_user_id,
                        space_id = sid,
                        channel_id = cid,
                        "space ownership check failed for get_channel"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
            }
            Ok(None) => return Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for get_channel");
                return Err(ApiProblem::internal_server_error("failed to get space"));
            }
        }

        match state.channel_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            cid,
        ) {
            Ok(Some(record)) => {
                // Defense in depth: ensure the channel actually belongs to
                // the path space_id, preventing URL tampering.
                if record.space_id != sid {
                    tracing::warn!(
                        path_space_id = sid,
                        record_space_id = record.space_id,
                        channel_id = cid,
                        "channel space_id mismatch in get_channel"
                    );
                    return Err(ApiProblem::not_found("channel not found"));
                }
                Ok(ChannelResponse::from(record))
            }
            Ok(None) => Err(ApiProblem::not_found("channel not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get channel {cid}");
                Err(ApiProblem::internal_server_error("failed to get channel"))
            }
        }
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
    let result: ApiResult<()> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;
        let cid: i64 = channel_id.parse().map_err(|_| {
            tracing::warn!("invalid channel_id path parameter: {channel_id}");
            ApiProblem::bad_request("invalid channel_id path parameter")
        })?;
        let now = chrono::Utc::now().to_rfc3339();

        // IDOR fix (SECURITY_SPEC §4.2): only the space owner may mutate
        // channel settings. ChannelRecord has no owner_user_id, so we
        // authorize via the parent SpaceRecord.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(space)) => {
                if space.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %space.owner_user_id,
                        space_id = sid,
                        channel_id = cid,
                        "space ownership check failed for update_channel"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
            }
            Ok(None) => return Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for update_channel");
                return Err(ApiProblem::internal_server_error("failed to get space"));
            }
        }

        match state.channel_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            cid,
        ) {
            Ok(Some(mut record)) => {
                // Defense in depth: ensure the channel actually belongs to
                // the path space_id, preventing URL tampering.
                if record.space_id != sid {
                    tracing::warn!(
                        path_space_id = sid,
                        record_space_id = record.space_id,
                        channel_id = cid,
                        "channel space_id mismatch in update_channel"
                    );
                    return Err(ApiProblem::not_found("channel not found"));
                }
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

                match state.channel_store.update(&record) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to update channel {cid}");
                        Err(ApiProblem::internal_server_error("failed to update channel"))
                    }
                }
            }
            Ok(None) => Err(ApiProblem::not_found("channel not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get channel {cid} for update");
                Err(ApiProblem::internal_server_error("failed to get channel"))
            }
        }
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn delete_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, channel_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let sid: i64 = space_id.parse().map_err(|_| {
            tracing::warn!("invalid space_id path parameter: {space_id}");
            ApiProblem::bad_request("invalid space_id path parameter")
        })?;
        let cid: i64 = channel_id.parse().map_err(|_| {
            tracing::warn!("invalid channel_id path parameter: {channel_id}");
            ApiProblem::bad_request("invalid channel_id path parameter")
        })?;

        // IDOR fix (SECURITY_SPEC §4.2): only the space owner may delete
        // channels under it. ChannelRecord has no owner_user_id, so we
        // authorize via the parent SpaceRecord. We also fetch the channel
        // first to verify it actually belongs to the path space_id.
        match state.space_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            sid,
        ) {
            Ok(Some(space)) => {
                if space.owner_user_id != auth.actor_id {
                    tracing::warn!(
                        user_id = %auth.actor_id,
                        owner_user_id = %space.owner_user_id,
                        space_id = sid,
                        channel_id = cid,
                        "space ownership check failed for delete_channel"
                    );
                    return Err(ApiProblem::forbidden("space ownership check failed"));
                }
            }
            Ok(None) => return Err(ApiProblem::not_found("space not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get space {sid} for delete_channel");
                return Err(ApiProblem::internal_server_error("failed to get space"));
            }
        }

        // Defense in depth: verify the channel actually belongs to the
        // path space_id before deleting, preventing URL tampering.
        match state.channel_store.get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            cid,
        ) {
            Ok(Some(record)) => {
                if record.space_id != sid {
                    tracing::warn!(
                        path_space_id = sid,
                        record_space_id = record.space_id,
                        channel_id = cid,
                        "channel space_id mismatch in delete_channel"
                    );
                    return Err(ApiProblem::not_found("channel not found"));
                }
                match state.channel_store.delete(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    cid,
                ) {
                    Ok(()) => Ok(()),
                    Err(error) => {
                        tracing::error!(error = ?error, "failed to delete channel {cid}");
                        Err(ApiProblem::internal_server_error("failed to delete channel"))
                    }
                }
            }
            Ok(None) => Err(ApiProblem::not_found("channel not found")),
            Err(error) => {
                tracing::error!(error = ?error, "failed to get channel {cid} for delete");
                Err(ApiProblem::internal_server_error("failed to get channel"))
            }
        }
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
