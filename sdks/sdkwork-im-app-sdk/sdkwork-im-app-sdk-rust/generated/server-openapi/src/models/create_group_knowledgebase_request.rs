use serde::{Deserialize, Serialize};

/// Explicit command input. Group identity, tenancy, membership, and initial space metadata are authoritative IM state and are never client supplied.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateGroupKnowledgebaseRequest {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
