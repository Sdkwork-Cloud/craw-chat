use serde::{Deserialize, Serialize};

use crate::models::{ConversationAgentAssignment};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationAgentAssignments {
    pub generation: i64,

    pub source: String,

    pub agents: Vec<ConversationAgentAssignment>,
}
