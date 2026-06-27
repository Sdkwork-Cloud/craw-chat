use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    Online,
    Offline,
}

impl PresenceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
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
