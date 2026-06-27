use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{DataContentPart, MediaContentPart, SignalContentPart, StreamRefContentPart, TextContentPart};

#[derive(Debug, Clone)]
pub enum ContentPart {
    Text(TextContentPart),
    Data(DataContentPart),
    Media(MediaContentPart),
    Signal(SignalContentPart),
    StreamRef(StreamRefContentPart),
}

impl Serialize for ContentPart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Text(value) => value.serialize(serializer),
            Self::Data(value) => value.serialize(serializer),
            Self::Media(value) => value.serialize(serializer),
            Self::Signal(value) => value.serialize(serializer),
            Self::StreamRef(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ContentPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &["text", "data", "media", "signal", "stream_ref"];
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value
            .get("kind")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| de::Error::missing_field("kind"))?
            .to_owned();

        match kind.as_str() {
            "text" => Ok(Self::Text(serde_json::from_value(value).map_err(de::Error::custom)?)),
            "data" => Ok(Self::Data(serde_json::from_value(value).map_err(de::Error::custom)?)),
            "media" => Ok(Self::Media(serde_json::from_value(value).map_err(de::Error::custom)?)),
            "signal" => Ok(Self::Signal(serde_json::from_value(value).map_err(de::Error::custom)?)),
            "stream_ref" => Ok(Self::StreamRef(serde_json::from_value(value).map_err(de::Error::custom)?)),
            other => Err(de::Error::unknown_variant(other, VARIANTS)),
        }
    }
}

impl Default for ContentPart {
    fn default() -> Self {
        Self::Text(TextContentPart::default())
    }
}
