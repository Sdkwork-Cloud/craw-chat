use super::*;
use im_domain_core::conversation::{
    ConversationHandoffLifecycle, ConversationHandoffTransitionError,
    ConversationHandoffTransitionOutcome,
};

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn get_agent_handoff_state_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        self.get_agent_handoff_state_with_actor_kind(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
    }

    pub fn accept_agent_handoff_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        self.accept_agent_handoff_with_actor_kind(
            AcceptAgentHandoffCommand::from_auth_context(auth, conversation_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn resolve_agent_handoff_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        self.resolve_agent_handoff_with_actor_kind(
            ResolveAgentHandoffCommand::from_auth_context(auth, conversation_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn close_agent_handoff_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        self.close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand::from_auth_context(auth, conversation_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn get_agent_handoff_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.handoff");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        resolve_active_member(conversation, principal_id)?;
        policy::ensure_agent_handoff_conversation(conversation)?;
        conversation
            .aggregate
            .handoff_state()
            .cloned()
            .ok_or_else(|| {
                RuntimeError::ConversationTypeInvalid("agent handoff state missing".into())
            })
    }

    pub fn get_agent_handoff_state_with_actor_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.handoff");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        resolve_active_member_with_kind(conversation, principal_id, principal_kind)?;
        policy::ensure_agent_handoff_conversation(conversation)?;
        conversation
            .aggregate
            .handoff_state()
            .cloned()
            .ok_or_else(|| {
                RuntimeError::ConversationTypeInvalid("agent handoff state missing".into())
            })
    }

    pub fn accept_agent_handoff_with_actor_kind(
        &self,
        command: AcceptAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            command.accepted_by.as_str(),
            actor_kind,
            ConversationHandoffLifecycle::Accept,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    pub fn resolve_agent_handoff_with_actor_kind(
        &self,
        command: ResolveAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            command.resolved_by.as_str(),
            actor_kind,
            ConversationHandoffLifecycle::Resolve,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    pub fn close_agent_handoff_with_actor_kind(
        &self,
        command: CloseAgentHandoffCommand,
        actor_kind: &str,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        let outcome = self.transition_agent_handoff_status(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            command.closed_by.as_str(),
            actor_kind,
            ConversationHandoffLifecycle::Close,
        )?;
        self.finish_agent_handoff_transition(outcome)
    }

    fn transition_agent_handoff_status(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        actor_id: &str,
        actor_kind: &str,
        action: ConversationHandoffLifecycle,
    ) -> Result<AgentHandoffStatusTransitionOutcome, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let mut state = write_runtime_state(&self.state, "conversation-runtime.state.handoff");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        policy::ensure_agent_handoff_conversation(conversation)?;
        let actor_member = resolve_active_member_with_kind(conversation, actor_id, actor_kind)?;
        policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
        let actor = build_handoff_actor_view(&actor_member);
        let changed_at = conversation_timestamp();
        let transition = conversation
            .aggregate
            .transition_handoff_status(action, &actor, changed_at.clone())
            .map_err(map_handoff_transition_error)?;
        if transition.outcome == ConversationHandoffTransitionOutcome::Idempotent {
            return Ok(AgentHandoffStatusTransitionOutcome::Idempotent(
                transition.state,
            ));
        }

        let payload = AgentHandoffStatusChangedPayload {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            conversation_id: conversation_id.into(),
            previous_status: transition.previous_status,
            current_status: transition.state.status.clone(),
            changed_by: actor,
            changed_at,
            state: transition.state,
        };
        let retention_class = conversation_retention_class(conversation);
        Ok(AgentHandoffStatusTransitionOutcome::Mutated {
            payload,
            ordering_seq: transition.ordering_seq,
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            retention_class,
        })
    }

    fn finish_agent_handoff_transition(
        &self,
        outcome: AgentHandoffStatusTransitionOutcome,
    ) -> Result<AgentHandoffStateView, RuntimeError> {
        match outcome {
            AgentHandoffStatusTransitionOutcome::Idempotent(state) => Ok(state),
            AgentHandoffStatusTransitionOutcome::Mutated {
                payload,
                ordering_seq,
                actor_id,
                actor_kind,
                retention_class,
            } => {
                let envelope = build_agent_handoff_status_changed_envelope(
                    payload.clone(),
                    ordering_seq,
                    retention_class.as_str(),
                    actor_id.as_str(),
                    actor_kind.as_str(),
                );
                self.journal.append(envelope)?;
                Ok(payload.state)
            }
        }
    }
}

fn build_handoff_actor_view(member: &ConversationMember) -> ChangeAgentHandoffStatusView {
    ChangeAgentHandoffStatusView {
        id: member.principal_id.clone(),
        kind: member.principal_kind.clone(),
    }
}

fn map_handoff_transition_error(error: ConversationHandoffTransitionError) -> RuntimeError {
    match error {
        ConversationHandoffTransitionError::PermissionDenied(message) => {
            RuntimeError::PermissionDenied(message)
        }
        ConversationHandoffTransitionError::Conflict(message) => RuntimeError::Conflict(message),
    }
}
