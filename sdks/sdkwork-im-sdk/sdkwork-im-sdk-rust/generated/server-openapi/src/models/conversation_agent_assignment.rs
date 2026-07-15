use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationAgentAssignment {
    #[serde(rename = "agentId")]
    pub agent_id: String,

    #[serde(rename = "revisionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}
