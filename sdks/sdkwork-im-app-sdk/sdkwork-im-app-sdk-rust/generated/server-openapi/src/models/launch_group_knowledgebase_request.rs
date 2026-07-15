use serde::{Deserialize, Serialize};

/// Explicit command input. The response ticket is scoped to the authenticated group member and is consumed once by sdkwork-knowledgebase.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LaunchGroupKnowledgebaseRequest {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
