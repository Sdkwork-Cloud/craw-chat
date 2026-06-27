use im_domain_core::conversation::ConversationPolicy;
use im_domain_core::room::{
    ROOM_MEMBER_ATTRIBUTE_ROLE, ROOM_MEMBER_ROLE_OWNER, ROOM_MEMBER_ROLE_PARTICIPANT, RoomKind,
    is_room_business_type, room_kind_from_business_type,
};

use super::*;

fn room_create_request_key(
    tenant_id: &str,
    creator_kind: &str,
    creator_id: &str,
    room_id: &str,
) -> String {
    encode_conversation_key_segments([
        "room.create",
        tenant_id,
        creator_kind,
        creator_id,
        room_id,
    ])
}

fn room_create_replay_matches(
    existing: &RoomCreateReplayRecord,
    command: &CreateRoomCommand,
    creator_kind: &str,
) -> bool {
    existing.creator_id == command.creator_id
        && existing.creator_kind == creator_kind
        && existing.room_id == command.room_id
        && existing.room_kind == command.room_kind
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn create_room_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        room_id: String,
        room_kind: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_room_with_creator_kind(
            CreateRoomCommand::from_auth_context(auth, conversation_id, room_id, room_kind),
            auth.actor_kind.as_str(),
        )
    }

    pub fn create_room_with_creator_kind(
        &self,
        command: CreateRoomCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("roomId", command.room_id.as_str(), CONVERSATION_MAX_ID_BYTES)?;
        validate_payload_size("roomKind", command.room_kind.as_str(), CONVERSATION_MAX_KIND_BYTES)?;
        validate_payload_size(
            "creatorId",
            command.creator_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("creatorKind", creator_kind, CONVERSATION_MAX_KIND_BYTES)?;

        let room_kind = RoomKind::parse_wire_value(command.room_kind.as_str()).ok_or_else(|| {
            RuntimeError::InvalidInput(format!(
                "room kind must be one of live, chat, game; got {}",
                command.room_kind
            ))
        })?;

        if command.room_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "room create requires room_id".into(),
            ));
        }
        if command.conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "room create requires conversation_id".into(),
            ));
        }

        let request_key = room_create_request_key(
            command.tenant_id.as_str(),
            creator_kind,
            command.creator_id.as_str(),
            command.room_id.as_str(),
        );
        let business_binding = ConversationBusinessBinding {
            business_type: room_kind.business_type().into(),
            business_id: command.room_id.clone(),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );
        let created_at = conversation_timestamp();
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let owner_attributes = BTreeMap::from([
            (ROOM_MEMBER_ATTRIBUTE_ROLE.into(), ROOM_MEMBER_ROLE_OWNER.into()),
            ("roomKind".into(), room_kind.as_wire_value().into()),
        ]);
        validate_member_attributes_payload_size("ownerAttributes", &owner_attributes)?;
        let owner_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                creator_kind,
                command.creator_id.as_str(),
            ),
            command.creator_id.as_str(),
            creator_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            owner_attributes,
        );

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.room.create");
        if let Some(existing_conversation_id) = state.business_index.get(business_scope_key.as_str())
        {
            if existing_conversation_id != &command.conversation_id {
                return Err(RuntimeError::Conflict(format!(
                    "room id {} already mapped to conversation {existing_conversation_id}",
                    command.room_id
                )));
            }
        }

        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.room_create_request.as_ref() {
                if room_create_replay_matches(existing, &command, creator_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        command.conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "room create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "room create request conflicts with existing conversation id: {}",
                command.conversation_id
            )));
        }

        let event_id = format!("evt_{}_created", command.conversation_id);
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("group"),
            room_create_request: Some(RoomCreateReplayRecord {
                creator_id: command.creator_id.clone(),
                creator_kind: creator_kind.into(),
                room_id: command.room_id.clone(),
                room_kind: command.room_kind.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        conversation
            .aggregate
            .replace_business_binding(Some(business_binding.clone()));
        conversation.aggregate.replace_policy(Some({
            let mut policy = ConversationPolicy::default();
            policy.history_visibility = room_kind.default_history_visibility().into();
            policy
        }));

        let owner_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, owner_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&owner_member),
        );

        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            organization_id: command.organization_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.creator_id.clone(),
                actor_kind: creator_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at,
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id.clone(),
                "conversationType": "group",
                "businessType": business_binding.business_type.clone(),
                "businessId": business_binding.business_id.clone(),
                "roomKind": room_kind.as_wire_value(),
                "maxMembers": room_kind.default_max_members(),
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            owner_member.clone(),
            owner_ordering_seq,
            "standard",
            command.creator_id.as_str(),
            creator_kind,
        ))?;
        state
            .conversations
            .insert(scope_key, conversation);
        state
            .business_index
            .insert(business_scope_key, command.conversation_id.clone());
        drop(state);

        Ok(CreateConversationResult::applied_with_request_key(
            command.conversation_id,
            event_id,
            request_key,
        ))
    }

    pub fn enter_room_from_auth_context(
        &self,
        auth: &AppContext,
        room_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.enter_room_with_principal_kind(
            EnterRoomCommand::from_auth_context(auth, room_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn enter_room_with_principal_kind(
        &self,
        command: EnterRoomCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        validate_payload_size("roomId", command.room_id.as_str(), CONVERSATION_MAX_ID_BYTES)?;
        validate_payload_size(
            "principalId",
            command.principal_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "principalKind",
            command.principal_kind.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        if command.principal_kind != actor_kind {
            return Err(RuntimeError::PermissionDenied(format!(
                "enter room principal kind mismatch: expected {actor_kind}, got {}",
                command.principal_kind
            )));
        }

        let conversation_id =
            self.resolve_room_conversation_id(&command.tenant_id, &command.organization_id, &command.room_id)?;
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        );
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.room.enter");
            let conversation = state.conversations.get_mut(scope_key.as_str()).ok_or_else(|| {
                RuntimeError::ConversationNotFound(conversation_id.clone())
            })?;
            let room_kind = room_kind_for_conversation(conversation)?;
            policy::ensure_room_enter_allowed(conversation, room_kind)?;

            if let Some(existing) = conversation.roster.resolve_current_member_with_kind(
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
            ) {
                if existing.is_active() {
                    return Ok(existing);
                }
            }

            let invited_by = conversation
                .roster
                .members()
                .values()
                .find(|member| member.role == MembershipRole::Owner && member.is_active())
                .map(|member| member.principal_id.clone())
                .unwrap_or_else(|| command.principal_id.clone());

            let participant_attributes = BTreeMap::from([
                (
                    ROOM_MEMBER_ATTRIBUTE_ROLE.into(),
                    ROOM_MEMBER_ROLE_PARTICIPANT.into(),
                ),
                ("roomKind".into(), room_kind.as_wire_value().into()),
            ]);
            validate_member_attributes_payload_size(
                "participantAttributes",
                &participant_attributes,
            )?;
            let participant = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                conversation_id.as_str(),
                member_id(
                    conversation_id.as_str(),
                    command.principal_kind.as_str(),
                    command.principal_id.as_str(),
                ),
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
                MembershipRole::Guest,
                Some(invited_by),
                conversation_timestamp(),
                participant_attributes,
            );

            let member_epoch = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let envelope = build_member_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
                "conversation.member_joined",
                participant.clone(),
                member_epoch,
                retention_class.as_str(),
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
            );
            self.journal.append(envelope)?;
            upsert_member(conversation, participant.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&participant));
            participant
        };

        Ok(member)
    }

    pub fn leave_room_from_auth_context(
        &self,
        auth: &AppContext,
        room_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        let command = LeaveRoomCommand::from_auth_context(auth, room_id);
        let conversation_id = self.resolve_room_conversation_id(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.room_id.as_str(),
        )?;
        self.leave_conversation_with_actor_kind(
            LeaveConversationCommand {
                tenant_id: command.tenant_id,
                organization_id: command.organization_id,
                conversation_id,
                principal_id: command.principal_id,
            },
            command.principal_kind.as_str(),
        )
    }

    pub fn room_view_from_auth_context(
        &self,
        auth: &AppContext,
        room_id: String,
    ) -> Result<RoomView, RuntimeError> {
        self.room_view(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            room_id.as_str(),
        )
    }

    pub fn room_view(
        &self,
        tenant_id: &str,
        organization_id: &str,
        room_id: &str,
    ) -> Result<RoomView, RuntimeError> {
        validate_payload_size("roomId", room_id, CONVERSATION_MAX_ID_BYTES)?;
        let conversation_id =
            self.resolve_room_conversation_id(tenant_id, organization_id, room_id)?;
        let scope_key =
            conversation_scope_key(tenant_id, organization_id, conversation_id.as_str());
        let state = read_runtime_state(&self.state, "conversation-runtime.state.room.view");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.clone()))?;
        let binding = conversation
            .aggregate
            .business_binding()
            .ok_or_else(|| {
                RuntimeError::ConversationBindingNotFound(format!(
                    "conversation {conversation_id} has no room binding"
                ))
            })?;
        let room_kind = room_kind_from_business_type(binding.business_type.as_str())
            .ok_or_else(|| {
                RuntimeError::InvalidInput(format!(
                    "conversation {conversation_id} is not bound to a room"
                ))
            })?;

        Ok(RoomView {
            room_id: binding.business_id.clone(),
            room_kind: room_kind.as_wire_value().into(),
            conversation_id,
            active_member_count: conversation.roster.active_principal_count(),
            max_members: room_kind.default_max_members(),
        })
    }

    pub(crate) fn resolve_room_conversation_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        room_id: &str,
    ) -> Result<String, RuntimeError> {
        for business_type in [
            im_domain_core::room::ROOM_BUSINESS_TYPE_LIVE,
            im_domain_core::room::ROOM_BUSINESS_TYPE_CHAT,
            im_domain_core::room::ROOM_BUSINESS_TYPE_GAME,
        ] {
            let business_scope_key =
                conversation_business_scope_key(tenant_id, business_type, room_id);
            let state = read_runtime_state(&self.state, "conversation-runtime.state.room.resolve");
            if let Some(conversation_id) = state.business_index.get(business_scope_key.as_str()) {
                let scope_key = conversation_scope_key(
                    tenant_id,
                    organization_id,
                    conversation_id.as_str(),
                );
                if state.conversations.contains_key(scope_key.as_str()) {
                    return Ok(conversation_id.clone());
                }
            }
        }

        Err(RuntimeError::ConversationNotFound(format!(
            "room {room_id} is not registered"
        )))
    }
}

fn room_kind_for_conversation(conversation: &ConversationState) -> Result<RoomKind, RuntimeError> {
    let binding = conversation
        .aggregate
        .business_binding()
        .ok_or_else(|| RuntimeError::ConversationBindingNotFound(
            "room enter requires a business binding".into(),
        ))?;
    if !is_room_business_type(binding.business_type.as_str()) {
        return Err(RuntimeError::InvalidInput(format!(
            "conversation business type {} is not a room binding",
            binding.business_type
        )));
    }
    room_kind_from_business_type(binding.business_type.as_str()).ok_or_else(|| {
        RuntimeError::InvalidInput(format!(
            "unsupported room business type {}",
            binding.business_type
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryJournal;

    #[test]
    fn test_create_and_enter_game_room() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        let created = runtime
            .create_room_with_creator_kind(
                CreateRoomCommand {
                    tenant_id: "100001".into(),
                    organization_id: "org_a".into(),
                    conversation_id: "c_game_room".into(),
                    room_id: "room_game_001".into(),
                    room_kind: "game".into(),
                    creator_id: "1".into(),
                },
                "user",
            )
            .expect("game room create should succeed");
        assert_eq!(created.conversation_id, "c_game_room");

        let view = runtime
            .room_view("100001", "org_a", "room_game_001")
            .expect("room view should exist");
        assert_eq!(view.room_kind, "game");
        assert_eq!(view.active_member_count, 1);

        let joined = runtime
            .enter_room_with_principal_kind(
                EnterRoomCommand {
                    tenant_id: "100001".into(),
                    organization_id: "org_a".into(),
                    room_id: "room_game_001".into(),
                    principal_id: "1040".into(),
                    principal_kind: "user".into(),
                },
                "user",
            )
            .expect("player should enter game room");
        assert_eq!(joined.principal_id, "1040");

        let updated = runtime
            .room_view("100001", "org_a", "room_game_001")
            .expect("room view should update");
        assert_eq!(updated.active_member_count, 2);
    }
}
