//! Contact domain models for friend requests, friendships, blocks, and direct chats.
//!
//! This module defines the core domain types for IM contact management:
//! - `FriendRequest`: A friend request between users
//! - `Friendship`: An established friendship between users
//! - `UserBlock`: A block relationship between users
//! - `DirectChat`: A direct chat session between users
//! - `UserProfile`: User profile information
//! - `UserSettings`: User settings

#![allow(clippy::should_implement_trait)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Friend Request
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
    Expired,
}

impl FriendRequestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Declined => "declined",
            Self::Canceled => "canceled",
            Self::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "accepted" => Some(Self::Accepted),
            "declined" => Some(Self::Declined),
            "canceled" => Some(Self::Canceled),
            "expired" => Some(Self::Expired),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
    pub tenant_id: String,
    pub organization_id: String,
    pub request_id: String,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub request_message: Option<String>,
    pub status: FriendRequestStatus,
    pub expired_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Friendship
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendshipStatus {
    Active,
    Removed,
}

impl FriendshipStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Removed => "removed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "removed" => Some(Self::Removed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friendship {
    pub tenant_id: String,
    pub organization_id: String,
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub initiator_user_id: String,
    pub status: FriendshipStatus,
    pub established_at: Option<String>,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// User Block
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockScope {
    All,
    Friendship,
    DirectChat,
}

impl BlockScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Friendship => "friendship",
            Self::DirectChat => "direct_chat",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(Self::All),
            "friendship" => Some(Self::Friendship),
            "direct_chat" => Some(Self::DirectChat),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBlock {
    pub tenant_id: String,
    pub organization_id: String,
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: BlockScope,
    pub direct_chat_id: Option<String>,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Direct Chat
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectChatStatus {
    Active,
    Archived,
    Closed,
}

impl DirectChatStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
            Self::Closed => "closed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "archived" => Some(Self::Archived),
            "closed" => Some(Self::Closed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectChat {
    pub tenant_id: String,
    pub organization_id: String,
    pub direct_chat_id: String,
    pub left_actor_kind: String,
    pub left_actor_id: String,
    pub right_actor_kind: String,
    pub right_actor_id: String,
    pub pair_hash: String,
    pub status: DirectChatStatus,
    pub conversation_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// User Profile
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
    pub im_notification_prefs: String,
    pub im_mute_settings: String,
    pub im_privacy_settings: String,
    pub im_online_status: String,
    pub last_active_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// User Settings
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSetting {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
    pub setting_key: String,
    pub setting_value: String,
    pub updated_at: String,
}
