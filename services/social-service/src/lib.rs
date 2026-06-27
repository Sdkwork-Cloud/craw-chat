pub mod block;
pub mod direct_chat;
pub mod external;
pub mod friendship;
pub mod shared_channel;
mod control_routes;
mod http;
mod openapi;
mod openapi_contacts;
pub mod postgres;
mod runtime;
mod runtime_control;
mod shared_channel_sync_metrics;
mod shared_channel_sync_runtime;
mod shared_channel_sync_scheduler;

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
}
pub use runtime::SocialRuntime;
pub use shared_channel_sync_metrics::{
    render_shared_channel_sync_prometheus_from_env, shared_channel_sync_metrics,
    SharedChannelSyncMetrics,
};
pub use shared_channel_sync_scheduler::{
    spawn_shared_channel_sync_stale_reclaim_scheduler,
    spawn_shared_channel_sync_stale_reclaim_scheduler_from_env,
    SharedChannelSyncStaleReclaimSchedulerConfig,
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
