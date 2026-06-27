//! PostgreSQL adapter for social-service persistence.
//!
//! Provides database-backed storage for friend requests, friendships,
//! user blocks, direct chats, external connections, and shared channel policies.

pub mod config;
pub mod direct_chat_store;
pub mod external_store;
pub mod friend_request_store;
pub mod friendship_store;
pub mod organization_store;
pub mod shared_channel_store;
pub mod user_block_store;
pub mod user_profile_store;
pub mod user_settings_store;

pub use config::SocialPostgresConfig;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
pub use r2d2_postgres::postgres::NoTls;

/// Shared connection pool wrapper for all social stores.
#[derive(Clone)]
pub struct SocialPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl SocialPostgresPool {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { pool }
    }

    pub fn inner(&self) -> &Pool<PostgresConnectionManager<NoTls>> {
        &self.pool
    }
}

/// Run a blocking PostgreSQL operation off the async runtime.
pub(crate) fn run_postgres_io<F, T>(f: F) -> Result<T, im_platform_contracts::ContractError>
where
    F: FnOnce() -> Result<T, im_platform_contracts::ContractError> + Send,
    T: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(f)
            .join()
            .map_err(|_| postgres_io_thread_panic())?
    })
}

fn postgres_io_thread_panic() -> im_platform_contracts::ContractError {
    im_platform_contracts::ContractError::Unavailable(
        "postgres social blocking IO worker panicked".into(),
    )
}

pub(crate) fn build_social_pool(
    config: &config::SocialPostgresConfig,
) -> Result<SocialPostgresPool, im_platform_contracts::ContractError> {
    let pg_config = config.database_url().parse().map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "invalid postgres url: {error}"
        ))
    })?;
    let manager = PostgresConnectionManager::new(pg_config, NoTls);
    let pool = Pool::builder()
        .max_size(config.pool_max_size())
        .min_idle(config.pool_min_idle())
        .build(manager)
        .map_err(|error| {
            im_platform_contracts::ContractError::Unavailable(format!(
                "postgres pool build failed: {error}"
            ))
        })?;
    Ok(SocialPostgresPool::new(pool))
}

/// Map a postgres error to ContractError, redacting the connection URL.
pub(crate) fn postgres_unavailable(
    operation: &str,
    error: postgres::Error,
) -> im_platform_contracts::ContractError {
    im_platform_contracts::ContractError::Unavailable(format!(
        "postgres {operation} failed: {error}"
    ))
}

fn resolve_im_postgres_search_path_schema() -> Option<String> {
    let schema = sdkwork_database_config::claw_database::resolve_unified_postgres_schema("IM");
    (schema != "public").then_some(schema)
}

fn apply_postgres_search_path(
    client: &mut r2d2::PooledConnection<PostgresConnectionManager<NoTls>>,
    schema: &str,
) -> Result<(), im_platform_contracts::ContractError> {
    if !schema
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(im_platform_contracts::ContractError::Unavailable(format!(
            "invalid postgres search_path schema `{schema}`"
        )));
    }
    let sql = format!("SET search_path TO \"{schema}\", public");
    client.batch_execute(&sql).map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "postgres set search_path failed: {error}"
        ))
    })?;
    Ok(())
}

/// Get a client from the pool with unified IM schema search_path applied.
pub fn postgres_pool_client(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    operation: &str,
) -> Result<
    r2d2::PooledConnection<PostgresConnectionManager<NoTls>>,
    im_platform_contracts::ContractError,
> {
    let mut client = pool.get().map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "postgres pool get for {operation} failed: {error}"
        ))
    })?;
    if let Some(schema) = resolve_im_postgres_search_path_schema() {
        apply_postgres_search_path(&mut client, schema.as_str())?;
    }
    Ok(client)
}
