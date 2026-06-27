use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex};

use im_app_context::AppContext;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_stream::{StreamStateRecord, StreamStateStore};

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameOutcome, AppendStreamFrameRequest,
    CheckpointStreamRequest, CompleteStreamRequest, ListStreamFramesQuery, OpenStreamRequest,
    StreamFrameWindow, StreamSessionMutationOutcome,
};
use crate::error::StreamingError;
use crate::helpers::{
    ensure_stream_session_actor_access, lock_stream_mutex, resolve_stream_frame_sender,
    stream_abort_matches_request, stream_checkpoint_matches_request,
    stream_completion_matches_request, stream_frame_index, stream_scope_key,
    stream_session_matches_open_request, validate_abort_stream_request_payload_size,
    validate_append_frame_request_payload_size, validate_complete_stream_request_payload_size,
    validate_open_stream_request_payload_size, validate_stream_id,
};

const STREAM_FRAME_LIST_MAX_LIMIT: usize = 1000;

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<StreamingRuntime>,
}

pub struct StreamingRuntime {
    pub(crate) sessions: Mutex<HashMap<String, StreamSession>>,
    pub(crate) frames: Mutex<HashMap<String, BTreeMap<u64, StreamFrame>>>,
    pub(crate) state_store: Arc<dyn StreamStateStore>,
}

impl StreamingRuntime {
    pub fn with_store(state_store: Arc<dyn StreamStateStore>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            frames: Mutex::new(HashMap::new()),
            state_store,
        }
    }

    pub(crate) fn ensure_stream_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<(), StreamingError> {
        validate_stream_id(stream_id)?;
        let scope_key = stream_scope_key(tenant_id, stream_id);
        let needs_restore =
            !lock_stream_mutex(&self.sessions, "stream runtime").contains_key(scope_key.as_str());
        if !needs_restore {
            lock_stream_mutex(&self.frames, "stream runtime")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, stream_id)
            .map_err(StreamingError::stream_store)?;
        if let Some(record) = restored {
            lock_stream_mutex(&self.sessions, "stream runtime")
                .insert(scope_key.clone(), record.session);
            lock_stream_mutex(&self.frames, "stream runtime")
                .insert(scope_key, stream_frame_index(record.frames));
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AppContext,
        stream_id: &str,
    ) -> Result<StreamSession, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let session = lock_stream_mutex(&self.sessions, "stream runtime")
            .get(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .cloned()
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(&session, auth, stream_id)?;
        Ok(session)
    }

    pub fn open_stream(
        &self,
        auth: &AppContext,
        request: OpenStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self.open_stream_with_outcome(auth, request)?.session)
    }

    pub fn open_stream_with_outcome(
        &self,
        auth: &AppContext,
        request: OpenStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_open_stream_request_payload_size(&request)?;
        let durability_class = match request.durability_class.as_str() {
            "transient" => StreamDurabilityClass::Transient,
            "durableSession" => StreamDurabilityClass::DurableSession,
            "eventLog" => StreamDurabilityClass::EventLog,
            other => {
                return Err(StreamingError {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "invalid_durability_class",
                    message: format!("unsupported durability class: {other}"),
                });
            }
        };

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), request.stream_id.as_str());
        self.ensure_stream_state(auth.tenant_id.as_str(), request.stream_id.as_str())?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        if let Some(existing) = sessions.get(scope_key.as_str()).cloned() {
            if stream_session_matches_open_request(&existing, auth, &request, &durability_class) {
                drop(sessions);
                lock_stream_mutex(&self.frames, "stream runtime")
                    .entry(scope_key)
                    .or_default();
                return Ok(StreamSessionMutationOutcome {
                    session: existing,
                    applied: false,
                });
            }

            return Err(StreamingError::conflict(request.stream_id.as_str()));
        }

        let opened_at = utc_now_rfc3339_millis();
        let session = StreamSession {
            tenant_id: auth.tenant_id.clone(),
            stream_id: request.stream_id.clone(),
            owner_principal_id: auth.actor_id.clone(),
            owner_principal_kind: auth.actor_kind.clone(),
            stream_type: request.stream_type,
            scope_kind: request.scope_kind,
            scope_id: request.scope_id,
            durability_class,
            ordering_scope: "stream".into(),
            schema_ref: request.schema_ref,
            state: StreamSessionState::Opened,
            last_frame_seq: 0,
            last_checkpoint_seq: None,
            result_message_id: None,
            complete_frame_seq: None,
            abort_frame_seq: None,
            abort_reason: None,
            opened_at,
            closed_at: None,
            expires_at: None,
        };

        sessions.insert(scope_key.clone(), session.clone());
        drop(sessions);
        lock_stream_mutex(&self.frames, "stream runtime")
            .entry(scope_key)
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), request.stream_id.as_str())?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn append_frame(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AppendStreamFrameRequest,
    ) -> Result<StreamFrame, StreamingError> {
        Ok(self
            .append_frame_with_outcome(auth, stream_id, request)?
            .frame)
    }

    pub fn append_frame_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AppendStreamFrameRequest,
    ) -> Result<AppendStreamFrameOutcome, StreamingError> {
        validate_append_frame_request_payload_size(&request)?;
        if request.frame_seq == 0 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_frame_seq",
                message: "frameSeq must start from 1".into(),
            });
        }

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), stream_id);
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(scope_key.as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        let sender = resolve_stream_frame_sender(auth);

        let mut frames = lock_stream_mutex(&self.frames, "stream runtime");
        let stream_frames = frames.entry(scope_key).or_default();

        if let Some(existing) = stream_frames.get(&request.frame_seq) {
            let is_same_retry = existing.frame_type == request.frame_type
                && existing.schema_ref == request.schema_ref
                && existing.encoding == request.encoding
                && existing.payload == request.payload
                && existing.sender == sender
                && existing.attributes == request.attributes;
            if is_same_retry {
                return Ok(AppendStreamFrameOutcome {
                    frame: existing.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "stream_frame_conflict",
                message: format!("frame seq conflict: {}", request.frame_seq),
            });
        }

        if request.frame_seq != session.last_frame_seq + 1 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_frame_out_of_order",
                message: format!(
                    "expected next frame seq {}, got {}",
                    session.last_frame_seq + 1,
                    request.frame_seq
                ),
            });
        }

        let occurred_at = utc_now_rfc3339_millis();
        let frame = StreamFrame {
            tenant_id: auth.tenant_id.clone(),
            stream_id: session.stream_id.clone(),
            stream_type: session.stream_type.clone(),
            scope_kind: session.scope_kind.clone(),
            scope_id: session.scope_id.clone(),
            frame_seq: request.frame_seq,
            frame_type: request.frame_type,
            schema_ref: request.schema_ref,
            encoding: request.encoding,
            payload: request.payload,
            sender,
            attributes: request.attributes,
            occurred_at,
        };

        stream_frames.insert(frame.frame_seq, frame.clone());
        session.last_frame_seq = frame.frame_seq;
        session.state = StreamSessionState::Active;
        drop(frames);
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(AppendStreamFrameOutcome {
            frame,
            applied: true,
        })
    }

    pub fn checkpoint_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CheckpointStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .checkpoint_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn checkpoint_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CheckpointStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.last_checkpoint_seq == Some(request.frame_seq) {
            if stream_checkpoint_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

        if matches!(
            session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        if session
            .last_checkpoint_seq
            .is_some_and(|last_checkpoint_seq| request.frame_seq < last_checkpoint_seq)
        {
            return Err(StreamingError::conflict(stream_id));
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.last_checkpoint_seq = Some(request.frame_seq);
        session.state = StreamSessionState::Checkpointed;
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn complete_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CompleteStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .complete_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn complete_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CompleteStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_complete_stream_request_payload_size(&request)?;
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.state == StreamSessionState::Completed {
            if stream_completion_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

        if session.state == StreamSessionState::Aborted {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.result_message_id = request.result_message_id;
        session.complete_frame_seq = Some(request.frame_seq);
        session.state = StreamSessionState::Completed;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn abort_stream(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AbortStreamRequest,
    ) -> Result<StreamSession, StreamingError> {
        Ok(self
            .abort_stream_with_outcome(auth, stream_id, request)?
            .session)
    }

    pub fn abort_stream_with_outcome(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AbortStreamRequest,
    ) -> Result<StreamSessionMutationOutcome, StreamingError> {
        validate_abort_stream_request_payload_size(&request)?;
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let mut sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get_mut(stream_scope_key(auth.tenant_id.as_str(), stream_id).as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;

        if session.state == StreamSessionState::Aborted {
            if stream_abort_matches_request(session, auth, &request) {
                return Ok(StreamSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                });
            }
            return Err(StreamingError::conflict(stream_id));
        }

        if session.state == StreamSessionState::Completed {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "stream_state_invalid",
                message: format!("stream is already closed: {stream_id}"),
            });
        }

        if let Some(frame_seq) = request.frame_seq {
            session.last_frame_seq = session.last_frame_seq.max(frame_seq);
        }
        session.state = StreamSessionState::Aborted;
        session.abort_frame_seq = request.frame_seq;
        session.abort_reason = request.reason;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session = session.clone();
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), stream_id)?;

        Ok(StreamSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn list_frames(
        &self,
        auth: &AppContext,
        stream_id: &str,
        query: ListStreamFramesQuery,
    ) -> Result<StreamFrameWindow, StreamingError> {
        self.ensure_stream_state(auth.tenant_id.as_str(), stream_id)?;
        let after_frame_seq = query.after_frame_seq.unwrap_or(0);
        let limit = query.limit.unwrap_or(100);
        if limit == 0 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }
        if limit > STREAM_FRAME_LIST_MAX_LIMIT {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!(
                    "limit must be less than or equal to {STREAM_FRAME_LIST_MAX_LIMIT}"
                ),
            });
        }

        let scope_key = stream_scope_key(auth.tenant_id.as_str(), stream_id);
        let sessions = lock_stream_mutex(&self.sessions, "stream runtime");
        let session = sessions
            .get(scope_key.as_str())
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })?;
        ensure_stream_session_actor_access(session, auth, stream_id)?;
        drop(sessions);

        let frames = lock_stream_mutex(&self.frames, "stream runtime");
        let mut has_more = false;
        let mut items = Vec::new();
        if let Some(stream_frames) = frames.get(scope_key.as_str()) {
            for (_, frame) in stream_frames.range((Excluded(after_frame_seq), Unbounded)) {
                if items.len() == limit {
                    has_more = true;
                    break;
                }
                items.push(frame.clone());
            }
        }
        let next_after_frame_seq = items.last().map(|frame| frame.frame_seq);

        Ok(StreamFrameWindow {
            items,
            next_after_frame_seq,
            has_more,
        })
    }

    fn persist_state(&self, tenant_id: &str, stream_id: &str) -> Result<(), StreamingError> {
        self.state_store
            .save_state(self.state_record(tenant_id, stream_id)?)
            .map_err(StreamingError::stream_store)
    }

    fn state_record(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<StreamStateRecord, StreamingError> {
        let scope_key = stream_scope_key(tenant_id, stream_id);
        let session = lock_stream_mutex(&self.sessions, "stream runtime")
            .get(scope_key.as_str())
            .cloned()
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::CONFLICT,
                code: "stream_state_inconsistent",
                message: format!(
                    "stream session missing while persisting state for stream id: {stream_id}"
                ),
            })?;
        let frames = lock_stream_mutex(&self.frames, "stream runtime")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default()
            .into_values()
            .collect();

        Ok(StreamStateRecord {
            tenant_id: tenant_id.into(),
            stream_id: stream_id.into(),
            session,
            frames,
            updated_at: utc_now_rfc3339_millis(),
        })
    }
}

#[derive(Clone, Default)]
pub(crate) struct RuntimeMemoryStreamStateStore {
    pub(crate) states: Arc<Mutex<HashMap<String, StreamStateRecord>>>,
}

impl StreamStateStore for RuntimeMemoryStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        Ok(lock_stream_mutex(&self.states, "stream state store")
            .get(stream_scope_key(tenant_id, stream_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        let key = stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str());
        let mut states = lock_stream_mutex(&self.states, "stream state store");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        Ok(lock_stream_mutex(&self.states, "stream state store")
            .remove(stream_scope_key(tenant_id, stream_id).as_str())
            .is_some())
    }
}

impl Default for StreamingRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryStreamStateStore::default()))
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(StreamingRuntime::default()),
    }
}
