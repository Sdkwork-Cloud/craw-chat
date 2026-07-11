use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;

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
    pub next_before_seq: Option<u64>,
    pub has_more: bool,
}

/// Maximum number of messages to cache in memory per conversation.
/// Beyond this limit, oldest messages are evicted to bound memory usage.
pub const CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES: usize = 1000;

/// Maximum estimated serialized bytes cached per conversation.
pub const CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES: usize = 16 * 1024 * 1024;

/// Number of messages to evict when the cache exceeds the limit.
pub const CONVERSATION_MESSAGE_LOG_EVICTION_BATCH_SIZE: usize = 100;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConversationMessageLog {
    high_watermark: u64,
    messages: HashMap<String, StoredMessage>,
    message_bytes: HashMap<String, usize>,
    cached_message_bytes: usize,
    message_ids_by_seq: BTreeMap<u64, String>,
    pinned_message_ids_by_seq: BTreeMap<u64, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageCacheMutationOutcome<T> {
    pub value: T,
    pub evicted_message_ids: Vec<String>,
}

#[derive(Default)]
struct SerializedSizeCounter {
    bytes: usize,
}

impl Write for SerializedSizeCounter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(buffer.len());
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn estimated_serialized_bytes(value: &impl Serialize) -> usize {
    let mut counter = SerializedSizeCounter::default();
    serde_json::to_writer(&mut counter, value)
        .map(|()| counter.bytes)
        .unwrap_or(CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES)
}

impl ConversationMessageLog {
    pub fn high_watermark(&self) -> u64 {
        self.high_watermark
    }

    pub fn cached_message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn cached_message_bytes(&self) -> usize {
        self.cached_message_bytes
    }

    pub fn estimated_heap_bytes(&self) -> usize {
        const MESSAGE_CACHE_INDEX_OVERHEAD_BYTES: usize = 384;
        std::mem::size_of::<Self>()
            .saturating_add(self.cached_message_bytes)
            .saturating_add(
                self.messages
                    .len()
                    .saturating_mul(MESSAGE_CACHE_INDEX_OVERHEAD_BYTES),
            )
    }

    pub fn observe_high_watermark(&mut self, high_watermark: u64) {
        self.high_watermark = self.high_watermark.max(high_watermark);
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

    pub fn pinned_message_ids_page(&self, offset: usize, limit: usize) -> (Vec<String>, bool) {
        let limit = limit.max(1);
        let mut skipped = 0usize;
        let mut window = Vec::with_capacity(limit.saturating_add(1));
        for message_id in self.pinned_message_ids_by_seq.values() {
            if skipped < offset {
                skipped += 1;
                continue;
            }
            window.push(message_id.clone());
            if window.len() > limit {
                break;
            }
        }
        let has_more = window.len() > limit;
        if has_more {
            window.truncate(limit);
        }
        (window, has_more)
    }

    pub fn message(&self, message_id: &str) -> Option<&StoredMessage> {
        self.messages.get(message_id)
    }

    pub fn messages_in_order(&self) -> Vec<StoredMessage> {
        self.message_ids_by_seq
            .values()
            .filter_map(|message_id| self.messages.get(message_id.as_str()).cloned())
            .collect()
    }

    pub fn message_window_before(
        &self,
        before_seq: Option<u64>,
        page_size: usize,
    ) -> MessageHistoryWindow {
        let page_size = page_size.max(1);
        let upper_bound = before_seq
            .map(std::ops::Bound::Excluded)
            .unwrap_or(std::ops::Bound::Unbounded);
        let mut items = Vec::with_capacity(
            page_size
                .saturating_add(1)
                .min(self.message_ids_by_seq.len()),
        );
        for (_message_seq, message_id) in self
            .message_ids_by_seq
            .range((std::ops::Bound::Unbounded, upper_bound))
            .rev()
        {
            if let Some(stored) = self.messages.get(message_id.as_str()) {
                items.push(stored.clone());
            }
            if items.len() > page_size {
                break;
            }
        }
        let has_more = items.len() > page_size;
        if has_more {
            items.truncate(page_size);
        }
        let next_before_seq = has_more
            .then(|| items.last().map(|stored| stored.message.message_seq))
            .flatten();
        items.reverse();

        MessageHistoryWindow {
            items,
            high_watermark: self.high_watermark,
            next_before_seq,
            has_more,
        }
    }

    pub fn store_posted(&mut self, message: Message) -> Vec<String> {
        let mut message = message;
        message.body = message.body.with_derived_summary();
        self.store_cached(
            StoredMessage {
                message,
                recalled: false,
                reactions: BTreeMap::new(),
                pin: None,
            },
            None,
        )
    }

    pub fn store_hydrated(&mut self, stored: StoredMessage) -> Vec<String> {
        let protected_message_id = stored.message.message_id.clone();
        self.store_cached(stored, Some(protected_message_id.as_str()))
    }

    fn store_cached(
        &mut self,
        mut stored: StoredMessage,
        protected_message_id: Option<&str>,
    ) -> Vec<String> {
        stored.message.body = stored.message.body.with_derived_summary();
        let message_id = stored.message.message_id.clone();
        let message_seq = stored.message.message_seq;
        let estimated_bytes = estimated_serialized_bytes(&stored);
        self.high_watermark = self.high_watermark.max(message_seq);
        if let Some(existing) = self.messages.get(message_id.as_str()) {
            self.message_ids_by_seq
                .remove(&existing.message.message_seq);
            self.pinned_message_ids_by_seq
                .remove(&existing.message.message_seq);
        }
        if let Some(previous_bytes) = self.message_bytes.remove(message_id.as_str()) {
            self.cached_message_bytes = self.cached_message_bytes.saturating_sub(previous_bytes);
        }
        self.message_ids_by_seq
            .insert(message_seq, message_id.clone());
        if stored.pin.is_some() {
            self.pinned_message_ids_by_seq
                .insert(message_seq, message_id.clone());
        }
        self.cached_message_bytes = self.cached_message_bytes.saturating_add(estimated_bytes);
        self.message_bytes
            .insert(message_id.clone(), estimated_bytes);
        self.messages.insert(message_id, stored);
        self.evict_if_needed(protected_message_id)
    }

    /// Evicts oldest messages when cache size exceeds CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES.
    fn evict_if_needed(&mut self, protected_message_id: Option<&str>) -> Vec<String> {
        if self.messages.len() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES
            && self.cached_message_bytes <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES
        {
            return Vec::new();
        }

        let overflow = self
            .messages
            .len()
            .saturating_sub(CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES);
        let minimum_evict_count = if overflow == 0 {
            0
        } else {
            overflow.max(CONVERSATION_MESSAGE_LOG_EVICTION_BATCH_SIZE)
        };
        let mut projected_count = self.messages.len();
        let mut projected_bytes = self.cached_message_bytes;
        let mut unpinned_seqs = Vec::with_capacity(minimum_evict_count);
        for (seq, message_id) in &self.message_ids_by_seq {
            if unpinned_seqs.len() >= minimum_evict_count
                && projected_count <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES
                && projected_bytes <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES
            {
                break;
            }
            if protected_message_id == Some(message_id.as_str())
                || self
                    .messages
                    .get(message_id.as_str())
                    .is_some_and(|stored| stored.pin.is_some())
            {
                continue;
            }
            unpinned_seqs.push(*seq);
            projected_count = projected_count.saturating_sub(1);
            projected_bytes = projected_bytes.saturating_sub(
                self.message_bytes
                    .get(message_id.as_str())
                    .copied()
                    .unwrap_or_default(),
            );
        }
        let mut evicted_message_ids = Vec::with_capacity(unpinned_seqs.len());
        for seq in unpinned_seqs {
            self.evict_message_at_seq(seq, &mut evicted_message_ids);
        }

        if self.messages.len() > CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES
            || self.cached_message_bytes > CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES
        {
            let mut fallback_count = self.messages.len();
            let mut fallback_bytes = self.cached_message_bytes;
            let mut fallback_seqs = Vec::new();
            for (seq, message_id) in &self.message_ids_by_seq {
                if fallback_count <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES
                    && fallback_bytes <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES
                {
                    break;
                }
                if protected_message_id == Some(message_id.as_str()) {
                    continue;
                }
                fallback_seqs.push(*seq);
                fallback_count = fallback_count.saturating_sub(1);
                fallback_bytes = fallback_bytes.saturating_sub(
                    self.message_bytes
                        .get(message_id.as_str())
                        .copied()
                        .unwrap_or_default(),
                );
            }
            for seq in fallback_seqs {
                self.evict_message_at_seq(seq, &mut evicted_message_ids);
            }
        }
        evicted_message_ids
    }

    fn evict_message_at_seq(&mut self, seq: u64, evicted_message_ids: &mut Vec<String>) {
        let Some(message_id) = self.message_ids_by_seq.remove(&seq) else {
            return;
        };
        self.messages.remove(message_id.as_str());
        if let Some(message_bytes) = self.message_bytes.remove(message_id.as_str()) {
            self.cached_message_bytes = self.cached_message_bytes.saturating_sub(message_bytes);
        }
        self.pinned_message_ids_by_seq.remove(&seq);
        evicted_message_ids.push(message_id);
    }

    fn rebalance_mutated_message(&mut self, message_id: &str) -> Vec<String> {
        let Some(stored) = self.messages.get(message_id) else {
            return Vec::new();
        };
        let estimated_bytes = estimated_serialized_bytes(stored);
        let previous_bytes = self
            .message_bytes
            .insert(message_id.to_owned(), estimated_bytes)
            .unwrap_or_default();
        self.cached_message_bytes = self
            .cached_message_bytes
            .saturating_sub(previous_bytes)
            .saturating_add(estimated_bytes);
        self.evict_if_needed(None)
    }

    pub fn apply_edited(
        &mut self,
        edited: &MessageEdited,
    ) -> Option<MessageCacheMutationOutcome<()>> {
        {
            let stored = self.messages.get_mut(edited.message_id.as_str())?;
            stored.message.body = edited.body.clone().with_derived_summary();
            stored.message.committed_at = Some(edited.edited_at.clone());
        }
        Some(MessageCacheMutationOutcome {
            value: (),
            evicted_message_ids: self.rebalance_mutated_message(edited.message_id.as_str()),
        })
    }

    pub fn apply_recalled(
        &mut self,
        recalled: &MessageRecalled,
    ) -> Option<MessageCacheMutationOutcome<()>> {
        {
            let stored = self.messages.get_mut(recalled.message_id.as_str())?;
            stored.recalled = true;
            stored.message.body.summary = Some("[recalled]".into());
            stored.message.committed_at = Some(recalled.recalled_at.clone());
        }
        Some(MessageCacheMutationOutcome {
            value: (),
            evicted_message_ids: self.rebalance_mutated_message(recalled.message_id.as_str()),
        })
    }

    pub fn apply_reaction_added(
        &mut self,
        added: &MessageReactionAdded,
    ) -> Option<MessageCacheMutationOutcome<bool>> {
        let changed = {
            let stored = self.messages.get_mut(added.message_id.as_str())?;
            let actor_ids = stored
                .reactions
                .entry(added.reaction_key.clone())
                .or_insert_with(BTreeSet::new);
            actor_ids.insert(ReactionActorIdentity::from_sender(&added.reacted_by))
        };
        Some(MessageCacheMutationOutcome {
            value: changed,
            evicted_message_ids: if changed {
                self.rebalance_mutated_message(added.message_id.as_str())
            } else {
                Vec::new()
            },
        })
    }

    pub fn apply_reaction_removed(
        &mut self,
        removed: &MessageReactionRemoved,
    ) -> Option<MessageCacheMutationOutcome<bool>> {
        let changed = {
            let stored = self.messages.get_mut(removed.message_id.as_str())?;
            let Some(actor_ids) = stored.reactions.get_mut(removed.reaction_key.as_str()) else {
                return Some(MessageCacheMutationOutcome {
                    value: false,
                    evicted_message_ids: Vec::new(),
                });
            };
            let changed =
                actor_ids.remove(&ReactionActorIdentity::from_sender(&removed.removed_by));
            if actor_ids.is_empty() {
                stored.reactions.remove(removed.reaction_key.as_str());
            }
            changed
        };
        Some(MessageCacheMutationOutcome {
            value: changed,
            evicted_message_ids: if changed {
                self.rebalance_mutated_message(removed.message_id.as_str())
            } else {
                Vec::new()
            },
        })
    }

    pub fn apply_pinned(
        &mut self,
        pinned: &MessagePinned,
    ) -> Option<MessageCacheMutationOutcome<bool>> {
        let message_seq = {
            let stored = self.messages.get_mut(pinned.message_id.as_str())?;
            if stored.pin.is_some() {
                return Some(MessageCacheMutationOutcome {
                    value: false,
                    evicted_message_ids: Vec::new(),
                });
            }
            stored.pin = Some(StoredMessagePin {
                pinned_by: pinned.pinned_by.clone(),
                pinned_at: pinned.pinned_at.clone(),
            });
            stored.message.message_seq
        };
        self.pinned_message_ids_by_seq
            .insert(message_seq, pinned.message_id.clone());
        Some(MessageCacheMutationOutcome {
            value: true,
            evicted_message_ids: self.rebalance_mutated_message(pinned.message_id.as_str()),
        })
    }

    pub fn apply_unpinned(
        &mut self,
        unpinned: &MessageUnpinned,
    ) -> Option<MessageCacheMutationOutcome<bool>> {
        let (message_seq, changed) = {
            let stored = self.messages.get_mut(unpinned.message_id.as_str())?;
            (stored.message.message_seq, stored.pin.take().is_some())
        };
        if changed {
            self.pinned_message_ids_by_seq.remove(&message_seq);
        }
        Some(MessageCacheMutationOutcome {
            value: changed,
            evicted_message_ids: if changed {
                self.rebalance_mutated_message(unpinned.message_id.as_str())
            } else {
                Vec::new()
            },
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageLocatorIndex {
    conversation_ids: HashMap<String, String>,
    message_keys_by_conversation: HashMap<String, BTreeSet<String>>,
}

impl MessageLocatorIndex {
    pub fn register(&mut self, tenant_id: &str, message_id: &str, conversation_id: &str) {
        let message_key = message_locator_key(tenant_id, message_id);
        if let Some(previous_conversation_id) = self
            .conversation_ids
            .insert(message_key.clone(), conversation_id.to_owned())
            && previous_conversation_id != conversation_id
        {
            self.remove_reverse_entry(tenant_id, previous_conversation_id.as_str(), &message_key);
        }
        self.message_keys_by_conversation
            .entry(message_conversation_locator_key(tenant_id, conversation_id))
            .or_default()
            .insert(message_key);
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

    pub fn remove(&mut self, tenant_id: &str, message_id: &str) -> bool {
        let message_key = message_locator_key(tenant_id, message_id);
        let Some(conversation_id) = self.conversation_ids.remove(message_key.as_str()) else {
            return false;
        };
        self.remove_reverse_entry(tenant_id, conversation_id.as_str(), message_key.as_str());
        true
    }

    pub fn remove_conversation(&mut self, tenant_id: &str, conversation_id: &str) -> usize {
        let reverse_key = message_conversation_locator_key(tenant_id, conversation_id);
        let Some(message_keys) = self
            .message_keys_by_conversation
            .remove(reverse_key.as_str())
        else {
            return 0;
        };
        let removed = message_keys.len();
        for message_key in message_keys {
            self.conversation_ids.remove(message_key.as_str());
        }
        removed
    }

    pub fn len(&self) -> usize {
        self.conversation_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.conversation_ids.is_empty()
    }

    fn remove_reverse_entry(&mut self, tenant_id: &str, conversation_id: &str, message_key: &str) {
        let reverse_key = message_conversation_locator_key(tenant_id, conversation_id);
        let Some(message_keys) = self
            .message_keys_by_conversation
            .get_mut(reverse_key.as_str())
        else {
            return;
        };
        message_keys.remove(message_key);
        if message_keys.is_empty() {
            self.message_keys_by_conversation
                .remove(reverse_key.as_str());
        }
    }
}

fn message_locator_key(tenant_id: &str, message_id: &str) -> String {
    encode_message_key_segments([tenant_id, message_id])
}

fn message_conversation_locator_key(tenant_id: &str, conversation_id: &str) -> String {
    encode_message_key_segments([tenant_id, conversation_id])
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionTargetKind {
    Agent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MentionPart {
    pub target_kind: MentionTargetKind,
    pub target_id: String,
    pub display_text: String,
    pub assignment_generation: u64,
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
    Mention(MentionPart),
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
            Self::Mention(_) => "mention",
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

    pub fn as_mention(&self) -> Option<&MentionPart> {
        match self {
            Self::Mention(part) => Some(part),
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
            Self::Mention(part) => compact_summary_text(part.display_text.as_str()),
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

#[cfg(test)]
mod pinned_message_page_tests {
    use std::collections::BTreeMap;

    use super::*;

    fn demo_sender() -> Sender {
        Sender {
            id: "u1".into(),
            kind: "user".into(),
            member_id: Some("cm_u1".into()),
            device_id: None,
            session_id: None,
            metadata: BTreeMap::new(),
        }
    }

    fn demo_pinned_message(seq: u64, message_id: &str) -> Message {
        Message {
            tenant_id: "100001".into(),
            conversation_id: "c_pins".into(),
            message_id: message_id.into(),
            message_seq: seq,
            sender: demo_sender(),
            message_type: MessageType::Standard,
            delivery_mode: "discrete".into(),
            client_msg_id: Some(format!("client_{seq}")),
            stream_session_id: None,
            rtc_session_id: None,
            body: MessageBody {
                summary: Some("pinned".into()),
                parts: vec![ContentPart::text("pinned")],
                render_hints: BTreeMap::new(),
                reply_to: None,
            },
            attributes: BTreeMap::new(),
            metadata: BTreeMap::new(),
            occurred_at: format!("2026-04-07T12:00:{seq:02}.000Z"),
            committed_at: Some(format!("2026-04-07T12:00:{seq:02}.000Z")),
        }
    }

    #[test]
    fn pinned_message_ids_page_uses_maintained_index_without_full_scan() {
        let mut log = ConversationMessageLog::default();
        for (seq, message_id) in [(1_u64, "msg_1"), (2, "msg_2"), (3, "msg_3")] {
            let message = demo_pinned_message(seq, message_id);
            log.store_posted(message.clone());
            log.apply_pinned(&MessagePinned {
                tenant_id: message.tenant_id.clone(),
                conversation_id: message.conversation_id.clone(),
                message_id: message.message_id.clone(),
                message_seq: message.message_seq,
                pinned_by: message.sender.clone(),
                pinned_at: format!("2026-04-07T12:00:{seq:02}.000Z"),
            });
        }

        let (first_page, has_more) = log.pinned_message_ids_page(0, 2);
        assert!(has_more);
        assert_eq!(first_page, vec!["msg_1".to_string(), "msg_2".to_string()]);

        let (second_page, has_more) = log.pinned_message_ids_page(2, 2);
        assert!(!has_more);
        assert_eq!(second_page, vec!["msg_3".to_string()]);
    }

    #[test]
    fn hydrated_old_message_stays_hot_when_cache_is_full() {
        let mut log = ConversationMessageLog::default();
        for seq in 2..=CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES as u64 + 1 {
            log.store_posted(demo_pinned_message(seq, format!("msg_{seq}").as_str()));
        }

        let hydrated = StoredMessage {
            message: demo_pinned_message(1, "msg_1"),
            recalled: false,
            reactions: BTreeMap::new(),
            pin: None,
        };
        let evicted = log.store_hydrated(hydrated);

        assert!(log.message("msg_1").is_some());
        assert!(!evicted.iter().any(|message_id| message_id == "msg_1"));
        assert!(log.messages_in_order().len() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES);
    }

    #[test]
    fn pinned_oldest_batch_does_not_allow_message_cache_growth() {
        let mut log = ConversationMessageLog::default();
        for seq in 1..=CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES as u64 {
            let message = demo_pinned_message(seq, format!("msg_{seq}").as_str());
            log.store_posted(message.clone());
            if seq <= CONVERSATION_MESSAGE_LOG_EVICTION_BATCH_SIZE as u64 {
                log.apply_pinned(&MessagePinned {
                    tenant_id: message.tenant_id.clone(),
                    conversation_id: message.conversation_id.clone(),
                    message_id: message.message_id.clone(),
                    message_seq: message.message_seq,
                    pinned_by: message.sender.clone(),
                    pinned_at: message.occurred_at.clone(),
                });
            }
        }

        let newest_seq = CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES as u64 + 1;
        log.store_posted(demo_pinned_message(
            newest_seq,
            format!("msg_{newest_seq}").as_str(),
        ));

        assert!(log.messages_in_order().len() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_MESSAGES);
        assert!(log.message(format!("msg_{newest_seq}").as_str()).is_some());
    }

    #[test]
    fn message_cache_is_bounded_by_serialized_bytes() {
        const EXPECTED_MAX_CACHED_BYTES: usize = 16 * 1024 * 1024;
        let mut log = ConversationMessageLog::default();
        for seq in 1..=80_u64 {
            let mut message = demo_pinned_message(seq, format!("msg_large_{seq}").as_str());
            message.body = MessageBody {
                summary: None,
                parts: vec![ContentPart::text("x".repeat(256 * 1024))],
                render_hints: BTreeMap::new(),
                reply_to: None,
            };
            log.store_posted(message);
        }

        assert!(log.cached_message_bytes() <= EXPECTED_MAX_CACHED_BYTES);
        assert!(log.cached_message_count() < 80);
    }

    #[test]
    fn message_edit_rebalances_serialized_byte_budget_and_reports_evictions() {
        let mut log = ConversationMessageLog::default();
        for seq in 1..=40_u64 {
            log.store_posted(demo_pinned_message(seq, format!("msg_edit_{seq}").as_str()));
        }

        let mut evicted_message_ids = Vec::new();
        for seq in 1..=40_u64 {
            let outcome = log
                .apply_edited(&MessageEdited {
                    tenant_id: "100001".into(),
                    conversation_id: "c_pins".into(),
                    message_id: format!("msg_edit_{seq}"),
                    message_seq: seq,
                    body: MessageBody {
                        summary: None,
                        parts: vec![ContentPart::text("x".repeat(1024 * 1024))],
                        render_hints: BTreeMap::new(),
                        reply_to: None,
                    },
                    editor: demo_sender(),
                    edited_at: format!("2026-04-07T13:00:{seq:02}.000Z"),
                })
                .expect("cached message should be editable");
            evicted_message_ids.extend(outcome.evicted_message_ids);
        }

        assert!(
            log.cached_message_bytes() <= CONVERSATION_MESSAGE_LOG_MAX_CACHED_BYTES,
            "message mutations must keep the serialized cache within its hard byte budget"
        );
        assert!(!evicted_message_ids.is_empty());
    }
}
