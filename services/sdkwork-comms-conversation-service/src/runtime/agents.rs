use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct GroupAgentDefaultPolicy {
    policy_id: String,
    policy_version: u32,
    agents: Vec<ConversationAgentAssignment>,
}

impl Default for GroupAgentDefaultPolicy {
    fn default() -> Self {
        Self {
            policy_id: LEGACY_GROUP_AGENT_DEFAULT_POLICY_ID.into(),
            policy_version: LEGACY_GROUP_AGENT_DEFAULT_POLICY_VERSION,
            agents: legacy_group_agent_assignment_set().agents,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ConversationAgentAssignmentsEventPayload {
    pub generation: u64,
    pub source: ConversationAgentAssignmentSource,
    pub agents: Vec<ConversationAgentAssignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ConversationAgentsReplacedPayload {
    pub conversation_id: String,
    pub previous_generation: u64,
    pub agent_assignments: ConversationAgentAssignmentsEventPayload,
    pub replaced_at: String,
}

pub(super) fn apply_current_group_agent_default(
    aggregate: &mut ConversationAggregateState,
    policy: &GroupAgentDefaultPolicy,
) -> Result<ConversationAgentAssignmentsEventPayload, RuntimeError> {
    if policy.policy_id.trim().is_empty() || policy.policy_version == 0 {
        return Err(RuntimeError::Conflict(
            "group agent default policy identity is invalid".into(),
        ));
    }
    let generation = aggregate
        .replace_agent_assignments(
            ConversationAgentAssignmentSource::DefaultPolicy,
            policy.agents.clone(),
        )
        .map_err(agent_assignment_error_to_runtime)?;
    Ok(ConversationAgentAssignmentsEventPayload {
        generation,
        source: ConversationAgentAssignmentSource::DefaultPolicy,
        agents: policy.agents.clone(),
        policy_id: Some(policy.policy_id.clone()),
        policy_version: Some(policy.policy_version),
    })
}

pub(super) fn legacy_v1_group_agent_default() -> ConversationAgentAssignmentsEventPayload {
    let assignments = legacy_group_agent_assignment_set();
    ConversationAgentAssignmentsEventPayload {
        generation: assignments.generation,
        source: assignments.source,
        agents: assignments.agents,
        policy_id: Some(LEGACY_GROUP_AGENT_DEFAULT_POLICY_ID.into()),
        policy_version: Some(LEGACY_GROUP_AGENT_DEFAULT_POLICY_VERSION),
    }
}

pub(super) fn validate_created_group_agent_assignments(
    payload: &ConversationAgentAssignmentsEventPayload,
) -> Result<(), RuntimeError> {
    if payload.generation == 0 {
        return Err(RuntimeError::Conflict(
            "conversation.created.v2 agent assignment generation must be positive".into(),
        ));
    }
    if payload.source != ConversationAgentAssignmentSource::DefaultPolicy {
        return Err(RuntimeError::Conflict(
            "conversation.created.v2 agent assignment source must be default_policy".into(),
        ));
    }
    if payload
        .policy_id
        .as_deref()
        .is_none_or(|value| value.trim().is_empty())
        || payload.policy_version.is_none_or(|value| value == 0)
    {
        return Err(RuntimeError::Conflict(
            "conversation.created.v2 requires a versioned agent assignment policy".into(),
        ));
    }
    let mut aggregate = ConversationAggregateState::new("group");
    aggregate
        .restore_agent_assignments(
            payload.generation,
            payload.source.clone(),
            payload.agents.clone(),
        )
        .map_err(agent_assignment_error_to_runtime)
}

pub(super) fn agent_assignment_error_to_runtime(
    error: ConversationAgentAssignmentError,
) -> RuntimeError {
    match error {
        ConversationAgentAssignmentError::UnsupportedConversationType(conversation_type) => {
            RuntimeError::ConversationTypeInvalid(format!(
                "agent assignments require a group conversation, got {conversation_type}"
            ))
        }
        ConversationAgentAssignmentError::Empty => {
            RuntimeError::InvalidInput("at least one group agent is required".into())
        }
        ConversationAgentAssignmentError::TooMany { max, actual } => RuntimeError::InvalidInput(
            format!("group agent count must be between 1 and {max}, got {actual}"),
        ),
        ConversationAgentAssignmentError::InvalidAgentId(agent_id) => {
            RuntimeError::InvalidInput(format!("invalid group agent id: {agent_id}"))
        }
        ConversationAgentAssignmentError::InvalidRevisionId(revision_id) => {
            RuntimeError::InvalidInput(format!("invalid group agent revision id: {revision_id}"))
        }
        ConversationAgentAssignmentError::DuplicateAgentId(agent_id) => {
            RuntimeError::InvalidInput(format!("duplicate group agent id: {agent_id}"))
        }
        ConversationAgentAssignmentError::StaleGeneration { current, attempted } => {
            RuntimeError::Conflict(format!(
                "stale group agent assignment generation: current={current}, attempted={attempted}"
            ))
        }
        ConversationAgentAssignmentError::GenerationConflict { generation } => {
            RuntimeError::Conflict(format!(
                "conflicting group agent assignment payload at generation {generation}"
            ))
        }
        ConversationAgentAssignmentError::GenerationOverflow => {
            RuntimeError::Conflict("group agent assignment generation overflow".into())
        }
    }
}

pub(super) fn message_has_agent_mentions(body: &MessageBody) -> bool {
    body.parts
        .iter()
        .any(|part| matches!(part, ContentPart::Mention(_)))
}

pub(super) fn resolve_message_agent_mentions(
    conversation: &ConversationState,
    sender_member: &ConversationMember,
    body: &MessageBody,
) -> Result<Vec<ConversationAgentAssignment>, RuntimeError> {
    if !message_has_agent_mentions(body) {
        return Ok(Vec::new());
    }
    if sender_member.principal_kind != "user" {
        return Err(RuntimeError::PermissionDenied(
            "only an active user member may mention group agents".into(),
        ));
    }
    if conversation.aggregate.conversation_type() != "group" {
        return Err(RuntimeError::InvalidInput(
            "agent mentions require a group conversation".into(),
        ));
    }
    let assignments = conversation.aggregate.agent_assignments().ok_or_else(|| {
        RuntimeError::Conflict(format!(
            "group conversation is missing mandatory agent assignments: {}",
            sender_member.conversation_id
        ))
    })?;
    let mut resolved = Vec::new();
    let mut seen = HashSet::new();
    for (index, mention) in body
        .parts
        .iter()
        .filter_map(ContentPart::as_mention)
        .enumerate()
    {
        if mention.assignment_generation != assignments.generation {
            return Err(RuntimeError::Conflict(format!(
                "message agent mention generation is stale at mention {index}: supplied={}, current={}",
                mention.assignment_generation, assignments.generation
            )));
        }
        let assignment = assignments
            .agents
            .iter()
            .find(|assignment| assignment.agent_id == mention.target_id)
            .ok_or_else(|| {
                RuntimeError::InvalidInput(format!(
                    "message agent mention target is not assigned to this conversation: {}",
                    mention.target_id
                ))
            })?;
        if seen.insert(assignment.agent_id.clone()) {
            resolved.push(assignment.clone());
        }
    }
    Ok(resolved)
}

fn build_conversation_agents_replaced_envelope(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    payload: &ConversationAgentsReplacedPayload,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> Result<CommitEnvelope, RuntimeError> {
    let payload_json = runtime_json_string(payload)?;
    let payload_hash = sha256_hash(payload_json.as_bytes());
    Ok(CommitEnvelope {
        event_id: format!(
            "evt_{}_agents_{}_{}",
            event_id_component(conversation_id),
            payload.agent_assignments.generation,
            &payload_hash[..16]
        ),
        tenant_id: tenant_id.into(),
        organization_id: organization_id.into(),
        event_type: "conversation.agents_replaced".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.replaced_at.clone(),
        committed_at: payload.replaced_at.clone(),
        payload_schema: Some("conversation.agents_replaced.v1".into()),
        payload: payload_json,
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    })
}

impl ReplaceConversationAgentsCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        expected_generation: u64,
        agents: Vec<ConversationAgentAssignment>,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            replaced_by: auth.actor_id.clone(),
            expected_generation,
            agents,
        }
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub(super) fn hydrate_conversation_agent_metadata_if_missing(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        {
            let state = read_runtime_state(
                &self.state,
                "conversation-runtime.state.agents.hydration-check",
            );
            let already_loaded =
                state
                    .conversations
                    .get(scope_key.as_str())
                    .is_some_and(|conversation| {
                        conversation.aggregate.conversation_type() == "group"
                            && conversation.aggregate.agent_assignments().is_some()
                    });
            if already_loaded {
                return Ok(());
            }
        }

        let scope = CommitJournalAggregateScope {
            tenant_id: tenant_id.into(),
            aggregate_id: conversation_id.into(),
        };
        let mut cursor = None;
        loop {
            let page = self
                .journal
                .recorded_page_for_aggregate(
                    &scope,
                    cursor.as_ref(),
                    COMMIT_JOURNAL_REPLAY_BATCH_LIMIT,
                )
                .map_err(RuntimeError::from)?;
            if page.items.is_empty() {
                break;
            }
            let batch_len = page.items.len();
            for envelope in &page.items {
                if matches!(
                    envelope.event_type.as_str(),
                    "conversation.created" | "conversation.agents_replaced"
                ) {
                    self.apply_recovered_envelope(envelope)?;
                }
            }
            cursor = page.next_cursor;
            if batch_len < COMMIT_JOURNAL_REPLAY_BATCH_LIMIT {
                break;
            }
        }
        Ok(())
    }

    pub fn conversation_agent_assignments_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationAgentAssignmentSet, RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        self.hydrate_conversation_agent_metadata_if_missing(
            tenant_id,
            organization_id,
            conversation_id,
        )?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(&self.state, "conversation-runtime.state.agents.snapshot");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::ConversationTypeInvalid(format!(
                "agent assignments require a group conversation, got {}",
                conversation.aggregate.conversation_type()
            )));
        }
        conversation
            .aggregate
            .agent_assignments()
            .cloned()
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "group conversation is missing mandatory agent assignments: {conversation_id}"
                ))
            })
    }

    pub fn conversation_agent_assignments_snapshot_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationAgentAssignmentSet, RuntimeError> {
        self.require_active_member_with_kind(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )?;
        self.conversation_agent_assignments_snapshot(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
        )
    }

    pub fn replace_conversation_agents_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        expected_generation: u64,
        agents: Vec<ConversationAgentAssignment>,
    ) -> Result<ReplaceConversationAgentsResult, RuntimeError> {
        self.replace_conversation_agents_with_actor_kind(
            ReplaceConversationAgentsCommand::from_auth_context(
                auth,
                conversation_id,
                expected_generation,
                agents,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn replace_conversation_agents(
        &self,
        command: ReplaceConversationAgentsCommand,
    ) -> Result<ReplaceConversationAgentsResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
                command.replaced_by.as_str(),
            )?
            .principal_kind;
        self.replace_conversation_agents_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn replace_conversation_agents_with_actor_kind(
        &self,
        command: ReplaceConversationAgentsCommand,
        actor_kind: &str,
    ) -> Result<ReplaceConversationAgentsResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "replacedBy",
            command.replaced_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES)?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            actor_kind,
            command.replaced_by.as_str(),
        )?;
        self.hydrate_conversation_agent_metadata_if_missing(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        )?;
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let result = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.agents.replace");
            let result = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                let actor_member = resolve_active_member_with_kind(
                    conversation,
                    command.replaced_by.as_str(),
                    actor_kind,
                )?;
                policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
                if conversation.aggregate.conversation_type() != "group" {
                    return Err(RuntimeError::ConversationTypeInvalid(format!(
                        "agent assignments require a group conversation, got {}",
                        conversation.aggregate.conversation_type()
                    )));
                }
                if !matches!(
                    actor_member.role,
                    MembershipRole::Owner | MembershipRole::Admin
                ) {
                    return Err(RuntimeError::PermissionDenied(
                        "only a group owner or admin may replace agent assignments".into(),
                    ));
                }
                let current = conversation.aggregate.agent_assignments().ok_or_else(|| {
                    RuntimeError::Conflict(format!(
                        "group conversation is missing mandatory agent assignments: {}",
                        command.conversation_id
                    ))
                })?;
                let previous_generation = current.generation;
                if command.expected_generation != previous_generation {
                    return Err(RuntimeError::Conflict(format!(
                        "group agent assignment generation mismatch: expected={}, current={previous_generation}",
                        command.expected_generation
                    )));
                }

                let mut next_aggregate = conversation.aggregate.clone();
                let generation = next_aggregate
                    .replace_agent_assignments(
                        ConversationAgentAssignmentSource::ConversationOverride,
                        command.agents.clone(),
                    )
                    .map_err(agent_assignment_error_to_runtime)?;
                let ordering_seq = next_aggregate.next_commit_seq();
                let assignments = next_aggregate.agent_assignments().cloned().ok_or_else(|| {
                    RuntimeError::Conflict(
                        "group agent assignment replacement produced no snapshot".into(),
                    )
                })?;
                debug_assert_eq!(assignments.generation, generation);
                let replaced_at = conversation_timestamp();
                let payload = ConversationAgentsReplacedPayload {
                    conversation_id: command.conversation_id.clone(),
                    previous_generation,
                    agent_assignments: ConversationAgentAssignmentsEventPayload {
                        generation,
                        source: assignments.source.clone(),
                        agents: assignments.agents.clone(),
                        policy_id: None,
                        policy_version: None,
                    },
                    replaced_at: replaced_at.clone(),
                };
                let event = build_conversation_agents_replaced_envelope(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                    &payload,
                    ordering_seq,
                    conversation_retention_class(conversation).as_str(),
                    command.replaced_by.as_str(),
                    actor_member.principal_kind.as_str(),
                )?;

                self.journal.append(event.clone())?;
                conversation.aggregate = next_aggregate;
                ReplaceConversationAgentsResult {
                    event_id: event.event_id,
                    previous_generation,
                    assignments,
                    replaced_at,
                }
            };
            state.touch_conversation(scope_key.as_str());
            result
        };
        self.maybe_evict_after_write();
        Ok(result)
    }

    #[cfg(test)]
    pub(super) fn with_group_agent_default_policy_for_test(
        mut self,
        policy_id: &str,
        policy_version: u32,
        agents: Vec<ConversationAgentAssignment>,
    ) -> Self {
        self.group_agent_default_policy = GroupAgentDefaultPolicy {
            policy_id: policy_id.into(),
            policy_version,
            agents,
        };
        self
    }
}
