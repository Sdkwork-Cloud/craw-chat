use im_domain_core::conversation::{
    ConversationAgentHandoffView, ConversationInboxEntry, ConversationMember, MembershipRole,
    MembershipState,
};
use im_domain_core::message::{MessageBody, MessageType, Sender};
use im_domain_core::social::DirectChatStatus;
use serde::{Deserialize, Serialize};

use sdkwork_utils_rust::SdkWorkPageData;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineViewEntry {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub summary: Option<String>,
    #[serde(default = "default_sender")]
    pub sender: Sender,
    #[serde(default = "default_message_body")]
    pub body: MessageBody,
    #[serde(default = "default_message_type")]
    pub message_type: MessageType,
    #[serde(default = "default_delivery_mode")]
    pub delivery_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtc_session_id: Option<String>,
    #[serde(default)]
    pub occurred_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_until: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction_counts: Vec<MessageReactionCountView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<MessagePinView>,
}

pub type TimelineWindowView = SdkWorkPageData<TimelineViewEntry>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarySenderView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionCountView {
    pub reaction_key: String,
    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionActorView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinView {
    pub pinned_by: InteractionActorView,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReadReceiptReaderView {
    pub principal_id: String,
    pub principal_kind: String,
    pub member_id: String,
    pub read_seq: u64,
    pub updated_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReadReceiptSummaryView {
    pub active_member_count: u64,
    pub read_count: u64,
    pub readers: Vec<MessageReadReceiptReaderView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeliveryReceiptDeviceView {
    pub principal_id: String,
    pub principal_kind: String,
    pub member_id: String,
    pub device_id: String,
    pub sync_seq: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeliveryReceiptSummaryView {
    pub active_member_count: u64,
    pub offered_count: u64,
    pub delivered_count: u64,
    pub delivered_devices: Vec<MessageDeliveryReceiptDeviceView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInteractionSummaryView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub total_reaction_count: u64,
    pub reaction_counts: Vec<MessageReactionCountView>,
    pub pin: Option<MessagePinView>,
    #[serde(default)]
    pub read_receipt: MessageReadReceiptSummaryView,
    #[serde(default)]
    pub delivery_receipt: MessageDeliveryReceiptSummaryView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationMemberDirectoryEntry {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub role: MembershipRole,
    pub state: MembershipState,
    pub invited_by: Option<String>,
    pub joined_at: String,
    pub removed_at: Option<String>,
    pub attributes: std::collections::BTreeMap<String, String>,
}

impl ConversationMemberDirectoryEntry {
    pub fn from_member(member: &ConversationMember) -> Self {
        Self {
            tenant_id: member.tenant_id.clone(),
            conversation_id: member.conversation_id.clone(),
            member_id: member.member_id.clone(),
            principal_id: member.principal_id.clone(),
            principal_kind: member.principal_kind.clone(),
            role: member.role.clone(),
            state: member.state.clone(),
            invited_by: member.invited_by.clone(),
            joined_at: member.joined_at.clone(),
            removed_at: member.removed_at.clone(),
            attributes: member.attributes.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSummaryView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_count: u64,
    pub last_message_id: Option<String>,
    pub last_message_seq: u64,
    pub last_sender_id: Option<String>,
    pub last_sender_kind: Option<String>,
    pub last_sender: Option<SummarySenderView>,
    pub last_summary: Option<String>,
    pub last_message_at: Option<String>,
    pub agent_handoff: Option<ConversationAgentHandoffView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredClientRouteView {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
    pub registered_at: String,
}

fn default_organization_id() -> String {
    "0".to_owned()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRouteSyncFeedWindowView {
    pub items: Vec<im_domain_core::conversation::ClientRouteSyncFeedEntry>,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
    pub trimmed_through_seq: u64,
}

pub type InboxWindowView = SdkWorkPageData<ConversationInboxEntry>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum InboxListCursor {
    Start,
    Offset(usize),
    Keyset { activity_at: String, scope: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InboxKeysetCursorWire {
    pub activity_at: String,
    pub scope: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeFanoutTarget {
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRecipientView {
    pub principal_id: String,
    pub principal_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactView {
    pub tenant_id: String,
    #[serde(default = "default_contact_organization_id")]
    pub organization_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub contact_type: String,
    pub relationship_state: String,
    pub friendship_id: String,
    pub direct_chat_id: Option<String>,
    pub conversation_id: Option<String>,
    pub established_at: String,
    pub last_interaction_at: String,
}

pub type ContactWindowView = SdkWorkPageData<ContactView>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ContactListCursor {
    Start,
    Offset(usize),
    Keyset {
        last_interaction_at: String,
        target_user_id: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContactKeysetCursorWire {
    pub last_interaction_at: String,
    pub target_user_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MemberDirectoryListCursor {
    Start,
    Offset(usize),
    Keyset {
        role_rank: u8,
        joined_at: String,
        principal_id: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MemberDirectoryKeysetCursorWire {
    pub role_rank: u8,
    pub joined_at: String,
    pub principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PinnedMessagesListCursor {
    Start,
    Offset(usize),
    Keyset {
        pinned_at: String,
        message_seq: u64,
        message_id: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PinnedMessagesKeysetCursorWire {
    pub pinned_at: String,
    pub message_seq: u64,
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FavoriteMessagesListCursor {
    Start,
    Offset(usize),
    Keyset {
        favorited_at: String,
        favorite_id: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FavoriteMessagesKeysetCursorWire {
    pub favorited_at: String,
    pub favorite_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactDirectChatBindingView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) organization_id: Option<String>,
    pub(super) direct_chat_id: String,
    pub(super) conversation_id: String,
    pub(super) bound_at: String,
    #[serde(default = "default_direct_chat_status")]
    pub(super) status: DirectChatStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) updated_at: Option<String>,
}

fn default_direct_chat_status() -> DirectChatStatus {
    DirectChatStatus::Active
}

fn default_contact_organization_id() -> String {
    "default".to_owned()
}

fn default_sender() -> Sender {
    Sender {
        id: "system".into(),
        kind: "system".into(),
        member_id: None,
        device_id: None,
        session_id: None,
        metadata: Default::default(),
    }
}

fn default_message_body() -> MessageBody {
    MessageBody {
        summary: None,
        parts: Vec::new(),
        render_hints: Default::default(),
        reply_to: None,
    }
}

fn default_message_type() -> MessageType {
    MessageType::Standard
}

fn default_delivery_mode() -> String {
    "discrete".into()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ConversationCatalogEntry {
    pub(super) conversation_type: String,
    pub(super) created_at: String,
    #[serde(default = "default_history_visibility")]
    pub(super) history_visibility: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) title: Option<String>,
}

fn default_history_visibility() -> String {
    "joined".into()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationProfileView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub display_name: String,
    pub avatar_url: String,
    pub notice: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by_principal_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by_principal_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationProfileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationPreferencesView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub is_pinned: bool,
    pub is_muted: bool,
    pub is_marked_unread: bool,
    pub is_hidden: bool,
    pub updated_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationPreferencesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_marked_unread: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageFavoriteView {
    pub tenant_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub favorite_id: String,
    pub favorite_type: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: i32,
    pub title: String,
    pub content_preview: String,
    pub source_display_name: String,
    pub favorited_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteMessageRequest {
    pub conversation_id: String,
    pub favorite_type: String,
    pub title: String,
    pub content_preview: String,
    pub source_display_name: String,
}

pub type FavoriteMessagesWindowView = SdkWorkPageData<MessageFavoriteView>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageSearchHitView {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
}

pub type MessageSearchWindowView = SdkWorkPageData<MessageSearchHitView>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMessageFavoriteResponse {
    pub favorite_id: String,
    pub deleted: bool,
}

/// Per-principal message visibility snapshot produced by the projection layer.
///
/// `is_deleted = true` indicates the principal has soft-deleted (hidden) the
/// message from their own view; the underlying message record and other
/// principals' visibility are unaffected. The HTTP delete route returns
/// `204 No Content`; this type remains an internal projection/RPC snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageVisibilityMutationResult {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: i32,
    pub principal_kind: String,
    pub principal_id: String,
    pub is_deleted: bool,
    pub updated_at: String,
}
