use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeSubscriptionSyncResponse {
    pub subscriptions: Vec<String>,
}
