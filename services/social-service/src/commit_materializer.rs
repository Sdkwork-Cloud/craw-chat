//! Materialize social commit envelopes into supplemental PostgreSQL stores.

use std::sync::Arc;

use im_adapters_social_postgres::direct_chat_store::{DirectChatRecord, DirectChatStore};
use im_adapters_social_postgres::friend_request_store::{FriendRequestRecord, FriendRequestStore};
use im_adapters_social_postgres::friendship_store::{FriendshipRecord, FriendshipStore};
use im_adapters_social_postgres::user_block_store::{UserBlockRecord, UserBlockStore};
use im_adapters_social_postgres::wire_id::social_entity_id_to_i64;
use im_adapters_social_postgres::SocialPostgresPool;
use im_domain_events::social::{
    DirectChatBoundPayload, FriendRequestAcceptedPayload, FriendRequestCanceledPayload,
    FriendRequestDeclinedPayload, FriendRequestExpiredPayload, FriendRequestSubmittedPayload,
    FriendshipActivatedPayload, FriendshipRemovedPayload, UserBlockedPayload,
    UserBlockReleasedPayload,
};
use im_platform_contracts::CommitEnvelope;

pub struct SocialPostgresMaterializer {
    pool: SocialPostgresPool,
    friend_request_store: Arc<dyn FriendRequestStore>,
    friendship_store: Arc<dyn FriendshipStore>,
    user_block_store: Arc<dyn UserBlockStore>,
    direct_chat_store: Arc<dyn DirectChatStore>,
}

impl SocialPostgresMaterializer {
    pub fn from_pool(pool: SocialPostgresPool) -> Self {
        let pool_arc = Arc::new(pool.inner().clone());
        Self {
            pool,
            friend_request_store: Arc::new(
                im_adapters_social_postgres::friend_request_store::PostgresFriendRequestStore::new(
                    pool_arc.clone(),
                ),
            ),
            friendship_store: Arc::new(
                im_adapters_social_postgres::friendship_store::PostgresFriendshipStore::new(
                    pool_arc.clone(),
                ),
            ),
            user_block_store: Arc::new(
                im_adapters_social_postgres::user_block_store::PostgresUserBlockStore::new(
                    pool_arc.clone(),
                ),
            ),
            direct_chat_store: Arc::new(
                im_adapters_social_postgres::direct_chat_store::PostgresDirectChatStore::new(
                    pool_arc,
                ),
            ),
        }
    }

    pub fn friend_request_store(&self) -> Arc<dyn FriendRequestStore> {
        self.friend_request_store.clone()
    }

    pub fn friendship_store(&self) -> Arc<dyn FriendshipStore> {
        self.friendship_store.clone()
    }

    /// Best-effort replay materialization after journal commit (bootstrap / drift repair).
    pub fn try_materialize_commits(&self, commits: &[CommitEnvelope]) -> usize {
        let mut failures = 0usize;
        for commit in commits {
            if let Err(error) = self.try_materialize_commit(commit) {
                failures += 1;
                tracing::error!(
                    event_id = commit.event_id.as_str(),
                    event_type = commit.event_type.as_str(),
                    error = %error,
                    "social postgres materialization failed for commit"
                );
            }
        }
        failures
    }

    /// Materializes commits and returns the first error (write authority surfaces this before journal append).
    pub fn materialize_commits(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        if commits.len() > 1 {
            return im_adapters_social_postgres::materialize_commits_in_transaction(
                &self.pool,
                commits,
            );
        }
        for commit in commits {
            self.try_materialize_commit(commit)?;
        }
        Ok(())
    }

    /// Best-effort rollback for commits materialized before a journal append failure.
    pub fn compensate_commits(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        for commit in commits.iter().rev() {
            self.try_compensate_commit(commit)?;
        }
        Ok(())
    }

    fn try_compensate_commit(&self, commit: &CommitEnvelope) -> Result<(), String> {
        match commit.event_type.as_str() {
            "friend_request.submitted" => {
                let payload: FriendRequestSubmittedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.submitted payload: {error}")
                    })?;
                self.friend_request_store
                    .delete_by_id(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.request_id.as_str()),
                    )
                    .map_err(|error| format!("friend_request compensation delete failed: {error:?}"))
            }
            "friend_request.accepted" => {
                let payload: FriendRequestAcceptedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.accepted payload: {error}")
                    })?;
                self.friend_request_store
                    .revert_status_for_compensation(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.request_id.as_str()),
                        "pending",
                        payload.accepted_at.as_str(),
                    )
                    .map_err(|error| {
                        format!("friend_request accepted compensation revert failed: {error:?}")
                    })
            }
            "friend_request.declined" | "friend_request.canceled" | "friend_request.expired" => {
                self.compensate_friend_request_terminal_status(commit)
            }
            "friendship.activated" => {
                let payload: FriendshipActivatedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friendship.activated payload: {error}")
                    })?;
                self.friendship_store
                    .update_status(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.friendship_id.as_str()),
                        "removed",
                        payload.established_at.as_str(),
                    )
                    .map_err(|error| {
                        format!("friendship activated compensation revert failed: {error:?}")
                    })
            }
            "friendship.removed" => {
                let payload: FriendshipRemovedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friendship.removed payload: {error}")
                    })?;
                self.friendship_store
                    .update_status(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.friendship_id.as_str()),
                        "active",
                        payload.removed_at.as_str(),
                    )
                    .map_err(|error| format!("friendship removed compensation revert failed: {error:?}"))
            }
            "user_block.blocked" => {
                let payload: UserBlockedPayload = serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid user_block.blocked payload: {error}"))?;
                self.user_block_store
                    .delete_by_blocker(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.block_id.as_str()),
                        payload.blocker_user_id.as_str(),
                    )
                    .map_err(|error| format!("user_block compensation delete failed: {error:?}"))
                    .map(|_| ())
            }
            "user_block.released" => {
                let payload: UserBlockReleasedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid user_block.released payload: {error}")
                    })?;
                let record = UserBlockRecord {
                    tenant_id: commit.tenant_id.clone(),
                    organization_id: commit.organization_id.clone(),
                    block_id: social_entity_id_to_i64(payload.block_id.as_str()),
                    blocker_user_id: payload.blocker_user_id,
                    blocked_user_id: payload.blocked_user_id,
                    scope: "all".to_owned(),
                    direct_chat_id: None,
                    reason: None,
                    expires_at: None,
                    created_at: payload.released_at.clone(),
                    updated_at: payload.released_at,
                };
                self.user_block_store
                    .insert(&record)
                    .map_err(|error| format!("user_block release compensation insert failed: {error:?}"))
            }
            "direct_chat.bound" => {
                let payload: DirectChatBoundPayload = serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid direct_chat.bound payload: {error}"))?;
                self.direct_chat_store
                    .update_status(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.direct_chat_id.as_str()),
                        "closed",
                        payload.bound_at.as_str(),
                    )
                    .map_err(|error| format!("direct_chat compensation close failed: {error:?}"))
            }
            _ => Ok(()),
        }
    }

    fn compensate_friend_request_terminal_status(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let (request_id, updated_at) = match commit.event_type.as_str() {
            "friend_request.declined" => {
                let payload: FriendRequestDeclinedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.declined payload: {error}")
                    })?;
                (
                    extract_request_id_from_aggregate(commit)?,
                    payload.declined_at,
                )
            }
            "friend_request.canceled" => {
                let payload: FriendRequestCanceledPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.canceled payload: {error}")
                    })?;
                (
                    extract_request_id_from_aggregate(commit)?,
                    payload.canceled_at,
                )
            }
            "friend_request.expired" => {
                let payload: FriendRequestExpiredPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.expired payload: {error}")
                    })?;
                (
                    extract_request_id_from_aggregate(commit)?,
                    payload.expired_at,
                )
            }
            _ => return Ok(()),
        };
        self.friend_request_store
            .revert_status_for_compensation(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                request_id,
                "pending",
                updated_at.as_str(),
            )
            .map_err(|error| format!("friend_request terminal compensation revert failed: {error:?}"))
    }

    fn try_materialize_commit(&self, commit: &CommitEnvelope) -> Result<(), String> {
        match commit.event_type.as_str() {
            "friend_request.submitted" => self.materialize_friend_request_submitted(commit),
            "friend_request.accepted" => self.materialize_friend_request_status(commit, "accepted"),
            "friend_request.declined" => self.materialize_friend_request_status(commit, "declined"),
            "friend_request.canceled" => self.materialize_friend_request_status(commit, "canceled"),
            "friend_request.expired" => self.materialize_friend_request_status(commit, "expired"),
            "friendship.activated" => self.materialize_friendship_activated(commit),
            "friendship.removed" => self.materialize_friendship_removed(commit),
            "user_block.blocked" => self.materialize_user_blocked(commit),
            "user_block.released" => self.materialize_user_block_released(commit),
            "direct_chat.bound" => self.materialize_direct_chat_bound(commit),
            _ => Ok(()),
        }
    }

    fn materialize_friend_request_submitted(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: FriendRequestSubmittedPayload =
            serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid friend_request.submitted payload: {error}"))?;
        let record = FriendRequestRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            request_id: social_entity_id_to_i64(payload.request_id.as_str()),
            requester_user_id: payload.requester_user_id,
            target_user_id: payload.target_user_id,
            request_message: payload.request_message,
            status: "pending".to_string(),
            expired_at: payload.expires_at.clone(),
            created_at: payload.requested_at.clone(),
            updated_at: payload.requested_at,
        };
        self.friend_request_store
            .insert(&record)
            .map_err(|error| format!("friend_request insert failed: {error:?}"))
    }

    fn materialize_friend_request_status(
        &self,
        commit: &CommitEnvelope,
        status: &str,
    ) -> Result<(), String> {
        let updated_at = match status {
            "accepted" => {
                let payload: FriendRequestAcceptedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.accepted payload: {error}")
                    })?;
                self.update_friend_request_status_idempotent(
                    commit.tenant_id.as_str(),
                    commit.organization_id.as_str(),
                    social_entity_id_to_i64(payload.request_id.as_str()),
                    status,
                    payload.accepted_at.as_str(),
                )?;
                return Ok(());
            }
            "declined" => {
                let payload: FriendRequestDeclinedPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.declined payload: {error}")
                    })?;
                payload.declined_at
            }
            "canceled" => {
                let payload: FriendRequestCanceledPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.canceled payload: {error}")
                    })?;
                payload.canceled_at
            }
            "expired" => {
                let payload: FriendRequestExpiredPayload =
                    serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                        format!("invalid friend_request.expired payload: {error}")
                    })?;
                payload.expired_at
            }
            _ => return Ok(()),
        };
        let request_id = extract_request_id_from_aggregate(commit)?;
        self.update_friend_request_status_idempotent(
            commit.tenant_id.as_str(),
            commit.organization_id.as_str(),
            request_id,
            status,
            updated_at.as_str(),
        )
    }

    fn update_friend_request_status_idempotent(
        &self,
        tenant_id: &str,
        organization_id: &str,
        request_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), String> {
        match self.friend_request_store.update_status(
            tenant_id,
            organization_id,
            request_id,
            status,
            updated_at,
        ) {
            Ok(()) => Ok(()),
            Err(im_platform_contracts::ContractError::Conflict(_)) => {
                let existing = self
                    .friend_request_store
                    .get_by_id(tenant_id, organization_id, request_id)
                    .map_err(|error| format!("friend_request load failed: {error:?}"))?;
                match existing {
                    Some(record) if record.status == status => Ok(()),
                    Some(record) => Err(format!(
                        "friend_request {request_id} is not pending (current status: {})",
                        record.status
                    )),
                    None => Err(format!("friend_request {request_id} not found for status update")),
                }
            }
            Err(error) => Err(format!("friend_request update failed: {error:?}")),
        }
    }

    fn materialize_friendship_activated(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: FriendshipActivatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid friendship.activated payload: {error}"))?;
        let record = FriendshipRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            friendship_id: social_entity_id_to_i64(payload.friendship_id.as_str()),
            user_low_id: payload.user_low_id,
            user_high_id: payload.user_high_id,
            initiator_user_id: payload.initiator_user_id,
            status: "active".to_string(),
            established_at: Some(payload.established_at.clone()),
            updated_at: payload.established_at,
        };
        self.friendship_store
            .upsert_active(&record)
            .map_err(|error| format!("friendship upsert failed: {error:?}"))
    }

    fn materialize_friendship_removed(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: FriendshipRemovedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid friendship.removed payload: {error}"))?;
        self.friendship_store
            .update_status(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.friendship_id.as_str()),
                "removed",
                payload.removed_at.as_str(),
            )
            .map_err(|error| format!("friendship update failed: {error:?}"))
    }

    fn materialize_user_blocked(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: UserBlockedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid user_block.blocked payload: {error}"))?;
        let record = UserBlockRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            block_id: social_entity_id_to_i64(payload.block_id.as_str()),
            blocker_user_id: payload.blocker_user_id,
            blocked_user_id: payload.blocked_user_id,
            scope: payload.scope,
            direct_chat_id: payload
                .direct_chat_id
                .as_deref()
                .map(social_entity_id_to_i64),
            reason: None,
            expires_at: payload.expires_at,
            created_at: payload.effective_at.clone(),
            updated_at: payload.effective_at,
        };
        self.user_block_store
            .insert(&record)
            .map_err(|error| format!("user_block insert failed: {error:?}"))
    }

    fn materialize_user_block_released(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: UserBlockReleasedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid user_block.released payload: {error}"))?;
        self.user_block_store
            .delete_by_blocker(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.block_id.as_str()),
                payload.blocker_user_id.as_str(),
            )
            .map_err(|error| format!("user_block release failed: {error:?}"))?;
        Ok(())
    }

    fn materialize_direct_chat_bound(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: DirectChatBoundPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid direct_chat.bound payload: {error}"))?;
        let record = DirectChatRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            direct_chat_id: social_entity_id_to_i64(payload.direct_chat_id.as_str()),
            left_actor_kind: "user".to_string(),
            left_actor_id: payload.left_actor_id.clone(),
            right_actor_kind: "user".to_string(),
            right_actor_id: payload.right_actor_id.clone(),
            pair_hash: payload.pair_hash.clone(),
            status: "active".to_string(),
            conversation_id: Some(payload.conversation_id.clone()),
            created_at: payload.bound_at.clone(),
            updated_at: payload.bound_at.clone(),
        };
        self.direct_chat_store
            .insert(&record)
            .map_err(|error| format!("direct_chat insert failed: {error:?}"))
    }
}

fn extract_request_id_from_aggregate(commit: &CommitEnvelope) -> Result<i64, String> {
    Ok(social_entity_id_to_i64(commit.aggregate_id.as_str()))
}
