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

/// Keyset cursor for incremental commit-journal replay (partition + monotonic offset).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitJournalReplayCursor {
    pub partition_key: String,
    pub commit_offset: u64,
}

/// One bounded page of journal replay results.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommitJournalReplayPage {
    pub items: Vec<CommitEnvelope>,
    pub next_cursor: Option<CommitJournalReplayCursor>,
}

/// Default replay page size for journal recovery and projection consumers.
pub const COMMIT_JOURNAL_REPLAY_BATCH_LIMIT: usize = 500;

/// Tenant + aggregate filter for scoped journal replay (single conversation recovery).
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
        let mut positions = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            positions.push(self.append(envelope)?);
        }
        Ok(positions)
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "journal readback is not implemented by this backend".into(),
        ))
    }

    /// Bounded journal replay page. Backends must override with store-level pagination.
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

    /// Bounded replay page restricted to one aggregate (store-level SQL filter when supported).
    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let limit = limit.max(1);
        let mut page_cursor = cursor.cloned();
        let mut collected = Vec::new();
        loop {
            let page = self.recorded_page(page_cursor.as_ref(), limit)?;
            if page.items.is_empty() {
                break;
            }
            for envelope in page.items {
                if envelope.tenant_id == scope.tenant_id
                    && (envelope.aggregate_id == scope.aggregate_id
                        || envelope.scope_id == scope.aggregate_id)
                {
                    collected.push(envelope);
                    if collected.len() >= limit {
                        break;
                    }
                }
            }
            if collected.len() >= limit {
                let next_cursor = collected.last().map(|envelope| CommitJournalReplayCursor {
                    partition_key: envelope.ordering_key.clone(),
                    commit_offset: envelope.ordering_seq,
                });
                return Ok(CommitJournalReplayPage {
                    items: collected,
                    next_cursor,
                });
            }
            page_cursor = page.next_cursor;
            if page_cursor.is_none() {
                break;
            }
        }
        let next_cursor = collected.last().map(|envelope| CommitJournalReplayCursor {
            partition_key: envelope.ordering_key.clone(),
            commit_offset: envelope.ordering_seq,
        });
        Ok(CommitJournalReplayPage {
            items: collected,
            next_cursor,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionRecord {
    pub message_seq: u64,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionBatch {
    pub tenant_id: String,
    pub timeline_scope: String,
    pub records: Vec<TimelineProjectionRecord>,
}

/// Keyset page of timeline projection payloads ordered by `message_seq`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TimelineProjectionWindow {
    pub items: Vec<(u64, String)>,
    pub has_more: bool,
}

pub trait TimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError>;

    fn load_timeline(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError>;

    /// Load a keyset window of timeline entries with `message_seq > after_seq`.
    fn load_timeline_window(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<TimelineProjectionWindow, ContractError> {
        let mut items = self
            .load_timeline(tenant_id, timeline_scope)?
            .into_iter()
            .filter(|(message_seq, _)| *message_seq > after_seq)
            .collect::<Vec<_>>();
        items.sort_by_key(|(message_seq, _)| *message_seq);
        let has_more = items.len() > limit;
        items.truncate(limit);
        Ok(TimelineProjectionWindow { items, has_more })
    }

    fn upsert_timeline_entries(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        for record in records {
            self.upsert_timeline_entry(
                tenant_id,
                timeline_scope,
                record.message_seq,
                record.payload.as_str(),
            )?;
        }
        Ok(())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        for batch in batches {
            self.upsert_timeline_entries(
                batch.tenant_id.as_str(),
                batch.timeline_scope.as_str(),
                &batch.records,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RecordedOnlyJournal;

    impl CommitJournal for RecordedOnlyJournal {
        fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
            Err(ContractError::UnsupportedCapability(
                "test journal append is not implemented".into(),
            ))
        }

        fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
            Ok(vec![CommitEnvelope::minimal(
                "evt-default-recorded-page",
                "100001",
                "message.posted",
                "conversation",
                "c_default",
                1,
            )])
        }
    }

    #[test]
    fn default_recorded_page_fails_closed_instead_of_full_reading_recorded() {
        let result = RecordedOnlyJournal.recorded_page(None, 1);

        assert!(
            matches!(
                result,
                Err(ContractError::UnsupportedCapability(message))
                    if message.contains("recorded_page")
            ),
            "CommitJournal::recorded_page default must fail closed unless the backend implements store-level pagination"
        );
    }
}
