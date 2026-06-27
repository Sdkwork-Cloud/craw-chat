use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct QuotaProfileResponse {
    #[serde(rename = "maxConcurrentSessionsPerTenant")]
    pub max_concurrent_sessions_per_tenant: i64,

    #[serde(rename = "maxInflightMessages")]
    pub max_inflight_messages: i64,

    #[serde(rename = "maxPayloadBytes")]
    pub max_payload_bytes: i64,

    #[serde(rename = "maxSubscriptionsPerSession")]
    pub max_subscriptions_per_session: i64,

    #[serde(rename = "profileId")]
    pub profile_id: String,
}
