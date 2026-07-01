//! Process-wide shared PostgreSQL pools for every IM deployment profile.
//!
//! Standalone, cloud, unified-process, and split-service binaries MUST install one
//! sqlx lifecycle host and one r2d2 PostgreSQL pool per process via
//! [`bootstrap_im_process_database_pools_from_env`]. Adapters MUST reuse that r2d2
//! pool and MUST NOT open independent pools against the same DSN.

use std::sync::{Arc, OnceLock};

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_database_host::ImDatabaseHost;
use tracing::info;

use crate::bootstrap_im_database_from_env;

/// TLS connector type for the shared IM PostgreSQL r2d2 pool.
pub type ImSharedPostgresTlsConnector = postgres_native_tls::MakeTlsConnector;
/// Connection manager for the shared IM PostgreSQL r2d2 pool.
pub type ImSharedPostgresConnectionManager =
    PostgresConnectionManager<ImSharedPostgresTlsConnector>;
/// Canonical synchronous PostgreSQL pool shared by IM modules in one process.
pub type ImSharedPostgresR2d2Pool = Pool<ImSharedPostgresConnectionManager>;

/// Deprecated alias retained for callers migrating from the unified-process name.
pub type ImUnifiedProcessPools = ImProcessDatabasePools;

static IM_PROCESS_DATABASE_POOLS: OnceLock<ImProcessDatabasePools> = OnceLock::new();

/// Installed once per IM service or gateway process.
pub struct ImProcessDatabasePools {
    host: ImDatabaseHost,
    postgres_r2d2: Arc<ImSharedPostgresR2d2Pool>,
}

impl ImProcessDatabasePools {
    pub fn host(&self) -> &ImDatabaseHost {
        &self.host
    }

    pub fn postgres_r2d2(&self) -> Arc<ImSharedPostgresR2d2Pool> {
        self.postgres_r2d2.clone()
    }
}

/// Returns the installed process pool bundle when present.
pub fn im_process_database_pools() -> Option<&'static ImProcessDatabasePools> {
    IM_PROCESS_DATABASE_POOLS.get()
}

/// Deprecated alias retained for callers migrating from the unified-process name.
pub fn unified_process_pools() -> Option<&'static ImProcessDatabasePools> {
    im_process_database_pools()
}

/// Whether shared IM process pools were installed in this process.
pub fn is_im_process_database_pools_installed() -> bool {
    IM_PROCESS_DATABASE_POOLS.get().is_some()
}

/// Deprecated alias retained for callers migrating from the unified-process name.
pub fn is_im_unified_process_pools_installed() -> bool {
    is_im_process_database_pools_installed()
}

/// Shared r2d2 pool handle when process pools are installed.
pub fn shared_im_postgres_r2d2_pool() -> Option<Arc<ImSharedPostgresR2d2Pool>> {
    im_process_database_pools().map(ImProcessDatabasePools::postgres_r2d2)
}

/// Returns the shared r2d2 pool or a bootstrap error. Adapters MUST use this instead
/// of constructing independent pools.
pub fn ensure_im_process_postgres_r2d2_pool() -> Result<ImSharedPostgresR2d2Pool, String> {
    clone_shared_im_postgres_r2d2_pool().ok_or_else(|| {
        "IM process database pools are not installed; call \
         bootstrap_im_process_database_pools_from_env() at process entry before \
         opening PostgreSQL adapters"
            .to_owned()
    })
}

/// Cheap clone of the shared r2d2 pool for adapter `from_pool` wiring.
pub fn clone_shared_im_postgres_r2d2_pool() -> Option<ImSharedPostgresR2d2Pool> {
    shared_im_postgres_r2d2_pool().map(|pool| (*pool).clone())
}

fn im_database_url_configured() -> bool {
    std::env::var("SDKWORK_IM_DATABASE_URL")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || DatabaseConfig::from_env("IM").is_ok_and(|config| {
            config.engine == DatabaseEngine::Postgres && !config.url.trim().is_empty()
        })
}

/// Bootstraps IM pools when PostgreSQL is configured; otherwise no-op.
pub async fn try_bootstrap_im_process_database_pools_from_env(
) -> Result<Option<&'static ImProcessDatabasePools>, String> {
    if !im_database_url_configured() {
        return Ok(None);
    }

    let config = DatabaseConfig::from_env("IM")
        .map_err(|error| format!("read IM database config failed: {error}"))?;
    if config.engine != DatabaseEngine::Postgres {
        return Ok(None);
    }

    bootstrap_im_process_database_pools_from_env()
        .await
        .map(Some)
}

/// Bootstrap IM lifecycle (sqlx) plus one shared r2d2 pool for all modules in this process.
pub async fn bootstrap_im_process_database_pools_from_env(
) -> Result<&'static ImProcessDatabasePools, String> {
    if let Some(pools) = im_process_database_pools() {
        return Ok(pools);
    }

    let host = bootstrap_im_database_from_env().await?;
    let config = DatabaseConfig::from_env("IM")
        .map_err(|error| format!("read IM database config failed: {error}"))?;
    if config.engine != DatabaseEngine::Postgres {
        return Err(
            "IM shared PostgreSQL pool requires postgres engine when SDKWORK_IM_DATABASE_URL is set"
                .to_owned(),
        );
    }

    let postgres_r2d2 = Arc::new(build_im_postgres_r2d2_pool(&config)?);
    let pools = ImProcessDatabasePools {
        host,
        postgres_r2d2,
    };
    IM_PROCESS_DATABASE_POOLS
        .set(pools)
        .map_err(|_| "IM process database pools already installed in this process".to_owned())?;

    info!(
        target: "sdkwork.im",
        event = "im.database.process_pools_installed",
        max_connections = config.max_connections,
        min_connections = config.min_connections,
        database_url = %redact_postgres_url(config.url.as_str()),
        "installed shared IM sqlx lifecycle host and single postgres r2d2 pool"
    );

    Ok(im_process_database_pools().expect("process database pools installed"))
}

/// Deprecated alias retained for callers migrating from the unified-process name.
pub async fn bootstrap_im_unified_process_pools_from_env(
) -> Result<&'static ImProcessDatabasePools, String> {
    bootstrap_im_process_database_pools_from_env().await
}

pub(crate) fn build_im_postgres_r2d2_pool(
    config: &DatabaseConfig,
) -> Result<ImSharedPostgresR2d2Pool, String> {
    verify_production_sslmode(config.url.as_str());
    let pg_config = config.url.parse().map_err(|error| {
        format!(
            "invalid postgres url ({}): {error}",
            redact_postgres_url(config.url.as_str())
        )
    })?;
    let tls = make_tls_connector()
        .map_err(|error| format!("postgres TLS connector build failed: {error}"))?;
    let manager = PostgresConnectionManager::new(pg_config, tls);
    let min_idle = config.min_connections.min(config.max_connections);
    Pool::builder()
        .max_size(config.max_connections)
        .min_idle(Some(min_idle))
        .build(manager)
        .map_err(|error| {
            format!(
                "failed to create shared IM postgres r2d2 pool ({}): {error}",
                redact_postgres_url(config.url.as_str())
            )
        })
}

fn make_tls_connector() -> Result<postgres_native_tls::MakeTlsConnector, native_tls::Error> {
    let connector = native_tls::TlsConnector::builder().build()?;
    Ok(postgres_native_tls::MakeTlsConnector::new(connector))
}

fn verify_production_sslmode(database_url: &str) {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let is_production =
        !matches!(environment.as_str(), "" | "dev" | "development" | "test" | "testing");
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

fn redact_postgres_url(database_url: &str) -> String {
    let Ok(mut url) = url::Url::parse(database_url) else {
        return "<invalid-postgres-url>".to_owned();
    };
    if !url.username().is_empty() {
        let _ = url.set_username("<redacted>");
    }
    if url.password().is_some() {
        let _ = url.set_password(Some("<redacted>"));
    }
    url.to_string()
}
