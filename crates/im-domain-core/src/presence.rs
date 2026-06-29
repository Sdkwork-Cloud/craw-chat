use serde::{Deserialize, Serialize};

/// User presence status for availability indication.
///
/// Extended presence states beyond simple Online/Offline to support
/// enterprise scenarios where users need to indicate their availability
/// for communication (P1-3 fix).
///
/// # Status Hierarchy
///
/// ```text
/// Online ─▶ Away ─▶ Busy ─▶ Invisible ─▶ Offline
///   │         │       │         │          │
///   └─────────┴───────┴─────────┴──────────┘
///            All can transition to Offline
/// ```
///
/// # QoS Implications
///
/// - `Online`: Full push priority, all notifications delivered immediately
/// - `Away`: Normal push priority, non-urgent notifications may be batched
/// - `Busy`: High-priority only (calls, mentions), others queued
/// - `Invisible`: Appears Offline to others but receives messages
/// - `Offline`: No push, messages queued for next connection
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    /// User is actively available and responsive.
    /// All messages and notifications are delivered with high priority.
    Online,
    /// User is online but idle or stepped away.
    /// Messages delivered normally, but presence shows as "away" to others.
    Away,
    /// User is online but in do-not-disturb mode.
    /// Only high-priority notifications (calls, @mentions) are delivered immediately.
    Busy,
    /// User appears offline to others but is secretly connected.
    /// Useful for checking messages without appearing available.
    Invisible,
    /// User is disconnected or has explicitly set offline status.
    /// Messages are queued for delivery on next connection.
    Offline,
}

impl PresenceStatus {
    /// Get the string representation of the status.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Away => "away",
            Self::Busy => "busy",
            Self::Invisible => "invisible",
            Self::Offline => "offline",
        }
    }
    
    /// Parse a string into a PresenceStatus.
    /// Returns None for unknown values.
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "online" => Some(Self::Online),
            "away" => Some(Self::Away),
            "busy" => Some(Self::Busy),
            "invisible" => Some(Self::Invisible),
            "offline" => Some(Self::Offline),
            _ => None,
        }
    }
    
    /// Check if the user is technically connected (has an active session).
    /// Returns true for Online, Away, Busy, and Invisible states.
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Online | Self::Away | Self::Busy | Self::Invisible)
    }
    
    /// Check if the user should receive normal priority notifications.
    /// Returns false for Busy and Offline states.
    pub fn should_receive_normal_notifications(&self) -> bool {
        matches!(self, Self::Online | Self::Away | Self::Invisible)
    }
    
    /// Check if the user should receive high priority notifications only.
    /// Returns true only for Busy state.
    pub fn is_high_priority_only(&self) -> bool {
        matches!(self, Self::Busy)
    }
    
    /// Check if the user appears online to others.
    /// Returns false for Invisible and Offline states.
    pub fn appears_online_to_others(&self) -> bool {
        matches!(self, Self::Online | Self::Away | Self::Busy)
    }
    
    /// Get the push QoS level for this presence status.
    /// Higher values mean more aggressive push delivery.
    pub fn push_qos_level(&self) -> u8 {
        match self {
            Self::Online => 3,      // Immediate push, all notifications
            Self::Away => 2,        // Normal push, may batch non-urgent
            Self::Busy => 1,        // High priority only
            Self::Invisible => 2,   // Normal push but appears offline
            Self::Offline => 0,     // No push, queue for later
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceClientView {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub platform: Option<String>,
    pub session_id: Option<String>,
    pub status: PresenceStatus,
    pub last_sync_seq: u64,
    pub last_resume_at: Option<String>,
    pub last_seen_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceSnapshotView {
    pub tenant_id: String,
    pub principal_id: String,
    pub current_device_id: Option<String>,
    pub devices: Vec<PresenceClientView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceResumeView {
    pub tenant_id: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub session_id: Option<String>,
    pub device_id: String,
    pub resume_required: bool,
    pub resume_from_sync_seq: u64,
    pub latest_sync_seq: u64,
    pub resumed_at: String,
    pub presence: PresenceSnapshotView,
}
