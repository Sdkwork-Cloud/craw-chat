use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct QuotaProfileResponse {
    #[serde(rename = "maxConcurrentSessionsPerTenant")]
    pub max_concurrent_sessions_per_tenant: String,

    #[serde(rename = "maxInflightMessages")]
    pub max_inflight_messages: String,

    #[serde(rename = "maxPayloadBytes")]
    pub max_payload_bytes: String,

    #[serde(rename = "maxSubscriptionsPerSession")]
    pub max_subscriptions_per_session: String,

    #[serde(rename = "profileId")]
    pub profile_id: String,
}
