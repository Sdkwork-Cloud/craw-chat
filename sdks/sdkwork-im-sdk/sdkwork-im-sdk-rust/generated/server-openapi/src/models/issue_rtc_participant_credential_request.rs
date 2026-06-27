use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IssueRtcParticipantCredentialRequest {
    #[serde(rename = "participantId")]
    pub participant_id: String,
}
