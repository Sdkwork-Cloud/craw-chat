use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RouteNodeLifecycle {
    #[serde(rename = "drainStatus")]
    pub drain_status: String,

    #[serde(rename = "nodeId")]
    pub node_id: String,

    #[serde(rename = "ownedRouteCount")]
    pub owned_route_count: i64,

    #[serde(rename = "rebalanceState")]
    pub rebalance_state: String,
}
