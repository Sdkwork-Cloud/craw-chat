use serde::{Deserialize, Serialize};

use crate::models::{InteractionActorView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessagePinView {
    #[serde(rename = "pinnedBy")]
    pub pinned_by: InteractionActorView,

    #[serde(rename = "pinnedAt")]
    pub pinned_at: String,
}
