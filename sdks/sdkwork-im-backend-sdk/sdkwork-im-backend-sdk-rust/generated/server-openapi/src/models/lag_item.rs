use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LagItem {
    pub component: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "currentOffset")]
    pub current_offset: String,

    #[serde(rename = "committedOffset")]
    pub committed_offset: String,

    pub lag: String,
}
