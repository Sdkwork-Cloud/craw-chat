use serde::{Deserialize, Serialize};

use crate::{StoredMessagePinRecord, StoredMessageReactionRecord};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredMessageMutationTarget {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoredMessageMutation {
    Edited {
        target: StoredMessageMutationTarget,
        payload_json: String,
        payload_hash: String,
        edited_at: String,
    },
    Recalled {
        target: StoredMessageMutationTarget,
        recalled_at: String,
    },
    ReactionAdded {
        target: StoredMessageMutationTarget,
        reaction: StoredMessageReactionRecord,
    },
    ReactionRemoved {
        target: StoredMessageMutationTarget,
        reaction: StoredMessageReactionRecord,
    },
    Pinned {
        target: StoredMessageMutationTarget,
        pin: StoredMessagePinRecord,
    },
    Unpinned {
        target: StoredMessageMutationTarget,
    },
}

impl StoredMessageMutation {
    pub fn target(&self) -> &StoredMessageMutationTarget {
        match self {
            Self::Edited { target, .. }
            | Self::Recalled { target, .. }
            | Self::ReactionAdded { target, .. }
            | Self::ReactionRemoved { target, .. }
            | Self::Pinned { target, .. }
            | Self::Unpinned { target } => target,
        }
    }

    pub fn event_type(&self) -> &'static str {
        match self {
            Self::Edited { .. } => "message.edited",
            Self::Recalled { .. } => "message.recalled",
            Self::ReactionAdded { .. } => "message.reaction_added",
            Self::ReactionRemoved { .. } => "message.reaction_removed",
            Self::Pinned { .. } => "message.pin_added",
            Self::Unpinned { .. } => "message.pin_removed",
        }
    }
}
