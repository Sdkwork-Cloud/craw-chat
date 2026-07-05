use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_web_core::WebRequestContext;
use im_domain_core::social::{
    BlockScope, FriendRequestStatus, FriendshipStatus, UserBlock, UserBlockStatus, normalize_user_pair,
};
use im_domain_events::social::{
    FriendRequestCanceledPayload, FriendRequestDeclinedPayload, FriendshipRemovedPayload,
    SocialCommitEnvelopeInput, SocialEventType, UserBlockedPayload, UserBlockReleasedPayload,
    social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use serde::{Deserialize, Serialize};

use crate::api_payload::resource_item;
use crate::envelope::finish_enveloped_json;
use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::{
    SocialControlState, SocialRuntime, SocialWritePersistence, StoredUserBlock,
    active_friendship_record_for_pair, active_user_block_for_scope, archive_active_direct_chats_for_pair,
    pending_friend_request_records_for_pair,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReleaseUserBlockRequest {
    pub(crate) event_id: String,
    pub(crate) released_by_user_id: String,
    pub(crate) released_at: String,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OpenApiUserBlockResponse {
    pub(crate) user_block: UserBlock,
    pub(crate) latest_commit: CommitEnvelopeResponse,
    pub(crate) persistence: SocialWritePersistence,
}

// ---------------------------------------------------------------------------
// CommitEnvelope response adapter
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommitEnvelopeResponse {
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
        crate::friendship::ensure_auth_user_matches(
            auth,
            request.blocker_user_id.as_str(),
            "blockerUserId",
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

        let mut commits_to_persist = vec![commit.clone()];
        if matches!(user_block.scope, BlockScope::All | BlockScope::Friendship) {
            if let Ok(pair) = user_block.user_pair() {
                if let Some(existing) = active_friendship_record_for_pair(
                    &next_state,
                    tenant_id,
                    auth.organization_id.as_str(),
                    pair.user_low_id.as_str(),
                    pair.user_high_id.as_str(),
                ) {
                    let friendship_id = existing.friendship.friendship_id.clone();
                    let removed_at = user_block.updated_at.clone();
                    let removal_event_id =
                        format!("{}:friendship-removed", request.event_id.as_str());
                    let removal_payload = FriendshipRemovedPayload {
                        friendship_id: friendship_id.clone(),
                        user_low_id: existing.friendship.user_low_id.clone(),
                        user_high_id: existing.friendship.user_high_id.clone(),
                        removed_by_user_id: user_block.blocker_user_id.clone(),
                        removed_at: removed_at.clone(),
                    };
                    let removal_payload_json = serde_json::to_string(&removal_payload)
                        .expect("friendship removal payload should serialize into json");
                    let removal_commit = social_commit_envelope(SocialCommitEnvelopeInput {
                        event_id: removal_event_id.as_str(),
                        tenant_id,
                        organization_id: auth.organization_id.as_str(),
                        aggregate_type: AggregateType::Friendship,
                        aggregate_id: friendship_id.as_str(),
                        event_type: SocialEventType::FriendshipRemoved,
                        ordering_seq: existing.commits.len() as u64 + 1,
                        actor: EventActor {
                            actor_id: auth.actor_id.clone(),
                            actor_kind: auth.actor_kind.clone(),
                            actor_session_id: auth.session_id.clone(),
                        },
                        occurred_at: removed_at.as_str(),
                        committed_at: removed_at.as_str(),
                        payload: removal_payload_json.as_str(),
                    });
                    let mut record = next_state
                        .friendships
                        .get(friendship_id.as_str())
                        .cloned()
                        .expect("active friendship should exist after lookup");
                    record.friendship.status = FriendshipStatus::Removed;
                    record.friendship.updated_at = removed_at.clone();
                    record.commits.push(removal_commit.clone());
                    next_state.insert_friendship_record(friendship_id, record);
                    archive_active_direct_chats_for_pair(
                        &mut next_state,
                        tenant_id,
                        auth.organization_id.as_str(),
                        pair.user_low_id.as_str(),
                        pair.user_high_id.as_str(),
                        removed_at.as_str(),
                    );
                    commits_to_persist.push(removal_commit);
                }
            }
        }

        if matches!(user_block.scope, BlockScope::All | BlockScope::Friendship) {
            append_block_pending_friend_request_terminations(
                &mut next_state,
                &mut commits_to_persist,
                request.event_id.as_str(),
                &user_block,
                auth,
                tenant_id,
                auth.organization_id.as_str(),
            )?;
        }

        let persistence = if commits_to_persist.len() == 1 {
            self.persist_state_transition(&next_state, &commit)?
        } else {
            self.persist_state_transition_batch(&next_state, commits_to_persist.as_slice())?
        };
        *state = next_state;

        Ok(BlockedUser {
            user_block,
            latest_commit: commit,
            persistence,
        })
    }

    /// Align contact preference `isBlocked` with durable [`UserBlock`] state.
    pub(crate) fn sync_contact_block_preference(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        blocked_user_id: &str,
        should_block: bool,
        block_id: String,
        event_id: String,
    ) -> Result<(), SocialServiceError> {
        let owner_user_id = auth.social_principal_user_id();
        if owner_user_id == blocked_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_contact_block_target",
                "cannot block yourself from contact preferences",
            ));
        }

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let existing = active_user_block_for_scope(
            &state,
            tenant_id,
            owner_user_id,
            blocked_user_id,
            &BlockScope::All,
            None,
        );
        drop(state);

        if should_block {
            if existing.is_some() {
                return Ok(());
            }
            let effective_at = im_time::utc_now_rfc3339_millis();
            self.block_user(
                tenant_id,
                auth,
                BlockUserRequest {
                    block_id,
                    event_id,
                    blocker_user_id: owner_user_id.to_owned(),
                    blocked_user_id: blocked_user_id.to_owned(),
                    scope: BlockScope::All,
                    direct_chat_id: None,
                    expires_at: None,
                    effective_at,
                },
            )?;
            return Ok(());
        }

        if let Some(record) = existing {
            let released_at = im_time::utc_now_rfc3339_millis();
            self.release_user_block(
                tenant_id,
                auth,
                record.user_block.block_id.as_str(),
                ReleaseUserBlockRequest {
                    event_id,
                    released_by_user_id: owner_user_id.to_owned(),
                    released_at,
                },
            )?;
        }
        Ok(())
    }

    pub(crate) fn contact_is_blocked_all_scope(
        &self,
        tenant_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
    ) -> bool {
        let _ = self.refresh_state_from_authority_for_read();
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        active_user_block_for_scope(
            &state,
            tenant_id,
            owner_user_id,
            target_user_id,
            &BlockScope::All,
            None,
        )
        .is_some()
    }

    pub(crate) fn release_user_block(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        block_id: &str,
        request: ReleaseUserBlockRequest,
    ) -> Result<BlockedUser, SocialServiceError> {
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "releasedByUserId",
            request.released_by_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "releasedAt",
            request.released_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_user_block")?;
        validate_required_with_code(
            "releasedByUserId",
            request.released_by_user_id.as_str(),
            "invalid_user_block",
        )?;
        validate_required_with_code(
            "releasedAt",
            request.released_at.as_str(),
            "invalid_user_block",
        )?;
        crate::friendship::ensure_auth_user_matches(
            auth,
            request.released_by_user_id.as_str(),
            "releasedByUserId",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        let stored = next_state
            .user_blocks
            .get(block_id)
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "user_block_not_found",
                    format!("user block {block_id} was not found"),
                )
            })?;
        if stored.user_block.blocker_user_id != request.released_by_user_id {
            return Err(SocialServiceError::forbidden(
                "social_actor_mismatch",
                "releasedByUserId must match the block owner",
            ));
        }
        if !stored.user_block.status.is_active() {
            return Err(SocialServiceError::conflict(
                "user_block_not_active",
                format!("user block {block_id} is not active"),
            ));
        }

        let release_payload = UserBlockReleasedPayload {
            block_id: block_id.to_owned(),
            blocker_user_id: stored.user_block.blocker_user_id.clone(),
            blocked_user_id: stored.user_block.blocked_user_id.clone(),
            released_at: request.released_at.clone(),
        };
        let release_payload_json = serde_json::to_string(&release_payload)
            .expect("user block release payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::UserBlock,
            aggregate_id: block_id,
            event_type: SocialEventType::UserBlockReleased,
            ordering_seq: stored.commits.len() as u64 + 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.released_at.as_str(),
            committed_at: request.released_at.as_str(),
            payload: release_payload_json.as_str(),
        });

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

        let mut record = stored;
        record.user_block.status = UserBlockStatus::Released;
        record.user_block.updated_at = request.released_at.clone();
        record.commits.push(commit.clone());
        next_state.insert_user_block_record(block_id.to_owned(), record.clone());
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BlockedUser {
            user_block: record.user_block,
            latest_commit: commit,
            persistence,
        })
    }
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn block_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<BlockUserRequest>,
) -> Response {
    let result = (|| {
        let blocked = state
            .social_runtime
            .block_user(auth.tenant_id.as_str(), &auth, request)?;

        Ok(resource_item(SocialUserBlockCommitResponse {
            status: SocialUserBlockWriteStatus::Blocked,
            user_block: blocked.user_block,
            latest_commit: blocked.latest_commit.into(),
            persistence: blocked.persistence,
        }))
    })();
    finish_enveloped_json(&ctx, result)
}

pub(crate) async fn user_block_snapshot(
    Path(block_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
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
        let principal = auth.social_principal_user_id();
        if snapshot.user_block.blocker_user_id != principal
            && snapshot.user_block.blocked_user_id != principal
        {
            return Err(SocialServiceError::forbidden(
                "social_actor_mismatch",
                "user block snapshot is restricted to participants",
            ));
        }

        Ok(resource_item(SocialUserBlockSnapshotResponse {
            status: SocialUserBlockReadStatus::Snapshot,
            user_block: snapshot.user_block,
            commits: snapshot.commits.into_iter().map(Into::into).collect(),
        }))
    })();
    finish_enveloped_json(&ctx, result)
}

fn append_block_pending_friend_request_terminations(
    next_state: &mut SocialControlState,
    commits_to_persist: &mut Vec<CommitEnvelope>,
    block_event_id: &str,
    user_block: &UserBlock,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
) -> Result<(), SocialServiceError> {
    let pair = user_block
        .user_pair()
        .map_err(|error| SocialServiceError::invalid("invalid_user_block", error.to_string()))?;
    let pending = pending_friend_request_records_for_pair(
        next_state,
        tenant_id,
        organization_id,
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    for stored in pending {
        let request_id = stored.friend_request.request_id.clone();
        let terminated_at = user_block.updated_at.clone();
        let (event_type, payload_json, next_status) =
            if stored.friend_request.requester_user_id == user_block.blocker_user_id {
                (
                    SocialEventType::FriendRequestCanceled,
                    serde_json::to_string(&FriendRequestCanceledPayload {
                        request_id: request_id.clone(),
                        requester_user_id: stored.friend_request.requester_user_id.clone(),
                        target_user_id: stored.friend_request.target_user_id.clone(),
                        canceled_by_user_id: user_block.blocker_user_id.clone(),
                        canceled_at: terminated_at.clone(),
                    })
                    .expect("friend request cancel payload should serialize into json"),
                    FriendRequestStatus::Canceled,
                )
            } else {
                (
                    SocialEventType::FriendRequestDeclined,
                    serde_json::to_string(&FriendRequestDeclinedPayload {
                        request_id: request_id.clone(),
                        requester_user_id: stored.friend_request.requester_user_id.clone(),
                        target_user_id: stored.friend_request.target_user_id.clone(),
                        declined_by_user_id: user_block.blocker_user_id.clone(),
                        declined_at: terminated_at.clone(),
                    })
                    .expect("friend request decline payload should serialize into json"),
                    FriendRequestStatus::Declined,
                )
            };
        let termination_event_id = format!("{block_event_id}:friend-request-terminated:{request_id}");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: termination_event_id.as_str(),
            tenant_id,
            organization_id,
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id.as_str(),
            event_type,
            ordering_seq: stored.commits.len() as u64 + 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: terminated_at.as_str(),
            committed_at: terminated_at.as_str(),
            payload: payload_json.as_str(),
        });
        let mut record = next_state
            .friend_requests
            .get(request_id.as_str())
            .cloned()
            .expect("pending friend request should exist during block termination");
        record.friend_request.status = next_status;
        record.friend_request.updated_at = terminated_at;
        record.commits.push(commit.clone());
        next_state.insert_friend_request_record(request_id, record);
        commits_to_persist.push(commit);
    }
    Ok(())
}
