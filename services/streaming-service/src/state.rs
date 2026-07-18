use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use im_app_context::AppContext;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_stream::{
    StreamAppendOutcome, StreamCreateOutcome, StreamScope, StreamSessionRecord, StreamStateStore,
    StreamTransitionOutcome,
};
use sdkwork_utils_rust::{SdkWorkPageData, cursor_list_page_data};

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameOutcome, AppendStreamFrameRequest,
    CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest,
    StreamSessionMutationOutcome,
};
use crate::error::StreamingError;
use crate::helpers::{
    ensure_stream_session_actor_access, lock_stream_mutex, resolve_stream_frame_sender,
    stream_abort_matches_request, stream_checkpoint_matches_request,
    stream_completion_matches_request, stream_scope_key, stream_session_matches_open_request,
    validate_abort_stream_request_payload_size, validate_append_frame_request_payload_size,
    validate_complete_stream_request_payload_size, validate_open_stream_request_payload_size,
    validate_stream_frame_page_size, validate_stream_id,
};
use crate::metrics::StreamRuntimeMetrics;

const MAX_ACTIVE_STREAMS_PER_TENANT_ORGANIZATION: u64 = 1000;
const MAX_CONCURRENCY_RETRIES: usize = 8;

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<StreamingRuntime>,
}

impl AppState {
    pub fn new(runtime: Arc<StreamingRuntime>) -> Self {
        Self { runtime }
    }
}

pub struct StreamingRuntime {
    pub(crate) state_store: Arc<dyn StreamStateStore>,
    metrics: StreamRuntimeMetrics,
}

impl StreamingRuntime {
    pub fn with_store(state_store: Arc<dyn StreamStateStore>) -> Self {
        Self {
            state_store,
            metrics: StreamRuntimeMetrics::default(),
        }
    }

    pub fn check_store_ready(&self) -> Result<(), String> {
        self.state_store.check_ready().map_err(|error| {
            self.metrics.record_readiness_failure();
            format!("stream state store readiness failed: {error:?}")
        })
    }

    pub fn render_runtime_metrics_prometheus(&self) -> String {
        self.metrics.render_prometheus()
    }

    fn store_result<T>(&self, result: Result<T, ContractError>) -> Result<T, StreamingError> {
        result.map_err(|error| {
            self.metrics.record_store_error();
            StreamingError::stream_store(error)
        })
    }

    fn scope(auth: &AppContext, stream_id: &str) -> StreamScope {
        StreamScope::new(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            stream_id,
        )
    }

    fn load_record(
        &self,
        auth: &AppContext,
        stream_id: &str,
    ) -> Result<StreamSessionRecord, StreamingError> {
        validate_stream_id(stream_id)?;
        self.store_result(self.state_store.load_session(&Self::scope(auth, stream_id)))?
            .ok_or_else(|| StreamingError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "stream_not_found",
                message: format!("stream not found: {stream_id}"),
            })
    }

    pub fn session(
        &self,
        auth: &AppContext,
        stream_id: &str,
    ) -> Result<StreamSession, StreamingError> {
        let record = self.load_record(auth, stream_id)?;
        ensure_stream_session_actor_access(&record.session, auth, stream_id)?;
        Ok(record.session)
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
        let scope = Self::scope(auth, request.stream_id.as_str());
        let now = utc_now_rfc3339_millis();
        let session = StreamSession {
            tenant_id: auth.tenant_id.clone(),
            stream_id: request.stream_id.clone(),
            owner_principal_id: auth.actor_id.clone(),
            owner_principal_kind: auth.actor_kind.clone(),
            stream_type: request.stream_type.clone(),
            scope_kind: request.scope_kind.clone(),
            scope_id: request.scope_id.clone(),
            durability_class: durability_class.clone(),
            ordering_scope: "stream".into(),
            schema_ref: request.schema_ref.clone(),
            state: StreamSessionState::Opened,
            last_frame_seq: 0,
            last_checkpoint_seq: None,
            result_message_id: None,
            complete_frame_seq: None,
            abort_frame_seq: None,
            abort_reason: None,
            opened_at: now.clone(),
            closed_at: None,
            expires_at: None,
        };
        let record = StreamSessionRecord {
            scope,
            session: session.clone(),
            version: 1,
            updated_at: now,
        };
        match self.store_result(
            self.state_store
                .create_session(record, MAX_ACTIVE_STREAMS_PER_TENANT_ORGANIZATION),
        )? {
            StreamCreateOutcome::Applied(record) => Ok(StreamSessionMutationOutcome {
                session: record.session,
                applied: true,
            }),
            StreamCreateOutcome::Existing(record) => {
                if stream_session_matches_open_request(
                    &record.session,
                    auth,
                    &request,
                    &durability_class,
                ) {
                    Ok(StreamSessionMutationOutcome {
                        session: record.session,
                        applied: false,
                    })
                } else {
                    Err(StreamingError::conflict(request.stream_id.as_str()))
                }
            }
            StreamCreateOutcome::CapacityExceeded => {
                self.metrics.record_capacity_rejection();
                Err(StreamingError {
                    status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                    code: "stream_capacity_exceeded",
                    message: format!(
                        "tenant organization already has {MAX_ACTIVE_STREAMS_PER_TENANT_ORGANIZATION} active streams"
                    ),
                })
            }
        }
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
        let _append_timer = self.metrics.track_append();
        validate_append_frame_request_payload_size(&request)?;
        if request.frame_seq == 0 {
            return Err(StreamingError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_frame_seq",
                message: "frameSeq must start from 1".into(),
            });
        }
        let sender = resolve_stream_frame_sender(auth);
        for _ in 0..MAX_CONCURRENCY_RETRIES {
            let current = self.load_record(auth, stream_id)?;
            ensure_stream_session_actor_access(&current.session, auth, stream_id)?;
            if matches!(
                current.session.state,
                StreamSessionState::Completed | StreamSessionState::Aborted
            ) {
                return Err(StreamingError {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "stream_state_invalid",
                    message: format!("stream is already closed: {stream_id}"),
                });
            }
            if request.frame_seq > current.session.last_frame_seq + 1 {
                return Err(StreamingError {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "stream_frame_out_of_order",
                    message: format!(
                        "expected next frame seq {}, got {}",
                        current.session.last_frame_seq + 1,
                        request.frame_seq
                    ),
                });
            }
            let frame = StreamFrame {
                tenant_id: auth.tenant_id.clone(),
                stream_id: stream_id.to_owned(),
                stream_type: current.session.stream_type.clone(),
                scope_kind: current.session.scope_kind.clone(),
                scope_id: current.session.scope_id.clone(),
                frame_seq: request.frame_seq,
                frame_type: request.frame_type.clone(),
                schema_ref: request.schema_ref.clone(),
                encoding: request.encoding.clone(),
                payload: request.payload.clone(),
                sender: sender.clone(),
                attributes: request.attributes.clone(),
                occurred_at: utc_now_rfc3339_millis(),
            };
            let mut next = current.clone();
            next.version = current.version + 1;
            next.updated_at = utc_now_rfc3339_millis();
            next.session.last_frame_seq = frame.frame_seq;
            next.session.state = StreamSessionState::Active;
            match self.store_result(self.state_store.append_frame(
                current.version,
                next,
                frame.clone(),
            ))? {
                StreamAppendOutcome::Applied { frame, .. } => {
                    return Ok(AppendStreamFrameOutcome {
                        frame,
                        applied: true,
                    });
                }
                StreamAppendOutcome::Existing {
                    frame: existing, ..
                } => {
                    let same = existing.frame_type == request.frame_type
                        && existing.schema_ref == request.schema_ref
                        && existing.encoding == request.encoding
                        && existing.payload == request.payload
                        && existing.sender == sender
                        && existing.attributes == request.attributes;
                    return if same {
                        Ok(AppendStreamFrameOutcome {
                            frame: existing,
                            applied: false,
                        })
                    } else {
                        Err(StreamingError {
                            status: axum::http::StatusCode::CONFLICT,
                            code: "stream_frame_conflict",
                            message: format!("frame seq conflict: {}", request.frame_seq),
                        })
                    };
                }
                StreamAppendOutcome::VersionConflict => {
                    self.metrics.record_append_version_conflict();
                    continue;
                }
            }
        }
        self.metrics.record_concurrency_exhausted();
        Err(concurrency_exhausted(stream_id))
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
        self.transition(auth, stream_id, |session| {
            if session.last_checkpoint_seq == Some(request.frame_seq)
                && stream_checkpoint_matches_request(session, auth, &request)
            {
                return Ok(false);
            }
            if matches!(
                session.state,
                StreamSessionState::Completed | StreamSessionState::Aborted
            ) {
                return Err(StreamingError::conflict(stream_id));
            }
            if request.frame_seq < session.last_checkpoint_seq.unwrap_or(0) {
                return Err(StreamingError::conflict(stream_id));
            }
            session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
            session.last_checkpoint_seq = Some(request.frame_seq);
            session.state = StreamSessionState::Checkpointed;
            Ok(true)
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
        self.transition(auth, stream_id, |session| {
            if session.state == StreamSessionState::Completed {
                return if stream_completion_matches_request(session, auth, &request) {
                    Ok(false)
                } else {
                    Err(StreamingError::conflict(stream_id))
                };
            }
            if session.state == StreamSessionState::Aborted {
                return Err(StreamingError::conflict(stream_id));
            }
            session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
            session.result_message_id = request.result_message_id.clone();
            session.complete_frame_seq = Some(request.frame_seq);
            session.state = StreamSessionState::Completed;
            session.closed_at = Some(utc_now_rfc3339_millis());
            Ok(true)
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
        self.transition(auth, stream_id, |session| {
            if session.state == StreamSessionState::Aborted {
                return if stream_abort_matches_request(session, auth, &request) {
                    Ok(false)
                } else {
                    Err(StreamingError::conflict(stream_id))
                };
            }
            if session.state == StreamSessionState::Completed {
                return Err(StreamingError::conflict(stream_id));
            }
            session.last_frame_seq = session
                .last_frame_seq
                .max(request.frame_seq.unwrap_or(session.last_frame_seq));
            session.state = StreamSessionState::Aborted;
            session.abort_frame_seq = request.frame_seq;
            session.abort_reason = request.reason.clone();
            session.closed_at = Some(utc_now_rfc3339_millis());
            Ok(true)
        })
    }

    fn transition<F>(
        &self,
        auth: &AppContext,
        stream_id: &str,
        mut apply: F,
    ) -> Result<StreamSessionMutationOutcome, StreamingError>
    where
        F: FnMut(&mut StreamSession) -> Result<bool, StreamingError>,
    {
        for _ in 0..MAX_CONCURRENCY_RETRIES {
            let current = self.load_record(auth, stream_id)?;
            ensure_stream_session_actor_access(&current.session, auth, stream_id)?;
            let mut next = current.clone();
            if !apply(&mut next.session)? {
                return Ok(StreamSessionMutationOutcome {
                    session: current.session,
                    applied: false,
                });
            }
            next.version = current.version + 1;
            next.updated_at = utc_now_rfc3339_millis();
            match self.store_result(self.state_store.transition_session(current.version, next))? {
                StreamTransitionOutcome::Applied(record) => {
                    return Ok(StreamSessionMutationOutcome {
                        session: record.session,
                        applied: true,
                    });
                }
                StreamTransitionOutcome::VersionConflict => {
                    self.metrics.record_transition_version_conflict();
                    continue;
                }
            }
        }
        self.metrics.record_concurrency_exhausted();
        Err(concurrency_exhausted(stream_id))
    }

    pub fn list_frames(
        &self,
        auth: &AppContext,
        stream_id: &str,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<SdkWorkPageData<StreamFrame>, StreamingError> {
        let page_size = validate_stream_frame_page_size(page_size)?;
        let record = self.load_record(auth, stream_id)?;
        ensure_stream_session_actor_access(&record.session, auth, stream_id)?;
        let mut items = self.store_result(self.state_store.list_frames_after(
            &record.scope,
            after_frame_seq,
            page_size + 1,
        ))?;
        let has_more = items.len() > page_size;
        if has_more {
            items.truncate(page_size);
        }
        let next_cursor = has_more
            .then(|| items.last().map(|frame| frame.frame_seq.to_string()))
            .flatten();
        self.metrics.record_frame_page(items.len());
        Ok(cursor_list_page_data(
            items,
            page_size,
            next_cursor,
            has_more,
        ))
    }
}

fn concurrency_exhausted(stream_id: &str) -> StreamingError {
    StreamingError {
        status: axum::http::StatusCode::CONFLICT,
        code: "stream_concurrency_conflict",
        message: format!("stream changed concurrently; retry request: {stream_id}"),
    }
}

#[derive(Default)]
struct MemoryStreamState {
    sessions: HashMap<String, StreamSessionRecord>,
    frames: HashMap<String, BTreeMap<u64, StreamFrame>>,
}

#[derive(Default)]
pub(crate) struct RuntimeMemoryStreamStateStore {
    state: Mutex<MemoryStreamState>,
}

impl StreamStateStore for RuntimeMemoryStreamStateStore {
    fn check_ready(&self) -> Result<(), ContractError> {
        drop(lock_stream_mutex(
            &self.state,
            "runtime stream store readiness",
        ));
        Ok(())
    }

    fn load_session(
        &self,
        scope: &StreamScope,
    ) -> Result<Option<StreamSessionRecord>, ContractError> {
        Ok(lock_stream_mutex(&self.state, "stream state store")
            .sessions
            .get(scope_key(scope).as_str())
            .cloned())
    }

    fn create_session(
        &self,
        record: StreamSessionRecord,
        max_active_streams: u64,
    ) -> Result<StreamCreateOutcome, ContractError> {
        let mut state = lock_stream_mutex(&self.state, "stream state store");
        let key = scope_key(&record.scope);
        if let Some(existing) = state.sessions.get(key.as_str()) {
            return Ok(StreamCreateOutcome::Existing(existing.clone()));
        }
        let active = state
            .sessions
            .values()
            .filter(|candidate| {
                candidate.scope.tenant_id == record.scope.tenant_id
                    && candidate.scope.organization_id == record.scope.organization_id
                    && !matches!(
                        candidate.session.state,
                        StreamSessionState::Completed
                            | StreamSessionState::Aborted
                            | StreamSessionState::Expired
                    )
            })
            .count() as u64;
        if active >= max_active_streams {
            return Ok(StreamCreateOutcome::CapacityExceeded);
        }
        state.sessions.insert(key, record.clone());
        Ok(StreamCreateOutcome::Applied(record))
    }

    fn append_frame(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
        frame: StreamFrame,
    ) -> Result<StreamAppendOutcome, ContractError> {
        let mut state = lock_stream_mutex(&self.state, "stream state store");
        let key = scope_key(&next_session.scope);
        let Some(current) = state.sessions.get(key.as_str()).cloned() else {
            return Err(ContractError::Invalid(
                "stream session does not exist".into(),
            ));
        };
        if let Some(existing) = state
            .frames
            .get(key.as_str())
            .and_then(|frames| frames.get(&frame.frame_seq))
            .cloned()
        {
            return Ok(StreamAppendOutcome::Existing {
                session: current,
                frame: existing,
            });
        }
        if current.version != expected_version {
            return Ok(StreamAppendOutcome::VersionConflict);
        }
        state
            .frames
            .entry(key.clone())
            .or_default()
            .insert(frame.frame_seq, frame.clone());
        state.sessions.insert(key, next_session.clone());
        Ok(StreamAppendOutcome::Applied {
            session: next_session,
            frame,
        })
    }

    fn transition_session(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
    ) -> Result<StreamTransitionOutcome, ContractError> {
        let mut state = lock_stream_mutex(&self.state, "stream state store");
        let key = scope_key(&next_session.scope);
        if state
            .sessions
            .get(key.as_str())
            .map(|record| record.version)
            != Some(expected_version)
        {
            return Ok(StreamTransitionOutcome::VersionConflict);
        }
        state.sessions.insert(key, next_session.clone());
        Ok(StreamTransitionOutcome::Applied(next_session))
    }

    fn list_frames_after(
        &self,
        scope: &StreamScope,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<Vec<StreamFrame>, ContractError> {
        let state = lock_stream_mutex(&self.state, "stream state store");
        Ok(state
            .frames
            .get(scope_key(scope).as_str())
            .into_iter()
            .flat_map(|frames| {
                frames.range((
                    std::ops::Bound::Excluded(after_frame_seq),
                    std::ops::Bound::Unbounded,
                ))
            })
            .take(page_size)
            .map(|(_, frame)| frame.clone())
            .collect())
    }

    fn clear_stream(&self, scope: &StreamScope) -> Result<bool, ContractError> {
        let mut state = lock_stream_mutex(&self.state, "stream state store");
        let key = scope_key(scope);
        state.frames.remove(key.as_str());
        Ok(state.sessions.remove(key.as_str()).is_some())
    }
}

fn scope_key(scope: &StreamScope) -> String {
    stream_scope_key(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.stream_id.as_str(),
    )
}

impl Default for StreamingRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryStreamStateStore::default()))
    }
}
