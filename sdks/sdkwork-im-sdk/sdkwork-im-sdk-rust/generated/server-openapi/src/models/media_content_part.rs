use serde::{Deserialize, Serialize};

use crate::models::{DriveReference, MediaResource};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaContentPart {
    pub kind: String,

    pub drive: DriveReference,

    pub resource: MediaResource,

    #[serde(rename = "mediaRole")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_role: Option<String>,
}
