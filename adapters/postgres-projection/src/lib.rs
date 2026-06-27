//! PostgreSQL-backed durable projection stores for `projection-service`.

mod metadata_store;
mod timeline_store;

use im_platform_contracts::ContractError;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
pub use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::postgres::NoTls as PostgresNoTls;
use sdkwork_database_config::DatabaseConfig;
use tokio::runtime::Handle;

pub use metadata_store::PostgresMetadataStore;
pub use timeline_store::PostgresTimelineProjectionStore;

const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

pub type PostgresProjectionConnectionManager = PostgresConnectionManager<PostgresNoTls>;
pub type PostgresProjectionPool = Pool<PostgresProjectionConnectionManager>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresProjectionConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl PostgresProjectionConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            pool_max_size: DEFAULT_POOL_MAX_SIZE,
            pool_min_idle: Some(DEFAULT_POOL_MIN_IDLE),
        }
    }

    pub fn from_database_config(config: &DatabaseConfig) -> Self {
        Self {
            database_url: config.url.clone(),
            pool_max_size: config.max_connections,
            pool_min_idle: Some(config.min_connections),
        }
    }

    pub fn connect_pool(&self) -> Result<PostgresProjectionPool, ContractError> {
        if Handle::try_current().is_ok() {
            return self.connect_pool_bridged();
        }
        build_projection_pool(self)
    }

    /// Creates a pool on a dedicated OS thread when called from a Tokio runtime.
    pub fn connect_pool_bridged(&self) -> Result<PostgresProjectionPool, ContractError> {
        let config = self.clone();
        run_postgres_io(move || build_projection_pool(&config))
    }

    pub fn connect_stores(self) -> Result<PostgresProjectionStores, ContractError> {
        let pool = self.connect_pool()?;
        Ok(PostgresProjectionStores::from_pool(pool))
    }
}

fn build_projection_pool(
    config: &PostgresProjectionConfig,
) -> Result<PostgresProjectionPool, ContractError> {
    let pg_config = config
        .database_url
        .parse()
        .map_err(|error| postgres_config_error(config.database_url.as_str(), error))?;
    let manager = PostgresConnectionManager::new(pg_config, NoTls);
    Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(config.pool_min_idle)
        .build(manager)
        .map_err(|error| postgres_unavailable("create projection pool", error))
}

#[derive(Clone)]
pub struct PostgresProjectionStores {
    pub metadata: PostgresMetadataStore,
    pub timeline: PostgresTimelineProjectionStore,
    pool: PostgresProjectionPool,
}

impl PostgresProjectionStores {
    pub fn from_pool(pool: PostgresProjectionPool) -> Self {
        Self {
            metadata: PostgresMetadataStore::from_pool(pool.clone()),
            timeline: PostgresTimelineProjectionStore::from_pool(pool.clone()),
            pool,
        }
    }

    pub fn pool(&self) -> &PostgresProjectionPool {
        &self.pool
    }
}

pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    if Handle::try_current().is_err() {
        return operation();
    }

    std::thread::scope(|scope| {
        scope
            .spawn(operation)
            .join()
            .map_err(|_| postgres_io_thread_panic())?
    })
}

pub(crate) fn postgres_pool_client(
    pool: &PostgresProjectionPool,
    action: &'static str,
) -> Result<r2d2::PooledConnection<PostgresProjectionConnectionManager>, ContractError> {
    pool.get()
        .map_err(|error| postgres_unavailable(action, error))
}

pub(crate) fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub(crate) fn postgres_unavailable(
    action: &'static str,
    error: impl std::fmt::Display,
) -> ContractError {
    ContractError::Unavailable(format!("postgres projection {action} failed: {error}"))
}

fn postgres_config_error(
    database_url: &str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    let redacted = redact_postgres_url(database_url);
    ContractError::Unavailable(format!(
        "postgres projection database url is invalid ({redacted}): {error}"
    ))
}

fn postgres_io_thread_panic() -> ContractError {
    ContractError::Unavailable("postgres projection blocking IO worker panicked".into())
}

fn redact_postgres_url(database_url: &str) -> String {
    let Some(scheme_end) = database_url.find("://") else {
        return "<redacted>".into();
    };
    let after_scheme = scheme_end + 3;
    let Some(at_offset) = database_url[after_scheme..].find('@') else {
        return database_url.into();
    };
    let scheme = &database_url[..after_scheme];
    let host = &database_url[after_scheme + at_offset..];
    format!("{scheme}<redacted>{host}")
}

pub(crate) fn default_projection_organization_id() -> &'static str {
    "default"
}
