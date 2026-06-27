use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageReactionRequest {
    #[serde(rename = "reactionKey")]
    pub reaction_key: String,
}
