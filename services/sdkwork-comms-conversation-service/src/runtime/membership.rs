use super::*;
use std::collections::BTreeMap;

const SHARED_HISTORY_LINK_ATTRIBUTE_KEYS: [&str; 3] = [
    "sharedChannelPolicyId",
    "externalConnectionId",
    "externalMemberId",
];
const SHARED_CHANNEL_SYNC_REQUEST_KEY_ATTRIBUTE: &str = "sharedChannelSyncRequestKey";

fn has_non_empty_shared_history_link_value(
    attributes: &BTreeMap<String, String>,
    key: &str,
) -> bool {
    attributes
        .get(key)
        .is_some_and(|value| !value.trim().is_empty())
}

fn resolve_shared_history_linked_member(
    attributes: &BTreeMap<String, String>,
) -> Result<bool, RuntimeError> {
    let present_count = SHARED_HISTORY_LINK_ATTRIBUTE_KEYS
        .iter()
        .filter(|key| has_non_empty_shared_history_link_value(attributes, key))
        .count();
    if present_count == 0 {
        return Ok(false);
    }
    if present_count != SHARED_HISTORY_LINK_ATTRIBUTE_KEYS.len() {
        return Err(RuntimeError::InvalidInput(
            "shared history external-linked member requires sharedChannelPolicyId, externalConnectionId, and externalMemberId".into(),
        ));
    }

    Ok(true)
}

fn shared_history_link_attributes(
    shared_channel_policy_id: &str,
    external_connection_id: &str,
    external_member_id: &str,
    request_key: &str,
) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "sharedChannelPolicyId".into(),
            shared_channel_policy_id.into(),
        ),
        ("externalConnectionId".into(), external_connection_id.into()),
        ("externalMemberId".into(), external_member_id.into()),
        (
            SHARED_CHANNEL_SYNC_REQUEST_KEY_ATTRIBUTE.into(),
            request_key.into(),
        ),
    ])
}

fn shared_history_link_matches(
    member: &ConversationMember,
    command: &SyncSharedChannelLinkedMemberCommand,
) -> bool {
    member.role == MembershipRole::Guest
        && member.state == MembershipState::Linked
        && member.principal_kind == command.local_actor_kind
        && member
            .attributes
            .get("sharedChannelPolicyId")
            .map(String::as_str)
            == Some(command.shared_channel_policy_id.as_str())
        && member
            .attributes
            .get("externalConnectionId")
            .map(String::as_str)
            == Some(command.external_connection_id.as_str())
        && member
            .attributes
            .get("externalMemberId")
            .map(String::as_str)
            == Some(command.external_member_id.as_str())
}

fn shared_channel_sync_request_key_fence(member: &ConversationMember) -> Option<&str> {
    member
        .attributes
        .get(SHARED_CHANNEL_SYNC_REQUEST_KEY_ATTRIBUTE)
        .map(String::as_str)
        .filter(|request_key| !request_key.trim().is_empty())
}

fn shared_channel_sync_request_key(command: &SyncSharedChannelLinkedMemberCommand) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        command.tenant_id,
        command.conversation_id,
        command.shared_channel_policy_id,
        command.external_connection_id,
        command.local_actor_id,
        command.local_actor_kind,
        command.external_member_id
    )
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn list_members_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.list_members(auth.tenant_id.as_str(), organization_id_from_auth_context(auth).as_str(), conversation_id)
    }

    pub fn list_members_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<ListMembersResult, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.list_members_window(auth.tenant_id.as_str(), organization_id_from_auth_context(auth).as_str(), conversation_id, limit, cursor)
    }

    pub fn list_messages_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        self.list_messages_with_actor_kind(auth.tenant_id.as_str(), organization_id_from_auth_context(auth).as_str(), conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            after_seq,
            limit,
        )
    }

    pub fn read_cursor_view_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        self.read_cursor_view_with_actor_kind(auth.tenant_id.as_str(), "default", conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
    }

    pub fn add_member_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: MembershipRole,
        attributes: BTreeMap<String, String>,
    ) -> Result<ConversationMember, RuntimeError> {
        self.add_member_with_actor_kind_and_attributes(
            AddConversationMemberCommand::from_auth_context(
                auth,
                conversation_id,
                principal_id,
                principal_kind,
                role,
            ),
            auth.actor_kind.as_str(),
            attributes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn sync_shared_channel_linked_member_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        shared_channel_policy_id: String,
        external_connection_id: String,
        local_actor_id: String,
        local_actor_kind: String,
        external_member_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.sync_shared_channel_linked_member_from_auth_context_with_result(
            auth,
            conversation_id,
            shared_channel_policy_id,
            external_connection_id,
            local_actor_id,
            local_actor_kind,
            external_member_id,
        )
        .map(|result| result.member)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn sync_shared_channel_linked_member_from_auth_context_with_result(
        &self,
        auth: &AppContext,
        conversation_id: String,
        shared_channel_policy_id: String,
        external_connection_id: String,
        local_actor_id: String,
        local_actor_kind: String,
        external_member_id: String,
    ) -> Result<SyncSharedChannelLinkedMemberResult, RuntimeError> {
        self.sync_shared_channel_linked_member_with_requester_kind_with_result(
            SyncSharedChannelLinkedMemberCommand::from_auth_context(
                auth,
                conversation_id,
                shared_channel_policy_id,
                external_connection_id,
                local_actor_id,
                local_actor_kind,
                external_member_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn sync_shared_channel_linked_member(
        &self,
        command: SyncSharedChannelLinkedMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        self.sync_shared_channel_linked_member_with_requester_kind_with_result(command, "system")
            .map(|result| result.member)
    }

    pub fn add_member(
        &self,
        command: AddConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.invited_by.as_str(),
            )?
            .principal_kind;
        self.add_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn add_member_with_actor_kind(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.add_member_with_actor_kind_and_attributes(command, actor_kind, BTreeMap::new())
    }

    pub fn sync_shared_channel_linked_member_with_requester_kind(
        &self,
        command: SyncSharedChannelLinkedMemberCommand,
        requester_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.sync_shared_channel_linked_member_with_requester_kind_with_result(
            command,
            requester_kind,
        )
        .map(|result| result.member)
    }

    pub fn sync_shared_channel_linked_member_with_requester_kind_with_result(
        &self,
        command: SyncSharedChannelLinkedMemberCommand,
        requester_kind: &str,
    ) -> Result<SyncSharedChannelLinkedMemberResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "sharedChannelPolicyId",
            command.shared_channel_policy_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "externalConnectionId",
            command.external_connection_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "localActorId",
            command.local_actor_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "localActorKind",
            command.local_actor_kind.as_str(),
            CONVERSATION_MAX_KIND_BYTES,
        )?;
        validate_payload_size(
            "externalMemberId",
            command.external_member_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "syncedBy",
            command.synced_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("requesterKind", requester_kind, CONVERSATION_MAX_KIND_BYTES)?;
        policy::ensure_shared_channel_sync_requester_kind(requester_kind)?;

        if command.conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires conversation_id".into(),
            ));
        }
        if command.shared_channel_policy_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires shared_channel_policy_id".into(),
            ));
        }
        if command.external_connection_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires external_connection_id".into(),
            ));
        }
        if command.local_actor_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires local_actor_id".into(),
            ));
        }
        if command.local_actor_kind.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires local_actor_kind".into(),
            ));
        }
        if command.external_member_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires external_member_id".into(),
            ));
        }
        if command.synced_by.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires synced_by actor identity".into(),
            ));
        }

        let request_key = shared_channel_sync_request_key(&command);
        let attributes = shared_history_link_attributes(
            command.shared_channel_policy_id.as_str(),
            command.external_connection_id.as_str(),
            command.external_member_id.as_str(),
            request_key.as_str(),
        );
        validate_member_attributes_payload_size("memberAttributes", &attributes)?;
        resolve_shared_history_linked_member(&attributes)?;

        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let history_visibility = conversation
                .aggregate
                .policy()
                .map(|policy| policy.history_visibility.as_str())
                .unwrap_or("joined");
            if history_visibility != "shared" {
                return Err(RuntimeError::InvalidInput(format!(
                    "shared channel linked-member sync requires history_visibility=shared, got {history_visibility}"
                )));
            }

            if let Some(current_member) = conversation.roster.resolve_current_member_with_kind(
                command.local_actor_id.as_str(),
                command.local_actor_kind.as_str(),
            ) {
                if shared_history_link_matches(&current_member, &command) {
                    let mut member = current_member;
                    if shared_channel_sync_request_key_fence(&member).is_none() {
                        member.attributes.insert(
                            SHARED_CHANNEL_SYNC_REQUEST_KEY_ATTRIBUTE.to_owned(),
                            request_key.clone(),
                        );
                    }
                    let status = if shared_channel_sync_request_key_fence(&member)
                        == Some(request_key.as_str())
                    {
                        SyncSharedChannelLinkedMemberStatus::Replayed
                    } else {
                        SyncSharedChannelLinkedMemberStatus::AlreadyLinked
                    };
                    return Ok(SyncSharedChannelLinkedMemberResult { status, member });
                }

                return Err(RuntimeError::Conflict(format!(
                    "principal {} is already materialized as conversation member {} with incompatible shared-channel link truth",
                    command.local_actor_id, current_member.member_id
                )));
            }

            let member_episode = next_member_episode(
                conversation,
                command.local_actor_id.as_str(),
                command.local_actor_kind.as_str(),
            );
            let mut member = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_episode_id(
                    command.conversation_id.as_str(),
                    command.local_actor_kind.as_str(),
                    command.local_actor_id.as_str(),
                    member_episode,
                ),
                command.local_actor_id.as_str(),
                command.local_actor_kind.as_str(),
                MembershipRole::Guest,
                Some(command.synced_by.clone()),
                conversation_timestamp(),
                attributes,
            );
            member.state = MembershipState::Linked;

            let ordering_seq = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let envelope = build_member_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                "conversation.member_joined",
                member.clone(),
                ordering_seq,
                retention_class.as_str(),
                command.synced_by.as_str(),
                requester_kind,
            );

            self.journal.append(envelope)?;
            upsert_member(conversation, member.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&member));
            member
        };

        Ok(SyncSharedChannelLinkedMemberResult {
            status: SyncSharedChannelLinkedMemberStatus::Applied,
            member,
        })
    }

    fn add_member_with_actor_kind_and_attributes(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
        attributes: BTreeMap<String, String>,
    ) -> Result<ConversationMember, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
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
        validate_payload_size(
            "invitedBy",
            command.invited_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        validate_member_attributes_payload_size("memberAttributes", &attributes)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let invited_by_member = resolve_active_member_with_kind(
                conversation,
                command.invited_by.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&invited_by_member, actor_kind)?;
            policy::ensure_member_add_actor_allowed(conversation, &invited_by_member)?;
            let history_visibility = conversation
                .aggregate
                .policy()
                .map(|policy| policy.history_visibility.as_str())
                .unwrap_or("joined");

            if conversation
                .roster
                .resolve_current_member_with_kind(
                    command.principal_id.as_str(),
                    command.principal_kind.as_str(),
                )
                .is_some()
            {
                return Err(RuntimeError::MemberAlreadyExists(command.principal_id));
            }
            policy::ensure_member_add_request_allowed(
                conversation,
                &invited_by_member,
                &command.role,
            )?;
            let member_episode = next_member_episode(
                conversation,
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
            );
            let shared_history_linked = resolve_shared_history_linked_member(&attributes)?;

            let mut member = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_episode_id(
                    command.conversation_id.as_str(),
                    command.principal_kind.as_str(),
                    command.principal_id.as_str(),
                    member_episode,
                ),
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
                command.role,
                Some(command.invited_by.clone()),
                conversation_timestamp(),
                attributes,
            );
            if history_visibility == "invited" {
                member.state = MembershipState::Invited;
            } else if history_visibility == "shared" && shared_history_linked {
                member.state = MembershipState::Linked;
            }

            let member_epoch = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let actor_kind = invited_by_member.principal_kind.clone();
            let envelope = build_member_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                "conversation.member_joined",
                member.clone(),
                member_epoch,
                retention_class.as_str(),
                command.invited_by.as_str(),
                actor_kind.as_str(),
            );

            self.journal.append(envelope)?;
            upsert_member(conversation, member.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&member));
            member
        };

        Ok(member)
    }

    pub fn remove_member(
        &self,
        command: RemoveConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.removed_by.as_str(),
            )?
            .principal_kind;
        self.remove_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn remove_member_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        member_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.remove_member_with_actor_kind(
            RemoveConversationMemberCommand::from_auth_context(auth, conversation_id, member_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn remove_member_with_actor_kind(
        &self,
        command: RemoveConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "memberId",
            command.member_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "removedBy",
            command.removed_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let removed_by_member = resolve_active_member_with_kind(
                conversation,
                command.removed_by.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&removed_by_member, actor_kind)?;

            let mut member = conversation
                .roster
                .member(command.member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.member_id.clone()))?;
            policy::ensure_current_active_member_target(conversation, &member)?;
            policy::ensure_member_remove_allowed(conversation, &removed_by_member, &member)?;
            member.state = MembershipState::Removed;
            member.removed_at = Some(conversation_timestamp());

            let member_epoch = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let actor_kind = removed_by_member.principal_kind.clone();
            let envelope = build_member_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                "conversation.member_removed",
                member.clone(),
                member_epoch,
                retention_class.as_str(),
                command.removed_by.as_str(),
                actor_kind.as_str(),
            );

            self.journal.append(envelope)?;
            conversation.roster.deactivate_member(member.clone());
            member
        };

        Ok(member)
    }

    pub fn leave_conversation(
        &self,
        command: LeaveConversationCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.leave_conversation_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn leave_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.leave_conversation_with_actor_kind(
            LeaveConversationCommand::from_auth_context(auth, conversation_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn leave_conversation_with_actor_kind(
        &self,
        command: LeaveConversationCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "principalId",
            command.principal_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let leaving_member = resolve_active_member_with_kind(
                conversation,
                command.principal_id.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&leaving_member, actor_kind)?;
            policy::ensure_member_leave_allowed(conversation, &leaving_member)?;

            let mut member = leaving_member.clone();
            member.state = MembershipState::Left;
            member.removed_at = Some(conversation_timestamp());

            let member_epoch = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let actor_kind = leaving_member.principal_kind.clone();
            let envelope = build_member_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                "conversation.member_left",
                member.clone(),
                member_epoch,
                retention_class.as_str(),
                command.principal_id.as_str(),
                actor_kind.as_str(),
            );

            self.journal.append(envelope)?;
            conversation.roster.deactivate_member(member.clone());
            member
        };

        Ok(member)
    }

    pub fn transfer_conversation_owner(
        &self,
        command: TransferConversationOwnerCommand,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.transferred_by.as_str(),
            )?
            .principal_kind;
        self.transfer_conversation_owner_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn transfer_conversation_owner_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        target_member_id: String,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        self.transfer_conversation_owner_with_actor_kind(
            TransferConversationOwnerCommand::from_auth_context(
                auth,
                conversation_id,
                target_member_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn transfer_conversation_owner_with_actor_kind(
        &self,
        command: TransferConversationOwnerCommand,
        actor_kind: &str,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetMemberId",
            command.target_member_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "transferredBy",
            command.transferred_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let result = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let owner_member = resolve_active_member_with_kind(
                conversation,
                command.transferred_by.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&owner_member, actor_kind)?;
            let target_member = conversation
                .roster
                .member(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            policy::ensure_owner_transfer_allowed(conversation, &owner_member, &target_member)?;

            let transferred_at = conversation_timestamp();
            let actor_kind = owner_member.principal_kind.clone();
            let previous_owner = ConversationMember {
                role: MembershipRole::Admin,
                ..owner_member
            };
            let new_owner = ConversationMember {
                role: MembershipRole::Owner,
                ..target_member
            };

            let ordering_seq = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let payload = TransferConversationOwnerPayload {
                tenant_id: command.tenant_id.clone(),
                organization_id: command.organization_id.clone(),
                conversation_id: command.conversation_id.clone(),
                previous_owner,
                new_owner,
                transferred_at,
            };
            let event = build_owner_transfer_envelope(
                payload.clone(),
                ordering_seq,
                retention_class.as_str(),
                command.transferred_by.as_str(),
                actor_kind.as_str(),
            );

            self.journal.append(event.clone())?;
            conversation
                .roster
                .upsert_member(payload.previous_owner.clone());
            conversation.roster.upsert_member(payload.new_owner.clone());

            TransferConversationOwnerResult {
                event_id: event.event_id,
                transferred_at: payload.transferred_at.clone(),
                previous_owner: payload.previous_owner,
                new_owner: payload.new_owner,
            }
        };

        Ok(result)
    }

    pub fn change_conversation_member_role(
        &self,
        command: ChangeConversationMemberRoleCommand,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.changed_by.as_str(),
            )?
            .principal_kind;
        self.change_conversation_member_role_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn change_conversation_member_role_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        target_member_id: String,
        new_role: MembershipRole,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        self.change_conversation_member_role_with_actor_kind(
            ChangeConversationMemberRoleCommand::from_auth_context(
                auth,
                conversation_id,
                target_member_id,
                new_role,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn change_conversation_member_role_with_actor_kind(
        &self,
        command: ChangeConversationMemberRoleCommand,
        actor_kind: &str,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetMemberId",
            command.target_member_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "changedBy",
            command.changed_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let result = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let actor_member = resolve_active_member_with_kind(
                conversation,
                command.changed_by.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let target_member = conversation
                .roster
                .member(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            policy::ensure_current_active_member_target(conversation, &target_member)?;
            policy::ensure_member_role_change_allowed(
                conversation,
                &actor_member,
                &target_member,
                &command.new_role,
            )?;

            let changed_at = conversation_timestamp();
            let previous_member = target_member.clone();
            let updated_member = ConversationMember {
                role: command.new_role.clone(),
                ..target_member
            };
            let ordering_seq = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);
            let actor_kind = actor_member.principal_kind.clone();
            let payload = ChangeConversationMemberRolePayload {
                tenant_id: command.tenant_id.clone(),
                organization_id: command.organization_id.clone(),
                conversation_id: command.conversation_id.clone(),
                previous_member,
                updated_member,
                changed_at,
            };
            let event = build_member_role_changed_envelope(
                payload.clone(),
                ordering_seq,
                retention_class.as_str(),
                command.changed_by.as_str(),
                actor_kind.as_str(),
            );

            self.journal.append(event.clone())?;
            conversation
                .roster
                .upsert_member(payload.updated_member.clone());

            ChangeConversationMemberRoleResult {
                event_id: event.event_id,
                changed_at: payload.changed_at.clone(),
                previous_member: payload.previous_member,
                updated_member: payload.updated_member,
            }
        };

        Ok(result)
    }

    pub fn list_members(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        Ok(conversation
            .roster
            .members()
            .values()
            .filter(|member| member.is_active())
            .cloned()
            .collect())
    }

    pub fn list_members_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<ListMembersResult, RuntimeError> {
        let limit = normalize_member_list_limit(limit).map_err(RuntimeError::InvalidInput)?;
        let offset = parse_member_list_cursor(cursor)?;
        let items = self.list_members(tenant_id, organization_id, conversation_id)?;
        Ok(member_list_window(items, offset, limit))
    }

    pub fn update_read_cursor(
        &self,
        command: UpdateReadCursorCommand,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let actor_kind = self
            .require_active_member(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.update_read_cursor_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn update_read_cursor_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        read_seq: u64,
        last_read_message_id: Option<String>,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        self.update_read_cursor_with_actor_kind(
            UpdateReadCursorCommand::from_auth_context(
                auth,
                conversation_id,
                read_seq,
                last_read_message_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn update_read_cursor_with_actor_kind(
        &self,
        command: UpdateReadCursorCommand,
        actor_kind: &str,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "principalId",
            command.principal_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "lastReadMessageId",
            command.last_read_message_id.as_deref(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let cursor = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            if command.read_seq > conversation.message_log.high_watermark() {
                return Err(RuntimeError::ReadCursorInvalid(format!(
                    "read cursor exceeds conversation high watermark: {} > {}",
                    command.read_seq,
                    conversation.message_log.high_watermark()
                )));
            }

            let actor_member = resolve_active_member_with_kind(
                conversation,
                command.principal_id.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let retention_class = conversation_retention_class(conversation);
            let member_id = actor_member.member_id.clone();
            let read_cursors = conversation.roster.read_cursors();
            let cursor_missing = !read_cursors.contains_key(member_id.as_str());
            let cursor = read_cursors
                .get(member_id.as_str())
                .cloned()
                .unwrap_or_else(|| ConversationReadCursor {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    member_id: member_id.clone(),
                    principal_id: command.principal_id.clone(),
                    principal_kind: actor_member.principal_kind.clone(),
                    read_seq: 0,
                    last_read_message_id: None,
                    updated_at: conversation_timestamp(),
                });

            if command.read_seq > cursor.read_seq {
                let updated_cursor = ConversationReadCursor {
                    read_seq: command.read_seq,
                    last_read_message_id: command.last_read_message_id.clone(),
                    updated_at: conversation_timestamp(),
                    ..cursor
                };
                self.journal.append(build_read_cursor_envelope(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                    updated_cursor.clone(),
                    updated_cursor.read_seq,
                    retention_class.as_str(),
                    command.principal_id.as_str(),
                    actor_member.principal_kind.as_str(),
                ))?;
                conversation
                    .roster
                    .read_cursors_mut()
                    .insert(member_id, updated_cursor.clone());
                updated_cursor
            } else {
                if cursor_missing {
                    conversation
                        .roster
                        .read_cursors_mut()
                        .insert(member_id, cursor.clone());
                }
                cursor
            }
        };

        Ok(cursor)
    }

    pub fn read_cursor_view(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        let cursor = conversation
            .roster
            .read_cursors()
            .get(member_id.as_str())
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })?;

        Ok(ConversationReadCursorView::from_cursor(
            cursor,
            conversation.message_log.received_unread_count_since(
                cursor.read_seq,
                cursor.principal_id.as_str(),
                cursor.principal_kind.as_str(),
            ),
        ))
    }

    pub fn read_cursor_view_with_actor_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id =
            resolve_active_member_id_with_kind(conversation, principal_id, principal_kind)?;
        let cursor = conversation
            .roster
            .read_cursors()
            .get(member_id.as_str())
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_kind}:{principal_id}"
                ))
            })?;

        Ok(ConversationReadCursorView::from_cursor(
            cursor,
            conversation.message_log.received_unread_count_since(
                cursor.read_seq,
                cursor.principal_id.as_str(),
                cursor.principal_kind.as_str(),
            ),
        ))
    }

    pub fn list_messages_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        let limit = validate_message_history_limit(limit)?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        policy::ensure_history_read_allowed(conversation, principal_id)?;

        Ok(message_history_window(
            conversation
                .message_log
                .message_window_after(after_seq.unwrap_or_default(), limit),
            after_seq,
        ))
    }

    pub fn list_messages_with_actor_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        let limit = validate_message_history_limit(limit)?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        policy::ensure_history_read_allowed_with_kind(conversation, principal_id, principal_kind)?;

        Ok(message_history_window(
            conversation
                .message_log
                .message_window_after(after_seq.unwrap_or_default(), limit),
            after_seq,
        ))
    }

    pub fn stored_message_from_auth_context(
        &self,
        auth: &AppContext,
        message_id: &str,
    ) -> Result<im_domain_core::message::StoredMessage, RuntimeError> {
        let organization_id = organization_id_from_auth_context(auth);
        let conversation_id = {
            let state = read_runtime_state(
                &self.state,
                "conversation-runtime.state.stored_message.locate",
            );
            state
                .message_locator
                .conversation_id(auth.tenant_id.as_str(), message_id)
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(message_id.into()))?
        };
        self.ensure_conversation_loaded(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let state = read_runtime_state(&self.state, "conversation-runtime.state.stored_message");
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id.as_str(),
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.clone()))?;
        policy::ensure_history_read_allowed_with_kind(
            conversation,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )?;
        conversation
            .message_log
            .message(message_id)
            .cloned()
            .ok_or_else(|| RuntimeError::MessageNotFound(message_id.into()))
    }

    pub fn list_pinned_message_ids_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<ListPinnedMessagesResult, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        let organization_id = organization_id_from_auth_context(auth);
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        );
        let state = read_runtime_state(&self.state, "conversation-runtime.state.pinned_messages");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let pinned_ids = conversation
            .message_log
            .messages_in_order()
            .into_iter()
            .filter(|stored| stored.pin.is_some())
            .map(|stored| stored.message.message_id)
            .collect::<Vec<_>>();
        let offset = parse_member_list_cursor(cursor)?;
        let limit = limit.max(1);
        if offset > pinned_ids.len() {
            return Ok(ListPinnedMessagesResult {
                message_ids: Vec::new(),
                next_cursor: None,
                has_more: false,
            });
        }
        let mut window = pinned_ids
            .into_iter()
            .skip(offset)
            .take(limit + 1)
            .collect::<Vec<_>>();
        let has_more = window.len() > limit;
        if has_more {
            window.truncate(limit);
        }
        let next_cursor = has_more.then(|| (offset + window.len()).to_string());
        Ok(ListPinnedMessagesResult {
            message_ids: window,
            next_cursor,
            has_more,
        })
    }

    pub fn retrieve_inbox_from_auth_context(
        &self,
        auth: &AppContext,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<InboxRetrieveResult, RuntimeError> {
        let state = read_runtime_state(&self.state, "conversation-runtime.state.inbox");
        let mut conversation_ids = state
            .conversations
            .values()
            .filter_map(|conversation| {
                conversation
                    .roster
                    .resolve_active_member_id_with_kind(
                        auth.actor_id.as_str(),
                        auth.actor_kind.as_str(),
                    )
                    .and_then(|_| {
                        conversation
                            .roster
                            .members()
                            .values()
                            .next()
                            .map(|member| member.conversation_id.clone())
                    })
            })
            .collect::<Vec<_>>();
        conversation_ids.sort();
        conversation_ids.dedup();
        let offset = parse_member_list_cursor(cursor)?;
        let limit = limit.max(1);
        if offset > conversation_ids.len() {
            return Ok(InboxRetrieveResult {
                conversation_ids: Vec::new(),
                next_cursor: None,
                has_more: false,
            });
        }
        let mut window = conversation_ids
            .into_iter()
            .skip(offset)
            .take(limit + 1)
            .collect::<Vec<_>>();
        let has_more = window.len() > limit;
        if has_more {
            window.truncate(limit);
        }
        let next_cursor = has_more.then(|| (offset + window.len()).to_string());
        Ok(InboxRetrieveResult {
            conversation_ids: window,
            next_cursor,
            has_more,
        })
    }
}

fn validate_message_history_limit(limit: usize) -> Result<usize, RuntimeError> {
    normalize_message_history_limit(Some(limit)).map_err(RuntimeError::InvalidInput)
}

fn parse_member_list_cursor(cursor: Option<&str>) -> Result<usize, RuntimeError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(0);
    };
    cursor.parse::<usize>().map_err(|_| {
        RuntimeError::InvalidInput(format!(
            "conversation member list cursor is invalid: {cursor}"
        ))
    })
}

fn member_list_window(
    items: Vec<ConversationMember>,
    offset: usize,
    limit: usize,
) -> ListMembersResult {
    if offset > items.len() {
        return ListMembersResult {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
        };
    }

    let mut window = items
        .into_iter()
        .skip(offset)
        .take(limit + 1)
        .collect::<Vec<_>>();
    let has_more = window.len() > limit;
    if has_more {
        window.truncate(limit);
    }
    let next_cursor = has_more.then(|| (offset + window.len()).to_string());

    ListMembersResult {
        items: window,
        next_cursor,
        has_more,
    }
}

fn message_history_window(
    window: im_domain_core::message::MessageHistoryWindow,
    after_seq: Option<u64>,
) -> MessageHistoryResult {
    let _ = after_seq;
    MessageHistoryResult {
        items: window.items,
        high_watermark: window.high_watermark,
        next_after_seq: window.next_after_seq,
        has_more: window.has_more,
    }
}
