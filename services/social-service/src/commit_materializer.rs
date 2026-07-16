//! Materialize social commit envelopes into supplemental PostgreSQL stores.

use std::sync::Arc;

use im_adapters_social_postgres::SocialPostgresPool;
use im_adapters_social_postgres::direct_chat_store::DirectChatStore;
use im_adapters_social_postgres::friend_request_store::FriendRequestStore;
use im_adapters_social_postgres::friendship_store::FriendshipStore;
use im_platform_contracts::{CommitEnvelope, ContractError};

pub struct SocialPostgresMaterializer {
    pool: SocialPostgresPool,
    friend_request_store: Arc<dyn FriendRequestStore>,
    friendship_store: Arc<dyn FriendshipStore>,
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

    /// Materialize journal replay or drift-repair commits in an adapter-owned transaction.
    pub fn materialize_commits(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        if commits.is_empty() {
            return Ok(());
        }
        im_adapters_social_postgres::materialize_commits_in_transaction(&self.pool, commits)
    }

    /// Materialize online writes on the journal-owned transaction.
    pub fn materialize_commits_on_transaction(
        &self,
        txn: &mut postgres::Transaction<'_>,
        commits: &[CommitEnvelope],
    ) -> Result<(), ContractError> {
        im_adapters_social_postgres::materialize_commits_on_transaction(txn, commits)
    }
}
