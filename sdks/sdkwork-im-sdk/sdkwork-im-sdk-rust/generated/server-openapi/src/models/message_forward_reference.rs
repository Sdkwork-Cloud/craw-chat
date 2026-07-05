use serde::{Deserialize, Serialize};

/// Source tracing metadata for a forwarded message. Carries attribution to the original message across conversations so the UI can render a "Forwarded from <sender>" label and preserve audit provenance. The forwarder remains the Sender of the new message; this object only records where the content originated. Cross-conversation recall visibility is intentionally NOT cascaded — recipients of a forward see the original snapshot at forward-time.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageForwardReference {
    /// Original message identifier in the source conversation.
    #[serde(rename = "originalMessageId")]
    pub original_message_id: String,

    /// Original conversation identifier (may differ from the forward target).
    #[serde(rename = "originalConversationId")]
    pub original_conversation_id: String,

    /// Original sender principal id (preserves author attribution).
    #[serde(rename = "originalSenderId")]
    pub original_sender_id: String,

    /// Original sender principal kind (e.g. `user`, `app`).
    #[serde(rename = "originalSenderKind")]
    pub original_sender_kind: String,

    /// Original sender display name at forward time (snapshot).
    #[serde(rename = "originalSenderDisplayName")]
    pub original_sender_display_name: String,

    /// RFC 3339 timestamp of the original message occurrence.
    #[serde(rename = "originalOccurredAt")]
    pub original_occurred_at: String,

    /// RFC 3339 timestamp when the forward action was performed.
    #[serde(rename = "forwardedAt")]
    pub forwarded_at: String,

    /// Snapshot of the original content preview at forward time.
    #[serde(rename = "contentPreview")]
    pub content_preview: String,

    /// Number of times this message has been forwarded along the chain (1 for the first forward, incrementing on each subsequent forward).
    #[serde(rename = "forwardCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_count: Option<i64>,
}
