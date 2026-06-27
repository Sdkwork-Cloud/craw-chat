use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub type MediaMetadata = BTreeMap<String, JsonValue>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Image,
    Video,
    Audio,
    Voice,
    Document,
    Archive,
    Model,
    Other,
}

impl MediaKind {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Voice => "voice",
            Self::Document => "document",
            Self::Archive => "archive",
            Self::Model => "model",
            Self::Other => "other",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    Drive,
    ExternalUrl,
    DataUrl,
    ProviderAsset,
    Generated,
}

impl MediaSource {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Drive => "drive",
            Self::ExternalUrl => "external_url",
            Self::DataUrl => "data_url",
            Self::ProviderAsset => "provider_asset",
            Self::Generated => "generated",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaChecksumAlgorithm {
    Sha256,
    Md5,
    Etag,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaChecksum {
    pub algorithm: MediaChecksumAlgorithm,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaVisibility {
    Private,
    Tenant,
    Organization,
    Public,
    Signed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAccess {
    pub visibility: MediaVisibility,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaProvenance {
    Uploaded,
    Generated,
    Edited,
    Imported,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaModerationStatus {
    Unknown,
    Pending,
    Approved,
    Rejected,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAiProvenance {
    pub provenance: Option<MediaProvenance>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub prompt_id: Option<String>,
    pub generation_task_id: Option<String>,
    pub source_media_ids: Option<Vec<String>>,
    pub seed: Option<String>,
    pub moderation_status: Option<MediaModerationStatus>,
    pub safety_labels: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DriveReference {
    pub drive_uri: String,
    pub space_id: String,
    pub node_id: String,
    pub node_version: Option<String>,
}

impl DriveReference {
    pub fn canonical_uri(space_id: &str, node_id: &str) -> String {
        format!("drive://spaces/{space_id}/nodes/{node_id}")
    }

    pub fn is_canonical(&self) -> bool {
        self.drive_uri == Self::canonical_uri(self.space_id.as_str(), self.node_id.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MediaResource {
    pub id: Option<String>,
    pub kind: MediaKind,
    pub source: MediaSource,
    pub url: Option<String>,
    pub public_url: Option<String>,
    pub uri: Option<String>,
    pub object_blob_id: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<String>,
    pub checksum: Option<MediaChecksum>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_seconds: Option<u32>,
    pub alt_text: Option<String>,
    pub title: Option<String>,
    pub poster: Option<Box<MediaResource>>,
    pub thumbnails: Option<Vec<MediaResource>>,
    pub variants: Option<Vec<MediaResource>>,
    pub access: Option<MediaAccess>,
    pub ai: Option<MediaAiProvenance>,
    pub metadata: Option<MediaMetadata>,
}

impl MediaResource {
    pub fn content_length(&self) -> Option<u64> {
        self.size_bytes
            .as_deref()
            .and_then(|value| value.trim().parse::<u64>().ok())
    }
}
