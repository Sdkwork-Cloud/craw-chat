//! Configuration for the social PostgreSQL adapter.

use tokio::runtime::Handle;

use crate::{build_social_pool, run_postgres_io, SocialPostgresPool};

const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

/// Configuration for connecting to PostgreSQL for social persistence.
#[derive(Clone, Debug)]
pub struct SocialPostgresConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl SocialPostgresConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            pool_max_size: DEFAULT_POOL_MAX_SIZE,
            pool_min_idle: Some(DEFAULT_POOL_MIN_IDLE),
        }
    }

    pub fn with_pool_max_size(mut self, pool_max_size: u32) -> Self {
        self.pool_max_size = pool_max_size.max(1);
        if let Some(pool_min_idle) = self.pool_min_idle {
            self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        }
        self
    }

    pub fn with_pool_min_idle(mut self, pool_min_idle: u32) -> Self {
        self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        self
    }

    /// Create config from sdkwork-database config (§33 unified pool config).
    pub fn from_database_config(config: &sdkwork_database_config::DatabaseConfig) -> Self {
        Self {
            database_url: config.url.clone(),
            pool_max_size: config.max_connections,
            pool_min_idle: Some(config.min_connections),
        }
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn pool_max_size(&self) -> u32 {
        self.pool_max_size
    }

    pub fn pool_min_idle(&self) -> Option<u32> {
        self.pool_min_idle
    }

    /// Create a connection pool from this configuration.
    pub fn connect_pool(&self) -> Result<SocialPostgresPool, im_platform_contracts::ContractError> {
        if Handle::try_current().is_ok() {
            return self.connect_pool_bridged();
        }
        build_social_pool(self)
    }

    /// Creates a pool on a dedicated OS thread when called from a Tokio runtime.
    pub fn connect_pool_bridged(&self) -> Result<SocialPostgresPool, im_platform_contracts::ContractError> {
        let config = self.clone();
        run_postgres_io(move || build_social_pool(&config))
    }
}
