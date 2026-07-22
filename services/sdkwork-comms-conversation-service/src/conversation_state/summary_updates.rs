use crate::conversation_state::scope::scope_key;
use crate::conversation_state::{ConversationStateService, lock_conversation_state_mutex};

impl ConversationStateService {
    pub(crate) fn update_timeline_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        summary: Option<String>,
    ) {
        let key = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(entries) =
            lock_conversation_state_mutex(&self.entries, "conversation_state store").get_mut(key.as_str())
            && let Some(entry) = entries
                .values_mut()
                .find(|item| item.message_id.as_str() == message_id)
        {
            entry.summary = summary;
        }
    }

    pub(crate) fn update_conversation_summary_if_last(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        summary: Option<String>,
        occurred_at: String,
    ) {
        if let Some(view) = lock_conversation_state_mutex(&self.summaries, "summary store")
            .get_mut(scope_key(tenant_id, organization_id, conversation_id).as_str())
            && view.last_message_id.as_deref() == Some(message_id)
        {
            view.last_summary = summary;
            view.last_message_at = Some(occurred_at);
        }
    }
}
