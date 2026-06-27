use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationCreatedPayload {
    conversation_type: String,
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
            event_id: envelope.event_id.clone(),
        }),
        _ => None,
    }
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
            "conversation.policy_applied" => {
                self.apply_recovered_conversation_policy_applied(envelope)
            }
            "conversation.member_joined" => self.apply_recovered_member_joined(envelope),
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
        let business_binding = match (payload.business_type.clone(), payload.business_id.clone()) {
            (Some(business_type), Some(business_id)) => Some(ConversationBusinessBinding {
                business_type,
                business_id,
            }),
            _ => None,
        };
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            conversation.aggregate =
                ConversationAggregateState::new(payload.conversation_type.clone());
            if let Some(record) = generic_create_record {
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, member.clone());
        conversation.roster.ensure_default_read_cursor(&member);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .observe_member_epoch(envelope.ordering_seq);
        conversation.roster.deactivate_member(member);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, payload.previous_owner);
        upsert_member(conversation, payload.new_owner);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, payload.updated_member);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
        let mut state = write_runtime_state(&self.state, "runtime state");
        {
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
            conversation.message_log.store_posted(message.clone());
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
        }
        state.message_locator.register_message(&message);
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .message_log
            .apply_edited(&edited)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.edited without message {}",
                    edited.message_id
                ))
            })?;
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .message_log
            .apply_recalled(&recalled)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.recalled without message {}",
                    recalled.message_id
                ))
            })?;
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .message_log
            .apply_reaction_added(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_added without message {}",
                    reaction.message_id
                ))
            })?;
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .message_log
            .apply_reaction_removed(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_removed without message {}",
                    reaction.message_id
                ))
            })?;
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
        conversation.message_log.apply_pinned(&pin).ok_or_else(|| {
            RuntimeError::Conflict(format!(
                "cannot replay message.pin_added without message {}",
                pin.message_id
            ))
        })?;
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
        let scope_key =
            conversation_scope_key_for_envelope(envelope);
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
            .message_log
            .apply_unpinned(&pin)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.pin_removed without message {}",
                    pin.message_id
                ))
            })?;
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

    fn recovered_created_envelope() -> CommitEnvelope {
        let payload = RecoveredConversationCreatedPayload {
            conversation_type: "group".into(),
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
