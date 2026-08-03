//! Shared PostgreSQL pool, configuration, I/O bridge, and value conversion support.

use chrono::{DateTime, Utc};
use im_platform_contracts::ContractError;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use tokio::runtime::Handle;

use crate::{PostgresAggregateStore, PostgresCommitJournal};

const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

pub type PostgresJournalTlsConnector = postgres_native_tls::MakeTlsConnector;
pub type PostgresJournalConnectionManager = PostgresConnectionManager<PostgresJournalTlsConnector>;

#[derive(Clone)]
pub struct PostgresJournalPool(Option<Pool<PostgresJournalConnectionManager>>);

impl PostgresJournalPool {
    pub fn from_pool(pool: Pool<PostgresJournalConnectionManager>) -> Self {
        Self(Some(pool))
    }

    pub fn inner(&self) -> &Pool<PostgresJournalConnectionManager> {
        self.0
            .as_ref()
            .expect("postgres journal pool should remain initialized")
    }
}

impl std::ops::Deref for PostgresJournalPool {
    type Target = Pool<PostgresJournalConnectionManager>;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl Drop for PostgresJournalPool {
    fn drop(&mut self) {
        if let Some(pool) = self.0.take() {
            drop_journal_pool_off_runtime(pool);
        }
    }
}

fn drop_journal_pool_off_runtime(pool: Pool<PostgresJournalConnectionManager>) {
    if Handle::try_current().is_err() {
        drop(pool);
        return;
    }
    std::thread::spawn(move || drop(pool));
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresJournalConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl PostgresJournalConfig {
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

    pub fn connect_pool(&self) -> Result<PostgresJournalPool, ContractError> {
        if Handle::try_current().is_ok() {
            return self.connect_pool_bridged();
        }
        build_journal_pool(self)
    }

    pub fn connect_pool_bridged(&self) -> Result<PostgresJournalPool, ContractError> {
        let config = self.clone();
        run_postgres_io(move || build_journal_pool(&config))
    }

    pub fn connect(self) -> Result<PostgresCommitJournal, ContractError> {
        self.connect_pool().map(PostgresCommitJournal::from_pool)
    }
}

fn build_journal_pool(
    _config: &PostgresJournalConfig,
) -> Result<PostgresJournalPool, ContractError> {
    if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
        return Ok(PostgresJournalPool::from_pool(pool));
    }
    #[cfg(test)]
    {
        return build_journal_pool_local(_config);
    }
    #[cfg(not(test))]
    {
        let _ = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool();
        Err(ContractError::Unavailable(
            "IM process PostgreSQL pool is unavailable".into(),
        ))
    }
}

#[cfg(test)]
fn build_journal_pool_local(
    config: &PostgresJournalConfig,
) -> Result<PostgresJournalPool, ContractError> {
    verify_production_sslmode(config.database_url.as_str())?;
    let pg_config = config
        .database_url
        .parse()
        .map_err(|error| postgres_config_error(config.database_url.as_str(), error))?;
    let tls = make_tls_connector().map_err(|_| {
        ContractError::Unavailable("postgres journal TLS connector build failed".into())
    })?;
    let manager = PostgresConnectionManager::new(pg_config, tls);
    Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(config.pool_min_idle)
        .build(manager)
        .map_err(|error| postgres_unavailable("create journal pool", error))
        .map(PostgresJournalPool::from_pool)
}

#[cfg(test)]
fn make_tls_connector() -> Result<postgres_native_tls::MakeTlsConnector, native_tls::Error> {
    let connector = native_tls::TlsConnector::builder().build()?;
    Ok(postgres_native_tls::MakeTlsConnector::new(connector))
}

#[cfg(test)]
fn verify_production_sslmode(database_url: &str) -> Result<(), ContractError> {
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
        return Err(ContractError::Unavailable(format!(
            "P0-12 production fail-closed: SDKWORK_DATABASE_URL requires TLS in environment {environment}"
        )));
    }
    Ok(())
}

pub(crate) fn postgres_jsonb_payload(payload: &str) -> Result<serde_json::Value, ContractError> {
    serde_json::from_str(payload)
        .map_err(|_| ContractError::Invalid("postgres journal payload must be valid JSON".into()))
}

pub(crate) fn postgres_timestamptz(
    value: &str,
    field: &'static str,
) -> Result<DateTime<Utc>, ContractError> {
    DateTime::parse_from_rfc3339(value.trim())
        .map(|instant| instant.with_timezone(&Utc))
        .or_else(|_| value.trim().parse::<DateTime<Utc>>())
        .map_err(|_| ContractError::Invalid(format!("postgres journal {field} must be RFC3339")))
}

pub(crate) fn postgres_row_get<T>(
    row: &postgres::Row,
    column: usize,
    action: &'static str,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> postgres::types::FromSql<'a>,
{
    row.try_get(column).map_err(|_| {
        ContractError::Unavailable(format!(
            "postgres journal {action} returned an incompatible {field} field"
        ))
    })
}

pub(crate) fn postgres_bigint_input(value: u64, field: &'static str) -> Result<i64, ContractError> {
    i64::try_from(value).map_err(|_| {
        ContractError::Invalid(format!(
            "postgres journal {field} exceeds the PostgreSQL BIGINT range"
        ))
    })
}

pub(crate) fn postgres_bigint_output(
    value: i64,
    field: &'static str,
) -> Result<u64, ContractError> {
    u64::try_from(value).map_err(|_| {
        ContractError::Unavailable(format!(
            "postgres journal returned an invalid {field} field"
        ))
    })
}

pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    if let Ok(handle) = Handle::try_current()
        && handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread
    {
        return tokio::task::block_in_place(operation);
    }
    std::thread::scope(|scope| {
        scope
            .spawn(operation)
            .join()
            .map_err(|_| postgres_io_thread_panic())?
    })
}

fn postgres_io_thread_panic() -> ContractError {
    ContractError::Unavailable("postgres journal blocking IO worker panicked".into())
}

fn resolve_im_postgres_search_path_schema() -> Result<Option<String>, ContractError> {
    let schema = sdkwork_database_config::workspace_database::resolve_workspace_postgres_schema()
        .map_err(|error| {
        ContractError::Unavailable(format!("invalid workspace postgres profile: {error}"))
    })?;
    Ok((schema != "public").then_some(schema))
}

fn apply_postgres_search_path(
    client: &mut r2d2::PooledConnection<PostgresJournalConnectionManager>,
    schema: &str,
) -> Result<(), ContractError> {
    if !schema
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(ContractError::Unavailable(format!(
            "invalid postgres search_path schema `{schema}`"
        )));
    }
    let sql = format!("SET search_path TO \"{schema}\", public");
    client
        .batch_execute(&sql)
        .map_err(|error| postgres_unavailable_db("set search_path", error))?;
    Ok(())
}

pub(crate) fn postgres_pool_client(
    pool: &PostgresJournalPool,
    action: &'static str,
) -> Result<r2d2::PooledConnection<PostgresJournalConnectionManager>, ContractError> {
    let mut client = pool
        .get()
        .map_err(|error| postgres_unavailable(action, error))?;
    if let Some(schema) = resolve_im_postgres_search_path_schema()? {
        apply_postgres_search_path(&mut client, schema.as_str())?;
    }
    Ok(client)
}

pub(crate) fn now_rfc3339() -> String {
    im_time::utc_now_rfc3339_millis()
}

use tracing;

pub(crate) fn postgres_unavailable(
    action: &'static str,
    error: impl std::fmt::Display,
) -> ContractError {
    tracing::error!(
        action,
        error = %error,
        "postgres journal operation failed"
    );
    ContractError::Unavailable(format!("postgres journal {action} failed"))
}

pub(crate) fn postgres_unavailable_db(
    action: &'static str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    tracing::error!(
        action,
        error = %error,
        "postgres journal database operation failed"
    );
    ContractError::Unavailable(format!("postgres journal {action} failed"))
}

#[cfg(test)]
fn postgres_config_error(
    _database_url: &str,
    _error: r2d2_postgres::postgres::Error,
) -> ContractError {
    ContractError::Unavailable("postgres journal database URL is invalid".into())
}

pub fn conversation_member_access_gate_from_pool(
    pool: PostgresJournalPool,
) -> std::sync::Arc<dyn im_platform_contracts::ConversationMemberAccessGate> {
    use im_platform_contracts::AggregateStoreConversationMemberAccessGate;
    std::sync::Arc::new(AggregateStoreConversationMemberAccessGate::new(
        std::sync::Arc::new(PostgresAggregateStore::from_pool(pool)),
    ))
}
