mod cluster_bus;
mod conversation_aggregate_store;
mod id_generator;
mod message_store;
mod outbox_store;
mod provider;
mod push_provider;
mod retention_scope_store;
mod search_provider;
mod seq_allocator;

pub use provider::*;
pub use sdkwork_im_contract_admin::{AdminCapabilityProfileRecord, AdminCapabilityProfileStore};
pub use sdkwork_im_contract_agent::{
    AgentSubject, AgentSubjectRecord, AgentSubjectStore, AutomationExecutionRecord,
    AutomationExecutionStore,
};
pub use sdkwork_im_contract_control::{
    normalize_realtime_organization_id, realtime_client_route_scope_key,
    realtime_principal_scope_key, realtime_scope_key_parts,
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowHighRiskRecord,
    RealtimeEventWindowRecord, RealtimeEventWindowStore, RealtimeMatchingSubscriptionQuery,
    RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
pub use sdkwork_im_contract_core::{
    ContractError, LeaseGrant, LeaseStore, MetadataSnapshotRecord, MetadataStore, ObjectDescriptor,
    ObjectPutRequest, ObjectStore,
};
pub use sdkwork_im_contract_message::{
    CommitEnvelope, CommitJournal, CommitPosition, TimelineProjectionBatch,
    TimelineProjectionRecord, TimelineProjectionStore,
};
pub use sdkwork_im_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
pub use sdkwork_im_contract_stream::{StreamStateRecord, StreamStateStore};

pub use cluster_bus::ClusterEventBus;
pub use push_provider::{PushDeliveryResult, PushMessage, PushProvider};
pub use retention_scope_store::RetentionScopeStore;
pub use search_provider::{SearchProvider, SearchResult, SearchableMessage};
pub use seq_allocator::ConversationSeqAllocator;

// 新增：消息真值存储契约
pub use conversation_aggregate_store::{
    ConversationAggregateState, ConversationAggregateStore, ConversationMemberRecord,
    ReadCursorRecord,
};
pub use id_generator::{IdGenerator, IdGeneratorConfig};
pub use message_store::{MessageStore, MessageWindow, StoredMessageRecord};
pub use outbox_store::{OutboxEventRecord, OutboxPublishStatus, OutboxStore};

pub use sdkwork_communication_rtc_service::{
    RtcContractError, RtcCreateMediaSessionRequest, RtcMediaSessionMode, RtcParticipantCredential,
    RtcProviderEventKind, RtcProviderPort, RtcProviderWebhookEvent, RtcProviderWebhookParseRequest,
    RtcRecordingArtifact, RtcSessionHandle, rtc_provider_payload_hash,
};
