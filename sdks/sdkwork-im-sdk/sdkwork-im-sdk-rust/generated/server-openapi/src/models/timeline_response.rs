use serde::{Deserialize, Serialize};

use crate::models::{TimelineViewEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TimelineResponse {
    pub items: Vec<TimelineViewEntry>,

    #[serde(rename = "nextAfterSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_after_seq: Option<i64>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
