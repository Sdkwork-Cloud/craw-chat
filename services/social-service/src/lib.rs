mod api_payload;
pub mod block;
pub mod commit_materializer;
mod contact_open_api_backend;
mod control_access;
mod control_routes;
pub mod direct_chat;
pub mod direct_chat_binder;
mod envelope;
pub mod external;
pub mod friend_request_expiration;
mod friend_request_rate_limit;
pub mod friendship;
mod http;
pub mod journal_bootstrap;
mod openapi;
mod openapi_contacts;
pub mod postgres;
mod postgres_write_authority;
pub mod projection_bridge;
mod runtime;
mod runtime_control;
pub mod shared_channel;
mod shared_channel_sync_metrics;
mod shared_channel_sync_runtime;
mod shared_channel_sync_scheduler;
mod social_materializer_metrics;
pub mod social_realtime;
mod user_directory;

use serde::{Deserialize, Serialize};

pub use control_routes::{build_control_domain_api_router, build_control_public_router};
pub use http::build_app;
pub use openapi::build_open_api_router;
pub use openapi::init_open_api_id_generator;
pub use openapi_contacts::init_contact_open_api_id_generator;
pub use postgres::{
    PostgresAppState, app_state_from_postgres_pool, try_postgres_app_state_from_database_url_env,
};

/// Initialize all social-service ID generators from the database.
///
/// Must be called during async service startup before any request is served.
/// This ensures the open-api and contact open-api handlers use database-backed
/// node_id allocation instead of falling back to node 0.
pub async fn init_id_generators() {
    init_open_api_id_generator().await;
    init_contact_open_api_id_generator().await;
    contact_open_api_backend::init_contact_postgres_store().await;
}
pub use commit_materializer::SocialPostgresMaterializer;
pub use direct_chat_binder::{BindDirectChatConversationInput, DirectChatConversationBinder};
pub use friend_request_expiration::spawn_friend_request_expiration_scheduler_from_env;
pub use journal_bootstrap::build_social_runtime_from_env;
pub use runtime::SocialRuntime;
pub use shared_channel_sync_metrics::{
    SharedChannelSyncMetrics, render_shared_channel_sync_prometheus_from_env,
    shared_channel_sync_metrics,
};
pub use shared_channel_sync_scheduler::{
    SharedChannelSyncStaleReclaimSchedulerConfig,
    spawn_shared_channel_sync_stale_reclaim_scheduler,
    spawn_shared_channel_sync_stale_reclaim_scheduler_from_env,
};
pub use social_materializer_metrics::{
    postgres_atomic_write_failure_count, postgres_materialization_failure_count,
    record_postgres_atomic_write_failures, record_postgres_materialization_failures,
    render_prometheus as render_social_materializer_prometheus,
};
pub use social_realtime::{
    LoggingSocialRealtimeFanout, SOCIAL_OUTBOX_AGGREGATE_TYPE, SocialRealtimeFanout,
    build_social_realtime_outbox_record, social_realtime_recipients_for_commit,
};

pub const SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD: u32 = 3;

pub trait SharedChannelLinkedMemberSyncTrigger: Send + Sync {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelLinkedMemberSyncRequest {
    pub tenant_id: String,
    pub conversation_id: String,
    pub shared_channel_policy_id: String,
    pub external_connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedChannelSyncDeliveryProofStatus {
    TransportAccepted,
    Applied,
    AlreadyLinked,
    Replayed,
    Failed,
}
