use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EventActor {
    #[serde(rename = "actorId")]
    pub actor_id: String,

    #[serde(rename = "actorKind")]
    pub actor_kind: String,

    #[serde(rename = "actorSessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_session_id: Option<String>,
}
