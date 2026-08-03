use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

use im_adapters_local_memory::MemoryCommitJournal;
use im_domain_core::social::{
    BlockScope, DirectChat, DirectChatStatus, ExternalConnection, ExternalConnectionKind,
    ExternalConnectionStatus, ExternalMemberLink, FriendRequest, FriendRequestStatus, Friendship,
    SharedChannelPolicy, UserBlock, UserBlockStatus, normalize_actor_pair, normalize_user_pair,
};
use im_domain_events::normalize_commit_organization_id;
use im_platform_contracts::{CommitEnvelope, CommitJournal, ContractError};
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};

use crate::SharedChannelLinkedMemberSyncRequest;

// ---------------------------------------------------------------------------
// SocialStateStore
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum SocialStateStore {
    Memory(Arc<Mutex<SocialControlState>>),
    Database {
        _pool: im_adapters_social_postgres::SocialPostgresPool,
    },
}

impl SocialStateStore {
    pub(crate) fn memory() -> Self {
        Self::Memory(Arc::new(Mutex::new(SocialControlState::default())))
    }

    pub fn database(pool: im_adapters_social_postgres::SocialPostgresPool) -> Self {
        Self::Database { _pool: pool }
    }

    pub(crate) fn load(&self) -> Result<SocialControlState, String> {
        match self {
            Self::Memory(state) => {
                let mut loaded =
                    lock_social_state_mutex(state, "social-state-store.memory").clone();
                loaded.rebuild_social_indexes();
                Ok(loaded)
            }
            Self::Database { _pool: _ } => {
                // Normalized PostgreSQL stores are queried by the service methods that need
                // authoritative state. This process-local state is only a bounded hot cache.
                Ok(SocialControlState::default())
            }
        }
    }

    pub(crate) fn save(&self, state: &SocialControlState) -> Result<(), String> {
        match self {
            Self::Memory(slot) => {
                *lock_social_state_mutex(slot, "social-state-store.memory") = state.clone();
                Ok(())
            }
            Self::Database { _pool: _ } => {
                // The normalized write authority has already committed the transaction.
                Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Index key types
// ---------------------------------------------------------------------------

pub(crate) fn organization_id_from_commits(commits: &[CommitEnvelope]) -> String {
    commits
        .last()
        .map(|commit| normalize_commit_organization_id(commit.organization_id.as_str()))
        .unwrap_or_else(|| normalize_commit_organization_id(""))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialPairIndexKey {
    tenant_id: String,
    organization_id: String,
    left_id: String,
    right_id: String,
}

impl SocialPairIndexKey {
    pub(crate) fn new(
        tenant_id: &str,
        organization_id: &str,
        left_id: &str,
        right_id: &str,
    ) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            organization_id: normalize_commit_organization_id(organization_id),
            left_id: left_id.to_owned(),
            right_id: right_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialUserIndexKey {
    tenant_id: String,
    organization_id: String,
    user_id: String,
}

impl SocialUserIndexKey {
    pub(crate) fn new(tenant_id: &str, organization_id: &str, user_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            organization_id: normalize_commit_organization_id(organization_id),
            user_id: user_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialUserBlockScopeIndexKey {
    tenant_id: String,
    blocker_user_id: String,
    blocked_user_id: String,
    scope: String,
    direct_chat_id: Option<String>,
}

impl SocialUserBlockScopeIndexKey {
    pub(crate) fn new(user_block: &UserBlock) -> Self {
        let direct_chat_id = if matches!(user_block.scope, BlockScope::DirectChat) {
            user_block.direct_chat_id.clone()
        } else {
            None
        };
        Self {
            tenant_id: user_block.tenant_id.clone(),
            blocker_user_id: user_block.blocker_user_id.clone(),
            blocked_user_id: user_block.blocked_user_id.clone(),
            scope: block_scope_index_label(&user_block.scope).to_owned(),
            direct_chat_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialDirectChatBlockIndexKey {
    tenant_id: String,
    direct_chat_id: String,
}

impl SocialDirectChatBlockIndexKey {
    pub(crate) fn new(tenant_id: &str, direct_chat_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            direct_chat_id: direct_chat_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialExternalConnectionTargetIndexKey {
    tenant_id: String,
    external_tenant_id: String,
    connection_kind: String,
}

impl SocialExternalConnectionTargetIndexKey {
    pub(crate) fn new(
        tenant_id: &str,
        external_tenant_id: &str,
        connection_kind: &ExternalConnectionKind,
    ) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            external_tenant_id: external_tenant_id.to_owned(),
            connection_kind: external_connection_kind_index_label(connection_kind).to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialExternalMemberMappingIndexKey {
    tenant_id: String,
    connection_id: String,
    external_member_id: String,
}

impl SocialExternalMemberMappingIndexKey {
    pub(crate) fn new(tenant_id: &str, connection_id: &str, external_member_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
            external_member_id: external_member_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialConnectionIndexKey {
    tenant_id: String,
    connection_id: String,
}

impl SocialConnectionIndexKey {
    pub(crate) fn new(tenant_id: &str, connection_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SharedChannelRetryIndexKey {
    last_failed_at: String,
}

impl SharedChannelRetryIndexKey {
    pub(crate) fn new(last_failed_at: &str) -> Self {
        Self {
            last_failed_at: last_failed_at.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SharedChannelLeaseIndexKey {
    lease_expires_at: String,
}

impl SharedChannelLeaseIndexKey {
    pub(crate) fn new(lease_expires_at: &str) -> Self {
        Self {
            lease_expires_at: lease_expires_at.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialSharedChannelPolicyTargetIndexKey {
    tenant_id: String,
    connection_id: String,
    channel_id: String,
}

impl SocialSharedChannelPolicyTargetIndexKey {
    pub(crate) fn new(tenant_id: &str, connection_id: &str, channel_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
            channel_id: channel_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SocialCommittedEventIndexKey {
    tenant_id: String,
    event_id: String,
}

impl SocialCommittedEventIndexKey {
    pub(crate) fn new(tenant_id: &str, event_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            event_id: event_id.to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Stored record types
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StoredFriendRequest {
    pub(crate) friend_request: FriendRequest,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StoredFriendship {
    pub(crate) friendship: Friendship,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StoredUserBlock {
    pub(crate) user_block: UserBlock,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredDirectChat {
    pub(crate) direct_chat: DirectChat,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredExternalConnection {
    pub(crate) external_connection: ExternalConnection,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StoredExternalMemberLink {
    pub(crate) external_member_link: ExternalMemberLink,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StoredSharedChannelPolicy {
    pub(crate) shared_channel_policy: SharedChannelPolicy,
    pub(crate) commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StoredSharedChannelSyncDeliveryProof {
    pub(crate) delivered_at: String,
    pub(crate) status: crate::SharedChannelSyncDeliveryProofStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) proof_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) target: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PendingSharedChannelSyncRequest {
    pub(crate) request: SharedChannelLinkedMemberSyncRequest,
    pub(crate) failure_count: u32,
    pub(crate) last_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) last_failed_at: Option<String>,
    pub(crate) owner_actor_id: Option<String>,
    pub(crate) owner_actor_kind: Option<String>,
    pub(crate) claimed_at: Option<String>,
    pub(crate) lease_expires_at: Option<String>,
}

// ---------------------------------------------------------------------------
// SocialCommittedEvent
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) enum SocialCommittedEvent {
    FriendRequest {
        record: StoredFriendRequest,
        commit: CommitEnvelope,
    },
    Friendship {
        record: StoredFriendship,
        commit: CommitEnvelope,
    },
    UserBlock {
        record: StoredUserBlock,
        commit: CommitEnvelope,
    },
    DirectChat {
        record: StoredDirectChat,
        commit: CommitEnvelope,
    },
    ExternalConnection {
        record: StoredExternalConnection,
        commit: CommitEnvelope,
    },
    ExternalMemberLink {
        record: StoredExternalMemberLink,
        commit: CommitEnvelope,
    },
    SharedChannelPolicy {
        record: StoredSharedChannelPolicy,
        commit: CommitEnvelope,
    },
}

impl SocialCommittedEvent {
    pub(crate) fn commit(&self) -> &CommitEnvelope {
        match self {
            Self::FriendRequest { commit, .. }
            | Self::Friendship { commit, .. }
            | Self::UserBlock { commit, .. }
            | Self::DirectChat { commit, .. }
            | Self::ExternalConnection { commit, .. }
            | Self::ExternalMemberLink { commit, .. }
            | Self::SharedChannelPolicy { commit, .. } => commit,
        }
    }

    pub(crate) fn aggregate_label(&self) -> &'static str {
        match self {
            Self::FriendRequest { .. } => "friend_request",
            Self::Friendship { .. } => "friendship",
            Self::UserBlock { .. } => "user_block",
            Self::DirectChat { .. } => "direct_chat",
            Self::ExternalConnection { .. } => "external_connection",
            Self::ExternalMemberLink { .. } => "external_member_link",
            Self::SharedChannelPolicy { .. } => "shared_channel_policy",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SocialCommittedEventPointer {
    FriendRequest {
        request_id: String,
        commit_index: usize,
    },
    Friendship {
        friendship_id: String,
        commit_index: usize,
    },
    UserBlock {
        block_id: String,
        commit_index: usize,
    },
    DirectChat {
        direct_chat_id: String,
        commit_index: usize,
    },
    ExternalConnection {
        connection_id: String,
        commit_index: usize,
    },
    ExternalMemberLink {
        link_id: String,
        commit_index: usize,
    },
    SharedChannelPolicy {
        policy_id: String,
        commit_index: usize,
    },
}

impl SocialCommittedEventPointer {
    fn with_commit_index(&self, commit_index: usize) -> Self {
        match self {
            Self::FriendRequest { request_id, .. } => Self::FriendRequest {
                request_id: request_id.clone(),
                commit_index,
            },
            Self::Friendship { friendship_id, .. } => Self::Friendship {
                friendship_id: friendship_id.clone(),
                commit_index,
            },
            Self::UserBlock { block_id, .. } => Self::UserBlock {
                block_id: block_id.clone(),
                commit_index,
            },
            Self::DirectChat { direct_chat_id, .. } => Self::DirectChat {
                direct_chat_id: direct_chat_id.clone(),
                commit_index,
            },
            Self::ExternalConnection { connection_id, .. } => Self::ExternalConnection {
                connection_id: connection_id.clone(),
                commit_index,
            },
            Self::ExternalMemberLink { link_id, .. } => Self::ExternalMemberLink {
                link_id: link_id.clone(),
                commit_index,
            },
            Self::SharedChannelPolicy { policy_id, .. } => Self::SharedChannelPolicy {
                policy_id: policy_id.clone(),
                commit_index,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// SocialControlState
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SocialControlState {
    pub(crate) friend_requests: BTreeMap<String, StoredFriendRequest>,
    pub(crate) friendships: BTreeMap<String, StoredFriendship>,
    pub(crate) user_blocks: BTreeMap<String, StoredUserBlock>,
    pub(crate) direct_chats: BTreeMap<String, StoredDirectChat>,
    pub(crate) external_connections: BTreeMap<String, StoredExternalConnection>,
    pub(crate) external_member_links: BTreeMap<String, StoredExternalMemberLink>,
    pub(crate) shared_channel_policies: BTreeMap<String, StoredSharedChannelPolicy>,
    pub(crate) pending_shared_channel_sync_requests:
        BTreeMap<String, PendingSharedChannelSyncRequest>,
    pub(crate) dead_letter_shared_channel_sync_requests:
        BTreeMap<String, PendingSharedChannelSyncRequest>,
    pub(crate) delivered_shared_channel_sync_requests: BTreeMap<String, String>,
    pub(crate) delivered_shared_channel_sync_delivery_proofs:
        BTreeMap<String, StoredSharedChannelSyncDeliveryProof>,
    pub(crate) recent_shared_channel_sync_deliveries: BTreeMap<String, String>,
    #[serde(skip)]
    pub(crate) pending_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) accepted_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) friend_request_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) active_friendship_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_friendship_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) friendship_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) active_direct_chat_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    pub(crate) direct_chat_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) active_user_block_scope_index: BTreeMap<SocialUserBlockScopeIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_friendship_block_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_direct_chat_block_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_direct_chat_block_chat_index: BTreeMap<SocialDirectChatBlockIndexKey, String>,
    #[serde(skip)]
    pub(crate) committed_event_index:
        BTreeMap<SocialCommittedEventIndexKey, SocialCommittedEventPointer>,
    #[serde(skip)]
    pub(crate) active_external_connection_target_index:
        BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_external_member_mapping_index:
        BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_external_member_connection_index:
        BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) active_shared_channel_policy_target_index:
        BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    #[serde(skip)]
    pub(crate) active_shared_channel_policy_connection_index:
        BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) pending_shared_channel_retry_index:
        BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pub(crate) pending_shared_channel_lease_index:
        BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    /// Commit envelopes for friend requests that have been evicted from
    /// `friend_requests` after reaching a terminal state. Kept lightweight
    /// (envelopes only, no full aggregate state) so that idempotency checks
    /// via `committed_event()` still work after eviction. The PostgreSQL
    /// supplemental store remains the source of truth for the full record.
    #[serde(skip)]
    pub(crate) evicted_friend_request_commits: BTreeMap<String, Vec<CommitEnvelope>>,
}

impl SocialControlState {
    pub(crate) fn rebuild_social_indexes(&mut self) {
        self.rebuild_social_friend_request_indexes();
        self.rebuild_social_pair_indexes();
        self.rebuild_social_user_block_indexes();
        self.rebuild_social_external_collaboration_indexes();
        self.rebuild_shared_channel_pending_indexes();
        self.rebuild_social_committed_event_index();
    }

    fn rebuild_social_friend_request_indexes(&mut self) {
        self.pending_friend_request_pair_index.clear();
        self.accepted_friend_request_pair_index.clear();
        self.friend_request_user_index.clear();
        for record in self.friend_requests.values() {
            index_friend_request_record(
                &mut self.pending_friend_request_pair_index,
                &mut self.accepted_friend_request_pair_index,
                &mut self.friend_request_user_index,
                record,
            );
        }
    }

    fn rebuild_social_pair_indexes(&mut self) {
        self.active_friendship_pair_index.clear();
        self.active_friendship_user_index.clear();
        self.friendship_pair_index.clear();
        self.active_direct_chat_pair_index.clear();
        self.direct_chat_pair_index.clear();
        for record in self.friendships.values() {
            index_friendship_record(
                &mut self.active_friendship_pair_index,
                &mut self.active_friendship_user_index,
                &mut self.friendship_pair_index,
                record,
            );
        }
        for record in self.direct_chats.values() {
            index_direct_chat_record(
                &mut self.active_direct_chat_pair_index,
                &mut self.direct_chat_pair_index,
                record,
            );
        }
    }

    fn rebuild_social_user_block_indexes(&mut self) {
        self.active_user_block_scope_index.clear();
        self.active_friendship_block_pair_index.clear();
        self.active_direct_chat_block_pair_index.clear();
        self.active_direct_chat_block_chat_index.clear();
        for record in self.user_blocks.values() {
            index_user_block_record(
                &mut self.active_user_block_scope_index,
                &mut self.active_friendship_block_pair_index,
                &mut self.active_direct_chat_block_pair_index,
                &mut self.active_direct_chat_block_chat_index,
                record,
            );
        }
    }

    fn rebuild_social_external_collaboration_indexes(&mut self) {
        self.active_external_connection_target_index.clear();
        self.active_external_member_mapping_index.clear();
        self.active_external_member_connection_index.clear();
        self.active_shared_channel_policy_target_index.clear();
        self.active_shared_channel_policy_connection_index.clear();
        for record in self.external_connections.values() {
            index_external_connection_record(
                &mut self.active_external_connection_target_index,
                record,
            );
        }
        for record in self.external_member_links.values() {
            index_external_member_link_record(
                &mut self.active_external_member_mapping_index,
                &mut self.active_external_member_connection_index,
                record,
            );
        }
        for record in self.shared_channel_policies.values() {
            index_shared_channel_policy_record(
                &mut self.active_shared_channel_policy_target_index,
                &mut self.active_shared_channel_policy_connection_index,
                record,
            );
        }
    }

    fn rebuild_shared_channel_pending_indexes(&mut self) {
        self.pending_shared_channel_retry_index.clear();
        self.pending_shared_channel_lease_index.clear();
        for (request_key, pending) in &self.pending_shared_channel_sync_requests {
            index_pending_shared_channel_sync_request(
                &mut self.pending_shared_channel_retry_index,
                &mut self.pending_shared_channel_lease_index,
                request_key.as_str(),
                pending,
            );
        }
    }

    fn rebuild_social_committed_event_index(&mut self) {
        self.committed_event_index.clear();
        for record in self.friend_requests.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::FriendRequest {
                    request_id: record.friend_request.request_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.friendships.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::Friendship {
                    friendship_id: record.friendship.friendship_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.user_blocks.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::UserBlock {
                    block_id: record.user_block.block_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.direct_chats.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::DirectChat {
                    direct_chat_id: record.direct_chat.direct_chat_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.external_connections.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::ExternalConnection {
                    connection_id: record.external_connection.connection_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.external_member_links.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::ExternalMemberLink {
                    link_id: record.external_member_link.link_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.shared_channel_policies.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::SharedChannelPolicy {
                    policy_id: record.shared_channel_policy.policy_id.clone(),
                    commit_index: 0,
                },
            );
        }
    }

    pub(crate) fn committed_event(
        &self,
        tenant_id: &str,
        event_id: &str,
    ) -> Option<SocialCommittedEvent> {
        let pointer = self
            .committed_event_index
            .get(&SocialCommittedEventIndexKey::new(tenant_id, event_id))?;
        match pointer {
            SocialCommittedEventPointer::FriendRequest {
                request_id,
                commit_index,
            } => {
                if let Some(record) = self.friend_requests.get(request_id) {
                    let commit = record.commits.get(*commit_index)?.clone();
                    if commit.tenant_id != tenant_id || commit.event_id != event_id {
                        return None;
                    }
                    return Some(SocialCommittedEvent::FriendRequest {
                        record: record.clone(),
                        commit,
                    });
                }
                // Record was evicted from memory after reaching a terminal
                // state. Reconstruct the commit from the retained envelopes.
                let commits = self.evicted_friend_request_commits.get(request_id)?;
                let commit = commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                let record = reconstruct_evicted_friend_request(request_id, commits)?;
                Some(SocialCommittedEvent::FriendRequest { record, commit })
            }
            SocialCommittedEventPointer::Friendship {
                friendship_id,
                commit_index,
            } => {
                let record = self.friendships.get(friendship_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::Friendship { record, commit })
            }
            SocialCommittedEventPointer::UserBlock {
                block_id,
                commit_index,
            } => {
                let record = self.user_blocks.get(block_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::UserBlock { record, commit })
            }
            SocialCommittedEventPointer::DirectChat {
                direct_chat_id,
                commit_index,
            } => {
                let record = self.direct_chats.get(direct_chat_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::DirectChat { record, commit })
            }
            SocialCommittedEventPointer::ExternalConnection {
                connection_id,
                commit_index,
            } => {
                let record = self.external_connections.get(connection_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::ExternalConnection { record, commit })
            }
            SocialCommittedEventPointer::ExternalMemberLink {
                link_id,
                commit_index,
            } => {
                let record = self.external_member_links.get(link_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::ExternalMemberLink { record, commit })
            }
            SocialCommittedEventPointer::SharedChannelPolicy {
                policy_id,
                commit_index,
            } => {
                let record = self.shared_channel_policies.get(policy_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::SharedChannelPolicy { record, commit })
            }
        }
    }

    // Record insert/unindex helpers

    pub(crate) fn insert_friend_request_record(
        &mut self,
        request_id: String,
        record: StoredFriendRequest,
    ) {
        if let Some(previous) = self.friend_requests.insert(request_id, record.clone()) {
            unindex_friend_request_record(
                &mut self.pending_friend_request_pair_index,
                &mut self.accepted_friend_request_pair_index,
                &mut self.friend_request_user_index,
                &previous,
            );
        }
        index_friend_request_record(
            &mut self.pending_friend_request_pair_index,
            &mut self.accepted_friend_request_pair_index,
            &mut self.friend_request_user_index,
            &record,
        );
        self.index_friend_request_commits(
            record.friend_request.request_id.as_str(),
            record.commits.as_slice(),
        );
    }

    /// Remove a friend request record from in-memory state.
    ///
    /// Friend requests are high-volume, short-lived aggregates. Once a request
    /// reaches a terminal state (accepted/declined/canceled/expired) the
    /// PostgreSQL supplemental store is the source of truth. Evicting the
    /// record from memory prevents unbounded growth (OOM) as the user base
    /// scales. The journal remains the write authority 鈥?replay rebuilds the
    /// record if needed, and the PG store fallback covers lookups.
    pub(crate) fn evict_friend_request_record(&mut self, request_id: &str) {
        if let Some(record) = self.friend_requests.remove(request_id) {
            unindex_friend_request_record(
                &mut self.pending_friend_request_pair_index,
                &mut self.accepted_friend_request_pair_index,
                &mut self.friend_request_user_index,
                &record,
            );
            // Retain commit envelopes so idempotency checks via
            // committed_event() still work after eviction. Only the
            // envelopes are kept (not the full aggregate state) to keep
            // memory usage bounded.
            self.evicted_friend_request_commits
                .insert(request_id.to_owned(), record.commits.clone());
        }
    }

    pub(crate) fn insert_friendship_record(
        &mut self,
        friendship_id: String,
        record: StoredFriendship,
    ) {
        if let Some(previous) = self.friendships.insert(friendship_id, record.clone()) {
            unindex_friendship_record(
                &mut self.active_friendship_pair_index,
                &mut self.active_friendship_user_index,
                &mut self.friendship_pair_index,
                &previous,
            );
        }
        index_friendship_record(
            &mut self.active_friendship_pair_index,
            &mut self.active_friendship_user_index,
            &mut self.friendship_pair_index,
            &record,
        );
        self.index_friendship_commits(
            record.friendship.friendship_id.as_str(),
            record.commits.as_slice(),
        );
    }

    pub(crate) fn insert_user_block_record(&mut self, block_id: String, record: StoredUserBlock) {
        if let Some(previous) = self.user_blocks.insert(block_id, record.clone()) {
            unindex_user_block_record(
                &mut self.active_user_block_scope_index,
                &mut self.active_friendship_block_pair_index,
                &mut self.active_direct_chat_block_pair_index,
                &mut self.active_direct_chat_block_chat_index,
                &previous,
            );
        }
        index_user_block_record(
            &mut self.active_user_block_scope_index,
            &mut self.active_friendship_block_pair_index,
            &mut self.active_direct_chat_block_pair_index,
            &mut self.active_direct_chat_block_chat_index,
            &record,
        );
        self.index_user_block_commits(
            record.user_block.block_id.as_str(),
            record.commits.as_slice(),
        );
    }

    pub(crate) fn insert_direct_chat_record(
        &mut self,
        direct_chat_id: String,
        record: StoredDirectChat,
    ) {
        if let Some(previous) = self.direct_chats.insert(direct_chat_id, record.clone()) {
            unindex_direct_chat_record(
                &mut self.active_direct_chat_pair_index,
                &mut self.direct_chat_pair_index,
                &previous,
            );
        }
        index_direct_chat_record(
            &mut self.active_direct_chat_pair_index,
            &mut self.direct_chat_pair_index,
            &record,
        );
        self.index_direct_chat_commits(
            record.direct_chat.direct_chat_id.as_str(),
            record.commits.as_slice(),
        );
    }

    pub(crate) fn insert_external_connection_record(
        &mut self,
        connection_id: String,
        record: StoredExternalConnection,
    ) {
        if let Some(previous) = self
            .external_connections
            .insert(connection_id, record.clone())
        {
            unindex_external_connection_record(
                &mut self.active_external_connection_target_index,
                &previous,
            );
        }
        index_external_connection_record(
            &mut self.active_external_connection_target_index,
            &record,
        );
        self.index_external_connection_commits(
            record.external_connection.connection_id.as_str(),
            record.commits.as_slice(),
        );
    }

    pub(crate) fn insert_external_member_link_record(
        &mut self,
        link_id: String,
        record: StoredExternalMemberLink,
    ) {
        if let Some(previous) = self.external_member_links.insert(link_id, record.clone()) {
            unindex_external_member_link_record(
                &mut self.active_external_member_mapping_index,
                &mut self.active_external_member_connection_index,
                &previous,
            );
        }
        index_external_member_link_record(
            &mut self.active_external_member_mapping_index,
            &mut self.active_external_member_connection_index,
            &record,
        );
        self.index_external_member_link_commits(
            record.external_member_link.link_id.as_str(),
            record.commits.as_slice(),
        );
    }

    pub(crate) fn insert_shared_channel_policy_record(
        &mut self,
        policy_id: String,
        record: StoredSharedChannelPolicy,
    ) {
        if let Some(previous) = self
            .shared_channel_policies
            .insert(policy_id, record.clone())
        {
            unindex_shared_channel_policy_record(
                &mut self.active_shared_channel_policy_target_index,
                &mut self.active_shared_channel_policy_connection_index,
                &previous,
            );
        }
        index_shared_channel_policy_record(
            &mut self.active_shared_channel_policy_target_index,
            &mut self.active_shared_channel_policy_connection_index,
            &record,
        );
        self.index_shared_channel_policy_commits(
            record.shared_channel_policy.policy_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn index_friend_request_commits(&mut self, request_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::FriendRequest {
                request_id: request_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_friendship_commits(&mut self, friendship_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::Friendship {
                friendship_id: friendship_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_user_block_commits(&mut self, block_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::UserBlock {
                block_id: block_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_direct_chat_commits(&mut self, direct_chat_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::DirectChat {
                direct_chat_id: direct_chat_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_external_connection_commits(
        &mut self,
        connection_id: &str,
        commits: &[CommitEnvelope],
    ) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::ExternalConnection {
                connection_id: connection_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_external_member_link_commits(&mut self, link_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::ExternalMemberLink {
                link_id: link_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_shared_channel_policy_commits(&mut self, policy_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::SharedChannelPolicy {
                policy_id: policy_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    pub(crate) fn upsert_pending_shared_channel_sync_request(
        &mut self,
        request_key: String,
        pending: PendingSharedChannelSyncRequest,
    ) {
        let old_pending = self
            .pending_shared_channel_sync_requests
            .insert(request_key.clone(), pending.clone());
        if let Some(old) = old_pending {
            unindex_pending_shared_channel_sync_request(
                &mut self.pending_shared_channel_retry_index,
                &mut self.pending_shared_channel_lease_index,
                request_key.as_str(),
                &old,
            );
        }
        index_pending_shared_channel_sync_request(
            &mut self.pending_shared_channel_retry_index,
            &mut self.pending_shared_channel_lease_index,
            request_key.as_str(),
            &pending,
        );
    }

    #[allow(dead_code)]
    pub(crate) fn record_failed_shared_channel_sync_requests(
        &mut self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
        error: &str,
        now: &str,
    ) -> bool {
        self.record_failed_shared_channel_sync_requests_with_owner_preservation(
            requests, error, now, false,
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialDerivedSnapshotStatus {
    Current,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialWritePersistence {
    pub(crate) journal_authority: bool,
    pub(crate) snapshot_status: SocialDerivedSnapshotStatus,
}

// ---------------------------------------------------------------------------
// SocialRuntime
// ---------------------------------------------------------------------------

pub struct SocialRuntime {
    state_store: SocialStateStore,
    commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    pub(crate) state: RwLock<SocialControlState>,
    postgres_store: Option<Arc<crate::normalized_store::SocialPostgresNormalizedStore>>,
    postgres_atomic_write_authority:
        Option<Arc<dyn crate::postgres_write_authority::SocialAtomicWriteAuthority>>,
    outbox_store: Option<Arc<dyn im_platform_contracts::OutboxStore>>,
    id_generator: Option<Arc<dyn im_platform_contracts::IdGenerator>>,
    realtime_fanout: RwLock<Option<Arc<dyn crate::social_realtime::SocialRealtimeFanout>>>,
    direct_chat_binder:
        RwLock<Option<Arc<dyn crate::direct_chat_binder::DirectChatConversationBinder>>>,
    shared_channel_sync_trigger:
        RwLock<Option<Arc<dyn crate::SharedChannelLinkedMemberSyncTrigger>>>,
    #[allow(dead_code)]
    pub(crate) shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool,
    pub(crate) friend_request_expiration_scheduler_started: AtomicBool,
    user_directory: Option<Arc<dyn crate::user_directory::SocialUserDirectory>>,
}

impl SocialRuntime {
    pub(crate) fn new(
        state_store: SocialStateStore,
        commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    ) -> Self {
        let state = state_store
            .load()
            .expect("SocialStateStore memory/database initialization cannot fail");
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(state),
            postgres_store: None,
            postgres_atomic_write_authority: None,
            outbox_store: None,
            id_generator: None,
            realtime_fanout: RwLock::new(None),
            direct_chat_binder: RwLock::new(None),
            shared_channel_sync_trigger: RwLock::new(None),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
            friend_request_expiration_scheduler_started: AtomicBool::new(false),
            user_directory: None,
        }
    }

    #[doc(hidden)]
    pub fn for_test() -> Self {
        Self::new(
            SocialStateStore::memory(),
            Arc::new(MemoryCommitJournal::default()),
        )
    }

    pub fn with_postgres_write_authority(
        mut self,
        journal: im_adapters_postgres_journal::PostgresCommitJournal,
        pool: im_adapters_social_postgres::SocialPostgresPool,
    ) -> Self {
        let normalized_store =
            Arc::new(crate::normalized_store::SocialPostgresNormalizedStore::from_pool(pool));
        self.postgres_atomic_write_authority = Some(Arc::new(
            crate::postgres_write_authority::SocialPostgresAtomicWriteAuthority::new(
                journal,
                normalized_store.clone(),
            ),
        ));
        self.postgres_store = Some(normalized_store);
        self
    }

    pub fn with_outbox_store(mut self, store: Arc<dyn im_platform_contracts::OutboxStore>) -> Self {
        self.outbox_store = Some(store);
        self
    }

    pub fn with_id_generator(
        mut self,
        id_generator: Arc<dyn im_platform_contracts::IdGenerator>,
    ) -> Self {
        self.id_generator = Some(id_generator);
        self
    }

    pub(crate) fn friend_request_rate_limit_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::friend_request_store::FriendRequestStore>>
    {
        self.postgres_store
            .as_ref()
            .map(|store| store.friend_request_store())
    }

    pub(crate) fn friendship_inventory_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::friendship_store::FriendshipStore>> {
        self.postgres_store
            .as_ref()
            .map(|store| store.friendship_store())
    }

    pub(crate) fn direct_chat_inventory_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::direct_chat_store::DirectChatStore>> {
        self.postgres_store
            .as_ref()
            .map(|store| store.direct_chat_store())
    }

    pub(crate) fn user_block_authority_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::user_block_store::UserBlockStore>> {
        self.postgres_store
            .as_ref()
            .map(|store| store.user_block_store())
    }

    pub(crate) fn external_connection_authority_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::external_store::ExternalConnectionStore>> {
        self.postgres_store
            .as_ref()
            .map(|store| store.external_connection_store())
    }

    pub(crate) fn external_member_link_authority_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::external_store::ExternalMemberLinkStore>> {
        self.postgres_store
            .as_ref()
            .map(|store| store.external_member_link_store())
    }

    pub(crate) fn shared_channel_policy_authority_store(
        &self,
    ) -> Option<Arc<dyn im_adapters_social_postgres::shared_channel_store::SharedChannelPolicyStore>>
    {
        self.postgres_store
            .as_ref()
            .map(|store| store.shared_channel_policy_store())
    }

    pub fn set_realtime_fanout(
        &self,
        fanout: Arc<dyn crate::social_realtime::SocialRealtimeFanout>,
    ) {
        *self
            .realtime_fanout
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = Some(fanout);
    }

    pub fn set_direct_chat_conversation_binder(
        &self,
        binder: Arc<dyn crate::direct_chat_binder::DirectChatConversationBinder>,
    ) {
        *self
            .direct_chat_binder
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = Some(binder);
    }

    pub(crate) fn bind_direct_chat_conversation_if_configured(
        &self,
        input: crate::direct_chat_binder::BindDirectChatConversationInput,
    ) -> Result<(), String> {
        let binder = self
            .direct_chat_binder
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        let Some(binder) = binder else {
            return Ok(());
        };
        binder.bind_direct_chat_conversation(input)
    }

    /// Generate the next numeric snowflake record id for social entities
    /// persisted in bigint-keyed tables (friendships, direct chats).
    pub(crate) fn next_social_record_id(
        &self,
        label: &str,
    ) -> Result<String, crate::friendship::SocialServiceError> {
        self.id_generator
            .as_ref()
            .ok_or_else(|| {
                crate::friendship::SocialServiceError::dependency_unavailable(
                    "id_generator_unavailable",
                    format!(
                        "social {label} record id generation requires a configured id generator"
                    ),
                )
            })?
            .next_id()
            .map(|value| value.to_string())
            .map_err(|error| {
                crate::friendship::SocialServiceError::invalid(
                    "id_generation_failed",
                    format!("social {label} id generation failed: {error:?}"),
                )
            })
    }

    fn resolve_social_realtime_delivery(&self) -> (bool, bool) {
        let has_fanout = self
            .realtime_fanout
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .is_some();
        let has_outbox = self.outbox_store.is_some() && self.id_generator.is_some();
        (has_fanout, has_outbox)
    }

    fn ensure_social_realtime_delivery(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        let (has_fanout, has_outbox) = self.resolve_social_realtime_delivery();
        crate::social_realtime::ensure_realtime_delivery_configured(has_fanout, has_outbox, commits)
    }

    fn persist_commits_to_authority(
        &self,
        commits: &[CommitEnvelope],
    ) -> Result<(Vec<CommitEnvelope>, bool), String> {
        if let Some(authority) = self.postgres_atomic_write_authority.as_ref() {
            let inserted = authority
                .append_and_write(commits.to_vec())
                .map_err(|error| {
                    crate::social_write_metrics::record_postgres_atomic_write_failures(
                        commits.len() as u64,
                    );
                    tracing::error!(
                        error = ?error,
                        commit_count = commits.len(),
                        "atomic social postgres write rolled back"
                    );
                    format!(
                        "atomic social postgres write failed: {}",
                        contract_error_message(error)
                    )
                })?;
            crate::conversation_state_bridge::try_apply_social_commits_to_conversation_state(
                &inserted,
            );
            return Ok((inserted, true));
        }

        if self.postgres_store.is_some() {
            crate::social_write_metrics::record_postgres_atomic_write_failures(commits.len() as u64);
            tracing::error!(
                commit_count = commits.len(),
                "social postgres normalized store has no coordinated journal authority"
            );
            return Err(
                "social postgres writes require a coordinated postgres journal authority".into(),
            );
        }

        let append_result = if commits.len() == 1 {
            self.commit_journal.append(commits[0].clone()).map(|_| ())
        } else {
            self.commit_journal
                .append_batch(commits.to_vec())
                .map(|_| ())
        };
        if let Err(error) = append_result {
            return Err(format!(
                "failed to append social commit journal before state write: {}",
                contract_error_message(error)
            ));
        }
        Ok((commits.to_vec(), false))
    }

    fn finalize_persisted_commits(&self, commits: &[CommitEnvelope]) {
        if commits.is_empty() {
            return;
        }
        let fanout = self
            .realtime_fanout
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        if let Some(fanout) = fanout.as_ref() {
            crate::social_realtime::try_publish_social_commits(Some(fanout.as_ref()), commits);
        } else if let (Some(outbox_store), Some(id_generator)) =
            (self.outbox_store.as_ref(), self.id_generator.as_ref())
        {
            for commit in commits {
                match crate::social_realtime::build_social_realtime_outbox_record(
                    commit,
                    id_generator.as_ref(),
                ) {
                    Ok(Some(record)) => {
                        if let Err(error) = outbox_store.enqueue(record) {
                            tracing::warn!(
                                event_id = commit.event_id.as_str(),
                                event_type = commit.event_type.as_str(),
                                error = ?error,
                                "social outbox enqueue failed"
                            );
                        }
                    }
                    Ok(None) => {}
                    Err(error) => {
                        tracing::warn!(
                            event_id = commit.event_id.as_str(),
                            event_type = commit.event_type.as_str(),
                            error = %error,
                            "social outbox record build failed"
                        );
                    }
                }
            }
        }
    }

    pub(crate) fn with_user_directory(
        mut self,
        user_directory: Arc<dyn crate::user_directory::SocialUserDirectory>,
    ) -> Self {
        self.user_directory = Some(user_directory);
        self
    }

    pub(crate) fn validate_friend_request_target(
        &self,
        tenant_id: &str,
        organization_id: &str,
        target_user_id: &str,
    ) -> Result<(), crate::friendship::SocialServiceError> {
        if let Some(directory) = self.user_directory.as_ref() {
            return directory.validate_friend_request_target(
                tenant_id,
                organization_id,
                target_user_id,
            );
        }
        if crate::friend_request_rate_limit::is_production_like_environment() {
            return Err(
                crate::friendship::SocialServiceError::dependency_unavailable(
                    "social_user_directory_unconfigured",
                    "social user directory is not configured",
                ),
            );
        }
        Ok(())
    }

    pub(crate) fn shared_channel_linked_member_sync_trigger(
        &self,
    ) -> Option<Arc<dyn crate::SharedChannelLinkedMemberSyncTrigger>> {
        self.shared_channel_sync_trigger
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone()
    }

    pub fn set_shared_channel_linked_member_sync_trigger(
        &self,
        trigger: Arc<dyn crate::SharedChannelLinkedMemberSyncTrigger>,
    ) {
        *self
            .shared_channel_sync_trigger
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = Some(trigger);
    }

    pub fn dispatch_shared_channel_sync_requests(
        &self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
    ) -> Result<(), String> {
        if requests.is_empty() {
            return Ok(());
        }

        let now = utc_now_rfc3339_millis();
        {
            let mut state = self
                .state
                .write()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
            state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
            drop(state);
            self.persist_state_snapshot("shared-channel sync proactive stale reclaim")?;
        }

        let trigger = self
            .shared_channel_sync_trigger
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        let Some(trigger) = trigger else {
            let mut state = self
                .state
                .write()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
            for request in requests {
                if state.is_shared_channel_sync_delivered(request) {
                    continue;
                }
                state.ensure_pending_shared_channel_sync_request(
                    request.clone(),
                    "shared-channel sync trigger is not configured",
                    now.as_str(),
                );
            }
            drop(state);
            return self.persist_state_snapshot("shared-channel sync trigger backlog");
        };

        for request in requests {
            if self
                .state
                .read()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                .is_shared_channel_sync_delivered(request)
            {
                continue;
            }
            match trigger.trigger(request.clone()) {
                Ok(()) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.mark_shared_channel_sync_delivered(
                        request,
                        crate::SharedChannelSyncDeliveryProofStatus::Applied,
                        now.as_str(),
                        None,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync delivery")?;
                }
                Err(error) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.record_failed_shared_channel_sync_requests_with_owner_preservation(
                        std::slice::from_ref(request),
                        error.as_str(),
                        now.as_str(),
                        false,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync failure")?;
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn persist_state_snapshot(&self, context: &str) -> Result<(), String> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        self.state_store
            .save(&state)
            .map_err(|error| format!("{context}: {error}"))
    }

    pub(crate) fn recover_poisoned_social_runtime_lock<T>(
        poisoned: std::sync::PoisonError<T>,
    ) -> T {
        tracing::warn!(
            "social runtime lock was poisoned by a prior panic; continuing with inner state"
        );
        poisoned.into_inner()
    }

    // -----------------------------------------------------------------------
    // Query methods
    // -----------------------------------------------------------------------

    pub fn direct_chat_snapshot(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<StoredDirectChat> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .direct_chats
            .get(direct_chat_id)
            .filter(|record| record.direct_chat.tenant_id == tenant_id)
            .cloned()
    }

    pub fn active_direct_chat_access_block(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<UserBlock> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        active_direct_chat_scoped_user_block(&state, tenant_id, direct_chat_id)
    }

    pub fn active_friendship_access_block_for_pair(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_a: &str,
        user_b: &str,
    ) -> Option<UserBlock> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        active_friendship_scoped_user_block(&state, tenant_id, organization_id, user_a, user_b)
    }

    /// Returns an active user block that prevents direct messaging between two users.
    pub fn active_direct_message_block_for_pair(
        &self,
        tenant_id: &str,
        organization_id: &str,
        sender_user_id: &str,
        peer_user_id: &str,
    ) -> Result<Option<UserBlock>, String> {
        if let Some(store) = self.user_block_authority_store() {
            for (blocker_user_id, blocked_user_id) in [
                (sender_user_id, peer_user_id),
                (peer_user_id, sender_user_id),
            ] {
                for scope in ["all", "direct_chat"] {
                    let record = store
                        .find_active_block(
                            tenant_id,
                            organization_id,
                            blocker_user_id,
                            blocked_user_id,
                            scope,
                        )
                        .map_err(|error| {
                            format!("normalized user-block lookup failed: {error:?}")
                        })?;
                    if let Some(record) = record {
                        return user_block_from_authority_record(record).map(Some);
                    }
                }
            }
            return Ok(None);
        }
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        Ok(active_direct_message_block_for_pair(
            &state,
            tenant_id,
            organization_id,
            sender_user_id,
            peer_user_id,
        ))
    }

    /// Refreshes social authority and rejects direct messaging when an active block exists.
    pub fn ensure_direct_message_allowed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        sender_user_id: &str,
        peer_user_id: &str,
    ) -> Result<(), String> {
        let _read_lock = self.acquire_cross_instance_read_lock()?;
        self.refresh_state_from_authority_for_read()?;
        if let Some(user_block) = self.active_direct_message_block_for_pair(
            tenant_id,
            organization_id,
            sender_user_id,
            peer_user_id,
        )? {
            return Err(format!(
                "direct message blocked by user block {}",
                user_block.block_id
            ));
        }
        Ok(())
    }

    pub(crate) fn user_block_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        block_id: &str,
    ) -> Result<Option<StoredUserBlock>, String> {
        if let Some(store) = self.user_block_authority_store() {
            return store
                .get_by_id(
                    tenant_id,
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(block_id)
                        .map_err(|error| format!("invalid user-block id: {error:?}"))?,
                )
                .map_err(|error| format!("normalized user-block lookup failed: {error:?}"))?
                .map(user_block_from_authority_record)
                .transpose()
                .map(|record| {
                    record.map(|mut user_block| {
                        user_block.block_id = block_id.to_owned();
                        StoredUserBlock {
                            user_block,
                            commits: Vec::new(),
                        }
                    })
                });
        }

        Ok(self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .user_blocks
            .get(block_id)
            .filter(|record| record.user_block.tenant_id == tenant_id)
            .cloned())
    }

    pub fn authoritative_active_friendships_for_user(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Result<Vec<Friendship>, String> {
        let state = self.cached_state_for_query();
        let mut friendships =
            active_friendship_records_for_user(&state, tenant_id, organization_id, user_id)
                .into_iter()
                .map(|record| record.friendship)
                .collect::<Vec<_>>();
        friendships.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| left.friendship_id.cmp(&right.friendship_id))
        });
        Ok(friendships)
    }

    pub fn authoritative_active_direct_chat_for_pair(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_low_id: &str,
        user_high_id: &str,
    ) -> Result<Option<DirectChat>, String> {
        let state = self.cached_state_for_query();
        Ok(active_direct_chat_record_for_pair(
            &state,
            tenant_id,
            organization_id,
            user_low_id,
            user_high_id,
        )
        .map(|record| record.direct_chat))
    }

    pub fn external_connection_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        connection_id: &str,
    ) -> Result<Option<StoredExternalConnection>, String> {
        if let Some(store) = self.external_connection_authority_store() {
            return store
                .get_by_id(
                    tenant_id,
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(connection_id)
                        .map_err(|error| format!("invalid external-connection id: {error:?}"))?,
                )
                .map_err(|error| {
                    format!("normalized external-connection lookup failed: {error:?}")
                })?
                .map(external_connection_from_authority_record)
                .transpose()
                .map(|record| {
                    record.map(|mut external_connection| {
                        external_connection.connection_id = connection_id.to_owned();
                        StoredExternalConnection {
                            external_connection,
                            commits: Vec::new(),
                        }
                    })
                });
        }

        Ok(self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .external_connections
            .get(connection_id)
            .filter(|record| record.external_connection.tenant_id == tenant_id)
            .cloned())
    }

    fn cached_state_for_query(&self) -> SocialControlState {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone()
    }

    // -----------------------------------------------------------------------
    // State persistence
    // -----------------------------------------------------------------------

    fn persistence_with_snapshot_status(
        &self,
        snapshot_status: SocialDerivedSnapshotStatus,
    ) -> SocialWritePersistence {
        SocialWritePersistence {
            journal_authority: false,
            snapshot_status,
        }
    }

    pub(crate) fn current_persistence(&self) -> SocialWritePersistence {
        self.persistence_with_snapshot_status(SocialDerivedSnapshotStatus::Current)
    }

    pub(crate) fn persist_state_transition(
        &self,
        next: &SocialControlState,
        commit: &CommitEnvelope,
    ) -> Result<SocialWritePersistence, String> {
        self.ensure_social_realtime_delivery(std::slice::from_ref(commit))?;
        let (inserted_commits, _postgres_written) =
            self.persist_commits_to_authority(std::slice::from_ref(commit))?;
        self.finalize_persisted_commits(&inserted_commits);
        self.state_store.save(next)?;
        Ok(self.current_persistence())
    }

    pub(crate) fn persist_state_transition_batch(
        &self,
        next: &SocialControlState,
        commits: &[CommitEnvelope],
    ) -> Result<SocialWritePersistence, String> {
        if commits.is_empty() {
            return Ok(self.current_persistence());
        }
        self.ensure_social_realtime_delivery(commits)?;
        let (inserted_commits, _postgres_written) = self.persist_commits_to_authority(commits)?;
        self.finalize_persisted_commits(&inserted_commits);
        self.state_store.save(next)?;
        Ok(self.current_persistence())
    }

    pub(crate) fn acquire_cross_instance_write_lock(&self) -> Result<(), String> {
        self.ensure_social_authority_available()
    }

    pub(crate) fn acquire_cross_instance_read_lock(&self) -> Result<(), String> {
        self.ensure_social_authority_available()
    }

    // -----------------------------------------------------------------------
    // State refresh from authority
    // -----------------------------------------------------------------------

    pub(crate) fn refresh_state_from_authority_for_write(&self) -> Result<(), String> {
        self.ensure_social_authority_available()
    }

    /// Validate that normalized PostgreSQL authority is installed for production reads.
    pub(crate) fn refresh_state_from_authority_for_read(&self) -> Result<(), String> {
        self.ensure_social_authority_available()
    }

    pub(crate) fn ensure_social_authority_available(&self) -> Result<(), String> {
        if crate::friend_request_rate_limit::is_production_like_environment()
            && (self.postgres_store.is_none() || self.postgres_atomic_write_authority.is_none())
        {
            return Err("social normalized PostgreSQL authority is not installed".to_owned());
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Idempotent request retry detection
    // -----------------------------------------------------------------------

    pub(crate) fn resolve_committed_social_event_retry<T>(
        &self,
        state: &SocialControlState,
        commit: &CommitEnvelope,
        project: impl FnOnce(SocialCommittedEvent, SocialWritePersistence) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        let Some(existing) =
            state.committed_event(commit.tenant_id.as_str(), commit.event_id.as_str())
        else {
            return Ok(None);
        };
        if existing.commit() != commit {
            return Err(social_event_id_conflict_message(
                commit.event_id.as_str(),
                &existing,
            ));
        }
        let persistence = self.current_persistence();
        project(existing, persistence).map(Some)
    }
}

fn lock_social_state_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned {lock_name} lock");
            poisoned.into_inner()
        }
    }
}

// ---------------------------------------------------------------------------
// Index helpers
// ---------------------------------------------------------------------------

fn block_scope_index_label(scope: &BlockScope) -> &'static str {
    match scope {
        BlockScope::All => "all",
        BlockScope::Friendship => "friendship",
        BlockScope::DirectChat => "direct_chat",
    }
}

fn user_block_from_authority_record(
    record: im_adapters_social_postgres::user_block_store::UserBlockRecord,
) -> Result<UserBlock, String> {
    let scope = match record.scope.as_str() {
        "all" => BlockScope::All,
        "friendship" => BlockScope::Friendship,
        "direct_chat" => BlockScope::DirectChat,
        other => return Err(format!("normalized user-block scope is invalid: {other}")),
    };
    Ok(UserBlock {
        tenant_id: record.tenant_id,
        block_id: record.block_id.to_string(),
        blocker_user_id: record.blocker_user_id,
        blocked_user_id: record.blocked_user_id,
        scope,
        direct_chat_id: record.direct_chat_id.map(|value| value.to_string()),
        status: UserBlockStatus::Active,
        expires_at: record.expires_at,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

fn external_connection_kind_index_label(kind: &ExternalConnectionKind) -> &'static str {
    match kind {
        ExternalConnectionKind::SharedChannel => "shared_channel",
    }
}

fn external_connection_from_authority_record(
    record: im_adapters_social_postgres::external_store::ExternalConnectionRecord,
) -> Result<ExternalConnection, String> {
    let connection_kind = match record.connection_kind.as_str() {
        "shared_channel" => ExternalConnectionKind::SharedChannel,
        other => {
            return Err(format!(
                "normalized external-connection kind is invalid: {other}"
            ));
        }
    };
    let status = match record.status.as_str() {
        "active" => ExternalConnectionStatus::Active,
        "suspended" => ExternalConnectionStatus::Suspended,
        "revoked" => ExternalConnectionStatus::Revoked,
        other => {
            return Err(format!(
                "normalized external-connection status is invalid: {other}"
            ));
        }
    };
    Ok(ExternalConnection {
        tenant_id: record.tenant_id,
        connection_id: record.connection_id.to_string(),
        external_tenant_id: record.external_tenant_id,
        external_org_name: record.external_org_name,
        connection_kind,
        status,
        established_at: record.established_at,
        updated_at: record.updated_at,
    })
}

fn contract_error_message(error: ContractError) -> String {
    match error {
        ContractError::UnsupportedCapability(message)
        | ContractError::Conflict(message)
        | ContractError::Unavailable(message)
        | ContractError::Invalid(message) => message,
    }
}

fn social_event_id_conflict_message(event_id: &str, existing: &SocialCommittedEvent) -> String {
    let committed = existing.commit();
    format!(
        "eventId {} is already committed for {} {}",
        event_id,
        existing.aggregate_label(),
        committed.aggregate_id
    )
}

fn index_social_commits(
    index: &mut BTreeMap<SocialCommittedEventIndexKey, SocialCommittedEventPointer>,
    commits: &[CommitEnvelope],
    pointer: SocialCommittedEventPointer,
) {
    for (commit_index, commit) in commits.iter().enumerate() {
        index.insert(
            SocialCommittedEventIndexKey::new(commit.tenant_id.as_str(), commit.event_id.as_str()),
            pointer.with_commit_index(commit_index),
        );
    }
}

fn index_friend_request_record(
    pending_pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    accepted_pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    record: &StoredFriendRequest,
) {
    let fr = &record.friend_request;
    let Ok(pair) = fr.user_pair() else {
        return;
    };
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        fr.tenant_id.as_str(),
        organization_id.as_str(),
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    let target_index = match fr.status {
        FriendRequestStatus::Pending => pending_pair_index,
        FriendRequestStatus::Accepted => accepted_pair_index,
        _ => return,
    };
    target_index
        .entry(pair_key)
        .or_default()
        .insert(fr.request_id.clone());
    user_index
        .entry(SocialUserIndexKey::new(
            fr.tenant_id.as_str(),
            organization_id.as_str(),
            fr.requester_user_id.as_str(),
        ))
        .or_default()
        .insert(fr.request_id.clone());
    if fr.requester_user_id != fr.target_user_id {
        user_index
            .entry(SocialUserIndexKey::new(
                fr.tenant_id.as_str(),
                organization_id.as_str(),
                fr.target_user_id.as_str(),
            ))
            .or_default()
            .insert(fr.request_id.clone());
    }
}

fn unindex_friend_request_record(
    pending_pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    accepted_pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    record: &StoredFriendRequest,
) {
    let fr = &record.friend_request;
    let Ok(pair) = fr.user_pair() else {
        return;
    };
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        fr.tenant_id.as_str(),
        organization_id.as_str(),
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    for index in [pending_pair_index, accepted_pair_index] {
        if let Some(set) = index.get_mut(&pair_key) {
            set.remove(&fr.request_id);
            if set.is_empty() {
                index.remove(&pair_key);
            }
        }
    }
    for user_id in [&fr.requester_user_id, &fr.target_user_id] {
        let key = SocialUserIndexKey::new(
            fr.tenant_id.as_str(),
            organization_id.as_str(),
            user_id.as_str(),
        );
        if let Some(set) = user_index.get_mut(&key) {
            set.remove(&fr.request_id);
            if set.is_empty() {
                user_index.remove(&key);
            }
        }
    }
}

/// Reconstruct a minimal `StoredFriendRequest` from the commit envelopes
/// retained after eviction. Used by `committed_event()` so idempotency
/// checks still work for terminal friend requests that were evicted from
/// the in-memory `friend_requests` map.
pub(crate) fn reconstruct_evicted_friend_request(
    request_id: &str,
    commits: &[CommitEnvelope],
) -> Option<StoredFriendRequest> {
    use im_domain_events::social::{
        FriendRequestAcceptedPayload, FriendRequestCanceledPayload, FriendRequestDeclinedPayload,
        FriendRequestSubmittedPayload,
    };
    let submitted = commits
        .iter()
        .find(|c| c.event_type == "friend_request.submitted")?;
    let payload: FriendRequestSubmittedPayload = serde_json::from_str(&submitted.payload).ok()?;
    let tenant_id = submitted.tenant_id.clone();
    let mut friend_request = FriendRequest {
        tenant_id: tenant_id.clone(),
        request_id: payload.request_id.clone(),
        requester_user_id: payload.requester_user_id.clone(),
        target_user_id: payload.target_user_id.clone(),
        status: FriendRequestStatus::Pending,
        request_message: payload.request_message.clone(),
        expired_at: payload.expires_at.clone(),
        created_at: payload.requested_at.clone(),
        updated_at: payload.requested_at.clone(),
    };
    // Apply terminal commit if present.
    for commit in commits {
        match commit.event_type.as_str() {
            "friend_request.accepted" => {
                if let Ok(p) = serde_json::from_str::<FriendRequestAcceptedPayload>(&commit.payload)
                {
                    friend_request.status = FriendRequestStatus::Accepted;
                    friend_request.updated_at = p.accepted_at.clone();
                }
            }
            "friend_request.declined" => {
                if let Ok(p) = serde_json::from_str::<FriendRequestDeclinedPayload>(&commit.payload)
                {
                    friend_request.status = FriendRequestStatus::Declined;
                    friend_request.updated_at = p.declined_at.clone();
                }
            }
            "friend_request.canceled" => {
                if let Ok(p) = serde_json::from_str::<FriendRequestCanceledPayload>(&commit.payload)
                {
                    friend_request.status = FriendRequestStatus::Canceled;
                    friend_request.updated_at = p.canceled_at.clone();
                }
            }
            _ => {}
        }
    }
    let _ = request_id; // request_id is already in the payload
    Some(StoredFriendRequest {
        friend_request,
        commits: commits.to_vec(),
    })
}

fn index_friendship_record(
    active_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    active_user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredFriendship,
) {
    let f = &record.friendship;
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        f.tenant_id.as_str(),
        organization_id.as_str(),
        f.user_low_id.as_str(),
        f.user_high_id.as_str(),
    );
    pair_index
        .entry(pair_key.clone())
        .or_default()
        .insert(f.friendship_id.clone());
    if f.status.is_active() {
        active_pair_index.insert(pair_key, f.friendship_id.clone());
        for user_id in [&f.user_low_id, &f.user_high_id] {
            active_user_index
                .entry(SocialUserIndexKey::new(
                    f.tenant_id.as_str(),
                    organization_id.as_str(),
                    user_id.as_str(),
                ))
                .or_default()
                .insert(f.friendship_id.clone());
        }
    }
}

fn unindex_friendship_record(
    active_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    active_user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredFriendship,
) {
    let f = &record.friendship;
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        f.tenant_id.as_str(),
        organization_id.as_str(),
        f.user_low_id.as_str(),
        f.user_high_id.as_str(),
    );
    if let Some(set) = pair_index.get_mut(&pair_key) {
        set.remove(&f.friendship_id);
        if set.is_empty() {
            pair_index.remove(&pair_key);
        }
    }
    if active_pair_index
        .get(&pair_key)
        .is_some_and(|id| *id == f.friendship_id)
    {
        active_pair_index.remove(&pair_key);
    }
    for user_id in [&f.user_low_id, &f.user_high_id] {
        let key = SocialUserIndexKey::new(
            f.tenant_id.as_str(),
            organization_id.as_str(),
            user_id.as_str(),
        );
        if let Some(set) = active_user_index.get_mut(&key) {
            set.remove(&f.friendship_id);
            if set.is_empty() {
                active_user_index.remove(&key);
            }
        }
    }
}

fn index_user_block_record(
    active_scope_index: &mut BTreeMap<SocialUserBlockScopeIndexKey, String>,
    friendship_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_chat_index: &mut BTreeMap<SocialDirectChatBlockIndexKey, String>,
    record: &StoredUserBlock,
) {
    if !record.user_block.status.is_active() {
        return;
    }
    active_scope_index.insert(
        SocialUserBlockScopeIndexKey::new(&record.user_block),
        record.user_block.block_id.clone(),
    );
    let Some(pair_key) = user_block_pair_index_key(
        &record.user_block,
        organization_id_from_commits(&record.commits).as_str(),
    ) else {
        return;
    };
    match record.user_block.scope {
        BlockScope::All => {
            friendship_pair_index.insert(pair_key.clone(), record.user_block.block_id.clone());
            direct_chat_pair_index.insert(pair_key, record.user_block.block_id.clone());
        }
        BlockScope::Friendship => {
            friendship_pair_index.insert(pair_key, record.user_block.block_id.clone());
        }
        BlockScope::DirectChat => {
            if let Some(direct_chat_id) = record.user_block.direct_chat_id.as_deref() {
                direct_chat_chat_index.insert(
                    SocialDirectChatBlockIndexKey::new(
                        record.user_block.tenant_id.as_str(),
                        direct_chat_id,
                    ),
                    record.user_block.block_id.clone(),
                );
            }
            direct_chat_pair_index.insert(pair_key, record.user_block.block_id.clone());
        }
    }
}

fn unindex_user_block_record(
    active_scope_index: &mut BTreeMap<SocialUserBlockScopeIndexKey, String>,
    friendship_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_chat_index: &mut BTreeMap<SocialDirectChatBlockIndexKey, String>,
    record: &StoredUserBlock,
) {
    let scope_key = SocialUserBlockScopeIndexKey::new(&record.user_block);
    if active_scope_index
        .get(&scope_key)
        .is_some_and(|id| *id == record.user_block.block_id)
    {
        active_scope_index.remove(&scope_key);
    }
    let Some(pair_key) = user_block_pair_index_key(
        &record.user_block,
        organization_id_from_commits(&record.commits).as_str(),
    ) else {
        return;
    };
    for index in [friendship_pair_index, direct_chat_pair_index] {
        if index
            .get(&pair_key)
            .is_some_and(|id| *id == record.user_block.block_id)
        {
            index.remove(&pair_key);
        }
    }
    if let Some(direct_chat_id) = record.user_block.direct_chat_id.as_deref() {
        let chat_key = SocialDirectChatBlockIndexKey::new(
            record.user_block.tenant_id.as_str(),
            direct_chat_id,
        );
        if direct_chat_chat_index
            .get(&chat_key)
            .is_some_and(|id| *id == record.user_block.block_id)
        {
            direct_chat_chat_index.remove(&chat_key);
        }
    }
}

fn index_direct_chat_record(
    active_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredDirectChat,
) {
    let dc = &record.direct_chat;
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        dc.tenant_id.as_str(),
        organization_id.as_str(),
        dc.left_actor_id.as_str(),
        dc.right_actor_id.as_str(),
    );
    pair_index
        .entry(pair_key.clone())
        .or_default()
        .insert(dc.direct_chat_id.clone());
    if dc.status.is_active() {
        active_pair_index.insert(pair_key, dc.direct_chat_id.clone());
    }
}

fn unindex_direct_chat_record(
    active_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    pair_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredDirectChat,
) {
    let dc = &record.direct_chat;
    let organization_id = organization_id_from_commits(&record.commits);
    let pair_key = SocialPairIndexKey::new(
        dc.tenant_id.as_str(),
        organization_id.as_str(),
        dc.left_actor_id.as_str(),
        dc.right_actor_id.as_str(),
    );
    if let Some(set) = pair_index.get_mut(&pair_key) {
        set.remove(&dc.direct_chat_id);
        if set.is_empty() {
            pair_index.remove(&pair_key);
        }
    }
    if active_pair_index
        .get(&pair_key)
        .is_some_and(|id| *id == dc.direct_chat_id)
    {
        active_pair_index.remove(&pair_key);
    }
}

fn index_external_connection_record(
    active_target_index: &mut BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    record: &StoredExternalConnection,
) {
    if !record.external_connection.status.is_active() {
        return;
    }
    let key = SocialExternalConnectionTargetIndexKey::new(
        record.external_connection.tenant_id.as_str(),
        record.external_connection.external_tenant_id.as_str(),
        &record.external_connection.connection_kind,
    );
    active_target_index.insert(key, record.external_connection.connection_id.clone());
}

fn unindex_external_connection_record(
    active_target_index: &mut BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    record: &StoredExternalConnection,
) {
    let key = SocialExternalConnectionTargetIndexKey::new(
        record.external_connection.tenant_id.as_str(),
        record.external_connection.external_tenant_id.as_str(),
        &record.external_connection.connection_kind,
    );
    if active_target_index
        .get(&key)
        .is_some_and(|id| *id == record.external_connection.connection_id)
    {
        active_target_index.remove(&key);
    }
}

fn index_external_member_link_record(
    active_mapping_index: &mut BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredExternalMemberLink,
) {
    if !record.external_member_link.status.is_active() {
        return;
    }
    let link = &record.external_member_link;
    let mapping_key = SocialExternalMemberMappingIndexKey::new(
        link.tenant_id.as_str(),
        link.connection_id.as_str(),
        link.external_member_id.as_str(),
    );
    active_mapping_index.insert(mapping_key, link.link_id.clone());
    let connection_key =
        SocialConnectionIndexKey::new(link.tenant_id.as_str(), link.connection_id.as_str());
    active_connection_index
        .entry(connection_key)
        .or_default()
        .insert(link.link_id.clone());
}

fn unindex_external_member_link_record(
    active_mapping_index: &mut BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredExternalMemberLink,
) {
    let link = &record.external_member_link;
    let mapping_key = SocialExternalMemberMappingIndexKey::new(
        link.tenant_id.as_str(),
        link.connection_id.as_str(),
        link.external_member_id.as_str(),
    );
    if active_mapping_index
        .get(&mapping_key)
        .is_some_and(|id| *id == link.link_id)
    {
        active_mapping_index.remove(&mapping_key);
    }
    let connection_key =
        SocialConnectionIndexKey::new(link.tenant_id.as_str(), link.connection_id.as_str());
    if let Some(set) = active_connection_index.get_mut(&connection_key) {
        set.remove(&link.link_id);
        if set.is_empty() {
            active_connection_index.remove(&connection_key);
        }
    }
}

fn index_shared_channel_policy_record(
    active_target_index: &mut BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredSharedChannelPolicy,
) {
    if !record.shared_channel_policy.status.is_active() {
        return;
    }
    let policy = &record.shared_channel_policy;
    let target_key = SocialSharedChannelPolicyTargetIndexKey::new(
        policy.tenant_id.as_str(),
        policy.connection_id.as_str(),
        policy.channel_id.as_str(),
    );
    active_target_index.insert(target_key, policy.policy_id.clone());
    let connection_key =
        SocialConnectionIndexKey::new(policy.tenant_id.as_str(), policy.connection_id.as_str());
    active_connection_index
        .entry(connection_key)
        .or_default()
        .insert(policy.policy_id.clone());
}

fn unindex_shared_channel_policy_record(
    active_target_index: &mut BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredSharedChannelPolicy,
) {
    let policy = &record.shared_channel_policy;
    let target_key = SocialSharedChannelPolicyTargetIndexKey::new(
        policy.tenant_id.as_str(),
        policy.connection_id.as_str(),
        policy.channel_id.as_str(),
    );
    if active_target_index
        .get(&target_key)
        .is_some_and(|id| *id == policy.policy_id)
    {
        active_target_index.remove(&target_key);
    }
    let connection_key =
        SocialConnectionIndexKey::new(policy.tenant_id.as_str(), policy.connection_id.as_str());
    if let Some(set) = active_connection_index.get_mut(&connection_key) {
        set.remove(&policy.policy_id);
        if set.is_empty() {
            active_connection_index.remove(&connection_key);
        }
    }
}

fn index_pending_shared_channel_sync_request(
    retry_index: &mut BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    lease_index: &mut BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    request_key: &str,
    pending: &PendingSharedChannelSyncRequest,
) {
    if let Some(last_failed_at) = pending.last_failed_at.as_deref() {
        retry_index
            .entry(SharedChannelRetryIndexKey::new(last_failed_at))
            .or_default()
            .insert(request_key.to_owned());
    }
    if let Some(lease_expires_at) = pending.lease_expires_at.as_deref() {
        lease_index
            .entry(SharedChannelLeaseIndexKey::new(lease_expires_at))
            .or_default()
            .insert(request_key.to_owned());
    }
}

fn unindex_pending_shared_channel_sync_request(
    retry_index: &mut BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    lease_index: &mut BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    request_key: &str,
    pending: &PendingSharedChannelSyncRequest,
) {
    if let Some(last_failed_at) = pending.last_failed_at.as_deref() {
        let key = SharedChannelRetryIndexKey::new(last_failed_at);
        if let Some(set) = retry_index.get_mut(&key) {
            set.remove(request_key);
            if set.is_empty() {
                retry_index.remove(&key);
            }
        }
    }
    if let Some(lease_expires_at) = pending.lease_expires_at.as_deref() {
        let key = SharedChannelLeaseIndexKey::new(lease_expires_at);
        if let Some(set) = lease_index.get_mut(&key) {
            set.remove(request_key);
            if set.is_empty() {
                lease_index.remove(&key);
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) fn shared_channel_sync_request_key(
    request: &SharedChannelLinkedMemberSyncRequest,
) -> String {
    crate::shared_channel_sync_runtime::shared_channel_sync_request_key(request)
}

fn user_block_pair_index_key(
    user_block: &UserBlock,
    organization_id: &str,
) -> Option<SocialPairIndexKey> {
    let pair = user_block.user_pair().ok()?;
    Some(SocialPairIndexKey::new(
        user_block.tenant_id.as_str(),
        organization_id,
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    ))
}

// ---------------------------------------------------------------------------
// Active record query helpers
// ---------------------------------------------------------------------------

pub(crate) fn active_friendship_scoped_user_block(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_a: &str,
    user_b: &str,
) -> Option<UserBlock> {
    let pair = normalize_user_pair(user_a, user_b).ok()?;
    let pair_key = SocialPairIndexKey::new(
        tenant_id,
        organization_id,
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    active_user_block_by_id(
        state,
        state.active_friendship_block_pair_index.get(&pair_key)?,
    )
}

fn active_direct_chat_scoped_user_block(
    state: &SocialControlState,
    tenant_id: &str,
    direct_chat_id: &str,
) -> Option<UserBlock> {
    let direct_chat_record = state
        .direct_chats
        .get(direct_chat_id)
        .filter(|record| record.direct_chat.tenant_id == tenant_id)?;
    let direct_chat = &direct_chat_record.direct_chat;
    let organization_id = organization_id_from_commits(&direct_chat_record.commits);
    let pair = normalize_user_pair(
        direct_chat.left_actor_id.as_str(),
        direct_chat.right_actor_id.as_str(),
    )
    .ok()?;
    let chat_key = SocialDirectChatBlockIndexKey::new(tenant_id, direct_chat_id);
    if let Some(block_id) = state.active_direct_chat_block_chat_index.get(&chat_key)
        && let Some(user_block) = active_user_block_by_id(state, block_id)
    {
        return Some(user_block);
    }
    let pair_key = SocialPairIndexKey::new(
        tenant_id,
        organization_id.as_str(),
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    active_user_block_by_id(
        state,
        state.active_direct_chat_block_pair_index.get(&pair_key)?,
    )
}

fn active_user_block_by_id(state: &SocialControlState, block_id: &str) -> Option<UserBlock> {
    let record = state.user_blocks.get(block_id)?;
    if !record.user_block.status.is_active() {
        return None;
    }
    if user_block_is_expired(record.user_block.expires_at.as_deref()) {
        return None;
    }
    Some(record.user_block.clone())
}

fn user_block_is_expired(expires_at: Option<&str>) -> bool {
    let Some(expires_at) = expires_at.map(str::trim).filter(|value| !value.is_empty()) else {
        return false;
    };
    let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(expires_at) else {
        return false;
    };
    expiry.with_timezone(&chrono::Utc) <= chrono::Utc::now()
}

pub(crate) fn active_friend_request_block_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    requester_user_id: &str,
    target_user_id: &str,
) -> Option<UserBlock> {
    active_friendship_scoped_user_block(
        state,
        tenant_id,
        organization_id,
        requester_user_id,
        target_user_id,
    )
    .or_else(|| {
        active_direct_message_block_for_pair(
            state,
            tenant_id,
            organization_id,
            requester_user_id,
            target_user_id,
        )
    })
}

fn active_direct_message_block_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    sender_user_id: &str,
    peer_user_id: &str,
) -> Option<UserBlock> {
    if let Ok(pair) = normalize_user_pair(sender_user_id, peer_user_id) {
        let pair_key = SocialPairIndexKey::new(
            tenant_id,
            organization_id,
            pair.user_low_id.as_str(),
            pair.user_high_id.as_str(),
        );
        if let Some(block_id) = state.active_direct_chat_block_pair_index.get(&pair_key)
            && let Some(user_block) = active_user_block_by_id(state, block_id.as_str())
        {
            return Some(user_block);
        }
    }
    for scope in [BlockScope::All, BlockScope::DirectChat] {
        if let Some(record) = active_user_block_for_scope(
            state,
            tenant_id,
            peer_user_id,
            sender_user_id,
            &scope,
            None,
        ) {
            return Some(record.user_block);
        }
    }
    active_user_block_for_scope(
        state,
        tenant_id,
        sender_user_id,
        peer_user_id,
        &BlockScope::All,
        None,
    )
    .map(|record| record.user_block)
}

pub(crate) fn active_user_block_for_scope(
    state: &SocialControlState,
    tenant_id: &str,
    blocker_user_id: &str,
    blocked_user_id: &str,
    scope: &BlockScope,
    direct_chat_id: Option<&str>,
) -> Option<StoredUserBlock> {
    let probe = UserBlock {
        tenant_id: tenant_id.to_owned(),
        block_id: String::new(),
        blocker_user_id: blocker_user_id.to_owned(),
        blocked_user_id: blocked_user_id.to_owned(),
        scope: scope.clone(),
        status: UserBlockStatus::Active,
        direct_chat_id: direct_chat_id.map(ToOwned::to_owned),
        expires_at: None,
        created_at: String::new(),
        updated_at: String::new(),
    };
    state
        .user_blocks
        .get(
            state
                .active_user_block_scope_index
                .get(&SocialUserBlockScopeIndexKey::new(&probe))?,
        )
        .filter(|record| record.user_block.status.is_active())
        .filter(|record| !user_block_is_expired(record.user_block.expires_at.as_deref()))
        .cloned()
}

pub(crate) fn active_friendship_records_for_user(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_id: &str,
) -> Vec<StoredFriendship> {
    let key = SocialUserIndexKey::new(tenant_id, organization_id, user_id);
    state
        .active_friendship_user_index
        .get(&key)
        .into_iter()
        .flat_map(|friendship_ids| friendship_ids.iter())
        .filter_map(|friendship_id| {
            state
                .friendships
                .get(friendship_id)
                .filter(|record| record.friendship.status.is_active())
                .cloned()
        })
        .collect()
}

pub(crate) fn active_direct_chat_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    left_actor_id: &str,
    right_actor_id: &str,
) -> Option<StoredDirectChat> {
    let actor_pair =
        SocialPairIndexKey::new(tenant_id, organization_id, left_actor_id, right_actor_id);
    state
        .direct_chats
        .get(
            state
                .active_direct_chat_pair_index
                .get(&actor_pair)?
                .as_str(),
        )
        .filter(|record| record.direct_chat.status.is_active())
        .cloned()
}

// ---------------------------------------------------------------------------
// Friend request query helpers
// ---------------------------------------------------------------------------

fn first_indexed_friend_request_record_for_pair(
    state: &SocialControlState,
    index: &BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    key: &SocialPairIndexKey,
    expected_status: FriendRequestStatus,
) -> Option<StoredFriendRequest> {
    index.get(key)?.iter().find_map(|request_id| {
        state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.status == expected_status)
            .cloned()
    })
}

pub(crate) fn pending_friend_request_records_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> Vec<StoredFriendRequest> {
    let key = SocialPairIndexKey::new(tenant_id, organization_id, user_low_id, user_high_id);
    state
        .pending_friend_request_pair_index
        .get(&key)
        .into_iter()
        .flat_map(|request_ids| request_ids.iter())
        .filter_map(|request_id| {
            state
                .friend_requests
                .get(request_id)
                .filter(|record| record.friend_request.status == FriendRequestStatus::Pending)
                .cloned()
        })
        .collect()
}

pub(crate) fn open_friend_request_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    pair_has_materialized_friendship: bool,
) -> Option<StoredFriendRequest> {
    let key = SocialPairIndexKey::new(tenant_id, organization_id, user_low_id, user_high_id);
    first_indexed_friend_request_record_for_pair(
        state,
        &state.pending_friend_request_pair_index,
        &key,
        FriendRequestStatus::Pending,
    )
    .or_else(|| {
        if pair_has_materialized_friendship {
            None
        } else {
            accepted_friend_request_record_for_pair(
                state,
                tenant_id,
                organization_id,
                user_low_id,
                user_high_id,
            )
        }
    })
}

pub(crate) fn accepted_friend_request_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> Option<StoredFriendRequest> {
    let key = SocialPairIndexKey::new(tenant_id, organization_id, user_low_id, user_high_id);
    first_indexed_friend_request_record_for_pair(
        state,
        &state.accepted_friend_request_pair_index,
        &key,
        FriendRequestStatus::Accepted,
    )
}

pub(crate) fn friend_request_records_for_user(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_id: &str,
) -> Vec<StoredFriendRequest> {
    let key = SocialUserIndexKey::new(tenant_id, organization_id, user_id);
    state
        .friend_request_user_index
        .get(&key)
        .into_iter()
        .flat_map(|request_ids| request_ids.iter())
        .filter_map(|request_id| {
            state
                .friend_requests
                .get(request_id)
                .filter(|record| record.friend_request.tenant_id == tenant_id)
                .cloned()
        })
        .collect()
}

pub(crate) fn friendship_pair_has_materialized_record(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> bool {
    state
        .friendship_pair_index
        .contains_key(&SocialPairIndexKey::new(
            tenant_id,
            organization_id,
            user_low_id,
            user_high_id,
        ))
}

pub(crate) fn active_friendship_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> Option<StoredFriendship> {
    let key = SocialPairIndexKey::new(tenant_id, organization_id, user_low_id, user_high_id);
    state
        .friendships
        .get(state.active_friendship_pair_index.get(&key)?.as_str())
        .filter(|record| record.friendship.status.is_active())
        .cloned()
}

pub(crate) fn social_pair_block_conflict_details(user_block: &UserBlock) -> serde_json::Value {
    serde_json::json!({
        "blockId": user_block.block_id.clone(),
        "blockerUserId": user_block.blocker_user_id.clone(),
        "blockedUserId": user_block.blocked_user_id.clone(),
        "scope": user_block.scope.clone(),
        "directChatId": user_block.direct_chat_id.clone(),
    })
}

pub(crate) fn archive_active_direct_chats_for_pair(
    state: &mut SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    archived_at: &str,
) {
    let pair_hash = normalize_actor_pair(user_low_id, user_high_id)
        .expect("validated friendship pair should normalize into direct chat pair")
        .pair_hash;
    let actor_pair = normalize_actor_pair(user_low_id, user_high_id)
        .expect("validated friendship pair should normalize into direct chat pair");
    let index_key = SocialPairIndexKey::new(
        tenant_id,
        organization_id,
        actor_pair.left_actor_id.as_str(),
        actor_pair.right_actor_id.as_str(),
    );
    let direct_chat_ids = state
        .direct_chat_pair_index
        .get(&index_key)
        .cloned()
        .unwrap_or_default();
    for direct_chat_id in direct_chat_ids {
        let Some(mut record) = state.direct_chats.get(direct_chat_id.as_str()).cloned() else {
            continue;
        };
        if record.direct_chat.pair_hash != pair_hash || !record.direct_chat.status.is_active() {
            continue;
        }
        record.direct_chat.status = DirectChatStatus::Archived;
        record.direct_chat.updated_at = archived_at.to_owned();
        state.insert_direct_chat_record(direct_chat_id, record);
    }
}

// ---------------------------------------------------------------------------
// ID generation
// ---------------------------------------------------------------------------

pub(crate) fn deterministic_social_id(prefix: &str, seed: &str) -> String {
    let digest = sha256_hash(seed.as_bytes());
    format!("{prefix}{}", &digest[..24])
}

#[cfg(test)]
mod postgres_write_authority_tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use im_adapters_local_memory::MemoryCommitJournal;
    use im_platform_contracts::{CommitEnvelope, ContractError};

    use super::{SocialRuntime, SocialStateStore};
    use crate::postgres_write_authority::SocialAtomicWriteAuthority;

    struct InjectedAtomicWriteAuthority {
        calls: AtomicUsize,
        fail: bool,
    }

    impl InjectedAtomicWriteAuthority {
        fn new(fail: bool) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                fail,
            }
        }
    }

    impl SocialAtomicWriteAuthority for InjectedAtomicWriteAuthority {
        fn append_and_write(
            &self,
            mut commits: Vec<CommitEnvelope>,
        ) -> Result<Vec<CommitEnvelope>, ContractError> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            if self.fail {
                return Err(ContractError::Unavailable(
                    "injected atomic transaction failure".into(),
                ));
            }
            for commit in &mut commits {
                commit.ordering_seq = 41;
            }
            Ok(commits)
        }
    }

    fn sample_commit(event_id: &str) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "tenant-social-atomic",
            "social.atomicity_tested",
            "social_test",
            "aggregate-social-atomic",
            3,
        )
    }

    #[test]
    fn atomic_failure_never_falls_back_to_the_non_postgres_journal() {
        let fallback_journal = MemoryCommitJournal::default();
        let authority = Arc::new(InjectedAtomicWriteAuthority::new(true));
        let mut runtime = SocialRuntime::new(
            SocialStateStore::memory(),
            Arc::new(fallback_journal.clone()),
        );
        runtime.postgres_atomic_write_authority = Some(authority.clone());

        let result = runtime.persist_commits_to_authority(&[sample_commit("evt-atomic-fail")]);

        assert!(result.is_err());
        assert_eq!(authority.calls.load(Ordering::Relaxed), 1);
        assert!(
            fallback_journal.recorded().is_empty(),
            "an atomic PostgreSQL failure must not append to a fallback authority"
        );
    }

    #[test]
    fn atomic_success_returns_database_allocated_commit_metadata() {
        let fallback_journal = MemoryCommitJournal::default();
        let authority = Arc::new(InjectedAtomicWriteAuthority::new(false));
        let mut runtime = SocialRuntime::new(
            SocialStateStore::memory(),
            Arc::new(fallback_journal.clone()),
        );
        runtime.postgres_atomic_write_authority = Some(authority.clone());

        let (inserted, postgres_written) = runtime
            .persist_commits_to_authority(&[sample_commit("evt-atomic-success")])
            .expect("injected atomic authority should succeed");

        assert!(postgres_written);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0].ordering_seq, 41);
        assert_eq!(authority.calls.load(Ordering::Relaxed), 1);
        assert!(fallback_journal.recorded().is_empty());
    }

    #[test]
    fn memory_development_authority_keeps_its_bounded_local_behavior() {
        let journal = MemoryCommitJournal::default();
        let runtime = SocialRuntime::new(SocialStateStore::memory(), Arc::new(journal.clone()));
        let commit = sample_commit("evt-memory-authority");

        let (inserted, postgres_written) = runtime
            .persist_commits_to_authority(std::slice::from_ref(&commit))
            .expect("memory authority should append without a postgres normalized store");

        assert!(!postgres_written);
        assert_eq!(inserted, vec![commit.clone()]);
        assert_eq!(journal.recorded(), vec![commit]);
    }
}
