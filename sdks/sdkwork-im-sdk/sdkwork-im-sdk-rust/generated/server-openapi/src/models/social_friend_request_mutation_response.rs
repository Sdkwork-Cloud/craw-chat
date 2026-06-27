use serde::{Deserialize, Serialize};

use crate::models::{FriendRequest};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestMutationResponse {
    #[serde(rename = "friendRequest")]
    pub friend_request: FriendRequest,
}
