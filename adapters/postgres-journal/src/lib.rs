//! PostgreSQL-backed implementation of the IM [`CommitJournal`] contract.
//!
//! This adapter writes durable conversation/message commit events into the
//! `im_commit_journal` table defined by
//! `database/ddl/baseline/postgres/0001_im_baseline.sql` (via `database/` lifecycle module).
//!
//! It replaces the previous single-machine JSONL append file
//! (`adapters/local-disk/src/journal.rs`) as the production source of truth,
//! while keeping the synchronous [`CommitJournal`] trait surface stable so
//! callers in `conversation-runtime` and `sdkwork-im-cloud-gateway` do not change.
//!
//! ## Threading bridge
//!
//! [`CommitJournal`] is a synchronous trait, but PostgreSQL I/O must never
//! block the tokio runtime. Like `adapters/postgres-realtime`, this crate
//! uses a synchronous `r2d2` connection pool and bridges each call onto a
//! dedicated blocking scope via [`run_postgres_io`]. A future cross-cutting
//! optimization (tracked as P3) may move both realtime and journal adapters
//! to an async-native pool; that refactor is intentionally out of scope here
//! so the data-layer change stays surgical.
//!
//! ## Spec alignment
//!
//! - DATABASE_SPEC §5.1 (`event_log`) and §17 (event consistency).
//! - `im_commit_journal` is the append-only, cursor-indexed event log; the
//!   composite primary key `(partition_key, commit_offset)` and the unique
//!   `event_id` enforce idempotent appends.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use im_domain_core::retention::retention_until_from_envelope;
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA,
    COMMIT_JOURNAL_REPLAY_BATCH_LIMIT, CommitJournal, CommitJournalAggregateEventTypeQuery,
    CommitJournalAggregateScope, CommitJournalReplayCursor, CommitJournalReplayPage,
    CommitPosition, ContractError,
};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_utils_rust::sha256_hash;
use tokio::runtime::Handle;

mod aggregate_store;
mod automation_execution_store;
mod message_post_persistence;
mod message_store;
mod notification_task_store;
mod outbox_store;
mod retention_cleanup;
mod retention_metrics;
mod retention_reconcile;
mod retention_scheduler;
mod search_store;
mod seq_allocator;
mod stream_state_store;

pub use aggregate_store::PostgresAggregateStore;
pub use automation_execution_store::PostgresAutomationExecutionStore;
pub use im_platform_contracts::CommitJournalReplayCursor as JournalReplayCursor;
pub use message_post_persistence::{
    PostgresDurableConversationEventWriter, PostgresDurableMessagePostWriter,
};
pub use message_store::PostgresMessageStore;
pub use notification_task_store::PostgresNotificationTaskStore;
pub use outbox_store::PostgresOutboxStore;
pub use retention_cleanup::{RetentionCleanupReport, purge_expired_retention_batch};
pub use retention_metrics::{RetentionPurgeMetrics, retention_purge_metrics};
pub use retention_reconcile::{
    PostgresRetentionScopeStore, RetentionReconcileReport, clear_conversation_retention_until,
};
pub use retention_scheduler::{
    RetentionPurgeSchedulerConfig, RetentionPurgeSchedulerHandle, spawn_retention_purge_scheduler,
    spawn_retention_purge_scheduler_from_env,
};
pub use search_store::{MemberSearchQuery, PostgresSearchProvider};
pub use seq_allocator::PostgresConversationSeqAllocator;
pub use stream_state_store::PostgresStreamStateStore;

/// Default upper bound on pooled PostgreSQL connections for the journal store.
///
/// Production deployments should configure a larger pool size based on:
/// - Number of concurrent requests
/// - Database server capabilities (CPU, memory)
/// - Network latency to database
/// - Typical connection reuse rate
///
/// Recommendation: Start with 50% of max_connections from database config
/// and adjust based on monitoring metrics.
const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

/// TLS connector type for r2d2-backed PostgreSQL pools.
///
/// P0-12 (SECURITY_SPEC): uses `postgres-native-tls` so the `sslmode` URL
/// parameter is honored. With `sslmode=disable` the connector is never
/// invoked (plaintext TCP); with `sslmode=require` or `verify-full` a real
/// TLS handshake is performed. This allows dev/test to keep using plaintext
/// while production enforces TLS via the DSN.
pub type PostgresJournalTlsConnector = postgres_native_tls::MakeTlsConnector;
/// Connection manager / pool type aliases mirroring the realtime adapter.
pub type PostgresJournalConnectionManager = PostgresConnectionManager<PostgresJournalTlsConnector>;

/// Owned r2d2 pool that drops off Tokio worker threads.
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
    if tokio::runtime::Handle::try_current().is_err() {
        drop(pool);
        return;
    }
    std::thread::spawn(move || drop(pool));
}

/// Resolved connection configuration for the journal store.
///
/// `database_url` follows the standard PostgreSQL URI form. Production
/// deployments MUST set `sslmode=require` (or `verify-full`) on the URL;
/// [`verify_production_sslmode`] enforces this fail-closed at pool build
/// time. See `docs/部署/postgresql-database-configuration.md`.
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

    /// Build the r2d2 connection pool.
    ///
    /// Errors here are surfaced as [`ContractError::Unavailable`] without
    /// driver or connection-string details.
    pub fn connect_pool(&self) -> Result<PostgresJournalPool, ContractError> {
        if Handle::try_current().is_ok() {
            return self.connect_pool_bridged();
        }
        build_journal_pool(self)
    }

    /// Creates a pool on a dedicated OS thread when called from a Tokio runtime.
    pub fn connect_pool_bridged(&self) -> Result<PostgresJournalPool, ContractError> {
        let config = self.clone();
        run_postgres_io(move || build_journal_pool(&config))
    }

    pub fn connect(self) -> Result<PostgresCommitJournal, ContractError> {
        let pool = self.connect_pool()?;
        Ok(PostgresCommitJournal {
            pool,
            partition_prefix: Arc::from(""),
        })
    }
}

fn build_journal_pool(
    config: &PostgresJournalConfig,
) -> Result<PostgresJournalPool, ContractError> {
    if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
        return Ok(PostgresJournalPool::from_pool(pool));
    }
    if cfg!(test) {
        return build_journal_pool_local(config);
    }
    let _ = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool();
    Err(ContractError::Unavailable(
        "IM process PostgreSQL pool is unavailable".into(),
    ))
}

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
            "P0-12 production fail-closed: SDKWORK_IM_DATABASE_URL must contain \
                 sslmode=require or sslmode=verify-full in production \
                 (current environment={environment}). Refusing to start with a \
                 plaintext database connection."
        )));
    }
    Ok(())
}

/// PostgreSQL implementation of [`CommitJournal`].
///
/// Writes are append-only and idempotent on `event_id` (the table's unique
/// constraint `uk_im_commit_journal_event`). Re-appending the same event id
/// returns the previously committed position instead of erroring, preserving
/// at-least-once delivery semantics for upstream producers.
#[derive(Clone)]
pub struct PostgresCommitJournal {
    pool: PostgresJournalPool,
    /// Optional logical namespace prepended to every `partition_key`. Empty
    /// by default; reserved for future multi-shard routing.
    partition_prefix: Arc<str>,
}

impl PostgresCommitJournal {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self {
            pool,
            partition_prefix: Arc::from(""),
        }
    }

    /// Incremental journal replay cursor keyed by `(partition_key, commit_offset)`.
    pub fn recorded_after(
        &self,
        cursor: Option<&JournalReplayCursor>,
    ) -> Result<(Vec<CommitEnvelope>, Option<JournalReplayCursor>), ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let cursor = cursor.cloned();
        run_postgres_io(move || {
            load_recorded_page(
                &pool,
                &prefix,
                cursor.as_ref(),
                COMMIT_JOURNAL_REPLAY_BATCH_LIMIT as i64,
            )
        })
    }

    pub fn with_partition_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.partition_prefix = Arc::from(prefix.into());
        self
    }

    pub fn pool(&self) -> &PostgresJournalPool {
        &self.pool
    }

    pub fn partition_prefix(&self) -> &Arc<str> {
        &self.partition_prefix
    }

    /// Appends a batch and applies a related PostgreSQL mutation in one transaction.
    ///
    /// The journal owns aggregate sequence allocation for this path. Partition locks are
    /// acquired in lexical order so concurrent multi-aggregate batches cannot deadlock.
    /// The callback receives only newly inserted envelopes with their committed
    /// `ordering_seq` values. Exact event-ID replay never reapplies an older mutation.
    pub fn append_batch_with_allocated_sequences_in_transaction<F>(
        &self,
        mut envelopes: Vec<CommitEnvelope>,
        apply: F,
    ) -> Result<Vec<CommitPosition>, ContractError>
    where
        F: for<'txn> FnOnce(
                &mut r2d2_postgres::postgres::Transaction<'txn>,
                &[CommitEnvelope],
            ) -> Result<(), ContractError>
            + Send,
    {
        if envelopes.is_empty() {
            return Ok(Vec::new());
        }

        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "journal coordinated append")?;
            let mut txn = client.transaction().map_err(|error| {
                postgres_unavailable_db("journal coordinated append begin", error)
            })?;
            lock_journal_partitions(
                &mut txn,
                prefix.as_ref(),
                &envelopes,
                "journal coordinated append lock",
            )?;
            let existing_event_ids =
                allocate_next_ordering_sequences(&mut txn, prefix.as_ref(), &mut envelopes)?;
            let positions = append_many_on_transaction(&mut txn, prefix.as_ref(), &envelopes)?;
            let inserted_envelopes = envelopes
                .iter()
                .filter(|envelope| !existing_event_ids.contains(&envelope.event_id))
                .cloned()
                .collect::<Vec<_>>();
            apply(&mut txn, &inserted_envelopes)?;
            txn.commit().map_err(|error| {
                postgres_unavailable_db("journal coordinated append commit", error)
            })?;
            Ok(positions)
        })
    }
}

impl CommitJournal for PostgresCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || append_one(&pool, &prefix, &envelope))
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Ok(Vec::new());
        }
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || append_many(&pool, &prefix, envelopes))
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "PostgreSQL journal readback requires recorded_page keyset replay; unbounded recorded() is disabled"
                .into(),
        ))
    }

    fn recorded_page(
        &self,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let cursor = cursor.cloned();
        let limit = i64::try_from(limit.clamp(1, COMMIT_JOURNAL_REPLAY_BATCH_LIMIT))
            .unwrap_or(COMMIT_JOURNAL_REPLAY_BATCH_LIMIT as i64);
        run_postgres_io(move || {
            let (items, next_cursor) = load_recorded_page(&pool, &prefix, cursor.as_ref(), limit)?;
            Ok(CommitJournalReplayPage { items, next_cursor })
        })
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let scope = scope.clone();
        let cursor = cursor.cloned();
        let limit = i64::try_from(limit.clamp(1, COMMIT_JOURNAL_REPLAY_BATCH_LIMIT))
            .unwrap_or(COMMIT_JOURNAL_REPLAY_BATCH_LIMIT as i64);
        run_postgres_io(move || {
            let (items, next_cursor) =
                load_recorded_page_for_aggregate(&pool, &prefix, &scope, cursor.as_ref(), limit)?;
            Ok(CommitJournalReplayPage { items, next_cursor })
        })
    }

    fn recorded_page_for_aggregate_event_types(
        &self,
        query: &CommitJournalAggregateEventTypeQuery,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        validate_aggregate_event_type_query(query)?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let query = query.clone();
        let cursor = cursor.cloned();
        let limit = i64::try_from(limit.clamp(1, COMMIT_JOURNAL_REPLAY_BATCH_LIMIT))
            .unwrap_or(COMMIT_JOURNAL_REPLAY_BATCH_LIMIT as i64);
        run_postgres_io(move || {
            let (items, next_cursor) = load_recorded_page_for_aggregate_event_types(
                &pool,
                &prefix,
                &query,
                cursor.as_ref(),
                limit,
            )?;
            Ok(CommitJournalReplayPage { items, next_cursor })
        })
    }
}

// ---------------------------------------------------------------------------
// SQL constants — kept parameterised to prevent injection. Bindings match the
// `im_commit_journal` column order exactly (see migration lines 5-22).
// ---------------------------------------------------------------------------

/// Insert a single event, returning the committed `(partition_key, commit_offset)`.
///
/// `ON CONFLICT (event_id) DO NOTHING` makes the append idempotent: a replayed
/// producer event is absorbed and the existing position is read back, rather
/// than raising a constraint violation the caller cannot distinguish from a
/// genuine write failure.
pub(crate) const APPEND_EVENT_SQL: &str = r#"
insert into im_commit_journal (
    partition_key,
    commit_offset,
    event_id,
    tenant_id,
    organization_id,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    event_type,
    payload_json,
    payload_hash,
    idempotency_key,
    occurred_at,
    created_at,
    retention_until
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13, $14, $15)
on conflict (event_id) do nothing
returning partition_key, commit_offset
"#;

pub(crate) const LOAD_EVENT_BY_ID_SQL: &str = r#"
select
    partition_key,
    commit_offset,
    tenant_id,
    organization_id,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    event_type,
    payload_hash,
    idempotency_key,
    occurred_at,
    retention_until
from im_commit_journal
where event_id = $1
"#;

/// Look up an existing journal row by the composite primary key
/// `(partition_key, commit_offset)`. Used after a SQLSTATE 23505 collision on
/// the primary key to determine whether the occupying row belongs to the same
/// `event_id` (defensive idempotent replay) or to a different `event_id`
/// (genuine position conflict that must surface as `Conflict`).
pub(crate) const LOAD_EVENT_BY_POSITION_SQL: &str = r#"
select event_id, partition_key, commit_offset
from im_commit_journal
where partition_key = $1 and commit_offset = $2
"#;

const LOCK_JOURNAL_PARTITION_SQL: &str =
    "select pg_advisory_xact_lock(hashtextextended($1, 0::bigint))";

const LOAD_MAX_AGGREGATE_SEQ_SQL: &str = r#"
select coalesce(max(commit_offset), 0)::bigint
from im_commit_journal
where partition_key = $1
"#;

const LOAD_RECORDED_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
order by partition_key asc, commit_offset asc
limit $2
"#;

const LOAD_RECORDED_AFTER_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and (partition_key, commit_offset) > ($2, $3)
order by partition_key asc, commit_offset asc
limit $4
"#;

const LOAD_RECORDED_AGGREGATE_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and aggregate_id = $3
order by commit_offset asc
limit $4
"#;

const LOAD_RECORDED_AGGREGATE_AFTER_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and aggregate_id = $3
  and commit_offset > $4
order by commit_offset asc
limit $5
"#;

const LOAD_RECORDED_AGGREGATE_EVENT_TYPES_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_type = $4
  and aggregate_id = $5
  and event_type = any($6)
order by commit_offset asc
limit $7
"#;

const LOAD_RECORDED_AGGREGATE_EVENT_TYPES_AFTER_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at::text,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_type = $4
  and aggregate_id = $5
  and event_type = any($6)
  and commit_offset > $7
order by commit_offset asc
limit $8
"#;

// ---------------------------------------------------------------------------
// Blocking I/O helpers (executed off the async runtime by `run_postgres_io`).
// ---------------------------------------------------------------------------

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

fn journal_replay_row_get<T>(
    row: &postgres::Row,
    column: usize,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> postgres::types::FromSql<'a>,
{
    postgres_row_get(row, column, "replay", field)
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

/// Outcome of an `INSERT ... ON CONFLICT (event_id) DO NOTHING` against
/// `im_commit_journal`. Distinguishes the three possible results so the
/// caller can resolve the final commit position correctly:
///
/// - `Inserted`: new row written; read position from the RETURNING clause.
/// - `EventIdAbsorbed`: ON CONFLICT absorbed a duplicate `event_id`;
///   read the previously committed position by `event_id` (idempotent replay).
/// - `PositionCollision`: SQLSTATE 23505 on the `(partition_key,
///   commit_offset)` primary key with a different `event_id`; look up the
///   occupying row by position to confirm and surface a `Conflict`.
enum InsertOutcome {
    Inserted(r2d2_postgres::postgres::Row),
    EventIdAbsorbed,
    PositionCollision,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JournalEventFingerprint {
    partition_key: String,
    commit_offset: i64,
    tenant_id: String,
    organization_id: String,
    aggregate_type: String,
    aggregate_id: String,
    aggregate_seq: i64,
    event_type: String,
    payload_hash: String,
    idempotency_key: Option<String>,
    occurred_at_micros: i64,
    retention_until_micros: Option<i64>,
}

impl JournalEventFingerprint {
    fn from_envelope(prefix: &str, envelope: &CommitEnvelope) -> Result<Self, ContractError> {
        let aggregate_seq = journal_aggregate_seq(envelope.ordering_seq)?;
        let occurred_at = postgres_timestamptz(envelope.occurred_at.as_str(), "occurred_at")?;
        let retention_until = journal_retention_until(envelope)
            .as_deref()
            .map(|value| postgres_timestamptz(value, "retention_until"))
            .transpose()?;
        Ok(Self {
            partition_key: compose_partition_key(prefix, envelope.ordering_key.as_str()),
            commit_offset: aggregate_seq,
            tenant_id: envelope.tenant_id.clone(),
            organization_id: envelope.normalized_organization_id(),
            aggregate_type: envelope.aggregate_type.as_wire_value().to_owned(),
            aggregate_id: envelope.aggregate_id.clone(),
            aggregate_seq,
            event_type: envelope.event_type.clone(),
            payload_hash: sha256_hash(envelope.payload.as_bytes()),
            idempotency_key: envelope.idempotency_key.clone(),
            occurred_at_micros: occurred_at.timestamp_micros(),
            retention_until_micros: retention_until.map(|value| value.timestamp_micros()),
        })
    }

    fn from_row(row: &r2d2_postgres::postgres::Row) -> Result<Self, ContractError> {
        Ok(Self {
            partition_key: journal_fingerprint_row_get(row, 0, "partition_key")?,
            commit_offset: journal_fingerprint_row_get(row, 1, "commit_offset")?,
            tenant_id: journal_fingerprint_row_get(row, 2, "tenant_id")?,
            organization_id: journal_fingerprint_row_get(row, 3, "organization_id")?,
            aggregate_type: journal_fingerprint_row_get(row, 4, "aggregate_type")?,
            aggregate_id: journal_fingerprint_row_get(row, 5, "aggregate_id")?,
            aggregate_seq: journal_fingerprint_row_get(row, 6, "aggregate_seq")?,
            event_type: journal_fingerprint_row_get(row, 7, "event_type")?,
            payload_hash: journal_fingerprint_row_get(row, 8, "payload_hash")?,
            idempotency_key: journal_fingerprint_row_get(row, 9, "idempotency_key")?,
            occurred_at_micros: journal_fingerprint_row_get::<DateTime<Utc>>(
                row,
                10,
                "occurred_at",
            )?
            .timestamp_micros(),
            retention_until_micros: journal_fingerprint_row_get::<Option<DateTime<Utc>>>(
                row,
                11,
                "retention_until",
            )?
            .map(|value| value.timestamp_micros()),
        })
    }

    fn position(&self) -> (String, i64) {
        (self.partition_key.clone(), self.commit_offset)
    }
}

fn journal_fingerprint_row_get<T>(
    row: &r2d2_postgres::postgres::Row,
    column: usize,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> r2d2_postgres::postgres::types::FromSql<'a>,
{
    postgres_row_get(row, column, "event replay", field)
}

pub(crate) fn ensure_journal_event_replay_matches(
    existing: &JournalEventFingerprint,
    attempted: &JournalEventFingerprint,
    _event_id: &str,
) -> Result<(), ContractError> {
    let mut mismatched_fields = Vec::new();
    if existing.partition_key != attempted.partition_key {
        mismatched_fields.push("partition_key");
    }
    if existing.commit_offset != attempted.commit_offset {
        mismatched_fields.push("commit_offset");
    }
    if existing.tenant_id != attempted.tenant_id {
        mismatched_fields.push("tenant_id");
    }
    if existing.organization_id != attempted.organization_id {
        mismatched_fields.push("organization_id");
    }
    if existing.aggregate_type != attempted.aggregate_type {
        mismatched_fields.push("aggregate_type");
    }
    if existing.aggregate_id != attempted.aggregate_id {
        mismatched_fields.push("aggregate_id");
    }
    if existing.aggregate_seq != attempted.aggregate_seq {
        mismatched_fields.push("aggregate_seq");
    }
    if existing.event_type != attempted.event_type {
        mismatched_fields.push("event_type");
    }
    if existing.payload_hash != attempted.payload_hash {
        mismatched_fields.push("payload_hash");
    }
    if existing.idempotency_key != attempted.idempotency_key {
        mismatched_fields.push("idempotency_key");
    }
    if existing.occurred_at_micros != attempted.occurred_at_micros {
        mismatched_fields.push("occurred_at");
    }
    if existing.retention_until_micros != attempted.retention_until_micros {
        mismatched_fields.push("retention_until");
    }

    if mismatched_fields.is_empty() {
        return Ok(());
    }
    Err(ContractError::Conflict(format!(
        "journal event already exists with different immutable fields: {}",
        mismatched_fields.join(", ")
    )))
}

pub(crate) fn journal_position_conflict() -> ContractError {
    ContractError::Conflict(
        "journal commit position is already occupied by a different event".into(),
    )
}

pub(crate) fn resolve_journal_event_id_replay(
    txn: &mut r2d2_postgres::postgres::Transaction<'_>,
    prefix: &str,
    envelope: &CommitEnvelope,
    action: &'static str,
) -> Result<(String, i64), ContractError> {
    let row = txn
        .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
        .map_err(|error| postgres_unavailable_db(action, error))?;
    let existing = JournalEventFingerprint::from_row(&row)?;
    let attempted = JournalEventFingerprint::from_envelope(prefix, envelope)?;
    ensure_journal_event_replay_matches(&existing, &attempted, envelope.event_id.as_str())?;
    Ok(existing.position())
}

pub(crate) fn journal_aggregate_seq(ordering_seq: u64) -> Result<i64, ContractError> {
    postgres_bigint_input(ordering_seq, "ordering sequence")?
        .checked_add(1)
        .ok_or_else(|| {
            ContractError::Invalid(
                "journal ordering sequence exceeds the PostgreSQL BIGINT range".into(),
            )
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

fn journal_replay_ordering_seq(aggregate_seq: i64) -> Result<u64, ContractError> {
    let ordering_seq = aggregate_seq.checked_sub(1).ok_or_else(|| {
        ContractError::Unavailable(
            "postgres journal returned an invalid aggregate_seq field".into(),
        )
    })?;
    postgres_bigint_output(ordering_seq, "aggregate_seq")
}

fn journal_partitions(prefix: &str, envelopes: &[CommitEnvelope]) -> BTreeSet<String> {
    envelopes
        .iter()
        .map(|envelope| compose_partition_key(prefix, envelope.ordering_key.as_str()))
        .collect()
}

fn lock_journal_partitions(
    txn: &mut r2d2_postgres::postgres::Transaction<'_>,
    prefix: &str,
    envelopes: &[CommitEnvelope],
    action: &'static str,
) -> Result<(), ContractError> {
    for partition_key in journal_partitions(prefix, envelopes) {
        txn.query_one(LOCK_JOURNAL_PARTITION_SQL, &[&partition_key])
            .map_err(|error| postgres_unavailable_db(action, error))?;
    }
    Ok(())
}

fn allocate_next_ordering_sequences(
    txn: &mut r2d2_postgres::postgres::Transaction<'_>,
    prefix: &str,
    envelopes: &mut [CommitEnvelope],
) -> Result<BTreeSet<String>, ContractError> {
    let mut next_by_partition = BTreeMap::new();
    for partition_key in journal_partitions(prefix, envelopes) {
        let row = txn
            .query_one(LOAD_MAX_AGGREGATE_SEQ_SQL, &[&partition_key])
            .map_err(|error| postgres_unavailable_db("journal aggregate sequence lookup", error))?;
        let current: i64 = postgres_row_get(&row, 0, "aggregate sequence lookup", "aggregate_seq")?;
        next_by_partition.insert(partition_key, current);
    }

    let mut existing_by_event = BTreeMap::new();
    for envelope in envelopes.iter() {
        let existing = txn
            .query_opt(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
            .map_err(|error| {
                postgres_unavailable_db("journal aggregate sequence replay lookup", error)
            })?;
        if let Some(row) = existing {
            existing_by_event.insert(
                envelope.event_id.clone(),
                JournalEventFingerprint::from_row(&row)?.aggregate_seq,
            );
        }
    }
    assign_ordering_sequences(
        prefix,
        envelopes,
        &mut next_by_partition,
        &existing_by_event,
    )?;
    Ok(existing_by_event.into_keys().collect())
}

fn assign_ordering_sequences(
    prefix: &str,
    envelopes: &mut [CommitEnvelope],
    next_by_partition: &mut BTreeMap<String, i64>,
    existing_by_event: &BTreeMap<String, i64>,
) -> Result<(), ContractError> {
    for envelope in envelopes {
        let partition_key = compose_partition_key(prefix, envelope.ordering_key.as_str());
        let aggregate_seq = if let Some(existing) = existing_by_event.get(&envelope.event_id) {
            *existing
        } else {
            let current = next_by_partition.get_mut(&partition_key).ok_or_else(|| {
                ContractError::Unavailable(
                    "journal aggregate sequence partition lock state is missing".into(),
                )
            })?;
            *current = current.checked_add(1).ok_or_else(|| {
                ContractError::Conflict("journal aggregate sequence is exhausted".into())
            })?;
            *current
        };
        let zero_based = aggregate_seq.checked_sub(1).ok_or_else(|| {
            ContractError::Unavailable(
                "journal aggregate sequence lookup returned a non-positive value".into(),
            )
        })?;
        envelope.ordering_seq = postgres_bigint_output(zero_based, "aggregate_seq")?;
    }
    Ok(())
}

fn append_one(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
) -> Result<CommitPosition, ContractError> {
    let mut client = postgres_pool_client(pool, "journal append")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("journal append begin", error))?;
    lock_journal_partitions(
        &mut txn,
        prefix,
        std::slice::from_ref(envelope),
        "journal append lock",
    )?;

    let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
    let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
    let payload_hash = sha256_hash(envelope.payload.as_bytes());
    let created_at = Utc::now();
    // `commit_offset` and `aggregate_seq` must be > 0 (CHECK constraints on
    // `im_commit_journal`). `ordering_seq` is 0-indexed by the runtime (created
    // event = 0, first member = 1, ...), so we map it to a 1-indexed position
    // via `ordering_seq + 1`. Using `ordering_seq.max(1)` instead would map
    // both ordering_seq=0 and ordering_seq=1 to commit_offset=1, causing a
    // PK collision between the created event and the first member_joined event.
    let aggregate_seq = journal_aggregate_seq(envelope.ordering_seq)?;
    let commit_offset = aggregate_seq;
    let organization_id = envelope.normalized_organization_id();
    let occurred_at = postgres_timestamptz(envelope.occurred_at.as_str(), "occurred_at")?;
    let retention_until = journal_retention_until(envelope)
        .as_deref()
        .map(|value| postgres_timestamptz(value, "retention_until"))
        .transpose()?;

    // Wrap the INSERT in a SAVEPOINT: a `(partition_key, commit_offset)`
    // primary-key collision raises SQLSTATE 23505 and aborts the transaction.
    // Rolling back to the savepoint restores a usable transaction so we can
    // inspect the occupying row and either absorb an idempotent replay (same
    // `event_id`) or surface a genuine `Conflict` (different `event_id` claims
    // the position).
    let outcome = {
        let mut savepoint = txn
            .savepoint("im_journal_append")
            .map_err(|error| postgres_unavailable_db("journal append savepoint", error))?;
        let result = savepoint.query_opt(
            APPEND_EVENT_SQL,
            &[
                &partition_key,
                &commit_offset,
                &envelope.event_id,
                &envelope.tenant_id,
                &organization_id,
                &envelope.aggregate_type.as_wire_value(),
                &envelope.aggregate_id,
                &aggregate_seq,
                &envelope.event_type,
                &payload_json,
                &payload_hash,
                &envelope.idempotency_key,
                &occurred_at,
                &created_at,
                &retention_until,
            ],
        );
        match result {
            Ok(row) => {
                // Release the savepoint; the transaction remains usable.
                savepoint.commit().map_err(|error| {
                    postgres_unavailable_db("journal append savepoint commit", error)
                })?;
                match row {
                    Some(row) => InsertOutcome::Inserted(row),
                    None => InsertOutcome::EventIdAbsorbed,
                }
            }
            Err(error) if is_unique_violation(&error) => {
                savepoint.rollback().map_err(|error| {
                    postgres_unavailable_db("journal append savepoint rollback", error)
                })?;
                InsertOutcome::PositionCollision
            }
            Err(error) => {
                return Err(postgres_unavailable_db("journal append insert", error));
            }
        }
    };

    let (final_partition, final_offset) = match outcome {
        InsertOutcome::Inserted(row) => {
            let partition: String = postgres_row_get(&row, 0, "append", "partition_key")?;
            let offset: i64 = postgres_row_get(&row, 1, "append", "commit_offset")?;
            (partition, postgres_bigint_output(offset, "commit_offset")?)
        }
        // ON CONFLICT (event_id) absorbed the row: read the previously
        // committed position by event_id. This path is the idempotent replay
        // of the exact same producer event.
        InsertOutcome::EventIdAbsorbed => {
            let (partition, offset) = resolve_journal_event_id_replay(
                &mut txn,
                prefix,
                envelope,
                "journal append conflict lookup",
            )?;
            (partition, postgres_bigint_output(offset, "commit_offset")?)
        }
        // PK (partition_key, commit_offset) violated with a different
        // event_id. Look up the occupying row by position: if it carries the
        // same event_id, treat as idempotent (defensive — ON CONFLICT should
        // have caught it); otherwise surface a Conflict so callers map it to
        // HTTP 409 instead of an opaque 503.
        InsertOutcome::PositionCollision => {
            let row = txn
                .query_one(
                    LOAD_EVENT_BY_POSITION_SQL,
                    &[&partition_key, &commit_offset],
                )
                .map_err(|error| {
                    postgres_unavailable_db("journal append position lookup", error)
                })?;
            let existing_event_id: String =
                postgres_row_get(&row, 0, "position lookup", "event_id")?;
            if existing_event_id == envelope.event_id {
                let (partition, offset) = resolve_journal_event_id_replay(
                    &mut txn,
                    prefix,
                    envelope,
                    "journal append defensive replay lookup",
                )?;
                (partition, postgres_bigint_output(offset, "commit_offset")?)
            } else {
                return Err(journal_position_conflict());
            }
        }
    };

    txn.commit()
        .map_err(|error| postgres_unavailable_db("journal append commit", error))?;

    Ok(CommitPosition::new(final_partition, final_offset))
}

fn append_many(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelopes: Vec<CommitEnvelope>,
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "journal append_batch")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("journal append_batch begin", error))?;
    lock_journal_partitions(&mut txn, prefix, &envelopes, "journal append_batch lock")?;
    let positions = append_many_on_transaction(&mut txn, prefix, &envelopes)?;
    txn.commit()
        .map_err(|error| postgres_unavailable("journal append_batch commit", error))?;
    Ok(positions)
}

fn append_many_on_transaction(
    txn: &mut r2d2_postgres::postgres::Transaction<'_>,
    prefix: &str,
    envelopes: &[CommitEnvelope],
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut positions = Vec::with_capacity(envelopes.len());
    for envelope in envelopes {
        let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
        let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
        let payload_hash = sha256_hash(envelope.payload.as_bytes());
        let created_at = Utc::now();
        // Map 0-indexed `ordering_seq` to 1-indexed `commit_offset`/`aggregate_seq`.
        // See `append_one` for why `.max(1)` would collide ordering_seq=0 and =1.
        let aggregate_seq = journal_aggregate_seq(envelope.ordering_seq)?;
        let commit_offset = aggregate_seq;
        let organization_id = envelope.normalized_organization_id();
        let occurred_at = postgres_timestamptz(envelope.occurred_at.as_str(), "occurred_at")?;
        let retention_until = journal_retention_until(envelope)
            .as_deref()
            .map(|value| postgres_timestamptz(value, "retention_until"))
            .transpose()?;

        let outcome = {
            let mut savepoint = txn
                .savepoint("im_journal_append_batch")
                .map_err(|error| postgres_unavailable("journal append_batch savepoint", error))?;
            let result = savepoint.query_opt(
                APPEND_EVENT_SQL,
                &[
                    &partition_key,
                    &commit_offset,
                    &envelope.event_id,
                    &envelope.tenant_id,
                    &organization_id,
                    &envelope.aggregate_type.as_wire_value(),
                    &envelope.aggregate_id,
                    &aggregate_seq,
                    &envelope.event_type,
                    &payload_json,
                    &payload_hash,
                    &envelope.idempotency_key,
                    &occurred_at,
                    &created_at,
                    &retention_until,
                ],
            );
            match result {
                Ok(row) => {
                    savepoint.commit().map_err(|error| {
                        postgres_unavailable("journal append_batch savepoint commit", error)
                    })?;
                    match row {
                        Some(row) => InsertOutcome::Inserted(row),
                        None => InsertOutcome::EventIdAbsorbed,
                    }
                }
                Err(error) if is_unique_violation(&error) => {
                    savepoint.rollback().map_err(|error| {
                        postgres_unavailable("journal append_batch savepoint rollback", error)
                    })?;
                    InsertOutcome::PositionCollision
                }
                Err(error) => {
                    return Err(postgres_unavailable("journal append_batch insert", error));
                }
            }
        };

        let (final_partition, final_offset) = match outcome {
            InsertOutcome::Inserted(row) => {
                let partition: String = postgres_row_get(&row, 0, "append_batch", "partition_key")?;
                let offset: i64 = postgres_row_get(&row, 1, "append_batch", "commit_offset")?;
                (partition, postgres_bigint_output(offset, "commit_offset")?)
            }
            InsertOutcome::EventIdAbsorbed => {
                let (partition, offset) = resolve_journal_event_id_replay(
                    txn,
                    prefix,
                    envelope,
                    "journal append_batch conflict lookup",
                )?;
                (partition, postgres_bigint_output(offset, "commit_offset")?)
            }
            InsertOutcome::PositionCollision => {
                let row = txn
                    .query_one(
                        LOAD_EVENT_BY_POSITION_SQL,
                        &[&partition_key, &commit_offset],
                    )
                    .map_err(|error| {
                        postgres_unavailable("journal append_batch position lookup", error)
                    })?;
                let existing_event_id: String =
                    postgres_row_get(&row, 0, "position lookup", "event_id")?;
                if existing_event_id == envelope.event_id {
                    let (partition, offset) = resolve_journal_event_id_replay(
                        txn,
                        prefix,
                        envelope,
                        "journal append_batch defensive replay lookup",
                    )?;
                    (partition, postgres_bigint_output(offset, "commit_offset")?)
                } else {
                    return Err(journal_position_conflict());
                }
            }
        };

        positions.push(CommitPosition::new(final_partition, final_offset));
    }

    Ok(positions)
}

/// Reconstruct committed envelopes in append order using an explicit page bound.
///
/// Only the columns needed to rehydrate a [`CommitEnvelope`] are selected;
/// the full event payload is round-tripped as text to preserve its original
/// JSON encoding. Envelopes reconstructed here are best-effort projections —
/// callers that need the authoritative aggregate state should replay through
/// the domain layer rather than consume this projection directly.
fn load_recorded_page(
    pool: &PostgresJournalPool,
    prefix: &str,
    cursor: Option<&JournalReplayCursor>,
    limit: i64,
) -> Result<(Vec<CommitEnvelope>, Option<JournalReplayCursor>), ContractError> {
    let mut client = postgres_pool_client(pool, "journal recorded")?;
    let pattern = format!("{prefix}%");
    let rows = if let Some(cursor) = cursor {
        let commit_offset = postgres_bigint_input(cursor.commit_offset, "replay cursor")?;
        client
            .query(
                LOAD_RECORDED_AFTER_SQL,
                &[&pattern, &cursor.partition_key, &commit_offset, &limit],
            )
            .map_err(|error| postgres_unavailable("journal recorded after select", error))?
    } else {
        client
            .query(LOAD_RECORDED_SQL, &[&pattern, &limit])
            .map_err(|error| postgres_unavailable("journal recorded select", error))?
    };

    parse_journal_replay_rows(rows, prefix, cursor)
}

fn load_recorded_page_for_aggregate(
    pool: &PostgresJournalPool,
    prefix: &str,
    scope: &CommitJournalAggregateScope,
    cursor: Option<&JournalReplayCursor>,
    limit: i64,
) -> Result<(Vec<CommitEnvelope>, Option<JournalReplayCursor>), ContractError> {
    let mut client = postgres_pool_client(pool, "journal recorded aggregate")?;
    let pattern = format!("{prefix}%");
    let rows = if let Some(cursor) = cursor {
        let commit_offset = postgres_bigint_input(cursor.commit_offset, "replay cursor")?;
        client
            .query(
                LOAD_RECORDED_AGGREGATE_AFTER_SQL,
                &[
                    &pattern,
                    &scope.tenant_id,
                    &scope.aggregate_id,
                    &commit_offset,
                    &limit,
                ],
            )
            .map_err(|error| {
                postgres_unavailable("journal aggregate recorded after select", error)
            })?
    } else {
        client
            .query(
                LOAD_RECORDED_AGGREGATE_SQL,
                &[&pattern, &scope.tenant_id, &scope.aggregate_id, &limit],
            )
            .map_err(|error| postgres_unavailable("journal aggregate recorded select", error))?
    };
    parse_journal_replay_rows(rows, prefix, None)
}

fn load_recorded_page_for_aggregate_event_types(
    pool: &PostgresJournalPool,
    prefix: &str,
    query: &CommitJournalAggregateEventTypeQuery,
    cursor: Option<&JournalReplayCursor>,
    limit: i64,
) -> Result<(Vec<CommitEnvelope>, Option<JournalReplayCursor>), ContractError> {
    let mut client = postgres_pool_client(pool, "journal aggregate event-type replay")?;
    let pattern = format!("{prefix}%");
    let rows = if let Some(cursor) = cursor {
        let commit_offset = postgres_bigint_input(cursor.commit_offset, "replay cursor")?;
        client
            .query(
                LOAD_RECORDED_AGGREGATE_EVENT_TYPES_AFTER_SQL,
                &[
                    &pattern,
                    &query.tenant_id,
                    &query.organization_id,
                    &query.aggregate_type,
                    &query.aggregate_id,
                    &query.event_types,
                    &commit_offset,
                    &limit,
                ],
            )
            .map_err(|error| {
                postgres_unavailable("journal aggregate event-type replay after select", error)
            })?
    } else {
        client
            .query(
                LOAD_RECORDED_AGGREGATE_EVENT_TYPES_SQL,
                &[
                    &pattern,
                    &query.tenant_id,
                    &query.organization_id,
                    &query.aggregate_type,
                    &query.aggregate_id,
                    &query.event_types,
                    &limit,
                ],
            )
            .map_err(|error| {
                postgres_unavailable("journal aggregate event-type replay select", error)
            })?
    };
    parse_journal_replay_rows(rows, prefix, cursor)
}

fn validate_aggregate_event_type_query(
    query: &CommitJournalAggregateEventTypeQuery,
) -> Result<(), ContractError> {
    if query.tenant_id.trim().is_empty()
        || query.organization_id.trim().is_empty()
        || query.aggregate_type.trim().is_empty()
        || query.aggregate_id.trim().is_empty()
        || query.event_types.is_empty()
        || query
            .event_types
            .iter()
            .any(|event_type| event_type.trim().is_empty())
    {
        return Err(ContractError::Invalid(
            "journal aggregate event-type query contains an empty scope or event type".into(),
        ));
    }
    Ok(())
}

#[derive(Default)]
struct ReplayEnvelopeMetadata {
    event_version: u16,
    payload_schema: Option<String>,
    causation_id: Option<String>,
    correlation_id: Option<String>,
    actor: Option<EventActor>,
}

fn replay_envelope_metadata(
    event_type: &str,
    payload: &str,
) -> Result<ReplayEnvelopeMetadata, ContractError> {
    let mut metadata = ReplayEnvelopeMetadata {
        event_version: 1,
        ..ReplayEnvelopeMetadata::default()
    };
    let needs_payload = matches!(
        event_type,
        "conversation.created"
            | "conversation.agents_replaced"
            | "message.posted"
            | AGENT_MENTION_DISPATCH_EVENT_TYPE
    );
    if !needs_payload {
        return Ok(metadata);
    }
    let value: serde_json::Value = serde_json::from_str(payload).map_err(|_| {
        ContractError::Unavailable("postgres journal replay payload is invalid JSON".into())
    })?;
    metadata.payload_schema = Some(match event_type {
        "conversation.created" => {
            let assignments = value.get("agentAssignments");
            let source = assignments
                .and_then(|value| value.get("source"))
                .and_then(serde_json::Value::as_str);
            match source {
                Some("conversation_override") => {
                    metadata.event_version = 3;
                    "conversation.created.v3"
                }
                Some("default_policy") => {
                    metadata.event_version = 2;
                    "conversation.created.v2"
                }
                None if assignments.is_some() => {
                    metadata.event_version = 2;
                    "conversation.created.v2"
                }
                None => "conversation.created.v1",
                Some(_) => {
                    return Err(ContractError::Conflict(
                        "postgres journal replay contains an unknown conversation agent assignment source"
                            .into(),
                    ));
                }
            }
        }
        "conversation.agents_replaced" => "conversation.agents_replaced.v1",
        "message.posted" => "message.posted.v1",
        AGENT_MENTION_DISPATCH_EVENT_TYPE => AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA,
        _ => unreachable!("payload parsing is gated by needs_payload"),
    }
    .into());

    match event_type {
        AGENT_MENTION_DISPATCH_EVENT_TYPE => {
            metadata.causation_id = value
                .get("causationEventId")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            metadata.correlation_id = metadata.causation_id.clone();
            metadata.actor = Some(EventActor {
                actor_id: value
                    .get("senderPrincipalId")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                actor_kind: value
                    .get("senderPrincipalKind")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                actor_session_id: None,
            });
        }
        "message.posted" => {
            if let Some(sender) = value.get("sender") {
                metadata.actor = Some(EventActor {
                    actor_id: sender
                        .get("id")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default()
                        .to_owned(),
                    actor_kind: sender
                        .get("kind")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default()
                        .to_owned(),
                    actor_session_id: sender
                        .get("sessionId")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned),
                });
            }
        }
        _ => {}
    }
    Ok(metadata)
}

fn parse_journal_replay_rows(
    rows: Vec<postgres::Row>,
    prefix: &str,
    cursor: Option<&JournalReplayCursor>,
) -> Result<(Vec<CommitEnvelope>, Option<JournalReplayCursor>), ContractError> {
    let mut envelopes = Vec::with_capacity(rows.len());
    let mut next_cursor = cursor.cloned();
    for row in rows {
        let event_id: String = journal_replay_row_get(&row, 0, "event_id")?;
        let tenant_id: String = journal_replay_row_get(&row, 1, "tenant_id")?;
        let organization_id: String = journal_replay_row_get(&row, 2, "organization_id")?;
        let event_type: String = journal_replay_row_get(&row, 3, "event_type")?;
        let aggregate_type_str: String = journal_replay_row_get(&row, 4, "aggregate_type")?;
        let aggregate_id: String = journal_replay_row_get(&row, 5, "aggregate_id")?;
        let aggregate_seq: i64 = journal_replay_row_get(&row, 6, "aggregate_seq")?;
        let occurred_at: String = journal_replay_row_get(&row, 7, "occurred_at")?;
        let payload: String = journal_replay_row_get(&row, 8, "payload_json")?;
        let idempotency_key: Option<String> = journal_replay_row_get(&row, 9, "idempotency_key")?;
        let partition_key: String = journal_replay_row_get(&row, 10, "partition_key")?;
        let commit_offset: i64 = journal_replay_row_get(&row, 11, "commit_offset")?;
        let aggregate_type = parse_aggregate_type(aggregate_type_str.as_str());
        let replay_metadata = replay_envelope_metadata(event_type.as_str(), payload.as_str())?;
        let replay_scope = replay_scope_for_journal_row(
            &aggregate_type,
            tenant_id.as_str(),
            aggregate_id.as_str(),
            partition_key.as_str(),
            prefix,
        );
        let ordering_seq = journal_replay_ordering_seq(aggregate_seq)?;
        let commit_offset = postgres_bigint_output(commit_offset, "commit_offset")?;

        envelopes.push(CommitEnvelope {
            event_id,
            tenant_id,
            organization_id: im_domain_events::normalize_commit_organization_id(
                organization_id.as_str(),
            ),
            event_type,
            event_version: replay_metadata.event_version,
            aggregate_type,
            aggregate_id: aggregate_id.clone(),
            scope_type: replay_scope.scope_type,
            scope_id: replay_scope.scope_id,
            ordering_key: replay_scope.ordering_key,
            ordering_seq,
            causation_id: replay_metadata.causation_id,
            correlation_id: replay_metadata.correlation_id,
            idempotency_key,
            actor: replay_metadata.actor.unwrap_or(EventActor {
                actor_id: String::new(),
                actor_kind: String::new(),
                actor_session_id: None,
            }),
            occurred_at: occurred_at.clone(),
            committed_at: occurred_at,
            payload_schema: replay_metadata.payload_schema,
            payload,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        });
        next_cursor = Some(JournalReplayCursor {
            partition_key,
            commit_offset,
        });
    }
    Ok((envelopes, next_cursor))
}

struct ReplayJournalScope {
    scope_type: String,
    scope_id: String,
    ordering_key: String,
}

fn replay_scope_for_journal_row(
    aggregate_type: &AggregateType,
    tenant_id: &str,
    aggregate_id: &str,
    partition_key: &str,
    partition_prefix: &str,
) -> ReplayJournalScope {
    let ordering_key = replay_ordering_key_from_partition(
        tenant_id,
        aggregate_id,
        partition_key,
        partition_prefix,
    );
    let (scope_type, scope_id) = match aggregate_type {
        AggregateType::Conversation => ("conversation".to_owned(), aggregate_id.to_owned()),
        AggregateType::DirectChat => ("direct_chat".to_owned(), aggregate_id.to_owned()),
        AggregateType::Friendship => ("friendship".to_owned(), aggregate_id.to_owned()),
        AggregateType::FriendRequest => ("friend_request".to_owned(), aggregate_id.to_owned()),
        _ => (
            aggregate_type.as_wire_value().to_owned(),
            aggregate_id.to_owned(),
        ),
    };
    ReplayJournalScope {
        scope_type,
        scope_id,
        ordering_key,
    }
}

fn replay_ordering_key_from_partition(
    tenant_id: &str,
    aggregate_id: &str,
    partition_key: &str,
    partition_prefix: &str,
) -> String {
    if partition_prefix.is_empty() {
        if !partition_key.is_empty() {
            return partition_key.to_owned();
        }
    } else if let Some(stripped) = partition_key.strip_prefix(partition_prefix) {
        let ordering_key = stripped.strip_prefix(':').unwrap_or(stripped);
        if !ordering_key.is_empty() {
            return ordering_key.to_owned();
        }
    }
    CommitEnvelope::ordering_key(tenant_id, aggregate_id)
}

/// Bridge a blocking PostgreSQL operation off the async runtime.
///
/// When called from a multi-threaded Tokio runtime worker thread, uses
/// [`tokio::task::block_in_place`] to move other tasks on the current worker
/// to other workers, preventing async runtime starvation under concurrent
/// load. Falls back to [`std::thread::scope`] for non-Tokio contexts and
/// current-thread runtimes (e.g., `#[tokio::test]`).
pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current()
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

fn resolve_im_postgres_search_path_schema() -> Option<String> {
    let schema = sdkwork_database_config::claw_database::resolve_unified_postgres_schema("IM");
    (schema != "public").then_some(schema)
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
    if let Some(schema) = resolve_im_postgres_search_path_schema() {
        apply_postgres_search_path(&mut client, schema.as_str())?;
    }
    Ok(client)
}

pub(crate) fn compose_partition_key(prefix: &str, ordering_key: &str) -> String {
    if prefix.is_empty() {
        ordering_key.to_string()
    } else {
        format!("{prefix}:{ordering_key}")
    }
}

pub(crate) fn now_rfc3339() -> String {
    im_time::utc_now_rfc3339_millis()
}
pub(crate) fn postgres_unavailable(
    action: &'static str,
    _error: impl std::fmt::Display,
) -> ContractError {
    ContractError::Unavailable(format!("postgres journal {action} failed"))
}

pub(crate) fn postgres_unavailable_db(
    action: &'static str,
    _error: r2d2_postgres::postgres::Error,
) -> ContractError {
    ContractError::Unavailable(format!("postgres journal {action} failed"))
}

/// Returns true when the postgres error is a unique constraint violation
/// (SQLSTATE 23505). Used to absorb `(partition_key, commit_offset)` primary
/// key collisions in `append`/`append_batch` so that replayed producer events
/// stay idempotent alongside the existing `ON CONFLICT (event_id) DO NOTHING`
/// path.
pub(crate) fn is_unique_violation(error: &r2d2_postgres::postgres::Error) -> bool {
    error.as_db_error().map(|db_error| db_error.code())
        == Some(&r2d2_postgres::postgres::error::SqlState::UNIQUE_VIOLATION)
}

/// Best-effort mapping from the stored aggregate-type string back to the enum.
///
/// Unknown values fall back to a neutral variant rather than erroring, so a
/// forward-incompatible row never blocks journal replay. The authoritative
/// enum is `im_domain_events::AggregateType`.
fn parse_aggregate_type(value: &str) -> AggregateType {
    match value {
        "conversation" => AggregateType::Conversation,
        "space" => AggregateType::Space,
        "chat_group" => AggregateType::ChatGroup,
        "friend_request" => AggregateType::FriendRequest,
        "friendship" => AggregateType::Friendship,
        "external_connection" => AggregateType::ExternalConnection,
        "external_member_link" => AggregateType::ExternalMemberLink,
        "shared_channel_policy" => AggregateType::SharedChannelPolicy,
        "stream_session" => AggregateType::StreamSession,
        "rtc_session" => AggregateType::RtcSession,
        "tenant_policy" => AggregateType::TenantPolicy,
        "direct_chat" => AggregateType::DirectChat,
        "notification" => AggregateType::Notification,
        "automation_execution" => AggregateType::AutomationExecution,
        "user_block" => AggregateType::UserBlock,
        _ => AggregateType::Conversation,
    }
}

// ---------------------------------------------------------------------------
// Error formatting helpers — never emit raw connection URLs (which carry
// credentials) into logs or error surfaces.
// ---------------------------------------------------------------------------

fn postgres_config_error(
    _database_url: &str,
    _error: r2d2_postgres::postgres::Error,
) -> ContractError {
    ContractError::Unavailable("postgres journal database URL is invalid".into())
}

pub(crate) fn journal_retention_until(envelope: &CommitEnvelope) -> Option<String> {
    retention_until_from_envelope(
        envelope.retention_class.as_str(),
        envelope.occurred_at.as_str(),
    )
}

/// Build a conversation member access gate backed by projection membership rows.
pub fn conversation_member_access_gate_from_pool(
    pool: PostgresJournalPool,
) -> std::sync::Arc<dyn im_platform_contracts::ConversationMemberAccessGate> {
    use im_platform_contracts::AggregateStoreConversationMemberAccessGate;
    std::sync::Arc::new(AggregateStoreConversationMemberAccessGate::new(
        std::sync::Arc::new(PostgresAggregateStore::from_pool(pool)),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use im_domain_events::{AggregateType, CommitEnvelope};
    use im_platform_contracts::ContractError;

    use super::{
        AGENT_MENTION_DISPATCH_EVENT_TYPE, AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA,
        JournalEventFingerprint, assign_ordering_sequences, compose_partition_key,
        ensure_journal_event_replay_matches, journal_aggregate_seq, journal_position_conflict,
        journal_replay_ordering_seq, postgres_bigint_input, postgres_bigint_output,
        postgres_jsonb_payload, postgres_timestamptz, postgres_unavailable,
        replay_envelope_metadata,
    };

    fn replay_envelope() -> CommitEnvelope {
        CommitEnvelope::minimal(
            "evt_replay_fingerprint",
            "tenant_a",
            "message.posted",
            "conversation",
            "conversation_a",
            7,
        )
        .with_organization_id("organization_a")
        .with_payload(
            "message.posted.v1",
            r#"{"conversationId":"conversation_a","messageId":"101"}"#,
        )
    }

    #[test]
    fn journal_event_fingerprint_accepts_an_exact_envelope_clone() {
        let envelope = replay_envelope();
        let existing = JournalEventFingerprint::from_envelope("journal", &envelope)
            .expect("fixture envelope must be valid");
        let replay = JournalEventFingerprint::from_envelope("journal", &envelope.clone())
            .expect("fixture replay must be valid");

        assert_eq!(
            ensure_journal_event_replay_matches(&existing, &replay, envelope.event_id.as_str()),
            Ok(())
        );
    }

    #[test]
    fn journal_event_fingerprint_matches_postgres_microsecond_precision() {
        let mut original = replay_envelope();
        original.occurred_at = "2026-07-11T01:02:03.123456789Z".into();
        let mut postgres_round_trip = original.clone();
        postgres_round_trip.occurred_at = "2026-07-11T01:02:03.123456000Z".into();
        let existing = JournalEventFingerprint::from_envelope("journal", &original)
            .expect("fixture envelope must be valid");
        let replay = JournalEventFingerprint::from_envelope("journal", &postgres_round_trip)
            .expect("PostgreSQL precision fixture must be valid");

        assert_eq!(
            ensure_journal_event_replay_matches(&existing, &replay, original.event_id.as_str(),),
            Ok(())
        );
    }

    #[test]
    fn journal_event_fingerprint_rejects_each_persisted_identity_change() {
        let envelope = replay_envelope();
        let existing = JournalEventFingerprint::from_envelope("journal", &envelope)
            .expect("fixture envelope must be valid");
        let mut changed_envelopes = Vec::new();

        let mut changed = envelope.clone();
        changed.tenant_id = "tenant_b".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.organization_id = "organization_b".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.ordering_key = "different-ordering-key".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.ordering_seq += 1;
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.aggregate_type = AggregateType::DirectChat;
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.aggregate_id = "conversation_b".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.event_type = "message.edited".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.payload = r#"{"conversationId":"conversation_a","messageId":"102"}"#.into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.idempotency_key = Some("different-idempotency-key".into());
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.occurred_at = "2026-07-11T01:02:03Z".into();
        changed_envelopes.push(changed);

        let mut changed = envelope.clone();
        changed.retention_class = "ephemeral".into();
        changed_envelopes.push(changed);

        for changed in changed_envelopes {
            let attempted = JournalEventFingerprint::from_envelope("journal", &changed)
                .expect("changed fixture must remain structurally valid");
            assert!(matches!(
                ensure_journal_event_replay_matches(
                    &existing,
                    &attempted,
                    envelope.event_id.as_str()
                ),
                Err(ContractError::Conflict(_))
            ));
        }
    }

    #[test]
    fn journal_aggregate_seq_rejects_values_outside_postgres_bigint() {
        assert_eq!(journal_aggregate_seq(0), Ok(1));
        assert_eq!(journal_aggregate_seq((i64::MAX - 1) as u64), Ok(i64::MAX));
        assert!(matches!(
            journal_aggregate_seq(i64::MAX as u64),
            Err(ContractError::Invalid(_))
        ));
        assert!(matches!(
            journal_aggregate_seq(u64::MAX),
            Err(ContractError::Invalid(_))
        ));
    }

    #[test]
    fn postgres_bigint_conversions_reject_aliasing_and_corrupt_rows() {
        assert_eq!(postgres_bigint_input(0, "cursor"), Ok(0));
        assert_eq!(
            postgres_bigint_input(i64::MAX as u64, "cursor"),
            Ok(i64::MAX)
        );
        assert!(matches!(
            postgres_bigint_input(i64::MAX as u64 + 1, "cursor"),
            Err(ContractError::Invalid(_))
        ));

        assert_eq!(postgres_bigint_output(1, "commit_offset"), Ok(1));
        assert!(matches!(
            postgres_bigint_output(-1, "commit_offset"),
            Err(ContractError::Unavailable(_))
        ));
        assert_eq!(journal_replay_ordering_seq(1), Ok(0));
        assert!(matches!(
            journal_replay_ordering_seq(0),
            Err(ContractError::Unavailable(_))
        ));
    }

    #[test]
    fn postgres_unavailable_does_not_expose_driver_or_database_details() {
        let error = postgres_unavailable(
            "append",
            "duplicate key contains tenant_secret partition_secret event_secret",
        );
        let ContractError::Unavailable(message) = error else {
            panic!("postgres failures must remain unavailable");
        };

        assert!(message.contains("append"));
        assert!(!message.contains("tenant_secret"));
        assert!(!message.contains("partition_secret"));
        assert!(!message.contains("event_secret"));
        assert!(!message.contains("duplicate key"));
    }

    #[test]
    fn invalid_journal_payloads_and_timestamps_are_not_reported_as_conflicts() {
        assert!(matches!(
            postgres_jsonb_payload("not-json"),
            Err(ContractError::Invalid(_))
        ));
        assert!(matches!(
            postgres_timestamptz("not-a-timestamp", "occurred_at"),
            Err(ContractError::Invalid(_))
        ));
    }

    #[test]
    fn replay_metadata_restores_agent_event_versions_and_dispatch_identity() {
        let v3 = replay_envelope_metadata(
            "conversation.created",
            r#"{
                "conversationType":"group",
                "agentAssignments":{
                    "generation":1,
                    "source":"conversation_override",
                    "agents":[{"agentId":"agent.im.writer"}]
                }
            }"#,
        )
        .expect("v3 payload should infer replay metadata");
        assert_eq!(v3.event_version, 3);
        assert_eq!(
            v3.payload_schema.as_deref(),
            Some("conversation.created.v3")
        );

        let v2 = replay_envelope_metadata(
            "conversation.created",
            r#"{
                "conversationType":"group",
                "agentAssignments":{
                    "generation":1,
                    "source":"default_policy",
                    "agents":[{"agentId":"agent.im.default"}],
                    "policyId":"policy.im.group.default",
                    "policyVersion":1
                }
            }"#,
        )
        .expect("v2 payload should infer replay metadata");
        assert_eq!(v2.event_version, 2);
        assert_eq!(
            v2.payload_schema.as_deref(),
            Some("conversation.created.v2")
        );

        let dispatch = replay_envelope_metadata(
            AGENT_MENTION_DISPATCH_EVENT_TYPE,
            r#"{
                "causationEventId":"evt_message_posted",
                "senderPrincipalId":"user_1",
                "senderPrincipalKind":"user"
            }"#,
        )
        .expect("dispatch payload should infer replay metadata");
        assert_eq!(dispatch.event_version, 1);
        assert_eq!(
            dispatch.payload_schema.as_deref(),
            Some(AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA)
        );
        assert_eq!(dispatch.causation_id.as_deref(), Some("evt_message_posted"));
        assert_eq!(
            dispatch.actor.as_ref().map(|actor| actor.actor_id.as_str()),
            Some("user_1")
        );
        assert_eq!(
            dispatch
                .actor
                .as_ref()
                .map(|actor| actor.actor_kind.as_str()),
            Some("user")
        );
    }

    #[test]
    fn replay_metadata_rejects_unknown_created_assignment_source() {
        assert!(matches!(
            replay_envelope_metadata(
                "conversation.created",
                r#"{
                    "conversationType":"group",
                    "agentAssignments":{
                        "generation":1,
                        "source":"unknown_policy",
                        "agents":[]
                    }
                }"#
            ),
            Err(ContractError::Conflict(_))
        ));
    }

    #[test]
    fn journal_conflicts_do_not_expose_event_or_partition_identifiers() {
        let envelope = replay_envelope();
        let existing = JournalEventFingerprint::from_envelope("journal", &envelope)
            .expect("fixture envelope must be valid");
        let mut changed = envelope.clone();
        changed.tenant_id = "tenant_sensitive".into();
        changed.ordering_key = "partition_sensitive".into();
        let attempted = JournalEventFingerprint::from_envelope("journal", &changed)
            .expect("changed fixture must remain structurally valid");

        let ContractError::Conflict(replay_message) =
            ensure_journal_event_replay_matches(&existing, &attempted, "event_sensitive")
                .expect_err("changed replay must conflict")
        else {
            panic!("changed replay must remain a conflict");
        };
        let ContractError::Conflict(position_message) = journal_position_conflict() else {
            panic!("occupied position must remain a conflict");
        };

        for message in [replay_message, position_message] {
            assert!(!message.contains("tenant_sensitive"));
            assert!(!message.contains("partition_sensitive"));
            assert!(!message.contains("event_sensitive"));
            assert!(!message.contains("evt_replay_fingerprint"));
        }
    }

    #[test]
    fn coordinated_sequence_assignment_is_contiguous_and_replay_stable() {
        let mut envelopes = vec![
            CommitEnvelope::minimal("evt-existing", "tenant-a", "space.updated", "space", "1", 9),
            CommitEnvelope::minimal("evt-new-a", "tenant-a", "space.updated", "space", "1", 9),
            CommitEnvelope::minimal("evt-new-b", "tenant-a", "group.updated", "group", "2", 9),
        ];
        let partition_a = compose_partition_key("space", &envelopes[0].ordering_key);
        let partition_b = compose_partition_key("space", &envelopes[2].ordering_key);
        let mut current = BTreeMap::from([(partition_a, 2_i64), (partition_b, 0_i64)]);
        let existing = BTreeMap::from([("evt-existing".to_owned(), 2_i64)]);

        assign_ordering_sequences("space", &mut envelopes, &mut current, &existing)
            .expect("sequence allocation should succeed");

        assert_eq!(
            envelopes[0].ordering_seq, 1,
            "replay keeps its stored sequence"
        );
        assert_eq!(
            envelopes[1].ordering_seq, 2,
            "next event receives sequence 3"
        );
        assert_eq!(
            envelopes[2].ordering_seq, 0,
            "new aggregate starts at sequence 1"
        );
    }

    #[test]
    fn coordinated_sequence_assignment_rejects_exhausted_aggregate() {
        let mut envelopes = vec![CommitEnvelope::minimal(
            "evt-overflow",
            "tenant-a",
            "space.updated",
            "space",
            "1",
            0,
        )];
        let partition = compose_partition_key("space", &envelopes[0].ordering_key);
        let mut current = BTreeMap::from([(partition, i64::MAX)]);

        assert!(matches!(
            assign_ordering_sequences("space", &mut envelopes, &mut current, &BTreeMap::new()),
            Err(ContractError::Conflict(_))
        ));
    }
}
