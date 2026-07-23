use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalDataAvailability {
    pub state: String,

    pub source: String,

    pub complete: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
