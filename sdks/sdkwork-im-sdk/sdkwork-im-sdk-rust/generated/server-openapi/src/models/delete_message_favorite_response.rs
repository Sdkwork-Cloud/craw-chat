use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteMessageFavoriteResponse {
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,

    pub deleted: bool,
}
