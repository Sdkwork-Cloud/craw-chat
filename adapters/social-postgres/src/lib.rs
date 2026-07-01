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

/// TLS connector type for r2d2-backed PostgreSQL pools.
///
/// P0-12 (SECURITY_SPEC): uses `postgres-native-tls` so the `sslmode` URL
/// parameter is honored. With `sslmode=disable` the connector is never
/// invoked (plaintext TCP); with `sslmode=require` or `verify-full` a real
/// TLS handshake is performed. This allows dev/test to keep using plaintext
/// while production enforces TLS via the DSN.
pub type SocialPostgresTlsConnector = postgres_native_tls::MakeTlsConnector;
/// Connection manager type alias shared by all social stores.
pub type SocialPostgresConnectionManager = PostgresConnectionManager<SocialPostgresTlsConnector>;

/// Shared connection pool wrapper for all social stores.
#[derive(Clone)]
pub struct SocialPostgresPool(Option<Pool<SocialPostgresConnectionManager>>);

impl SocialPostgresPool {
    pub fn new(pool: Pool<SocialPostgresConnectionManager>) -> Self {
        Self(Some(pool))
    }

    pub fn inner(&self) -> &Pool<SocialPostgresConnectionManager> {
        self.0
            .as_ref()
            .expect("social postgres pool should remain initialized")
    }
}

impl Drop for SocialPostgresPool {
    fn drop(&mut self) {
        if let Some(pool) = self.0.take() {
            drop_social_postgres_pool_off_runtime(pool);
        }
    }
}

fn drop_social_postgres_pool_off_runtime(pool: Pool<SocialPostgresConnectionManager>) {
    if tokio::runtime::Handle::try_current().is_err() {
        drop(pool);
        return;
    }
    std::thread::spawn(move || drop(pool));
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
    if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
        return Ok(SocialPostgresPool::new(pool));
    }
    if cfg!(test) {
        return build_social_pool_local(config);
    }
    Err(im_platform_contracts::ContractError::Unavailable(
        sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
            .err()
            .unwrap_or_else(|| "IM process database pools are not installed".to_owned()),
    ))
}

fn build_social_pool_local(
    config: &config::SocialPostgresConfig,
) -> Result<SocialPostgresPool, im_platform_contracts::ContractError> {
    let database_url = config.database_url();
    verify_production_sslmode(database_url);
    let pg_config = database_url.parse().map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "invalid postgres url: {error}"
        ))
    })?;
    let tls = make_tls_connector().map_err(|error| {
        im_platform_contracts::ContractError::Unavailable(format!(
            "postgres TLS connector build failed: {error}"
        ))
    })?;
    let manager = PostgresConnectionManager::new(pg_config, tls);
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

/// Build a `native-tls` connector for PostgreSQL.
///
/// Uses the system trust store for certificate verification. The actual TLS
/// negotiation is gated by the `sslmode` URL parameter: when `sslmode=disable`
/// the `postgres` crate never invokes this connector.
fn make_tls_connector() -> Result<postgres_native_tls::MakeTlsConnector, native_tls::Error> {
    let connector = native_tls::TlsConnector::builder().build()?;
    Ok(postgres_native_tls::MakeTlsConnector::new(connector))
}

/// P0-12 fail-closed: in production, the database URL MUST contain
/// `sslmode=require` or `sslmode=verify-full`. This prevents silent plaintext
/// connections to production databases (SECURITY_SPEC §4.3).
fn verify_production_sslmode(database_url: &str) {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let is_production = !matches!(environment.as_str(), "" | "dev" | "development" | "test" | "testing");
    if !is_production {
        return;
    }
    let lowered = database_url.to_ascii_lowercase();
    let requires_tls = lowered.contains("sslmode=require")
        || lowered.contains("sslmode=verify-ca")
        || lowered.contains("sslmode=verify-full")
        || lowered.contains("sslmode=verifyca")
        || lowered.contains("sslmode=verifyfull");
    if !requires_tls {
        panic!(
            "P0-12 production fail-closed: SDKWORK_IM_DATABASE_URL must contain sslmode=require or sslmode=verify-full in production (current environment={environment}). Refusing to start with a plaintext database connection."
        );
    }
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
    client: &mut r2d2::PooledConnection<SocialPostgresConnectionManager>,
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
    pool: &Pool<SocialPostgresConnectionManager>,
    operation: &str,
) -> Result<
    r2d2::PooledConnection<SocialPostgresConnectionManager>,
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
