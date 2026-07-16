//! Space commit journal bootstrap aligned with social-service patterns.

use im_adapters_local_disk::FileCommitJournal;
use im_adapters_local_memory::MemoryCommitJournal;
use im_adapters_postgres_journal::PostgresCommitJournal;
use im_adapters_social_postgres::SpacePostgresMaterializer;
use im_platform_contracts::{
    COMMIT_JOURNAL_REPLAY_BATCH_LIMIT, CommitEnvelope, CommitJournal, CommitJournalAggregateScope,
    CommitJournalReplayCursor, CommitJournalReplayPage, ContractError, replay_commit_journal_pages,
};
use tracing::info;

/// Production-capable commit journal backend for space-service processes.
#[derive(Clone)]
pub enum SpaceCommitJournal {
    Memory(MemoryCommitJournal),
    File(FileCommitJournal),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for SpaceCommitJournal {
    fn append(
        &self,
        envelope: CommitEnvelope,
    ) -> Result<im_platform_contracts::CommitPosition, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::append(journal, envelope),
            Self::File(journal) => CommitJournal::append(journal, envelope),
            Self::Postgres(journal) => CommitJournal::append(journal, envelope),
        }
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<im_platform_contracts::CommitPosition>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::append_batch(journal, envelopes),
            Self::File(journal) => CommitJournal::append_batch(journal, envelopes),
            Self::Postgres(journal) => CommitJournal::append_batch(journal, envelopes),
        }
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded(journal),
            Self::File(journal) => CommitJournal::recorded(journal),
            Self::Postgres(journal) => CommitJournal::recorded(journal),
        }
    }

    fn recorded_page(
        &self,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded_page(journal, cursor, limit),
            Self::File(journal) => CommitJournal::recorded_page(journal, cursor, limit),
            Self::Postgres(journal) => CommitJournal::recorded_page(journal, cursor, limit),
        }
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
            Self::File(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
            Self::Postgres(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
        }
    }
}

pub fn replay_space_journal_to_read_model(
    journal: &SpaceCommitJournal,
    materializer: &SpacePostgresMaterializer,
) {
    let mut failures = 0_usize;
    let replayed_commits =
        match replay_commit_journal_pages(journal, COMMIT_JOURNAL_REPLAY_BATCH_LIMIT, |commits| {
            failures = failures.saturating_add(materializer.try_materialize_commits(commits));
            Ok(())
        }) {
            Ok(replayed_commits) => replayed_commits,
            Err(error) => {
                tracing::warn!(
                    error = ?error,
                    "space read-model replay skipped because commit journal readback is unavailable"
                );
                return;
            }
        };
    if replayed_commits == 0 {
        return;
    }
    if failures > 0 {
        crate::space_materializer_metrics::record_postgres_materialization_failures(
            failures as u64,
        );
    }
    info!(
        commit_count = replayed_commits,
        materialization_failures = failures,
        "replayed space commit journal into supplemental postgres read model"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;

    #[test]
    fn memory_journal_variant_delegates_append() {
        let journal = SpaceCommitJournal::Memory(MemoryCommitJournal::default());
        let envelope = CommitEnvelope::minimal(
            "evt-space-1",
            "100001",
            "space.created",
            "space",
            "space-1",
            1,
        );
        let position = journal.append(envelope).expect("append should succeed");
        assert_eq!(position.offset, 1);
        assert_eq!(position.partition, "local-memory");
    }
}
