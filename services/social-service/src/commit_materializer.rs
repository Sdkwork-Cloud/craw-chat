//! Materialize social commit envelopes into supplemental PostgreSQL stores.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_adapters_social_postgres::direct_chat_store::DirectChatStore;
use im_adapters_social_postgres::friend_request_store::FriendRequestStore;
use im_adapters_social_postgres::friendship_store::FriendshipStore;
use im_adapters_social_postgres::user_block_store::{UserBlockRecord, UserBlockStore};
use im_adapters_social_postgres::wire_id::social_entity_id_to_i64;
use im_domain_events::social::{
    DirectChatBoundPayload, FriendRequestAcceptedPayload, FriendRequestCanceledPayload,
    FriendRequestDeclinedPayload, FriendRequestExpiredPayload, FriendRequestSubmittedPayload,
    FriendshipActivatedPayload, FriendshipRemovedPayload, UserBlockReleasedPayload,
    UserBlockedPayload,
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

    pub fn direct_chat_store(&self) -> Arc<dyn DirectChatStore> {
        self.direct_chat_store.clone()
    }

    /// Best-effort replay materialization after journal commit (bootstrap / drift repair).
    pub fn try_materialize_commits(&self, commits: &[CommitEnvelope]) -> usize {
        if commits.is_empty() {
            return 0;
        }
        if let Err(error) = self.materialize_commits(commits) {
            tracing::error!(
                error = %error,
                commit_count = commits.len(),
                "social postgres replay materialization failed"
            );
            return commits.len();
        }
        0
    }

    /// Materializes commits and returns the first error (write authority surfaces this before journal append).
    pub fn materialize_commits(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        if commits.is_empty() {
            return Ok(());
        }
        im_adapters_social_postgres::materialize_commits_in_transaction(&self.pool, commits)
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
                    .map_err(|error| {
                        format!("friend_request compensation delete failed: {error:?}")
                    })
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
                    serde_json::from_str(commit.payload.as_str())
                        .map_err(|error| format!("invalid friendship.removed payload: {error}"))?;
                self.friendship_store
                    .update_status(
                        commit.tenant_id.as_str(),
                        commit.organization_id.as_str(),
                        social_entity_id_to_i64(payload.friendship_id.as_str()),
                        "active",
                        payload.removed_at.as_str(),
                    )
                    .map_err(|error| {
                        format!("friendship removed compensation revert failed: {error:?}")
                    })
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
                    serde_json::from_str(commit.payload.as_str())
                        .map_err(|error| format!("invalid user_block.released payload: {error}"))?;
                let scope = payload.scope.unwrap_or_else(|| "all".to_owned());
                let record = UserBlockRecord {
                    tenant_id: commit.tenant_id.clone(),
                    organization_id: commit.organization_id.clone(),
                    block_id: social_entity_id_to_i64(payload.block_id.as_str()),
                    blocker_user_id: payload.blocker_user_id,
                    blocked_user_id: payload.blocked_user_id,
                    scope,
                    direct_chat_id: payload
                        .direct_chat_id
                        .as_deref()
                        .map(social_entity_id_to_i64),
                    reason: None,
                    expires_at: payload.expires_at,
                    created_at: payload
                        .effective_at
                        .unwrap_or_else(|| payload.released_at.clone()),
                    updated_at: payload.released_at,
                };
                self.user_block_store.insert(&record).map_err(|error| {
                    format!("user_block release compensation insert failed: {error:?}")
                })
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

    fn compensate_friend_request_terminal_status(
        &self,
        commit: &CommitEnvelope,
    ) -> Result<(), String> {
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
            .map_err(|error| {
                format!("friend_request terminal compensation revert failed: {error:?}")
            })
    }
}

fn extract_request_id_from_aggregate(commit: &CommitEnvelope) -> Result<i64, String> {
    Ok(social_entity_id_to_i64(commit.aggregate_id.as_str()))
}
