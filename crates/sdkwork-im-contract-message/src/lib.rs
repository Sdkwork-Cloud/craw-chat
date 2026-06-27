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
