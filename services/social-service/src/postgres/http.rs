//! Postgres-backed supplemental social routes merged into comms-social-service.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_platform_contracts::IdGenerator;

/// Shared state for Postgres supplemental social handlers.
#[derive(Clone)]
pub struct PostgresAppState {
    pub postgres_pool: SocialPostgresPool,
    pub friend_request_store:
        Arc<dyn im_adapters_social_postgres::friend_request_store::FriendRequestStore>,
    pub friendship_store: Arc<dyn im_adapters_social_postgres::friendship_store::FriendshipStore>,
    pub user_block_store: Arc<dyn im_adapters_social_postgres::user_block_store::UserBlockStore>,
    pub user_profile_store:
        Arc<dyn im_adapters_social_postgres::user_profile_store::UserProfileStore>,
    pub user_settings_store:
        Arc<dyn im_adapters_social_postgres::user_settings_store::UserSettingsStore>,
    pub direct_chat_store: Arc<dyn im_adapters_social_postgres::direct_chat_store::DirectChatStore>,
    pub presence_cache: Option<im_adapters_redis_cache::presence_cache::RedisPresenceCache>,
    pub session_cache: Option<im_adapters_redis_cache::session_cache::RedisSessionCache>,
    pub id_generator: Arc<dyn IdGenerator>,
}
