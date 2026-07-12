use im_domain_events::CommitEnvelope;

use im_domain_core::conversation::{
    ConversationAgentAssignmentSet, ConversationAgentAssignmentSource, ConversationAggregateState,
    legacy_group_agent_assignment_set,
};

use crate::model::{ConversationCatalogEntry, ConversationProfileView, ConversationSummaryView};
use crate::projection::{
    ConversationAgentsReplacedProjectionPayload, ConversationCreatedPayload,
    ConversationPolicyAppliedProjectionPayload, ProjectionError, handoff_view_from_created_payload,
    title_from_created_payload,
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
        let projected_agent_assignments = projected_created_agent_assignments(event, &payload)?;
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
                        agent_assignments: None,
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
            if let Some(assignments) = projected_agent_assignments
                && entry
                    .agent_assignments
                    .as_ref()
                    .is_none_or(|current| assignments.generation >= current.generation)
            {
                entry.agent_assignments = Some(assignments);
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

    pub(crate) fn apply_conversation_agents_replaced(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        if event.event_version != 1
            || event.payload_schema.as_deref() != Some("conversation.agents_replaced.v1")
        {
            return Err(ProjectionError::InvalidEvent(format!(
                "unsupported conversation.agents_replaced version: eventVersion={}, payloadSchema={:?}",
                event.event_version, event.payload_schema
            )));
        }
        let payload: ConversationAgentsReplacedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        if payload.conversation_id.trim() != event.aggregate_id.trim() {
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced conversationId {} does not match aggregate {}",
                payload.conversation_id, event.aggregate_id
            )));
        }
        if payload.replaced_at.trim().is_empty() {
            return Err(ProjectionError::InvalidEvent(
                "conversation.agents_replaced replacedAt must not be empty".into(),
            ));
        }
        if payload.agent_assignments.source
            != ConversationAgentAssignmentSource::ConversationOverride
            || payload.agent_assignments.policy_id.is_some()
            || payload.agent_assignments.policy_version.is_some()
        {
            return Err(ProjectionError::InvalidEvent(
                "conversation.agents_replaced must contain a policy-free conversation_override assignment snapshot".into(),
            ));
        }
        let expected_generation = payload.previous_generation.checked_add(1).ok_or_else(|| {
            ProjectionError::InvalidEvent("conversation.agents_replaced generation overflow".into())
        })?;
        if payload.agent_assignments.generation != expected_generation {
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced generation is not contiguous: previous={}, next={}",
                payload.previous_generation, payload.agent_assignments.generation
            )));
        }
        let next_assignments = payload.agent_assignments.assignment_set();
        validate_projected_agent_assignments(&next_assignments)?;

        let key = scope_key_for_event(event);
        let catalog_is_cold = !lock_projection_mutex(&self.conversations, "conversation store")
            .contains_key(key.as_str());
        if catalog_is_cold {
            self.load_conversation_catalog_from_durable_store(key.as_str());
        }
        let mut conversations = lock_projection_mutex(&self.conversations, "conversation store");
        let entry = conversations.get_mut(key.as_str()).ok_or_else(|| {
            ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced requires conversation.created for {}",
                event.aggregate_id
            ))
        })?;
        if entry.conversation_type != "group" {
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced requires group conversation, got {}",
                entry.conversation_type
            )));
        }
        let current = entry.agent_assignments.as_ref().ok_or_else(|| {
            ProjectionError::InvalidEvent(
                "group conversation is missing mandatory projected agent assignments".into(),
            )
        })?;
        if next_assignments.generation < current.generation {
            return Ok(());
        }
        if next_assignments.generation == current.generation {
            if next_assignments == *current {
                return Ok(());
            }
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced conflicts at generation {}",
                current.generation
            )));
        }
        if payload.previous_generation != current.generation {
            return Err(ProjectionError::InvalidEvent(format!(
                "conversation.agents_replaced previous generation {} does not match current {}",
                payload.previous_generation, current.generation
            )));
        }
        entry.agent_assignments = Some(next_assignments);
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
                agent_assignments: None,
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

    pub fn conversation_agent_assignments(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<ConversationAgentAssignmentSet> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(assignments) = lock_projection_mutex(&self.conversations, "conversation store")
            .get(scope.as_str())
            .and_then(agent_assignments_from_catalog_entry)
        {
            return Some(assignments);
        }
        self.load_conversation_catalog_from_durable_store(scope.as_str())
            .and_then(|entry| agent_assignments_from_catalog_entry(&entry))
    }
}

fn projected_created_agent_assignments(
    event: &CommitEnvelope,
    payload: &ConversationCreatedPayload,
) -> Result<Option<ConversationAgentAssignmentSet>, ProjectionError> {
    if payload.conversation_type != "group" {
        if payload.agent_assignments.is_some() {
            return Err(ProjectionError::InvalidEvent(
                "non-group conversation.created must not contain agentAssignments".into(),
            ));
        }
        return Ok(None);
    }
    let assignments = match (event.event_version, event.payload_schema.as_deref()) {
        (1, Some("conversation.created.v1")) | (1, None) => legacy_group_agent_assignment_set(),
        (2, Some("conversation.created.v2")) => {
            let event_assignments = payload.agent_assignments.as_ref().ok_or_else(|| {
                ProjectionError::InvalidEvent(
                    "conversation.created.v2 group requires agentAssignments".into(),
                )
            })?;
            if event_assignments.generation != 1
                || event_assignments.source != ConversationAgentAssignmentSource::DefaultPolicy
                || event_assignments
                    .policy_id
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty())
                || event_assignments
                    .policy_version
                    .is_none_or(|value| value == 0)
            {
                return Err(ProjectionError::InvalidEvent(
                    "conversation.created.v2 requires a generation-1 versioned default_policy assignment snapshot"
                        .into(),
                ));
            }
            event_assignments.assignment_set()
        }
        (3, Some("conversation.created.v3")) => {
            let event_assignments = payload.agent_assignments.as_ref().ok_or_else(|| {
                ProjectionError::InvalidEvent(
                    "conversation.created.v3 group requires agentAssignments".into(),
                )
            })?;
            if event_assignments.generation != 1
                || event_assignments.source
                    != ConversationAgentAssignmentSource::ConversationOverride
                || event_assignments.policy_id.is_some()
                || event_assignments.policy_version.is_some()
            {
                return Err(ProjectionError::InvalidEvent(
                    "conversation.created.v3 requires a generation-1 policy-free conversation_override assignment snapshot"
                        .into(),
                ));
            }
            event_assignments.assignment_set()
        }
        (event_version, payload_schema) => {
            return Err(ProjectionError::InvalidEvent(format!(
                "unsupported group conversation.created version: eventVersion={event_version}, payloadSchema={payload_schema:?}"
            )));
        }
    };
    validate_projected_agent_assignments(&assignments)?;
    Ok(Some(assignments))
}

fn agent_assignments_from_catalog_entry(
    entry: &ConversationCatalogEntry,
) -> Option<ConversationAgentAssignmentSet> {
    entry
        .agent_assignments
        .clone()
        .or_else(|| (entry.conversation_type == "group").then(legacy_group_agent_assignment_set))
}

pub(crate) fn normalize_conversation_catalog_entry(
    mut entry: ConversationCatalogEntry,
) -> Result<ConversationCatalogEntry, ProjectionError> {
    if entry.conversation_type == "group" {
        if entry.agent_assignments.is_none() {
            entry.agent_assignments = Some(legacy_group_agent_assignment_set());
        }
        if let Some(assignments) = entry.agent_assignments.as_ref() {
            validate_projected_agent_assignments(assignments)?;
        }
        return Ok(entry);
    }
    if entry.agent_assignments.is_some() {
        return Err(ProjectionError::InvalidEvent(format!(
            "non-group conversation catalog snapshot must not contain agent assignments: {}",
            entry.conversation_type
        )));
    }
    Ok(entry)
}

fn validate_projected_agent_assignments(
    assignments: &ConversationAgentAssignmentSet,
) -> Result<(), ProjectionError> {
    if assignments.generation == 0 {
        return Err(ProjectionError::InvalidEvent(
            "projected agent assignment generation must be positive".into(),
        ));
    }
    let mut aggregate = ConversationAggregateState::new("group");
    aggregate
        .restore_agent_assignments(
            assignments.generation,
            assignments.source.clone(),
            assignments.agents.clone(),
        )
        .map_err(|error| {
            ProjectionError::InvalidEvent(format!(
                "projected agent assignments are invalid: {error:?}"
            ))
        })
}

#[cfg(test)]
mod agent_assignment_catalog_tests {
    use super::*;

    #[test]
    fn legacy_group_catalog_snapshot_receives_the_fixed_compatibility_default() {
        let legacy: ConversationCatalogEntry = serde_json::from_str(
            r#"{
                "conversation_type":"group",
                "created_at":"2026-07-11T00:00:00.000Z",
                "history_visibility":"joined",
                "title":"Legacy Group"
            }"#,
        )
        .expect("legacy catalog snapshot should deserialize");

        let normalized = normalize_conversation_catalog_entry(legacy)
            .expect("legacy group catalog should normalize");
        let assignments = normalized
            .agent_assignments
            .expect("legacy group should receive compatibility assignments");
        assert_eq!(assignments.generation, 1);
        assert_eq!(assignments.agents[0].agent_id, "agent.im.default");
    }

    #[test]
    fn catalog_snapshot_normalization_rejects_invalid_assignment_invariants() {
        let invalid_group = ConversationCatalogEntry {
            conversation_type: "group".into(),
            created_at: "2026-07-11T00:00:00.000Z".into(),
            history_visibility: "joined".into(),
            title: None,
            agent_assignments: Some(ConversationAgentAssignmentSet {
                generation: 0,
                source: ConversationAgentAssignmentSource::DefaultPolicy,
                agents: Vec::new(),
            }),
        };
        assert!(normalize_conversation_catalog_entry(invalid_group).is_err());

        let invalid_direct = ConversationCatalogEntry {
            conversation_type: "direct".into(),
            created_at: "2026-07-11T00:00:00.000Z".into(),
            history_visibility: "joined".into(),
            title: None,
            agent_assignments: Some(legacy_group_agent_assignment_set()),
        };
        assert!(normalize_conversation_catalog_entry(invalid_direct).is_err());
    }
}
