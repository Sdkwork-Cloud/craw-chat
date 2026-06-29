use std::cmp::Ordering as CmpOrdering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path as StdPath, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use getrandom::fill as fill_random;
use im_adapters_local_disk::{FileCommitJournal, read_commit_journal_file};
use im_adapters_local_memory::MemoryCommitJournal;
use im_domain_core::social::{
    BlockScope, DirectChat, DirectChatStatus, ExternalConnection, ExternalConnectionKind,
    ExternalConnectionStatus, ExternalMemberLink, ExternalMemberLinkStatus, FriendRequest,
    FriendRequestStatus, Friendship, FriendshipStatus, SharedChannelPolicy,
    SharedChannelPolicyStatus, UserBlock, UserBlockStatus, normalize_actor_pair,
    normalize_user_pair,
};
use im_domain_events::normalize_commit_organization_id;
use im_platform_contracts::{CommitEnvelope, CommitJournal, ContractError};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use sdkwork_utils_rust::sha256_hash;

use crate::SharedChannelLinkedMemberSyncRequest;

const SOCIAL_STATE_FILE_NAME: &str = "social-state.json";
const SOCIAL_COMMIT_JOURNAL_FILE_NAME: &str = "social-commit-journal.json";
const SOCIAL_TRANSACTION_MARKER_FILE_NAME: &str = "social-transaction-marker.json";
const SOCIAL_WRITE_LOCK_FILE_NAME: &str = "social-write.lock";
const SOCIAL_COMMIT_PARTITION: &str = "control-plane-social";

// ---------------------------------------------------------------------------
// SocialStateStore
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum SocialStateStore {
    Memory(Arc<Mutex<SocialControlState>>),
    File {
        file_path: Arc<PathBuf>,
        io_lock: Arc<Mutex<()>>,
    },
    Database {
        pool: im_adapters_social_postgres::SocialPostgresPool,
    },
}

impl SocialStateStore {
    pub(crate) fn memory() -> Self {
        Self::Memory(Arc::new(Mutex::new(SocialControlState::default())))
    }

    pub(crate) fn file(file_path: impl Into<PathBuf>) -> Self {
        Self::File {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn database(pool: im_adapters_social_postgres::SocialPostgresPool) -> Self {
        Self::Database { pool }
    }

    pub(crate) fn load(&self) -> Result<SocialControlState, String> {
        match self {
            Self::Memory(state) => {
                let mut loaded =
                    lock_social_state_mutex(state, "social-state-store.memory").clone();
                loaded.rebuild_social_indexes();
                Ok(loaded)
            }
            Self::File { file_path, io_lock } => {
                let _guard = lock_social_state_mutex(io_lock, "social-state-store.file-io");
                if !file_path.exists() {
                    return Ok(SocialControlState::default());
                }
                let content = fs::read_to_string(file_path.as_path()).map_err(|error| {
                    format!(
                        "failed to read social state file {}: {error}",
                        file_path.display()
                    )
                })?;
                if content.trim().is_empty() {
                    return Err(format!(
                        "social state file {} is empty",
                        file_path.display()
                    ));
                }
                let mut loaded: SocialControlState =
                    serde_json::from_str(&content).map_err(|error| {
                        format!(
                            "failed to parse social state file {}: {error}",
                            file_path.display()
                        )
                    })?;
                loaded.rebuild_social_indexes();
                Ok(loaded)
            }
            Self::Database { pool: _ } => {
                // Database mode loads from PostgreSQL on demand
                // For now, return default state - actual DB queries happen through store methods
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
            Self::File { file_path, io_lock } => {
                let _guard = lock_social_state_mutex(io_lock, "social-state-store.file-io");
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).map_err(|error| {
                        format!(
                            "failed to create social state parent directory {}: {error}",
                            parent.display()
                        )
                    })?;
                }
                let payload = serde_json::to_string_pretty(state)
                    .map_err(|error| format!("failed to serialize social state: {error}"))?;
                write_file_atomically(file_path.as_path(), payload.as_bytes(), "social state file")
            }
            Self::Database { pool: _ } => {
                // Database mode persists through individual store operations
                // No bulk save needed - each mutation writes directly to PostgreSQL
                Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Index key types
// ---------------------------------------------------------------------------

fn organization_id_from_commits(commits: &[CommitEnvelope]) -> String {
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

    pub(crate) fn committed_event_keys(&self) -> BTreeSet<(String, String)> {
        let mut event_keys = BTreeSet::new();
        for record in self.friend_requests.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.friendships.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.user_blocks.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.direct_chats.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.external_connections.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.external_member_links.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.shared_channel_policies.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        event_keys
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
                let record = self.friend_requests.get(request_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
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

    pub(crate) fn replay_commit_journal_file(
        &mut self,
        journal_path: &StdPath,
    ) -> Result<bool, String> {
        let commits = read_commit_journal_file(journal_path).map_err(|error| {
            format!(
                "failed to read social commit journal {}: {}",
                journal_path.display(),
                contract_error_message(error)
            )
        })?;
        let mut known_event_keys = self.committed_event_keys();
        let mut changed = false;
        for commit in commits {
            if !known_event_keys.insert((commit.tenant_id.clone(), commit.event_id.clone())) {
                continue;
            }
            self.apply_social_commit(commit)?;
            changed = true;
        }
        Ok(changed)
    }

    fn apply_social_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let event_type = commit.event_type.clone();
        match event_type.as_str() {
            "friend_request.submitted" => self.apply_friend_request_commit(commit),
            "friend_request.accepted" => self.apply_friend_request_accepted_commit(commit),
            "friend_request.declined" => self.apply_friend_request_declined_commit(commit),
            "friend_request.canceled" => self.apply_friend_request_canceled_commit(commit),
            "friendship.activated" => self.apply_friendship_commit(commit),
            "friendship.removed" => self.apply_friendship_removed_commit(commit),
            "user_block.blocked" => self.apply_user_block_commit(commit),
            "direct_chat.bound" => self.apply_direct_chat_commit(commit),
            "external_connection.established" => self.apply_external_connection_commit(commit),
            "external_member_link.bound" => self.apply_external_member_link_commit(commit),
            "shared_channel_policy.applied" => self.apply_shared_channel_policy_commit(commit),
            _ => Err(format!(
                "unsupported social replay event type {} for aggregate {}",
                event_type, commit.aggregate_id
            )),
        }
    }

    fn apply_friend_request_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::FriendRequestSubmittedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friend request replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friend_request",
            payload.request_id.as_str(),
        )?;
        normalize_user_pair(
            payload.requester_user_id.as_str(),
            payload.target_user_id.as_str(),
        )
        .map_err(|error| {
            format!(
                "failed to validate friend request replay payload for {}: {error}",
                commit.event_id
            )
        })?;

        let friend_request = FriendRequest {
            tenant_id: commit.tenant_id.clone(),
            request_id: payload.request_id.clone(),
            requester_user_id: payload.requester_user_id,
            target_user_id: payload.target_user_id,
            status: FriendRequestStatus::Pending,
            request_message: payload.request_message,
            expired_at: None,
            created_at: payload.requested_at.clone(),
            updated_at: payload.requested_at,
        };
        let request_id = friend_request.request_id.clone();
        let mut record = self
            .friend_requests
            .get(request_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredFriendRequest {
                friend_request: friend_request.clone(),
                commits: Vec::new(),
            });
        record.friend_request = friend_request;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_accepted_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: im_domain_events::social::FriendRequestAcceptedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friend request accept replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friend_request",
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request accept replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(record.friend_request.status, FriendRequestStatus::Pending) {
            return Err(format!(
                "friend request accept replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        if payload.accepted_by_user_id != record.friend_request.target_user_id {
            return Err(format!(
                "friend request accept replay payload for {} must be accepted by target user {}",
                commit.event_id, record.friend_request.target_user_id
            ));
        }
        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Accepted;
        record.friend_request.updated_at = payload.accepted_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_declined_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: im_domain_events::social::FriendRequestDeclinedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friend request decline replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friend_request",
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request decline replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(
            record.friend_request.status,
            FriendRequestStatus::Pending | FriendRequestStatus::Declined
        ) {
            return Err(format!(
                "friend request decline replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        if payload.declined_by_user_id != record.friend_request.target_user_id {
            return Err(format!(
                "friend request decline replay payload for {} must be declined by target user {}",
                commit.event_id, record.friend_request.target_user_id
            ));
        }
        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Declined;
        record.friend_request.updated_at = payload.declined_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_canceled_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: im_domain_events::social::FriendRequestCanceledPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friend request cancel replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friend_request",
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request cancel replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(record.friend_request.status, FriendRequestStatus::Pending) {
            return Err(format!(
                "friend request cancel replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Canceled;
        record.friend_request.updated_at = payload.canceled_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friendship_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::FriendshipActivatedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friendship replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friendship",
            payload.friendship_id.as_str(),
        )?;
        let pair = normalize_user_pair(payload.user_low_id.as_str(), payload.user_high_id.as_str())
            .map_err(|error| {
                format!(
                    "failed to validate friendship replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        let friendship = Friendship {
            tenant_id: commit.tenant_id.clone(),
            friendship_id: payload.friendship_id.clone(),
            user_low_id: pair.user_low_id,
            user_high_id: pair.user_high_id,
            initiator_user_id: payload.initiator_user_id,
            status: FriendshipStatus::Active,
            established_at: Some(payload.established_at.clone()),
            updated_at: payload.established_at,
        };
        let friendship_id = friendship.friendship_id.clone();
        let mut record = self
            .friendships
            .get(friendship_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredFriendship {
                friendship: friendship.clone(),
                commits: Vec::new(),
            });
        record.friendship = friendship;
        record.commits.push(commit);
        self.insert_friendship_record(friendship_id, record);
        Ok(())
    }

    fn apply_friendship_removed_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::FriendshipRemovedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse friendship removed replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "friendship",
            payload.friendship_id.as_str(),
        )?;
        normalize_user_pair(payload.user_low_id.as_str(), payload.user_high_id.as_str()).map_err(
            |error| {
                format!(
                    "failed to validate friendship removal replay payload for {}: {error}",
                    commit.event_id
                )
            },
        )?;
        let mut record = self
            .friendships
            .get(payload.friendship_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friendship removed replay payload for {} references missing friendship {}",
                    commit.event_id, payload.friendship_id
                )
            })?;
        if !matches!(record.friendship.status, FriendshipStatus::Active) {
            return Err(format!(
                "friendship removed replay payload for {} cannot transition friendship {} from {:?}",
                commit.event_id, payload.friendship_id, record.friendship.status
            ));
        }
        let friendship_id = record.friendship.friendship_id.clone();
        record.friendship.status = FriendshipStatus::Removed;
        record.friendship.updated_at = payload.removed_at;
        record.commits.push(commit);
        self.insert_friendship_record(friendship_id, record);
        Ok(())
    }

    fn apply_user_block_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::UserBlockedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse user block replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(&commit, "user_block", payload.block_id.as_str())?;
        let scope: BlockScope =
            serde_json::from_str(&format!("\"{}\"", payload.scope)).map_err(|error| {
                format!(
                    "failed to parse user block scope '{}' for {}: {error}",
                    payload.scope, commit.event_id
                )
            })?;
        let user_block = UserBlock {
            tenant_id: commit.tenant_id.clone(),
            block_id: payload.block_id.clone(),
            blocker_user_id: payload.blocker_user_id,
            blocked_user_id: payload.blocked_user_id,
            scope,
            status: UserBlockStatus::Active,
            direct_chat_id: payload.direct_chat_id,
            expires_at: payload.expires_at,
            created_at: payload.effective_at.clone(),
            updated_at: payload.effective_at,
        };
        let block_id = user_block.block_id.clone();
        let mut record = self
            .user_blocks
            .get(block_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredUserBlock {
                user_block: user_block.clone(),
                commits: Vec::new(),
            });
        record.user_block = user_block;
        record.commits.push(commit);
        self.insert_user_block_record(block_id, record);
        Ok(())
    }

    fn apply_direct_chat_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::DirectChatBoundPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse direct chat replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "direct_chat",
            payload.direct_chat_id.as_str(),
        )?;
        let direct_chat = DirectChat {
            tenant_id: commit.tenant_id.clone(),
            direct_chat_id: payload.direct_chat_id.clone(),
            left_actor_id: payload.left_actor_id,
            right_actor_id: payload.right_actor_id,
            pair_hash: payload.pair_hash,
            status: DirectChatStatus::Active,
            conversation_id: Some(payload.conversation_id),
            created_at: payload.bound_at.clone(),
            updated_at: payload.bound_at,
        };
        let direct_chat_id = direct_chat.direct_chat_id.clone();
        let mut record = self
            .direct_chats
            .get(direct_chat_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredDirectChat {
                direct_chat: direct_chat.clone(),
                commits: Vec::new(),
            });
        record.direct_chat = direct_chat;
        record.commits.push(commit);
        self.insert_direct_chat_record(direct_chat_id, record);
        Ok(())
    }

    fn apply_external_connection_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::ExternalConnectionEstablishedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse external connection replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "external_connection",
            payload.connection_id.as_str(),
        )?;
        let connection_kind: ExternalConnectionKind =
            serde_json::from_str(&format!("\"{}\"", payload.connection_kind)).map_err(|error| {
                format!(
                    "failed to parse external connection kind '{}' for {}: {error}",
                    payload.connection_kind, commit.event_id
                )
            })?;
        let external_connection = ExternalConnection {
            tenant_id: commit.tenant_id.clone(),
            connection_id: payload.connection_id.clone(),
            external_tenant_id: payload.external_tenant_id,
            external_org_name: payload.external_org_name,
            connection_kind,
            status: ExternalConnectionStatus::Active,
            established_at: payload.established_at.clone(),
            updated_at: payload.established_at,
        };
        let connection_id = external_connection.connection_id.clone();
        let mut record = self
            .external_connections
            .get(connection_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredExternalConnection {
                external_connection: external_connection.clone(),
                commits: Vec::new(),
            });
        record.external_connection = external_connection;
        record.commits.push(commit);
        self.insert_external_connection_record(connection_id, record);
        Ok(())
    }

    fn apply_external_member_link_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::ExternalMemberLinkBoundPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse external member link replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "external_member_link",
            payload.link_id.as_str(),
        )?;
        let external_member_link = ExternalMemberLink {
            tenant_id: commit.tenant_id.clone(),
            link_id: payload.link_id.clone(),
            connection_id: payload.connection_id,
            external_member_id: payload.external_member_id,
            local_actor_id: payload.local_actor_id,
            local_actor_kind: payload.local_actor_kind,
            external_display_name: payload.external_display_name,
            status: ExternalMemberLinkStatus::Active,
            linked_at: payload.linked_at.clone(),
            updated_at: payload.linked_at,
        };
        let link_id = external_member_link.link_id.clone();
        let mut record = self
            .external_member_links
            .get(link_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredExternalMemberLink {
                external_member_link: external_member_link.clone(),
                commits: Vec::new(),
            });
        record.external_member_link = external_member_link;
        record.commits.push(commit);
        self.insert_external_member_link_record(link_id, record);
        Ok(())
    }

    fn apply_shared_channel_policy_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: im_domain_events::social::SharedChannelPolicyAppliedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse shared channel policy replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target_envelope(
            &commit,
            "shared_channel_policy",
            payload.policy_id.as_str(),
        )?;
        let shared_channel_policy = SharedChannelPolicy {
            tenant_id: commit.tenant_id.clone(),
            policy_id: payload.policy_id.clone(),
            connection_id: payload.connection_id,
            channel_id: payload.channel_id,
            conversation_id: payload.conversation_id,
            policy_version: payload.policy_version,
            history_visibility: payload.history_visibility,
            status: SharedChannelPolicyStatus::Active,
            applied_at: payload.applied_at.clone(),
            updated_at: payload.applied_at,
        };
        let policy_id = shared_channel_policy.policy_id.clone();
        let mut record = self
            .shared_channel_policies
            .get(policy_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredSharedChannelPolicy {
                shared_channel_policy: shared_channel_policy.clone(),
                commits: Vec::new(),
            });
        record.shared_channel_policy = shared_channel_policy;
        record.commits.push(commit);
        self.insert_shared_channel_policy_record(policy_id, record);
        Ok(())
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

    // Merge helpers (used during journal replay to incorporate snapshot-only data)

    pub(crate) fn merge_pending_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, pending) in &other.pending_shared_channel_sync_requests {
            if !self.pending_shared_channel_sync_requests.contains_key(key) {
                self.upsert_pending_shared_channel_sync_request(key.clone(), pending.clone());
            }
        }
    }

    pub(crate) fn merge_dead_letter_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, dead_letter) in &other.dead_letter_shared_channel_sync_requests {
            self.dead_letter_shared_channel_sync_requests
                .entry(key.clone())
                .or_insert_with(|| dead_letter.clone());
        }
    }

    pub(crate) fn merge_delivered_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, delivered_at) in &other.delivered_shared_channel_sync_requests {
            self.delivered_shared_channel_sync_requests
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(delivered_at, existing) {
                        *existing = delivered_at.clone();
                    }
                })
                .or_insert_with(|| delivered_at.clone());
        }
    }

    pub(crate) fn merge_delivered_shared_channel_sync_delivery_proofs_from(
        &mut self,
        other: &Self,
    ) {
        for (key, proof) in &other.delivered_shared_channel_sync_delivery_proofs {
            self.delivered_shared_channel_sync_delivery_proofs
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(
                        proof.delivered_at.as_str(),
                        existing.delivered_at.as_str(),
                    ) || (proof.delivered_at == existing.delivered_at
                        && existing.status
                            == crate::SharedChannelSyncDeliveryProofStatus::TransportAccepted
                        && proof.status
                            != crate::SharedChannelSyncDeliveryProofStatus::TransportAccepted)
                    {
                        *existing = proof.clone();
                    }
                })
                .or_insert_with(|| proof.clone());
        }
    }

    pub(crate) fn merge_recent_shared_channel_sync_deliveries_from(&mut self, other: &Self) {
        for (key, delivered_at) in &other.recent_shared_channel_sync_deliveries {
            self.recent_shared_channel_sync_deliveries
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(delivered_at, existing) {
                        *existing = delivered_at.clone();
                    }
                })
                .or_insert_with(|| delivered_at.clone());
        }
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
            requests,
            error,
            now,
            false,
        )
    }
}

// ---------------------------------------------------------------------------
// Transaction marker types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialTransactionMarkerStatus {
    PendingSnapshotRepair,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialTransactionMarker {
    pub(crate) status: SocialTransactionMarkerStatus,
    pub(crate) event_id: String,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialDerivedSnapshotStatus {
    Current,
    RepairRequired,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialWritePersistence {
    pub(crate) journal_authority: bool,
    pub(crate) snapshot_status: SocialDerivedSnapshotStatus,
}

// ---------------------------------------------------------------------------
// SocialWriteLockGuard
// ---------------------------------------------------------------------------

pub(crate) struct SocialWriteLockGuard {
    file: fs::File,
}

impl Drop for SocialWriteLockGuard {
    fn drop(&mut self) {
        if let Err(error) = self.file.unlock() {
            tracing::warn!("failed to unlock social write lock: {error}");
        }
    }
}

// Use fs4 lock extension trait
use fs4::fs_std::FileExt;

// ---------------------------------------------------------------------------
// SocialAuthorityLoad
// ---------------------------------------------------------------------------

struct SocialAuthorityLoad {
    state: SocialControlState,
    replay_error: Option<String>,
}

// ---------------------------------------------------------------------------
// SocialRuntime
// ---------------------------------------------------------------------------

pub struct SocialRuntime {
    state_store: SocialStateStore,
    commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    pub(crate) state: RwLock<SocialControlState>,
    authority_replay_error: RwLock<Option<String>>,
    journal_path: Option<Arc<PathBuf>>,
    tx_marker_path: Option<Arc<PathBuf>>,
    write_lock_path: Option<Arc<PathBuf>>,
    snapshot_failpoint_path: Option<Arc<PathBuf>>,
    shared_channel_sync_trigger:
        RwLock<Option<Arc<dyn crate::SharedChannelLinkedMemberSyncTrigger>>>,
    #[allow(dead_code)]
    pub(crate) shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialRuntimeFailpoints {
    fail_next_snapshot_save: bool,
}

impl Default for SocialRuntime {
    fn default() -> Self {
        Self::new(
            SocialStateStore::memory(),
            Arc::new(MemoryCommitJournal::default()),
        )
    }
}

impl SocialRuntime {
    pub fn new(
        state_store: SocialStateStore,
        commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    ) -> Self {
        Self::new_with_snapshot_failpoint(state_store, commit_journal, None)
    }

    pub fn new_with_snapshot_failpoint(
        state_store: SocialStateStore,
        commit_journal: Arc<dyn CommitJournal + Send + Sync>,
        snapshot_failpoint_path: Option<PathBuf>,
    ) -> Self {
        let authority_load = Self::load_social_state_for_authority(
            &state_store,
            "failed to load social state during runtime bootstrap",
        );
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(authority_load.state),
            authority_replay_error: RwLock::new(authority_load.replay_error),
            journal_path: None,
            tx_marker_path: None,
            write_lock_path: None,
            snapshot_failpoint_path: snapshot_failpoint_path.map(Arc::new),
            shared_channel_sync_trigger: RwLock::new(None),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
    }

    pub fn from_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Self {
        let state_dir = runtime_dir.as_ref().join("state");
        let journal_path = state_dir.join(SOCIAL_COMMIT_JOURNAL_FILE_NAME);
        let tx_marker_path = state_dir.join(SOCIAL_TRANSACTION_MARKER_FILE_NAME);
        let write_lock_path = state_dir.join(SOCIAL_WRITE_LOCK_FILE_NAME);
        let state_store = SocialStateStore::file(state_dir.join(SOCIAL_STATE_FILE_NAME));
        let commit_journal = Arc::new(FileCommitJournal::new(
            SOCIAL_COMMIT_PARTITION,
            journal_path.clone(),
        ));
        let authority_load = Self::load_state_with_journal_replay(
            &state_store,
            journal_path.as_path(),
            Some(tx_marker_path.as_path()),
        );
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(authority_load.state),
            authority_replay_error: RwLock::new(authority_load.replay_error),
            journal_path: Some(Arc::new(journal_path)),
            tx_marker_path: Some(Arc::new(tx_marker_path)),
            write_lock_path: Some(Arc::new(write_lock_path)),
            snapshot_failpoint_path: Some(Arc::new(state_dir.join("social-failpoints.json"))),
            shared_channel_sync_trigger: RwLock::new(None),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
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
    // State loading
    // -----------------------------------------------------------------------

    fn load_social_state_for_authority(
        state_store: &SocialStateStore,
        context: &str,
    ) -> SocialAuthorityLoad {
        match state_store.load() {
            Ok(state) => SocialAuthorityLoad {
                state,
                replay_error: None,
            },
            Err(error) => {
                let replay_error = format!("{context}: {error}");
                tracing::warn!(
                    "{replay_error}. social authority is unavailable until the snapshot is repaired"
                );
                SocialAuthorityLoad {
                    state: SocialControlState::default(),
                    replay_error: Some(replay_error),
                }
            }
        }
    }

    fn load_state_with_journal_replay(
        state_store: &SocialStateStore,
        journal_path: &StdPath,
        tx_marker_path: Option<&StdPath>,
    ) -> SocialAuthorityLoad {
        if journal_path.exists() {
            let snapshot_load = Self::load_social_state_for_authority(
                state_store,
                "failed to load social snapshot during journal replay bootstrap",
            );
            let snapshot_state = snapshot_load.state;
            let mut replayed_state = match Self::replay_state_from_commit_journal(journal_path) {
                Ok(state) => state,
                Err(error) => {
                    let replay_error = format!(
                        "failed to replay social commit journal {}: {error}",
                        journal_path.display()
                    );
                    tracing::warn!(
                        "{replay_error}. social authority is unavailable until the journal is repaired"
                    );
                    return SocialAuthorityLoad {
                        state: snapshot_state,
                        replay_error: Some(replay_error),
                    };
                }
            };
            replayed_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state.merge_delivered_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state
                .merge_delivered_shared_channel_sync_delivery_proofs_from(&snapshot_state);
            replayed_state.merge_recent_shared_channel_sync_deliveries_from(&snapshot_state);
            if let Err(error) = state_store.save(&replayed_state) {
                tracing::warn!(
                    "failed to persist replayed social state {}: {error}. continuing with in-memory replayed state",
                    journal_path.display()
                );
            }
            if let Some(marker_path) = tx_marker_path
                && let Err(error) = clear_social_transaction_marker(marker_path)
            {
                tracing::warn!(
                    "failed to clear social transaction marker after journal replay {}: {error}",
                    marker_path.display()
                );
            }
            return SocialAuthorityLoad {
                state: replayed_state,
                replay_error: None,
            };
        }

        Self::load_social_state_for_authority(
            state_store,
            "failed to load social state without commit journal",
        )
    }

    fn replay_state_from_commit_journal(
        journal_path: &StdPath,
    ) -> Result<SocialControlState, String> {
        let mut replayed_state = SocialControlState::default();
        replayed_state.replay_commit_journal_file(journal_path)?;
        replayed_state.rebuild_social_indexes();
        Ok(replayed_state)
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

    pub(crate) fn user_block_snapshot(
        &self,
        tenant_id: &str,
        block_id: &str,
    ) -> Option<StoredUserBlock> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .user_blocks
            .get(block_id)
            .filter(|record| record.user_block.tenant_id == tenant_id)
            .cloned()
    }

    pub fn authoritative_active_friendships_for_user(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Result<Vec<Friendship>, String> {
        let state = self.authoritative_state_for_query()?;
        let mut friendships = active_friendship_records_for_user(
            &state,
            tenant_id,
            organization_id,
            user_id,
        )
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
        let state = self.authoritative_state_for_query()?;
        Ok(
            active_direct_chat_record_for_pair(
                &state,
                tenant_id,
                organization_id,
                user_low_id,
                user_high_id,
            )
            .map(|record| record.direct_chat),
        )
    }

    pub fn external_connection_snapshot(
        &self,
        tenant_id: &str,
        connection_id: &str,
    ) -> Option<StoredExternalConnection> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .external_connections
            .get(connection_id)
            .filter(|record| record.external_connection.tenant_id == tenant_id)
            .cloned()
    }

    fn authoritative_state_for_query(&self) -> Result<SocialControlState, String> {
        match self.journal_path.as_deref() {
            Some(journal_path) => {
                let authority_load = Self::load_state_with_journal_replay(
                    &self.state_store,
                    journal_path,
                    self.tx_marker_path.as_deref().map(|path| path.as_path()),
                );
                if let Some(error) = authority_load.replay_error {
                    return Err(error);
                }
                Ok(authority_load.state)
            }
            None => self.state_store.load(),
        }
    }

    // -----------------------------------------------------------------------
    // State persistence
    // -----------------------------------------------------------------------

    fn persistence_with_snapshot_status(
        &self,
        snapshot_status: SocialDerivedSnapshotStatus,
    ) -> SocialWritePersistence {
        SocialWritePersistence {
            journal_authority: matches!(self.state_store, SocialStateStore::File { .. }),
            snapshot_status,
        }
    }

    pub(crate) fn current_persistence(&self) -> SocialWritePersistence {
        self.persistence_with_snapshot_status(SocialDerivedSnapshotStatus::Current)
    }

    fn repair_required_persistence(&self) -> SocialWritePersistence {
        self.persistence_with_snapshot_status(SocialDerivedSnapshotStatus::RepairRequired)
    }

    fn write_pending_tx_marker(&self, event_id: &str) -> Result<(), String> {
        let Some(path) = self.tx_marker_path.as_deref() else {
            return Ok(());
        };
        write_social_transaction_marker(
            path,
            &SocialTransactionMarker {
                status: SocialTransactionMarkerStatus::PendingSnapshotRepair,
                event_id: event_id.to_owned(),
            },
        )
    }

    fn clear_pending_tx_marker(&self) -> Result<bool, String> {
        let Some(path) = self.tx_marker_path.as_deref() else {
            return Ok(false);
        };
        clear_social_transaction_marker(path)
    }

    fn consume_fail_next_snapshot_save(&self) -> Result<bool, String> {
        let Some(path) = self.snapshot_failpoint_path.as_deref() else {
            return Ok(false);
        };
        if !path.exists() {
            return Ok(false);
        }
        let content = fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read social failpoint file {}: {error}",
                path.display()
            )
        })?;
        if content.trim().is_empty() {
            return Ok(false);
        }
        let mut failpoints: SocialRuntimeFailpoints = serde_json::from_str(content.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse social failpoint file {}: {error}",
                    path.display()
                )
            })?;
        if !failpoints.fail_next_snapshot_save {
            return Ok(false);
        }
        failpoints.fail_next_snapshot_save = false;
        let payload = serde_json::to_string_pretty(&failpoints)
            .map_err(|error| format!("failed to serialize social failpoints: {error}"))?;
        fs::write(path, payload).map_err(|error| {
            format!(
                "failed to consume social failpoint file {}: {error}",
                path.display()
            )
        })?;
        Ok(true)
    }

    pub(crate) fn persist_state_transition(
        &self,
        next: &SocialControlState,
        commit: &CommitEnvelope,
    ) -> Result<SocialWritePersistence, String> {
        self.commit_journal
            .append(commit.clone())
            .map_err(|error| {
                format!(
                    "failed to append social commit journal before state write: {}",
                    contract_error_message(error)
                )
            })?;
        self.write_pending_tx_marker(commit.event_id.as_str())?;
        if self.consume_fail_next_snapshot_save()? {
            return Ok(self.repair_required_persistence());
        }
        if self.state_store.save(next).is_err() {
            return Ok(self.repair_required_persistence());
        }
        if self.clear_pending_tx_marker().is_err() {
            return Ok(self.repair_required_persistence());
        }
        Ok(self.current_persistence())
    }

    pub(crate) fn persist_state_transition_batch(
        &self,
        next: &SocialControlState,
        commits: &[CommitEnvelope],
    ) -> Result<SocialWritePersistence, String> {
        let Some(marker_event_id) = commits.first().map(|commit| commit.event_id.as_str()) else {
            return Ok(self.current_persistence());
        };
        self.commit_journal
            .append_batch(commits.to_vec())
            .map_err(|error| {
                format!(
                    "failed to append social commit journal batch before state write: {}",
                    contract_error_message(error)
                )
            })?;
        self.write_pending_tx_marker(marker_event_id)?;
        if self.consume_fail_next_snapshot_save()? {
            return Ok(self.repair_required_persistence());
        }
        if self.state_store.save(next).is_err() {
            return Ok(self.repair_required_persistence());
        }
        if self.clear_pending_tx_marker().is_err() {
            return Ok(self.repair_required_persistence());
        }
        Ok(self.current_persistence())
    }

    // -----------------------------------------------------------------------
    // Write lock management
    // -----------------------------------------------------------------------

    pub(crate) fn acquire_cross_instance_write_lock(
        &self,
    ) -> Result<Option<SocialWriteLockGuard>, String> {
        let Some(path) = self.write_lock_path.as_deref() else {
            return Ok(None);
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create social lock directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)
            .map_err(|error| {
                format!(
                    "failed to open social write lock {}: {error}",
                    path.display()
                )
            })?;
        file.lock_exclusive().map_err(|error| {
            format!(
                "failed to acquire social write lock {}: {error}",
                path.display()
            )
        })?;
        Ok(Some(SocialWriteLockGuard { file }))
    }

    pub(crate) fn acquire_cross_instance_read_lock(
        &self,
    ) -> Result<Option<SocialWriteLockGuard>, String> {
        let Some(path) = self.write_lock_path.as_deref() else {
            return Ok(None);
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create social lock directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)
            .map_err(|error| {
                format!(
                    "failed to open social read lock {}: {error}",
                    path.display()
                )
            })?;
        file.lock_shared().map_err(|error| {
            format!(
                "failed to acquire social read lock {}: {error}",
                path.display()
            )
        })?;
        Ok(Some(SocialWriteLockGuard { file }))
    }

    // -----------------------------------------------------------------------
    // State refresh from authority
    // -----------------------------------------------------------------------

    pub(crate) fn refresh_state_from_authority_for_write(&self) -> Result<(), String> {
        let Some(journal_path) = self.journal_path.as_deref() else {
            self.ensure_social_authority_available()?;
            return Ok(());
        };

        let mut authoritative_state = if journal_path.exists() {
            match Self::replay_state_from_commit_journal(journal_path) {
                Ok(replayed) => replayed,
                Err(error) => {
                    let replay_error = format!(
                        "failed to replay social commit journal {} during cross-instance refresh: {error}",
                        journal_path.display()
                    );
                    *self
                        .authority_replay_error
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                        Some(replay_error.clone());
                    return Err(replay_error);
                }
            }
        } else {
            match self.state_store.load() {
                Ok(snapshot_state) => snapshot_state,
                Err(error) => {
                    let replay_error = format!(
                        "failed to load social snapshot during cross-instance write refresh without commit journal: {error}"
                    );
                    *self
                        .authority_replay_error
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                        Some(replay_error.clone());
                    return Err(replay_error);
                }
            }
        };
        let snapshot_state = match self.state_store.load() {
            Ok(snapshot_state) => snapshot_state,
            Err(error) if journal_path.exists() => {
                tracing::warn!(
                    "failed to load social snapshot during cross-instance write refresh: {error}. continuing from commit journal authority"
                );
                SocialControlState::default()
            }
            Err(error) => {
                let replay_error = format!(
                    "failed to load social snapshot during cross-instance write refresh without commit journal: {error}"
                );
                *self
                    .authority_replay_error
                    .write()
                    .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                    Some(replay_error.clone());
                return Err(replay_error);
            }
        };
        authoritative_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state.merge_delivered_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state
            .merge_delivered_shared_channel_sync_delivery_proofs_from(&snapshot_state);
        authoritative_state.merge_recent_shared_channel_sync_deliveries_from(&snapshot_state);
        *self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = authoritative_state;
        *self
            .authority_replay_error
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = None;
        Ok(())
    }

    pub(crate) fn ensure_social_authority_available(&self) -> Result<(), String> {
        let replay_error = self
            .authority_replay_error
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        if let Some(error) = replay_error {
            return Err(error);
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Snapshot repair
    // -----------------------------------------------------------------------

    pub(crate) fn repair_derived_snapshot_best_effort(
        &self,
        state: &SocialControlState,
    ) -> SocialWritePersistence {
        if self.state_store.save(state).is_ok() && self.clear_pending_tx_marker().is_ok() {
            self.current_persistence()
        } else {
            self.repair_required_persistence()
        }
    }

    pub fn repair_derived_snapshot(&self) -> Result<SocialRuntimeRepairResponse, String> {
        let pending_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        let mut repaired_state = if let Some(journal_path) = self.journal_path.as_deref() {
            Self::replay_state_from_commit_journal(journal_path).map_err(|error| {
                format!("failed to replay social commit journal during repair: {error}")
            })?
        } else {
            self.state
                .read()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                .clone()
        };
        repaired_state.merge_pending_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_dead_letter_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_delivered_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_delivered_shared_channel_sync_delivery_proofs_from(&pending_state);
        repaired_state.merge_recent_shared_channel_sync_deliveries_from(&pending_state);
        self.state_store
            .save(&repaired_state)
            .map_err(|error| format!("failed to repair derived social state snapshot: {error}"))?;
        let transaction_marker_cleared = self.clear_pending_tx_marker().map_err(|error| {
            format!("failed to clear social transaction marker after repair: {error}")
        })?;
        *self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = repaired_state.clone();
        Ok(SocialRuntimeRepairResponse {
            status: SocialRuntimeRepairStatus::Repaired,
            journal_authority: matches!(self.state_store, SocialStateStore::File { .. }),
            snapshot_updated: true,
            transaction_marker_cleared,
            aggregate_counts: repaired_state.aggregate_counts(),
        })
    }

    // -----------------------------------------------------------------------
    // Event replay detection
    // -----------------------------------------------------------------------

    pub(crate) fn replay_committed_social_event<T>(
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
        let persistence = self.repair_derived_snapshot_best_effort(state);
        project(existing, persistence).map(Some)
    }
}

// ---------------------------------------------------------------------------
// Repair response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialRuntimeRepairStatus {
    Repaired,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAggregateCountsResponse {
    pub friend_requests: usize,
    pub friendships: usize,
    pub user_blocks: usize,
    pub direct_chats: usize,
    pub external_connections: usize,
    pub external_member_links: usize,
    pub shared_channel_policies: usize,
    pub pending_shared_channel_sync_requests: usize,
    pub dead_letter_shared_channel_sync_requests: usize,
    pub delivered_shared_channel_sync_requests: usize,
    pub recent_shared_channel_sync_deliveries: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialRuntimeRepairResponse {
    pub status: SocialRuntimeRepairStatus,
    pub journal_authority: bool,
    pub snapshot_updated: bool,
    pub transaction_marker_cleared: bool,
    pub aggregate_counts: SocialAggregateCountsResponse,
}

impl SocialControlState {
    fn aggregate_counts(&self) -> SocialAggregateCountsResponse {
        SocialAggregateCountsResponse {
            friend_requests: self.friend_requests.len(),
            friendships: self.friendships.len(),
            user_blocks: self.user_blocks.len(),
            direct_chats: self.direct_chats.len(),
            external_connections: self.external_connections.len(),
            external_member_links: self.external_member_links.len(),
            shared_channel_policies: self.shared_channel_policies.len(),
            pending_shared_channel_sync_requests: self.pending_shared_channel_sync_requests.len(),
            dead_letter_shared_channel_sync_requests: self
                .dead_letter_shared_channel_sync_requests
                .len(),
            delivered_shared_channel_sync_requests: self
                .delivered_shared_channel_sync_requests
                .len(),
            recent_shared_channel_sync_deliveries: self.recent_shared_channel_sync_deliveries.len(),
        }
    }
}

// ---------------------------------------------------------------------------
// File I/O helpers
// ---------------------------------------------------------------------------

fn write_file_atomically(
    file_path: &StdPath,
    payload: &[u8],
    store_name: &str,
) -> Result<(), String> {
    let parent = file_path
        .parent()
        .ok_or_else(|| format!("{store_name} path has no parent: {}", file_path.display()))?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "failed to create {store_name} parent directory {}: {error}",
            parent.display()
        )
    })?;

    let temp_path = atomic_temp_path(file_path)?;
    let write_result = (|| {
        let mut temp_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(temp_path.as_path())
            .map_err(|error| {
                format!(
                    "failed to create {store_name} temp file {}: {error}",
                    temp_path.display()
                )
            })?;
        temp_file.write_all(payload).map_err(|error| {
            format!(
                "failed to write {store_name} temp file {}: {error}",
                temp_path.display()
            )
        })?;
        temp_file.sync_all().map_err(|error| {
            format!(
                "failed to sync {store_name} temp file {}: {error}",
                temp_path.display()
            )
        })?;
        drop(temp_file);
        replace_file_atomically(temp_path.as_path(), file_path).map_err(|error| {
            format!(
                "failed to atomically replace {store_name} {} from temp file {}: {error}",
                file_path.display(),
                temp_path.display()
            )
        })?;
        sync_parent_directory(parent, store_name)?;
        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(temp_path.as_path());
    }
    write_result
}

fn atomic_temp_path(file_path: &StdPath) -> Result<PathBuf, String> {
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("file path has no valid file name: {}", file_path.display()))?;
    let mut random = [0_u8; 8];
    fill_random(&mut random).map_err(|error| {
        format!(
            "failed to generate temporary file suffix for {}: {error}",
            file_path.display()
        )
    })?;
    let suffix = u64::from_le_bytes(random);
    Ok(file_path.with_file_name(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        suffix
    )))
}

#[cfg(windows)]
fn replace_file_atomically(temp_path: &StdPath, file_path: &StdPath) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(
            lp_existing_file_name: *const u16,
            lp_new_file_name: *const u16,
            dw_flags: u32,
        ) -> i32;
    }

    let existing = temp_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let new = file_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let replaced = unsafe {
        MoveFileExW(
            existing.as_ptr(),
            new.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if replaced == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(())
}

#[cfg(not(windows))]
fn replace_file_atomically(temp_path: &StdPath, file_path: &StdPath) -> Result<(), String> {
    fs::rename(temp_path, file_path).map_err(|error| error.to_string())
}

fn sync_parent_directory(parent: &StdPath, store_name: &str) -> Result<(), String> {
    #[cfg(unix)]
    {
        fs::File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|error| {
                format!(
                    "failed to sync {store_name} parent directory {}: {error}",
                    parent.display()
                )
            })?;
    }
    let _ = (parent, store_name);
    Ok(())
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

fn write_social_transaction_marker(
    marker_path: &StdPath,
    marker: &SocialTransactionMarker,
) -> Result<(), String> {
    if let Some(parent) = marker_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create social transaction marker parent directory {}: {error}",
                parent.display()
            )
        })?;
    }
    let payload = serde_json::to_string_pretty(marker)
        .map_err(|error| format!("failed to serialize social transaction marker: {error}"))?;
    fs::write(marker_path, payload).map_err(|error| {
        format!(
            "failed to write social transaction marker file {}: {error}",
            marker_path.display()
        )
    })
}

fn clear_social_transaction_marker(marker_path: &StdPath) -> Result<bool, String> {
    if !marker_path.exists() {
        return Ok(false);
    }
    fs::remove_file(marker_path).map_err(|error| {
        format!(
            "failed to remove social transaction marker file {}: {error}",
            marker_path.display()
        )
    })?;
    Ok(true)
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

fn external_connection_kind_index_label(kind: &ExternalConnectionKind) -> &'static str {
    match kind {
        ExternalConnectionKind::SharedChannel => "shared_channel",
    }
}

fn validate_social_commit_target_envelope(
    commit: &CommitEnvelope,
    expected_aggregate_label: &str,
    expected_aggregate_id: &str,
) -> Result<(), String> {
    let actual_aggregate_label = commit.aggregate_type.as_wire_value().trim_matches('"');
    if actual_aggregate_label != expected_aggregate_label {
        return Err(format!(
            "social commit {} aggregate type mismatch: expected {}, got {}",
            commit.event_id, expected_aggregate_label, actual_aggregate_label
        ));
    }
    if commit.aggregate_id != expected_aggregate_id {
        return Err(format!(
            "social commit {} aggregate id mismatch: expected {}, got {}",
            commit.event_id, expected_aggregate_id, commit.aggregate_id
        ));
    }
    Ok(())
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
pub(crate) fn shared_channel_sync_request_key(request: &SharedChannelLinkedMemberSyncRequest) -> String {
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
    state
        .user_blocks
        .get(block_id)
        .filter(|record| record.user_block.status.is_active())
        .map(|record| record.user_block.clone())
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
    let actor_pair = SocialPairIndexKey::new(tenant_id, organization_id, left_actor_id, right_actor_id);
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
            first_indexed_friend_request_record_for_pair(
                state,
                &state.accepted_friend_request_pair_index,
                &key,
                FriendRequestStatus::Accepted,
            )
        }
    })
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

// ---------------------------------------------------------------------------
// Timestamp helpers
// ---------------------------------------------------------------------------

#[allow(dead_code)]
fn current_unix_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn is_canonical_rfc3339_millis_utc(timestamp: &str) -> bool {
    let bytes = timestamp.as_bytes();
    if bytes.len() != 24 {
        return false;
    }
    for index in [4, 7] {
        if bytes[index] != b'-' {
            return false;
        }
    }
    if bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes[19] != b'.' {
        return false;
    }
    if bytes[23] != b'Z' {
        return false;
    }
    for index in [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18, 20, 21, 22] {
        if !bytes[index].is_ascii_digit() {
            return false;
        }
    }
    true
}

fn compare_canonical_rfc3339_millis_utc(left: &str, right: &str) -> Option<CmpOrdering> {
    if !is_canonical_rfc3339_millis_utc(left) || !is_canonical_rfc3339_millis_utc(right) {
        return None;
    }
    Some(left.cmp(right))
}

fn timestamp_newer_for_recency(candidate: &str, existing: &str) -> bool {
    matches!(
        compare_canonical_rfc3339_millis_utc(candidate, existing),
        Some(CmpOrdering::Greater)
    )
}
