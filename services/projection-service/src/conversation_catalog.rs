use im_domain_events::CommitEnvelope;

use crate::model::{ConversationCatalogEntry, ConversationProfileView, ConversationSummaryView};
use crate::projection::{
    ConversationCreatedPayload, ConversationPolicyAppliedProjectionPayload, ProjectionError,
    handoff_view_from_created_payload, title_from_created_payload,
};
use crate::scope::{scope_key, scope_key_for_event};
use crate::{TimelineProjectionService, lock_projection_mutex};

impl TimelineProjectionService {
    pub(crate) fn apply_conversation_created(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: ConversationCreatedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let handoff_view = handoff_view_from_created_payload(&payload)?;
        let title = title_from_created_payload(&payload);
        let key = scope_key_for_event(event);
        {
            let mut conversations =
                lock_projection_mutex(&self.conversations, "conversation store");
            let entry =
                conversations
                    .entry(key.clone())
                    .or_insert_with(|| ConversationCatalogEntry {
                        conversation_type: payload.conversation_type.clone(),
                        created_at: event.committed_at.clone(),
                        history_visibility: "joined".into(),
                        title: None,
                    });
            entry.conversation_type = payload.conversation_type.clone();
            entry.created_at = event.committed_at.clone();
            entry.history_visibility = "joined".into();
            if let Some(title) = title.clone()
                && entry
                    .title
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
            {
                entry.title = Some(title);
            }
        }
        self.apply_created_conversation_profile_title(event, key.as_str(), title);
        let conversation_id = event.aggregate_id.clone();
        let tenant_id = event.tenant_id.clone();
        let mut summaries = lock_projection_mutex(&self.summaries, "summary store");
        let summary = summaries
            .entry(key)
            .or_insert_with(|| ConversationSummaryView {
                tenant_id: tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                message_count: 0,
                last_message_id: None,
                last_message_seq: 0,
                last_sender_id: None,
                last_sender_kind: None,
                last_sender: None,
                last_summary: None,
                last_message_at: None,
                agent_handoff: None,
            });
        if handoff_view.is_some() {
            summary.agent_handoff = handoff_view;
        }
        Ok(())
    }

    pub(crate) fn apply_conversation_policy_applied(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: ConversationPolicyAppliedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        if payload.conversation_id.trim() != event.aggregate_id.trim() {
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.policy_applied conversationId {} does not match aggregate {}",
                payload.conversation_id, event.aggregate_id
            )));
        }
        if payload.policy_version.trim().is_empty() {
            return Err(ProjectionError::InvalidEvent(
                "conversation.policy_applied policyVersion must not be empty".into(),
            ));
        }
        let key = scope_key_for_event(event);
        let mut conversations = lock_projection_mutex(&self.conversations, "conversation store");
        let entry = conversations
            .entry(key.clone())
            .or_insert_with(|| ConversationCatalogEntry {
                conversation_type: "unknown".into(),
                created_at: event.committed_at.clone(),
                history_visibility: payload.history_visibility.clone(),
                title: None,
            });
        entry.history_visibility = payload.history_visibility;
        if im_domain_core::retention::retention_is_indefinite(
            im_domain_core::retention::retention_class_from_policy_ref(
                payload.retention_policy_ref.as_str(),
            )
            .as_str(),
        ) {
            let mut entries = lock_projection_mutex(&self.entries, "projection store");
            if let Some(entry) = entries.get_mut(key.as_str()) {
                for item in entry.values_mut() {
                    item.retention_until = None;
                }
            }
        }
        Ok(())
    }

    fn apply_created_conversation_profile_title(
        &self,
        event: &CommitEnvelope,
        scope: &str,
        title: Option<String>,
    ) {
        let Some(display_name) = title
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            return;
        };

        let mut profiles =
            lock_projection_mutex(&self.conversation_profiles, "conversation profile store");
        let profile = profiles
            .entry(scope.to_owned())
            .or_insert_with(|| ConversationProfileView {
                tenant_id: event.tenant_id.clone(),
                conversation_id: event.aggregate_id.clone(),
                display_name: String::new(),
                avatar_url: String::new(),
                notice: String::new(),
                updated_at: event.committed_at.clone(),
                updated_by_principal_kind: Some(event.actor.actor_kind.clone()),
                updated_by_principal_id: Some(event.actor.actor_id.clone()),
            });
        if profile.display_name.trim().is_empty() {
            profile.display_name = display_name.to_owned();
            profile.updated_at = event.committed_at.clone();
            profile.updated_by_principal_kind = Some(event.actor.actor_kind.clone());
            profile.updated_by_principal_id = Some(event.actor.actor_id.clone());
        }
    }

    pub(crate) fn history_visibility_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> String {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(entry) =
            lock_projection_mutex(&self.conversations, "conversation store").get(scope.as_str())
        {
            return entry.history_visibility.clone();
        }
        self.load_conversation_catalog_from_durable_store(scope.as_str())
            .map(|entry| entry.history_visibility)
            .unwrap_or_else(|| "joined".into())
    }
}
