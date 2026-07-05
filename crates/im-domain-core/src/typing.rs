//! Typing indicator domain types.
//!
//! Typing indicators are ephemeral signals: a member is currently composing a
//! message in a conversation. They are NOT persisted to the durable journal
//! and are NOT replayed on reconnect. The typical lifetime is 5 seconds,
//! refreshed on each keystroke by the client.
//!
//! Wire format (`conversation.typing` realtime event payload):
//! ```json
//! {
//!   "conversationId": "c_xxx",
//!   "userId": "1108",
//!   "userKind": "user",
//!   "occurredAt": "2026-07-04T01:23:45.678Z"
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Realtime event type tag for typing indicators.
pub const TYPING_EVENT_TYPE: &str = "conversation.typing";

/// Realtime scope type for conversation-scoped events.
pub const TYPING_SCOPE_TYPE: &str = "conversation";

/// Default typing indicator TTL in seconds.
pub const TYPING_DEFAULT_TTL_SECONDS: u64 = 5;

/// Ephemeral payload carried by a `conversation.typing` realtime event.
///
/// Intentionally minimal: clients only need to know who is typing and where.
/// The `occurredAt` timestamp lets clients discard stale indicators if a
/// push is delayed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypingIndicator {
    pub conversation_id: String,
    pub user_id: String,
    pub user_kind: String,
    pub occurred_at: String,
}

impl TypingIndicator {
    pub fn new(
        conversation_id: impl Into<String>,
        user_id: impl Into<String>,
        user_kind: impl Into<String>,
        occurred_at: impl Into<String>,
    ) -> Self {
        Self {
            conversation_id: conversation_id.into(),
            user_id: user_id.into(),
            user_kind: user_kind.into(),
            occurred_at: occurred_at.into(),
        }
    }

    /// Serialize to a JSON string for the realtime event `payload` field.
    pub fn to_payload_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Response payload for `GET /conversations/{conversationId}/typing`.
///
/// Lists principals whose typing state is currently live (within TTL) in the
/// conversation. Excludes the querying client's own principal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypingIndicatorList {
    pub conversation_id: String,
    pub items: Vec<TypingIndicatorListItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypingIndicatorListItem {
    pub user_id: String,
    pub user_kind: String,
}
