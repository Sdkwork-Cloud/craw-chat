use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ArchiveGroupConversationResponse {
    pub accepted: bool,

    #[serde(rename = "resourceId")]
    pub resource_id: String,

    pub status: String,

    #[serde(rename = "archiveEventId")]
    pub archive_event_id: String,

    #[serde(rename = "archivedAt")]
    pub archived_at: String,

    #[serde(rename = "knowledgebaseArchiveScheduled")]
    pub knowledgebase_archive_scheduled: bool,
}
