use axum::Router;
use axum::routing::get;
use social_service::postgres::{
    PostgresAppState, block, contact, direct_chat, user_profile, user_search, user_settings,
};

pub fn build_supplemental_app(state: PostgresAppState) -> Router {
    Router::new()
        .route("/im/v3/api/social/users", get(user_search::search_users))
        .route("/im/v3/api/social/contacts", get(contact::list_contacts))
        .route("/im/v3/api/social/user_blocks", get(block::list_blocks))
        .route(
            "/im/v3/api/social/direct_chats",
            get(direct_chat::list_direct_chats),
        )
        .route(
            "/im/v3/api/social/direct_chats/{direct_chat_id}",
            get(direct_chat::get_direct_chat),
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
