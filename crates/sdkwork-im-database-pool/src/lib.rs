//! Sdkwork IM database pool bootstrap through `sdkwork-database`.
//!
//! Every IM process (standalone and cloud) MUST call
//! [`bootstrap_im_process_database_pools_from_env`] once at startup, so all modules
//! reuse one sqlx lifecycle host and one shared r2d2 pool.

mod engine_policy;
mod shared_postgres;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{DatabasePool, PoolError, create_pool_from_config};

const IM_DATABASE_SERVICE_NAME: &str = "IM";

pub use engine_policy::{ensure_im_core_postgres_authority, im_core_requires_postgres_authority};
pub use sdkwork_im_database_host::{
    ImDatabaseHost, bootstrap_im_database, bootstrap_im_database_from_env,
};
pub use shared_postgres::{
    ImProcessDatabasePools, ImSharedPostgresConnectionManager, ImSharedPostgresR2d2Pool,
    ImSharedPostgresTlsConnector, bootstrap_im_process_database_pools_from_env,
    clone_shared_im_postgres_r2d2_pool, ensure_im_process_postgres_r2d2_pool,
    im_process_database_pools, is_im_process_database_pools_installed,
    shared_im_postgres_r2d2_pool,
};

/// Create the canonical IM sqlx pool from `SDKWORK_IM_DATABASE_*` environment variables.
pub async fn create_im_database_pool_from_env() -> Result<DatabasePool, PoolError> {
    if let Some(pools) = im_process_database_pools() {
        return Ok(pools.host().pool().clone());
    }
    let config = DatabaseConfig::from_env(IM_DATABASE_SERVICE_NAME)?;
    ensure_im_core_postgres_authority(&config).map_err(PoolError::DatabaseConfig)?;
    create_pool_from_config(config).await
}

/// Create the IM pool and apply the application-root `database/` lifecycle when enabled.
///
/// Prefer [`bootstrap_im_process_database_pools_from_env`] for PostgreSQL deployments.
pub async fn create_and_bootstrap_im_database_pool_from_env() -> Result<ImDatabaseHost, String> {
    if im_process_database_pools().is_some() {
        return Err(
            "IM database lifecycle already bootstrapped via bootstrap_im_process_database_pools_from_env"
                .to_owned(),
        );
    }
    let pool = create_im_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_im_database(pool).await
}
