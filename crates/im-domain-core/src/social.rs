use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocialInvariantError {
    EmptyId {
        field: &'static str,
    },
    IdenticalPair {
        left_field: &'static str,
        right_field: &'static str,
        value: String,
    },
    PairHashMismatch {
        expected: String,
        actual: String,
    },
}

impl Display for SocialInvariantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyId { field } => write!(f, "{field} cannot be empty"),
            Self::IdenticalPair {
                left_field,
                right_field,
                value,
            } => write!(
                f,
                "{left_field} and {right_field} must be different, got {value}"
            ),
            Self::PairHashMismatch { expected, actual } => {
                write!(f, "pair_hash mismatch: expected {expected}, got {actual}")
            }
        }
    }
}

impl Error for SocialInvariantError {}

fn ensure_non_empty(value: &str, field: &'static str) -> Result<(), SocialInvariantError> {
    if value.trim().is_empty() {
        return Err(SocialInvariantError::EmptyId { field });
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedUserPair {
    pub user_low_id: String,
    pub user_high_id: String,
}

impl NormalizedUserPair {
    pub fn try_new(user_a: &str, user_b: &str) -> Result<Self, SocialInvariantError> {
        ensure_non_empty(user_a, "user_a")?;
        ensure_non_empty(user_b, "user_b")?;

        if user_a == user_b {
            return Err(SocialInvariantError::IdenticalPair {
                left_field: "user_a",
                right_field: "user_b",
                value: user_a.into(),
            });
        }

        let (user_low_id, user_high_id) = if user_a <= user_b {
            (user_a, user_b)
        } else {
            (user_b, user_a)
        };

        Ok(Self {
            user_low_id: user_low_id.into(),
            user_high_id: user_high_id.into(),
        })
    }

    pub fn pair_key(&self) -> String {
        format!("{}:{}", self.user_low_id, self.user_high_id)
    }
}

pub fn normalize_user_pair(
    user_a: &str,
    user_b: &str,
) -> Result<NormalizedUserPair, SocialInvariantError> {
    NormalizedUserPair::try_new(user_a, user_b)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedActorPair {
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub pair_hash: String,
}

impl NormalizedActorPair {
    pub fn try_new(actor_a: &str, actor_b: &str) -> Result<Self, SocialInvariantError> {
        ensure_non_empty(actor_a, "actor_a")?;
        ensure_non_empty(actor_b, "actor_b")?;

        if actor_a == actor_b {
            return Err(SocialInvariantError::IdenticalPair {
                left_field: "actor_a",
                right_field: "actor_b",
                value: actor_a.into(),
            });
        }

        let (left_actor_id, right_actor_id) = if actor_a <= actor_b {
            (actor_a, actor_b)
        } else {
            (actor_b, actor_a)
        };
        let pair_hash = format!("{left_actor_id}:{right_actor_id}");

        Ok(Self {
            left_actor_id: left_actor_id.into(),
            right_actor_id: right_actor_id.into(),
            pair_hash,
        })
    }
}

pub fn normalize_actor_pair(
    actor_a: &str,
    actor_b: &str,
) -> Result<NormalizedActorPair, SocialInvariantError> {
    NormalizedActorPair::try_new(actor_a, actor_b)
}

pub fn direct_chat_pair_hash(actor_a: &str, actor_b: &str) -> Result<String, SocialInvariantError> {
    Ok(NormalizedActorPair::try_new(actor_a, actor_b)?.pair_hash)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
    Expired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
    pub tenant_id: String,
    pub request_id: String,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub status: FriendRequestStatus,
    pub request_message: Option<String>,
    pub expired_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl FriendRequest {
    pub fn user_pair(&self) -> Result<NormalizedUserPair, SocialInvariantError> {
        NormalizedUserPair::try_new(
            self.requester_user_id.as_str(),
            self.target_user_id.as_str(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendshipStatus {
    Active,
    Removed,
}

impl FriendshipStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friendship {
    pub tenant_id: String,
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub initiator_user_id: String,
    pub status: FriendshipStatus,
    pub established_at: Option<String>,
    pub updated_at: String,
}

impl Friendship {
    pub fn pair(&self) -> Result<NormalizedUserPair, SocialInvariantError> {
        NormalizedUserPair::try_new(self.user_low_id.as_str(), self.user_high_id.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendshipEventType {
    Requested,
    Accepted,
    Declined,
    Canceled,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipEvent {
    pub tenant_id: String,
    pub event_id: String,
    pub friendship_id: String,
    pub event_type: FriendshipEventType,
    pub operator_user_id: Option<String>,
    pub reason: Option<String>,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockScope {
    All,
    Friendship,
    DirectChat,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserBlockStatus {
    Active,
    Released,
    Expired,
}

impl UserBlockStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBlock {
    pub tenant_id: String,
    pub block_id: String,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: BlockScope,
    pub status: UserBlockStatus,
    pub direct_chat_id: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl UserBlock {
    pub fn user_pair(&self) -> Result<NormalizedUserPair, SocialInvariantError> {
        NormalizedUserPair::try_new(self.blocker_user_id.as_str(), self.blocked_user_id.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectChatStatus {
    Active,
    Archived,
    Closed,
}

impl DirectChatStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectChat {
    pub tenant_id: String,
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub right_actor_id: String,
    pub pair_hash: String,
    pub status: DirectChatStatus,
    pub conversation_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl DirectChat {
    pub fn actor_pair(&self) -> Result<NormalizedActorPair, SocialInvariantError> {
        let pair = NormalizedActorPair::try_new(
            self.left_actor_id.as_str(),
            self.right_actor_id.as_str(),
        )?;
        if pair.pair_hash != self.pair_hash {
            return Err(SocialInvariantError::PairHashMismatch {
                expected: pair.pair_hash,
                actual: self.pair_hash.clone(),
            });
        }

        Ok(pair)
    }

    pub fn has_conversation_binding(&self) -> bool {
        self.conversation_id
            .as_ref()
            .is_some_and(|conversation_id| !conversation_id.trim().is_empty())
    }
}

pub fn ensure_cross_tenant_connection(
    tenant_id: &str,
    external_tenant_id: &str,
) -> Result<(), SocialInvariantError> {
    ensure_non_empty(tenant_id, "tenant_id")?;
    ensure_non_empty(external_tenant_id, "external_tenant_id")?;

    if tenant_id == external_tenant_id {
        return Err(SocialInvariantError::IdenticalPair {
            left_field: "tenant_id",
            right_field: "external_tenant_id",
            value: tenant_id.into(),
        });
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalConnectionKind {
    SharedChannel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalConnectionStatus {
    Active,
    Suspended,
    Revoked,
}

impl ExternalConnectionStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalConnection {
    pub tenant_id: String,
    pub connection_id: String,
    pub external_tenant_id: String,
    pub external_org_name: Option<String>,
    pub connection_kind: ExternalConnectionKind,
    pub status: ExternalConnectionStatus,
    pub established_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalMemberLinkStatus {
    Active,
    Revoked,
}

impl ExternalMemberLinkStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMemberLink {
    pub tenant_id: String,
    pub link_id: String,
    pub connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
    pub external_display_name: Option<String>,
    pub status: ExternalMemberLinkStatus,
    pub linked_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedChannelPolicyStatus {
    Active,
    Suspended,
}

impl SharedChannelPolicyStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelPolicy {
    pub tenant_id: String,
    pub policy_id: String,
    pub connection_id: String,
    pub channel_id: String,
    pub conversation_id: Option<String>,
    pub policy_version: u64,
    pub history_visibility: String,
    pub status: SharedChannelPolicyStatus,
    pub applied_at: String,
    pub updated_at: String,
}
