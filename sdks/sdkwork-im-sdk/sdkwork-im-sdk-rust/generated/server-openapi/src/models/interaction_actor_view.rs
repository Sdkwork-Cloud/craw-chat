use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InteractionActorView {
    pub id: String,

    pub kind: String,
}
