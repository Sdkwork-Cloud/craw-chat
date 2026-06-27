use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};

use crate::friendship::{self, AppState};
use crate::{block, direct_chat, external, shared_channel, SocialRuntime};

pub fn build_control_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/control/social/friend_requests",
            get(friendship::list_friend_requests).post(friendship::submit_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}",
            get(friendship::friend_request_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/accept",
            post(friendship::accept_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/decline",
            post(friendship::decline_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/cancel",
            post(friendship::cancel_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friendships",
            post(friendship::activate_friendship),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}",
            get(friendship::friendship_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}/remove",
            post(friendship::remove_friendship),
        )
        .route(
            "/backend/v3/api/control/social/user_blocks",
            post(block::block_user),
        )
        .route(
            "/backend/v3/api/control/social/user_blocks/{block_id}",
            get(block::user_block_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/bindings",
            post(direct_chat::bind_direct_chat),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/{direct_chat_id}",
            get(direct_chat::direct_chat_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_connections",
            post(external::establish_external_connection),
        )
        .route(
            "/backend/v3/api/control/social/external_connections/{connection_id}",
            get(external::external_connection_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links",
            post(external::bind_external_member_link),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links/{link_id}",
            get(external::external_member_link_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies",
            post(shared_channel::apply_shared_channel_policy),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies/{policy_id}",
            get(shared_channel::shared_channel_policy_snapshot),
        )
        .merge(crate::runtime_control::build_runtime_control_routes())
        .with_state(state)
}

pub fn build_control_public_router(social_runtime: Arc<SocialRuntime>) -> Router {
    mount_im_infra_routes(
        build_control_domain_api_router(AppState { social_runtime }),
        im_service_router_config(),
    )
}
