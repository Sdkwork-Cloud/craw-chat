use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AgentSubject {
    pub agent_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    pub metadata: std::collections::HashMap<String, String>,
}
