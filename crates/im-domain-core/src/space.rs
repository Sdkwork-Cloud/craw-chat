//! Space domain models for spaces, groups, channels, members, invitations, and bans.
//!
//! This module defines the core domain types for IM space management:
//! - `Space`: A top-level organizational container (like Discord server, Slack workspace)
//! - `SpaceMember`: User membership in a space
//! - `ChatGroup`: A group chat within a space
//! - `GroupMember`: User membership in a group
//! - `ChatChannel`: A channel within a space
//! - `ChannelAccessRule`: Permission rules for channel access
//! - `Invitation`: An invitation to join a space/group/channel
//! - `BanRecord`: A ban record for a user

#![allow(clippy::should_implement_trait)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Space
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpaceType {
    Organization,
    Team,
    Project,
    Community,
}

impl SpaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Organization => "organization",
            Self::Team => "team",
            Self::Project => "project",
            Self::Community => "community",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "organization" => Some(Self::Organization),
            "team" => Some(Self::Team),
            "project" => Some(Self::Project),
            "community" => Some(Self::Community),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub tenant_id: String,
    pub organization_id: String,
    pub space_id: String,
    pub space_name: String,
    pub space_type: SpaceType,
    pub owner_user_id: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: i32,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Space Member
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpaceMemberRole {
    Owner,
    Admin,
    Member,
    Guest,
}

impl SpaceMemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Admin => "admin",
            Self::Member => "member",
            Self::Guest => "guest",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(Self::Owner),
            "admin" => Some(Self::Admin),
            "member" => Some(Self::Member),
            "guest" => Some(Self::Guest),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMember {
    pub tenant_id: String,
    pub organization_id: String,
    pub space_id: String,
    pub user_id: String,
    pub role: SpaceMemberRole,
    pub nickname: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Chat Group
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupType {
    Normal,
    Announcement,
    Project,
    Department,
}

impl GroupType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Announcement => "announcement",
            Self::Project => "project",
            Self::Department => "department",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Self::Normal),
            "announcement" => Some(Self::Announcement),
            "project" => Some(Self::Project),
            "department" => Some(Self::Department),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatGroup {
    pub tenant_id: String,
    pub organization_id: String,
    pub group_id: String,
    pub space_id: Option<String>,
    pub group_name: String,
    pub group_type: GroupType,
    pub owner_user_id: String,
    pub conversation_id: Option<String>,
    pub max_members: i32,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub announcement: Option<String>,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Group Member
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupMemberRole {
    Owner,
    Admin,
    Member,
    Muted,
}

impl GroupMemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Admin => "admin",
            Self::Member => "member",
            Self::Muted => "muted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(Self::Owner),
            "admin" => Some(Self::Admin),
            "member" => Some(Self::Member),
            "muted" => Some(Self::Muted),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub tenant_id: String,
    pub organization_id: String,
    pub group_id: String,
    pub user_id: String,
    pub role: GroupMemberRole,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Chat Channel
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Text,
    Voice,
    Announcement,
    Forum,
}

impl ChannelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Voice => "voice",
            Self::Announcement => "announcement",
            Self::Forum => "forum",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "text" => Some(Self::Text),
            "voice" => Some(Self::Voice),
            "announcement" => Some(Self::Announcement),
            "forum" => Some(Self::Forum),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatChannel {
    pub tenant_id: String,
    pub organization_id: String,
    pub channel_id: String,
    pub space_id: String,
    pub channel_name: String,
    pub channel_type: ChannelType,
    pub description: Option<String>,
    pub conversation_id: Option<String>,
    pub position: i32,
    pub is_nsfw: bool,
    pub is_pinned: bool,
    pub topic: Option<String>,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Channel Access Rule
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleType {
    Allow,
    Deny,
}

impl AccessRuleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Self::Allow),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAccessRule {
    pub tenant_id: String,
    pub organization_id: String,
    pub rule_id: String,
    pub channel_id: String,
    pub rule_type: AccessRuleType,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Invitation
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvitationTargetType {
    Space,
    Group,
    Channel,
}

impl InvitationTargetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Space => "space",
            Self::Group => "group",
            Self::Channel => "channel",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "space" => Some(Self::Space),
            "group" => Some(Self::Group),
            "channel" => Some(Self::Channel),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Canceled,
}

impl InvitationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Declined => "declined",
            Self::Expired => "expired",
            Self::Canceled => "canceled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "accepted" => Some(Self::Accepted),
            "declined" => Some(Self::Declined),
            "expired" => Some(Self::Expired),
            "canceled" => Some(Self::Canceled),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub tenant_id: String,
    pub organization_id: String,
    pub invitation_id: String,
    pub inviter_user_id: String,
    pub invitee_user_id: Option<String>,
    pub invitee_email: Option<String>,
    pub invitee_phone: Option<String>,
    pub target_type: InvitationTargetType,
    pub target_id: String,
    pub role: String,
    pub status: InvitationStatus,
    pub message: Option<String>,
    pub expires_at: Option<String>,
    pub accepted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Ban Record
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub ban_id: String,
    pub target_type: String,
    pub target_id: String,
    pub banned_user_id: String,
    pub banned_by_user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub unbanned_at: Option<String>,
    pub unbanned_by_user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
