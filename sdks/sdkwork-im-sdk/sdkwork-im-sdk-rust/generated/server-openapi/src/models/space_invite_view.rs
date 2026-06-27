use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceInviteView {
    #[serde(rename = "inviteCode")]
    pub invite_code: String,

    #[serde(rename = "spaceId")]
    pub space_id: String,
}
