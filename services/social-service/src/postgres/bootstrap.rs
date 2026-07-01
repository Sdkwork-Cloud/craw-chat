//! Bootstrap helpers for optional Postgres-backed supplemental routes.

use std::sync::Arc;

use im_adapters_social_postgres::direct_chat_store::PostgresDirectChatStore;
use im_adapters_social_postgres::friend_request_store::PostgresFriendRequestStore;
use im_adapters_social_postgres::friendship_store::PostgresFriendshipStore;
use im_adapters_social_postgres::user_block_store::PostgresUserBlockStore;
use im_adapters_social_postgres::user_profile_store::PostgresUserProfileStore;
use im_adapters_social_postgres::user_settings_store::PostgresUserSettingsStore;
use im_adapters_social_postgres::{SocialPostgresConfig, SocialPostgresPool};

use super::http::PostgresAppState;
use super::id::build_runtime_id_generator_for_social;

pub const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

pub async fn app_state_from_postgres_pool(pool: SocialPostgresPool) -> PostgresAppState {
    let pool_arc = Arc::new(pool.inner().clone());
    PostgresAppState {
        postgres_pool: pool,
        friend_request_store: Arc::new(PostgresFriendRequestStore::new(pool_arc.clone())),
        friendship_store: Arc::new(PostgresFriendshipStore::new(pool_arc.clone())),
        user_block_store: Arc::new(PostgresUserBlockStore::new(pool_arc.clone())),
        user_profile_store: Arc::new(PostgresUserProfileStore::new(pool_arc.clone())),
        user_settings_store: Arc::new(PostgresUserSettingsStore::new(pool_arc.clone())),
        direct_chat_store: Arc::new(PostgresDirectChatStore::new(pool_arc)),
        presence_cache: None,
        session_cache: None,
        id_generator: build_runtime_id_generator_for_social().await,
    }
}

pub async fn try_postgres_app_state_from_database_url_env() -> Option<PostgresAppState> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM").ok()?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return None;
    }
    let pool = SocialPostgresConfig::from_database_config(&config)
        .connect_pool()
        .ok()?;
    Some(app_state_from_postgres_pool(pool).await)
}
