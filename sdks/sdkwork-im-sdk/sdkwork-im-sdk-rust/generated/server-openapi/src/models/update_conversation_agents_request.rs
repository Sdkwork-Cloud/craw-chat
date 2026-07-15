use serde::{Deserialize, Serialize};

use crate::models::{ConversationAgentAssignment};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateConversationAgentsRequest {
    #[serde(rename = "expectedGeneration")]
    pub expected_generation: i64,

    #[serde(rename = "agentAssignments")]
    pub agent_assignments: Vec<ConversationAgentAssignment>,
}
