use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalConversationOperationalMetrics {
    #[serde(rename = "laggingScopeCount")]
    pub lagging_scope_count: String,

    #[serde(rename = "maxOperationalLag")]
    pub max_operational_lag: String,

    #[serde(rename = "pendingOutboxEventCount")]
    pub pending_outbox_event_count: String,

    #[serde(rename = "failedOutboxAttemptCount")]
    pub failed_outbox_attempt_count: String,
}
