use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MigrateRoutesRequest {
    #[serde(rename = "targetNodeId")]
    pub target_node_id: String,
}
