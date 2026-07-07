use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialWritePersistence {
    #[serde(rename = "journalAuthority")]
    pub journal_authority: bool,

    #[serde(rename = "snapshotStatus")]
    pub snapshot_status: String,
}
