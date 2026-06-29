//! High-performance concurrent call signaling runtime using lock-free data structures.
//!
//! ## Architecture
//!
//! This module implements a lock-free, epoch-based fencing architecture for call signaling:
//!
//! - **DashMap**: Concurrent HashMap that allows lock-free reads and fine-grained write locking
//! - **Epoch-based fencing**: Each session has an epoch that increments on state transitions,
//!   preventing stale concurrent writes from corrupting state
//! - **Atomic persistence**: State changes are persisted before being applied to memory,
//!   ensuring crash consistency
//!
//! ## Thread Safety
//!
//! - `sessions: DashMap<String, Session>` - Lock-free concurrent access to session state
//! - `signals: DashMap<String, BTreeMap<u64, SignalEvent>>` - Lock-free signal storage
//! - `epoch_counter: AtomicU64` - Global epoch allocator with atomic increment
//!
//! ## RTC Provider Integration
//!
//! When an `RtcProviderPort` handle is wired in, `create_session` delegates media session
//! creation to the RTC provider plugin (`sdkwork-rtc`), which returns the real
//! `provider_session_id`, `access_endpoint`, and `provider_region`. Participant
//! credentials are then issued by the provider rather than synthesized locally.

use std::collections::BTreeMap;
use std::ops::Bound::Excluded;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use im_app_context::AppContext;
use im_domain_core::audit::{
    AuditActorType, AuditEmitter, AuditEvent, AuditEventType, AuditOutcome, LoggingAuditEmitter,
};
use im_domain_core::rtc::{
    Session, SessionEpoch, SessionParticipants, SessionState, SignalEvent,
    StateRecord, StateStore, SignalRateTracker,
};
use im_platform_contracts::{
    IdGenerator, OutboxEventRecord, OutboxPublishStatus, OutboxStore,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_communication_rtc_service::{
    RtcContractError, RtcCreateMediaSessionRequest, RtcMediaSessionMode, RtcParticipantCredential,
    RtcProviderPort, RtcSessionHandle,
};
use sdkwork_im_contract_core::ContractError;
use sdkwork_utils_rust::sha256_hash;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, PostRtcSignalRequest,
    SessionMutationOutcome, UpdateRtcSessionRequest,
};
use crate::error::CallingError;
use crate::helpers::{
    rtc_session_scope_key, validate_create_request_payload_size,
    validate_invite_request_payload_size, validate_participant_ids_payload_size,
    validate_post_signal_request_payload_size, validate_update_request_payload_size,
};

const SIGNAL_LIST_MAX_LIMIT: usize = 1000;
/// Aggregate type for RTC session outbox events.
const OUTBOX_AGGREGATE_TYPE: &str = "rtc_session";

/// High-performance concurrent call signaling runtime.
///
/// Uses DashMap for lock-free concurrent access and epoch-based fencing
/// to prevent stale concurrent writes from corrupting state.
#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<CallingRuntime>,
}

/// Concurrent call signaling runtime with lock-free data structures.
///
/// # Concurrency Model
///
/// - **Reads**: Lock-free via DashMap's sharded concurrent access
/// - **Writes**: Fine-grained per-key locking via DashMap's entry API
/// - **Epochs**: Atomic counter for fencing, avoiding global coordination
///
/// # Persistence Model
///
/// State changes follow write-ahead persistence pattern:
/// 1. Increment epoch atomically
/// 2. Persist new state to state_store
/// 3. Apply to in-memory DashMap
///
/// If persistence fails, the epoch is consumed but state is not applied.
/// This is safe because epochs are monotonic and never reused.
pub struct CallingRuntime {
    /// Concurrent session storage with per-key fine-grained locking.
    /// Key: `rtc_session_scope_key(tenant_id, rtc_session_id)`
    pub(crate) sessions: DashMap<String, Session>,
    /// Concurrent signal storage with per-key fine-grained locking.
    /// Key: `rtc_session_scope_key(tenant_id, rtc_session_id)`
    /// Value: BTreeMap ordered by signal_seq for efficient range queries.
    pub(crate) signals: DashMap<String, BTreeMap<u64, SignalEvent>>,
    /// Durable state persistence layer.
    pub(crate) state_store: Arc<dyn StateStore>,
    /// Optional RTC provider handle for real media integration.
    /// When `None`, the runtime operates in signaling-only mode (provider
    /// fields stay `None`); when `Some`, media session creation and
    /// participant credential issuance delegate to the RTC provider plugin
    /// (`sdkwork-rtc`) via `RtcProviderPort`.
    pub(crate) rtc_provider: Option<Arc<dyn RtcProviderPort>>,
    /// Optional transactional outbox for durable event publishing.
    /// When `None`, lifecycle events are not enqueued for downstream
    /// consumers (development only). Production deployments SHOULD wire a
    /// `PostgresOutboxStore` via `with_outbox_store` and set
    /// `SDKWORK_RTC_REQUIRE_OUTBOX=true` to fail-closed on missing outbox.
    pub(crate) outbox_store: Option<Arc<dyn OutboxStore>>,
    /// Audit emitter for security-relevant and mutation audit events.
    /// Defaults to `LoggingAuditEmitter` which ships events to the
    /// application log pipeline (SIEM-compatible). Never blocks
    /// user-facing operations: emission failures are logged and swallowed.
    pub(crate) audit_emitter: Arc<dyn AuditEmitter>,
    /// Snowflake ID generator for audit event IDs and outbox outbox IDs.
    /// Defaults to an in-process atomic counter (development only).
    /// Production deployments SHOULD wire `RuntimeSnowflakeIdGenerator`
    /// via `with_id_generator` for cross-process uniqueness.
    pub(crate) id_generator: Arc<dyn IdGenerator>,
    /// Global epoch counter for fencing tokens.
    /// Monotonically increasing, never reused.
    epoch_counter: AtomicU64,
}

impl CallingRuntime {
    pub fn with_store(state_store: Arc<dyn StateStore>) -> Self {
        Self {
            sessions: DashMap::new(),
            signals: DashMap::new(),
            state_store,
            rtc_provider: None,
            outbox_store: None,
            audit_emitter: Arc::new(LoggingAuditEmitter),
            id_generator: Arc::new(LocalCounterIdGenerator::default()),
            epoch_counter: AtomicU64::new(1),
        }
    }

    /// Wire a real RTC provider plugin (`sdkwork-rtc`) for media session
    /// creation and participant credential issuance.
    ///
    /// When this is set, `create_session` delegates to the provider to
    /// obtain `provider_session_id` / `access_endpoint` / `provider_region`,
    /// and `issue_participant_credential` returns real provider-issued
    /// credentials instead of locally synthesized ones.
    pub fn with_rtc_provider(mut self, rtc_provider: Arc<dyn RtcProviderPort>) -> Self {
        self.rtc_provider = Some(rtc_provider);
        self
    }

    /// Wire a transactional outbox store for durable event publishing.
    ///
    /// When set, session lifecycle events (created, invited, accepted,
    /// rejected, ended, signal posted) are enqueued for at-least-once
    /// delivery to downstream consumers via the `FOR UPDATE SKIP LOCKED`
    /// drain pattern. Enqueue is best-effort: the durable state has
    /// already been persisted when enqueue runs, so a failure logs a
    /// warning but does not roll back the state transition.
    pub fn with_outbox_store(mut self, outbox_store: Option<Arc<dyn OutboxStore>>) -> Self {
        self.outbox_store = outbox_store;
        self
    }

    /// Wire an audit emitter for security-relevant and mutation audit
    /// events.
    ///
    /// Defaults to `LoggingAuditEmitter`. Production deployments requiring
    /// tamper-evident audit storage SHOULD wire a dedicated emitter
    /// (e.g. `PostgresAuditEmitter` writing to `im_audit_records`).
    pub fn with_audit_emitter(mut self, audit_emitter: Arc<dyn AuditEmitter>) -> Self {
        self.audit_emitter = audit_emitter;
        self
    }

    /// Wire a Snowflake ID generator for audit event IDs and outbox
    /// outbox IDs.
    ///
    /// Defaults to `LocalCounterIdGenerator` (in-process atomic counter).
    /// Production deployments MUST wire a cross-process-unique generator
    /// such as `RuntimeSnowflakeIdGenerator` to prevent ID collisions
    /// across service instances.
    pub fn with_id_generator(mut self, id_generator: Arc<dyn IdGenerator>) -> Self {
        self.id_generator = id_generator;
        self
    }

    /// Allocate a new epoch atomically.
    ///
    /// Epochs are monotonically increasing and never reused,
    /// ensuring fencing tokens are globally unique.
    fn allocate_epoch(&self) -> SessionEpoch {
        self.epoch_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Enqueue a session lifecycle event into the transactional outbox.
    ///
    /// Best-effort: the durable state has already been persisted when this
    /// runs, so an enqueue failure logs a warning but does NOT roll back
    /// the state transition. Downstream consumers will miss the event, but
    /// state consistency is preserved. The trade-off is documented in
    /// `docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md`.
    ///
    /// Idempotency: the `event_id` is derived from `outbox_id` (Snowflake),
    /// which is globally unique, so replays produce distinct outbox rows
    /// rather than colliding. The `uk_im_outbox_events_event` unique
    /// constraint on `(tenant_id, organization_id, event_id)` prevents
    /// duplicate delivery if the same outbox row is enqueued twice.
    fn enqueue_outbox_event(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) {
        let Some(outbox) = self.outbox_store.as_ref() else {
            return;
        };

        let payload_json = match serde_json::to_string(&payload) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(
                    target: "sdkwork.im.calls.outbox",
                    error = %e,
                    event_type,
                    rtc_session_id,
                    "rtc outbox payload serialize failed; event dropped"
                );
                return;
            }
        };
        let payload_hash = sha256_hash(payload_json.as_bytes());
        let now = utc_now_rfc3339_millis();
        let outbox_id = match self.id_generator.next_id() {
            Ok(id) => id.to_string(),
            Err(e) => {
                tracing::warn!(
                    target: "sdkwork.im.calls.outbox",
                    error = %format!("{e:?}"),
                    event_type,
                    rtc_session_id,
                    "rtc outbox id allocation failed; event dropped"
                );
                return;
            }
        };
        // event_id is globally unique (derived from Snowflake outbox_id) so
        // the `uk_im_outbox_events_event` constraint cannot reject legitimate
        // lifecycle events even under retry.
        let event_id = format!("rtc:{event_type}:{outbox_id}");

        let record = OutboxEventRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            outbox_id,
            aggregate_type: OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: rtc_session_id.into(),
            event_id,
            event_type: event_type.into(),
            payload_json,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        if let Err(e) = outbox.enqueue(record) {
            tracing::warn!(
                target: "sdkwork.im.calls.outbox",
                error = %format!("{e:?}"),
                event_type,
                rtc_session_id,
                tenant_id = %auth.tenant_id,
                "rtc outbox enqueue failed; state already persisted, event will not be delivered"
            );
        }
    }

    /// Emit a security/mutation audit event. Best-effort: never blocks
    /// user-facing operations. Emission failures are logged and swallowed
    /// so a faulty audit sink cannot take down call signaling.
    fn emit_audit(
        &self,
        auth: &AppContext,
        event_type: AuditEventType,
        action: &str,
        target_id: &str,
        outcome: AuditOutcome,
        reason: Option<String>,
        metadata: serde_json::Value,
    ) {
        let event_id = match self.id_generator.next_id() {
            Ok(id) => id,
            Err(e) => {
                tracing::warn!(
                    target: "sdkwork.im.calls.audit",
                    error = %format!("{e:?}"),
                    action,
                    "rtc audit id allocation failed; audit event dropped"
                );
                return;
            }
        };
        let actor_type = match auth.actor_kind.as_str() {
            "user" => AuditActorType::User,
            "system" => AuditActorType::System,
            "admin" => AuditActorType::Admin,
            "job" => AuditActorType::Job,
            "service" | "api_key" => AuditActorType::Service,
            _ => AuditActorType::Anonymous,
        };

        let event = AuditEvent::builder()
            .event_id(event_id)
            .event_type(event_type)
            .timestamp(utc_now_rfc3339_millis())
            .tenant_id(auth.tenant_id.clone())
            .organization_id(auth.organization_id.clone())
            .user_id(Some(auth.user_id.clone()))
            .session_id(auth.session_id.clone())
            .actor_type(actor_type)
            .actor_id(auth.actor_id.clone())
            .action(action.into())
            .target_type(OUTBOX_AGGREGATE_TYPE.into())
            .target_id(target_id.into())
            .outcome(outcome)
            .reason_opt(reason)
            .metadata(metadata)
            .build();

        let event = match event {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!(
                    target: "sdkwork.im.calls.audit",
                    error = %e,
                    action,
                    "rtc audit event build failed; audit event dropped"
                );
                return;
            }
        };

        if let Err(e) = self.audit_emitter.emit(event) {
            tracing::warn!(
                target: "sdkwork.im.calls.audit",
                error = %e,
                action,
                "rtc audit emit failed (non-blocking)"
            );
        }
    }

    pub(crate) fn ensure_rtc_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<(), CallingError> {
        let scope_key = rtc_session_scope_key(tenant_id, rtc_session_id);
        // Fast path: session already in memory. Use `contains_key` (read
        // lock, immediately released) to avoid holding a shard lock across
        // the state-store load on the slow path.
        if self.sessions.contains_key(scope_key.as_str()) {
            self.signals.entry(scope_key).or_default();
            return Ok(());
        }

        // Slow path: load from durable state store. Between the
        // `contains_key` check above and the `entry().or_insert()` below,
        // a concurrent `create_session` may have already inserted the
        // session. Using `or_insert` (not `insert`) ensures we do NOT
        // overwrite the concurrent create with potentially stale loaded
        // state — this closes the TOCTOU window.
        let restored = self
            .state_store
            .load_state(tenant_id, rtc_session_id)
            .map_err(|err| {
                CallingError::state_store(ContractError::Unavailable(format!(
                    "call state store load failed: {err:?}"
                )))
            })?;
        if let Some(record) = restored {
            self.sessions
                .entry(scope_key.clone())
                .or_insert(record.session);
            self.signals
                .entry(scope_key)
                .or_insert(signal_index(record.signals));
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
    ) -> Result<Session, CallingError> {
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        self.sessions
            .get(rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .map(|entry| entry.clone())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })
    }

    pub fn create_session(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<Session, CallingError> {
        Ok(self
            .create_session_with_outcome(auth, request)?
            .session)
    }

    pub fn create_session_with_outcome(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<SessionMutationOutcome, CallingError> {
        validate_create_request_payload_size(&request)?;

        let scope_key =
            rtc_session_scope_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
        self.ensure_rtc_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;

        // Atomically claim the session slot via the entry API. Previous
        // implementation used a read-then-insert pattern (get → check →
        // insert), which allowed two concurrent create requests for the same
        // `rtc_session_id` to both pass the idempotency check, both invoke
        // the RTC provider, and the second insert to silently overwrite the
        // first. The entry API closes the race by holding the shard write
        // lock across the check-and-claim.
        let entry = self.sessions.entry(scope_key.clone());
        match entry {
            dashmap::mapref::entry::Entry::Occupied(occupied) => {
                let existing = occupied.get();
                if call_session_matches_create_request(existing, auth, &request) {
                    let session = existing.clone();
                    drop(occupied);
                    self.signals.entry(scope_key).or_default();
                    // Idempotent replay: not a new mutation, no audit/outbox.
                    Ok(SessionMutationOutcome {
                        session,
                        applied: false,
                    })
                } else {
                    // Conflict: a different session already owns this id.
                    self.emit_audit(
                        auth,
                        AuditEventType::SecurityPermissionDenied,
                        "create_call_session",
                        request.rtc_session_id.as_str(),
                        AuditOutcome::Denied,
                        Some("rtc_session_id_collision".into()),
                        serde_json::json!({
                            "existing_initiator": existing.initiator_id,
                            "existing_initiator_kind": existing.initiator_kind,
                        }),
                    );
                    Err(CallingError::conflict(request.rtc_session_id.as_str()))
                }
            }
            dashmap::mapref::entry::Entry::Vacant(vacant) => {
                // Resolve provider fields BEFORE allocating the epoch so a
                // provider failure does not consume a fencing token. The
                // provider call happens outside the shard lock to avoid
                // blocking other sessions while the provider responds.
                let provider_fields =
                    self.resolve_provider_media_session(auth, &request)?;

                // Allocate the epoch only after the provider succeeds.
                let epoch = self.allocate_epoch();
                let started_at = utc_now_rfc3339_millis();

                let session = Session {
                    tenant_id: auth.tenant_id.clone(),
                    rtc_session_id: request.rtc_session_id.clone(),
                    conversation_id: request.conversation_id,
                    rtc_mode: request.rtc_mode,
                    initiator_id: auth.actor_id.clone(),
                    initiator_kind: auth.actor_kind.clone(),
                    provider_plugin_id: provider_fields.provider_plugin_id,
                    provider_session_id: provider_fields.provider_session_id,
                    access_endpoint: provider_fields.access_endpoint,
                    provider_region: provider_fields.provider_region,
                    state: SessionState::Started,
                    signaling_stream_id: None,
                    artifact_message_id: None,
                    started_at: started_at.clone(),
                    ended_at: None,
                    initiating_at: Some(started_at.clone()),
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
                    epoch,
                    version: 1,
                    participants: SessionParticipants::default(),
                    last_activity_at: Some(started_at.clone()),
                    signal_rate_tracker: SignalRateTracker::default(),
                };

                vacant.insert(session.clone());
                self.signals.entry(scope_key).or_default();
                self.persist_state(auth, request.rtc_session_id.as_str())?;

                // Audit + outbox emission after durable persistence succeeds.
                let metadata = serde_json::json!({
                    "rtc_mode": session.rtc_mode,
                    "conversation_id": session.conversation_id,
                    "provider_plugin_id": session.provider_plugin_id,
                    "provider_session_id": session.provider_session_id,
                    "provider_region": session.provider_region,
                    "epoch": session.epoch,
                    "version": session.version,
                });
                self.emit_audit(
                    auth,
                    AuditEventType::MutationCallSessionCreated,
                    "create_call_session",
                    session.rtc_session_id.as_str(),
                    AuditOutcome::Success,
                    None,
                    metadata.clone(),
                );
                self.enqueue_outbox_event(
                    auth,
                    session.rtc_session_id.as_str(),
                    "rtc.session.created",
                    serde_json::json!({
                        "rtc_session_id": session.rtc_session_id,
                        "tenant_id": session.tenant_id,
                        "organization_id": auth.organization_id,
                        "initiator_id": session.initiator_id,
                        "initiator_kind": session.initiator_kind,
                        "rtc_mode": session.rtc_mode,
                        "conversation_id": session.conversation_id,
                        "state": session.state.as_str(),
                        "epoch": session.epoch,
                        "started_at": started_at,
                    }),
                );

                Ok(SessionMutationOutcome {
                    session,
                    applied: true,
                })
            }
        }
    }

    pub fn invite_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<Session, CallingError> {
        Ok(self
            .invite_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn invite_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<SessionMutationOutcome, CallingError> {
        validate_invite_request_payload_size(&request)?;
        validate_participant_ids_payload_size(&request.participant_ids)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;

        let mut session_ref = self
            .sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if session_ref.state != SessionState::Started {
            let invalid_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::MutationCallParticipantChanged,
                "invite_call_session",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_state_invalid".into()),
                serde_json::json!({ "state": invalid_state }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {invalid_state}: {rtc_session_id}"
                ),
            });
        }

        // Idempotent: if the same signaling_stream_id was already set and no
        // new participants are being added, return the unmodified session.
        let stream_id_unchanged = match (
            session_ref.signaling_stream_id.as_deref(),
            request.signaling_stream_id.as_deref(),
        ) {
            (Some(existing), Some(new)) => existing == new,
            (None, None) => true,
            _ => false,
        };
        let all_already_invited = request
            .participant_ids
            .iter()
            .all(|id| session_ref.participants.invited_ids.contains(id));
        if stream_id_unchanged && all_already_invited {
            let session = session_ref.clone();
            drop(session_ref);
            return Ok(SessionMutationOutcome {
                session,
                applied: false,
            });
        }

        // Apply signaling stream id and merge new participants (deduped).
        let mut added_participant_ids: Vec<String> = Vec::new();
        session_ref.signaling_stream_id = request.signaling_stream_id.clone();
        for participant_id in request.participant_ids {
            if !session_ref.participants.invited_ids.contains(&participant_id) {
                session_ref.participants.invited_ids.push(participant_id.clone());
                added_participant_ids.push(participant_id);
            }
        }
        let session = session_ref.clone();
        drop(session_ref);
        self.persist_state(auth, rtc_session_id)?;

        // Audit + outbox emission after durable persistence succeeds.
        let metadata = serde_json::json!({
            "signaling_stream_id": session.signaling_stream_id,
            "added_participant_ids": added_participant_ids,
            "invited_ids_total": session.participants.invited_ids.len(),
            "epoch": session.epoch,
            "version": session.version,
        });
        self.emit_audit(
            auth,
            AuditEventType::MutationCallParticipantChanged,
            "invite_call_session",
            rtc_session_id,
            AuditOutcome::Success,
            None,
            metadata.clone(),
        );
        self.enqueue_outbox_event(
            auth,
            rtc_session_id,
            "rtc.session.invited",
            serde_json::json!({
                "rtc_session_id": session.rtc_session_id,
                "tenant_id": session.tenant_id,
                "organization_id": auth.organization_id,
                "signaling_stream_id": session.signaling_stream_id,
                "added_participant_ids": added_participant_ids,
                "invited_ids": session.participants.invited_ids,
                "epoch": session.epoch,
            }),
        );

        Ok(SessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn accept_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<Session, CallingError> {
        Ok(self
            .accept_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn accept_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<SessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut session_ref = self
            .sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        // Authorization check: initiator or invited/accepted participants only
        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "accept_call_session",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({ "initiator_id": initiator_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to accept this call session".into(),
            });
        }

        if session_ref.state == SessionState::Accepted {
            if session_ref.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                let session = session_ref.clone();
                drop(session_ref);
                return Ok(SessionMutationOutcome {
                    session,
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session_ref.state != SessionState::Started {
            let invalid_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::MutationCallSessionStateChanged,
                "accept_call_session",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_state_invalid".into()),
                serde_json::json!({ "state": invalid_state }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {invalid_state}: {rtc_session_id}"
                ),
            });
        }

        // Increment epoch and version for fencing
        let new_epoch = self.allocate_epoch();
        let now = utc_now_rfc3339_millis();
        session_ref.epoch = new_epoch;
        session_ref.version += 1;
        session_ref.state = SessionState::Accepted;
        session_ref.connecting_at = Some(now.clone());
        session_ref.last_activity_at = Some(now.clone());
        if let Some(artifact_message_id) = request.artifact_message_id {
            session_ref.artifact_message_id = Some(artifact_message_id);
        }
        // Track accepted participant (deduped; an initiator who calls accept
        // twice via different code paths should not appear twice).
        let mut newly_accepted = false;
        if !session_ref.participants.accepted_ids.contains(&auth.actor_id) {
            session_ref.participants.accepted_ids.push(auth.actor_id.clone());
            newly_accepted = true;
        }
        let session = session_ref.clone();
        drop(session_ref);
        self.persist_state(auth, rtc_session_id)?;

        // Audit + outbox emission after durable persistence succeeds.
        let metadata = serde_json::json!({
            "from_state": SessionState::Started.as_str(),
            "to_state": session.state.as_str(),
            "artifact_message_id": session.artifact_message_id,
            "newly_accepted_participant": newly_accepted,
            "accepted_ids": session.participants.accepted_ids,
            "epoch": session.epoch,
            "version": session.version,
        });
        self.emit_audit(
            auth,
            AuditEventType::MutationCallSessionStateChanged,
            "accept_call_session",
            rtc_session_id,
            AuditOutcome::Success,
            None,
            metadata.clone(),
        );
        self.enqueue_outbox_event(
            auth,
            rtc_session_id,
            "rtc.session.accepted",
            serde_json::json!({
                "rtc_session_id": session.rtc_session_id,
                "tenant_id": session.tenant_id,
                "organization_id": auth.organization_id,
                "state": session.state.as_str(),
                "accepted_participant_id": auth.actor_id,
                "newly_accepted": newly_accepted,
                "accepted_ids": session.participants.accepted_ids,
                "artifact_message_id": session.artifact_message_id,
                "epoch": session.epoch,
                "connecting_at": session.connecting_at,
            }),
        );

        Ok(SessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn reject_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<Session, CallingError> {
        Ok(self
            .reject_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn reject_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<SessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut session_ref = self
            .sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        // Authorization check: initiator or invited/accepted participants only
        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "reject_call_session",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({ "initiator_id": initiator_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to reject this call session".into(),
            });
        }

        if session_ref.state == SessionState::Rejected {
            if session_ref.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                let session = session_ref.clone();
                drop(session_ref);
                return Ok(SessionMutationOutcome {
                    session,
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session_ref.state != SessionState::Started {
            let invalid_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::MutationCallSessionStateChanged,
                "reject_call_session",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_state_invalid".into()),
                serde_json::json!({ "state": invalid_state }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {invalid_state}: {rtc_session_id}"
                ),
            });
        }

        // Increment epoch and version for fencing
        let new_epoch = self.allocate_epoch();
        let now = utc_now_rfc3339_millis();
        let from_state = session_ref.state.clone();
        session_ref.epoch = new_epoch;
        session_ref.version += 1;
        session_ref.state = SessionState::Rejected;
        session_ref.ended_at = Some(now.clone());
        session_ref.ended_reason = Some("declined".into());
        session_ref.last_activity_at = Some(now.clone());
        if let Some(artifact_message_id) = request.artifact_message_id {
            session_ref.artifact_message_id = Some(artifact_message_id);
        }
        let session = session_ref.clone();
        drop(session_ref);
        self.persist_state(auth, rtc_session_id)?;

        // Audit + outbox emission after durable persistence succeeds.
        let metadata = serde_json::json!({
            "from_state": from_state.as_str(),
            "to_state": session.state.as_str(),
            "ended_reason": session.ended_reason,
            "artifact_message_id": session.artifact_message_id,
            "epoch": session.epoch,
            "version": session.version,
        });
        self.emit_audit(
            auth,
            AuditEventType::MutationCallSessionStateChanged,
            "reject_call_session",
            rtc_session_id,
            AuditOutcome::Success,
            None,
            metadata.clone(),
        );
        self.enqueue_outbox_event(
            auth,
            rtc_session_id,
            "rtc.session.rejected",
            serde_json::json!({
                "rtc_session_id": session.rtc_session_id,
                "tenant_id": session.tenant_id,
                "organization_id": auth.organization_id,
                "state": session.state.as_str(),
                "rejecting_participant_id": auth.actor_id,
                "ended_reason": session.ended_reason,
                "artifact_message_id": session.artifact_message_id,
                "epoch": session.epoch,
                "ended_at": session.ended_at,
            }),
        );

        // Revoke active media credentials by closing the provider session.
        // Best-effort: failures are logged and swallowed so a provider
        // outage during teardown cannot block the terminal transition.
        self.revoke_session_credentials(auth, rtc_session_id, session.state.as_str());

        Ok(SessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn end_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<Session, CallingError> {
        Ok(self
            .end_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn end_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<SessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut session_ref = self
            .sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        // Authorization check: initiator or invited/accepted participants only
        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "end_call_session",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({ "initiator_id": initiator_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to end this call session".into(),
            });
        }

        if session_ref.state == SessionState::Ended {
            if session_ref.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                let session = session_ref.clone();
                drop(session_ref);
                return Ok(SessionMutationOutcome {
                    session,
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session_ref.state == SessionState::Rejected {
            let invalid_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::MutationCallSessionStateChanged,
                "end_call_session",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_already_rejected".into()),
                serde_json::json!({ "state": invalid_state }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!("call session is already rejected: {rtc_session_id}"),
            });
        }

        // Increment epoch and version for fencing
        let new_epoch = self.allocate_epoch();
        let now = utc_now_rfc3339_millis();
        let from_state = session_ref.state.clone();
        session_ref.epoch = new_epoch;
        session_ref.version += 1;
        session_ref.state = SessionState::Ended;
        session_ref.ended_at = Some(now.clone());
        session_ref.ended_reason = Some("normal".into());
        session_ref.last_activity_at = Some(now.clone());
        if let Some(artifact_message_id) = request.artifact_message_id {
            session_ref.artifact_message_id = Some(artifact_message_id);
        }
        let session = session_ref.clone();
        drop(session_ref);
        self.persist_state(auth, rtc_session_id)?;

        // Audit + outbox emission after durable persistence succeeds.
        let metadata = serde_json::json!({
            "from_state": from_state.as_str(),
            "to_state": session.state.as_str(),
            "ended_reason": session.ended_reason,
            "artifact_message_id": session.artifact_message_id,
            "epoch": session.epoch,
            "version": session.version,
        });
        self.emit_audit(
            auth,
            AuditEventType::MutationCallSessionStateChanged,
            "end_call_session",
            rtc_session_id,
            AuditOutcome::Success,
            None,
            metadata.clone(),
        );
        self.enqueue_outbox_event(
            auth,
            rtc_session_id,
            "rtc.session.ended",
            serde_json::json!({
                "rtc_session_id": session.rtc_session_id,
                "tenant_id": session.tenant_id,
                "organization_id": auth.organization_id,
                "state": session.state.as_str(),
                "ending_participant_id": auth.actor_id,
                "ended_reason": session.ended_reason,
                "artifact_message_id": session.artifact_message_id,
                "epoch": session.epoch,
                "ended_at": session.ended_at,
            }),
        );

        // Revoke active media credentials by closing the provider session.
        // Best-effort: failures are logged and swallowed so a provider
        // outage during teardown cannot block the terminal transition.
        self.revoke_session_credentials(auth, rtc_session_id, session.state.as_str());

        Ok(SessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn post_signal(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<SignalEvent, CallingError> {
        validate_post_signal_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;

        let sender = resolve_rtc_signal_sender(auth);
        let occurred_at = utc_now_rfc3339_millis();

        // TOCTOU-safe signal posting: hold the `sessions` shard read lock
        // across the terminal-state check AND the `signals` shard write
        // lock acquisition + signal insertion. This closes the race window
        // where a concurrent `end_session`/`reject_session` could transition
        // the session to a terminal state between the check and the insert.
        //
        // Lock ordering is `sessions → signals`, consistent with
        // `create_session` and `ensure_rtc_state`, so this cannot deadlock.
        //
        // The `sessions` read lock prevents concurrent writers (state
        // transitions) but allows concurrent readers, minimizing contention.
        let session_ref = self
            .sessions
            .get(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        // Authorization check (IDOR fix per SECURITY_SPEC §4.2): only the
        // initiator, invited/accepted participants, or principals holding
        // `im.calls.credentials.issue` may post signals to a session.
        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "post_call_signal",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({ "initiator_id": initiator_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to post signals to this call session".into(),
            });
        }

        if session_ref.state.is_terminal() {
            let terminal_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::MutationCallSignalPosted,
                "post_call_signal",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_terminal".into()),
                serde_json::json!({ "state": terminal_state }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is in terminal state {terminal_state}: {rtc_session_id}"
                ),
            });
        }

        // Clone session fields needed for the SignalEvent while holding the
        // read lock, ensuring a consistent snapshot.
        let rtc_session_id_owned = session_ref.rtc_session_id.clone();
        let conversation_id = session_ref.conversation_id.clone();
        let rtc_mode = session_ref.rtc_mode.clone();

        // Acquire the signals shard write lock while still holding the
        // sessions read lock. This is the critical TOCTOU fix: no concurrent
        // state transition can occur between the terminal-state check above
        // and the signal insertion below.
        let mut signals_ref = self.signals.entry(scope_key).or_default();
        let next_signal_seq = signals_ref
            .last_key_value()
            .map(|(seq, _)| *seq + 1)
            .unwrap_or(1);

        let event = SignalEvent {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: rtc_session_id_owned,
            signal_seq: next_signal_seq,
            conversation_id,
            rtc_mode,
            signal_type: request.signal_type,
            schema_ref: request.schema_ref,
            payload: request.payload,
            sender,
            signaling_stream_id: request.signaling_stream_id,
            occurred_at,
        };

        signals_ref.insert(event.signal_seq, event.clone());
        drop(signals_ref);
        drop(session_ref);
        self.persist_state(auth, rtc_session_id)?;

        // Audit + outbox emission after durable persistence succeeds.
        let signal_seq = event.signal_seq;
        let signal_type = event.signal_type.clone();
        let schema_ref = event.schema_ref.clone();
        let signaling_stream_id = event.signaling_stream_id.clone();
        self.emit_audit(
            auth,
            AuditEventType::MutationCallSignalPosted,
            "post_call_signal",
            rtc_session_id,
            AuditOutcome::Success,
            None,
            serde_json::json!({
                "signal_seq": signal_seq,
                "signal_type": signal_type,
                "schema_ref": schema_ref,
                "signaling_stream_id": signaling_stream_id,
            }),
        );
        self.enqueue_outbox_event(
            auth,
            rtc_session_id,
            "rtc.signal.posted",
            serde_json::json!({
                "rtc_session_id": rtc_session_id,
                "tenant_id": auth.tenant_id,
                "organization_id": auth.organization_id,
                "signal_seq": signal_seq,
                "signal_type": signal_type,
                "schema_ref": schema_ref,
                "signaling_stream_id": signaling_stream_id,
                "sender_id": auth.actor_id,
                "sender_kind": auth.actor_kind,
                "occurred_at": event.occurred_at,
            }),
        );

        Ok(event)
    }

    pub fn list_signals(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        after_signal_seq: Option<u64>,
        limit: Option<usize>,
    ) -> Result<(Vec<SignalEvent>, bool), CallingError> {
        let after_signal_seq = after_signal_seq.unwrap_or(0);
        let limit = limit.unwrap_or(100);
        if limit == 0 {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }
        if limit > SIGNAL_LIST_MAX_LIMIT {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!(
                    "limit must be less than or equal to {SIGNAL_LIST_MAX_LIMIT}"
                ),
            });
        }

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let session_ref = self
            .sessions
            .get(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        // Authorization check (IDOR fix per SECURITY_SPEC §4.2): only the
        // initiator, invited/accepted participants, or principals holding
        // `im.calls.credentials.issue` may list signals of a session.
        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "list_call_signals",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({ "initiator_id": initiator_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to list signals of this call session".into(),
            });
        }
        drop(session_ref);

        let mut has_more = false;
        let mut items: Vec<SignalEvent> = Vec::new();
        if let Some(session_signals) = self.signals.get(scope_key.as_str()) {
            for (_, event) in session_signals.range::<u64, _>((Excluded(after_signal_seq), std::ops::Bound::Unbounded)) {
                if items.len() == limit {
                    has_more = true;
                    break;
                }
                items.push(event.clone());
            }
        }

        Ok((items, has_more))
    }

    /// Issue a real participant credential via the RTC provider plugin when
    /// one is wired in. Returns a clear error when no provider is available
    /// so callers know media join is not possible.
    pub fn issue_participant_credential(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, CallingError> {
        // Session lookup + authorization (IDOR fix per SECURITY_SPEC §4.3):
        // the caller must be authorized for the session AND the target
        // `participant_id` must be a session participant. Without this check,
        // any authenticated tenant member could request media credentials for
        // arbitrary principals or sessions. Auth is checked BEFORE the
        // provider-availability check so unauthorized callers never learn
        // whether a provider is wired.
        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        let session_ref = self
            .sessions
            .get(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "issue_participant_credential",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({
                    "initiator_id": initiator_id,
                    "participant_id": participant_id,
                }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to issue credentials for this call session"
                    .into(),
            });
        }

        if !is_session_participant(participant_id, &session_ref) {
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "issue_participant_credential",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("participant_not_in_session".into()),
                serde_json::json!({ "participant_id": participant_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "participant_not_in_session",
                message: format!(
                    "participant {participant_id} is not a member of call session {rtc_session_id}"
                ),
            });
        }

        // Terminal-state defense-in-depth: a session that has ended, been
        // rejected, canceled, timed out, or failed must not issue new media
        // credentials. The handler also checks this, but the runtime must be
        // self-protecting so internal callers cannot bypass the guard.
        if session_ref.state.is_terminal() {
            let terminal_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecuritySessionLifecycle,
                "issue_participant_credential",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_terminal".into()),
                serde_json::json!({
                    "participant_id": participant_id,
                    "state": terminal_state,
                }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is in terminal state {terminal_state}; credentials cannot be issued: {rtc_session_id}"
                ),
            });
        }
        drop(session_ref);

        let rtc_provider = match self.rtc_provider.as_ref() {
            Some(provider) => provider,
            None => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "issue_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some("rtc_provider_not_configured".into()),
                    serde_json::json!({ "participant_id": participant_id }),
                );
                return Err(CallingError {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    code: "rtc_provider_not_configured",
                    message: "RTC provider is not wired; media credential issuance is unavailable. Configure sdkwork-rtc integration to enable calls.".into(),
                });
            }
        };

        let result = rtc_provider
            .issue_participant_credential(
                auth.tenant_id.as_str(),
                rtc_session_id,
                participant_id,
                None,
            )
            .map_err(map_rtc_contract_error);

        match &result {
            Ok(_) => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "issue_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Success,
                    None,
                    serde_json::json!({
                        "participant_id": participant_id,
                        "provider_plugin_id": rtc_provider.descriptor().plugin_id,
                    }),
                );
            }
            Err(err) => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "issue_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some(err.code.into()),
                    serde_json::json!({
                        "participant_id": participant_id,
                        "error_code": err.code,
                    }),
                );
            }
        }
        result
    }

    /// Refresh an expiring participant credential via the RTC provider.
    ///
    /// The caller must be authorized for the session (initiator, participant,
    /// or `im.calls.credentials.issue` holder), the `participant_id` must be a
    /// session participant, and the session must not be in a terminal state.
    /// The provider's `refresh_participant_credential` issues a fresh
    /// credential with a new `expires_at`, extending media access without
    /// requiring the participant to re-join.
    pub fn refresh_participant_credential(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, CallingError> {
        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        let session_ref = self
            .sessions
            .get(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if !is_authorized_for_session(auth, &session_ref) {
            let initiator_id = session_ref.initiator_id.clone();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "refresh_participant_credential",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("principal_not_authorized".into()),
                serde_json::json!({
                    "initiator_id": initiator_id,
                    "participant_id": participant_id,
                }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "call_session_forbidden",
                message: "principal is not authorized to refresh credentials for this call session"
                    .into(),
            });
        }

        if !is_session_participant(participant_id, &session_ref) {
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecurityPermissionDenied,
                "refresh_participant_credential",
                rtc_session_id,
                AuditOutcome::Denied,
                Some("participant_not_in_session".into()),
                serde_json::json!({ "participant_id": participant_id }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "participant_not_in_session",
                message: format!(
                    "participant {participant_id} is not a member of call session {rtc_session_id}"
                ),
            });
        }

        if session_ref.state.is_terminal() {
            let terminal_state = session_ref.state.as_str().to_string();
            drop(session_ref);
            self.emit_audit(
                auth,
                AuditEventType::SecuritySessionLifecycle,
                "refresh_participant_credential",
                rtc_session_id,
                AuditOutcome::Failure,
                Some("call_session_terminal".into()),
                serde_json::json!({
                    "participant_id": participant_id,
                    "state": terminal_state,
                }),
            );
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is in terminal state {terminal_state}; credentials cannot be refreshed: {rtc_session_id}"
                ),
            });
        }
        drop(session_ref);

        let rtc_provider = match self.rtc_provider.as_ref() {
            Some(provider) => provider,
            None => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "refresh_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some("rtc_provider_not_configured".into()),
                    serde_json::json!({ "participant_id": participant_id }),
                );
                return Err(CallingError {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    code: "rtc_provider_not_configured",
                    message: "RTC provider is not wired; media credential refresh is unavailable. Configure sdkwork-rtc integration to enable calls.".into(),
                });
            }
        };

        let result = rtc_provider
            .refresh_participant_credential(
                auth.tenant_id.as_str(),
                rtc_session_id,
                participant_id,
                None,
            )
            .map_err(map_rtc_contract_error);

        match &result {
            Ok(credential) => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "refresh_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Success,
                    None,
                    serde_json::json!({
                        "participant_id": participant_id,
                        "provider_plugin_id": rtc_provider.descriptor().plugin_id,
                        "expires_at": credential.expires_at,
                    }),
                );
                self.enqueue_outbox_event(
                    auth,
                    rtc_session_id,
                    "rtc.credential.refreshed",
                    serde_json::json!({
                        "rtc_session_id": rtc_session_id,
                        "tenant_id": auth.tenant_id,
                        "organization_id": auth.organization_id,
                        "participant_id": participant_id,
                        "expires_at": credential.expires_at,
                    }),
                );
            }
            Err(err) => {
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "refresh_participant_credential",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some(err.code.into()),
                    serde_json::json!({
                        "participant_id": participant_id,
                        "error_code": err.code,
                    }),
                );
            }
        }
        result
    }

    /// Best-effort credential revocation when a session transitions to a
    /// terminal state. Calls the RTC provider's `close_session` to tear down
    /// the media room and invalidate all active participant credentials.
    ///
    /// Failures are logged and swallowed (best-effort) so a provider outage
    /// during teardown cannot block the session state transition. The session
    /// state is already persisted as terminal; lingering credentials will
    /// expire per their TTL.
    fn revoke_session_credentials(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        terminal_state: &str,
    ) {
        let Some(rtc_provider) = self.rtc_provider.as_ref() else {
            return;
        };
        let result = rtc_provider.close_session(auth.tenant_id.as_str(), rtc_session_id);
        match result {
            Ok(closed) => {
                tracing::info!(
                    target: "sdkwork.im.calls.credentials",
                    rtc_session_id,
                    tenant_id = %auth.tenant_id,
                    terminal_state,
                    closed,
                    "RTC provider session closed; participant credentials revoked"
                );
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "revoke_session_credentials",
                    rtc_session_id,
                    AuditOutcome::Success,
                    None,
                    serde_json::json!({
                        "terminal_state": terminal_state,
                        "provider_closed": closed,
                    }),
                );
                self.enqueue_outbox_event(
                    auth,
                    rtc_session_id,
                    "rtc.credentials.revoked",
                    serde_json::json!({
                        "rtc_session_id": rtc_session_id,
                        "tenant_id": auth.tenant_id,
                        "organization_id": auth.organization_id,
                        "terminal_state": terminal_state,
                        "provider_closed": closed,
                    }),
                );
            }
            Err(err) => {
                let mapped = map_rtc_contract_error(err);
                let error_code = mapped.code.to_string();
                tracing::warn!(
                    target: "sdkwork.im.calls.credentials",
                    rtc_session_id,
                    tenant_id = %auth.tenant_id,
                    terminal_state,
                    error = %mapped.message,
                    error_code = %error_code,
                    "RTC provider close_session failed during terminal transition; \
                     credentials will expire per TTL"
                );
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "revoke_session_credentials",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some(error_code.clone()),
                    serde_json::json!({
                        "terminal_state": terminal_state,
                        "error_code": error_code,
                    }),
                );
            }
        }
    }

    /// Expose the wired RTC provider handle so callers can invoke other
    /// provider port operations without re-resolving the provider.
    pub fn rtc_provider(&self) -> Option<&Arc<dyn RtcProviderPort>> {
        self.rtc_provider.as_ref()
    }

    /// Resolve provider fields by delegating media session creation to the
    /// RTC provider plugin. Returns `None` for all provider fields when no
    /// RTC provider is wired (signaling-only mode).
    fn resolve_provider_media_session(
        &self,
        auth: &AppContext,
        request: &CreateRtcSessionRequest,
    ) -> Result<ProviderMediaSessionFields, CallingError> {
        let rtc_provider = match self.rtc_provider.as_ref() {
            Some(provider) => provider,
            None => {
                // Signaling-only mode: no provider integration. This is
                // acceptable for development/testing but will fail real
                // media join. Production deployments must wire the RTC
                // provider via `with_rtc_provider`.
                tracing::warn!(
                    rtc_session_id = %request.rtc_session_id,
                    "RTC provider is not wired; call session will operate in signaling-only mode without provider media"
                );
                return Ok(ProviderMediaSessionFields::default());
            }
        };

        // Map the IM call request to the RTC media session request.
        // The provider owns session creation, region selection, and
        // credential issuance per `../sdkwork-rtc/docs/rtc-im-boundary.md`.
        let media_mode = parse_rtc_media_session_mode(&request.rtc_mode);
        let create_request = RtcCreateMediaSessionRequest {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: request.rtc_session_id.clone(),
            media_mode,
            room_id: Some(request.rtc_session_id.clone()),
            region: None,
        };

        let handle: RtcSessionHandle = rtc_provider
            .create_session(create_request)
            .map_err(map_rtc_contract_error)?;

        // The provider plugin ID identifies which RTC provider is handling
        // this session (e.g. "rtc-volcengine"). It comes from the provider's
        // descriptor, not from the session handle.
        let provider_plugin_id = rtc_provider.descriptor().plugin_id;

        Ok(ProviderMediaSessionFields {
            provider_plugin_id: Some(provider_plugin_id),
            provider_session_id: Some(handle.provider_session_id),
            access_endpoint: handle.access_endpoint,
            provider_region: handle.region,
        })
    }

    fn persist_state(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
    ) -> Result<(), CallingError> {
        let record = self.state_record(auth.tenant_id.as_str(), rtc_session_id)?;
        match self.state_store.save_state(record) {
            Ok(()) => Ok(()),
            Err(err) => {
                // Persistence failed: the in-memory mutation has already been
                // applied to the DashMap, so the memory view now diverges from
                // durable state. Capture the session snapshot before eviction
                // so we can emit a `session.revoked` outbox event for
                // downstream consumers (WebSocket gateway, notification
                // service) to notify connected clients.
                let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
                let session_snapshot = self
                    .sessions
                    .get(scope_key.as_str())
                    .map(|entry| entry.clone());

                // Evict the in-memory entry so the next access re-loads from
                // the state store, preventing the divergence from becoming
                // permanent.
                self.sessions.remove(scope_key.as_str());
                self.signals.remove(scope_key.as_str());

                // Audit: persistence failure is a security-relevant lifecycle
                // event (the session state is unreliable).
                self.emit_audit(
                    auth,
                    AuditEventType::SecuritySessionLifecycle,
                    "persist_state",
                    rtc_session_id,
                    AuditOutcome::Failure,
                    Some("state_persist_failed".into()),
                    serde_json::json!({
                        "error": format!("{err:?}"),
                        "session_state": session_snapshot.as_ref().map(|s| s.state.as_str()),
                    }),
                );

                // Outbox: notify downstream consumers that the session is
                // revoked. Clients connected via WebSocket should receive a
                // `session.revoked` signal so they can tear down media and
                // surface a clear error instead of retrying against stale
                // state.
                if let Some(session) = &session_snapshot {
                    self.enqueue_outbox_event(
                        auth,
                        rtc_session_id,
                        "rtc.session.revoked",
                        serde_json::json!({
                            "rtc_session_id": rtc_session_id,
                            "tenant_id": auth.tenant_id,
                            "organization_id": auth.organization_id,
                            "state": session.state.as_str(),
                            "epoch": session.epoch,
                            "version": session.version,
                            "reason": "state_persist_failed",
                        }),
                    );
                }

                Err(CallingError {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    code: "call_session_revoked",
                    message: format!(
                        "call session state persistence failed; session {rtc_session_id} \
                         has been revoked and must be re-fetched or recreated: {err:?}"
                    ),
                })
            }
        }
    }

    fn state_record(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<StateRecord, CallingError> {
        let scope_key = rtc_session_scope_key(tenant_id, rtc_session_id);
        let session = self
            .sessions
            .get(scope_key.as_str())
            .map(|entry| entry.clone())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "call_state_inconsistent",
                message: format!(
                    "call session missing while persisting state for session id: {rtc_session_id}"
                ),
            })?;
        let signals: Vec<SignalEvent> = self
            .signals
            .get(scope_key.as_str())
            .map(|entry| entry.value().clone())
            .unwrap_or_default()
            .into_values()
            .collect();

        Ok(StateRecord {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            session,
            signals,
            updated_at: utc_now_rfc3339_millis(),
        })
    }
}

/// Provider fields resolved by delegating to the RTC product service.
/// All fields are `None` in signaling-only mode (no RTC service wired).
#[derive(Clone, Debug, Default)]
struct ProviderMediaSessionFields {
    provider_plugin_id: Option<String>,
    provider_session_id: Option<String>,
    access_endpoint: Option<String>,
    provider_region: Option<String>,
}

#[derive(Clone, Default)]
pub(crate) struct RuntimeMemoryStateStore {
    pub(crate) states: DashMap<String, StateRecord>,
}

/// In-process Snowflake-compatible ID generator for development.
///
/// Uses a monotonically-increasing `AtomicI64` counter starting from 1.
/// Suitable for single-process development/testing only — production
/// deployments MUST wire `RuntimeSnowflakeIdGenerator` (or equivalent)
/// via [`CallingRuntime::with_id_generator`] to guarantee cross-process
/// uniqueness for audit event IDs and outbox outbox IDs.
///
/// The counter never returns 0 (which would collide with default
/// `i64::default()` semantics in some downstream consumers).
#[derive(Default)]
pub(crate) struct LocalCounterIdGenerator {
    counter: AtomicI64,
}

impl IdGenerator for LocalCounterIdGenerator {
    fn next_id(&self) -> Result<i64, ContractError> {
        // fetch_add returns the previous value; start at 1 so the first
        // returned ID is 1 (not 0).
        Ok(self.counter.fetch_add(1, Ordering::SeqCst) + 1)
    }

    fn node_id(&self) -> u16 {
        0
    }

    fn next_id_at(&self, _timestamp_millis: u64) -> Result<i64, ContractError> {
        self.next_id()
    }
}

impl StateStore for RuntimeMemoryStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<StateRecord>, sdkwork_communication_rtc_service::RtcContractError> {
        Ok(self
            .states
            .get(rtc_session_scope_key(tenant_id, rtc_session_id).as_str())
            .map(|entry| entry.clone()))
    }

    fn save_state(
        &self,
        record: StateRecord,
    ) -> Result<(), sdkwork_communication_rtc_service::RtcContractError> {
        let key = rtc_session_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());

        // Use DashMap's entry API for atomic epoch-based fencing
        match self.states.get_mut(key.as_str()) {
            Some(mut existing) => {
                // Epoch-based fencing: only accept if new epoch is greater or equal.
                // When equal, use merge_monotonic for conflict resolution.
                if record.session.epoch > existing.session.epoch {
                    // Higher epoch wins outright
                    *existing = record;
                } else if record.session.epoch == existing.session.epoch {
                    // Same epoch: merge monotonically
                    *existing = existing.clone().merge_monotonic(record);
                } else {
                    // Lower epoch: stale write. Return a Conflict error so the
                    // caller knows its in-memory mutation was not persisted and
                    // can surface the inconsistency instead of silently
                    // diverging from durable state.
                    return Err(
                        sdkwork_communication_rtc_service::RtcContractError::Conflict(format!(
                            "stale epoch rejected: existing={} incoming={}",
                            existing.session.epoch, record.session.epoch
                        )),
                    );
                }
            }
            None => {
                // No existing record: insert directly
                drop(self.states.insert(key, record));
            }
        }
        Ok(())
    }

    fn clear_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<bool, sdkwork_communication_rtc_service::RtcContractError> {
        Ok(self
            .states
            .remove(rtc_session_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

impl Default for CallingRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryStateStore::default()))
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(crate::app::build_default_calling_runtime()),
    }
}

fn signal_index(signals: Vec<SignalEvent>) -> BTreeMap<u64, SignalEvent> {
    signals.into_iter().map(|s| (s.signal_seq, s)).collect()
}

/// Map the IM call's `rtc_mode` string to the RTC media session mode enum.
/// The RTC `RtcMediaSessionMode` enum has `Audio`, `Video`, and `Live`.
/// Unknown values default to `Video` (the most common audio+video call) to
/// preserve forward compatibility with newer call modes that IM may
/// introduce before RTC is updated.
fn parse_rtc_media_session_mode(rtc_mode: &str) -> RtcMediaSessionMode {
    match rtc_mode.to_ascii_lowercase().as_str() {
        "audio" | "voice" => RtcMediaSessionMode::Audio,
        "live" | "broadcast" => RtcMediaSessionMode::Live,
        // `video`, `audiovideo`, `audio-video`, and any unknown mode map to
        // `Video` (audio + video双向通话).
        _ => RtcMediaSessionMode::Video,
    }
}

/// Map RTC contract errors to IM calling errors, preserving status semantics.
fn map_rtc_contract_error(error: RtcContractError) -> CallingError {
    let (status, code, message) = match error {
        RtcContractError::Conflict(detail) => (
            axum::http::StatusCode::CONFLICT,
            "rtc_conflict",
            detail,
        ),
        RtcContractError::UnsupportedCapability(detail) => (
            axum::http::StatusCode::BAD_REQUEST,
            "rtc_unsupported_capability",
            detail,
        ),
        RtcContractError::Unavailable(detail) => (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "rtc_unavailable",
            detail,
        ),
    };
    CallingError {
        status,
        code,
        message: format!("RTC provider error: {message}"),
    }
}

/// Check if the principal is authorized to mutate the session.
///
/// Authorization rules (per SECURITY_SPEC §4.2):
/// - Initiator is always authorized
/// - Invited participants are authorized
/// - Accepted participants are authorized
/// - Principals with `im.calls.credentials.issue` permission are authorized
fn is_authorized_for_session(auth: &AppContext, session: &Session) -> bool {
    // Initiator is always authorized
    if auth.actor_id == session.initiator_id && auth.actor_kind == session.initiator_kind {
        return true;
    }

    // Permission-based authorization (for operators/admins)
    if auth.has_permission("im.calls.credentials.issue") {
        return true;
    }

    // Check if in invited participants
    if session.participants.invited_ids.contains(&auth.actor_id) {
        return true;
    }

    // Check if already accepted
    if session.participants.accepted_ids.contains(&auth.actor_id) {
        return true;
    }

    false
}

/// Check if a `participant_id` is a member of the session.
///
/// A participant is the initiator or appears in `invited_ids`/`accepted_ids`.
/// Used by `issue_participant_credential` to prevent issuing media
/// credentials for principals who are not part of the call (IDOR fix per
/// SECURITY_SPEC §4.3 — credential issuance must be scoped to session
/// participants).
fn is_session_participant(participant_id: &str, session: &Session) -> bool {
    if participant_id == session.initiator_id {
        return true;
    }
    if session.participants.invited_ids.iter().any(|p| p == participant_id) {
        return true;
    }
    if session.participants.accepted_ids.iter().any(|p| p == participant_id) {
        return true;
    }
    false
}

// Re-export helpers used by other modules.
pub(crate) use crate::helpers::{
    resolve_rtc_signal_sender, call_session_matches_create_request,
};
