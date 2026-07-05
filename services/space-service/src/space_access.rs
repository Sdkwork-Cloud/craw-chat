//! Shared space authorization helpers for space-service handlers.

use im_adapters_social_postgres::organization_store::{ChannelRecord, SpaceRecord};
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

use crate::http::AppState;

pub fn parse_space_id(space_id: &str) -> Result<i64, ApiProblem> {
    space_id.parse().map_err(|_| {
        tracing::warn!("invalid space_id path parameter: {space_id}");
        ApiProblem::bad_request("invalid space_id path parameter")
    })
}

pub fn parse_entity_id(entity_id: &str, field: &str) -> Result<i64, ApiProblem> {
    entity_id.parse().map_err(|_| {
        tracing::warn!("invalid {field} path parameter: {entity_id}");
        ApiProblem::bad_request(format!("invalid {field} path parameter"))
    })
}

pub fn load_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
) -> Result<SpaceRecord, ApiProblem> {
    state
        .space_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            space_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, space_id, "failed to load space");
            ApiProblem::internal_server_error("failed to load space")
        })?
        .ok_or_else(|| ApiProblem::not_found("space not found"))
}

pub fn actor_can_read_space(
    state: &AppState,
    auth: &AppContext,
    space: &SpaceRecord,
) -> Result<(), ApiProblem> {
    if space.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.space_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        space.space_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(ApiProblem::forbidden("space membership required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space membership",
            ))
        }
    }
}

pub fn actor_can_manage_space(
    state: &AppState,
    auth: &AppContext,
    space: &SpaceRecord,
) -> Result<(), ApiProblem> {
    if space.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.space_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        space.space_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(member)) if member.role == "admin" => Ok(()),
        Ok(Some(_)) => Err(ApiProblem::forbidden("space admin permission required")),
        Ok(None) => Err(ApiProblem::forbidden("space admin permission required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space admin membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space admin membership",
            ))
        }
    }
}

pub fn ensure_user_not_banned_in_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    user_id: &str,
) -> Result<(), ApiProblem> {
    match state.ban_store.get_active_by_user(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        "space",
        space_id,
        user_id,
    ) {
        Ok(Some(_)) => Err(ApiProblem::forbidden("user is banned from this space")),
        Ok(None) => Ok(()),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space ban status");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space ban status",
            ))
        }
    }
}

pub fn normalize_space_member_role(role: Option<&str>, allow_owner: bool) -> Result<String, ApiProblem> {
    match role.unwrap_or("member") {
        "owner" if allow_owner => Ok("owner".to_owned()),
        "owner" => Err(ApiProblem::bad_request("owner role cannot be assigned directly")),
        "admin" => Ok("admin".to_owned()),
        "member" => Ok("member".to_owned()),
        "guest" => Ok("guest".to_owned()),
        other => {
            tracing::warn!(role = other, "invalid space member role");
            Err(ApiProblem::bad_request("invalid space member role"))
        }
    }
}

pub fn load_channel_in_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    channel_id: i64,
) -> Result<ChannelRecord, ApiProblem> {
    let channel = state
        .channel_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            channel_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, channel_id, "failed to load channel");
            ApiProblem::internal_server_error("failed to load channel")
        })?
        .ok_or_else(|| ApiProblem::not_found("channel not found"))?;

    if channel.space_id != space_id {
        tracing::warn!(
            path_space_id = space_id,
            record_space_id = channel.space_id,
            channel_id,
            "channel does not belong to requested space"
        );
        return Err(ApiProblem::not_found("channel not found"));
    }
    Ok(channel)
}
