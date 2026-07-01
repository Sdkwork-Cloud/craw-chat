use im_domain_core::conversation::{
    ConversationAgentHandoffView, ConversationInboxEntry, ConversationMember, MembershipRole,
    MembershipState,
};
use im_domain_core::message::{MessageBody, MessageType, Sender};
use im_domain_core::social::DirectChatStatus;
use serde::{Deserialize, Serialize};

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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineWindowView {
    pub items: Vec<TimelineViewEntry>,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
}

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
pub struct MessageInteractionSummaryView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub total_reaction_count: u64,
    pub reaction_counts: Vec<MessageReactionCountView>,
    pub pin: Option<MessagePinView>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxWindowView {
    pub items: Vec<ConversationInboxEntry>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactWindowView {
    pub items: Vec<ContactView>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteMessagesWindowView {
    pub items: Vec<MessageFavoriteView>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMessageFavoriteResponse {
    pub favorite_id: String,
    pub deleted: bool,
}
