use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalOperationalMetrics {
    #[serde(rename = "clientRouteWindowCount")]
    pub client_route_window_count: String,

    #[serde(rename = "pendingRealtimeEventCount")]
    pub pending_realtime_event_count: String,
}
