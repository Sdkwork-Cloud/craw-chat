//! Atomic PostgreSQL authority for social journal and relational read-model writes.

use std::sync::Arc;

use im_adapters_postgres_journal::PostgresCommitJournal;
use im_platform_contracts::{CommitEnvelope, ContractError};

use crate::commit_materializer::SocialPostgresMaterializer;

pub(crate) trait SocialAtomicWriteAuthority: Send + Sync {
    fn append_and_materialize(
        &self,
        commits: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitEnvelope>, ContractError>;
}

pub(crate) struct SocialPostgresAtomicWriteAuthority {
    journal: PostgresCommitJournal,
    materializer: Arc<SocialPostgresMaterializer>,
}

impl SocialPostgresAtomicWriteAuthority {
    pub(crate) fn new(
        journal: PostgresCommitJournal,
        materializer: Arc<SocialPostgresMaterializer>,
    ) -> Self {
        Self {
            journal,
            materializer,
        }
    }
}

impl SocialAtomicWriteAuthority for SocialPostgresAtomicWriteAuthority {
    fn append_and_materialize(
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
                    self.materializer
                        .materialize_commits_on_transaction(txn, sequenced_commits)?;
                    inserted_commits.extend_from_slice(sequenced_commits);
                    Ok(())
                },
            )?;
        Ok(inserted_commits)
    }
}
