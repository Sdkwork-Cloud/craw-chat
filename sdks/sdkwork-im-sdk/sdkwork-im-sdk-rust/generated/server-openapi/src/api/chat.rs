use std::sync::Arc;

use crate::api::paths::im_path;
use crate::api::paths::append_query_string;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AckResponse, AddConversationMemberRequest, BindDirectChatRequest, ChangeConversationMemberRoleRequest, ContactsResponse, ConversationMember, ConversationPreferencesView, ConversationProfileView, ConversationSummaryView, CreateAgentDialogRequest, CreateConversationRequest, CreateConversationResult, CreateRoomRequest, DeleteMessageFavoriteResponse, EditMessageRequest, EnterRoomResponse, FavoriteMessageRequest, FavoriteMessagesResponse, InboxResponse, ListMembersResponse, MemberDirectoryResponse, MessageFavoriteView, MessageInteractionSummaryView, MessagePinMutationResult, MessageReactionMutationResult, MessageReactionRequest, MessageVisibilityMutationResult, PinnedMessagesResponse, PostMessageRequest, PostedMessageResponse, ReadCursorView, RemoveConversationMemberRequest, RoomView, TimelineResponse, TransferConversationOwnerRequest, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest};

#[derive(Clone)]
pub struct ChatApi {
    client: Arc<SdkworkHttpClient>,
}

impl ChatApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// List IM contacts
    pub async fn contacts_list(&self, limit: Option<i64>, cursor: Option<&str>) -> Result<ContactsResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("limit", limit, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
        ]);
        let path = append_query_string(im_path(&"/chat/contacts".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// Retrieve current inbox window
    pub async fn inbox_retrieve(&self, limit: Option<i64>, cursor: Option<&str>) -> Result<InboxResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("limit", limit, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
        ]);
        let path = append_query_string(im_path(&"/chat/inbox".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// Create a conversation
    pub async fn conversations_create(&self, body: &CreateConversationRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/conversations".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create an agent dialog
    pub async fn conversations_agent_dialogs_create(&self, body: &CreateAgentDialogRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/conversations/agent_dialogs".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create an agent handoff
    pub async fn conversations_agent_handoffs_create(&self, body: &CreateAgentDialogRequest) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&"/chat/conversations/agent_handoffs".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create a system channel
    pub async fn conversations_system_channels_create(&self, body: &CreateConversationRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/conversations/system_channels".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Create a thread conversation
    pub async fn conversations_threads_create(&self, body: &CreateConversationRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/conversations/threads".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Bind a direct chat conversation
    pub async fn conversations_direct_chats_bind(&self, body: &BindDirectChatRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/conversations/direct_chats/bindings".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Retrieve agent handoff state
    pub async fn conversations_agent_handoff_retrieve(&self, conversation_id: &str) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/agent_handoff", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Accept agent handoff
    pub async fn conversations_agent_handoff_accept(&self, conversation_id: &str) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/agent_handoff/accept", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Resolve agent handoff
    pub async fn conversations_agent_handoff_resolve(&self, conversation_id: &str) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/agent_handoff/resolve", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Close agent handoff
    pub async fn conversations_agent_handoff_close(&self, conversation_id: &str) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/agent_handoff/close", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Retrieve conversation summary
    pub async fn conversations_retrieve(&self, conversation_id: &str) -> Result<ConversationSummaryView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// List conversation members
    pub async fn conversations_members_list(&self, conversation_id: &str, limit: Option<i64>, cursor: Option<&str>) -> Result<ListMembersResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("limit", limit, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
        ]);
        let path = append_query_string(im_path(&format!("/chat/conversations/{}/members", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false)))), &query);
        self.client.get(&path, None, None).await
    }

    /// Add a conversation member
    pub async fn conversations_members_add(&self, conversation_id: &str, body: &AddConversationMemberRequest) -> Result<ConversationMember, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/members/add", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Remove a conversation member
    pub async fn conversations_members_remove(&self, conversation_id: &str, body: &RemoveConversationMemberRequest) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/members/remove", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Transfer conversation owner
    pub async fn conversations_members_transfer_owner(&self, conversation_id: &str, body: &TransferConversationOwnerRequest) -> Result<ConversationMember, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/members/transfer_owner", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Change conversation member role
    pub async fn conversations_members_change_role(&self, conversation_id: &str, body: &ChangeConversationMemberRoleRequest) -> Result<ConversationMember, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/members/change_role", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Leave a conversation
    pub async fn conversations_members_leave(&self, conversation_id: &str) -> Result<AckResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/members/leave", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Retrieve conversation preferences
    pub async fn conversations_preferences_retrieve(&self, conversation_id: &str) -> Result<ConversationPreferencesView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/preferences", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update conversation preferences
    pub async fn conversations_preferences_update(&self, conversation_id: &str, body: &UpdateConversationPreferencesRequest) -> Result<ConversationPreferencesView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/preferences", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Retrieve conversation profile
    pub async fn conversations_profile_retrieve(&self, conversation_id: &str) -> Result<ConversationProfileView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/profile", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update conversation profile
    pub async fn conversations_profile_update(&self, conversation_id: &str, body: &UpdateConversationProfileRequest) -> Result<ConversationProfileView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/profile", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Retrieve read cursor
    pub async fn conversations_read_cursor_retrieve(&self, conversation_id: &str) -> Result<ReadCursorView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/read_cursor", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update read cursor
    pub async fn conversations_read_cursor_update(&self, conversation_id: &str, body: &UpdateReadCursorRequest) -> Result<ReadCursorView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/read_cursor", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// List member directory
    pub async fn conversations_member_directory_list(&self, conversation_id: &str) -> Result<MemberDirectoryResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/member_directory", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// List conversation message timeline
    pub async fn conversations_messages_list(&self, conversation_id: &str, after_seq: Option<i64>, limit: Option<i64>) -> Result<TimelineResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("afterSeq", after_seq, "form", true, false, None),
            QueryParameterSpec::new("limit", limit, "form", true, false, None),
        ]);
        let path = append_query_string(im_path(&format!("/chat/conversations/{}/messages", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false)))), &query);
        self.client.get(&path, None, None).await
    }

    /// Post a conversation message
    pub async fn conversations_messages_create(&self, conversation_id: &str, body: &PostMessageRequest) -> Result<PostedMessageResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/messages", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Publish a system channel message
    pub async fn conversations_system_channel_publish(&self, conversation_id: &str, body: &PostMessageRequest) -> Result<PostedMessageResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/system_channel/publish", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// List pinned messages
    pub async fn conversations_pins_list(&self, conversation_id: &str) -> Result<PinnedMessagesResponse, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/pins", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Retrieve message interaction summary
    pub async fn conversations_messages_interaction_summary_retrieve(&self, conversation_id: &str, message_id: &str) -> Result<MessageInteractionSummaryView, SdkworkError> {
        let path = im_path(&format!("/chat/conversations/{}/messages/{}/interaction_summary", serialize_path_parameter(conversation_id, PathParameterSpec::new("conversationId", "simple", false)), serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Edit a message
    pub async fn messages_edit(&self, message_id: &str, body: &EditMessageRequest) -> Result<PostedMessageResponse, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/edit", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Recall a message
    pub async fn messages_recall(&self, message_id: &str) -> Result<PostedMessageResponse, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/recall", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// List message favorites
    pub async fn messages_favorites_list(&self, limit: Option<i64>, cursor: Option<&str>, favorite_type: Option<&str>, q: Option<&str>) -> Result<FavoriteMessagesResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("limit", limit, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("favoriteType", favorite_type, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(im_path(&"/chat/messages/favorites".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// Favorite a message
    pub async fn messages_favorites_create(&self, message_id: &str, body: &FavoriteMessageRequest) -> Result<MessageFavoriteView, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/favorites", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete a message favorite
    pub async fn messages_favorites_delete(&self, favorite_id: &str) -> Result<DeleteMessageFavoriteResponse, SdkworkError> {
        let path = im_path(&format!("/chat/messages/favorites/{}", serialize_path_parameter(favorite_id, PathParameterSpec::new("favoriteId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// Delete message visibility for the current principal
    pub async fn messages_visibility_delete(&self, message_id: &str) -> Result<MessageVisibilityMutationResult, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/visibility", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// Add a message reaction
    pub async fn messages_reactions_create(&self, message_id: &str, body: &MessageReactionRequest) -> Result<MessageReactionMutationResult, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/reactions", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Remove a message reaction
    pub async fn messages_reactions_delete(&self, message_id: &str, body: &MessageReactionRequest) -> Result<MessageReactionMutationResult, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/reactions/remove", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Pin a message
    pub async fn messages_pin_create(&self, message_id: &str) -> Result<MessagePinMutationResult, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/pin", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Unpin a message
    pub async fn messages_pin_delete(&self, message_id: &str) -> Result<MessagePinMutationResult, SdkworkError> {
        let path = im_path(&format!("/chat/messages/{}/unpin", serialize_path_parameter(message_id, PathParameterSpec::new("messageId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Create a live, chat, or game room bound to a group conversation
    pub async fn rooms_create(&self, body: &CreateRoomRequest) -> Result<CreateConversationResult, SdkworkError> {
        let path = im_path(&"/chat/rooms".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get room metadata and active member count
    pub async fn rooms_get(&self, room_id: &str) -> Result<RoomView, SdkworkError> {
        let path = im_path(&format!("/chat/rooms/{}", serialize_path_parameter(room_id, PathParameterSpec::new("roomId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Enter a room as the authenticated principal
    pub async fn rooms_enter(&self, room_id: &str) -> Result<EnterRoomResponse, SdkworkError> {
        let path = im_path(&format!("/chat/rooms/{}/enter", serialize_path_parameter(room_id, PathParameterSpec::new("roomId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Leave a room as the authenticated principal
    pub async fn rooms_leave(&self, room_id: &str) -> Result<EnterRoomResponse, SdkworkError> {
        let path = im_path(&format!("/chat/rooms/{}/leave", serialize_path_parameter(room_id, PathParameterSpec::new("roomId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

}

struct PathParameterSpec<'a> {
    name: &'a str,
    style: &'a str,
    explode: bool,
}

impl<'a> PathParameterSpec<'a> {
    fn new(name: &'a str, style: &'a str, explode: bool) -> Self {
        Self { name, style, explode }
    }
}

fn serialize_path_parameter<T: serde::Serialize>(value: T, spec: PathParameterSpec<'_>) -> String {
    let value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    if value.is_null() {
        return String::new();
    }
    let style = if spec.style.is_empty() { "simple" } else { spec.style };
    match value {
        serde_json::Value::Array(values) => serialize_path_array(spec.name, &values, style, spec.explode),
        serde_json::Value::Object(values) => serialize_path_object(spec.name, &values, style, spec.explode),
        value => format!("{}{}", path_primitive_prefix(spec.name, style), percent_encode(&primitive_to_string(&value))),
    }
}

fn serialize_path_array(name: &str, values: &[serde_json::Value], style: &str, explode: bool) -> String {
    let serialized = values
        .iter()
        .filter(|value| !value.is_null())
        .map(|value| percent_encode(&primitive_to_string(value)))
        .collect::<Vec<_>>();
    if serialized.is_empty() {
        return path_prefix(name, style);
    }
    if style == "matrix" {
        if explode {
            return serialized.iter().map(|item| format!(";{}={}", name, item)).collect::<Vec<_>>().join("");
        }
        return format!(";{}={}", name, serialized.join(","));
    }
    let separator = if explode { "." } else { "," };
    format!("{}{}", path_prefix(name, style), serialized.join(separator))
}

fn serialize_path_object(
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
) -> String {
    let mut entries = Vec::new();
    let mut exploded = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        let escaped_key = percent_encode(key);
        let escaped_value = percent_encode(&primitive_to_string(value));
        if explode {
            if style == "matrix" {
                exploded.push(format!(";{}={}", escaped_key, escaped_value));
            } else {
                exploded.push(format!("{}={}", escaped_key, escaped_value));
            }
        } else {
            entries.push(escaped_key);
            entries.push(escaped_value);
        }
    }
    if style == "matrix" {
        if explode {
            return exploded.join("");
        }
        return format!(";{}={}", name, entries.join(","));
    }
    if explode {
        let separator = if style == "label" { "." } else { "," };
        return format!("{}{}", path_prefix(name, style), exploded.join(separator));
    }
    format!("{}{}", path_prefix(name, style), entries.join(","))
}

fn path_prefix(name: &str, style: &str) -> String {
    match style {
        "label" => ".".to_string(),
        "matrix" => format!(";{}", name),
        _ => String::new(),
    }
}

fn path_primitive_prefix(name: &str, style: &str) -> String {
    if style == "matrix" {
        format!(";{}=", name)
    } else {
        path_prefix(name, style)
    }
}


struct QueryParameterSpec<'a> {
    name: &'a str,
    value: serde_json::Value,
    style: &'a str,
    explode: bool,
    allow_reserved: bool,
    content_type: Option<&'a str>,
}

impl<'a> QueryParameterSpec<'a> {
    fn new<T: serde::Serialize>(
        name: &'a str,
        value: T,
        style: &'a str,
        explode: bool,
        allow_reserved: bool,
        content_type: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            style,
            explode,
            allow_reserved,
            content_type,
        }
    }
}

fn build_query_string(parameters: &[QueryParameterSpec<'_>]) -> String {
    let mut pairs = Vec::new();
    for parameter in parameters {
        append_serialized_parameter(&mut pairs, parameter);
    }
    pairs.join("&")
}

fn append_serialized_parameter(pairs: &mut Vec<String>, parameter: &QueryParameterSpec<'_>) {
    if parameter.value.is_null() {
        return;
    }
    if parameter.content_type.is_some() {
        pairs.push(format!(
            "{}={}",
            percent_encode(parameter.name),
            encode_query_value(&parameter.value.to_string(), parameter.allow_reserved)
        ));
        return;
    }

    let style = if parameter.style.is_empty() { "form" } else { parameter.style };
    match &parameter.value {
        serde_json::Value::Array(values) => append_array_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        serde_json::Value::Object(values) if style == "deepObject" => append_deep_object_parameter(pairs, parameter.name, values, parameter.allow_reserved),
        serde_json::Value::Object(values) => append_object_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        value => pairs.push(format!("{}={}", percent_encode(parameter.name), encode_query_value(&primitive_to_string(value), parameter.allow_reserved))),
    }
}

fn append_array_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &[serde_json::Value],
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let serialized = values.iter().filter(|value| !value.is_null()).map(primitive_to_string).collect::<Vec<_>>();
    if serialized.is_empty() {
        return;
    }
    if style == "form" && explode {
        for item in serialized {
            pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&item, allow_reserved)));
        }
        return;
    }
    pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
}

fn append_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let mut serialized = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        if style == "form" && explode {
            pairs.push(format!("{}={}", percent_encode(key), encode_query_value(&primitive_to_string(value), allow_reserved)));
        } else {
            serialized.push(key.clone());
            serialized.push(primitive_to_string(value));
        }
    }
    if !serialized.is_empty() {
        pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
    }
}

fn append_deep_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    allow_reserved: bool,
) {
    for (key, value) in values {
        if !value.is_null() {
            pairs.push(format!("{}={}", percent_encode(&format!("{}[{}]", name, key)), encode_query_value(&primitive_to_string(value), allow_reserved)));
        }
    }
}

fn encode_query_value(value: &str, allow_reserved: bool) -> String {
    let mut encoded = percent_encode(value);
    if !allow_reserved {
        return encoded;
    }
    for (escaped, reserved) in [
        ("%3A", ":"), ("%2F", "/"), ("%3F", "?"), ("%23", "#"),
        ("%5B", "["), ("%5D", "]"), ("%40", "@"), ("%21", "!"),
        ("%24", "$"), ("%26", "&"), ("%27", "'"), ("%28", "("),
        ("%29", ")"), ("%2A", "*"), ("%2B", "+"), ("%2C", ","),
        ("%3B", ";"), ("%3D", "="),
    ] {
        encoded = encoded.replace(escaped, reserved);
    }
    encoded
}

fn primitive_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        other => other.to_string(),
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{:02X}", byte).chars().collect(),
        })
        .collect()
}
