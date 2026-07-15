use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConversationResult {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "requestKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_key: Option<String>,

    #[serde(rename = "deliveryStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,

    #[serde(rename = "proofVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,

    /// Present only when initializeKnowledgebase was true. A failed value means group creation succeeded but the optional remote Knowledgebase provisioning attempt did not complete; the group owner can retry from the Knowledgebase action.
    #[serde(rename = "knowledgebaseInitialization")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledgebase_initialization: Option<String>,
}
