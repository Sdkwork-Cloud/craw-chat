use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex};

use im_app_context::AppContext;
use im_domain_core::rtc::{
    RtcSession, RtcSessionState, RtcSignalEvent, RtcStateRecord, RtcStateStore,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, PostRtcSignalRequest,
    RtcSessionMutationOutcome, UpdateRtcSessionRequest,
};
use crate::error::CallingError;
use crate::helpers::{
    call_session_matches_create_request, lock_call_mutex, resolve_rtc_signal_sender,
    rtc_session_scope_key, validate_create_request_payload_size,
    validate_invite_request_payload_size, validate_post_signal_request_payload_size,
    validate_update_request_payload_size,
};

const SIGNAL_LIST_MAX_LIMIT: usize = 1000;

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<CallingRuntime>,
}

pub struct CallingRuntime {
    pub(crate) sessions: Mutex<HashMap<String, RtcSession>>,
    pub(crate) signals: Mutex<HashMap<String, BTreeMap<u64, RtcSignalEvent>>>,
    pub(crate) state_store: Arc<dyn RtcStateStore>,
}

impl CallingRuntime {
    pub fn with_store(state_store: Arc<dyn RtcStateStore>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            signals: Mutex::new(HashMap::new()),
            state_store,
        }
    }

    pub(crate) fn ensure_rtc_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<(), CallingError> {
        let scope_key = rtc_session_scope_key(tenant_id, rtc_session_id);
        let needs_restore = !lock_call_mutex(&self.sessions, "call runtime")
            .contains_key(scope_key.as_str());
        if !needs_restore {
            lock_call_mutex(&self.signals, "call runtime")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, rtc_session_id)
            .map_err(|err| {
                CallingError::state_store(ContractError::Unavailable(format!(
                    "call state store load failed: {err:?}"
                )))
            })?;
        if let Some(record) = restored {
            lock_call_mutex(&self.sessions, "call runtime")
                .insert(scope_key.clone(), record.session);
            lock_call_mutex(&self.signals, "call runtime")
                .insert(scope_key, signal_index(record.signals));
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
    ) -> Result<RtcSession, CallingError> {
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let session = lock_call_mutex(&self.sessions, "call runtime")
            .get(
                rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str(),
            )
            .cloned()
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;
        Ok(session)
    }

    pub fn create_session(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSession, CallingError> {
        Ok(self
            .create_session_with_outcome(auth, request)?
            .session)
    }

    pub fn create_session_with_outcome(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, CallingError> {
        validate_create_request_payload_size(&request)?;

        let scope_key =
            rtc_session_scope_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
        self.ensure_rtc_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        if let Some(existing) = sessions.get(scope_key.as_str()).cloned() {
            if call_session_matches_create_request(&existing, auth, &request) {
                drop(sessions);
                lock_call_mutex(&self.signals, "call runtime")
                    .entry(scope_key)
                    .or_default();
                return Ok(RtcSessionMutationOutcome {
                    session: existing,
                    applied: false,
                });
            }

            return Err(CallingError::conflict(request.rtc_session_id.as_str()));
        }

        let started_at = utc_now_rfc3339_millis();
        let session = RtcSession {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: request.rtc_session_id.clone(),
            conversation_id: request.conversation_id,
            rtc_mode: request.rtc_mode,
            initiator_id: auth.actor_id.clone(),
            initiator_kind: auth.actor_kind.clone(),
            provider_plugin_id: None,
            provider_session_id: None,
            access_endpoint: None,
            provider_region: None,
            state: RtcSessionState::Started,
            signaling_stream_id: None,
            artifact_message_id: None,
            started_at,
            ended_at: None,
        };

        sessions.insert(scope_key.clone(), session.clone());
        drop(sessions);
        lock_call_mutex(&self.signals, "call runtime")
            .entry(scope_key)
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn invite_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSession, CallingError> {
        Ok(self
            .invite_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn invite_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, CallingError> {
        validate_invite_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if session.state != RtcSessionState::Started {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {}: {rtc_session_id}",
                    session.state.as_wire_value()
                ),
            });
        }

        // Idempotent: if the same signaling_stream_id was already set, return the unmodified session
        if let Some(existing_stream_id) = session.signaling_stream_id.as_deref() {
            if Some(existing_stream_id) == request.signaling_stream_id.as_deref() {
                return Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
        }

        session.signaling_stream_id = request.signaling_stream_id;
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn accept_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, CallingError> {
        Ok(self
            .accept_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn accept_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if session.state == RtcSessionState::Accepted {
            if session.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                return Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session.state != RtcSessionState::Started {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {}: {rtc_session_id}",
                    session.state.as_wire_value()
                ),
            });
        }

        session.state = RtcSessionState::Accepted;
        if let Some(artifact_message_id) = request.artifact_message_id {
            session.artifact_message_id = Some(artifact_message_id);
        }
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn reject_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, CallingError> {
        Ok(self
            .reject_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn reject_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if session.state == RtcSessionState::Rejected {
            if session.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                return Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session.state != RtcSessionState::Started {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already in state {}: {rtc_session_id}",
                    session.state.as_wire_value()
                ),
            });
        }

        session.state = RtcSessionState::Rejected;
        session.ended_at = Some(utc_now_rfc3339_millis());
        if let Some(artifact_message_id) = request.artifact_message_id {
            session.artifact_message_id = Some(artifact_message_id);
        }
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn end_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, CallingError> {
        Ok(self
            .end_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn end_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, CallingError> {
        validate_update_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if session.state == RtcSessionState::Ended {
            if session.artifact_message_id.as_deref() == request.artifact_message_id.as_deref() {
                return Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(CallingError::conflict(rtc_session_id));
        }

        if session.state == RtcSessionState::Rejected {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!("call session is already rejected: {rtc_session_id}"),
            });
        }

        session.state = RtcSessionState::Ended;
        session.ended_at = Some(utc_now_rfc3339_millis());
        if let Some(artifact_message_id) = request.artifact_message_id {
            session.artifact_message_id = Some(artifact_message_id);
        }
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn post_signal(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<RtcSignalEvent, CallingError> {
        validate_post_signal_request_payload_size(&request)?;

        let scope_key = rtc_session_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        self.ensure_rtc_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_call_mutex(&self.sessions, "call runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            })?;

        if matches!(
            session.state,
            RtcSessionState::Ended | RtcSessionState::Rejected
        ) {
            return Err(CallingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "call_session_state_invalid",
                message: format!(
                    "call session is already closed: {rtc_session_id}"
                ),
            });
        }

        let sender = resolve_rtc_signal_sender(auth);

        let mut signals = lock_call_mutex(&self.signals, "call runtime");
        let session_signals = signals.entry(scope_key).or_default();

        let next_signal_seq = session_signals
            .last_key_value()
            .map(|(seq, _)| *seq + 1)
            .unwrap_or(1);

        let occurred_at = utc_now_rfc3339_millis();
        let event = RtcSignalEvent {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: session.rtc_session_id.clone(),
            signal_seq: next_signal_seq,
            conversation_id: session.conversation_id.clone(),
            rtc_mode: session.rtc_mode.clone(),
            signal_type: request.signal_type,
            schema_ref: request.schema_ref,
            payload: request.payload,
            sender,
            signaling_stream_id: request.signaling_stream_id,
            occurred_at,
        };

        session_signals.insert(event.signal_seq, event.clone());
        drop(signals);
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(event)
    }

    pub fn list_signals(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        after_signal_seq: Option<u64>,
        limit: Option<usize>,
    ) -> Result<(Vec<RtcSignalEvent>, bool), CallingError> {
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
        let sessions = lock_call_mutex(&self.sessions, "call runtime");
        if !sessions.contains_key(scope_key.as_str()) {
            return Err(CallingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "call_session_not_found",
                message: format!("call session not found: {rtc_session_id}"),
            });
        }
        drop(sessions);

        let signals = lock_call_mutex(&self.signals, "call runtime");
        let mut has_more = false;
        let mut items: Vec<RtcSignalEvent> = Vec::new();
        if let Some(session_signals) = signals.get(scope_key.as_str()) {
            for (_, event) in session_signals.range::<u64, _>((Excluded(after_signal_seq), Unbounded)) {
                if items.len() == limit {
                    has_more = true;
                    break;
                }
                items.push(event.clone());
            }
        }

        Ok((items, has_more))
    }

    fn persist_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), CallingError> {
        self.state_store
            .save_state(self.state_record(tenant_id, rtc_session_id)?)
            .map_err(|err| {
                CallingError::state_store(ContractError::Unavailable(format!(
                    "call state store save failed: {err:?}"
                )))
            })
    }

    fn state_record(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<RtcStateRecord, CallingError> {
        let scope_key = rtc_session_scope_key(tenant_id, rtc_session_id);
        let session = lock_call_mutex(&self.sessions, "call runtime")
            .get(scope_key.as_str())
            .cloned()
            .ok_or_else(|| CallingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "call_state_inconsistent",
                message: format!(
                    "call session missing while persisting state for session id: {rtc_session_id}"
                ),
            })?;
        let signals: Vec<RtcSignalEvent> = lock_call_mutex(&self.signals, "call runtime")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default()
            .into_values()
            .collect();

        Ok(RtcStateRecord {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            session,
            signals,
            updated_at: utc_now_rfc3339_millis(),
        })
    }
}

#[derive(Clone, Default)]
pub(crate) struct RuntimeMemoryRtcStateStore {
    pub(crate) states: Arc<Mutex<HashMap<String, RtcStateRecord>>>,
}

impl RtcStateStore for RuntimeMemoryRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, sdkwork_communication_rtc_service::RtcContractError> {
        Ok(lock_call_mutex(&self.states, "rtc state store")
            .get(rtc_session_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned())
    }

    fn save_state(
        &self,
        record: RtcStateRecord,
    ) -> Result<(), sdkwork_communication_rtc_service::RtcContractError> {
        let key = rtc_session_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());
        let mut states = lock_call_mutex(&self.states, "rtc state store");
        let next: RtcStateRecord = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
        Ok(())
    }

    fn clear_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<bool, sdkwork_communication_rtc_service::RtcContractError> {
        Ok(lock_call_mutex(&self.states, "rtc state store")
            .remove(rtc_session_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

impl Default for CallingRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()))
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(CallingRuntime::default()),
    }
}

fn signal_index(signals: Vec<RtcSignalEvent>) -> BTreeMap<u64, RtcSignalEvent> {
    signals.into_iter().map(|s| (s.signal_seq, s)).collect()
}