//! Postgres-backed supplemental social routes merged into comms-social-service.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_platform_contracts::IdGenerator;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

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

/// Run a synchronous Postgres-backed operation off the Tokio async worker
/// pool.
///
/// Postgres supplemental social handlers use `r2d2` sync connection pools
/// (`SocialPostgresPool`). Calling these methods directly on an async worker
/// thread blocks the Tokio runtime and can cause request-pending stalls under
/// load. Routing this work through `spawn_blocking` moves it to the dedicated
/// blocking thread pool, per `RUST_CODE_SPEC.md §6` ("Do not hold locks
/// across `.await`") and mirrors `crate::envelope::run_blocking_social_call`.
pub async fn run_blocking_postgres_call<F, T>(
    state: PostgresAppState,
    operation: F,
) -> Result<T, ApiProblem>
where
    F: FnOnce(PostgresAppState) -> Result<T, ApiProblem> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(state))
        .await
        .map_err(|error| {
            tracing::error!(?error, "postgres blocking join failed");
            ApiProblem::internal_server_error("postgres blocking join failed")
        })?
}
