use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use axum::extract::{DefaultBodyLimit, Extension, Query, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{AppContext, resolve_web_environment_from_process_env};
use im_time::utc_now_rfc3339_millis;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_web_core::{
    WebEnvironment, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
    problem_response, ProblemCorrelation,
};
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use serde::{Deserialize, Serialize};
use sdkwork_utils_rust::sha256_hash;
use tokio::sync::Semaphore;
use tracing::{error, info};

const AUDIT_RECORD_ID_MAX_BYTES: usize = 256;
const AUDIT_AGGREGATE_TYPE_MAX_BYTES: usize = 128;
const AUDIT_AGGREGATE_ID_MAX_BYTES: usize = 256;
const AUDIT_ACTION_MAX_BYTES: usize = 128;
const AUDIT_PAYLOAD_MAX_BYTES: usize = 128 * 1024;
const AUDIT_RECORD_LIST_MAX_LIMIT: usize = 1000;
const AUDIT_RECORD_DELIVERY_PROOF_VERSION: &str = "audit.record.delivery-proof.v1";
const AUDIT_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_AUDIT_MAX_IN_FLIGHT_REQUESTS";
const AUDIT_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const AUDIT_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const AUDIT_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_AUDIT_MAX_REQUEST_BODY_BYTES";
const AUDIT_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const AUDIT_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
/// Default upper bound on pooled PostgreSQL connections for the audit store.
///
/// Kept aligned with `adapters/postgres-journal` (`DEFAULT_POOL_MAX_SIZE = 16`)
/// so a single database is not over-subscribed when both stores are active.
const AUDIT_POSTGRES_POOL_MAX_SIZE: u32 = 16;

#[derive(Clone)]
pub struct AppState {
    runtime: Arc<AuditRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecord {
    pub tenant_id: String,
    pub record_id: String,
    pub audit_seq: u64,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub action: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub actor_session_id: Option<String>,
    pub payload: Option<String>,
    pub recorded_at: String,
    pub chain_prev_hash: Option<String>,
    pub chain_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditExportBundle {
    pub tenant_id: String,
    pub exported_at: String,
    pub total: usize,
    pub items: Vec<AuditRecord>,
    pub chain_head_hash: Option<String>,
    pub chain_valid: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditChainVerification {
    pub tenant_id: String,
    pub verified_at: String,
    pub total: usize,
    pub chain_head_hash: Option<String>,
    pub chain_valid: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordAuditAnchor {
    pub record_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub action: String,
    pub payload: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAuditRecordsQuery {
    pub after_audit_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditRecordMutationOutcome {
    pub record: AuditRecord,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRecordDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecordMutationResponse {
    #[serde(flatten)]
    pub record: AuditRecord,
    pub request_key: String,
    pub delivery_status: AuditRecordDeliveryStatus,
    pub proof_version: String,
}

impl AuditRecordMutationResponse {
    pub fn from_outcome(outcome: AuditRecordMutationOutcome, request_key: String) -> Self {
        Self {
            record: outcome.record,
            request_key,
            delivery_status: if outcome.applied {
                AuditRecordDeliveryStatus::Applied
            } else {
                AuditRecordDeliveryStatus::Replayed
            },
            proof_version: AUDIT_RECORD_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

/// Audit runtime that dispatches each public method to a selected backend.
///
/// The backend is chosen at construction time (see [`AuditRuntime::from_env`])
/// and is transparent to handlers: the six public method signatures are stable
/// regardless of whether records live in process memory (dev/test) or in
/// PostgreSQL (production). Production never silently degrades to the
/// in-memory backend when a database URL is configured.
#[derive(Default)]
pub struct AuditRuntime {
    backend: AuditBackend,
}

/// Private persistence backend selected by [`AuditRuntime::from_env`].
///
/// `InMemory` is the default and the only backend used outside production.
/// `Postgres` is selected only when `SDKWORK_IM_ENVIRONMENT=prod` (the
/// process default) and `SDKWORK_IM_DATABASE_URL` is set; initialization
/// failure there is fail-closed (the runtime refuses to start rather than
/// fall back to volatile storage).
enum AuditBackend {
    InMemory {
        records: RwLock<HashMap<String, TenantAuditRecords>>,
    },
    Postgres(PostgresAuditStore),
}

impl Default for AuditBackend {
    fn default() -> Self {
        AuditBackend::InMemory {
            records: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Default)]
struct TenantAuditRecords {
    by_record_id: HashMap<String, AuditRecord>,
    by_audit_seq: BTreeMap<u64, String>,
    record_order: Vec<String>,
}

impl TenantAuditRecords {
    fn get(&self, record_id: &str) -> Option<&AuditRecord> {
        self.by_record_id.get(record_id)
    }

    fn last(&self) -> Option<&AuditRecord> {
        self.record_order
            .last()
            .and_then(|record_id| self.by_record_id.get(record_id.as_str()))
    }

    fn push(&mut self, record: AuditRecord) {
        self.record_order.push(record.record_id.clone());
        self.by_audit_seq
            .insert(record.audit_seq, record.record_id.clone());
        self.by_record_id.insert(record.record_id.clone(), record);
    }

    fn ordered_items(&self) -> Vec<AuditRecord> {
        self.record_order
            .iter()
            .filter_map(|record_id| self.by_record_id.get(record_id.as_str()).cloned())
            .collect()
    }

    fn next_audit_seq(&self) -> u64 {
        self.by_audit_seq
            .last_key_value()
            .map_or(1, |(seq, _)| seq + 1)
    }

    fn window(&self, after_audit_seq: u64, limit: usize) -> AuditRecordListResponse {
        let mut items = Vec::new();
        let mut has_more = false;
        for (_, record_id) in self
            .by_audit_seq
            .range((Excluded(after_audit_seq), Unbounded))
        {
            if items.len() == limit {
                has_more = true;
                break;
            }
            if let Some(record) = self.by_record_id.get(record_id.as_str()).cloned() {
                items.push(record);
            }
        }
        let next_after_audit_seq = items.last().map(|record| record.audit_seq);

        AuditRecordListResponse {
            items,
            next_after_audit_seq,
            has_more,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecordListResponse {
    items: Vec<AuditRecord>,
    next_after_audit_seq: Option<u64>,
    has_more: bool,
}

#[derive(Debug)]
pub struct AuditError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl AuditError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn conflict(record_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "audit_record_conflict",
            message: format!(
                "audit record request conflicts with existing idempotency key: {record_id}"
            ),
        }
    }

    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }
}

/// Map [`AuditError::status`] to the canonical [`WebFrameworkErrorKind`].
fn audit_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
    use axum::http::StatusCode;
    match *status {
        StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkErrorKind::PayloadTooLarge,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        StatusCode::NOT_IMPLEMENTED => WebFrameworkErrorKind::NotImplemented,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<AuditError> for ApiProblem {
    fn from(error: AuditError) -> Self {
        let framework_error = WebFrameworkError {
            kind: audit_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

/// Fallback `IntoResponse` for contexts where [`WebRequestContext`] is not
/// available (e.g. middleware without context injection). Produces a
/// `application/problem+json` response without `traceId`.
impl IntoResponse for AuditError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: audit_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

impl AuditRuntime {
    pub fn record_anchor(
        &self,
        auth: &AppContext,
        request: RecordAuditAnchor,
    ) -> Result<AuditRecord, AuditError> {
        Ok(self.record_anchor_with_outcome(auth, request)?.record)
    }

    pub fn record_anchor_with_outcome(
        &self,
        auth: &AppContext,
        request: RecordAuditAnchor,
    ) -> Result<AuditRecordMutationOutcome, AuditError> {
        match &self.backend {
            AuditBackend::InMemory { records } => {
                record_anchor_in_memory(records, auth, request)
            }
            AuditBackend::Postgres(store) => store.record_anchor_with_outcome(auth, request),
        }
    }

    pub fn list_records(&self, auth: &AppContext) -> Vec<AuditRecord> {
        match &self.backend {
            AuditBackend::InMemory { records } => Self::read_records(records)
                .get(auth.tenant_id.as_str())
                .map(TenantAuditRecords::ordered_items)
                .unwrap_or_default(),
            AuditBackend::Postgres(store) => store.list_records(auth).unwrap_or_else(|error| {
                error!(
                    code = error.code,
                    message = %error.message,
                    "audit-service failed to load records for list_records"
                );
                Vec::new()
            }),
        }
    }

    pub fn list_records_window(
        &self,
        auth: &AppContext,
        query: ListAuditRecordsQuery,
    ) -> Result<AuditRecordListResponse, AuditError> {
        let after_audit_seq = query.after_audit_seq.unwrap_or(0);
        let limit = query.limit.unwrap_or(100);
        if limit == 0 {
            return Err(AuditError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }
        if limit > AUDIT_RECORD_LIST_MAX_LIMIT {
            return Err(AuditError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!(
                    "limit must be less than or equal to {AUDIT_RECORD_LIST_MAX_LIMIT}"
                ),
            });
        }

        match &self.backend {
            AuditBackend::InMemory { records } => Ok(Self::read_records(records)
                .get(auth.tenant_id.as_str())
                .map(|tenant_records: &TenantAuditRecords| {
                    tenant_records.window(after_audit_seq, limit)
                })
                .unwrap_or_else(|| AuditRecordListResponse {
                    items: Vec::new(),
                    next_after_audit_seq: None,
                    has_more: false,
                })),
            AuditBackend::Postgres(store) => {
                store.list_records_window(auth, after_audit_seq, limit)
            }
        }
    }

    pub fn export_bundle(&self, auth: &AppContext) -> AuditExportBundle {
        let items = self.list_records(auth);
        let chain_head_hash = items.last().map(|record| record.chain_hash.clone());
        let chain_valid = verify_audit_records_chain(auth.tenant_id.as_str(), items.as_slice());
        AuditExportBundle {
            tenant_id: auth.tenant_id.clone(),
            exported_at: utc_now_rfc3339_millis(),
            total: items.len(),
            items,
            chain_head_hash,
            chain_valid,
        }
    }

    pub fn verify_chain(&self, auth: &AppContext) -> AuditChainVerification {
        let items = self.list_records(auth);
        let chain_head_hash = items.last().map(|record| record.chain_hash.clone());
        let chain_valid = verify_audit_records_chain(auth.tenant_id.as_str(), items.as_slice());
        AuditChainVerification {
            tenant_id: auth.tenant_id.clone(),
            verified_at: utc_now_rfc3339_millis(),
            total: items.len(),
            chain_head_hash,
            chain_valid,
        }
    }

    /// Construct the runtime using process environment variables.
    ///
    /// Selection rules (see `log_audit_persistence_warning`/ADR):
    /// - `SDKWORK_IM_ENVIRONMENT=dev|test` → in-memory backend.
    /// - `SDKWORK_IM_ENVIRONMENT=prod` (the default) without
    ///   `SDKWORK_IM_DATABASE_URL` → in-memory backend with an `error!` log
    ///   warning that audit records will not survive restart.
    /// - `SDKWORK_IM_ENVIRONMENT=prod` with `SDKWORK_IM_DATABASE_URL` →
    ///   PostgreSQL backend. Initialization failure is fail-closed: the
    ///   process panics rather than silently degrading to in-memory storage.
    pub fn from_env() -> Self {
        Self {
            backend: resolve_audit_backend_from_env(),
        }
    }

    fn read_records(
        records: &RwLock<HashMap<String, TenantAuditRecords>>,
    ) -> RwLockReadGuard<'_, HashMap<String, TenantAuditRecords>> {
        match records.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("recovering poisoned audit-service records read lock");
                poisoned.into_inner()
            }
        }
    }

    fn write_records(
        records: &RwLock<HashMap<String, TenantAuditRecords>>,
    ) -> RwLockWriteGuard<'_, HashMap<String, TenantAuditRecords>> {
        match records.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("recovering poisoned audit-service records write lock");
                poisoned.into_inner()
            }
        }
    }
}

fn record_anchor_in_memory(
    records: &RwLock<HashMap<String, TenantAuditRecords>>,
    auth: &AppContext,
    request: RecordAuditAnchor,
) -> Result<AuditRecordMutationOutcome, AuditError> {
    validate_record_audit_anchor_request(&request)?;
    let recorded_at = utc_now_rfc3339_millis();
    let mut records = AuditRuntime::write_records(records);
    let tenant_records = records.entry(auth.tenant_id.clone()).or_default();
    if let Some(existing) = tenant_records.get(request.record_id.as_str()).cloned() {
        if audit_record_matches_request(&existing, auth, &request) {
            return Ok(AuditRecordMutationOutcome {
                record: existing,
                applied: false,
            });
        }
        return Err(AuditError::conflict(request.record_id.as_str()));
    }
    let chain_prev_hash = tenant_records
        .last()
        .map(|record| record.chain_hash.clone());
    let next_audit_seq = tenant_records.next_audit_seq();
    let chain_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
        tenant_id: auth.tenant_id.as_str(),
        record_id: request.record_id.as_str(),
        audit_seq: next_audit_seq,
        aggregate_type: request.aggregate_type.as_str(),
        aggregate_id: request.aggregate_id.as_str(),
        action: request.action.as_str(),
        actor_id: auth.actor_id.as_str(),
        actor_kind: auth.actor_kind.as_str(),
        actor_session_id: auth.session_id.as_deref(),
        payload: request.payload.as_deref(),
        recorded_at: recorded_at.as_str(),
        chain_prev_hash: chain_prev_hash.as_deref(),
    });
    let record = AuditRecord {
        tenant_id: auth.tenant_id.clone(),
        record_id: request.record_id,
        audit_seq: next_audit_seq,
        aggregate_type: request.aggregate_type,
        aggregate_id: request.aggregate_id,
        action: request.action,
        actor_id: auth.actor_id.clone(),
        actor_kind: auth.actor_kind.clone(),
        actor_session_id: auth.session_id.clone(),
        payload: request.payload,
        recorded_at,
        chain_prev_hash,
        chain_hash,
    };
    tenant_records.push(record.clone());
    Ok(AuditRecordMutationOutcome {
        record,
        applied: true,
    })
}

// ---------------------------------------------------------------------------
// PostgreSQL durable audit backend
// ---------------------------------------------------------------------------
//
// Mirrors the `r2d2` + `r2d2_postgres` bridge pattern established by
// `adapters/postgres-journal`: a synchronous r2d2 connection pool with each
// blocking DB operation run on a dedicated OS thread via `std::thread::scope`
// so the synchronous `AuditRuntime` method signatures stay stable while the
// Postgres driver never nests Tokio runtimes.
//
// All writes serialize per (tenant_id, organization_id) through a transaction
// scoped advisory lock (`pg_advisory_xact_lock`) so the next `audit_seq` and
// the chain hash can be computed atomically. The unique constraint on
// `record_id` guarantees idempotency: a replayed request reads back the
// previously stored row and returns `applied=false` when the eight-field
// `audit_record_matches_request` predicate still holds, or surfaces
// `audit_record_conflict` otherwise.

/// TLS connector type for r2d2-backed PostgreSQL pools.
///
/// P0-12 (SECURITY_SPEC): uses `postgres-native-tls` so the `sslmode` URL
/// parameter is honored. With `sslmode=disable` the connector is never
/// invoked (plaintext TCP); with `sslmode=require` or `verify-full` a real
/// TLS handshake is performed. This allows dev/test to keep using plaintext
/// while production enforces TLS via the DSN.
type AuditPostgresTlsConnector = postgres_native_tls::MakeTlsConnector;
/// r2d2 connection manager / pool type aliases for the audit store.
type AuditPostgresConnectionManager = PostgresConnectionManager<AuditPostgresTlsConnector>;
type AuditPostgresPool = Pool<AuditPostgresConnectionManager>;

/// PostgreSQL-backed audit ledger.
#[derive(Clone)]
struct PostgresAuditStore {
    pool: AuditPostgresPool,
}

impl PostgresAuditStore {
    fn from_database_url(database_url: &str) -> Result<Self, AuditError> {
        let pool = build_audit_pool(database_url)?;
        Ok(Self { pool })
    }

    fn record_anchor_with_outcome(
        &self,
        auth: &AppContext,
        request: RecordAuditAnchor,
    ) -> Result<AuditRecordMutationOutcome, AuditError> {
        validate_record_audit_anchor_request(&request)?;
        let pool = self.pool.clone();
        let auth = auth.clone();
        run_audit_postgres_io(move || insert_audit_record(&pool, &auth, request))
    }

    fn list_records(&self, auth: &AppContext) -> Result<Vec<AuditRecord>, AuditError> {
        let pool = self.pool.clone();
        let auth = auth.clone();
        run_audit_postgres_io(move || select_all_audit_records(&pool, &auth))
    }

    fn list_records_window(
        &self,
        auth: &AppContext,
        after_audit_seq: u64,
        limit: usize,
    ) -> Result<AuditRecordListResponse, AuditError> {
        let pool = self.pool.clone();
        let auth = auth.clone();
        run_audit_postgres_io(move || {
            select_audit_records_window(&pool, &auth, after_audit_seq, limit)
        })
    }
}

const INSERT_AUDIT_RECORD_SQL: &str = r#"
insert into im_audit_records (
    tenant_id,
    organization_id,
    audit_seq,
    record_id,
    aggregate_type,
    aggregate_id,
    action,
    actor_id,
    actor_kind,
    actor_session_id,
    payload,
    recorded_at,
    chain_prev_hash,
    chain_hash
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
"#;

const SELECT_EXISTING_BY_RECORD_ID_SQL: &str = r#"
select
    tenant_id,
    record_id,
    audit_seq,
    aggregate_type,
    aggregate_id,
    action,
    actor_id,
    actor_kind,
    actor_session_id,
    payload,
    recorded_at,
    chain_prev_hash,
    chain_hash
from im_audit_records
where tenant_id = $1 and organization_id = $2 and record_id = $3
"#;

const SELECT_LAST_FOR_TENANT_SQL: &str = r#"
select audit_seq, chain_hash
from im_audit_records
where tenant_id = $1 and organization_id = $2
order by audit_seq desc
limit 1
for update
"#;

const SELECT_ALL_AUDIT_RECORDS_SQL: &str = r#"
select
    tenant_id,
    record_id,
    audit_seq,
    aggregate_type,
    aggregate_id,
    action,
    actor_id,
    actor_kind,
    actor_session_id,
    payload,
    recorded_at,
    chain_prev_hash,
    chain_hash
from im_audit_records
where tenant_id = $1 and organization_id = $2
order by audit_seq asc
"#;

const SELECT_AUDIT_RECORDS_WINDOW_SQL: &str = r#"
select
    tenant_id,
    record_id,
    audit_seq,
    aggregate_type,
    aggregate_id,
    action,
    actor_id,
    actor_kind,
    actor_session_id,
    payload,
    recorded_at,
    chain_prev_hash,
    chain_hash
from im_audit_records
where tenant_id = $1 and organization_id = $2 and audit_seq > $3
order by audit_seq asc
limit $4
"#;

const AUDIT_ADVISORY_LOCK_SQL: &str = "select pg_advisory_xact_lock(hashtext($1))";

fn insert_audit_record(
    pool: &AuditPostgresPool,
    auth: &AppContext,
    request: RecordAuditAnchor,
) -> Result<AuditRecordMutationOutcome, AuditError> {
    let mut client = audit_pool_client(pool, "audit insert")?;
    let mut txn = client
        .transaction()
        .map_err(|error| audit_db_error("audit insert begin", error))?;

    // Serialize per (tenant_id, organization_id) so the next audit_seq and
    // chain hash can be read and written atomically. `hashtext` returns int4
    // which widens to the bigint argument of `pg_advisory_xact_lock`.
    let lock_key = format!("{}:{}", auth.tenant_id, auth.organization_id);
    txn.execute(AUDIT_ADVISORY_LOCK_SQL, &[&lock_key])
        .map_err(|error| audit_db_error("audit advisory lock", error))?;

    // Idempotency check: a replayed record_id reads back the stored row and
    // returns `applied=false` when the eight-field predicate still holds.
    let existing_row = txn
        .query_opt(
            SELECT_EXISTING_BY_RECORD_ID_SQL,
            &[&auth.tenant_id, &auth.organization_id, &request.record_id],
        )
        .map_err(|error| audit_db_error("audit select existing", error))?;
    if let Some(row) = existing_row {
        let existing = row_to_audit_record(&row)?;
        if audit_record_matches_request(&existing, auth, &request) {
            txn.commit()
                .map_err(|error| audit_db_error("audit idempotent commit", error))?;
            return Ok(AuditRecordMutationOutcome {
                record: existing,
                applied: false,
            });
        }
        return Err(AuditError::conflict(request.record_id.as_str()));
    }

    // Read the current chain tip to derive the next audit_seq and prev hash.
    let last_row = txn
        .query_opt(
            SELECT_LAST_FOR_TENANT_SQL,
            &[&auth.tenant_id, &auth.organization_id],
        )
        .map_err(|error| audit_db_error("audit select last", error))?;
    let (next_audit_seq, chain_prev_hash) = match last_row {
        Some(row) => {
            let last_seq: i64 = row.get(0);
            let last_hash: String = row.get(1);
            let next = u64::try_from(last_seq)
                .ok()
                .and_then(|seq| seq.checked_add(1))
                .ok_or_else(|| {
                    AuditError::internal(
                        "audit_seq_overflow",
                        format!("audit_seq overflowed u64 (last_seq={last_seq})"),
                    )
                })?;
            (next, Some(last_hash))
        }
        None => (1u64, None),
    };

    let recorded_at = utc_now_rfc3339_millis();
    let chain_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
        tenant_id: auth.tenant_id.as_str(),
        record_id: request.record_id.as_str(),
        audit_seq: next_audit_seq,
        aggregate_type: request.aggregate_type.as_str(),
        aggregate_id: request.aggregate_id.as_str(),
        action: request.action.as_str(),
        actor_id: auth.actor_id.as_str(),
        actor_kind: auth.actor_kind.as_str(),
        actor_session_id: auth.session_id.as_deref(),
        payload: request.payload.as_deref(),
        recorded_at: recorded_at.as_str(),
        chain_prev_hash: chain_prev_hash.as_deref(),
    });
    let audit_seq_i64 = i64::try_from(next_audit_seq)
        .map_err(|_| AuditError::internal("audit_seq_overflow", "audit_seq overflowed i64"))?;

    let record = AuditRecord {
        tenant_id: auth.tenant_id.clone(),
        record_id: request.record_id,
        audit_seq: next_audit_seq,
        aggregate_type: request.aggregate_type,
        aggregate_id: request.aggregate_id,
        action: request.action,
        actor_id: auth.actor_id.clone(),
        actor_kind: auth.actor_kind.clone(),
        actor_session_id: auth.session_id.clone(),
        payload: request.payload,
        recorded_at,
        chain_prev_hash,
        chain_hash,
    };

    txn.execute(
        INSERT_AUDIT_RECORD_SQL,
        &[
            &record.tenant_id,
            &auth.organization_id,
            &audit_seq_i64,
            &record.record_id,
            &record.aggregate_type,
            &record.aggregate_id,
            &record.action,
            &record.actor_id,
            &record.actor_kind,
            &record.actor_session_id,
            &record.payload,
            &record.recorded_at,
            &record.chain_prev_hash,
            &record.chain_hash,
        ],
    )
    .map_err(|error| audit_db_error("audit insert", error))?;

    txn.commit()
        .map_err(|error| audit_db_error("audit insert commit", error))?;

    Ok(AuditRecordMutationOutcome {
        record,
        applied: true,
    })
}

fn select_all_audit_records(
    pool: &AuditPostgresPool,
    auth: &AppContext,
) -> Result<Vec<AuditRecord>, AuditError> {
    let mut client = audit_pool_client(pool, "audit select all")?;
    let rows = client
        .query(
            SELECT_ALL_AUDIT_RECORDS_SQL,
            &[&auth.tenant_id, &auth.organization_id],
        )
        .map_err(|error| audit_db_error("audit select all", error))?;
    rows.iter().map(row_to_audit_record).collect()
}

fn select_audit_records_window(
    pool: &AuditPostgresPool,
    auth: &AppContext,
    after_audit_seq: u64,
    limit: usize,
) -> Result<AuditRecordListResponse, AuditError> {
    let mut client = audit_pool_client(pool, "audit select window")?;
    let after_seq_i64 = i64::try_from(after_audit_seq).map_err(|_| {
        AuditError::internal(
            "audit_seq_overflow",
            "after_audit_seq overflowed i64",
        )
    })?;
    // Fetch limit+1 rows so `has_more` can be derived without a second query,
    // matching the in-memory `TenantAuditRecords::window` semantics.
    let fetch_count = i64::try_from(limit)
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| AuditError::internal("limit_overflow", "limit overflowed i64"))?;

    let rows = client
        .query(
            SELECT_AUDIT_RECORDS_WINDOW_SQL,
            &[
                &auth.tenant_id,
                &auth.organization_id,
                &after_seq_i64,
                &fetch_count,
            ],
        )
        .map_err(|error| audit_db_error("audit select window", error))?;

    let mut items: Vec<AuditRecord> = rows.iter().map(row_to_audit_record).collect::<Result<_, _>>()?;
    let has_more = items.len() > limit;
    if has_more {
        items.truncate(limit);
    }
    let next_after_audit_seq = items.last().map(|record| record.audit_seq);
    Ok(AuditRecordListResponse {
        items,
        next_after_audit_seq,
        has_more,
    })
}

fn row_to_audit_record(row: &r2d2_postgres::postgres::Row) -> Result<AuditRecord, AuditError> {
    let audit_seq_i64: i64 = row.get(2);
    let audit_seq = u64::try_from(audit_seq_i64).map_err(|_| {
        AuditError::internal(
            "audit_seq_overflow",
            format!("audit_seq {audit_seq_i64} from database is negative"),
        )
    })?;
    Ok(AuditRecord {
        tenant_id: row.get(0),
        record_id: row.get(1),
        audit_seq,
        aggregate_type: row.get(3),
        aggregate_id: row.get(4),
        action: row.get(5),
        actor_id: row.get(6),
        actor_kind: row.get(7),
        actor_session_id: row.get(8),
        payload: row.get(9),
        recorded_at: row.get(10),
        chain_prev_hash: row.get(11),
        chain_hash: row.get(12),
    })
}

fn build_audit_pool(database_url: &str) -> Result<AuditPostgresPool, AuditError> {
    if let Some(pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() {
        return Ok(pool);
    }
    if cfg!(test) {
        return build_audit_pool_local(database_url);
    }
    Err(AuditError::internal(
        "audit_persistence_failed",
        sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
            .err()
            .unwrap_or_else(|| "IM process database pools are not installed".to_owned()),
    ))
}

fn build_audit_pool_local(database_url: &str) -> Result<AuditPostgresPool, AuditError> {
    verify_production_sslmode(database_url);
    let pg_config = database_url.parse().map_err(|error| {
        AuditError::internal(
            "audit_persistence_failed",
            format!(
                "invalid database url ({}): {error}",
                redact_postgres_url(database_url)
            ),
        )
    })?;
    let tls = make_tls_connector().map_err(|error| {
        AuditError::internal(
            "audit_persistence_failed",
            format!(
                "postgres audit TLS connector build failed: {error}"
            ),
        )
    })?;
    let manager = PostgresConnectionManager::new(pg_config, tls);
    Pool::builder()
        .max_size(AUDIT_POSTGRES_POOL_MAX_SIZE)
        .build(manager)
        .map_err(|error| {
            AuditError::internal(
                "audit_persistence_failed",
                format!(
                    "failed to create audit pool ({}): {error}",
                    redact_postgres_url(database_url)
                ),
            )
        })
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

fn audit_pool_client(
    pool: &AuditPostgresPool,
    action: &'static str,
) -> Result<r2d2::PooledConnection<AuditPostgresConnectionManager>, AuditError> {
    pool.get().map_err(|error| {
        AuditError::internal(
            "audit_persistence_failed",
            format!("postgres audit {action} pool acquire failed: {error}"),
        )
    })
}

/// Bridge a blocking PostgreSQL operation off the async runtime.
///
/// Mirrors `adapters/postgres-journal::run_postgres_io`: synchronous postgres
/// driver work runs on a dedicated OS thread via `std::thread::scope` so the
/// blocking `postgres` crate never nests Tokio runtimes.
fn run_audit_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, AuditError> + Send,
) -> Result<T, AuditError>
where
    T: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(operation)
            .join()
            .map_err(|_| {
                AuditError::internal(
                    "audit_persistence_failed",
                    "postgres audit blocking IO worker panicked",
                )
            })?
    })
}

fn audit_db_error(action: &'static str, error: r2d2_postgres::postgres::Error) -> AuditError {
    AuditError::internal(
        "audit_persistence_failed",
        format!(
            "postgres audit {action} failed: {}",
            format_audit_db_error(&error)
        ),
    )
}

fn format_audit_db_error(error: &r2d2_postgres::postgres::Error) -> String {
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

/// Redact credentials from a PostgreSQL connection URL before it enters an
/// error message or log line. Mirrors `adapters/postgres-journal::redact_postgres_url`.
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

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), AuditError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(AuditError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }
    Ok(())
}

pub fn validate_record_audit_anchor_request(request: &RecordAuditAnchor) -> Result<(), AuditError> {
    validate_payload_size(
        "recordId",
        request.record_id.as_str(),
        AUDIT_RECORD_ID_MAX_BYTES,
    )?;
    validate_payload_size(
        "aggregateType",
        request.aggregate_type.as_str(),
        AUDIT_AGGREGATE_TYPE_MAX_BYTES,
    )?;
    validate_payload_size(
        "aggregateId",
        request.aggregate_id.as_str(),
        AUDIT_AGGREGATE_ID_MAX_BYTES,
    )?;
    validate_payload_size("action", request.action.as_str(), AUDIT_ACTION_MAX_BYTES)?;
    if let Some(payload) = request.payload.as_deref() {
        validate_payload_size("payload", payload, AUDIT_PAYLOAD_MAX_BYTES)?;
    }
    Ok(())
}

pub fn verify_audit_export_bundle_integrity(bundle: &AuditExportBundle) -> bool {
    if bundle.total != bundle.items.len() {
        return false;
    }

    let actual_chain_valid = verify_audit_records_chain(bundle.tenant_id.as_str(), &bundle.items);
    if bundle.chain_valid != actual_chain_valid {
        return false;
    }

    let actual_chain_head_hash = bundle.items.last().map(|record| record.chain_hash.clone());
    if bundle.chain_head_hash != actual_chain_head_hash {
        return false;
    }

    actual_chain_valid
}

fn verify_audit_records_chain(tenant_id: &str, items: &[AuditRecord]) -> bool {
    let mut previous_hash: Option<&str> = None;

    for item in items {
        if item.tenant_id != tenant_id {
            return false;
        }
        if item.chain_prev_hash.as_deref() != previous_hash {
            return false;
        }

        let expected_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
            tenant_id: item.tenant_id.as_str(),
            record_id: item.record_id.as_str(),
            audit_seq: item.audit_seq,
            aggregate_type: item.aggregate_type.as_str(),
            aggregate_id: item.aggregate_id.as_str(),
            action: item.action.as_str(),
            actor_id: item.actor_id.as_str(),
            actor_kind: item.actor_kind.as_str(),
            actor_session_id: item.actor_session_id.as_deref(),
            payload: item.payload.as_deref(),
            recorded_at: item.recorded_at.as_str(),
            chain_prev_hash: previous_hash,
        });
        if item.chain_hash != expected_hash {
            return false;
        }

        previous_hash = Some(item.chain_hash.as_str());
    }

    true
}

struct AuditRecordHashInput<'a> {
    tenant_id: &'a str,
    record_id: &'a str,
    audit_seq: u64,
    aggregate_type: &'a str,
    aggregate_id: &'a str,
    action: &'a str,
    actor_id: &'a str,
    actor_kind: &'a str,
    actor_session_id: Option<&'a str>,
    payload: Option<&'a str>,
    recorded_at: &'a str,
    chain_prev_hash: Option<&'a str>,
}

fn compute_audit_record_chain_hash(input: AuditRecordHashInput<'_>) -> String {
    let canonical = serde_json::json!([
        input.tenant_id,
        input.record_id,
        input.audit_seq,
        input.aggregate_type,
        input.aggregate_id,
        input.action,
        input.actor_id,
        input.actor_kind,
        input.actor_session_id.unwrap_or(""),
        input.payload.unwrap_or(""),
        input.recorded_at,
        input.chain_prev_hash.unwrap_or(""),
    ]);
    let canonical_bytes = serde_json::to_vec(&canonical).unwrap_or_default();
    sha256_hash(&canonical_bytes)
}

const AUDIT_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

/// Resolve the audit backend from process environment variables.
///
/// Selection rules:
/// - `SDKWORK_IM_ENVIRONMENT=dev|test` → in-memory backend (info! log).
/// - `SDKWORK_IM_ENVIRONMENT=prod` (the default) without
///   `SDKWORK_IM_DATABASE_URL` → fail-closed startup panic.
/// - `SDKWORK_IM_ENVIRONMENT=prod` with `SDKWORK_IM_DATABASE_URL` →
///   PostgreSQL backend. Initialization failure is fail-closed: the process
///   panics rather than silently degrading to in-memory storage.
fn resolve_audit_backend_from_env() -> AuditBackend {
    let environment = resolve_web_environment_from_process_env();
    let database_url = std::env::var(AUDIT_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    match (environment, database_url) {
        (WebEnvironment::Dev | WebEnvironment::Test, _) => {
            info!("audit-service using in-memory audit ledger (development/test)");
            AuditBackend::InMemory {
                records: RwLock::new(HashMap::new()),
            }
        }
        (WebEnvironment::Prod, None) => {
            error!(
                env = "SDKWORK_IM_ENVIRONMENT=prod",
                hint = "set SDKWORK_IM_DATABASE_URL to enable durable audit storage",
                "audit-service fail-closed: production requires durable audit storage"
            );
            panic!(
                "audit-service fail-closed: set {AUDIT_DATABASE_URL_ENV} for durable audit storage in production"
            );
        }
        (WebEnvironment::Prod, Some(database_url)) => {
            match PostgresAuditStore::from_database_url(database_url.as_str()) {
                Ok(store) => {
                    info!("audit-service using PostgreSQL-backed durable audit ledger (production)");
                    AuditBackend::Postgres(store)
                }
                Err(error) => {
                    let redacted = redact_postgres_url(database_url.as_str());
                    error!(
                        code = error.code,
                        message = %error.message,
                        database_url = %redacted,
                        "audit-service failed to initialize PostgreSQL backend in production; refusing to start (fail-closed)"
                    );
                    panic!(
                        "audit-service fail-closed: PostgreSQL backend initialization failed ({redacted}): {}",
                        error.message
                    );
                }
            }
        }
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(AuditRuntime::from_env()),
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(AuditRuntime::from_env()))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/backend/v3/api/audit/records", post(record_anchor).get(list_records))
        .route("/backend/v3/api/audit/export", get(export_bundle))
        .route("/backend/v3/api/audit/verify", get(verify_chain))
        .with_state(state)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(Arc::new(AuditRuntime::from_env()))),
        im_service_router_config(),
    )
}

pub fn build_app(runtime: Arc<AuditRuntime>) -> Router {
    mount_im_infra_routes(build_business_router(runtime), im_service_router_config())
}

pub fn build_business_router(runtime: Arc<AuditRuntime>) -> Router {
    let state = AppState { runtime };
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state))
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if matches!(
        request.uri().path(),
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json" | "/docs"
    ) {
        return next.run(request).await;
    }
    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            let problem = ApiProblem::dependency_unavailable(
                "server is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return AuditError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message:
                    "server is at maximum in-flight request capacity, please retry later".to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

async fn openapi_json() -> Result<Json<serde_json::Value>, AuditError> {
    Ok(Json(build_audit_service_openapi_document().map_err(
        |message| AuditError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&audit_service_openapi_spec()))
}

fn build_audit_service_openapi_document() -> Result<serde_json::Value, String> {
    // Extract routes from `build_domain_api_router` (not `build_business_router`)
    // because the business router only `.merge()`s the domain router; the
    // extractor follows direct `.route()` calls and does not recurse into
    // merged sub-routers. The domain router owns the audit endpoints.
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_domain_api_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &audit_service_openapi_spec(),
        &routes,
        audit_service_tag,
        audit_service_requires_app_context,
        audit_service_summary,
    ))
}

fn audit_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Audit Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the audit-service router for audit record mutation, export, verification, and record listing flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn audit_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "audit".to_owned(),
    }
}

fn audit_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn audit_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check audit service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check audit service readiness".to_owned(),
        _ => format!(
            "{} {}",
            audit_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn audit_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

async fn record_anchor(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Response {
    let result: ApiResult<AuditRecordMutationResponse> = (|| {
        ensure_audit_write_access(&auth)?;
        validate_record_audit_anchor_request(&request)?;
        let request_key = audit_record_request_key(&auth, request.record_id.as_str());
        Ok(AuditRecordMutationResponse::from_outcome(
            state.runtime.record_anchor_with_outcome(&auth, request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}

async fn list_records(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ListAuditRecordsQuery>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<AuditRecordListResponse> = (|| {
        ensure_audit_read_access(&auth)?;
        Ok(state.runtime.list_records_window(&auth, query)?)
    })();
    finish_api_json(&ctx, result)
}

async fn export_bundle(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<AuditExportBundle> = (|| {
        ensure_audit_read_access(&auth)?;
        Ok(state.runtime.export_bundle(&auth))
    })();
    finish_api_json(&ctx, result)
}

async fn verify_chain(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<AuditChainVerification> = (|| {
        ensure_audit_read_access(&auth)?;
        Ok(state.runtime.verify_chain(&auth))
    })();
    finish_api_json(&ctx, result)
}

fn ensure_audit_read_access(auth: &AppContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.read"))
}

fn ensure_audit_write_access(auth: &AppContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.write"))
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(AUDIT_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUDIT_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(AUDIT_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(AUDIT_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUDIT_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(AUDIT_MAX_REQUEST_BODY_BYTES_MAX)
}

pub fn audit_record_request_key(auth: &AppContext, record_id: &str) -> String {
    format!("{}:audit-record:{}", auth.tenant_id, record_id)
}

fn audit_record_matches_request(
    existing: &AuditRecord,
    auth: &AppContext,
    request: &RecordAuditAnchor,
) -> bool {
    existing.tenant_id == auth.tenant_id
        && existing.record_id == request.record_id
        && existing.aggregate_type == request.aggregate_type
        && existing.aggregate_id == request.aggregate_id
        && existing.action == request.action
        && existing.actor_id == auth.actor_id
        && existing.actor_kind == auth.actor_kind
        && existing.payload == request.payload
}
