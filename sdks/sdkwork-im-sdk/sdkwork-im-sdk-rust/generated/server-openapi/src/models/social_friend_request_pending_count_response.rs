use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestPendingCountResponse {
    pub count: i64,
}
