use im_domain_core::conversation::ConversationScenario;
use im_domain_core::social::normalize_actor_pair;

use super::support::{
    canonical_agent_dialog_business_id, resolve_agent_dialog_conversation_id,
    resolve_direct_chat_binding_ids,
};

use super::*;

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn create_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        conversation_type: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_from_auth_context_with_creator_attributes(
            auth,
            conversation_id,
            conversation_type,
            BTreeMap::new(),
        )
    }

    pub fn create_conversation_from_auth_context_with_creator_attributes(
        &self,
        auth: &AppContext,
        conversation_id: String,
        conversation_type: String,
        creator_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind_and_attributes(
            CreateConversationCommand::from_auth_context(auth, conversation_id, conversation_type),
            auth.actor_kind.as_str(),
            creator_attributes,
        )
    }

    pub fn create_conversation(
        &self,
        command: CreateConversationCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind(command, "user")
    }

    pub fn create_conversation_with_creator_kind(
        &self,
        command: CreateConversationCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind_and_attributes(
            command,
            creator_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_conversation_with_creator_kind_and_attributes(
        &self,
        command: CreateConversationCommand,
        creator_kind: &str,
        creator_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "creatorId",
            command.creator_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "conversationType",
            command.conversation_type.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        validate_payload_size("creatorKind", creator_kind, CONVERSATION_MAX_KIND_BYTES)?;
        validate_member_attributes_payload_size("creatorAttributes", &creator_attributes)?;
        policy::ensure_generic_creatable_conversation_type(command.conversation_type.as_str())?;
        let request_key = generic_conversation_create_request_key(
            command.tenant_id.as_str(),
            creator_kind,
            command.creator_id.as_str(),
            command.conversation_id.as_str(),
        );
        let creator_id = command.creator_id.clone();
        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let creator_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                creator_kind,
                creator_id.as_str(),
            ),
            creator_id.as_str(),
            creator_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            creator_attributes,
        );
        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.generic_create_request.as_ref() {
                if generic_conversation_create_replay_matches(existing, &command, creator_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        command.conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "conversation create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "conversation create request conflicts with existing conversation id: {}",
                command.conversation_id
            )));
        }
        let event_id = format!("evt_{}_created", command.conversation_id);
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new(command.conversation_type.clone()),
            generic_create_request: Some(GenericConversationCreateReplayRecord {
                creator_id: creator_id.clone(),
                creator_kind: creator_kind.into(),
                requested_kind: command.conversation_type.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        let creator_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, creator_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&creator_member),
        );
        let creation_members = vec![(creator_member.clone(), creator_ordering_seq)];
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
                actor_id: creator_id.clone(),
                actor_kind: creator_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": command.conversation_type
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            creation_members.as_slice(),
            creator_id.as_str(),
            creator_kind,
        )?;
        state.conversations.insert(scope_key, conversation);
        drop(state);

        self.maybe_evict_after_write();
        Ok(CreateConversationResult::applied_with_request_key(
            command.conversation_id,
            event_id,
            request_key,
        ))
    }

    pub fn create_agent_dialog(
        &self,
        command: CreateAgentDialogCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind(command, "user")
    }

    pub fn create_thread_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        parent_conversation_id: String,
        root_message_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand::from_auth_context(
                auth,
                conversation_id,
                parent_conversation_id,
                root_message_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn create_thread_conversation_with_creator_kind(
        &self,
        command: CreateThreadConversationCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "parentConversationId",
            command.parent_conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "rootMessageId",
            command.root_message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "creatorId",
            command.creator_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("creatorKind", creator_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let request_key = thread_conversation_create_request_key(
            command.tenant_id.as_str(),
            creator_kind,
            command.creator_id.as_str(),
            command.conversation_id.as_str(),
        );
        if command.conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires conversation_id".into(),
            ));
        }
        if command.parent_conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires parent_conversation_id".into(),
            ));
        }
        if command.root_message_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires root_message_id".into(),
            ));
        }
        if command.creator_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires creator identity".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let parent_scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.parent_conversation_id.as_str(),
        );
        let business_binding = ConversationBusinessBinding {
            business_type: "thread".into(),
            business_id: command.root_message_id.clone(),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );
        let event_id = format!("evt_{}_created", command.conversation_id);

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.thread_create_request.as_ref() {
                if thread_conversation_create_replay_matches(existing, &command, creator_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        command.conversation_id.clone(),
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "thread create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "thread create request conflicts with existing conversation id: {}",
                command.conversation_id
            )));
        }
        if let Some(existing_conversation_id) =
            state.business_index.get(business_scope_key.as_str())
        {
            return Err(RuntimeError::Conflict(format!(
                "thread root message {} already mapped to conversation {existing_conversation_id}",
                command.root_message_id
            )));
        }

        let located_parent_id = state
            .message_locator
            .conversation_id(command.tenant_id.as_str(), command.root_message_id.as_str())
            .ok_or_else(|| RuntimeError::MessageNotFound(command.root_message_id.clone()))?
            .to_owned();
        if located_parent_id != command.parent_conversation_id {
            return Err(RuntimeError::InvalidInput(format!(
                "thread root message {} does not belong to parent conversation {}",
                command.root_message_id, command.parent_conversation_id
            )));
        }

        let parent_conversation = state
            .conversations
            .get(parent_scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::ConversationNotFound(command.parent_conversation_id.clone())
            })?;
        if parent_conversation.aggregate.scenario() != ConversationScenario::Group {
            return Err(RuntimeError::ConversationTypeInvalid(format!(
                "thread parent conversation {} must be group, got {}",
                command.parent_conversation_id,
                parent_conversation.aggregate.conversation_type()
            )));
        }
        let parent_creator = resolve_active_member_with_kind(
            parent_conversation,
            command.creator_id.as_str(),
            creator_kind,
        )?;
        policy::ensure_actor_kind_matches_member(&parent_creator, creator_kind)?;
        let root_message = parent_conversation
            .message_log
            .message(command.root_message_id.as_str())
            .ok_or_else(|| RuntimeError::MessageNotFound(command.root_message_id.clone()))?;
        if root_message.recalled {
            return Err(RuntimeError::InvalidInput(format!(
                "thread root message {} is already recalled",
                command.root_message_id
            )));
        }
        let auto_subscribed_root_author = parent_conversation
            .roster
            .resolve_active_member_with_kind(
                root_message.message.sender.id.as_str(),
                root_message.message.sender.kind.as_str(),
            )
            .filter(|member| member.principal_id != command.creator_id);

        let thread_owner_attributes = BTreeMap::from([
            ("threadRole".into(), "owner".into()),
            (
                "parentConversationId".into(),
                command.parent_conversation_id.clone(),
            ),
            ("rootMessageId".into(), command.root_message_id.clone()),
        ]);
        validate_member_attributes_payload_size("threadOwnerAttributes", &thread_owner_attributes)?;
        let thread_owner = build_conversation_member_with_attributes(
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
            Some(command.creator_id.clone()),
            created_at.clone(),
            thread_owner_attributes,
        );

        let mut thread_conversation = ConversationState {
            aggregate: ConversationAggregateState::new("thread"),
            thread_create_request: Some(ThreadConversationCreateReplayRecord {
                creator_id: command.creator_id.clone(),
                creator_kind: creator_kind.into(),
                parent_conversation_id: command.parent_conversation_id.clone(),
                root_message_id: command.root_message_id.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        thread_conversation
            .aggregate
            .replace_business_binding(Some(business_binding.clone()));
        let mut thread_members = Vec::new();
        let owner_ordering_seq = thread_conversation.aggregate.next_member_epoch();
        upsert_member(&mut thread_conversation, thread_owner.clone());
        upsert_read_cursor(
            &mut thread_conversation,
            build_default_read_cursor(&thread_owner),
        );
        thread_members.push((thread_owner, owner_ordering_seq));

        if let Some(root_author) = auto_subscribed_root_author {
            let root_author_attributes = BTreeMap::from([
                ("threadRole".into(), "root_author".into()),
                (
                    "parentConversationId".into(),
                    command.parent_conversation_id.clone(),
                ),
                ("rootMessageId".into(), command.root_message_id.clone()),
            ]);
            validate_member_attributes_payload_size(
                "rootAuthorAttributes",
                &root_author_attributes,
            )?;
            let root_author_member = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_id(
                    command.conversation_id.as_str(),
                    root_author.principal_kind.as_str(),
                    root_author.principal_id.as_str(),
                ),
                root_author.principal_id.as_str(),
                root_author.principal_kind.as_str(),
                MembershipRole::Member,
                Some(command.creator_id.clone()),
                created_at.clone(),
                root_author_attributes,
            );
            let root_author_ordering_seq = thread_conversation.aggregate.next_member_epoch();
            upsert_member(&mut thread_conversation, root_author_member.clone());
            upsert_read_cursor(
                &mut thread_conversation,
                build_default_read_cursor(&root_author_member),
            );
            thread_members.push((root_author_member, root_author_ordering_seq));
        }

        debug_assert_eq!(owner_ordering_seq, 1);
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
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id.clone(),
                "conversationType": "thread",
                "businessType": business_binding.business_type.clone(),
                "businessId": business_binding.business_id.clone(),
                "parentConversationId": command.parent_conversation_id.clone(),
                "rootMessageId": command.root_message_id.clone()
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            thread_members.as_slice(),
            command.creator_id.as_str(),
            creator_kind,
        )?;
        state
            .conversations
            .insert(scope_key.clone(), thread_conversation);
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

    pub fn bind_direct_chat_conversation(
        &self,
        command: BindDirectChatConversationCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.bind_direct_chat_conversation_with_binder_kind(command, "system")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bind_direct_chat_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        direct_chat_id: String,
        left_actor_id: String,
        left_actor_kind: String,
        right_actor_id: String,
        right_actor_kind: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.bind_direct_chat_conversation_with_binder_kind(
            BindDirectChatConversationCommand::from_auth_context(
                auth,
                conversation_id,
                direct_chat_id,
                left_actor_id,
                left_actor_kind,
                right_actor_id,
                right_actor_kind,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn bind_direct_chat_conversation_with_binder_kind(
        &self,
        command: BindDirectChatConversationCommand,
        binder_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "directChatId",
            command.direct_chat_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "leftActorId",
            command.left_actor_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "leftActorKind",
            command.left_actor_kind.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        validate_payload_size(
            "rightActorId",
            command.right_actor_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "rightActorKind",
            command.right_actor_kind.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        validate_payload_size(
            "boundBy",
            command.bound_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("binderKind", binder_kind, CONVERSATION_MAX_KIND_BYTES)?;
        policy::ensure_direct_chat_binding_requester_kind(binder_kind)?;
        if command.bound_by.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "direct chat binding requires binder identity".into(),
            ));
        }
        if command.left_actor_kind.trim().is_empty() || command.right_actor_kind.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "direct chat binding requires actor kinds for both participants".into(),
            ));
        }
        let (conversation_id, direct_chat_id) = resolve_direct_chat_binding_ids(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.left_actor_kind.as_str(),
            command.left_actor_id.as_str(),
            command.right_actor_kind.as_str(),
            command.right_actor_id.as_str(),
            command.conversation_id.as_str(),
            command.direct_chat_id.as_str(),
        )?;
        let request_key = direct_chat_binding_request_key(
            command.tenant_id.as_str(),
            binder_kind,
            command.bound_by.as_str(),
            conversation_id.as_str(),
        );

        let pair = normalize_actor_pair(
            command.left_actor_id.as_str(),
            command.right_actor_id.as_str(),
        )
        .map_err(|error| RuntimeError::InvalidInput(error.to_string()))?;
        let created_at = conversation_timestamp();
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        );
        let business_binding = ConversationBusinessBinding {
            business_type: "direct_chat".into(),
            business_id: direct_chat_id.clone(),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );
        let event_id = format!("evt_{}_created", conversation_id);

        let anchor_attributes = BTreeMap::from([
            (
                "businessType".into(),
                business_binding.business_type.clone(),
            ),
            ("businessId".into(), business_binding.business_id.clone()),
            ("directChatId".into(), direct_chat_id.clone()),
            ("pairHash".into(), pair.pair_hash.clone()),
            ("directChatRole".into(), "anchor".into()),
            ("peerActorId".into(), pair.right_actor_id.clone()),
            ("peerActorKind".into(), command.right_actor_kind.clone()),
        ]);
        validate_member_attributes_payload_size("directChatAnchorAttributes", &anchor_attributes)?;
        let anchor_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            member_id(
                conversation_id.as_str(),
                command.left_actor_kind.as_str(),
                pair.left_actor_id.as_str(),
            ),
            pair.left_actor_id.as_str(),
            command.left_actor_kind.as_str(),
            MembershipRole::Owner,
            None,
            created_at.clone(),
            anchor_attributes,
        );
        let peer_attributes = BTreeMap::from([
            (
                "businessType".into(),
                business_binding.business_type.clone(),
            ),
            ("businessId".into(), business_binding.business_id.clone()),
            ("directChatId".into(), direct_chat_id.clone()),
            ("pairHash".into(), pair.pair_hash.clone()),
            ("directChatRole".into(), "peer".into()),
            ("peerActorId".into(), pair.left_actor_id.clone()),
            ("peerActorKind".into(), command.left_actor_kind.clone()),
        ]);
        validate_member_attributes_payload_size("directChatPeerAttributes", &peer_attributes)?;
        let peer_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            member_id(
                conversation_id.as_str(),
                command.right_actor_kind.as_str(),
                pair.right_actor_id.as_str(),
            ),
            pair.right_actor_id.as_str(),
            command.right_actor_kind.as_str(),
            MembershipRole::Member,
            Some(pair.left_actor_id.clone()),
            created_at.clone(),
            peer_attributes,
        );

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.direct_chat_binding_request.as_ref() {
                if direct_chat_binding_replay_matches(
                    existing,
                    &command,
                    binder_kind,
                    &pair,
                    direct_chat_id.as_str(),
                ) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "direct chat binding request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "direct chat binding request conflicts with existing conversation id: {conversation_id}"
            )));
        }
        if let Some(existing_conversation_id) =
            state.business_index.get(business_scope_key.as_str())
        {
            if existing_conversation_id != &conversation_id {
                return Err(RuntimeError::Conflict(format!(
                    "business binding {}/{} already mapped to conversation {}",
                    business_binding.business_type,
                    business_binding.business_id,
                    existing_conversation_id
                )));
            }
        }

        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("direct"),
            direct_chat_binding_request: Some(DirectChatBindingReplayRecord {
                bound_by: command.bound_by.clone(),
                binder_kind: binder_kind.into(),
                direct_chat_id: direct_chat_id.clone(),
                anchor_actor_id: pair.left_actor_id.clone(),
                anchor_actor_kind: command.left_actor_kind.clone(),
                peer_actor_id: pair.right_actor_id.clone(),
                peer_actor_kind: command.right_actor_kind.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        conversation
            .aggregate
            .replace_business_binding(Some(business_binding.clone()));
        let anchor_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, anchor_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&anchor_member));
        let peer_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, peer_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&peer_member));
        let creation_members = vec![
            (anchor_member.clone(), anchor_ordering_seq),
            (peer_member.clone(), peer_ordering_seq),
        ];
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            organization_id: command.organization_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.bound_by.clone(),
                actor_kind: binder_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": conversation_id.clone(),
                "conversationType": "direct",
                "businessType": business_binding.business_type.clone(),
                "businessId": business_binding.business_id.clone(),
                "directChat": {
                    "directChatId": direct_chat_id.clone(),
                    "anchorActorId": pair.left_actor_id.clone(),
                    "anchorActorKind": command.left_actor_kind.clone(),
                    "peerActorId": pair.right_actor_id.clone(),
                    "peerActorKind": command.right_actor_kind.clone(),
                    "pairHash": pair.pair_hash.clone()
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            creation_members.as_slice(),
            command.bound_by.as_str(),
            binder_kind,
        )?;
        state.conversations.insert(scope_key, conversation);
        state
            .business_index
            .insert(business_scope_key, conversation_id.clone());
        drop(state);

        Ok(CreateConversationResult::applied_with_request_key(
            conversation_id,
            event_id,
            request_key,
        ))
    }

    pub fn create_agent_dialog_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        agent_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_from_auth_context_with_requester_attributes(
            auth,
            conversation_id,
            agent_id,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_dialog_from_auth_context_with_requester_attributes(
        &self,
        auth: &AppContext,
        conversation_id: String,
        agent_id: String,
        requester_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind_and_attributes(
            CreateAgentDialogCommand::from_auth_context(auth, conversation_id, agent_id),
            auth.actor_kind.as_str(),
            requester_attributes,
        )
    }

    pub fn create_agent_dialog_with_requester_kind(
        &self,
        command: CreateAgentDialogCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind_and_attributes(
            command,
            requester_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_dialog_with_requester_kind_and_attributes(
        &self,
        command: CreateAgentDialogCommand,
        requester_kind: &str,
        requester_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "requesterId",
            command.requester_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "agentId",
            command.agent_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_standard_agent_id(command.agent_id.as_str())?;
        validate_payload_size("requesterKind", requester_kind, CONVERSATION_MAX_KIND_BYTES)?;
        policy::ensure_agent_dialog_requester_kind(requester_kind)?;
        let conversation_id = resolve_agent_dialog_conversation_id(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            requester_kind,
            command.requester_id.as_str(),
            command.agent_id.as_str(),
            command.conversation_id.as_str(),
        )?;
        let business_binding = ConversationBusinessBinding {
            business_type: "agent_dialog".into(),
            business_id: canonical_agent_dialog_business_id(
                requester_kind,
                command.requester_id.as_str(),
                command.agent_id.as_str(),
            ),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );
        let request_key = agent_dialog_create_request_key(
            command.tenant_id.as_str(),
            requester_kind,
            command.requester_id.as_str(),
            conversation_id.as_str(),
        );

        if command.requester_id == command.agent_id {
            return Err(RuntimeError::PermissionDenied(
                "agent dialog agent must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        );
        let mut requester_member_attributes =
            BTreeMap::from([("dialogRole".into(), "requester".into())]);
        requester_member_attributes.extend(requester_attributes);
        validate_member_attributes_payload_size(
            "requesterAttributes",
            &requester_member_attributes,
        )?;
        let requester_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            member_id(
                conversation_id.as_str(),
                requester_kind,
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            requester_member_attributes,
        );
        let agent_member_attributes = BTreeMap::from([
            ("agentId".into(), command.agent_id.clone()),
            ("dialogRole".into(), "assistant".into()),
            (
                "businessType".into(),
                business_binding.business_type.clone(),
            ),
            ("businessId".into(), business_binding.business_id.clone()),
        ]);
        validate_member_attributes_payload_size("agentAttributes", &agent_member_attributes)?;
        let agent_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            member_id(
                conversation_id.as_str(),
                "agent",
                command.agent_id.as_str(),
            ),
            command.agent_id.as_str(),
            "agent",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            agent_member_attributes,
        );

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation_id) = state.business_index.get(business_scope_key.as_str())
        {
            if existing_conversation_id != &conversation_id {
                return Err(RuntimeError::Conflict(format!(
                    "agent dialog binding already mapped to conversation {existing_conversation_id}"
                )));
            }
        }
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.agent_dialog_create_request.as_ref() {
                if agent_dialog_create_replay_matches(existing, &command, requester_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "agent dialog create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "agent dialog create request conflicts with existing conversation id: {conversation_id}"
            )));
        }

        let event_id = format!("evt_{}_created", conversation_id);
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("agent_dialog"),
            agent_dialog_create_request: Some(AgentDialogCreateReplayRecord {
                requester_id: command.requester_id.clone(),
                requester_kind: requester_kind.into(),
                agent_id: command.agent_id.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        conversation
            .aggregate
            .replace_business_binding(Some(business_binding.clone()));
        let requester_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, requester_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&requester_member),
        );
        let agent_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, agent_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&agent_member));
        let creation_members = vec![
            (requester_member.clone(), requester_ordering_seq),
            (agent_member.clone(), agent_ordering_seq),
        ];
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            organization_id: command.organization_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": conversation_id.clone(),
                "conversationType": "agent_dialog",
                "businessType": business_binding.business_type.clone(),
                "businessId": business_binding.business_id.clone(),
                "agentDialog": {
                    "agentId": command.agent_id.clone()
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            conversation_id.as_str(),
            creation_members.as_slice(),
            command.requester_id.as_str(),
            requester_kind,
        )?;
        state.conversations.insert(scope_key, conversation);
        state
            .business_index
            .insert(business_scope_key, conversation_id.clone());
        drop(state);

        Ok(CreateConversationResult::applied_with_request_key(
            conversation_id,
            event_id,
            request_key,
        ))
    }

    pub fn create_system_channel(
        &self,
        command: CreateSystemChannelCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind(command, "system")
    }

    pub fn create_system_channel_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        subscriber_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_from_auth_context_with_subscriber_attributes(
            auth,
            conversation_id,
            subscriber_id,
            BTreeMap::new(),
        )
    }

    pub fn create_system_channel_from_auth_context_with_subscriber_attributes(
        &self,
        auth: &AppContext,
        conversation_id: String,
        subscriber_id: String,
        subscriber_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind_and_subscriber_attributes(
            CreateSystemChannelCommand::from_auth_context(auth, conversation_id, subscriber_id),
            auth.actor_kind.as_str(),
            subscriber_attributes,
        )
    }

    pub fn create_system_channel_with_requester_kind(
        &self,
        command: CreateSystemChannelCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind_and_subscriber_attributes(
            command,
            requester_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_system_channel_with_requester_kind_and_subscriber_attributes(
        &self,
        command: CreateSystemChannelCommand,
        requester_kind: &str,
        subscriber_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "requesterId",
            command.requester_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "subscriberId",
            command.subscriber_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("requesterKind", requester_kind, CONVERSATION_MAX_KIND_BYTES)?;
        policy::ensure_system_channel_requester_kind(requester_kind)?;
        let request_key = system_channel_create_request_key(
            command.tenant_id.as_str(),
            requester_kind,
            command.requester_id.as_str(),
            command.conversation_id.as_str(),
        );

        if command.subscriber_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "system channel requires subscriber id".into(),
            ));
        }
        if command.requester_id == command.subscriber_id {
            return Err(RuntimeError::PermissionDenied(
                "system channel subscriber must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let publisher_member_attributes =
            BTreeMap::from([("channelRole".into(), "publisher".into())]);
        validate_member_attributes_payload_size(
            "publisherAttributes",
            &publisher_member_attributes,
        )?;
        let publisher_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                requester_kind,
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            publisher_member_attributes,
        );
        let mut subscriber_member_attributes =
            BTreeMap::from([("channelRole".into(), "subscriber".into())]);
        subscriber_member_attributes.extend(subscriber_attributes);
        validate_member_attributes_payload_size(
            "subscriberAttributes",
            &subscriber_member_attributes,
        )?;
        let subscriber_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                "user",
                command.subscriber_id.as_str(),
            ),
            command.subscriber_id.as_str(),
            "user",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            subscriber_member_attributes,
        );

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.system_channel_create_request.as_ref() {
                if system_channel_create_replay_matches(existing, &command, requester_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        command.conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "system channel create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "system channel create request conflicts with existing conversation id: {}",
                command.conversation_id
            )));
        }

        let event_id = format!("evt_{}_created", command.conversation_id);
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("system_channel"),
            system_channel_create_request: Some(SystemChannelCreateReplayRecord {
                requester_id: command.requester_id.clone(),
                requester_kind: requester_kind.into(),
                subscriber_id: command.subscriber_id.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        let publisher_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, publisher_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&publisher_member),
        );
        let subscriber_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, subscriber_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&subscriber_member),
        );
        let creation_members = vec![
            (publisher_member.clone(), publisher_ordering_seq),
            (subscriber_member.clone(), subscriber_ordering_seq),
        ];
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
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id.clone(),
                "conversationType": "system_channel",
                "systemChannel": {
                    "subscriberId": command.subscriber_id.clone()
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            creation_members.as_slice(),
            command.requester_id.as_str(),
            requester_kind,
        )?;
        state.conversations.insert(scope_key, conversation);
        drop(state);

        Ok(CreateConversationResult::applied_with_request_key(
            command.conversation_id,
            event_id,
            request_key,
        ))
    }

    pub fn create_agent_handoff(
        &self,
        command: CreateAgentHandoffCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind(command, "agent")
    }

    pub fn create_agent_handoff_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_from_auth_context_with_target_attributes(
            auth,
            conversation_id,
            target_id,
            target_kind,
            handoff_session_id,
            handoff_reason,
            BTreeMap::new(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_agent_handoff_from_auth_context_with_target_attributes(
        &self,
        auth: &AppContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
        target_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind_and_target_attributes(
            CreateAgentHandoffCommand::from_auth_context(
                auth,
                conversation_id,
                target_id,
                target_kind,
                handoff_session_id,
                handoff_reason,
            ),
            auth.actor_kind.as_str(),
            target_attributes,
        )
    }

    pub fn create_agent_handoff_with_source_kind(
        &self,
        command: CreateAgentHandoffCommand,
        source_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind_and_target_attributes(
            command,
            source_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_handoff_with_source_kind_and_target_attributes(
        &self,
        command: CreateAgentHandoffCommand,
        source_kind: &str,
        target_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "sourceId",
            command.source_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetId",
            command.target_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetKind",
            command.target_kind.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        validate_payload_size(
            "handoffSessionId",
            command.handoff_session_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "handoffReason",
            command.handoff_reason.as_deref(),
            CONVERSATION_MAX_REASON_BYTES,
        )?;
        validate_payload_size("sourceKind", source_kind, CONVERSATION_MAX_KIND_BYTES)?;
        policy::ensure_agent_handoff_source_kind(source_kind)?;
        policy::ensure_agent_handoff_target_kind(command.target_kind.as_str())?;
        let request_key = agent_handoff_create_request_key(
            command.tenant_id.as_str(),
            source_kind,
            command.source_id.as_str(),
            command.conversation_id.as_str(),
        );

        if command.handoff_session_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff requires handoff session id".into(),
            ));
        }
        if command.source_id == command.target_id {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff target must differ from source".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());

        let mut source_attributes = BTreeMap::from([
            ("handoffRole".into(), "source".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("targetId".into(), command.target_id.clone()),
            ("targetKind".into(), command.target_kind.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            source_attributes.insert("handoffReason".into(), reason);
        }
        validate_member_attributes_payload_size("sourceAttributes", &source_attributes)?;
        let source_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                source_kind,
                command.source_id.as_str(),
            ),
            command.source_id.as_str(),
            source_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            source_attributes,
        );

        let mut target_member_attributes = BTreeMap::from([
            ("handoffRole".into(), "target".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("sourceAgentId".into(), command.source_id.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            target_member_attributes.insert("handoffReason".into(), reason);
        }
        target_member_attributes.extend(target_attributes);
        validate_member_attributes_payload_size("targetAttributes", &target_member_attributes)?;
        let target_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.target_kind.as_str(),
                command.target_id.as_str(),
            ),
            command.target_id.as_str(),
            command.target_kind.as_str(),
            MembershipRole::Member,
            Some(command.source_id.clone()),
            created_at.clone(),
            target_member_attributes,
        );

        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.creation");
        if let Some(existing_conversation) = state.conversations.get(scope_key.as_str()) {
            if let Some(existing) = existing_conversation.agent_handoff_create_request.as_ref() {
                if agent_handoff_create_replay_matches(existing, &command, source_kind) {
                    return Ok(CreateConversationResult::replayed_with_request_key(
                        command.conversation_id,
                        existing.event_id.clone(),
                        request_key,
                    ));
                }
                return Err(RuntimeError::Conflict(format!(
                    "agent handoff create request conflicts with existing conversation idempotency key: {request_key}"
                )));
            }
            return Err(RuntimeError::Conflict(format!(
                "agent handoff create request conflicts with existing conversation id: {}",
                command.conversation_id
            )));
        }

        let event_id = format!("evt_{}_created", command.conversation_id);
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new_agent_handoff(AgentHandoffStateView {
                tenant_id: command.tenant_id.clone(),
                conversation_id: command.conversation_id.clone(),
                status: "open".into(),
                source: ChangeAgentHandoffStatusView {
                    id: command.source_id.clone(),
                    kind: source_kind.into(),
                },
                target: ChangeAgentHandoffStatusView {
                    id: command.target_id.clone(),
                    kind: command.target_kind.clone(),
                },
                handoff_session_id: command.handoff_session_id.clone(),
                handoff_reason: command.handoff_reason.clone(),
                accepted_at: None,
                accepted_by: None,
                resolved_at: None,
                resolved_by: None,
                closed_at: None,
                closed_by: None,
            }),
            agent_handoff_create_request: Some(AgentHandoffCreateReplayRecord {
                source_id: command.source_id.clone(),
                source_kind: source_kind.into(),
                target_id: command.target_id.clone(),
                target_kind: command.target_kind.clone(),
                handoff_session_id: command.handoff_session_id.clone(),
                handoff_reason: command.handoff_reason.clone(),
                event_id: event_id.clone(),
            }),
            ..ConversationState::default()
        };
        let source_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, source_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&source_member));
        let target_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, target_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&target_member));
        let creation_members = vec![
            (source_member.clone(), source_ordering_seq),
            (target_member.clone(), target_ordering_seq),
        ];
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
                actor_id: command.source_id.clone(),
                actor_kind: source_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id.clone(),
                "conversationType": "agent_handoff",
                "source": {
                    "id": command.source_id.clone(),
                    "kind": source_kind
                },
                "target": {
                    "id": command.target_id.clone(),
                    "kind": command.target_kind.clone()
                },
                "handoff": {
                    "sessionId": command.handoff_session_id.clone(),
                    "reason": command.handoff_reason.clone(),
                    "status": "open"
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        append_conversation_creation_batch(
            &self.journal,
            envelope,
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            creation_members.as_slice(),
            command.source_id.as_str(),
            source_kind,
        )?;
        state.conversations.insert(scope_key, conversation);
        drop(state);

        Ok(CreateConversationResult::applied_with_request_key(
            command.conversation_id,
            event_id,
            request_key,
        ))
    }
}

fn append_conversation_creation_batch<J>(
    journal: &J,
    created_envelope: CommitEnvelope,
    tenant_id: &str,
    conversation_id: &str,
    members: &[(ConversationMember, u64)],
    actor_id: &str,
    actor_kind: &str,
) -> Result<(), RuntimeError>
where
    J: CommitJournal,
{
    let organization_id = created_envelope.organization_id.clone();
    let mut envelopes = Vec::with_capacity(members.len() + 1);
    envelopes.push(created_envelope);
    for (member, ordering_seq) in members {
        envelopes.push(build_member_envelope(
            tenant_id,
            organization_id.as_str(),
            conversation_id,
            "conversation.member_joined",
            member.clone(),
            *ordering_seq,
            "standard",
            actor_id,
            actor_kind,
        ));
    }
    journal.append_batch(envelopes)?;
    Ok(())
}
