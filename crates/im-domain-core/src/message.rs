use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::media::{DriveReference, MediaKind, MediaResource};

pub type MessageAttributes = BTreeMap<String, String>;

pub const SDKWORK_IM_JSON_ENCODING: &str = "application/json";
pub const SDKWORK_IM_MESSAGE_SCHEMA_LOCATION: &str = "urn:sdkwork:sdkwork-im:message:location";
pub const SDKWORK_IM_MESSAGE_SCHEMA_LINK: &str = "urn:sdkwork:sdkwork-im:message:link";
pub const SDKWORK_IM_MESSAGE_SCHEMA_CARD: &str = "urn:sdkwork:sdkwork-im:message:card";
pub const SDKWORK_IM_MESSAGE_SCHEMA_MUSIC: &str = "urn:sdkwork:sdkwork-im:message:music";
pub const SDKWORK_IM_MESSAGE_SCHEMA_CONTACT: &str = "urn:sdkwork:sdkwork-im:message:contact";
pub const SDKWORK_IM_MESSAGE_SCHEMA_STICKER: &str = "urn:sdkwork:sdkwork-im:message:sticker";
pub const SDKWORK_IM_MESSAGE_SCHEMA_VOICE: &str = "urn:sdkwork:sdkwork-im:message:voice";
pub const SDKWORK_IM_MESSAGE_SCHEMA_AGENT: &str = "urn:sdkwork:sdkwork-im:message:agent";
pub const SDKWORK_IM_MESSAGE_SCHEMA_AI_IMAGE: &str = "urn:sdkwork:sdkwork-im:message:ai_image";
pub const SDKWORK_IM_MESSAGE_SCHEMA_AI_VIDEO: &str = "urn:sdkwork:sdkwork-im:message:ai_video";
pub const SDKWORK_IM_CUSTOM_MESSAGE_SCHEMA_PREFIX: &str = "urn:sdkwork:sdkwork-im:message:custom:";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Standard,
    System,
    Signal,
}

impl MessageType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::System => "system",
            Self::Signal => "signal",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub id: String,
    pub kind: String,
    pub member_id: Option<String>,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: MessageAttributes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub sender: Sender,
    pub message_type: MessageType,
    pub delivery_mode: String,
    pub client_msg_id: Option<String>,
    pub stream_session_id: Option<String>,
    pub rtc_session_id: Option<String>,
    pub body: MessageBody,
    pub attributes: MessageAttributes,
    pub metadata: MessageAttributes,
    pub occurred_at: String,
    pub committed_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEdited {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub body: MessageBody,
    pub editor: Sender,
    pub edited_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecalled {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub recalled_by: Sender,
    pub recalled_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionAdded {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub reacted_by: Sender,
    pub reacted_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionRemoved {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub removed_by: Sender,
    pub removed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinned {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub pinned_by: Sender,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageUnpinned {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub unpinned_by: Sender,
    pub unpinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredMessagePin {
    pub pinned_by: Sender,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionActorIdentity {
    pub kind: String,
    pub id: String,
}

impl ReactionActorIdentity {
    pub fn from_sender(sender: &Sender) -> Self {
        Self {
            kind: sender.kind.clone(),
            id: sender.id.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredMessage {
    pub message: Message,
    pub recalled: bool,
    pub reactions: BTreeMap<String, BTreeSet<ReactionActorIdentity>>,
    pub pin: Option<StoredMessagePin>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageHistoryWindow {
    pub items: Vec<StoredMessage>,
    pub high_watermark: u64,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
}

/// Maximum number of messages to cache in memory per conversation.
/// Beyond this limit, oldest messages are evicted to bound memory usage.
pub const CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES: usize = 1000;

/// Number of messages to evict when the cache exceeds the limit.
pub const CONVERSATION_MESSAGE_LOG_EVICTION_BATCH_SIZE: usize = 100;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConversationMessageLog {
    high_watermark: u64,
    messages: HashMap<String, StoredMessage>,
    message_ids_by_seq: BTreeMap<u64, String>,
}

impl ConversationMessageLog {
    pub fn high_watermark(&self) -> u64 {
        self.high_watermark
    }

    pub fn next_message_seq(&mut self) -> u64 {
        self.high_watermark += 1;
        self.high_watermark
    }

    pub fn unread_count_since(&self, read_seq: u64) -> u64 {
        self.high_watermark.saturating_sub(read_seq)
    }

    pub fn received_unread_count_since(
        &self,
        read_seq: u64,
        principal_id: &str,
        principal_kind: &str,
    ) -> u64 {
        self.message_ids_by_seq
            .range((
                std::ops::Bound::Excluded(read_seq),
                std::ops::Bound::Unbounded,
            ))
            .filter_map(|(_, message_id)| self.messages.get(message_id.as_str()))
            .filter(|stored| {
                stored.message.sender.id != principal_id
                    || stored.message.sender.kind != principal_kind
            })
            .count() as u64
    }

    pub fn message(&self, message_id: &str) -> Option<&StoredMessage> {
        self.messages.get(message_id)
    }

    pub fn messages_in_order(&self) -> Vec<StoredMessage> {
        self.message_window_after(0, usize::MAX).items
    }

    pub fn message_window_after(&self, after_seq: u64, limit: usize) -> MessageHistoryWindow {
        let mut items = Vec::with_capacity(limit.min(self.messages.len()));
        let mut has_more = false;
        for (_message_seq, message_id) in self.message_ids_by_seq.range((
            std::ops::Bound::Excluded(after_seq),
            std::ops::Bound::Unbounded,
        )) {
            if items.len() == limit {
                has_more = true;
                break;
            }
            if let Some(stored) = self.messages.get(message_id.as_str()) {
                items.push(stored.clone());
            }
        }
        let next_after_seq = items.last().map(|stored| stored.message.message_seq);

        MessageHistoryWindow {
            items,
            high_watermark: self.high_watermark,
            next_after_seq,
            has_more,
        }
    }

    pub fn store_posted(&mut self, message: Message) {
        let mut message = message;
        message.body = message.body.with_derived_summary();
        self.high_watermark = self.high_watermark.max(message.message_seq);
        if let Some(existing) = self.messages.get(message.message_id.as_str())
            && existing.message.message_seq != message.message_seq
        {
            self.message_ids_by_seq
                .remove(&existing.message.message_seq);
        }
        self.message_ids_by_seq
            .insert(message.message_seq, message.message_id.clone());
        self.messages.insert(
            message.message_id.clone(),
            StoredMessage {
                message,
                recalled: false,
                reactions: BTreeMap::new(),
                pin: None,
            },
        );
        // Evict oldest messages when cache exceeds limit
        self.evict_if_needed();
    }

    /// Evicts oldest messages when cache size exceeds CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES.
    /// Pinned messages are preserved during eviction.
    fn evict_if_needed(&mut self) {
        if self.messages.len() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES {
            return;
        }

        let evict_count = CONVERSATION_MESSAGE_LOG_EVICTION_BATCH_SIZE;

        // Collect sequence numbers to evict (oldest first)
        let seqs_to_evict: Vec<u64> = self
            .message_ids_by_seq
            .keys()
            .take(evict_count)
            .cloned()
            .collect();

        // Evict messages, skipping pinned ones
        for seq in seqs_to_evict {
            if self.messages.len() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES {
                break;
            }

            if let Some(message_id) = self.message_ids_by_seq.get(&seq) {
                // Skip eviction if message is pinned
                if let Some(stored) = self.messages.get(message_id.as_str())
                    && stored.pin.is_some()
                {
                    continue;
                }

                // Remove the message
                self.messages.remove(message_id.as_str());
                self.message_ids_by_seq.remove(&seq);
            }
        }
    }

    pub fn apply_edited(&mut self, edited: &MessageEdited) -> Option<&StoredMessage> {
        let stored = self.messages.get_mut(edited.message_id.as_str())?;
        stored.message.body = edited.body.clone().with_derived_summary();
        stored.message.committed_at = Some(edited.edited_at.clone());
        Some(stored)
    }

    pub fn apply_recalled(&mut self, recalled: &MessageRecalled) -> Option<&StoredMessage> {
        let stored = self.messages.get_mut(recalled.message_id.as_str())?;
        stored.recalled = true;
        stored.message.body.summary = Some("[recalled]".into());
        stored.message.committed_at = Some(recalled.recalled_at.clone());
        Some(stored)
    }

    pub fn apply_reaction_added(&mut self, added: &MessageReactionAdded) -> Option<bool> {
        let stored = self.messages.get_mut(added.message_id.as_str())?;
        let actor_ids = stored
            .reactions
            .entry(added.reaction_key.clone())
            .or_insert_with(BTreeSet::new);
        Some(actor_ids.insert(ReactionActorIdentity::from_sender(&added.reacted_by)))
    }

    pub fn apply_reaction_removed(&mut self, removed: &MessageReactionRemoved) -> Option<bool> {
        let stored = self.messages.get_mut(removed.message_id.as_str())?;
        let Some(actor_ids) = stored.reactions.get_mut(removed.reaction_key.as_str()) else {
            return Some(false);
        };
        let changed = actor_ids.remove(&ReactionActorIdentity::from_sender(&removed.removed_by));
        if actor_ids.is_empty() {
            stored.reactions.remove(removed.reaction_key.as_str());
        }
        Some(changed)
    }

    pub fn apply_pinned(&mut self, pinned: &MessagePinned) -> Option<bool> {
        let stored = self.messages.get_mut(pinned.message_id.as_str())?;
        if stored.pin.is_some() {
            return Some(false);
        }
        stored.pin = Some(StoredMessagePin {
            pinned_by: pinned.pinned_by.clone(),
            pinned_at: pinned.pinned_at.clone(),
        });
        Some(true)
    }

    pub fn apply_unpinned(&mut self, unpinned: &MessageUnpinned) -> Option<bool> {
        let stored = self.messages.get_mut(unpinned.message_id.as_str())?;
        Some(stored.pin.take().is_some())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageLocatorIndex {
    conversation_ids: HashMap<String, String>,
}

impl MessageLocatorIndex {
    pub fn register(&mut self, tenant_id: &str, message_id: &str, conversation_id: &str) {
        self.conversation_ids.insert(
            message_locator_key(tenant_id, message_id),
            conversation_id.to_owned(),
        );
    }

    pub fn register_message(&mut self, message: &Message) {
        self.register(
            message.tenant_id.as_str(),
            message.message_id.as_str(),
            message.conversation_id.as_str(),
        );
    }

    pub fn conversation_id(&self, tenant_id: &str, message_id: &str) -> Option<&str> {
        self.conversation_ids
            .get(message_locator_key(tenant_id, message_id).as_str())
            .map(String::as_str)
    }
}

fn message_locator_key(tenant_id: &str, message_id: &str) -> String {
    encode_message_key_segments([tenant_id, message_id])
}

fn encode_message_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReplyReference {
    pub message_id: String,
    pub sender_display_name: String,
    pub content_preview: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageBody {
    pub summary: Option<String>,
    pub parts: Vec<ContentPart>,
    pub render_hints: MessageAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageReplyReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPart {
    pub schema_ref: String,
    pub encoding: String,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MediaPart {
    pub resource: MediaResource,
    pub drive: DriveReference,
    pub media_role: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalPart {
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRefPart {
    pub stream_id: String,
    pub stream_type: String,
    pub state: String,
}

impl MessageBody {
    pub fn derived_summary(&self) -> Option<String> {
        self.parts
            .iter()
            .filter_map(ContentPart::structured_summary)
            .next()
            .or_else(|| {
                self.parts
                    .iter()
                    .filter_map(ContentPart::fallback_summary)
                    .next()
            })
            .or_else(|| {
                self.parts
                    .iter()
                    .filter_map(ContentPart::text_summary)
                    .next()
            })
    }

    pub fn summary_or_derived(&self) -> Option<String> {
        normalize_summary(self.summary.clone()).or_else(|| self.derived_summary())
    }

    pub fn with_derived_summary(mut self) -> Self {
        self.summary = normalize_summary(self.summary.take()).or_else(|| self.derived_summary());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
// Keeping the content variants inline preserves the public message contract and
// serde shape across services; boxing only media would add cross-crate API churn
// for a layout optimization that is not release-blocking here.
#[allow(clippy::large_enum_variant)]
pub enum ContentPart {
    Text(TextPart),
    Data(DataPart),
    Media(MediaPart),
    Signal(SignalPart),
    StreamRef(StreamRefPart),
}

impl ContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(TextPart { text: text.into() })
    }

    pub fn media(part: MediaPart) -> Self {
        Self::Media(part)
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Data(_) => "data",
            Self::Media(_) => "media",
            Self::Signal(_) => "signal",
            Self::StreamRef(_) => "stream_ref",
        }
    }

    pub fn as_media(&self) -> Option<&MediaPart> {
        match self {
            Self::Media(part) => Some(part),
            _ => None,
        }
    }

    fn structured_summary(&self) -> Option<String> {
        match self {
            Self::Data(part) => summarize_structured_data_part(part),
            _ => None,
        }
    }

    fn fallback_summary(&self) -> Option<String> {
        match self {
            Self::Media(part) => summarize_media_part(part),
            Self::Signal(part) => compact_summary_text(part.signal_type.as_str()),
            Self::StreamRef(part) => compact_summary_text(part.stream_type.as_str())
                .map(|stream_type| format!("Stream: {stream_type}"))
                .or_else(|| Some("Stream".into())),
            Self::Data(_) | Self::Text(_) => None,
        }
    }

    fn text_summary(&self) -> Option<String> {
        match self {
            Self::Text(part) => compact_summary_text(part.text.as_str()),
            _ => None,
        }
    }
}

fn normalize_summary(summary: Option<String>) -> Option<String> {
    summary.and_then(|value| compact_summary_text(value.as_str()))
}

fn summarize_structured_data_part(part: &DataPart) -> Option<String> {
    let payload = parse_json_payload(part.payload.as_str());
    match part.schema_ref.as_str() {
        SDKWORK_IM_MESSAGE_SCHEMA_LOCATION => summarize_location_payload(payload.as_ref()),
        SDKWORK_IM_MESSAGE_SCHEMA_LINK => payload
            .as_ref()
            .and_then(|value| string_field(value, &["title", "url"]))
            .map(|value| format!("Link: {value}"))
            .or_else(|| Some("Link".into())),
        SDKWORK_IM_MESSAGE_SCHEMA_CARD => payload
            .as_ref()
            .and_then(|value| string_field(value, &["title", "subtitle"]))
            .map(|value| format!("Card: {value}"))
            .or_else(|| Some("Card".into())),
        SDKWORK_IM_MESSAGE_SCHEMA_MUSIC => payload
            .as_ref()
            .and_then(|value| string_field(value, &["title", "artist", "url"]))
            .map(|value| format!("Music: {value}"))
            .or_else(|| Some("Music".into())),
        SDKWORK_IM_MESSAGE_SCHEMA_CONTACT => payload
            .as_ref()
            .and_then(|value| string_field(value, &["displayName", "contactId"]))
            .map(|value| format!("Contact: {value}"))
            .or_else(|| Some("Contact".into())),
        SDKWORK_IM_MESSAGE_SCHEMA_STICKER => Some("Sticker".into()),
        SDKWORK_IM_MESSAGE_SCHEMA_VOICE => Some("Voice message".into()),
        SDKWORK_IM_MESSAGE_SCHEMA_AGENT => payload
            .as_ref()
            .and_then(|value| string_field(value, &["agentName", "agentId"]))
            .map(|value| format!("Agent: {value}"))
            .or_else(|| Some("Agent".into())),
        SDKWORK_IM_MESSAGE_SCHEMA_AI_IMAGE => Some("AI image generated".into()),
        SDKWORK_IM_MESSAGE_SCHEMA_AI_VIDEO => Some("AI video generated".into()),
        schema_ref => schema_ref
            .strip_prefix(SDKWORK_IM_CUSTOM_MESSAGE_SCHEMA_PREFIX)
            .and_then(compact_summary_text)
            .map(|custom_type| format!("Custom: {custom_type}")),
    }
}

fn summarize_location_payload(payload: Option<&JsonValue>) -> Option<String> {
    let Some(payload) = payload else {
        return Some("Location".into());
    };

    if let Some(name) = string_field(payload, &["name", "address"]) {
        return Some(format!("Location: {name}"));
    }

    let latitude = payload.get("latitude").and_then(JsonValue::as_f64);
    let longitude = payload.get("longitude").and_then(JsonValue::as_f64);
    match (latitude, longitude) {
        (Some(latitude), Some(longitude)) => {
            Some(format!("Location: {latitude:.4}, {longitude:.4}"))
        }
        _ => Some("Location".into()),
    }
}

fn summarize_media_part(part: &MediaPart) -> Option<String> {
    let kind = resolve_media_kind(&part.resource);
    match kind {
        MediaKind::Image => Some("Image".into()),
        MediaKind::Video => Some("Video".into()),
        MediaKind::Audio => Some("Audio".into()),
        MediaKind::Voice => Some("Voice".into()),
        MediaKind::Document => Some("Document".into()),
        MediaKind::Archive => Some("Archive".into()),
        MediaKind::Model => Some("Model".into()),
        MediaKind::Other => Some("File".into()),
    }
}

fn resolve_media_kind(resource: &MediaResource) -> MediaKind {
    resource.kind.clone()
}

fn parse_json_payload(payload: &str) -> Option<JsonValue> {
    if payload.trim().is_empty() {
        return None;
    }

    serde_json::from_str(payload).ok()
}

fn string_field(payload: &JsonValue, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        payload
            .get(*key)
            .and_then(JsonValue::as_str)
            .and_then(compact_summary_text)
    })
}

fn compact_summary_text(value: &str) -> Option<String> {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return None;
    }

    let mut chars = normalized.chars();
    let mut compact = String::new();
    for _ in 0..120 {
        let Some(ch) = chars.next() else {
            return Some(normalized);
        };
        compact.push(ch);
    }

    if chars.next().is_some() {
        compact.push_str("...");
        return Some(compact);
    }

    Some(normalized)
}
