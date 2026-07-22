//! Atomic PostgreSQL authority for the Social journal and normalized state writes.

use std::sync::Arc;

use im_adapters_postgres_journal::PostgresCommitJournal;
use im_platform_contracts::{CommitEnvelope, ContractError};

use crate::normalized_store::SocialPostgresNormalizedStore;

pub(crate) trait SocialAtomicWriteAuthority: Send + Sync {
    fn append_and_write(
        &self,
        commits: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitEnvelope>, ContractError>;
}

pub(crate) struct SocialPostgresAtomicWriteAuthority {
    journal: PostgresCommitJournal,
    normalized_store: Arc<SocialPostgresNormalizedStore>,
}

impl SocialPostgresAtomicWriteAuthority {
    pub(crate) fn new(
        journal: PostgresCommitJournal,
        normalized_store: Arc<SocialPostgresNormalizedStore>,
    ) -> Self {
        Self {
            journal,
            normalized_store,
        }
    }
}

impl SocialAtomicWriteAuthority for SocialPostgresAtomicWriteAuthority {
    fn append_and_write(
        &self,
        commits: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitEnvelope>, ContractError> {
        if commits.is_empty() {
            return Ok(Vec::new());
        }

        let mut inserted_commits = Vec::new();
        self.journal
            .append_batch_with_allocated_sequences_in_transaction(
                commits,
                |txn, sequenced_commits| {
                    self.normalized_store
                        .write_commits_on_transaction(txn, sequenced_commits)?;
                    inserted_commits.extend_from_slice(sequenced_commits);
                    Ok(())
                },
            )?;
        Ok(inserted_commits)
    }
}
