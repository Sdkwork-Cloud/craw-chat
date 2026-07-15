use serde::{Deserialize, Serialize};

/// Explicit command input. The group target and archive actor are derived from the authenticated request context and path and cannot be supplied by the caller.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ArchiveGroupConversationRequest {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
