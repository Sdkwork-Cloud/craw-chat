use serde::{Deserialize, Serialize};

use crate::models::{CommitEnvelopeResponse, SocialWritePersistence, UserBlock};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OpenApiUserBlockResponse {
    #[serde(rename = "userBlock")]
    pub user_block: UserBlock,

    #[serde(rename = "latestCommit")]
    pub latest_commit: CommitEnvelopeResponse,

    pub persistence: SocialWritePersistence,
}
