use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "mediaKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,

    pub source: String,

    pub uri: String,

    #[serde(rename = "publicUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "fileName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "sizeBytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<String>,

    #[serde(rename = "fileSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,

    #[serde(rename = "durationSeconds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<Box<MediaResource>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<MediaResource>>,
}
