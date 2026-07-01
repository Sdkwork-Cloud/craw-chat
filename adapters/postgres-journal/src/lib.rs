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

use std::sync::Arc;

use chrono::{DateTime, Utc};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_domain_core::retention::retention_until_from_envelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_utils_rust::sha256_hash;
use tokio::runtime::Handle;

mod aggregate_store;
mod message_store;
mod outbox_store;
mod retention_cleanup;
mod retention_metrics;
mod retention_reconcile;
mod retention_scheduler;
mod search_store;

pub use aggregate_store::PostgresAggregateStore;
pub use message_store::PostgresMessageStore;
pub use outbox_store::PostgresOutboxStore;
pub use retention_cleanup::{purge_expired_retention_batch, RetentionCleanupReport};
pub use retention_metrics::{retention_purge_metrics, RetentionPurgeMetrics};
pub use retention_reconcile::{
    clear_conversation_retention_until, PostgresRetentionScopeStore, RetentionReconcileReport,
};
pub use retention_scheduler::{
    spawn_retention_purge_scheduler, spawn_retention_purge_scheduler_from_env,
    RetentionPurgeSchedulerConfig, RetentionPurgeSchedulerHandle,
};
pub use search_store::PostgresSearchProvider;

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
    /// Errors here are surfaced as [`ContractError::Unavailable`] with a
    /// redacted URL (see [`redact_postgres_url`]) so connection-string
    /// credentials never leak into logs.
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

fn build_journal_pool(config: &PostgresJournalConfig) -> Result<PostgresJournalPool, ContractError> {
    if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
        return Ok(PostgresJournalPool::from_pool(pool));
    }
    if cfg!(test) {
        return build_journal_pool_local(config);
    }
    Err(ContractError::Unavailable(
        sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
            .err()
            .unwrap_or_else(|| "IM process database pools are not installed".to_owned()),
    ))
}

fn build_journal_pool_local(
    config: &PostgresJournalConfig,
) -> Result<PostgresJournalPool, ContractError> {
    verify_production_sslmode(config.database_url.as_str());
    let pg_config = config
        .database_url
        .parse()
        .map_err(|error| postgres_config_error(config.database_url.as_str(), error))?;
    let tls = make_tls_connector().map_err(|error| {
        ContractError::Unavailable(format!(
            "postgres journal TLS connector build failed: {error}"
        ))
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

    pub fn with_partition_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.partition_prefix = Arc::from(prefix.into());
        self
    }

    pub fn pool(&self) -> &PostgresJournalPool {
        &self.pool
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
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || load_recorded(&pool, &prefix))
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
const APPEND_EVENT_SQL: &str = r#"
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

const LOAD_EVENT_BY_ID_SQL: &str = r#"
select partition_key, commit_offset
from im_commit_journal
where event_id = $1
"#;

/// Look up an existing journal row by the composite primary key
/// `(partition_key, commit_offset)`. Used after a SQLSTATE 23505 collision on
/// the primary key to determine whether the occupying row belongs to the same
/// `event_id` (defensive idempotent replay) or to a different `event_id`
/// (genuine position conflict that must surface as `Conflict`).
const LOAD_EVENT_BY_POSITION_SQL: &str = r#"
select event_id, partition_key, commit_offset
from im_commit_journal
where partition_key = $1 and commit_offset = $2
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
    occurred_at,
    payload_json::text,
    idempotency_key,
    partition_key
from im_commit_journal
where partition_key like $1 || '%'
order by partition_key asc, commit_offset asc
"#;

// ---------------------------------------------------------------------------
// Blocking I/O helpers (executed off the async runtime by `run_postgres_io`).
// ---------------------------------------------------------------------------

pub(crate) fn postgres_jsonb_payload(payload: &str) -> Result<serde_json::Value, ContractError> {
    serde_json::from_str(payload).map_err(|error| {
        ContractError::Conflict(format!(
            "postgres journal payload must be valid JSONB: {error}"
        ))
    })
}

pub(crate) fn postgres_timestamptz(value: &str, field: &'static str) -> Result<DateTime<Utc>, ContractError> {
    DateTime::parse_from_rfc3339(value.trim())
        .map(|instant| instant.with_timezone(&Utc))
        .or_else(|_| value.trim().parse::<DateTime<Utc>>())
        .map_err(|error| {
            ContractError::Conflict(format!(
                "postgres journal {field} must be RFC3339: {error}"
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

fn append_one(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
) -> Result<CommitPosition, ContractError> {
    let mut client = postgres_pool_client(pool, "journal append")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("journal append begin", error))?;

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
    let aggregate_seq = i64::try_from(envelope.ordering_seq)
        .unwrap_or(0)
        .saturating_add(1);
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
                savepoint
                    .commit()
                    .map_err(|error| postgres_unavailable_db("journal append savepoint commit", error))?;
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
            let partition: String = row.get(0);
            let offset: i64 = row.get(1);
            (partition, offset as u64)
        }
        // ON CONFLICT (event_id) absorbed the row: read the previously
        // committed position by event_id. This path is the idempotent replay
        // of the exact same producer event.
        InsertOutcome::EventIdAbsorbed => {
            let row = txn
                .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
                .map_err(|error| postgres_unavailable_db("journal append conflict lookup", error))?;
            let partition: String = row.get(0);
            let offset: i64 = row.get(1);
            (partition, offset as u64)
        }
        // PK (partition_key, commit_offset) violated with a different
        // event_id. Look up the occupying row by position: if it carries the
        // same event_id, treat as idempotent (defensive — ON CONFLICT should
        // have caught it); otherwise surface a Conflict so callers map it to
        // HTTP 409 instead of an opaque 503.
        InsertOutcome::PositionCollision => {
            let row = txn
                .query_one(LOAD_EVENT_BY_POSITION_SQL, &[&partition_key, &commit_offset])
                .map_err(|error| postgres_unavailable_db("journal append position lookup", error))?;
            let existing_event_id: String = row.get(0);
            let partition: String = row.get(1);
            let offset: i64 = row.get(2);
            if existing_event_id == envelope.event_id {
                (partition, offset as u64)
            } else {
                return Err(ContractError::Conflict(format!(
                    "journal position (partition_key={partition_key}, commit_offset={commit_offset}) \
                     is already occupied by event_id={existing_event_id}; \
                     cannot append event_id={}",
                    envelope.event_id
                )));
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

    let mut positions = Vec::with_capacity(envelopes.len());
    for envelope in &envelopes {
        let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
        let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
        let payload_hash = sha256_hash(envelope.payload.as_bytes());
        let created_at = Utc::now();
        // Map 0-indexed `ordering_seq` to 1-indexed `commit_offset`/`aggregate_seq`.
        // See `append_one` for why `.max(1)` would collide ordering_seq=0 and =1.
        let aggregate_seq = i64::try_from(envelope.ordering_seq)
            .unwrap_or(0)
            .saturating_add(1);
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
                    savepoint
                        .commit()
                        .map_err(|error| postgres_unavailable("journal append_batch savepoint commit", error))?;
                    match row {
                        Some(row) => InsertOutcome::Inserted(row),
                        None => InsertOutcome::EventIdAbsorbed,
                    }
                }
                Err(error) if is_unique_violation(&error) => {
                    savepoint
                        .rollback()
                        .map_err(|error| postgres_unavailable("journal append_batch savepoint rollback", error))?;
                    InsertOutcome::PositionCollision
                }
                Err(error) => {
                    return Err(postgres_unavailable("journal append_batch insert", error));
                }
            }
        };

        let (final_partition, final_offset) = match outcome {
            InsertOutcome::Inserted(row) => {
                let partition: String = row.get(0);
                let offset: i64 = row.get(1);
                (partition, offset as u64)
            }
            InsertOutcome::EventIdAbsorbed => {
                let row = txn
                    .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
                    .map_err(|error| {
                        postgres_unavailable("journal append_batch conflict lookup", error)
                    })?;
                let partition: String = row.get(0);
                let offset: i64 = row.get(1);
                (partition, offset as u64)
            }
            InsertOutcome::PositionCollision => {
                let row = txn
                    .query_one(LOAD_EVENT_BY_POSITION_SQL, &[&partition_key, &commit_offset])
                    .map_err(|error| {
                        postgres_unavailable("journal append_batch position lookup", error)
                    })?;
                let existing_event_id: String = row.get(0);
                let partition: String = row.get(1);
                let offset: i64 = row.get(2);
                if existing_event_id == envelope.event_id {
                    (partition, offset as u64)
                } else {
                    return Err(ContractError::Conflict(format!(
                        "journal position (partition_key={partition_key}, commit_offset={commit_offset}) \
                         is already occupied by event_id={existing_event_id}; \
                         cannot append event_id={}",
                        envelope.event_id
                    )));
                }
            }
        };

        positions.push(CommitPosition::new(final_partition, final_offset));
    }

    txn.commit()
        .map_err(|error| postgres_unavailable("journal append_batch commit", error))?;

    Ok(positions)
}

/// Reconstruct committed envelopes in append order.
///
/// Only the columns needed to rehydrate a [`CommitEnvelope`] are selected;
/// the full event payload is round-tripped as text to preserve its original
/// JSON encoding. Envelopes reconstructed here are best-effort projections —
/// callers that need the authoritative aggregate state should replay through
/// the domain layer rather than consume this projection directly.
fn load_recorded(
    pool: &PostgresJournalPool,
    prefix: &str,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    let mut client = postgres_pool_client(pool, "journal recorded")?;
    let pattern = format!("{prefix}%");
    let rows = client
        .query(LOAD_RECORDED_SQL, &[&pattern])
        .map_err(|error| postgres_unavailable("journal recorded select", error))?;

    let mut envelopes = Vec::with_capacity(rows.len());
    for row in rows {
        let event_id: String = row.get(0);
        let tenant_id: String = row.get(1);
        let organization_id: String = row.get(2);
        let event_type: String = row.get(3);
        let aggregate_type_str: String = row.get(4);
        let aggregate_id: String = row.get(5);
        let aggregate_seq: i64 = row.get(6);
        let occurred_at: String = row.get(7);
        let payload: String = row.get(8);
        let idempotency_key: Option<String> = row.get(9);
        let partition_key: String = row.get(10);
        let aggregate_type = parse_aggregate_type(aggregate_type_str.as_str());
        let scope = replay_scope_for_journal_row(
            &aggregate_type,
            tenant_id.as_str(),
            aggregate_id.as_str(),
            partition_key.as_str(),
            prefix,
        );
        let ordering_seq = aggregate_seq.saturating_sub(1).max(0) as u64;

        envelopes.push(CommitEnvelope {
            event_id,
            tenant_id,
            organization_id: im_domain_events::normalize_commit_organization_id(
                organization_id.as_str(),
            ),
            event_type,
            event_version: 1,
            aggregate_type,
            aggregate_id: aggregate_id.clone(),
            scope_type: scope.scope_type,
            scope_id: scope.scope_id,
            ordering_key: scope.ordering_key,
            ordering_seq,
            causation_id: None,
            correlation_id: None,
            idempotency_key,
            actor: EventActor {
                actor_id: String::new(),
                actor_kind: String::new(),
                actor_session_id: None,
            },
            occurred_at: occurred_at.clone(),
            committed_at: occurred_at,
            payload_schema: None,
            payload,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        });
    }
    Ok(envelopes)
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
/// Runs synchronous postgres driver work on a dedicated OS thread via
/// [`std::thread::scope`] so the blocking `postgres` crate never nests Tokio runtimes.
pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
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
    error: impl std::fmt::Display,
) -> ContractError {
    ContractError::Unavailable(format!("postgres journal {action} failed: {error}"))
}

pub(crate) fn postgres_unavailable_db(
    action: &'static str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    ContractError::Unavailable(format!(
        "postgres journal {action} failed: {}",
        format_postgres_db_error(&error)
    ))
}

fn format_postgres_db_error(error: &r2d2_postgres::postgres::Error) -> String {
    if let Some(db_error) = error.as_db_error() {
        let message = db_error.message().trim();
        if let Some(detail) = db_error
            .detail()
            .map(str::trim)
            .filter(|detail| !detail.is_empty())
        {
            return format!("{message}; {detail}");
        }
        return message.to_string();
    }
    error.to_string()
}

/// Returns true when the postgres error is a unique constraint violation
/// (SQLSTATE 23505). Used to absorb `(partition_key, commit_offset)` primary
/// key collisions in `append`/`append_batch` so that replayed producer events
/// stay idempotent alongside the existing `ON CONFLICT (event_id) DO NOTHING`
/// path.
fn is_unique_violation(error: &r2d2_postgres::postgres::Error) -> bool {
    error
        .as_db_error()
        .map(|db_error| db_error.code())
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
    database_url: &str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    let redacted = redact_postgres_url(database_url);
    ContractError::Unavailable(format!(
        "postgres journal database url is invalid ({redacted}): {error}"
    ))
}

/// Redact credentials from a PostgreSQL connection URL before it enters an
/// error message or log line. If the URL cannot be parsed as `scheme://user:pass@host`,
/// it is replaced wholesale with `<redacted>` to avoid leaking any fragment.
fn journal_retention_until(envelope: &CommitEnvelope) -> Option<String> {
    retention_until_from_envelope(envelope.retention_class.as_str(), envelope.occurred_at.as_str())
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
