use super::group_lifecycle::ensure_conversation_write_allowed;
use super::member_list_cursor::{
    MemberListCursorError, MemberListCursorScope, decode_member_list_cursor,
    encode_member_list_cursor,
};
use super::support::{ReadCursorEnvelopeInput, deactivate_roster_member, upsert_roster_member};
use super::*;
use im_platform_contracts::ConversationMemberPageCursor;
use sdkwork_utils_rust::{PageInfo, PageMode, cursor_list_page_data};
use std::collections::BTreeMap;

const SHARED_HISTORY_LINK_ATTRIBUTE_KEYS: [&str; 3] = [
    "sharedChannelPolicyId",
    "externalConnectionId",
    "externalMemberId",
];
const SHARED_CHANNEL_SYNC_REQUEST_KEY_ATTRIBUTE: &str = "sharedChannelSyncRequestKey";
const READ_CURSOR_JOURNAL_APPEND_MAX_ATTEMPTS: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct MessageHistoryReadRequest<'a> {
    tenant_id: &'a str,
    organization_id: &'a str,
    conversation_id: &'a str,
    principal_id: &'a str,
    principal_kind: Option<&'a str>,
    before_seq: Option<u64>,
    limit: usize,
}

impl<'a> MessageHistoryReadRequest<'a> {
    pub fn new(
        tenant_id: &'a str,
        organization_id: &'a str,
        conversation_id: &'a str,
        principal_id: &'a str,
        principal_kind: &'a str,
        before_seq: Option<u64>,
        limit: usize,
    ) -> Self {
        Self {
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind: Some(principal_kind),
            before_seq,
            limit,
        }
    }

    fn without_actor_kind(
        tenant_id: &'a str,
        organization_id: &'a str,
        conversation_id: &'a str,
        principal_id: &'a str,
        before_seq: Option<u64>,
        limit: usize,
    ) -> Self {
        Self {
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind: None,
            before_seq,
            limit,
        }
    }
}

fn is_journal_position_conflict(error: &RuntimeError) -> bool {
    let message = match error {
        RuntimeError::Contract(ContractError::Conflict(message))
        | RuntimeError::Conflict(message) => message.as_str(),
        _ => return false,
    };
    message.contains("journal position") && message.contains("already occupied")
}

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
        self.list_members(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
        )
    }

    pub fn list_members_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        page_size: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<ListMembersResult, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.list_members_window(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
            page_size,
            cursor,
        )
    }

    pub fn list_member_directory_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        page_size: Option<usize>,
        cursor: Option<&str>,
        query: &str,
    ) -> Result<ListMembersResult, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        let limit = normalize_member_list_limit(page_size).map_err(RuntimeError::InvalidInput)?;
        let offset = parse_member_list_cursor(cursor)?;
        let organization_id = organization_id_from_auth_context(auth);
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        );
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let window = conversation
            .roster
            .list_active_members_window_filtered(offset, limit, query);
        Ok(cursor_list_page_data(
            window.items,
            limit,
            window.next_offset.map(|value| value.to_string()),
            window.has_more,
        ))
    }

    pub fn list_messages_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        before_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        self.list_messages_with_actor_kind(MessageHistoryReadRequest::new(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            before_seq,
            limit,
        ))
    }

    pub fn read_cursor_view_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        self.read_cursor_view_with_actor_kind_and_device(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            auth.device_id.as_deref(),
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

    pub fn accept_conversation_invitation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        validate_payload_size(
            "conversationId",
            conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id.as_str(),
        );
        let organization_id = organization_id_from_auth_context(auth);
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let member = {
                let conversation = state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.clone()))?;
                ensure_conversation_write_allowed(conversation)?;
                let mut invited_member = conversation
                    .roster
                    .resolve_current_member_with_kind(
                        auth.actor_id.as_str(),
                        auth.actor_kind.as_str(),
                    )
                    .ok_or_else(|| {
                        RuntimeError::MemberNotFound(format!(
                            "invited member {}:{} not found in conversation {}",
                            auth.actor_kind, auth.actor_id, conversation_id
                        ))
                    })?;
                if invited_member.state != MembershipState::Invited {
                    return Err(RuntimeError::Conflict(format!(
                        "conversation member {} is not awaiting invitation acceptance (state={:?})",
                        invited_member.member_id, invited_member.state
                    )));
                }
                invited_member.state = MembershipState::Joined;
                let accepted_at = conversation_timestamp();
                invited_member.joined_at = accepted_at.clone();

                let member_epoch = conversation.aggregate.next_member_epoch();
                let retention_class = conversation_retention_class(conversation);
                let envelope = build_member_envelope(
                    auth.tenant_id.as_str(),
                    organization_id.as_str(),
                    conversation_id.as_str(),
                    "conversation.member_invitation_accepted",
                    invited_member.clone(),
                    member_epoch,
                    retention_class.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                );

                self.journal.append(envelope)?;
                upsert_roster_member(conversation, invited_member.clone());
                invited_member
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &member);
            member
        };
        Ok(member)
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
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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

    pub fn add_member_with_actor_kind_and_attributes(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
        attributes: BTreeMap<String, String>,
    ) -> Result<ConversationMember, RuntimeError> {
        self.add_member_with_actor_kind_and_attributes_inner(command, actor_kind, attributes)
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

        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let member = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                upsert_roster_member(conversation, member.clone());
                upsert_read_cursor(conversation, build_default_read_cursor(&member));
                member
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &member);
            member
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );

        Ok(SyncSharedChannelLinkedMemberResult {
            status: SyncSharedChannelLinkedMemberStatus::Applied,
            member,
        })
    }

    fn add_member_with_actor_kind_and_attributes_inner(
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
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let member = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                upsert_roster_member(conversation, member.clone());
                upsert_read_cursor(conversation, build_default_read_cursor(&member));
                member
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &member);
            member
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );

        Ok(member)
    }

    pub fn remove_member(
        &self,
        command: RemoveConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let member = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                deactivate_roster_member(conversation, member.clone());
                member
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &member);
            member
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );

        Ok(member)
    }

    pub fn leave_conversation(
        &self,
        command: LeaveConversationCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let member = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let member = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                deactivate_roster_member(conversation, member.clone());
                member
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &member);
            member
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );

        Ok(member)
    }

    pub fn transfer_conversation_owner(
        &self,
        command: TransferConversationOwnerCommand,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let result = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let result = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                    .ok_or_else(|| {
                        RuntimeError::MemberNotFound(command.target_member_id.clone())
                    })?;
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
                upsert_roster_member(conversation, payload.previous_owner.clone());
                upsert_roster_member(conversation, payload.new_owner.clone());

                TransferConversationOwnerResult {
                    event_id: event.event_id,
                    transferred_at: payload.transferred_at.clone(),
                    previous_owner: payload.previous_owner,
                    new_owner: payload.new_owner,
                }
            };
            state.sync_actor_inbox_members(
                organization_id.as_str(),
                &[result.previous_owner.clone(), result.new_owner.clone()],
            );
            result
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        Ok(result)
    }

    pub fn change_conversation_member_role(
        &self,
        command: ChangeConversationMemberRoleCommand,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let organization_id = command.organization_id.clone();
        let result = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.membership");
            let result = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                ensure_conversation_write_allowed(conversation)?;
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
                    .ok_or_else(|| {
                        RuntimeError::MemberNotFound(command.target_member_id.clone())
                    })?;
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
                upsert_roster_member(conversation, payload.updated_member.clone());

                ChangeConversationMemberRoleResult {
                    event_id: event.event_id,
                    changed_at: payload.changed_at.clone(),
                    previous_member: payload.previous_member,
                    updated_member: payload.updated_member,
                }
            };
            state.sync_actor_inbox_member(organization_id.as_str(), &result.updated_member);
            result
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            organization_id.as_str(),
            command.conversation_id.as_str(),
        );
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
        page_size: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<ListMembersResult, RuntimeError> {
        let limit = normalize_member_list_limit(page_size).map_err(RuntimeError::InvalidInput)?;
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let cursor_scope = MemberListCursorScope {
            tenant_id,
            organization_id: normalized_organization_id.as_str(),
            conversation_id,
        };
        let repository_cursor = cursor
            .map(|cursor| decode_member_list_cursor(cursor, cursor_scope))
            .transpose()
            .map_err(member_list_cursor_runtime_error)?;

        if let Some(aggregate_store) = self.aggregate_store.as_ref() {
            let page = aggregate_store
                .load_members_page(
                    tenant_id,
                    normalized_organization_id.as_str(),
                    conversation_id,
                    repository_cursor.as_ref(),
                    limit,
                )
                .map_err(RuntimeError::from)?;
            let next_cursor = match (page.has_more, page.next_cursor.as_ref()) {
                (true, Some(next_cursor)) => Some(
                    encode_member_list_cursor(cursor_scope, next_cursor)
                        .map_err(member_list_cursor_runtime_error)?,
                ),
                (true, None) => {
                    return Err(RuntimeError::Contract(ContractError::Invalid(
                        "conversation aggregate store returned has_more without next_cursor".into(),
                    )));
                }
                (false, _) => None,
            };
            return Ok(cursor_list_page_data(
                page.items
                    .iter()
                    .map(conversation_member_from_record)
                    .collect(),
                limit,
                next_cursor,
                page.has_more,
            ));
        }

        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        let window = conversation.roster.list_active_members_after(
            repository_cursor
                .as_ref()
                .map(|cursor| (cursor.principal_kind.as_str(), cursor.principal_id.as_str())),
            limit,
        );
        let next_cursor = if window.has_more {
            let last_member = window.items.last().ok_or_else(|| {
                RuntimeError::Contract(ContractError::Invalid(
                    "conversation roster returned has_more without a member".into(),
                ))
            })?;
            Some(
                encode_member_list_cursor(
                    cursor_scope,
                    &ConversationMemberPageCursor {
                        principal_kind: last_member.principal_kind.clone(),
                        principal_id: last_member.principal_id.clone(),
                    },
                )
                .map_err(member_list_cursor_runtime_error)?,
            )
        } else {
            None
        };
        Ok(cursor_list_page_data(
            window.items,
            limit,
            next_cursor,
            window.has_more,
        ))
    }

    pub fn update_read_cursor(
        &self,
        command: UpdateReadCursorCommand,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
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
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        )?;
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(command.organization_id.as_str());
        let store_high_watermark = if let Some(store) = self.message_store.as_ref() {
            Some(
                store
                    .read_high_watermark(
                        command.tenant_id.as_str(),
                        normalized_organization_id.as_str(),
                        command.conversation_id.as_str(),
                    )
                    .map_err(RuntimeError::from)?,
            )
        } else {
            None
        };
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let mut refreshed_journal_watermark = None;
        let mut append_attempt = 0usize;
        let cursor = loop {
            append_attempt += 1;
            let cursor_attempt = (|| -> Result<ConversationReadCursor, RuntimeError> {
                let mut state =
                    write_runtime_state(&self.state, "conversation-runtime.state.membership");
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                if let Some(high_watermark) = store_high_watermark {
                    conversation
                        .message_log
                        .observe_high_watermark(high_watermark);
                }
                if let Some(journal_watermark) = refreshed_journal_watermark.take() {
                    conversation.aggregate.observe_commit_seq(journal_watermark);
                }
                let high_watermark = conversation.message_log.high_watermark();
                if command.read_seq > high_watermark {
                    return Err(RuntimeError::ReadCursorInvalid(format!(
                        "read cursor exceeds conversation high watermark: {} > {}",
                        command.read_seq, high_watermark
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
                let principal_kind = actor_member.principal_kind.clone();
                let device_id = command.device_id.clone();
                let cursor_missing = conversation
                    .roster
                    .read_cursor(member_id.as_str(), device_id.as_deref())
                    .is_none();
                let cursor = conversation
                    .roster
                    .read_cursor(member_id.as_str(), device_id.as_deref())
                    .cloned()
                    .unwrap_or_else(|| ConversationReadCursor {
                        tenant_id: command.tenant_id.clone(),
                        conversation_id: command.conversation_id.clone(),
                        member_id: member_id.clone(),
                        principal_id: command.principal_id.clone(),
                        principal_kind: principal_kind.clone(),
                        device_id: device_id.clone(),
                        read_seq: 0,
                        last_read_message_id: None,
                        updated_at: conversation_timestamp(),
                    });

                if command.read_seq > cursor.read_seq {
                    let updated_cursor = ConversationReadCursor {
                        read_seq: command.read_seq,
                        last_read_message_id: command.last_read_message_id.clone(),
                        updated_at: conversation_timestamp(),
                        device_id: device_id.clone(),
                        ..cursor
                    };
                    // Allocate the next monotonic journal `ordering_seq` from
                    // the conversation aggregate, not from the message read
                    // sequence. The journal key is shared by every event type
                    // in this conversation.
                    let journal_ordering_seq = conversation.aggregate.next_commit_seq();
                    self.journal
                        .append(build_read_cursor_envelope(ReadCursorEnvelopeInput {
                            tenant_id: command.tenant_id.as_str(),
                            organization_id: command.organization_id.as_str(),
                            conversation_id: command.conversation_id.as_str(),
                            cursor: updated_cursor.clone(),
                            ordering_seq: journal_ordering_seq,
                            retention_class: retention_class.as_str(),
                            actor_id: command.principal_id.as_str(),
                            actor_kind: principal_kind.as_str(),
                        }))
                        .map_err(RuntimeError::from)?;
                    conversation
                        .roster
                        .upsert_read_cursor(updated_cursor.clone());
                    Ok(updated_cursor)
                } else {
                    if cursor_missing {
                        conversation.roster.upsert_read_cursor(cursor.clone());
                    }
                    Ok(cursor)
                }
            })();

            match cursor_attempt {
                Ok(cursor) => break cursor,
                Err(error)
                    if append_attempt < READ_CURSOR_JOURNAL_APPEND_MAX_ATTEMPTS
                        && is_journal_position_conflict(&error) =>
                {
                    refreshed_journal_watermark = self.load_journal_watermark_for_conversation(
                        command.tenant_id.as_str(),
                        command.conversation_id.as_str(),
                    )?;
                    if refreshed_journal_watermark.is_none() {
                        return Err(error);
                    }
                }
                Err(error) => return Err(error),
            }
        };

        self.best_effort_persist_aggregate_state(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );

        Ok(cursor)
    }

    pub fn read_cursor_view(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.membership");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        let cursor = conversation
            .roster
            .read_cursor(member_id.as_str(), None)
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
        self.read_cursor_view_with_actor_kind_and_device(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind,
            None,
        )
    }

    fn read_cursor_view_with_actor_kind_and_device(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: Option<&str>,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
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
            .read_cursor(member_id.as_str(), device_id)
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
        before_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        let limit = validate_message_history_limit(limit)?;
        self.list_messages_history_window(MessageHistoryReadRequest::without_actor_kind(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            before_seq,
            limit,
        ))
    }

    pub fn list_messages_with_actor_kind(
        &self,
        request: MessageHistoryReadRequest<'_>,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        self.list_messages_history_window(MessageHistoryReadRequest {
            limit: validate_message_history_limit(request.limit)?,
            ..request
        })
    }

    fn list_messages_history_window(
        &self,
        request: MessageHistoryReadRequest<'_>,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        if let Some(history) =
            self.list_messages_history_window_from_loaded_conversation(request)?
        {
            return Ok(history);
        }

        if let Some(history) = self
            .list_messages_history_window_from_store_if_joined_member_projection_allows(request)?
        {
            return Ok(history);
        }

        self.ensure_conversation_loaded(
            request.tenant_id,
            request.organization_id,
            request.conversation_id,
        )?;
        let scope_key = conversation_scope_key(
            request.tenant_id,
            request.organization_id,
            request.conversation_id,
        );
        {
            let state = read_runtime_state(&self.state, "conversation-runtime.state.list_messages");
            let conversation = state.conversations.get(scope_key.as_str()).ok_or_else(|| {
                RuntimeError::ConversationNotFound(request.conversation_id.into())
            })?;
            if let Some(kind) = request.principal_kind {
                policy::ensure_history_read_allowed_with_kind(
                    conversation,
                    request.principal_id,
                    kind,
                )?;
            } else {
                policy::ensure_history_read_allowed(conversation, request.principal_id)?;
            }
        }

        if let Some(store) = &self.message_store {
            let normalized_organization_id =
                im_domain_events::normalize_commit_organization_id(request.organization_id);
            let window = store
                .read_history_window(
                    request.tenant_id,
                    normalized_organization_id.as_str(),
                    request.conversation_id,
                    request.before_seq,
                    request.limit,
                )
                .map_err(RuntimeError::from)?;
            return message_history_window_from_store(window, request.limit);
        }

        let in_memory = {
            let state = read_runtime_state(
                &self.state,
                "conversation-runtime.state.list_messages.cache",
            );
            let conversation = state.conversations.get(scope_key.as_str()).ok_or_else(|| {
                RuntimeError::ConversationNotFound(request.conversation_id.into())
            })?;
            conversation
                .message_log
                .message_window_before(request.before_seq, request.limit)
        };
        Ok(message_history_window(in_memory, request.limit))
    }

    fn list_messages_history_window_from_loaded_conversation(
        &self,
        request: MessageHistoryReadRequest<'_>,
    ) -> Result<Option<MessageHistoryResult>, RuntimeError> {
        let scope_key = conversation_scope_key(
            request.tenant_id,
            request.organization_id,
            request.conversation_id,
        );
        {
            let state =
                read_runtime_state(&self.state, "conversation-runtime.state.list_messages.hot");
            let Some(conversation) = state.conversations.get(scope_key.as_str()) else {
                return Ok(None);
            };
            if let Some(kind) = request.principal_kind {
                policy::ensure_history_read_allowed_with_kind(
                    conversation,
                    request.principal_id,
                    kind,
                )?;
            } else {
                policy::ensure_history_read_allowed(conversation, request.principal_id)?;
            }
            if self.message_store.is_none() {
                return Ok(Some(message_history_window(
                    conversation
                        .message_log
                        .message_window_before(request.before_seq, request.limit),
                    request.limit,
                )));
            }
        }

        let Some(store) = self.message_store.as_ref() else {
            return Ok(None);
        };
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(request.organization_id);
        let window = store
            .read_history_window(
                request.tenant_id,
                normalized_organization_id.as_str(),
                request.conversation_id,
                request.before_seq,
                request.limit,
            )
            .map_err(RuntimeError::from)?;
        Ok(Some(message_history_window_from_store(
            window,
            request.limit,
        )?))
    }

    fn list_messages_history_window_from_store_if_joined_member_projection_allows(
        &self,
        request: MessageHistoryReadRequest<'_>,
    ) -> Result<Option<MessageHistoryResult>, RuntimeError> {
        let Some(store) = self.message_store.as_ref() else {
            return Ok(None);
        };
        let Some(aggregate_store) = self.aggregate_store.as_ref() else {
            return Ok(None);
        };
        let Some(principal_kind) = request.principal_kind else {
            return Ok(None);
        };
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(request.organization_id);
        let member = aggregate_store
            .load_member(
                request.tenant_id,
                normalized_organization_id.as_str(),
                request.conversation_id,
                principal_kind,
                request.principal_id,
            )
            .map_err(RuntimeError::from)?;
        let Some(member) = member else {
            return Ok(None);
        };
        if !member.membership_state.eq_ignore_ascii_case("joined") {
            return Ok(None);
        }

        let window = store
            .read_history_window(
                request.tenant_id,
                normalized_organization_id.as_str(),
                request.conversation_id,
                request.before_seq,
                request.limit,
            )
            .map_err(RuntimeError::from)?;
        Ok(Some(message_history_window_from_store(
            window,
            request.limit,
        )?))
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
        let offset = parse_member_list_cursor(cursor)?;
        let limit = limit.max(1);
        let (message_ids, has_more) = conversation
            .message_log
            .pinned_message_ids_page(offset, limit);
        let next_cursor = has_more.then(|| (offset + message_ids.len()).to_string());
        Ok(cursor_list_page_data(
            message_ids,
            limit,
            next_cursor,
            has_more,
        ))
    }

    pub fn list_inbox_from_auth_context(
        &self,
        auth: &AppContext,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<InboxListResult, RuntimeError> {
        let state = read_runtime_state(&self.state, "conversation-runtime.state.inbox");
        let offset = parse_member_list_cursor(cursor)?;
        let limit = limit.max(1);
        let organization_id =
            im_domain_events::normalize_commit_organization_id(auth.organization_id.as_str());
        let page = state.actor_inbox_page(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            offset,
            limit,
        );
        Ok(cursor_list_page_data(
            page.items,
            limit,
            page.next_cursor,
            page.has_more,
        ))
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

fn member_list_cursor_runtime_error(error: MemberListCursorError) -> RuntimeError {
    match error {
        MemberListCursorError::Invalid => {
            RuntimeError::InvalidInput("conversation member list cursor is invalid".into())
        }
        MemberListCursorError::Configuration(message) => {
            RuntimeError::Contract(ContractError::Unavailable(message))
        }
    }
}

fn message_history_page(
    items: Vec<im_domain_core::message::StoredMessage>,
    page_size: usize,
    high_watermark: u64,
    next_before_seq: Option<u64>,
    has_more: bool,
) -> MessageHistoryResult {
    MessageHistoryResult {
        page: SdkWorkPageData {
            items,
            page_info: PageInfo {
                mode: PageMode::Cursor,
                page: None,
                page_size: Some(page_size as i32),
                total_items: None,
                total_pages: None,
                next_cursor: None,
                has_more: Some(has_more),
            },
        },
        high_watermark,
        next_before_seq,
    }
}

fn message_history_window(
    window: im_domain_core::message::MessageHistoryWindow,
    page_size: usize,
) -> MessageHistoryResult {
    message_history_page(
        window.items,
        page_size,
        window.high_watermark,
        window.next_before_seq,
        window.has_more,
    )
}

fn message_history_window_from_store(
    window: im_platform_contracts::MessageWindow,
    page_size: usize,
) -> Result<MessageHistoryResult, RuntimeError> {
    let items = window
        .items
        .iter()
        .map(stored_message_from_record)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(message_history_window(
        im_domain_core::message::MessageHistoryWindow {
            items,
            high_watermark: window.high_watermark,
            next_before_seq: window.next_before_seq,
            has_more: window.has_more,
        },
        page_size,
    ))
}

pub(super) fn stored_message_from_record(
    record: &im_platform_contracts::StoredMessageRecord,
) -> Result<im_domain_core::message::StoredMessage, RuntimeError> {
    use im_domain_core::message::{MessageBody, MessageType, Sender, StoredMessage};
    let body: MessageBody =
        serde_json::from_str(record.payload_json.as_str()).map_err(|error| {
            RuntimeError::InvalidInput(format!("invalid stored message payload: {error}"))
        })?;
    let message_type = match record.message_type.as_str() {
        "system" => MessageType::System,
        "signal" => MessageType::Signal,
        _ => MessageType::Standard,
    };
    Ok(StoredMessage {
        message: im_domain_core::message::Message {
            tenant_id: record.tenant_id.clone(),
            conversation_id: record.conversation_id.clone(),
            message_id: record.message_id.to_string(),
            message_seq: record.message_seq,
            sender: Sender {
                id: record.sender_principal_id.clone(),
                kind: record.sender_principal_kind.clone(),
                member_id: None,
                device_id: record.sender_device_id.clone(),
                session_id: None,
                metadata: Default::default(),
            },
            message_type,
            delivery_mode: "discrete".into(),
            client_msg_id: record.client_msg_id.clone(),
            stream_session_id: None,
            rtc_session_id: None,
            body,
            attributes: Default::default(),
            metadata: Default::default(),
            occurred_at: record.created_at.clone(),
            committed_at: Some(record.updated_at.clone()),
        },
        recalled: record.deleted_at.is_some(),
        reactions: Default::default(),
        pin: None,
    })
}
