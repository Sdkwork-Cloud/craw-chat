use serde::{Deserialize, Serialize};

/// Parsed @mention metadata extracted from message text parts. Allows notification fanout to determine who was mentioned and client rendering to highlight mentions without re-parsing.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageMentions {
    /// Mentioned user identifiers (without leading @). Extracted from @<user_id> patterns in message text. Duplicates removed; first-seen order preserved.
    #[serde(rename = "userIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    /// Broad mention scopes (@here, @channel, @all) extracted from message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}
