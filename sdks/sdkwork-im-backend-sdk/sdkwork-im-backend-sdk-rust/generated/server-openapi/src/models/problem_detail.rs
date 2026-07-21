use serde::{Deserialize, Serialize};

use crate::models::{FieldError};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProblemDetail {
    pub r#type: String,

    pub title: String,

    pub status: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Platform or domain error code per API_SPEC.md §15.3.
    pub code: i64,

    /// Server-owned request correlation id.
    #[serde(rename = "traceId")]
    pub trace_id: String,

    /// Optional stable localization key such as errors.result.40001.
    #[serde(rename = "i18nKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,

    /// Optional effective BCP 47 locale used by framework message mapping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}
