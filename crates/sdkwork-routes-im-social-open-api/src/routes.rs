use axum::Router;
use axum::routing::{delete, get, post};
use social_service::postgres::{
    block, direct_chat, friendship, user_profile, user_search, user_settings, PostgresAppState,
};

pub fn build_supplemental_app(state: PostgresAppState) -> Router {
    Router::new()
        .route(
            "/im/v3/api/social/users",
            get(user_search::search_users),
        )
        .route(
            "/im/v3/api/social/friendships",
            get(friendship::list_friends),
        )
        .route("/im/v3/api/social/user_blocks", get(block::list_blocks))
        .route(
            "/im/v3/api/social/user_blocks/{block_id}",
            delete(block::unblock_user),
        )
        .route(
            "/im/v3/api/social/direct_chats",
            post(direct_chat::create_direct_chat).get(direct_chat::list_direct_chats),
        )
        .route(
            "/im/v3/api/social/direct_chats/{direct_chat_id}",
            get(direct_chat::get_direct_chat).patch(direct_chat::update_direct_chat),
        )
        .route(
            "/im/v3/api/social/users/{user_id}/profile",
            get(user_profile::get_user_profile).patch(user_profile::update_user_profile),
        )
        .route(
            "/im/v3/api/social/users/{user_id}/settings",
            get(user_settings::get_user_settings).patch(user_settings::update_user_settings),
        )
        .with_state(state)
}
