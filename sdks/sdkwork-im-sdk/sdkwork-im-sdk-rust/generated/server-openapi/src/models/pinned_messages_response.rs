use serde::{Deserialize, Serialize};

use crate::models::{MessageInteractionSummaryView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PinnedMessagesResponse {
    pub items: Vec<MessageInteractionSummaryView>,
}
