use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataContentPart {
    pub kind: String,

    #[serde(rename = "schemaRef")]
    pub schema_ref: String,

    pub encoding: String,

    pub payload: String,
}
