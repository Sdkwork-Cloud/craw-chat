use super::*;

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn conversation_business_binding(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationBusinessBinding, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.binding");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        conversation
            .aggregate
            .business_binding()
            .cloned()
            .ok_or_else(|| {
                RuntimeError::ConversationBindingNotFound(format!(
                    "conversation {conversation_id} has no business binding"
                ))
            })
    }

    pub fn conversation_business_binding_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationBusinessBinding, RuntimeError> {
        let organization_id = organization_id_from_auth_context(auth);
        if auth.actor_kind != "system" {
            self.require_active_member_with_kind(
                auth.tenant_id.as_str(),
                organization_id.as_str(),
                conversation_id,
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )?;
        }

        self.conversation_business_binding(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        )
    }
}
