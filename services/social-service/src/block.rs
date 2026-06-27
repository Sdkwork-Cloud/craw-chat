use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use im_app_context::AppContext;
use im_domain_core::social::{BlockScope, UserBlock, UserBlockStatus, normalize_user_pair};
use im_domain_events::social::{
    SocialCommitEnvelopeInput, SocialEventType, UserBlockedPayload, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use serde::{Deserialize, Serialize};

use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::{
    SocialRuntime, SocialWritePersistence, StoredUserBlock, active_user_block_for_scope,
};

const MAX_ID_BYTES: usize = 256;
const MAX_TIMESTAMP_BYTES: usize = 64;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BlockUserRequest {
    pub(crate) block_id: String,
    pub(crate) event_id: String,
    pub(crate) blocker_user_id: String,
    pub(crate) blocked_user_id: String,
    pub(crate) scope: BlockScope,
    pub(crate) direct_chat_id: Option<String>,
    pub(crate) expires_at: Option<String>,
    pub(crate) effective_at: String,
}

// ---------------------------------------------------------------------------
// Result types (business logic layer)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct BlockedUser {
    pub(crate) user_block: UserBlock,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

// ---------------------------------------------------------------------------
// HTTP response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialUserBlockWriteStatus {
    Blocked,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialUserBlockReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialUserBlockCommitResponse {
    status: SocialUserBlockWriteStatus,
    user_block: UserBlock,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialUserBlockSnapshotResponse {
    status: SocialUserBlockReadStatus,
    user_block: UserBlock,
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

fn validate_optional_payload_size(
    field: &'static str,
    value: Option<&str>,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if let Some(value) = value {
        validate_payload_size(field, value, max_bytes)?;
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
// Business logic: SocialRuntime block methods
// ---------------------------------------------------------------------------

impl SocialRuntime {
    pub(crate) fn block_user(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BlockUserRequest,
    ) -> Result<BlockedUser, SocialServiceError> {
        validate_payload_size("blockId", request.block_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "blockerUserId",
            request.blocker_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "blockedUserId",
            request.blocked_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "directChatId",
            request.direct_chat_id.as_deref(),
            MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "expiresAt",
            request.expires_at.as_deref(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_payload_size(
            "effectiveAt",
            request.effective_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("blockId", request.block_id.as_str(), "invalid_user_block")?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_user_block")?;
        validate_required_with_code(
            "blockerUserId",
            request.blocker_user_id.as_str(),
            "invalid_user_block",
        )?;
        validate_required_with_code(
            "blockedUserId",
            request.blocked_user_id.as_str(),
            "invalid_user_block",
        )?;
        validate_required_with_code(
            "effectiveAt",
            request.effective_at.as_str(),
            "invalid_user_block",
        )?;
        normalize_user_pair(
            request.blocker_user_id.as_str(),
            request.blocked_user_id.as_str(),
        )
        .map_err(|error| SocialServiceError::invalid("invalid_user_block", error.to_string()))?;

        if matches!(request.scope, BlockScope::DirectChat) {
            validate_required_with_code(
                "directChatId",
                request.direct_chat_id.as_deref().unwrap_or_default(),
                "invalid_user_block",
            )?;
        }

        let scope = serde_json::to_string(&request.scope)
            .expect("user block scope should serialize")
            .trim_matches('"')
            .to_owned();
        let payload = UserBlockedPayload {
            block_id: request.block_id.clone(),
            blocker_user_id: request.blocker_user_id.clone(),
            blocked_user_id: request.blocked_user_id.clone(),
            scope,
            direct_chat_id: request.direct_chat_id.clone(),
            expires_at: request.expires_at.clone(),
            effective_at: request.effective_at.clone(),
        };
        let payload_json =
            serde_json::to_string(&payload).expect("user block payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::UserBlock,
            aggregate_id: request.block_id.as_str(),
            event_type: SocialEventType::UserBlocked,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.effective_at.as_str(),
            committed_at: request.effective_at.as_str(),
            payload: payload_json.as_str(),
        });
        let user_block = UserBlock {
            tenant_id: tenant_id.into(),
            block_id: request.block_id.clone(),
            blocker_user_id: request.blocker_user_id,
            blocked_user_id: request.blocked_user_id,
            scope: request.scope,
            status: UserBlockStatus::Active,
            direct_chat_id: request.direct_chat_id,
            expires_at: request.expires_at,
            created_at: request.effective_at.clone(),
            updated_at: request.effective_at,
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
                    crate::runtime::SocialCommittedEvent::UserBlock { record, commit } => {
                        Ok(BlockedUser {
                            user_block: record.user_block,
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
            .user_blocks
            .contains_key(user_block.block_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "user_block_conflict",
                format!("user block {} already exists", user_block.block_id),
            ));
        }
        if let Some(direct_chat_id) = user_block.direct_chat_id.as_deref() {
            let direct_chat = next_state
                .direct_chats
                .get(direct_chat_id)
                .filter(|record| record.direct_chat.tenant_id == tenant_id)
                .filter(|record| record.direct_chat.status.is_active())
                .ok_or_else(|| {
                    SocialServiceError::invalid(
                        "invalid_user_block",
                        format!("direct chat {direct_chat_id} does not exist or is not active"),
                    )
                })?;
            let direct_chat_pair = normalize_user_pair(
                direct_chat.direct_chat.left_actor_id.as_str(),
                direct_chat.direct_chat.right_actor_id.as_str(),
            )
            .map_err(|error| {
                SocialServiceError::invalid(
                    "invalid_user_block",
                    format!("direct chat {direct_chat_id} cannot be used for user block: {error}"),
                )
            })?;
            let block_pair = user_block.user_pair().map_err(|error| {
                SocialServiceError::invalid("invalid_user_block", error.to_string())
            })?;
            if direct_chat_pair != block_pair {
                return Err(SocialServiceError::invalid(
                    "invalid_user_block",
                    format!(
                        "direct chat {direct_chat_id} does not match block pair {}",
                        block_pair.pair_key()
                    ),
                ));
            }
        }
        if active_user_block_for_scope(
            &next_state,
            tenant_id,
            user_block.blocker_user_id.as_str(),
            user_block.blocked_user_id.as_str(),
            &user_block.scope,
            user_block.direct_chat_id.as_deref(),
        )
        .is_some()
        {
            return Err(SocialServiceError::conflict(
                "user_block_scope_conflict",
                format!(
                    "active user block already exists for {} -> {} scope {:?}",
                    user_block.blocker_user_id, user_block.blocked_user_id, user_block.scope
                ),
            ));
        }

        next_state.insert_user_block_record(
            user_block.block_id.clone(),
            StoredUserBlock {
                user_block: user_block.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BlockedUser {
            user_block,
            latest_commit: commit,
            persistence,
        })
    }
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn block_user(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BlockUserRequest>,
) -> Result<Json<SocialUserBlockCommitResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let blocked = state
        .social_runtime
        .block_user(auth.tenant_id.as_str(), &auth, request)?;

    Ok(Json(SocialUserBlockCommitResponse {
        status: SocialUserBlockWriteStatus::Blocked,
        user_block: blocked.user_block,
        latest_commit: blocked.latest_commit.into(),
        persistence: blocked.persistence,
    }))
}

pub(crate) async fn user_block_snapshot(
    Path(block_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialUserBlockSnapshotResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .user_block_snapshot(auth.tenant_id.as_str(), block_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "user_block_not_found",
                format!("user block {block_id} was not found"),
            )
        })?;

    Ok(Json(SocialUserBlockSnapshotResponse {
        status: SocialUserBlockReadStatus::Snapshot,
        user_block: snapshot.user_block,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}
