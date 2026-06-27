use serde::{Deserialize, Serialize};

use crate::models::{Friendship};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendshipMutationResponse {
    pub friendship: Friendship,
}
