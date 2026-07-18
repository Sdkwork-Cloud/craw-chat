use im_domain_core::stream::{StreamFrame, StreamSession};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamScope {
    pub tenant_id: String,
    pub organization_id: String,
    pub stream_id: String,
}

impl StreamScope {
    pub fn new(
        tenant_id: impl Into<String>,
        organization_id: impl Into<String>,
        stream_id: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            stream_id: stream_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamSessionRecord {
    pub scope: StreamScope,
    pub session: StreamSession,
    pub version: u64,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamCreateOutcome {
    Applied(StreamSessionRecord),
    Existing(StreamSessionRecord),
    CapacityExceeded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamAppendOutcome {
    Applied {
        session: StreamSessionRecord,
        frame: StreamFrame,
    },
    Existing {
        session: StreamSessionRecord,
        frame: StreamFrame,
    },
    VersionConflict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamTransitionOutcome {
    Applied(StreamSessionRecord),
    VersionConflict,
}

pub trait StreamStateStore: Send + Sync {
    fn check_ready(&self) -> Result<(), ContractError>;

    fn load_session(
        &self,
        scope: &StreamScope,
    ) -> Result<Option<StreamSessionRecord>, ContractError>;

    fn create_session(
        &self,
        record: StreamSessionRecord,
        max_active_streams: u64,
    ) -> Result<StreamCreateOutcome, ContractError>;

    fn append_frame(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
        frame: StreamFrame,
    ) -> Result<StreamAppendOutcome, ContractError>;

    fn transition_session(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
    ) -> Result<StreamTransitionOutcome, ContractError>;

    fn list_frames_after(
        &self,
        scope: &StreamScope,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<Vec<StreamFrame>, ContractError>;

    fn clear_stream(&self, scope: &StreamScope) -> Result<bool, ContractError>;
}
