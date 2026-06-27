use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use im_app_context::AppContext;
use im_domain_core::social::{DirectChat, DirectChatStatus, normalize_actor_pair};
use im_domain_events::social::{
    DirectChatBoundPayload, SocialCommitEnvelopeInput, SocialEventType, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use serde::{Deserialize, Serialize};

use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::{
    SocialRuntime, SocialWritePersistence, StoredDirectChat, active_direct_chat_record_for_pair,
};

const MAX_ID_BYTES: usize = 256;
const MAX_TIMESTAMP_BYTES: usize = 64;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BindDirectChatRequest {
    pub(crate) direct_chat_id: String,
    pub(crate) event_id: String,
    pub(crate) left_actor_id: String,
    pub(crate) right_actor_id: String,
    pub(crate) conversation_id: String,
    pub(crate) bound_at: String,
}

// ---------------------------------------------------------------------------
// Result types (business logic layer)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct BoundDirectChat {
    pub(crate) direct_chat: DirectChat,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

// ---------------------------------------------------------------------------
// HTTP response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialDirectChatWriteStatus {
    Bound,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialDirectChatReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialDirectChatCommitResponse {
    status: SocialDirectChatWriteStatus,
    direct_chat: DirectChat,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialDirectChatSnapshotResponse {
    status: SocialDirectChatReadStatus,
    direct_chat: DirectChat,
    commits: Vec<CommitEnvelopeResponse>,
}

// ---------------------------------------------------------------------------
// CommitEnvelope response adapter
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CommitEnvelopeResponse {
    event_id: String,
    tenant_id: String,
    event_type: String,
    event_version: u16,
    aggregate_type: String,
    aggregate_id: String,
    scope_type: String,
    scope_id: String,
    ordering_key: String,
    ordering_seq: u64,
    causation_id: Option<String>,
    correlation_id: Option<String>,
    idempotency_key: Option<String>,
    actor: EventActorResponse,
    occurred_at: String,
    committed_at: String,
    payload_schema: Option<String>,
    payload: String,
    retention_class: String,
    audit_class: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventActorResponse {
    actor_id: String,
    actor_kind: String,
    actor_session_id: Option<String>,
}

impl From<CommitEnvelope> for CommitEnvelopeResponse {
    fn from(value: CommitEnvelope) -> Self {
        Self {
            event_id: value.event_id,
            tenant_id: value.tenant_id,
            event_type: value.event_type,
            event_version: value.event_version,
            aggregate_type: value.aggregate_type.as_wire_value().into(),
            aggregate_id: value.aggregate_id,
            scope_type: value.scope_type,
            scope_id: value.scope_id,
            ordering_key: value.ordering_key,
            ordering_seq: value.ordering_seq,
            causation_id: value.causation_id,
            correlation_id: value.correlation_id,
            idempotency_key: value.idempotency_key,
            actor: EventActorResponse {
                actor_id: value.actor.actor_id,
                actor_kind: value.actor.actor_kind,
                actor_session_id: value.actor.actor_session_id,
            },
            occurred_at: value.occurred_at,
            committed_at: value.committed_at,
            payload_schema: value.payload_schema,
            payload: value.payload,
            retention_class: value.retention_class,
            audit_class: value.audit_class,
        }
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if value.len() > max_bytes {
        return Err(SocialServiceError::payload_too_large(
            field,
            max_bytes,
            value.len(),
        ));
    }
    Ok(())
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), SocialServiceError> {
    if value.trim().is_empty() {
        return Err(SocialServiceError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }
    Ok(())
}

fn social_event_id_conflict_message(
    event_id: &str,
    existing: &crate::runtime::SocialCommittedEvent,
) -> String {
    let committed = existing.commit();
    format!(
        "eventId {} is already committed for {} {}",
        event_id,
        existing.aggregate_label(),
        committed.aggregate_id
    )
}

// ---------------------------------------------------------------------------
// Business logic: SocialRuntime direct chat methods
// ---------------------------------------------------------------------------

impl SocialRuntime {
    pub(crate) fn bind_direct_chat(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BindDirectChatRequest,
    ) -> Result<BoundDirectChat, SocialServiceError> {
        validate_payload_size(
            "directChatId",
            request.direct_chat_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("leftActorId", request.left_actor_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "rightActorId",
            request.right_actor_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "conversationId",
            request.conversation_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size("boundAt", request.bound_at.as_str(), MAX_TIMESTAMP_BYTES)?;
        validate_required_with_code(
            "directChatId",
            request.direct_chat_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_direct_chat")?;
        validate_required_with_code(
            "leftActorId",
            request.left_actor_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code(
            "rightActorId",
            request.right_actor_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code(
            "conversationId",
            request.conversation_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code("boundAt", request.bound_at.as_str(), "invalid_direct_chat")?;
        let pair = normalize_actor_pair(
            request.left_actor_id.as_str(),
            request.right_actor_id.as_str(),
        )
        .map_err(|error| SocialServiceError::invalid("invalid_direct_chat", error.to_string()))?;

        let payload = DirectChatBoundPayload {
            direct_chat_id: request.direct_chat_id.clone(),
            conversation_id: request.conversation_id.clone(),
            left_actor_id: pair.left_actor_id.clone(),
            right_actor_id: pair.right_actor_id.clone(),
            pair_hash: pair.pair_hash.clone(),
            bound_at: request.bound_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("direct chat payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::DirectChat,
            aggregate_id: request.direct_chat_id.as_str(),
            event_type: SocialEventType::DirectChatBound,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.bound_at.as_str(),
            committed_at: request.bound_at.as_str(),
            payload: payload_json.as_str(),
        });
        let direct_chat = DirectChat {
            tenant_id: tenant_id.into(),
            direct_chat_id: request.direct_chat_id.clone(),
            left_actor_id: pair.left_actor_id.clone(),
            right_actor_id: pair.right_actor_id.clone(),
            pair_hash: pair.pair_hash.clone(),
            status: DirectChatStatus::Active,
            conversation_id: Some(request.conversation_id),
            created_at: request.bound_at.clone(),
            updated_at: request.bound_at,
        };

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::DirectChat { record, commit } => {
                        Ok(BoundDirectChat {
                            direct_chat: record.direct_chat,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_message(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .direct_chats
            .contains_key(direct_chat.direct_chat_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "direct_chat_conflict",
                format!("direct chat {} already exists", direct_chat.direct_chat_id),
            ));
        }
        if let Some(existing_direct_chat) = active_direct_chat_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            pair.left_actor_id.as_str(),
            pair.right_actor_id.as_str(),
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "direct_chat_pair_conflict",
                format!(
                    "active direct chat already exists for pair {}",
                    pair.pair_hash
                ),
                serde_json::json!({
                    "existingDirectChatId": existing_direct_chat.direct_chat.direct_chat_id,
                    "existingStatus": existing_direct_chat.direct_chat.status,
                    "leftActorId": existing_direct_chat.direct_chat.left_actor_id,
                    "rightActorId": existing_direct_chat.direct_chat.right_actor_id,
                    "conversationId": existing_direct_chat.direct_chat.conversation_id
                }),
            ));
        }

        next_state.insert_direct_chat_record(
            direct_chat.direct_chat_id.clone(),
            StoredDirectChat {
                direct_chat: direct_chat.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BoundDirectChat {
            direct_chat,
            latest_commit: commit,
            persistence,
        })
    }
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn bind_direct_chat(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatRequest>,
) -> Result<Json<SocialDirectChatCommitResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let bound = state
        .social_runtime
        .bind_direct_chat(auth.tenant_id.as_str(), &auth, request)?;

    Ok(Json(SocialDirectChatCommitResponse {
        status: SocialDirectChatWriteStatus::Bound,
        direct_chat: bound.direct_chat,
        latest_commit: bound.latest_commit.into(),
        persistence: bound.persistence,
    }))
}

pub(crate) async fn direct_chat_snapshot(
    Path(direct_chat_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialDirectChatSnapshotResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .direct_chat_snapshot(auth.tenant_id.as_str(), direct_chat_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "direct_chat_not_found",
                format!("direct chat {direct_chat_id} was not found"),
            )
        })?;

    Ok(Json(SocialDirectChatSnapshotResponse {
        status: SocialDirectChatReadStatus::Snapshot,
        direct_chat: snapshot.direct_chat,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}
