use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextContentPart {
    pub kind: String,

    pub text: String,
}
