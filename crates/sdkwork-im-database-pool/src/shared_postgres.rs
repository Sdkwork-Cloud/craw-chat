//! Process-wide shared PostgreSQL pools for every IM deployment profile.
//!
//! Standalone and cloud binaries MUST install one
//! sqlx lifecycle host and one r2d2 PostgreSQL pool per process via
//! [`bootstrap_im_process_database_pools_from_env`]. Adapters MUST reuse that r2d2
//! pool and MUST NOT open independent pools against the same DSN.

use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use r2d2::{HandleError, Pool};
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_database_config::DatabaseConfig;
use sdkwork_im_database_host::ImDatabaseHost;
use tracing::info;

use crate::{bootstrap_im_database, ensure_im_core_postgres_authority};

/// TLS connector type for the shared IM PostgreSQL r2d2 pool.
pub type ImSharedPostgresTlsConnector = postgres_native_tls::MakeTlsConnector;
/// Connection manager for the shared IM PostgreSQL r2d2 pool.
pub type ImSharedPostgresConnectionManager =
    PostgresConnectionManager<ImSharedPostgresTlsConnector>;
/// Canonical synchronous PostgreSQL pool shared by IM modules in one process.
pub type ImSharedPostgresR2d2Pool = Pool<ImSharedPostgresConnectionManager>;

static IM_PROCESS_DATABASE_POOLS: OnceLock<ImProcessDatabasePools> = OnceLock::new();

#[derive(Debug)]
struct CapturingPostgresErrorHandler {
    last_error: Arc<Mutex<Option<String>>>,
}

impl HandleError<r2d2_postgres::postgres::Error> for CapturingPostgresErrorHandler {
    fn handle_error(&self, error: r2d2_postgres::postgres::Error) {
        if let Ok(mut last_error) = self.last_error.lock() {
            *last_error = Some(format!("{error:?}"));
        }
    }
}

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

/// Whether shared IM process pools were installed in this process.
pub fn is_im_process_database_pools_installed() -> bool {
    IM_PROCESS_DATABASE_POOLS.get().is_some()
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

/// Bootstrap IM lifecycle (sqlx) plus one shared r2d2 pool for all modules in this process.
pub async fn bootstrap_im_process_database_pools_from_env()
-> Result<&'static ImProcessDatabasePools, String> {
    sdkwork_database_sqlx::enable_process_shared_database_pool();
    if let Some(pools) = im_process_database_pools() {
        return Ok(pools);
    }

    let config = DatabaseConfig::from_env("IM")
        .map_err(|error| format!("read IM database config failed: {error}"))?;
    ensure_im_core_postgres_authority(&config)?;

    let sqlx_pool = sdkwork_database_sqlx::create_pool_from_config(config.clone())
        .await
        .map_err(|error| format!("create IM process SQLx pool failed: {error}"))?;
    let sqlx_max_connections = sqlx_pool.config().max_connections;
    let host = bootstrap_im_database(sqlx_pool).await?;

    let r2d2_max_connections =
        sdkwork_database_sqlx::process_shared_temporary_driver_max_connections().ok_or_else(
            || {
                "IM r2d2 capacity was not reserved; set SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT before process pool bootstrap"
                    .to_owned()
            },
        )?;
    let mut r2d2_config = config.clone();
    r2d2_config.max_connections = r2d2_max_connections;
    r2d2_config.min_connections = r2d2_config.min_connections.min(r2d2_max_connections);
    let postgres_r2d2 = Arc::new(build_im_postgres_r2d2_pool(&r2d2_config)?);
    let pools = ImProcessDatabasePools {
        host,
        postgres_r2d2,
    };
    IM_PROCESS_DATABASE_POOLS
        .set(pools)
        .map_err(|_| "IM process database pools already installed in this process".to_owned())?;

    let pool_tuning = read_im_postgres_pool_tuning();
    info!(
        target: "sdkwork.im",
        event = "im.database.process_pools_installed",
        process_max_connections = config.max_connections,
        sqlx_max_connections,
        r2d2_max_connections,
        pool_connection_timeout_secs = pool_tuning.connection_timeout.as_secs(),
        pool_max_lifetime_secs = pool_tuning
            .max_lifetime
            .map(|d| d.as_secs())
            .unwrap_or(0),
        pool_idle_timeout_secs = pool_tuning
            .idle_timeout
            .map(|d| d.as_secs())
            .unwrap_or(0),
        database_url = %redact_postgres_url(config.url.as_str()),
        "installed shared IM sqlx lifecycle host and single postgres r2d2 pool"
    );

    Ok(im_process_database_pools().expect("process database pools installed"))
}

/// Tuning parameters for the shared r2d2 PostgreSQL pool. Defaults follow r2d2
/// best practices for a long-lived server process: short acquire timeout so
/// callers fail fast under pool exhaustion, bounded connection lifetime so
/// load balancers and PG `max_connections` see churn, and bounded idle timeout
/// so quiet workers release their connections back to PostgreSQL.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ImPostgresPoolTuning {
    pub connection_timeout: Duration,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
}

impl ImPostgresPoolTuning {
    /// Defaults applied when the corresponding env var is unset. Operators
    /// can override via the env vars documented below.
    const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 10;
    const DEFAULT_MAX_LIFETIME_SECS: u64 = 1800;
    const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 600;
}

fn parse_duration_env(var: &str, default_secs: u64) -> Result<Duration, String> {
    let raw = std::env::var(var).unwrap_or_default();
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Duration::from_secs(default_secs));
    }
    let parsed: u64 = trimmed
        .parse()
        .map_err(|error| format!("invalid {var} value {trimmed:?}: {error}"))?;
    if parsed == 0 {
        return Err(format!(
            "{var} must be > 0 (use a very large value to effectively disable the limit)"
        ));
    }
    Ok(Duration::from_secs(parsed))
}

fn parse_optional_duration_env(var: &str, default_secs: u64) -> Result<Option<Duration>, String> {
    let raw = std::env::var(var).unwrap_or_default();
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Some(Duration::from_secs(default_secs)));
    }
    // Operators may set `0` (or `disabled`/`none`) to explicitly disable the
    // lifetime/idle timeout — useful for environments where the connection
    // lifetime is managed externally (e.g. PgBouncer transaction pooling).
    let lowered = trimmed.to_ascii_lowercase();
    if lowered == "0" || lowered == "disabled" || lowered == "none" || lowered == "off" {
        return Ok(None);
    }
    let parsed: u64 = trimmed
        .parse()
        .map_err(|error| format!("invalid {var} value {trimmed:?}: {error}"))?;
    if parsed == 0 {
        return Ok(None);
    }
    Ok(Some(Duration::from_secs(parsed)))
}

fn read_im_postgres_pool_tuning() -> ImPostgresPoolTuning {
    let connection_timeout = parse_duration_env(
        "SDKWORK_DATABASE_POOL_CONNECTION_TIMEOUT_SECONDS",
        ImPostgresPoolTuning::DEFAULT_CONNECTION_TIMEOUT_SECS,
    )
    .expect("invalid SDKWORK_DATABASE_POOL_CONNECTION_TIMEOUT_SECONDS");
    let max_lifetime = parse_optional_duration_env(
        "SDKWORK_DATABASE_POOL_MAX_LIFETIME_SECONDS",
        ImPostgresPoolTuning::DEFAULT_MAX_LIFETIME_SECS,
    )
    .expect("invalid SDKWORK_DATABASE_POOL_MAX_LIFETIME_SECONDS");
    let idle_timeout = parse_optional_duration_env(
        "SDKWORK_DATABASE_POOL_IDLE_TIMEOUT_SECONDS",
        ImPostgresPoolTuning::DEFAULT_IDLE_TIMEOUT_SECS,
    )
    .expect("invalid SDKWORK_DATABASE_POOL_IDLE_TIMEOUT_SECONDS");
    ImPostgresPoolTuning {
        connection_timeout,
        max_lifetime,
        idle_timeout,
    }
}

pub(crate) fn build_im_postgres_r2d2_pool(
    config: &DatabaseConfig,
) -> Result<ImSharedPostgresR2d2Pool, String> {
    verify_production_sslmode(config.url.as_str())?;
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
    let tuning = read_im_postgres_pool_tuning();
    let last_error = Arc::new(Mutex::new(None));
    Pool::builder()
        .max_size(config.max_connections)
        .min_idle(Some(min_idle))
        .connection_timeout(tuning.connection_timeout)
        .max_lifetime(tuning.max_lifetime)
        .idle_timeout(tuning.idle_timeout)
        .error_handler(Box::new(CapturingPostgresErrorHandler {
            last_error: last_error.clone(),
        }))
        .build(manager)
        .map_err(|error| {
            let cause = last_error
                .lock()
                .ok()
                .and_then(|last_error| last_error.clone())
                .unwrap_or_else(|| format!("{error:?}"));
            format!(
                "failed to create shared IM postgres r2d2 pool ({}): {cause}",
                redact_postgres_url(config.url.as_str()),
            )
        })
}

fn make_tls_connector() -> Result<postgres_native_tls::MakeTlsConnector, native_tls::Error> {
    let connector = native_tls::TlsConnector::builder().build()?;
    Ok(postgres_native_tls::MakeTlsConnector::new(connector))
}

fn verify_production_sslmode(database_url: &str) -> Result<(), String> {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let is_production = !matches!(
        environment.as_str(),
        "" | "dev" | "development" | "test" | "testing"
    );
    if !is_production {
        return Ok(());
    }
    let lowered = database_url.to_ascii_lowercase();
    let requires_tls = lowered.contains("sslmode=require")
        || lowered.contains("sslmode=verify-ca")
        || lowered.contains("sslmode=verify-full")
        || lowered.contains("sslmode=verifyca")
        || lowered.contains("sslmode=verifyfull");
    if !requires_tls {
        return Err(format!(
            "P0-12 production fail-closed: SDKWORK_DATABASE_URL must contain \
             sslmode=require or sslmode=verify-full in production \
             (current environment={environment}). Refusing to start with a \
             plaintext database connection."
        ));
    }
    Ok(())
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
