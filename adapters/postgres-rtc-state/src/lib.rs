//! PostgreSQL-backed durable state store for RTC call sessions.
//!
//! This adapter implements [`StateStore`] from `im-domain-core::rtc` using
//! the `im_rtc_sessions` table (defined in
//! `database/ddl/baseline/postgres/0001_im_baseline.sql` and extended
//! by migration `0008_im_rtc_state_machine_expansion`).
//!
//! ## Storage model
//!
//! The full [`RtcStateRecord`] (session + signals) is serialized as JSONB
//! into the `payload_json` column. Key structured columns (`session_state`,
//! `started_at`, `ended_at`, `updated_at`, lifecycle timestamps) are also
//! updated from the deserialized session for querying and indexing.
//!
//! This hybrid approach gives:
//! - **Durability**: the JSONB document is the source of truth.
//! - **Queryability**: structured columns support index-backed scans for
//!   active sessions, state-based filtering, and retention cleanup.
//! - **Atomicity**: a single UPSERT in a transaction replaces the full
//!   state, avoiding the complexity of synchronizing two tables.
//!
//! ## Epoch fencing
//!
//! `save_state` uses `SELECT ... FOR UPDATE` to lock the row, then checks
//! the incoming epoch against the persisted epoch. Stale writes (incoming
//! epoch < persisted epoch) return [`RtcContractError::Conflict`] so the
//! caller can surface the inconsistency.
//!
//! ## Threading bridge
//!
//! [`StateStore`] is a synchronous trait. PostgreSQL I/O is bridged onto a
//! blocking scope via `tokio::task::block_in_place` when a tokio runtime is
//! present, matching the pattern in `adapters/postgres-journal`.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use im_app_context::is_production_like_im_environment;
use im_domain_core::retention::retention_until_from_class;
use im_domain_core::rtc::{RtcSession, RtcStateRecord, StateStore};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_communication_rtc_service::RtcContractError;
use sdkwork_utils_rust::sha256_hash;

/// TLS connector type for r2d2-backed PostgreSQL pools.
///
/// P0-12 (SECURITY_SPEC): uses `postgres-native-tls` so the `sslmode` URL
/// parameter is honored. With `sslmode=disable` the connector is never
/// invoked (plaintext TCP); with `sslmode=require` or `verify-full` a real
/// TLS handshake is performed. This allows dev/test to keep using plaintext
/// while production enforces TLS via the DSN.
pub type PostgresRtcTlsConnector = postgres_native_tls::MakeTlsConnector;
/// Connection manager / pool type aliases.
pub type PostgresRtcConnectionManager = PostgresConnectionManager<PostgresRtcTlsConnector>;
pub type PostgresRtcPool = Pool<PostgresRtcConnectionManager>;

const DEFAULT_POOL_MAX_SIZE: u32 = 8;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

/// Configuration for the PostgreSQL RTC state store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresRtcStateConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl PostgresRtcStateConfig {
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

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    /// Build the r2d2 connection pool.
    pub fn connect_pool(&self) -> Result<PostgresRtcPool, RtcContractError> {
        if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
            return Ok(pool);
        }
        if cfg!(test) {
            return self.connect_pool_local();
        }
        Err(RtcContractError::Unavailable(
            sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
                .err()
                .unwrap_or_else(|| "IM process database pools are not installed".to_owned()),
        ))
    }

    fn connect_pool_local(&self) -> Result<PostgresRtcPool, RtcContractError> {
        verify_production_sslmode(self.database_url.as_str())?;
        let pg_config = self
            .database_url
            .parse::<r2d2_postgres::postgres::Config>()
            .map_err(|err| RtcContractError::Unavailable(format!("invalid database_url: {err}")))?;
        let tls = make_tls_connector().map_err(|err| {
            RtcContractError::Unavailable(format!("postgres rtc TLS connector build failed: {err}"))
        })?;
        let manager = PostgresConnectionManager::new(pg_config, tls);
        Pool::builder()
            .max_size(self.pool_max_size)
            .min_idle(self.pool_min_idle)
            .build(manager)
            .map_err(|err| {
                RtcContractError::Unavailable(format!("postgres pool build failed: {err}"))
            })
    }
}

/// PostgreSQL-backed implementation of [`StateStore`].
///
/// Stores the full [`RtcStateRecord`] as JSONB in `im_rtc_sessions.payload_json`,
/// with structured columns updated for querying. Uses epoch-based fencing
/// via `SELECT ... FOR UPDATE` to reject stale concurrent writes.
#[derive(Clone)]
pub struct PostgresRtcStateStore {
    pool: PostgresRtcPool,
}

impl PostgresRtcStateStore {
    pub fn new(pool: PostgresRtcPool) -> Self {
        Self { pool }
    }

    pub fn from_config(config: &PostgresRtcStateConfig) -> Result<Self, RtcContractError> {
        let pool = config.connect_pool()?;
        Ok(Self::new(pool))
    }

    fn run_blocking<F, T>(&self, f: F) -> Result<T, RtcContractError>
    where
        F: FnOnce(&PostgresRtcPool) -> Result<T, RtcContractError> + Send,
        T: Send,
    {
        // Bridge synchronous PostgreSQL I/O onto a blocking scope when a
        // tokio runtime is present. This prevents blocking the async
        // executor thread.
        let pool = self.pool.clone();
        tokio::task::block_in_place(|| f(&pool))
    }
}

impl StateStore for PostgresRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, RtcContractError> {
        let tenant_id = tenant_id.to_string();
        let rtc_session_id = rtc_session_id.to_string();
        self.run_blocking(move |pool| {
            let mut client = pool
                .get()
                .map_err(|err| RtcContractError::Unavailable(format!("pool get failed: {err}")))?;
            let row = client
                .query_opt(
                    "SELECT payload_json::text FROM im_rtc_sessions
                 WHERE tenant_id = $1 AND rtc_session_id = $2",
                    &[&tenant_id, &rtc_session_id],
                )
                .map_err(|err| {
                    RtcContractError::Unavailable(format!("load_state query failed: {err}"))
                })?;
            match row {
                Some(row) => {
                    let payload: String = row.get(0);
                    let record: RtcStateRecord = serde_json::from_str(&payload).map_err(|err| {
                        RtcContractError::Unavailable(format!(
                            "load_state deserialize failed: {err}"
                        ))
                    })?;
                    Ok(Some(record))
                }
                None => Ok(None),
            }
        })
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), RtcContractError> {
        self.run_blocking(move |pool| {
            let mut client = pool
                .get()
                .map_err(|err| RtcContractError::Unavailable(format!("pool get failed: {err}")))?;

            let mut tx = client
                .transaction()
                .map_err(|err| RtcContractError::Unavailable(format!("tx begin failed: {err}")))?;

            // Lock the existing row (if any) for the duration of the
            // transaction so concurrent saves serialize correctly.
            let existing_epoch: Option<u64> = tx
                .query_opt(
                    "SELECT (payload_json::jsonb -> 'session' ->> 'epoch')::bigint
                     FROM im_rtc_sessions
                     WHERE tenant_id = $1 AND rtc_session_id = $2
                     FOR UPDATE",
                    &[&record.tenant_id, &record.rtc_session_id],
                )
                .map_err(|err| {
                    RtcContractError::Unavailable(format!("save_state lock failed: {err}"))
                })?
                .map(|row| row.get::<_, i64>(0) as u64);

            // Epoch fencing: reject stale or duplicate concurrent writes.
            if let Some(existing) = existing_epoch {
                if record.session.epoch <= existing {
                    return Err(RtcContractError::Conflict(format!(
                        "stale epoch rejected: existing={} incoming={}",
                        existing, record.session.epoch
                    )));
                }
            }

            let payload_json = serde_json::to_string(&record).map_err(|err| {
                RtcContractError::Unavailable(format!("save_state serialize failed: {err}"))
            })?;
            let payload_hash = sha256_hash(payload_json.as_bytes());
            let session_state = record.session.state.as_str();
            let started_at = parse_rtc_timestamp(record.session.started_at.as_str(), "started_at")?;
            let ended_at =
                parse_optional_rtc_timestamp(record.session.ended_at.as_deref(), "ended_at")?;
            let updated_at = parse_rtc_timestamp(record.updated_at.as_str(), "updated_at")?;
            let initiator_kind = record.session.initiator_kind.as_str();
            let initiator_id = record.session.initiator_id.as_str();
            let rtc_mode = record.session.rtc_mode.as_str();
            let conversation_id = record.session.conversation_id.as_deref();
            let provider_plugin_id = record.session.provider_plugin_id.as_deref();
            let provider_session_id = record.session.provider_session_id.as_deref();
            let provider_region = record.session.provider_region.as_deref();
            let access_endpoint = record.session.access_endpoint.as_deref();
            let signaling_stream_id = record.session.signaling_stream_id.as_deref();
            let artifact_message_id = record.session.artifact_message_id.as_deref();
            let latest_signal_seq: i64 = record
                .signals
                .iter()
                .map(|s| s.signal_seq as i64)
                .max()
                .unwrap_or(0);
            let initiating_at = parse_optional_rtc_timestamp(
                record.session.initiating_at.as_deref(),
                "initiating_at",
            )?;
            let ringing_at =
                parse_optional_rtc_timestamp(record.session.ringing_at.as_deref(), "ringing_at")?;
            let connecting_at = parse_optional_rtc_timestamp(
                record.session.connecting_at.as_deref(),
                "connecting_at",
            )?;
            let connected_at = parse_optional_rtc_timestamp(
                record.session.connected_at.as_deref(),
                "connected_at",
            )?;
            let on_hold_since = parse_optional_rtc_timestamp(
                record.session.on_hold_since.as_deref(),
                "on_hold_since",
            )?;
            let reconnecting_since = parse_optional_rtc_timestamp(
                record.session.reconnecting_since.as_deref(),
                "reconnecting_since",
            )?;
            let canceled_at =
                parse_optional_rtc_timestamp(record.session.canceled_at.as_deref(), "canceled_at")?;
            let failed_at =
                parse_optional_rtc_timestamp(record.session.failed_at.as_deref(), "failed_at")?;
            let timeout_at =
                parse_optional_rtc_timestamp(record.session.timeout_at.as_deref(), "timeout_at")?;
            let ended_reason = record.session.ended_reason.as_deref();
            let failure_reason = record.session.failure_reason.as_deref();
            let tenant_id = record.tenant_id.as_str();
            let rtc_session_id = record.rtc_session_id.as_str();
            let retention_until = match resolve_rtc_session_retention_until(&record) {
                Some(value) => Some(parse_rtc_timestamp(value.as_str(), "retention_until")?),
                None => None,
            };

            // UPSERT with full structured column update.
            let affected = tx
                .execute(
                    "INSERT INTO im_rtc_sessions (
                    tenant_id, rtc_session_id, conversation_id, rtc_mode,
                    initiator_principal_kind, initiator_principal_id,
                    provider_plugin_id, provider_session_id, provider_region,
                    access_endpoint, session_state, latest_signal_seq,
                    signaling_stream_id, artifact_message_id,
                    started_at, ended_at,
                    initiating_at, ringing_at, connecting_at, connected_at,
                    on_hold_since, reconnecting_since,
                    canceled_at, failed_at, timeout_at,
                    ended_reason, failure_reason,
                    payload_json, payload_hash,
                    retention_until,
                    created_at, updated_at
                 ) VALUES (
                    $1, $2, $3, $4,
                    $5, $6,
                    $7, $8, $9,
                    $10, $11, $12,
                    $13, $14,
                    $15, $16,
                    $17, $18, $19, $20,
                    $21, $22,
                    $23, $24, $25,
                    $26, $27,
                    $28, $29,
                    $30,
                    $31, $31
                 )
                 ON CONFLICT (tenant_id, rtc_session_id) DO UPDATE SET
                    conversation_id = EXCLUDED.conversation_id,
                    rtc_mode = EXCLUDED.rtc_mode,
                    initiator_principal_kind = EXCLUDED.initiator_principal_kind,
                    initiator_principal_id = EXCLUDED.initiator_principal_id,
                    provider_plugin_id = EXCLUDED.provider_plugin_id,
                    provider_session_id = EXCLUDED.provider_session_id,
                    provider_region = EXCLUDED.provider_region,
                    access_endpoint = EXCLUDED.access_endpoint,
                    session_state = EXCLUDED.session_state,
                    latest_signal_seq = EXCLUDED.latest_signal_seq,
                    signaling_stream_id = EXCLUDED.signaling_stream_id,
                    artifact_message_id = EXCLUDED.artifact_message_id,
                    started_at = EXCLUDED.started_at,
                    ended_at = EXCLUDED.ended_at,
                    initiating_at = EXCLUDED.initiating_at,
                    ringing_at = EXCLUDED.ringing_at,
                    connecting_at = EXCLUDED.connecting_at,
                    connected_at = EXCLUDED.connected_at,
                    on_hold_since = EXCLUDED.on_hold_since,
                    reconnecting_since = EXCLUDED.reconnecting_since,
                    canceled_at = EXCLUDED.canceled_at,
                    failed_at = EXCLUDED.failed_at,
                    timeout_at = EXCLUDED.timeout_at,
                    ended_reason = EXCLUDED.ended_reason,
                    failure_reason = EXCLUDED.failure_reason,
                    payload_json = EXCLUDED.payload_json,
                    payload_hash = EXCLUDED.payload_hash,
                    retention_until = EXCLUDED.retention_until,
                    updated_at = EXCLUDED.updated_at
                 WHERE COALESCE(
                    (im_rtc_sessions.payload_json::jsonb -> 'session' ->> 'epoch')::bigint,
                    0
                 ) < COALESCE(
                    (EXCLUDED.payload_json::jsonb -> 'session' ->> 'epoch')::bigint,
                    0
                 )",
                    &[
                        &tenant_id,
                        &rtc_session_id,
                        &conversation_id,
                        &rtc_mode,
                        &initiator_kind,
                        &initiator_id,
                        &provider_plugin_id,
                        &provider_session_id,
                        &provider_region,
                        &access_endpoint,
                        &session_state,
                        &latest_signal_seq,
                        &signaling_stream_id,
                        &artifact_message_id,
                        &started_at,
                        &ended_at,
                        &initiating_at,
                        &ringing_at,
                        &connecting_at,
                        &connected_at,
                        &on_hold_since,
                        &reconnecting_since,
                        &canceled_at,
                        &failed_at,
                        &timeout_at,
                        &ended_reason,
                        &failure_reason,
                        &payload_json,
                        &payload_hash,
                        &retention_until,
                        &updated_at,
                    ],
                )
                .map_err(|err| {
                    RtcContractError::Unavailable(format!("save_state upsert failed: {err}"))
                })?;
            if affected == 0 {
                return Err(RtcContractError::Conflict(
                    "concurrent rtc session save rejected by epoch fencing".into(),
                ));
            }

            tx.commit().map_err(|err| {
                RtcContractError::Unavailable(format!("save_state commit failed: {err}"))
            })?;

            Ok(())
        })
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, RtcContractError> {
        let tenant_id = tenant_id.to_string();
        let rtc_session_id = rtc_session_id.to_string();
        self.run_blocking(move |pool| {
            let mut client = pool
                .get()
                .map_err(|err| RtcContractError::Unavailable(format!("pool get failed: {err}")))?;
            let affected = client
                .execute(
                    "DELETE FROM im_rtc_sessions
                 WHERE tenant_id = $1 AND rtc_session_id = $2",
                    &[&tenant_id, &rtc_session_id],
                )
                .map_err(|err| {
                    RtcContractError::Unavailable(format!("clear_state delete failed: {err}"))
                })?;
            Ok(affected > 0)
        })
    }
}

/// Build a [`PostgresRtcStateStore`] from a database URL.
///
/// Convenience wrapper for `PostgresRtcStateConfig::new(url).connect_pool()`
/// followed by `PostgresRtcStateStore::new(pool)`.
pub fn build_postgres_rtc_state_store(
    database_url: &str,
) -> Result<PostgresRtcStateStore, RtcContractError> {
    let config = PostgresRtcStateConfig::new(database_url);
    let pool = config.connect_pool()?;
    Ok(PostgresRtcStateStore::new(pool))
}

/// Build a [`PostgresRtcStateStore`] wrapped in an [`Arc`] for shared use,
/// returning `Ok(None)` when the database URL is empty (signaling-only mode).
///
/// Production deployments MUST provide a valid database URL; the `Ok(None)`
/// fallback is intended for development/testing only. In production-like
/// environments, connection failures return `Err(RtcContractError::Unavailable)`
/// instead of panicking, allowing the caller to decide how to recover (e.g.
/// via a fail-closed check or K8s restart).
pub fn build_postgres_rtc_state_store_optional(
    database_url: Option<&str>,
) -> Result<Option<Arc<PostgresRtcStateStore>>, RtcContractError> {
    let url = match database_url {
        Some(url) => url.trim(),
        None => return Ok(None),
    };
    if url.is_empty() {
        return Ok(None);
    }
    match build_postgres_rtc_state_store(url) {
        Ok(store) => {
            tracing::info!("PostgresRtcStateStore connected successfully");
            Ok(Some(Arc::new(store)))
        }
        Err(err) => {
            if is_production_like_im_environment() {
                return Err(RtcContractError::Unavailable(
                    format!(
                        "PostgresRtcStateStore connection failed in production-like \
                         environment: {err:?}"
                    )
                    .into(),
                ));
            }
            tracing::warn!(
                error = %format!("{err:?}"),
                "PostgresRtcStateStore connection failed; durable store unavailable (development/test only)"
            );
            Ok(None)
        }
    }
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
fn verify_production_sslmode(database_url: &str) -> Result<(), RtcContractError> {
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
        return Err(RtcContractError::Unavailable(
            format!(
                "P0-12 production fail-closed: SDKWORK_IM_DATABASE_URL must contain \
                 sslmode=require or sslmode=verify-full in production \
                 (current environment={environment}). Refusing to start with a \
                 plaintext database connection."
            )
            .into(),
        ));
    }
    Ok(())
}

/// Parse an RFC3339 timestamp string into `DateTime<Utc>` so it serializes
/// as `TIMESTAMPTZ` (matching the column type). Passing raw `String`s
/// produces `VARCHAR`-typed parameters that fail serialization against
/// `TIMESTAMPTZ` columns with "error serializing parameter N".
fn parse_rtc_timestamp(
    value: &str,
    field: &'static str,
) -> Result<DateTime<Utc>, RtcContractError> {
    DateTime::parse_from_rfc3339(value.trim())
        .map(|instant| instant.with_timezone(&Utc))
        .or_else(|_| value.trim().parse::<DateTime<Utc>>())
        .map_err(|error| {
            RtcContractError::Unavailable(format!("rtc {field} must be RFC3339: {error}"))
        })
}

fn parse_optional_rtc_timestamp(
    value: Option<&str>,
    field: &'static str,
) -> Result<Option<DateTime<Utc>>, RtcContractError> {
    value.map(|v| parse_rtc_timestamp(v, field)).transpose()
}

/// Terminal RTC sessions use the canonical `ephemeral` retention class (7 days).
/// Active sessions keep `retention_until` unset so the shared purge job never
/// deletes in-flight calls.
fn resolve_rtc_session_retention_until(record: &RtcStateRecord) -> Option<String> {
    if !record.session.state.is_terminal() {
        return None;
    }
    let anchor = terminal_retention_anchor(&record.session).unwrap_or(record.updated_at.as_str());
    retention_until_from_class("ephemeral", anchor)
}

fn terminal_retention_anchor(session: &RtcSession) -> Option<&str> {
    session
        .ended_at
        .as_deref()
        .or(session.canceled_at.as_deref())
        .or(session.failed_at.as_deref())
        .or(session.timeout_at.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validates_database_url() {
        let config = PostgresRtcStateConfig::new("postgres://localhost/test");
        assert_eq!(config.database_url(), "postgres://localhost/test");
        assert_eq!(config.pool_max_size, DEFAULT_POOL_MAX_SIZE);
    }

    #[test]
    fn config_with_pool_max_size_clamps_to_minimum_1() {
        let config = PostgresRtcStateConfig::new("postgres://localhost/test").with_pool_max_size(0);
        assert_eq!(config.pool_max_size, 1);
    }

    #[test]
    fn build_optional_returns_none_for_empty_url() {
        assert!(build_postgres_rtc_state_store_optional(None).unwrap().is_none());
        assert!(build_postgres_rtc_state_store_optional(Some("")).unwrap().is_none());
        assert!(build_postgres_rtc_state_store_optional(Some("   ")).unwrap().is_none());
    }

    #[test]
    fn resolve_rtc_retention_until_for_terminal_session_only() {
        use im_domain_core::rtc::{
            RtcSession, RtcSessionState, RtcStateRecord, SessionParticipants, SignalRateTracker,
        };

        let record = RtcStateRecord {
            tenant_id: "100001".into(),
            rtc_session_id: "rtc_demo".into(),
            session: RtcSession {
                tenant_id: "100001".into(),
                organization_id: "default".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
                initiator_id: "1".into(),
                initiator_kind: "user".into(),
                provider_plugin_id: None,
                provider_session_id: None,
                access_endpoint: None,
                provider_region: None,
                state: RtcSessionState::Ended,
                signaling_stream_id: None,
                artifact_message_id: None,
                started_at: "2026-01-01T00:00:00.000Z".into(),
                ended_at: Some("2026-01-01T00:00:00.000Z".into()),
                initiating_at: None,
                ringing_at: None,
                connecting_at: None,
                connected_at: None,
                on_hold_since: None,
                reconnecting_since: None,
                canceled_at: None,
                failed_at: None,
                timeout_at: None,
                ended_reason: None,
                failure_reason: None,
                epoch: 1,
                version: 1,
                participants: SessionParticipants::default(),
                last_activity_at: None,
                signal_rate_tracker: SignalRateTracker::default(),
            },
            signals: Vec::new(),
            updated_at: "2026-01-01T00:00:00.000Z".into(),
        };

        assert_eq!(
            resolve_rtc_session_retention_until(&record).as_deref(),
            Some("2026-01-08T00:00:00.000Z")
        );

        let mut active = record.clone();
        active.session.state = RtcSessionState::Connected;
        active.session.ended_at = None;
        assert!(resolve_rtc_session_retention_until(&active).is_none());
    }
}
