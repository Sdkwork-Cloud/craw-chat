use sdkwork_im_contract_core::ContractError;

pub use im_domain_events::CommitEnvelope;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitPosition {
    pub partition: String,
    pub offset: u64,
}

impl CommitPosition {
    pub fn new(partition: impl Into<String>, offset: u64) -> Self {
        Self {
            partition: partition.into(),
            offset,
        }
    }

    pub fn cursor(&self) -> String {
        format!("{}:{}", self.partition, self.offset)
    }
}

/// Keyset cursor for incremental commit-journal replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitJournalReplayCursor {
    pub partition_key: String,
    pub commit_offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommitJournalReplayPage {
    pub items: Vec<CommitEnvelope>,
    pub next_cursor: Option<CommitJournalReplayCursor>,
}

pub const COMMIT_JOURNAL_REPLAY_BATCH_LIMIT: usize = 500;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitJournalAggregateScope {
    pub tenant_id: String,
    pub aggregate_id: String,
}

pub trait CommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError>;

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        envelopes
            .into_iter()
            .map(|envelope| self.append(envelope))
            .collect()
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "journal readback is not implemented by this backend".into(),
        ))
    }

    /// Load one bounded keyset page. Backends must implement store-level pagination.
    fn recorded_page(
        &self,
        _cursor: Option<&CommitJournalReplayCursor>,
        _limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "journal recorded_page requires an explicit store-level pagination implementation"
                .into(),
        ))
    }

    /// Load one aggregate-scoped page without scanning unrelated journal entries in memory.
    fn recorded_page_for_aggregate(
        &self,
        _scope: &CommitJournalAggregateScope,
        _cursor: Option<&CommitJournalReplayCursor>,
        _limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "journal recorded_page_for_aggregate requires an explicit store-level pagination implementation"
                .into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AppendOnlyJournal;

    impl CommitJournal for AppendOnlyJournal {
        fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
            Ok(CommitPosition::new("test", 1))
        }
    }

    #[test]
    fn pagination_defaults_fail_closed() {
        let journal = AppendOnlyJournal;
        let scope = CommitJournalAggregateScope {
            tenant_id: "tenant".into(),
            aggregate_id: "conversation".into(),
        };

        assert!(matches!(
            journal.recorded_page(None, 20),
            Err(ContractError::UnsupportedCapability(message)) if message.contains("recorded_page")
        ));
        assert!(matches!(
            journal.recorded_page_for_aggregate(&scope, None, 20),
            Err(ContractError::UnsupportedCapability(message)) if message.contains("recorded_page_for_aggregate")
        ));
    }
}
