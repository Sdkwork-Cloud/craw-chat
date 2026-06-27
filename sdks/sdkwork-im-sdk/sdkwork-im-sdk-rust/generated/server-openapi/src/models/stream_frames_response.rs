use serde::{Deserialize, Serialize};

use crate::models::{StreamFrameView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamFramesResponse {
    pub items: Vec<StreamFrameView>,

    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
