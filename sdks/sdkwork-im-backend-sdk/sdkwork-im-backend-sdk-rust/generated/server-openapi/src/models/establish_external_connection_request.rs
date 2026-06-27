use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstablishExternalConnectionRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,

    #[serde(rename = "connectionKind")]
    pub connection_kind: String,

    #[serde(rename = "establishedAt")]
    pub established_at: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "externalOrgName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_org_name: Option<String>,

    #[serde(rename = "externalTenantId")]
    pub external_tenant_id: String,
}
