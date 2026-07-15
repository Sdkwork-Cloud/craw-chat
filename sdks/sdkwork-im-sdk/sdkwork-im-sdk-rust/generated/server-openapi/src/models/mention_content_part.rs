use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MentionContentPart {
    pub kind: String,

    #[serde(rename = "targetKind")]
    pub target_kind: String,

    #[serde(rename = "targetId")]
    pub target_id: String,

    #[serde(rename = "displayText")]
    pub display_text: String,

    #[serde(rename = "assignmentGeneration")]
    pub assignment_generation: i64,
}
