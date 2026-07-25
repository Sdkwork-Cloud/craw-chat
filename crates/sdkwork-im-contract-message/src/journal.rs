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

pub const COMMIT_JOURNAL_REPLAY_BATCH_LIMIT: usize = 200;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitJournalAggregateScope {
    pub tenant_id: String,
    pub aggregate_id: String,
}

/// A bounded, organization-scoped event-type audit query for one aggregate.
/// Explicit recovery and audit tools use it without scanning unrelated
/// messages; ordinary business reads do not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitJournalAggregateEventTypeQuery {
    pub tenant_id: String,
    pub organization_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_types: Vec<String>,
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

    /// Load one keyset page for only the requested event types in one
    /// organization-scoped aggregate. Implementations must filter and page at
    /// the authoritative journal store.
    fn recorded_page_for_aggregate_event_types(
        &self,
        _query: &CommitJournalAggregateEventTypeQuery,
        _cursor: Option<&CommitJournalReplayCursor>,
        _limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "journal recorded_page_for_aggregate_event_types requires an explicit store-level pagination implementation"
                .into(),
        ))
    }
}

pub fn replay_commit_journal_pages(
    journal: &dyn CommitJournal,
    requested_batch_limit: usize,
    mut consume: impl FnMut(&[CommitEnvelope]) -> Result<(), ContractError>,
) -> Result<usize, ContractError> {
    let batch_limit = requested_batch_limit.clamp(1, COMMIT_JOURNAL_REPLAY_BATCH_LIMIT);
    let mut cursor: Option<CommitJournalReplayCursor> = None;
    let mut replayed_count = 0_usize;

    loop {
        let page = journal.recorded_page(cursor.as_ref(), batch_limit)?;
        if page.items.len() > batch_limit {
            return Err(ContractError::Unavailable(
                "journal backend returned a replay page larger than the requested bound".into(),
            ));
        }
        if page.items.is_empty() {
            if let Some(next_cursor) = page.next_cursor.as_ref()
                && cursor.as_ref() != Some(next_cursor)
            {
                return Err(ContractError::Unavailable(
                    "journal backend advanced the replay cursor without returning records".into(),
                ));
            }
            return Ok(replayed_count);
        }

        consume(page.items.as_slice())?;
        replayed_count = replayed_count
            .checked_add(page.items.len())
            .ok_or_else(|| {
                ContractError::Unavailable("journal replay record count overflowed usize".into())
            })?;

        let Some(next_cursor) = page.next_cursor else {
            return Ok(replayed_count);
        };
        if !replay_cursor_strictly_advances(cursor.as_ref(), &next_cursor) {
            return Err(ContractError::Unavailable(
                "journal backend returned a non-advancing replay cursor".into(),
            ));
        }
        cursor = Some(next_cursor);
    }
}

fn replay_cursor_strictly_advances(
    previous: Option<&CommitJournalReplayCursor>,
    next: &CommitJournalReplayCursor,
) -> bool {
    let Some(previous) = previous else {
        return true;
    };
    next.partition_key > previous.partition_key
        || (next.partition_key == previous.partition_key
            && next.commit_offset > previous.commit_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

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

    struct PagedJournal {
        events: Vec<CommitEnvelope>,
        requested_limits: Mutex<Vec<usize>>,
        repeat_cursor: bool,
    }

    impl CommitJournal for PagedJournal {
        fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
            Err(ContractError::UnsupportedCapability(
                "test journal is read-only".into(),
            ))
        }

        fn recorded_page(
            &self,
            cursor: Option<&CommitJournalReplayCursor>,
            limit: usize,
        ) -> Result<CommitJournalReplayPage, ContractError> {
            self.requested_limits
                .lock()
                .expect("requested limit lock")
                .push(limit);
            let start = cursor
                .map(|value| usize::try_from(value.commit_offset).unwrap_or(usize::MAX))
                .unwrap_or(0);
            let end = start.saturating_add(limit).min(self.events.len());
            let items = self.events.get(start..end).unwrap_or_default().to_vec();
            let next_cursor = (end < self.events.len()).then(|| CommitJournalReplayCursor {
                partition_key: "test".into(),
                commit_offset: if self.repeat_cursor {
                    cursor.map(|value| value.commit_offset).unwrap_or(0)
                } else {
                    u64::try_from(end).unwrap_or(u64::MAX)
                },
            });
            Ok(CommitJournalReplayPage { items, next_cursor })
        }
    }

    fn replay_events(count: usize) -> Vec<CommitEnvelope> {
        (0..count)
            .map(|index| {
                let event_id = format!("event-{index}");
                CommitEnvelope::minimal(
                    event_id.as_str(),
                    "tenant-1",
                    "message.posted",
                    "conversation",
                    "conversation-1",
                    u64::try_from(index).unwrap_or(u64::MAX).saturating_add(1),
                )
            })
            .collect()
    }

    #[test]
    fn bounded_replay_visits_every_record_across_multiple_pages() {
        let journal = PagedJournal {
            events: replay_events(501),
            requested_limits: Mutex::new(Vec::new()),
            repeat_cursor: false,
        };
        let mut batch_sizes = Vec::new();
        let replayed = replay_commit_journal_pages(&journal, usize::MAX, |batch| {
            batch_sizes.push(batch.len());
            Ok(())
        })
        .expect("bounded replay should succeed");

        assert_eq!(replayed, 501);
        assert_eq!(batch_sizes, vec![200, 200, 101]);
        assert!(
            journal
                .requested_limits
                .lock()
                .expect("requested limit lock")
                .iter()
                .all(|limit| *limit == COMMIT_JOURNAL_REPLAY_BATCH_LIMIT)
        );
    }

    #[test]
    fn bounded_replay_rejects_non_advancing_cursor() {
        let journal = PagedJournal {
            events: replay_events(201),
            requested_limits: Mutex::new(Vec::new()),
            repeat_cursor: true,
        };

        assert!(matches!(
            replay_commit_journal_pages(&journal, 200, |_| Ok(())),
            Err(ContractError::Unavailable(message)) if message.contains("non-advancing")
        ));
    }
}
