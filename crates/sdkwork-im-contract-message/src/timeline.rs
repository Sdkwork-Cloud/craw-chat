use sdkwork_im_contract_core::ContractError;

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

    /// Load a bounded keyset window with `message_seq > after_seq`.
    fn load_timeline_window(
        &self,
        _tenant_id: &str,
        _timeline_scope: &str,
        _after_seq: u64,
        _limit: usize,
    ) -> Result<TimelineProjectionWindow, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "timeline load_timeline_window requires an explicit store-level pagination implementation"
                .into(),
        ))
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

    struct UnpagedTimelineStore;

    impl TimelineProjectionStore for UnpagedTimelineStore {
        fn upsert_timeline_entry(
            &self,
            _tenant_id: &str,
            _timeline_scope: &str,
            _message_seq: u64,
            _payload: &str,
        ) -> Result<(), ContractError> {
            Ok(())
        }

        fn load_timeline(
            &self,
            _tenant_id: &str,
            _timeline_scope: &str,
        ) -> Result<Vec<(u64, String)>, ContractError> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn window_default_fails_closed() {
        assert!(matches!(
            UnpagedTimelineStore.load_timeline_window("tenant", "conversation", 0, 20),
            Err(ContractError::UnsupportedCapability(message)) if message.contains("load_timeline_window")
        ));
    }
}
