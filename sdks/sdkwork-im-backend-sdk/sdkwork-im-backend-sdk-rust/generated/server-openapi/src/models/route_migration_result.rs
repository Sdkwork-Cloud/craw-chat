use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RouteMigrationResult {
    #[serde(rename = "migratedRouteCount")]
    pub migrated_route_count: i64,

    #[serde(rename = "sourceDrainStatus")]
    pub source_drain_status: String,

    #[serde(rename = "sourceNodeId")]
    pub source_node_id: String,

    #[serde(rename = "sourceRebalanceState")]
    pub source_rebalance_state: String,

    #[serde(rename = "targetDrainStatus")]
    pub target_drain_status: String,

    #[serde(rename = "targetNodeId")]
    pub target_node_id: String,

    #[serde(rename = "targetRebalanceState")]
    pub target_rebalance_state: String,
}
