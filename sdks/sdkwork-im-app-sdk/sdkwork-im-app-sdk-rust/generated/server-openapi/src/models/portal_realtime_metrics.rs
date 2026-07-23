use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalRealtimeMetrics {
    #[serde(rename = "clientRouteWindowCount")]
    pub client_route_window_count: String,

    #[serde(rename = "pendingEventCount")]
    pub pending_event_count: String,

    #[serde(rename = "maxClientRouteWindowEventCount")]
    pub max_client_route_window_event_count: String,

    #[serde(rename = "clientRouteWindowCapacity")]
    pub client_route_window_capacity: String,

    #[serde(rename = "maxClientRouteWindowUsagePermille")]
    pub max_client_route_window_usage_permille: i64,

    #[serde(rename = "capacityTrimmedEventCount")]
    pub capacity_trimmed_event_count: String,

    #[serde(rename = "oldestPendingOccurredAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_pending_occurred_at: Option<String>,
}
