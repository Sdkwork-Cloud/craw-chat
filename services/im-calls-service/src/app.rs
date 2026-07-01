use std::sync::Arc;

use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use im_app_context::allows_header_only_app_context_fallback;
use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresOutboxStore};
use sdkwork_utils_rust::parse_bool;
use im_adapters_postgres_rtc_state::build_postgres_rtc_state_store_optional;
use im_adapters_redis_cache::rtc_state_store::build_redis_rtc_state_store_optional;
use im_domain_core::audit::{AuditEmitter, LoggingAuditEmitter};
use im_domain_core::rtc::StateStore;
use im_platform_contracts::{IdGenerator, OutboxStore};
use sdkwork_communication_rtc_service::RtcProviderPort;
use sdkwork_rtc_adapter_volcengine::VolcengineRtcProvider;
use tokio::sync::Semaphore;

use crate::error::CallingError;
use crate::handlers::{
    accept_call_session, create_call_session, end_call_session, invite_call_session,
    issue_participant_credential, post_call_signal, refresh_participant_credential,
    reject_call_session, retrieve_call_session,
};
use crate::helpers::{resolve_max_http_request_body_bytes, resolve_max_in_flight_requests};
use crate::openapi::{docs, openapi_json};
use crate::state::{AppState, CallingRuntime, LocalCounterIdGenerator, RuntimeMemoryStateStore};

/// Environment variable for the PostgreSQL RTC state database URL.
const ENV_RTC_STATE_DATABASE_URL: &str = "SDKWORK_RTC_STATE_DATABASE_URL";
/// Environment variable for the Redis RTC state cache URL (optional).
const ENV_RTC_STATE_REDIS_URL: &str = "SDKWORK_RTC_STATE_REDIS_URL";
/// Environment variable to require durable storage in production.
/// When `true` or `1`, missing/failed PostgreSQL connection aborts startup.
const ENV_RTC_STATE_REQUIRE_DURABLE: &str = "SDKWORK_RTC_STATE_REQUIRE_DURABLE";
/// Environment variable for the Volcengine RTC App ID (required for real
/// credential issuance).
const ENV_RTC_VOLCENGINE_APP_ID: &str = "SDKWORK_RTC_VOLCENGINE_APP_ID";
/// Environment variable for the Volcengine RTC App Key (required for real
/// credential issuance).
const ENV_RTC_VOLCENGINE_APP_KEY: &str = "SDKWORK_RTC_VOLCENGINE_APP_KEY";
/// Environment variable to require a properly configured RTC provider in
/// production. When `true` or `1`, missing provider credentials abort
/// startup instead of falling back to signaling-only mode.
const ENV_RTC_REQUIRE_PROVIDER: &str = "SDKWORK_RTC_REQUIRE_PROVIDER";
/// Environment variable for the PostgreSQL outbox database URL (durable
/// event publishing). When unset, lifecycle events are not enqueued.
const ENV_RTC_OUTBOX_DATABASE_URL: &str = "SDKWORK_RTC_OUTBOX_DATABASE_URL";
/// Environment variable to require a wired outbox store in production.
/// When `true` or `1`, missing outbox configuration aborts startup.
const ENV_RTC_REQUIRE_OUTBOX: &str = "SDKWORK_RTC_REQUIRE_OUTBOX";

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

/// Check whether the Volcengine RTC provider has signing credentials
/// configured (App ID + App Key). When this returns `false`, the provider
/// would issue development-placeholder credentials, which must never reach
/// production clients.
fn rtc_provider_signing_configured() -> bool {
    let app_id = std::env::var(ENV_RTC_VOLCENGINE_APP_ID)
        .ok()
        .filter(|v| !v.trim().is_empty());
    let app_key = std::env::var(ENV_RTC_VOLCENGINE_APP_KEY)
        .ok()
        .filter(|v| !v.trim().is_empty());
    app_id.is_some() && app_key.is_some()
}

/// Build the default RTC provider (`sdkwork-rtc` Volcengine adapter).
///
/// Returns `Some(provider)` when the Volcengine signing credentials
/// (`SDKWORK_RTC_VOLCENGINE_APP_ID` + `SDKWORK_RTC_VOLCENGINE_APP_KEY`)
/// are configured, enabling real media session creation and participant
/// credential issuance.
///
/// Returns `None` (signaling-only mode) when credentials are absent,
/// allowing development/testing without a real Volcengine account. In
/// this mode, `create_session` returns provider fields but
/// `issue_participant_credential` would return placeholder credentials —
/// so the runtime skips wiring the provider entirely, and credential
/// issuance returns a clear error instead of placeholder data.
///
/// ## Production fail-closed
///
/// When `SDKWORK_RTC_REQUIRE_PROVIDER=true` (or `1`), the function
/// panics at startup if signing credentials are missing. This prevents
/// the service from booting into signaling-only mode in production,
/// where clients would receive credential-issuance errors instead of
/// real media access.
///
/// Production deployments SHOULD set:
/// - `SDKWORK_RTC_VOLCENGINE_APP_ID=...`
/// - `SDKWORK_RTC_VOLCENGINE_APP_KEY=...`
/// - `SDKWORK_RTC_REQUIRE_PROVIDER=true`
///
/// This is the IM → RTC boundary defined in
/// `../sdkwork-rtc/docs/rtc-im-boundary.md`: IM owns call signaling, RTC
/// owns media session creation and participant credential issuance.
pub fn build_default_rtc_provider() -> Option<Arc<dyn RtcProviderPort>> {
    if !rtc_provider_signing_configured() {
        let require_provider = std::env::var(ENV_RTC_REQUIRE_PROVIDER)
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);
        if require_provider {
            panic!(
                "{ENV_RTC_REQUIRE_PROVIDER}=true but Volcengine RTC provider signing credentials \
                 are missing. Set {ENV_RTC_VOLCENGINE_APP_ID} and {ENV_RTC_VOLCENGINE_APP_KEY}. \
                 Aborting startup to prevent serving placeholder credentials in production."
            );
        }
        tracing::warn!(
            "RTC provider not configured (missing {ENV_RTC_VOLCENGINE_APP_ID}/{ENV_RTC_VOLCENGINE_APP_KEY}). \
             Running in signaling-only mode: credential issuance will return an error. \
             Set {ENV_RTC_REQUIRE_PROVIDER}=true to fail-closed in production."
        );
        return None;
    }
    tracing::info!("RTC provider: Volcengine (signing configured)");
    Some(Arc::new(VolcengineRtcProvider::default()))
}

/// Resolve the durable RTC state store from environment configuration.
///
/// ## Priority
///
/// 1. **PostgreSQL** (`SDKWORK_RTC_STATE_DATABASE_URL`) — durable source
///    of truth with epoch-based fencing via `SELECT ... FOR UPDATE`.
/// 2. **Redis** (`SDKWORK_RTC_STATE_REDIS_URL`) — hot-path cache with
///    Lua-atomic epoch fencing. Not durable; data is lost on Redis
///    restart or TTL expiry.
/// 3. **In-memory** (`RuntimeMemoryStateStore`) — development only.
///    State is lost on process restart.
///
/// ## Production fail-closed
///
/// When `SDKWORK_RTC_STATE_REQUIRE_DURABLE=true` (or `1`), the function
/// panics at startup if PostgreSQL is unavailable. This prevents silent
/// data loss in production by refusing to boot into a non-durable mode.
///
/// Production deployments SHOULD set:
/// - `SDKWORK_RTC_STATE_DATABASE_URL=postgres://...`
/// - `SDKWORK_RTC_STATE_REDIS_URL=redis://...` (optional, for hot cache)
/// - `SDKWORK_RTC_STATE_REQUIRE_DURABLE=true`
fn build_default_state_store() -> Arc<dyn StateStore> {
    let require_durable = std::env::var(ENV_RTC_STATE_REQUIRE_DURABLE)
        .ok()
        .and_then(|value| parse_bool(value.as_str()))
        .unwrap_or(!allows_header_only_app_context_fallback());

    let pg_url = std::env::var(ENV_RTC_STATE_DATABASE_URL).ok();
    let redis_url = std::env::var(ENV_RTC_STATE_REDIS_URL).ok();

    // Priority 1: PostgreSQL (durable source of truth).
    if let Some(store) = build_postgres_rtc_state_store_optional(pg_url.as_deref()) {
        tracing::info!("RTC state store: PostgreSQL (durable)");
        return store as Arc<dyn StateStore>;
    }

    // Fail-closed: if durable is required but PostgreSQL is unavailable,
    // abort startup instead of silently falling back to non-durable stores.
    if require_durable {
        panic!(
            "{ENV_RTC_STATE_REQUIRE_DURABLE}=true but PostgreSQL RTC state store is unavailable. \
             Set {ENV_RTC_STATE_DATABASE_URL} to a valid PostgreSQL connection string. \
             Aborting startup to prevent data loss."
        );
    }

    // Priority 2: Redis (hot cache, not durable).
    if let Some(store) = build_redis_rtc_state_store_optional(redis_url.as_deref()) {
        tracing::warn!(
            "RTC state store: Redis (cache-only, NOT durable). \
             Data loss on Redis restart or TTL expiry. \
             Set {ENV_RTC_STATE_DATABASE_URL} for production durability."
        );
        return store as Arc<dyn StateStore>;
    }

    // Priority 3: In-memory (development only).
    tracing::warn!(
        "RTC state store: in-memory (development only). \
         State is lost on process restart. \
         Set {ENV_RTC_STATE_DATABASE_URL} for production durability."
    );
    Arc::new(RuntimeMemoryStateStore::default())
}

/// Resolve the transactional outbox store from environment configuration.
///
/// When `SDKWORK_RTC_OUTBOX_DATABASE_URL` is set, constructs a
/// `PostgresOutboxStore` backed by an r2d2 connection pool. The outbox
/// implements the `FOR UPDATE SKIP LOCKED` drain pattern for
/// multi-worker concurrent event delivery.
///
/// Returns `None` when the env var is unset, disabling outbox emission
/// (development mode). Production deployments SHOULD set
/// `SDKWORK_RTC_REQUIRE_OUTBOX=true` to fail-closed on missing outbox.
fn build_default_outbox_store_optional() -> Option<Arc<dyn OutboxStore>> {
    let pg_url = std::env::var(ENV_RTC_OUTBOX_DATABASE_URL).ok()?;
    let config = PostgresJournalConfig::new(pg_url);
    let pool = match config.connect_pool() {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!(
                error = %format!("{e:?}"),
                "RTC outbox PostgreSQL pool initialization failed"
            );
            return None;
        }
    };
    let store = PostgresOutboxStore::from_pool(pool);
    tracing::info!("RTC outbox store: PostgreSQL (durable event publishing)");
    Some(Arc::new(store) as Arc<dyn OutboxStore>)
}

/// Resolve the audit emitter from environment configuration.
///
/// Defaults to `LoggingAuditEmitter` which ships audit events to the
/// application log pipeline (SIEM-compatible). This is suitable for
/// production deployments that ship logs to Elasticsearch/Splunk/Loki.
///
/// Production deployments requiring tamper-evident audit storage SHOULD
/// wire a dedicated emitter (e.g. `PostgresAuditEmitter` writing to
/// `im_audit_records`) via [`CallingRuntime::with_audit_emitter`].
fn build_default_audit_emitter() -> Arc<dyn AuditEmitter> {
    tracing::info!("RTC audit emitter: LoggingAuditEmitter (SIEM-compatible log pipeline)");
    Arc::new(LoggingAuditEmitter)
}

/// Resolve the Snowflake ID generator from environment configuration.
///
/// Defaults to `LocalCounterIdGenerator` (in-process atomic counter) which
/// is suitable for single-process development/testing only. Production
/// deployments MUST wire `RuntimeSnowflakeIdGenerator` (or equivalent)
/// via [`CallingRuntime::with_id_generator`] to guarantee cross-process
/// uniqueness for audit event IDs and outbox outbox IDs.
fn build_default_id_generator() -> Arc<dyn IdGenerator> {
    tracing::warn!(
        "RTC id generator: LocalCounterIdGenerator (in-process only). \
         Production deployments MUST wire RuntimeSnowflakeIdGenerator via \
         CallingRuntime::with_id_generator for cross-process uniqueness."
    );
    Arc::new(LocalCounterIdGenerator::default())
}

/// Fail-closed check for required outbox in production deployments.
fn enforce_require_outbox(outbox: Option<Arc<dyn OutboxStore>>) -> Option<Arc<dyn OutboxStore>> {
    let require_outbox = std::env::var(ENV_RTC_REQUIRE_OUTBOX)
        .ok()
        .and_then(|value| parse_bool(value.as_str()))
        .unwrap_or(!allows_header_only_app_context_fallback());
    if outbox.is_none() && require_outbox {
        panic!(
            "{ENV_RTC_REQUIRE_OUTBOX}=true but RTC outbox store is unavailable. \
             Set {ENV_RTC_OUTBOX_DATABASE_URL} to a valid PostgreSQL connection string. \
             Aborting startup to prevent silent event loss in production."
        );
    }
    outbox
}

/// Build a `CallingRuntime` wired with the environment-resolved state
/// store, RTC provider, outbox store, audit emitter, and ID generator.
///
/// Uses [`build_default_state_store`] to resolve the persistence layer
/// from `SDKWORK_RTC_STATE_DATABASE_URL` / `SDKWORK_RTC_STATE_REDIS_URL`.
/// Uses [`build_default_rtc_provider`] to resolve the RTC provider from
/// `SDKWORK_RTC_VOLCENGINE_APP_ID` / `SDKWORK_RTC_VOLCENGINE_APP_KEY`.
/// Uses [`build_default_outbox_store_optional`] to resolve the outbox
/// from `SDKWORK_RTC_OUTBOX_DATABASE_URL`.
/// Uses [`build_default_audit_emitter`] for SIEM-compatible audit emission.
/// Uses [`build_default_id_generator`] for in-process ID generation (dev).
///
/// When the RTC provider is not configured, the runtime operates in
/// signaling-only mode: `create_session` works but
/// `issue_participant_credential` returns a clear error. Production
/// deployments MUST set `SDKWORK_RTC_REQUIRE_PROVIDER=true` to fail-closed.
///
/// When the outbox is not configured, lifecycle events are not enqueued
/// (development mode). Production deployments SHOULD set
/// `SDKWORK_RTC_REQUIRE_OUTBOX=true` to fail-closed on missing outbox.
pub fn build_default_calling_runtime() -> CallingRuntime {
    let runtime = CallingRuntime::with_store(build_default_state_store())
        .with_outbox_store(enforce_require_outbox(build_default_outbox_store_optional()))
        .with_audit_emitter(build_default_audit_emitter())
        .with_id_generator(build_default_id_generator());
    match build_default_rtc_provider() {
        Some(provider) => runtime.with_rtc_provider(provider),
        None => runtime,
    }
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/im/v3/api/calls/sessions", post(create_call_session))
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}",
            get(retrieve_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/invite",
            post(invite_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/accept",
            post(accept_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/reject",
            post(reject_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/end",
            post(end_call_session),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/signals",
            post(post_call_signal),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/credentials",
            post(issue_participant_credential),
        )
        .route(
            "/im/v3/api/calls/sessions/{rtc_session_id}/credentials/refresh",
            post(refresh_participant_credential),
        )
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

pub fn build_business_router(runtime: Arc<CallingRuntime>) -> Router {
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
            let problem = sdkwork_routes_web_framework_backend_api::response::ApiProblem::dependency_unavailable(
                "server is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<sdkwork_web_core::WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return CallingError {
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