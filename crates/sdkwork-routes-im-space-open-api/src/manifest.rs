use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: open-api
pub const API_SURFACE: &str = "open-api";

pub const ROUTES: &[HttpRoute] = &[
    // Spaces
    HttpRoute::api_key_or_dual_token(HttpMethod::Post, paths::SPACES, "spaces", "spaces.create"),
    HttpRoute::api_key_or_dual_token(HttpMethod::Get, paths::SPACES, "spaces", "spaces.list"),
    HttpRoute::api_key_or_dual_token(HttpMethod::Get, paths::SPACE, "spaces", "spaces.retrieve"),
    HttpRoute::api_key_or_dual_token(HttpMethod::Patch, paths::SPACE, "spaces", "spaces.update"),
    HttpRoute::api_key_or_dual_token(HttpMethod::Delete, paths::SPACE, "spaces", "spaces.delete"),
    // Space members
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_MEMBERS,
        "spaces",
        "spaces.members.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_MEMBERS,
        "spaces",
        "spaces.members.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_MEMBER,
        "spaces",
        "spaces.members.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Patch,
        paths::SPACE_MEMBER,
        "spaces",
        "spaces.members.update",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_MEMBER,
        "spaces",
        "spaces.members.delete",
    ),
    // Groups
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_GROUPS,
        "spaces",
        "spaces.groups.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_GROUPS,
        "spaces",
        "spaces.groups.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_GROUP,
        "spaces",
        "spaces.groups.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Patch,
        paths::SPACE_GROUP,
        "spaces",
        "spaces.groups.update",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_GROUP,
        "spaces",
        "spaces.groups.delete",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_GROUP_TRANSFER_OWNER,
        "spaces",
        "spaces.groups.transferOwner",
    ),
    // Group members
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_GROUP_MEMBERS,
        "spaces",
        "spaces.groups.members.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_GROUP_MEMBERS,
        "spaces",
        "spaces.groups.members.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_GROUP_MEMBER,
        "spaces",
        "spaces.groups.members.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Patch,
        paths::SPACE_GROUP_MEMBER,
        "spaces",
        "spaces.groups.members.update",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_GROUP_MEMBER,
        "spaces",
        "spaces.groups.members.delete",
    ),
    // Channels
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_CHANNELS,
        "spaces",
        "spaces.channels.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_CHANNELS,
        "spaces",
        "spaces.channels.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_CHANNEL,
        "spaces",
        "spaces.channels.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Patch,
        paths::SPACE_CHANNEL,
        "spaces",
        "spaces.channels.update",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_CHANNEL,
        "spaces",
        "spaces.channels.delete",
    ),
    // Channel access rules
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_CHANNEL_ACCESS_RULES,
        "spaces",
        "spaces.channels.accessRules.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_CHANNEL_ACCESS_RULES,
        "spaces",
        "spaces.channels.accessRules.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_CHANNEL_ACCESS_RULE,
        "spaces",
        "spaces.channels.accessRules.delete",
    ),
    // Invitations
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_INVITES,
        "spaces",
        "spaces.invites.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_INVITES,
        "spaces",
        "spaces.invites.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_INVITE,
        "spaces",
        "spaces.invites.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_INVITE,
        "spaces",
        "spaces.invites.delete",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_INVITE_ACCEPT,
        "spaces",
        "spaces.invites.accept",
    ),
    // Bans
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::SPACE_BANS,
        "spaces",
        "spaces.bans.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_BANS,
        "spaces",
        "spaces.bans.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::SPACE_BAN,
        "spaces",
        "spaces.bans.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Delete,
        paths::SPACE_BAN,
        "spaces",
        "spaces.bans.delete",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
