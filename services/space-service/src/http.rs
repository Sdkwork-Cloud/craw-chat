//! Space Service HTTP routes.

use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, post};
use im_platform_contracts::IdGenerator;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};

use crate::ban;
use crate::channel;
use crate::channel_access_rule;
use crate::group;
use crate::group_member;
use crate::invitation;
use crate::space;
use crate::space_member;

/// Shared state for space service handlers.
#[derive(Clone)]
pub struct AppState {
    pub space_store: Arc<dyn im_adapters_social_postgres::organization_store::SpaceStore>,
    pub group_store: Arc<dyn im_adapters_social_postgres::organization_store::GroupStore>,
    pub channel_store: Arc<dyn im_adapters_social_postgres::organization_store::ChannelStore>,
    pub id_generator: Arc<dyn IdGenerator>,
}

pub fn build_embedded_app(state: AppState) -> Router {
    build_space_api_routes(state)
}

fn build_business_router(state: AppState) -> Router {
    build_space_api_routes(state)
}

pub fn build_app(state: AppState) -> Router {
    mount_im_infra_routes(build_business_router(state), im_service_router_config())
}

fn build_space_api_routes(state: AppState) -> Router {
    Router::new()
        // Spaces
        .route(
            "/im/v3/api/spaces",
            post(space::create_space).get(space::list_spaces),
        )
        .route(
            "/im/v3/api/spaces/{space_id}",
            get(space::get_space)
                .patch(space::update_space)
                .delete(space::delete_space),
        )
        // Space members
        .route(
            "/im/v3/api/spaces/{space_id}/members",
            post(space_member::add_space_member).get(space_member::list_space_members),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/members/{user_id}",
            get(space_member::get_space_member)
                .patch(space_member::update_space_member)
                .delete(space_member::remove_space_member),
        )
        // Groups
        .route(
            "/im/v3/api/spaces/{space_id}/groups",
            post(group::create_group).get(group::list_groups),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/groups/{group_id}",
            get(group::get_group)
                .patch(group::update_group)
                .delete(group::delete_group),
        )
        // Group members
        .route(
            "/im/v3/api/spaces/{space_id}/groups/{group_id}/members",
            post(group_member::add_group_member).get(group_member::list_group_members),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/groups/{group_id}/members/{user_id}",
            get(group_member::get_group_member)
                .patch(group_member::update_group_member)
                .delete(group_member::remove_group_member),
        )
        // Channels
        .route(
            "/im/v3/api/spaces/{space_id}/channels",
            post(channel::create_channel).get(channel::list_channels),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/channels/{channel_id}",
            get(channel::get_channel)
                .patch(channel::update_channel)
                .delete(channel::delete_channel),
        )
        // Channel access rules
        .route(
            "/im/v3/api/spaces/{space_id}/channels/{channel_id}/access_rules",
            post(channel_access_rule::create_access_rule)
                .get(channel_access_rule::list_access_rules),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/channels/{channel_id}/access_rules/{rule_id}",
            delete(channel_access_rule::delete_access_rule),
        )
        // Invitations
        .route(
            "/im/v3/api/spaces/{space_id}/invites",
            post(invitation::create_invitation).get(invitation::list_invitations),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/invites/{invite_code}",
            get(invitation::get_invitation).delete(invitation::revoke_invitation),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/invites/{invite_code}/accept",
            post(invitation::accept_invitation),
        )
        // Bans
        .route(
            "/im/v3/api/spaces/{space_id}/bans",
            post(ban::ban_user).get(ban::list_bans),
        )
        .route(
            "/im/v3/api/spaces/{space_id}/bans/{user_id}",
            get(ban::get_ban).delete(ban::unban_user),
        )
        .with_state(state)
}

pub fn build_public_app(state: AppState) -> Router {
    build_app(state)
}
