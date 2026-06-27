use serde::{Deserialize, Serialize};

use crate::models::{ConversationInboxPeerView, ConversationInboxPreferencesView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationInboxEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "agentHandoff")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_handoff: Option<bool>,

    #[serde(rename = "conversationType")]
    pub conversation_type: String,

    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "avatarUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "displaySource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_source: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer: Option<ConversationInboxPeerView>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<ConversationInboxPreferencesView>,

    #[serde(rename = "lastActivityAt")]
    pub last_activity_at: String,

    #[serde(rename = "lastMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,

    #[serde(rename = "lastSenderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sender_id: Option<String>,

    #[serde(rename = "messageCount")]
    pub message_count: i64,

    #[serde(rename = "lastMessageSeq")]
    pub last_message_seq: i64,

    #[serde(rename = "lastSummary")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_summary: Option<String>,

    #[serde(rename = "lastMessageAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,

    #[serde(rename = "unreadCount")]
    pub unread_count: i64,
}
