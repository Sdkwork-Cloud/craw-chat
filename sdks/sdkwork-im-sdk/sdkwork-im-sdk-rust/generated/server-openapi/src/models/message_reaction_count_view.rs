use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageReactionCountView {
    #[serde(rename = "reactionKey")]
    pub reaction_key: String,

    pub count: i64,
}
