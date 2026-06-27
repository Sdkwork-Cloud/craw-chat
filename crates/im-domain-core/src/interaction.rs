//! Interaction domain models for reactions, pins, threads, and conversation settings.
//!
//! This module defines the core domain types for IM message interactions:
//! - `MessageReaction`: A reaction to a message
//! - `MessagePin`: A pinned message
//! - `Thread`: A thread/discussion on a message
//! - `ThreadSubscription`: User subscription to a thread
//! - `ConversationSettings`: User settings for a conversation

#![allow(clippy::should_implement_trait)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Message Reaction
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReaction {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub user_id: String,
    pub reaction_type: String,
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Message Pin
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePin {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub pinned_by_user_id: String,
    pub pin_reason: Option<String>,
    pub pinned_at: String,
}

// ---------------------------------------------------------------------------
// Thread
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub tenant_id: String,
    pub organization_id: String,
    pub thread_id: String,
    pub conversation_id: String,
    pub root_message_id: String,
    pub thread_title: Option<String>,
    pub reply_count: i32,
    pub last_reply_at: Option<String>,
    pub last_reply_user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Thread Subscription
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationLevel {
    All,
    Mentions,
    None,
}

impl NotificationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Mentions => "mentions",
            Self::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(Self::All),
            "mentions" => Some(Self::Mentions),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSubscription {
    pub tenant_id: String,
    pub organization_id: String,
    pub thread_id: String,
    pub user_id: String,
    pub last_read_seq: i64,
    pub notification_level: NotificationLevel,
    pub subscribed_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Conversation Settings
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSettings {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub user_id: String,
    pub is_muted: bool,
    pub mute_until: Option<String>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_blocked: bool,
    pub notification_level: NotificationLevel,
    pub custom_name: Option<String>,
    pub settings_json: String,
    pub updated_at: String,
}
