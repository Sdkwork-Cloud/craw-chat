//! Normalized PostgreSQL stores for Social mutations and queries.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_adapters_social_postgres::direct_chat_store::DirectChatStore;
use im_adapters_social_postgres::external_store::{
    ExternalConnectionStore, ExternalMemberLinkStore,
};
use im_adapters_social_postgres::friend_request_store::FriendRequestStore;
use im_adapters_social_postgres::friendship_store::FriendshipStore;
use im_adapters_social_postgres::shared_channel_store::SharedChannelPolicyStore;
use im_adapters_social_postgres::user_block_store::UserBlockStore;
use im_platform_contracts::{CommitEnvelope, ContractError};

pub struct SocialPostgresNormalizedStore {
    friend_request_store: Arc<dyn FriendRequestStore>,
    friendship_store: Arc<dyn FriendshipStore>,
    direct_chat_store: Arc<dyn DirectChatStore>,
    user_block_store: Arc<dyn UserBlockStore>,
    external_connection_store: Arc<dyn ExternalConnectionStore>,
    external_member_link_store: Arc<dyn ExternalMemberLinkStore>,
    shared_channel_policy_store: Arc<dyn SharedChannelPolicyStore>,
}

impl SocialPostgresNormalizedStore {
    pub fn from_pool(pool: SocialPostgresPool) -> Self {
        let pool_arc = Arc::new(pool.inner().clone());
        Self {
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
            direct_chat_store: Arc::new(
                im_adapters_social_postgres::direct_chat_store::PostgresDirectChatStore::new(
                    pool_arc.clone(),
                ),
            ),
            user_block_store: Arc::new(
                im_adapters_social_postgres::user_block_store::PostgresUserBlockStore::new(
                    pool_arc.clone(),
                ),
            ),
            external_connection_store: Arc::new(
                im_adapters_social_postgres::external_store::PostgresExternalConnectionStore::new(
                    pool_arc.clone(),
                ),
            ),
            external_member_link_store: Arc::new(
                im_adapters_social_postgres::external_store::PostgresExternalMemberLinkStore::new(
                    pool_arc.clone(),
                ),
            ),
            shared_channel_policy_store: Arc::new(
                im_adapters_social_postgres::shared_channel_store::PostgresSharedChannelPolicyStore::new(
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

    pub fn user_block_store(&self) -> Arc<dyn UserBlockStore> {
        self.user_block_store.clone()
    }

    pub fn external_connection_store(&self) -> Arc<dyn ExternalConnectionStore> {
        self.external_connection_store.clone()
    }

    pub fn external_member_link_store(&self) -> Arc<dyn ExternalMemberLinkStore> {
        self.external_member_link_store.clone()
    }

    pub fn shared_channel_policy_store(&self) -> Arc<dyn SharedChannelPolicyStore> {
        self.shared_channel_policy_store.clone()
    }

    /// Write normalized Social rows on the journal-owned transaction.
    pub fn write_commits_on_transaction(
        &self,
        txn: &mut postgres::Transaction<'_>,
        commits: &[CommitEnvelope],
    ) -> Result<(), ContractError> {
        im_adapters_social_postgres::materialize_commits_on_transaction(txn, commits)
    }
}
