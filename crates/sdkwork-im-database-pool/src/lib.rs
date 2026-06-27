//! Sdkwork IM database pool bootstrap through `sdkwork-database`.
//!
//! R2D2-backed adapters may continue to use `sdkwork-database-config` directly until
//! they migrate to async sqlx pools. New async services should use this crate.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};

const IM_DATABASE_SERVICE_NAME: &str = "IM";

pub use sdkwork_im_database_host::{bootstrap_im_database, bootstrap_im_database_from_env, ImDatabaseHost};

/// Create the canonical IM sqlx pool from `SDKWORK_IM_DATABASE_*` environment variables.
pub async fn create_im_database_pool_from_env() -> Result<DatabasePool, PoolError> {
    let config = DatabaseConfig::from_env(IM_DATABASE_SERVICE_NAME)?;
    create_pool_from_config(config).await
}

/// Create the IM pool and apply the application-root `database/` lifecycle when enabled.
pub async fn create_and_bootstrap_im_database_pool_from_env() -> Result<ImDatabaseHost, String> {
    let pool = create_im_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_im_database(pool).await
}
