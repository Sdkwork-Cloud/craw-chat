//! Normalized PostgreSQL stores for Social mutations and queries.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_adapters_social_postgres::direct_chat_store::DirectChatStore;
use im_adapters_social_postgres::friend_request_store::FriendRequestStore;
use im_adapters_social_postgres::friendship_store::FriendshipStore;
use im_platform_contracts::{CommitEnvelope, ContractError};

pub struct SocialPostgresNormalizedStore {
    friend_request_store: Arc<dyn FriendRequestStore>,
    friendship_store: Arc<dyn FriendshipStore>,
    direct_chat_store: Arc<dyn DirectChatStore>,
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

    /// Write normalized Social rows on the journal-owned transaction.
    pub fn write_commits_on_transaction(
        &self,
        txn: &mut postgres::Transaction<'_>,
        commits: &[CommitEnvelope],
    ) -> Result<(), ContractError> {
        im_adapters_social_postgres::materialize_commits_on_transaction(txn, commits)
    }
}
