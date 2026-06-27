use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationPolicyAppliedPayload {
    pub conversation_id: String,
    pub policy_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_flags: Option<Vec<String>>,
    pub history_visibility: String,
    pub retention_policy_ref: String,
}

impl ConversationPolicyAppliedPayload {
    pub(super) fn into_policy(self) -> ConversationPolicy {
        ConversationPolicy {
            policy_version: self.policy_version,
            capability_flags: self.capability_flags,
            history_visibility: self.history_visibility,
            retention_policy_ref: self.retention_policy_ref,
        }
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn conversation_policy_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Option<ConversationPolicy>, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.governance");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        Ok(conversation.aggregate.policy().cloned())
    }

    pub fn apply_conversation_policy_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        policy: ConversationPolicy,
    ) -> Result<ConversationPolicy, RuntimeError> {
        self.apply_conversation_policy_with_actor_kind(
            ApplyConversationPolicyCommand::from_auth_context(auth, conversation_id, policy),
            auth.actor_kind.as_str(),
        )
    }

    pub fn apply_conversation_policy(
        &self,
        command: ApplyConversationPolicyCommand,
    ) -> Result<ConversationPolicy, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                command.applied_by.as_str(),
            )?
            .principal_kind;
        self.apply_conversation_policy_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn apply_conversation_policy_with_actor_kind(
        &self,
        command: ApplyConversationPolicyCommand,
        actor_kind: &str,
    ) -> Result<ConversationPolicy, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
        let (payload, ordering_seq, actor_kind, applied_at) = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.governance");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let actor_member = resolve_active_member_with_kind(
                conversation,
                command.applied_by.as_str(),
                actor_kind,
            )?;
            policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            policy::ensure_conversation_policy_write_allowed(conversation, &actor_member)?;

            let normalized = command.policy.normalize().map_err(RuntimeError::Conflict)?;
            conversation
                .aggregate
                .replace_policy(Some(normalized.clone()));
            let ordering_seq = conversation.aggregate.next_policy_epoch();
            let applied_at = conversation_timestamp();

            (
                ConversationPolicyAppliedPayload {
                    conversation_id: command.conversation_id.clone(),
                    policy_version: normalized.policy_version.clone(),
                    capability_flags: normalized.capability_flags.clone(),
                    history_visibility: normalized.history_visibility.clone(),
                    retention_policy_ref: normalized.retention_policy_ref.clone(),
                },
                ordering_seq,
                actor_member.principal_kind.clone(),
                applied_at,
            )
        };

        self.journal
            .append(build_conversation_policy_applied_envelope(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                payload.clone(),
                ordering_seq,
                applied_at.as_str(),
                command.applied_by.as_str(),
                actor_kind.as_str(),
            ))?;

        let retention_class = super::support::retention_class_from_policy_ref(
            payload.retention_policy_ref.as_str(),
        );
        if im_domain_core::retention::retention_is_indefinite(retention_class.as_str()) {
            if let Some(store) = &self.retention_scope_store {
                store
                    .clear_conversation_retention_until(
                        command.tenant_id.as_str(),
                        command.organization_id.as_str(),
                        command.conversation_id.as_str(),
                    )
                    .map_err(RuntimeError::from)?;
            }
        }

        Ok(payload.into_policy())
    }
}
