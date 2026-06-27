use serde::{Deserialize, Serialize};

use crate::models::{NotificationTask};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NotificationListResponse {
    pub items: Vec<NotificationTask>,
}
