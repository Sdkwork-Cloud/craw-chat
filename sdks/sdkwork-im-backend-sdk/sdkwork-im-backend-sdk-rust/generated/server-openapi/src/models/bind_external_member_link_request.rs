use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BindExternalMemberLinkRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "externalDisplayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_display_name: Option<String>,

    #[serde(rename = "externalMemberId")]
    pub external_member_id: String,

    #[serde(rename = "linkId")]
    pub link_id: String,

    #[serde(rename = "linkedAt")]
    pub linked_at: String,

    #[serde(rename = "localActorId")]
    pub local_actor_id: String,

    #[serde(rename = "localActorKind")]
    pub local_actor_kind: String,
}
