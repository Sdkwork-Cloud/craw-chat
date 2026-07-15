use super::support::upsert_roster_member;
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationCreatedPayload {
    conversation_type: String,
    #[serde(default)]
    member_user_ids: Vec<String>,
    agent_assignments: Option<agents::ConversationAgentAssignmentsEventPayload>,
    #[serde(default)]
    knowledgebase_initialization_requested: bool,
    business_type: Option<String>,
    business_id: Option<String>,
    room_kind: Option<String>,
    parent_conversation_id: Option<String>,
    root_message_id: Option<String>,
    direct_chat: Option<RecoveredDirectChatBindingPayload>,
    agent_dialog: Option<RecoveredAgentDialogCreatePayload>,
    system_channel: Option<RecoveredSystemChannelCreatePayload>,
    source: Option<ChangeAgentHandoffStatusView>,
    target: Option<ChangeAgentHandoffStatusView>,
    handoff: Option<RecoveredConversationHandoffPayload>,
}

fn recovered_created_agent_assignments(
    envelope: &CommitEnvelope,
    payload: &RecoveredConversationCreatedPayload,
) -> Result<Option<agents::ConversationAgentAssignmentsEventPayload>, RuntimeError> {
    if payload.conversation_type != "group" {
        return Ok(None);
    }
    match (envelope.event_version, envelope.payload_schema.as_deref()) {
        (1, Some("conversation.created.v1")) => Ok(Some(agents::legacy_v1_group_agent_default())),
        // The PostgreSQL journal predates event metadata columns and rebuilds
        // replay envelopes with `(event_version=1, payload_schema=None)`. Use
        // the self-describing assignment payload when it is present, while
        // retaining the fixed compatibility default for genuinely legacy rows.
        (1, None) if !replay_metadata_is_stripped(envelope) => {
            Err(RuntimeError::Conflict(format!(
                "conversation.created {} is missing replay metadata",
                envelope.event_id
            )))
        }
        (1, None) => match payload.agent_assignments.as_ref() {
            Some(assignments)
                if assignments.source
                    == ConversationAgentAssignmentSource::ConversationOverride
                    && assignments.generation == 1
                    && assignments.policy_id.is_none()
                    && assignments.policy_version.is_none() =>
            {
                agents::validate_created_group_agent_override_assignments(assignments)?;
                Ok(Some(assignments.clone()))
            }
            Some(assignments)
                if assignments.source == ConversationAgentAssignmentSource::DefaultPolicy
                    && assignments.policy_id.is_some()
                    && assignments.policy_version.is_some() =>
            {
                agents::validate_created_group_agent_assignments(assignments)?;
                Ok(Some(assignments.clone()))
            }
            Some(_) => Err(RuntimeError::Conflict(format!(
                "stripped conversation.created {} contains an invalid agent assignment snapshot",
                envelope.event_id
            ))),
            None => Ok(Some(agents::legacy_v1_group_agent_default())),
        },
        (2, Some("conversation.created.v2")) => {
            let assignments = payload.agent_assignments.clone().ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "conversation.created.v2 {} is missing mandatory agent assignments",
                    envelope.event_id
                ))
            })?;
            agents::validate_created_group_agent_assignments(&assignments)?;
            Ok(Some(assignments))
        }
        (3, Some("conversation.created.v3")) => {
            let assignments = payload.agent_assignments.clone().ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "conversation.created.v3 {} is missing mandatory agent assignments",
                    envelope.event_id
                ))
            })?;
            agents::validate_created_group_agent_override_assignments(&assignments)?;
            Ok(Some(assignments))
        }
        (event_version, payload_schema) => Err(RuntimeError::Conflict(format!(
            "unsupported group conversation.created version: eventVersion={event_version}, payloadSchema={payload_schema:?}"
        ))),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredAgentDialogCreatePayload {
    agent_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredSystemChannelCreatePayload {
    subscriber_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredDirectChatBindingPayload {
    direct_chat_id: String,
    anchor_actor_id: String,
    anchor_actor_kind: String,
    peer_actor_id: String,
    peer_actor_kind: String,
    pair_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationHandoffPayload {
    session_id: String,
    reason: Option<String>,
    status: String,
}

fn replay_metadata_is_stripped(envelope: &CommitEnvelope) -> bool {
    // `adapters/postgres-journal` stores the portable journal columns only.
    // Older rows therefore come back without event metadata that is not part
    // of that schema. The payload remains the authority for these events.
    envelope.event_version == 1
        && envelope.payload_schema.is_none()
        && envelope.actor.actor_id.is_empty()
        && envelope.actor.actor_kind.is_empty()
        && envelope.causation_id.is_none()
        && envelope.correlation_id.is_none()
}

fn generic_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<GenericConversationCreateReplayRecord> {
    if payload.business_type.is_some()
        || payload.business_id.is_some()
        || payload.source.is_some()
        || payload.target.is_some()
        || payload.handoff.is_some()
    {
        return None;
    }

    match payload.conversation_type.as_str() {
        "group" | "direct" => Some(GenericConversationCreateReplayRecord {
            creator_id: envelope.actor.actor_id.clone(),
            creator_kind: envelope.actor.actor_kind.clone(),
            requested_kind: payload.conversation_type.clone(),
            initial_member_user_ids: payload.member_user_ids.clone(),
            initial_agent_assignments: payload
                .agent_assignments
                .as_ref()
                .filter(|assignments| {
                    assignments.source == ConversationAgentAssignmentSource::ConversationOverride
                })
                .map(|assignments| assignments.agents.clone()),
            knowledgebase_initialization_requested: payload.knowledgebase_initialization_requested,
            event_id: envelope.event_id.clone(),
        }),
        _ => None,
    }
}

fn complete_generic_create_actor_from_roster(
    record: &mut GenericConversationCreateReplayRecord,
    roster: &ConversationRoster,
    event_id: &str,
) -> Result<(), RuntimeError> {
    let Some(owner) = roster
        .members()
        .values()
        .find(|member| matches!(member.role, MembershipRole::Owner) && member.is_active())
        .or_else(|| {
            roster
                .members()
                .values()
                .find(|member| matches!(member.role, MembershipRole::Owner))
        })
    else {
        return Ok(());
    };
    if (!record.creator_id.is_empty() && record.creator_id != owner.principal_id)
        || (!record.creator_kind.is_empty() && record.creator_kind != owner.principal_kind)
    {
        return Err(RuntimeError::Conflict(format!(
            "replayed owner member {event_id} conflicts with conversation create actor"
        )));
    }
    if record.creator_id.is_empty() {
        record.creator_id = owner.principal_id.clone();
    }
    if record.creator_kind.is_empty() {
        record.creator_kind = owner.principal_kind.clone();
    }
    Ok(())
}

fn agent_dialog_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<AgentDialogCreateReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.agent_dialog.as_ref(),
    ) {
        ("agent_dialog", Some(agent_dialog)) => Some(AgentDialogCreateReplayRecord {
            requester_id: envelope.actor.actor_id.clone(),
            requester_kind: envelope.actor.actor_kind.clone(),
            agent_id: agent_dialog.agent_id.clone(),
            event_id: envelope.event_id.clone(),
        }),
        _ => None,
    }
}

fn system_channel_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<SystemChannelCreateReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.system_channel.as_ref(),
    ) {
        ("system_channel", Some(system_channel)) => Some(SystemChannelCreateReplayRecord {
            requester_id: envelope.actor.actor_id.clone(),
            requester_kind: envelope.actor.actor_kind.clone(),
            subscriber_id: system_channel.subscriber_id.clone(),
            event_id: envelope.event_id.clone(),
        }),
        _ => None,
    }
}

fn agent_handoff_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<AgentHandoffCreateReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.source.as_ref(),
        payload.target.as_ref(),
        payload.handoff.as_ref(),
    ) {
        ("agent_handoff", Some(source), Some(target), Some(handoff)) => {
            Some(AgentHandoffCreateReplayRecord {
                source_id: source.id.clone(),
                source_kind: source.kind.clone(),
                target_id: target.id.clone(),
                target_kind: target.kind.clone(),
                handoff_session_id: handoff.session_id.clone(),
                handoff_reason: handoff.reason.clone(),
                event_id: envelope.event_id.clone(),
            })
        }
        _ => None,
    }
}

fn thread_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<ThreadConversationCreateReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.parent_conversation_id.as_ref(),
        payload.root_message_id.as_ref(),
    ) {
        ("thread", Some(parent_conversation_id), Some(root_message_id)) => {
            Some(ThreadConversationCreateReplayRecord {
                creator_id: envelope.actor.actor_id.clone(),
                creator_kind: envelope.actor.actor_kind.clone(),
                parent_conversation_id: parent_conversation_id.clone(),
                root_message_id: root_message_id.clone(),
                event_id: envelope.event_id.clone(),
            })
        }
        _ => None,
    }
}

fn room_create_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<RoomCreateReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.business_type.as_deref(),
        payload.business_id.as_ref(),
        payload.room_kind.as_deref(),
    ) {
        ("group", Some(business_type), Some(room_id), Some(room_kind))
            if im_domain_core::room::is_room_business_type(business_type) =>
        {
            Some(RoomCreateReplayRecord {
                creator_id: envelope.actor.actor_id.clone(),
                creator_kind: envelope.actor.actor_kind.clone(),
                room_id: room_id.clone(),
                room_kind: room_kind.to_string(),
                event_id: envelope.event_id.clone(),
            })
        }
        _ => None,
    }
}

fn direct_chat_binding_replay_record_from_recovered_payload(
    payload: &RecoveredConversationCreatedPayload,
    envelope: &CommitEnvelope,
) -> Option<DirectChatBindingReplayRecord> {
    match (
        payload.conversation_type.as_str(),
        payload.business_type.as_deref(),
        payload.business_id.as_ref(),
        payload.direct_chat.as_ref(),
    ) {
        ("direct", Some("direct_chat"), Some(_business_id), Some(direct_chat)) => {
            Some(DirectChatBindingReplayRecord {
                bound_by: envelope.actor.actor_id.clone(),
                binder_kind: envelope.actor.actor_kind.clone(),
                direct_chat_id: direct_chat.direct_chat_id.clone(),
                anchor_actor_id: direct_chat.anchor_actor_id.clone(),
                anchor_actor_kind: direct_chat.anchor_actor_kind.clone(),
                peer_actor_id: direct_chat.peer_actor_id.clone(),
                peer_actor_kind: direct_chat.peer_actor_kind.clone(),
                event_id: envelope.event_id.clone(),
            })
        }
        _ => None,
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn apply_recovered_envelope(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        match envelope.event_type.as_str() {
            "conversation.created" => self.apply_recovered_conversation_created(envelope),
            "conversation.agents_replaced" => {
                self.apply_recovered_conversation_agents_replaced(envelope)
            }
            "conversation.policy_applied" => {
                self.apply_recovered_conversation_policy_applied(envelope)
            }
            "conversation.group_archived" => self.apply_recovered_group_archived(envelope),
            "conversation.member_joined" => self.apply_recovered_member_joined(envelope),
            "conversation.member_invitation_accepted" => {
                self.apply_recovered_member_joined(envelope)
            }
            "conversation.member_removed" | "conversation.member_left" => {
                self.apply_recovered_member_deactivated(envelope)
            }
            "conversation.read_cursor_updated" => self.apply_recovered_read_cursor(envelope),
            "conversation.owner_transferred" => self.apply_recovered_owner_transfer(envelope),
            "conversation.member_role_changed" => {
                self.apply_recovered_member_role_changed(envelope)
            }
            "conversation.agent_handoff_status_changed" => {
                self.apply_recovered_handoff_status_changed(envelope)
            }
            "message.posted" => self.apply_recovered_message_posted(envelope),
            AGENT_MENTION_DISPATCH_EVENT_TYPE => {
                self.apply_recovered_agent_mention_dispatch_requested(envelope)
            }
            "message.edited" => self.apply_recovered_message_edited(envelope),
            "message.recalled" => self.apply_recovered_message_recalled(envelope),
            "message.reaction_added" => self.apply_recovered_message_reaction_added(envelope),
            "message.reaction_removed" => self.apply_recovered_message_reaction_removed(envelope),
            "message.pin_added" => self.apply_recovered_message_pinned(envelope),
            "message.pin_removed" => self.apply_recovered_message_unpinned(envelope),
            _ => Ok(()),
        }
    }

    fn apply_recovered_conversation_created(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: RecoveredConversationCreatedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.created {}: {error}",
                    envelope.event_id
                ))
            })?;
        let recovered_agent_assignments = recovered_created_agent_assignments(envelope, &payload)?;
        let business_binding = match (payload.business_type.clone(), payload.business_id.clone()) {
            (Some(business_type), Some(business_id)) => Some(ConversationBusinessBinding {
                business_type,
                business_id,
            }),
            _ => None,
        };
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let business_scope_key = business_binding.as_ref().map(|binding| {
            conversation_business_scope_key(
                envelope.tenant_id.as_str(),
                binding.business_type.as_str(),
                binding.business_id.as_str(),
            )
        });
        if let (Some(binding), Some(business_scope_key)) =
            (business_binding.as_ref(), business_scope_key.as_ref())
            && let Some(existing_conversation_id) =
                state.business_index.get(business_scope_key.as_str())
            && existing_conversation_id != envelope.scope_id.as_str()
        {
            return Err(RuntimeError::Conflict(format!(
                "replayed business binding {}/{} already mapped to conversation {existing_conversation_id}",
                binding.business_type, binding.business_id
            )));
        }

        {
            let generic_create_record =
                generic_create_replay_record_from_recovered_payload(&payload, envelope);
            let agent_dialog_create_record =
                agent_dialog_create_replay_record_from_recovered_payload(&payload, envelope);
            let system_channel_create_record =
                system_channel_create_replay_record_from_recovered_payload(&payload, envelope);
            let agent_handoff_create_record =
                agent_handoff_create_replay_record_from_recovered_payload(&payload, envelope);
            let thread_create_record =
                thread_create_replay_record_from_recovered_payload(&payload, envelope);
            let room_create_record =
                room_create_replay_record_from_recovered_payload(&payload, envelope);
            let direct_chat_binding_record =
                direct_chat_binding_replay_record_from_recovered_payload(&payload, envelope);
            let conversation = state.conversations.entry(scope_key).or_default();
            if conversation.aggregate.conversation_type().is_empty() {
                conversation.aggregate =
                    ConversationAggregateState::new(payload.conversation_type.clone());
            } else if conversation.aggregate.conversation_type() != payload.conversation_type {
                return Err(RuntimeError::Conflict(format!(
                    "replayed conversation.created {} changed conversation type from {} to {}",
                    envelope.event_id,
                    conversation.aggregate.conversation_type(),
                    payload.conversation_type
                )));
            }
            if let Some(assignments) = recovered_agent_assignments {
                let current_generation = conversation
                    .aggregate
                    .agent_assignments()
                    .map(|current| current.generation)
                    .unwrap_or_default();
                if assignments.generation >= current_generation {
                    conversation
                        .aggregate
                        .restore_agent_assignments(
                            assignments.generation,
                            assignments.source,
                            assignments.agents,
                        )
                        .map_err(agents::agent_assignment_error_to_runtime)?;
                }
            }
            if let Some(mut record) = generic_create_record {
                complete_generic_create_actor_from_roster(
                    &mut record,
                    &conversation.roster,
                    envelope.event_id.as_str(),
                )?;
                if let Some(existing) = conversation.generic_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed generic create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.generic_create_request = Some(record);
                }
            }
            if let Some(record) = agent_dialog_create_record {
                if let Some(existing) = conversation.agent_dialog_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed agent dialog create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.agent_dialog_create_request = Some(record);
                }
            }
            if let Some(record) = system_channel_create_record {
                if let Some(existing) = conversation.system_channel_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed system channel create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.system_channel_create_request = Some(record);
                }
            }
            if let Some(record) = agent_handoff_create_record {
                if let Some(existing) = conversation.agent_handoff_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed agent handoff create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.agent_handoff_create_request = Some(record);
                }
            }
            if let Some(record) = thread_create_record {
                if let Some(existing) = conversation.thread_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed thread create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.thread_create_request = Some(record);
                }
            }
            if let Some(record) = room_create_record {
                if let Some(existing) = conversation.room_create_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed room create request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.room_create_request = Some(record);
                }
            }
            if let Some(record) = direct_chat_binding_record {
                if let Some(existing) = conversation.direct_chat_binding_request.as_ref() {
                    if existing != &record {
                        return Err(RuntimeError::Conflict(format!(
                            "replayed direct chat binding request for conversation {} conflicts with existing replay fence",
                            envelope.scope_id
                        )));
                    }
                } else {
                    conversation.direct_chat_binding_request = Some(record);
                }
            }
            if let Some(binding) = business_binding.clone() {
                conversation
                    .aggregate
                    .replace_business_binding(Some(binding));
            }
            if let (Some(source), Some(target), Some(handoff)) =
                (payload.source, payload.target, payload.handoff)
            {
                conversation
                    .aggregate
                    .replace_handoff_state(Some(AgentHandoffStateView {
                        tenant_id: envelope.tenant_id.clone(),
                        conversation_id: envelope.scope_id.clone(),
                        status: handoff.status,
                        source,
                        target,
                        handoff_session_id: handoff.session_id,
                        handoff_reason: handoff.reason,
                        accepted_at: None,
                        accepted_by: None,
                        resolved_at: None,
                        resolved_by: None,
                        closed_at: None,
                        closed_by: None,
                    }));
            }
        }
        if let Some(business_scope_key) = business_scope_key {
            state
                .business_index
                .insert(business_scope_key, envelope.scope_id.clone());
        }
        Ok(())
    }

    fn apply_recovered_group_archived(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        if !replay_metadata_is_stripped(envelope)
            && (envelope.event_version != 1
                || envelope.payload_schema.as_deref() != Some("conversation.group_archived.v1"))
        {
            return Err(RuntimeError::Conflict(format!(
                "unsupported conversation.group_archived version for {}",
                envelope.event_id
            )));
        }
        let payload: ConversationGroupArchivedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.group_archived {}: {error}",
                    envelope.event_id
                ))
            })?;
        let actor_matches = replay_metadata_is_stripped(envelope)
            || (payload.archived_by == envelope.actor.actor_id
                && payload.archived_by_kind == envelope.actor.actor_kind);
        if payload.tenant_id != envelope.tenant_id
            || payload.organization_id != envelope.organization_id
            || payload.conversation_id != envelope.scope_id
            || payload.conversation_id != envelope.aggregate_id
            || envelope.aggregate_type != AggregateType::Conversation
            || envelope.scope_type != "conversation"
            || !actor_matches
        {
            return Err(RuntimeError::Conflict(format!(
                "conversation.group_archived {} has an invalid identity",
                envelope.event_id
            )));
        }
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay group archive without conversation {}",
                    envelope.scope_id
                ))
            })?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::Conflict(format!(
                "conversation.group_archived {} targets a non-group conversation",
                envelope.event_id
            )));
        }
        if let Some(existing_event_id) = conversation.aggregate.archive_event_id()
            && existing_event_id != envelope.event_id
        {
            return Err(RuntimeError::Conflict(format!(
                "conversation.group_archived {} conflicts with existing archive event {existing_event_id}",
                envelope.event_id
            )));
        }
        conversation.aggregate.apply_archive(
            payload.archived_at,
            envelope.event_id.clone(),
            envelope.ordering_seq,
        );
        Ok(())
    }

    fn apply_recovered_conversation_agents_replaced(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        if !replay_metadata_is_stripped(envelope)
            && (envelope.event_version != 1
                || envelope.payload_schema.as_deref() != Some("conversation.agents_replaced.v1"))
        {
            return Err(RuntimeError::Conflict(format!(
                "unsupported conversation.agents_replaced version for {}",
                envelope.event_id
            )));
        }
        let payload: agents::ConversationAgentsReplacedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.agents_replaced {}: {error}",
                    envelope.event_id
                ))
            })?;
        if payload.conversation_id != envelope.scope_id {
            return Err(RuntimeError::Conflict(format!(
                "conversation.agents_replaced {} scope does not match payload conversation id",
                envelope.event_id
            )));
        }
        if payload.agent_assignments.source
            != ConversationAgentAssignmentSource::ConversationOverride
            || payload.agent_assignments.policy_id.is_some()
            || payload.agent_assignments.policy_version.is_some()
        {
            return Err(RuntimeError::Conflict(format!(
                "conversation.agents_replaced {} must contain a conversation_override assignment set",
                envelope.event_id
            )));
        }
        let expected_generation = payload.previous_generation.checked_add(1).ok_or_else(|| {
            RuntimeError::Conflict(
                "conversation agent assignment generation overflow during replay".into(),
            )
        })?;
        if payload.agent_assignments.generation != expected_generation {
            return Err(RuntimeError::Conflict(format!(
                "conversation.agents_replaced {} generation is not contiguous: previous={}, next={}",
                envelope.event_id,
                payload.previous_generation,
                payload.agent_assignments.generation
            )));
        }
        let mut validated = ConversationAggregateState::new("group");
        validated
            .restore_agent_assignments(
                payload.agent_assignments.generation,
                payload.agent_assignments.source.clone(),
                payload.agent_assignments.agents.clone(),
            )
            .map_err(agents::agent_assignment_error_to_runtime)?;

        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(envelope.scope_id.clone()))?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::ConversationTypeInvalid(format!(
                "agent assignments require a group conversation, got {}",
                conversation.aggregate.conversation_type()
            )));
        }
        let current_generation = conversation
            .aggregate
            .agent_assignments()
            .map(|current| current.generation)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "group conversation is missing mandatory agent assignments: {}",
                    envelope.scope_id
                ))
            })?;
        if payload.agent_assignments.generation < current_generation {
            conversation
                .aggregate
                .observe_commit_seq(envelope.ordering_seq);
            return Ok(());
        }
        if payload.agent_assignments.generation > current_generation
            && payload.previous_generation != current_generation
        {
            return Err(RuntimeError::Conflict(format!(
                "conversation.agents_replaced {} does not continue current generation {}",
                envelope.event_id, current_generation
            )));
        }
        conversation
            .aggregate
            .restore_agent_assignments(
                payload.agent_assignments.generation,
                payload.agent_assignments.source,
                payload.agent_assignments.agents,
            )
            .map_err(agents::agent_assignment_error_to_runtime)?;
        conversation
            .aggregate
            .observe_commit_seq(envelope.ordering_seq);
        state.touch_conversation(scope_key.as_str());
        Ok(())
    }

    fn apply_recovered_conversation_policy_applied(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: ConversationPolicyAppliedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.policy_applied {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let policy = payload.into_policy().normalize().map_err(|error| {
            RuntimeError::Conflict(format!(
                "failed to normalize replayed conversation policy {}: {error}",
                envelope.event_id
            ))
        })?;
        conversation
            .aggregate
            .observe_policy_epoch(envelope.ordering_seq);
        conversation.aggregate.replace_policy(Some(policy));
        Ok(())
    }

    fn apply_recovered_member_joined(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_joined {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let organization_id =
            im_domain_events::normalize_commit_organization_id(envelope.organization_id.as_str());
        {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay event without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            if matches!(member.role, MembershipRole::Owner)
                && let Some(create_request) = conversation.generic_create_request.as_mut()
            {
                if (!create_request.creator_id.is_empty()
                    && create_request.creator_id != member.principal_id)
                    || (!create_request.creator_kind.is_empty()
                        && create_request.creator_kind != member.principal_kind)
                {
                    return Err(RuntimeError::Conflict(format!(
                        "replayed owner member {} conflicts with conversation create actor",
                        envelope.event_id
                    )));
                }
                if create_request.creator_id.is_empty() {
                    create_request.creator_id = member.principal_id.clone();
                }
                if create_request.creator_kind.is_empty() {
                    create_request.creator_kind = member.principal_kind.clone();
                }
            }
            conversation
                .aggregate
                .observe_member_epoch(envelope.ordering_seq);
            upsert_roster_member(conversation, member.clone());
            conversation.roster.ensure_default_read_cursor(&member);
        }
        state.sync_actor_inbox_member(organization_id.as_str(), &member);
        Ok(())
    }

    fn apply_recovered_member_deactivated(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay {} {}: {error}",
                    envelope.event_type, envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let organization_id =
            im_domain_events::normalize_commit_organization_id(envelope.organization_id.as_str());
        {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay event without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            conversation
                .aggregate
                .observe_member_epoch(envelope.ordering_seq);
            deactivate_roster_member(conversation, member.clone());
        }
        state.sync_actor_inbox_member(organization_id.as_str(), &member);
        Ok(())
    }

    fn apply_recovered_read_cursor(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let cursor: ConversationReadCursor = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.read_cursor_updated {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        // Advance the aggregate commit watermark so a later
        // `next_commit_seq()` allocation cannot reuse this journal slot.
        conversation
            .aggregate
            .observe_commit_seq(envelope.ordering_seq);
        upsert_read_cursor(conversation, cursor);
        Ok(())
    }

    fn apply_recovered_owner_transfer(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: TransferConversationOwnerPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.owner_transferred {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let organization_id =
            im_domain_events::normalize_commit_organization_id(envelope.organization_id.as_str());
        let members_to_sync = {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay event without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            conversation
                .aggregate
                .observe_member_epoch(envelope.ordering_seq);
            upsert_roster_member(conversation, payload.previous_owner.clone());
            upsert_roster_member(conversation, payload.new_owner.clone());
            vec![payload.previous_owner, payload.new_owner]
        };
        state.sync_actor_inbox_members(organization_id.as_str(), members_to_sync.as_slice());
        Ok(())
    }

    fn apply_recovered_member_role_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: ChangeConversationMemberRolePayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_role_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let organization_id =
            im_domain_events::normalize_commit_organization_id(envelope.organization_id.as_str());
        let updated_member = {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay event without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            conversation
                .aggregate
                .observe_member_epoch(envelope.ordering_seq);
            upsert_roster_member(conversation, payload.updated_member.clone());
            payload.updated_member
        };
        state.sync_actor_inbox_member(organization_id.as_str(), &updated_member);
        Ok(())
    }

    fn apply_recovered_handoff_status_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: AgentHandoffStatusChangedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.agent_handoff_status_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_handoff_status_epoch(envelope.ordering_seq);
        conversation
            .aggregate
            .replace_handoff_state(Some(payload.state));
        Ok(())
    }

    fn apply_recovered_message_posted(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let message: Message =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.posted {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let evicted_message_ids = {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay message.posted without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            let evicted_message_ids = conversation.message_log.store_posted(message.clone());
            conversation
                .aggregate
                .observe_commit_seq(envelope.ordering_seq);
            if let Some(request_key) = post_message_request_key_from_message(&message) {
                let replay_record = PostedMessageReplayRecord {
                    sender_id: message.sender.id.clone(),
                    sender_kind: message.sender.kind.clone(),
                    message_type: message.message_type.clone(),
                    body: message.body.clone(),
                    message_id: message.message_id.clone(),
                };
                if let Some(existing) = conversation
                    .posted_message_requests
                    .get(request_key.as_str())
                {
                    if existing != &replay_record {
                        return Err(RuntimeError::Conflict(format!(
                            "cannot replay message.posted with conflicting idempotency key {request_key}"
                        )));
                    }
                } else {
                    conversation
                        .posted_message_requests
                        .insert(request_key, replay_record);
                }
            }
            evicted_message_ids
        };
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(message.tenant_id.as_str(), message_id.as_str());
        }
        state.message_locator.register_message(&message);
        Ok(())
    }

    fn apply_recovered_agent_mention_dispatch_requested(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let metadata_is_stripped = replay_metadata_is_stripped(envelope);
        if !metadata_is_stripped
            && (envelope.event_version != 1
                || envelope.payload_schema.as_deref()
                    != Some(AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA))
        {
            return Err(RuntimeError::Conflict(format!(
                "unsupported agent mention dispatch event version for {}",
                envelope.event_id
            )));
        }
        let request: AgentMentionDispatchRequest = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay agent mention dispatch {}: {error}",
                    envelope.event_id
                ))
            })?;
        request.validate().map_err(|_error| {
            RuntimeError::Conflict(format!(
                "agent mention dispatch {} failed contract validation",
                envelope.event_id,
            ))
        })?;
        let actor_matches = if metadata_is_stripped {
            (envelope.actor.actor_id.is_empty()
                || request.sender_principal_id == envelope.actor.actor_id)
                && (envelope.actor.actor_kind.is_empty()
                    || request.sender_principal_kind == envelope.actor.actor_kind)
        } else {
            request.sender_principal_id == envelope.actor.actor_id
                && request.sender_principal_kind == envelope.actor.actor_kind
        };
        let causation_matches = if metadata_is_stripped {
            envelope
                .causation_id
                .as_deref()
                .is_none_or(|causation_id| causation_id == request.causation_event_id)
        } else {
            envelope.causation_id.as_deref() == Some(request.causation_event_id.as_str())
        };
        if request.tenant_id != envelope.tenant_id
            || request.organization_id != envelope.organization_id
            || request.conversation_id != envelope.scope_id
            || request.conversation_id != envelope.aggregate_id
            || envelope.aggregate_type != AggregateType::Conversation
            || envelope.scope_type != "conversation"
            || !causation_matches
            || !actor_matches
        {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} has an invalid identity or target set",
                envelope.event_id
            )));
        }
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay agent mention dispatch without conversation {}",
                    envelope.scope_id
                ))
            })?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} targets a non-group conversation",
                envelope.event_id
            )));
        }
        let assignments = conversation.aggregate.agent_assignments().ok_or_else(|| {
            RuntimeError::Conflict(format!(
                "agent mention dispatch {} targets a group without agent assignments",
                envelope.event_id
            ))
        })?;
        if assignments.generation != request.assignment_generation
            || request.targets.iter().any(|target| {
                assignments
                    .agents
                    .iter()
                    .find(|assignment| assignment.agent_id == target.agent_id)
                    .is_none_or(|assignment| assignment.revision_id != target.revision_id)
            })
        {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} does not match the current conversation assignment snapshot",
                envelope.event_id
            )));
        }
        if request.message_seq > conversation.message_log.high_watermark() {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} precedes its source message",
                envelope.event_id
            )));
        }
        let stored = conversation
            .message_log
            .message(request.message_id.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "agent mention dispatch {} is missing its source message",
                    envelope.event_id
                ))
            })?;
        let message = &stored.message;
        if message.tenant_id != request.tenant_id
            || message.conversation_id != request.conversation_id
            || message.message_seq != request.message_seq
            || message.sender.id != request.sender_principal_id
            || message.sender.kind != request.sender_principal_kind
            || message.body != request.body
            || message.occurred_at != request.requested_at
        {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} does not match its source message",
                envelope.event_id
            )));
        }
        let deterministic_identity_matches = envelope.event_id
            == super::agent_dispatch::deterministic_agent_dispatch_event_id(
                request.organization_id.as_str(),
                message,
            )
            && request.targets.iter().all(|target| {
                assignments
                    .agents
                    .iter()
                    .find(|assignment| assignment.agent_id == target.agent_id)
                    .is_some_and(|assignment| {
                        target.dispatch_id
                            == super::agent_dispatch::deterministic_agent_dispatch_id(
                                request.organization_id.as_str(),
                                message,
                                request.assignment_generation,
                                assignment,
                            )
                    })
            });
        if !deterministic_identity_matches {
            return Err(RuntimeError::Conflict(format!(
                "agent mention dispatch {} has a non-deterministic dispatch identity",
                envelope.event_id
            )));
        }
        conversation
            .aggregate
            .observe_commit_seq(envelope.ordering_seq);
        Ok(())
    }

    fn apply_recovered_message_edited(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let edited: MessageEdited =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.edited {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_edited(&edited)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.edited without message {}",
                    edited.message_id
                ))
            })?
            .evicted_message_ids;
        conversation
            .aggregate
            .observe_commit_seq(envelope.ordering_seq);
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(edited.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }

    fn apply_recovered_message_recalled(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let recalled: MessageRecalled =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.recalled {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_recalled(&recalled)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.recalled without message {}",
                    recalled.message_id
                ))
            })?
            .evicted_message_ids;
        conversation
            .aggregate
            .observe_commit_seq(envelope.ordering_seq);
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(recalled.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }

    fn apply_recovered_message_reaction_added(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let reaction: MessageReactionAdded = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.reaction_added {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_reaction_added(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_added without message {}",
                    reaction.message_id
                ))
            })?
            .evicted_message_ids;
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(reaction.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }

    fn apply_recovered_message_reaction_removed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let reaction: MessageReactionRemoved = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.reaction_removed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_reaction_removed(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_removed without message {}",
                    reaction.message_id
                ))
            })?
            .evicted_message_ids;
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(reaction.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }

    fn apply_recovered_message_pinned(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let pin: MessagePinned =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.pin_added {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_pinned(&pin)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.pin_added without message {}",
                    pin.message_id
                ))
            })?
            .evicted_message_ids;
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(pin.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }

    fn apply_recovered_message_unpinned(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let pin: MessageUnpinned =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.pin_removed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key = conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let evicted_message_ids = conversation
            .message_log
            .apply_unpinned(&pin)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.pin_removed without message {}",
                    pin.message_id
                ))
            })?
            .evicted_message_ids;
        self.metrics
            .record_message_evictions(evicted_message_ids.len());
        for message_id in evicted_message_ids {
            state
                .message_locator
                .remove(pin.tenant_id.as_str(), message_id.as_str());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_rwlock_write<T>(lock: &RwLock<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = lock.write().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    fn recovered_created_envelope_with_knowledgebase_initialization(
        knowledgebase_initialization_requested: bool,
    ) -> CommitEnvelope {
        let payload = RecoveredConversationCreatedPayload {
            conversation_type: "group".into(),
            member_user_ids: Vec::new(),
            agent_assignments: None,
            knowledgebase_initialization_requested,
            business_type: None,
            business_id: None,
            room_kind: None,
            parent_conversation_id: None,
            root_message_id: None,
            direct_chat: None,
            agent_dialog: None,
            system_channel: None,
            source: None,
            target: None,
            handoff: None,
        };
        CommitEnvelope {
            event_id: "evt_recovery_created".into(),
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: "c_demo".into(),
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            ordering_key: CommitEnvelope::ordering_key("100001", "c_demo"),
            ordering_seq: 1,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: "1".into(),
                actor_kind: "user".into(),
                actor_session_id: None,
            },
            occurred_at: "2026-04-12T00:00:00.000Z".into(),
            committed_at: "2026-04-12T00:00:00.000Z".into(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: serde_json::to_string(&payload).expect("payload should serialize"),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        }
    }

    fn recovered_created_envelope() -> CommitEnvelope {
        recovered_created_envelope_with_knowledgebase_initialization(false)
    }

    #[test]
    fn test_v1_group_recovery_does_not_consult_the_current_default_policy() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default())
            .with_group_agent_default_policy_for_test(
                "policy.im.group.current",
                9,
                vec![ConversationAgentAssignment::new(
                    "agent.im.current",
                    Some("revision.im.current.9".into()),
                )],
            );

        runtime
            .apply_recovered_envelope(&recovered_created_envelope())
            .expect("legacy group creation should replay");
        let assignments = runtime
            .conversation_agent_assignments_snapshot("100001", "0", "c_demo")
            .expect("legacy assignments should be available");

        assert_eq!(assignments.generation, 1);
        assert_eq!(assignments.agents[0].agent_id, "agent.im.default");
        assert_eq!(
            assignments.agents[0].revision_id.as_deref(),
            Some("revision.im.default.1")
        );
    }

    #[test]
    fn test_recovery_retains_group_knowledgebase_initialization_intent() {
        for knowledgebase_initialization_requested in [false, true] {
            let runtime = ConversationRuntime::new(InMemoryJournal::default());
            let envelope = recovered_created_envelope_with_knowledgebase_initialization(
                knowledgebase_initialization_requested,
            );

            runtime
                .apply_recovered_envelope(&envelope)
                .expect("group creation should replay");

            let scope_key = conversation_scope_key("100001", "0", "c_demo");
            let state = read_runtime_state(&runtime.state, "recovery intent test state");
            let replay_record = state
                .conversations
                .get(scope_key.as_str())
                .and_then(|conversation| conversation.generic_create_request.as_ref())
                .expect("recovered group creation should restore a generic replay record");
            assert_eq!(
                replay_record.knowledgebase_initialization_requested,
                knowledgebase_initialization_requested,
                "recovery must preserve the original group Knowledgebase initialization intent"
            );
        }
    }

    #[test]
    fn test_apply_recovered_conversation_created_recovers_from_poisoned_runtime_state_lock() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        let envelope = recovered_created_envelope();
        poison_rwlock_write(&runtime.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.apply_recovered_envelope(&envelope)
        }));
        assert!(
            result.is_ok(),
            "apply_recovered_envelope should not panic when runtime state lock is poisoned"
        );
        let apply_result = result.expect("panic status should be captured");
        assert!(apply_result.is_ok());
    }
}
