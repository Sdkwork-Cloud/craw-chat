use super::message_realtime::ConversationRealtimeEvent;
use super::*;
use im_domain_core::conversation::CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT;

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

struct ConversationAgentsReplacedEnvelopeInput<'a> {
    tenant_id: &'a str,
    organization_id: &'a str,
    conversation_id: &'a str,
    payload: &'a ConversationAgentsReplacedPayload,
    ordering_seq: u64,
    retention_class: &'a str,
    actor_id: &'a str,
    actor_kind: &'a str,
}

fn build_conversation_agents_replaced_envelope(
    input: ConversationAgentsReplacedEnvelopeInput<'_>,
) -> Result<CommitEnvelope, RuntimeError> {
    let payload_json = runtime_json_string(input.payload)?;
    let payload_hash = sha256_hash(payload_json.as_bytes());
    Ok(CommitEnvelope {
        event_id: format!(
            "evt_{}_agents_{}_{}",
            event_id_component(input.conversation_id),
            input.payload.agent_assignments.generation,
            &payload_hash[..16]
        ),
        tenant_id: input.tenant_id.into(),
        organization_id: input.organization_id.into(),
        event_type: "conversation.agents_replaced".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: input.conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: input.conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(input.tenant_id, input.conversation_id),
        ordering_seq: input.ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: input.actor_id.into(),
            actor_kind: input.actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: input.payload.replaced_at.clone(),
        committed_at: input.payload.replaced_at.clone(),
        payload_schema: Some("conversation.agents_replaced.v1".into()),
        payload: payload_json,
        retention_class: input.retention_class.into(),
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
                "conversation-runtime.state.agents.normalized-refresh-check",
            );
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
            if self.aggregate_store.is_none()
                && self.agent_integration_store.is_none()
                && conversation.aggregate.agent_assignments().is_some()
            {
                return Ok(());
            }
        }

        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let normalized_conversation = self.load_normalized_conversation(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        )?;
        if normalized_conversation.tenant_id != tenant_id
            || normalized_conversation.organization_id != normalized_organization_id
            || normalized_conversation.conversation_id != conversation_id
            || normalized_conversation.conversation_type != "group"
        {
            return Err(RuntimeError::Conflict(format!(
                "normalized conversation agent assignment scope is invalid for {conversation_id}"
            )));
        }
        if !matches!(
            normalized_conversation.lifecycle_state.as_str(),
            "active" | "archived"
        ) {
            return Err(RuntimeError::Conflict(format!(
                "normalized conversation lifecycle is invalid for agent assignments: {conversation_id}"
            )));
        }

        let store = self.agent_integration_store.as_ref().ok_or_else(|| {
            RuntimeError::Contract(ContractError::Unavailable(
                "normalized Agent integration store is required for assignment reads".into(),
            ))
        })?;
        let normalized_tenant_id =
            parse_normalized_assignment_scope_id(tenant_id, "tenantId", false)?;
        let normalized_organization_numeric_id = parse_normalized_assignment_scope_id(
            normalized_organization_id.as_str(),
            "organizationId",
            true,
        )?;
        let records = store.list_conversation_agents(
            normalized_tenant_id,
            normalized_organization_numeric_id,
            conversation_id,
            CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT.saturating_add(1),
        )?;
        if records.is_empty() {
            return Err(RuntimeError::Conflict(format!(
                "group conversation is missing normalized agent assignments: {conversation_id}"
            )));
        }
        if records.len() > CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT {
            return Err(RuntimeError::Conflict(format!(
                "normalized agent assignment count exceeds the domain limit for {conversation_id}"
            )));
        }

        let first = records.first().ok_or_else(|| {
            RuntimeError::Conflict(format!(
                "group conversation is missing normalized agent assignments: {conversation_id}"
            ))
        })?;
        let assignment_generation = first.assignment_generation;
        let assignment_source = first.assignment_source;
        let source_aggregate_version = first.source_aggregate_version;
        if assignment_generation == 0
            || source_aggregate_version == 0
            || source_aggregate_version > normalized_conversation.commit_seq
        {
            return Err(RuntimeError::Conflict(format!(
                "normalized agent assignment version is invalid for {conversation_id}"
            )));
        }
        let mut agents = Vec::with_capacity(records.len());
        for (position, record) in records.into_iter().enumerate() {
            if record.tenant_id != normalized_tenant_id
                || record.organization_id != normalized_organization_numeric_id
                || record.conversation_id != conversation_id
                || record.assignment_source != assignment_source
                || record.assignment_generation != assignment_generation
                || record.source_aggregate_version != source_aggregate_version
                || record.position != position as i32
                || !record.enabled
                || record.status != 0
            {
                return Err(RuntimeError::Conflict(format!(
                    "normalized agent assignment rows are inconsistent for {conversation_id}"
                )));
            }
            agents.push(ConversationAgentAssignment::new(
                record.agent_id,
                record.agent_revision_ref,
            ));
        }
        let assignment_source = match assignment_source {
            AgentAssignmentSource::DefaultPolicy => {
                ConversationAgentAssignmentSource::DefaultPolicy
            }
            AgentAssignmentSource::ConversationOverride => {
                ConversationAgentAssignmentSource::ConversationOverride
            }
        };

        let mut state = write_runtime_state(
            &self.state,
            "conversation-runtime.state.agents.normalized-refresh",
        );
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        conversation
            .aggregate
            .observe_commit_seq(normalized_conversation.commit_seq);
        conversation
            .aggregate
            .restore_agent_assignments(assignment_generation, assignment_source, agents)
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "normalized agent assignments are invalid for {conversation_id}: {error:?}"
                ))
            })?;
        state.touch_conversation(scope_key.as_str());
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
        let needs_post_commit_delivery = self.durable_conversation_event_writer.is_none();
        let (result, realtime_payload) = {
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
                let realtime_payload = runtime_json_string(&payload)?;
                let event = build_conversation_agents_replaced_envelope(
                    ConversationAgentsReplacedEnvelopeInput {
                        tenant_id: command.tenant_id.as_str(),
                        organization_id: command.organization_id.as_str(),
                        conversation_id: command.conversation_id.as_str(),
                        payload: &payload,
                        ordering_seq,
                        retention_class: conversation_retention_class(conversation).as_str(),
                        actor_id: command.replaced_by.as_str(),
                        actor_kind: actor_member.principal_kind.as_str(),
                    },
                )?;

                let mut candidate = conversation.clone();
                candidate.aggregate = next_aggregate;
                let assignment_change = self
                    .durable_conversation_event_writer
                    .as_ref()
                    .map(|_| build_normalized_agent_assignment_change(&event, &assignments))
                    .transpose()?;
                self.persist_normalized_conversation_changes_with_assignments(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                    &candidate,
                    Vec::new(),
                    Vec::new(),
                    assignment_change,
                    vec![event.clone()],
                )?;
                *conversation = candidate;
                (
                    ReplaceConversationAgentsResult {
                        event_id: event.event_id,
                        previous_generation,
                        assignments,
                        replaced_at,
                    },
                    realtime_payload,
                )
            };
            state.touch_conversation(scope_key.as_str());
            result
        };
        // PostgreSQL production wiring persists the outbox in the same
        // transaction as the journal event; the relay then owns delivery. The
        // in-memory/test path keeps the low-latency publisher and post-commit
        // outbox fallback used by the rest of the runtime.
        if needs_post_commit_delivery
            && let Err(error) =
                self.publish_or_enqueue_conversation_event(ConversationRealtimeEvent {
                    tenant_id: command.tenant_id.as_str(),
                    organization_id: command.organization_id.as_str(),
                    conversation_id: command.conversation_id.as_str(),
                    event_type: "conversation.agents_replaced",
                    journal_event_id: result.event_id.as_str(),
                    payload_json: realtime_payload,
                    occurred_at: result.replaced_at.as_str(),
                })
        {
            tracing::warn!(
                conversation_id = %command.conversation_id,
                event_id = %result.event_id,
                error = ?error,
                "conversation.agents_replaced realtime delivery failed after journal commit"
            );
        }
        self.maybe_evict_after_write();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_platform_contracts::{
        AgentDispatchRecord, AgentDispatchStatus, AgentReplyCommitResult,
        ConversationAgentAssignmentRecord, ConversationAgentBindingRecord,
        ConversationAggregateStore, ConversationMemberPage, ConversationMemberPageCursor,
        ConversationMemberRecord, NormalizedConversationRecord, ReadCursorPage,
        ReadCursorPageCursor, ReadCursorRecord, ReplaceConversationAgentAssignments,
    };
    use sdkwork_im_contract_message::{
        CommitJournalAggregateEventTypeQuery, CommitJournalAggregateScope,
        CommitJournalReplayCursor, CommitJournalReplayPage, CommitPosition,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone, Default)]
    struct ReadCountingJournal {
        inner: InMemoryJournal,
        read_calls: Arc<AtomicUsize>,
    }

    impl ReadCountingJournal {
        fn read_calls(&self) -> usize {
            self.read_calls.load(Ordering::SeqCst)
        }

        fn record_read(&self) {
            self.read_calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl CommitJournal for ReadCountingJournal {
        fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
            self.inner.append(envelope)
        }

        fn append_batch(
            &self,
            envelopes: Vec<CommitEnvelope>,
        ) -> Result<Vec<CommitPosition>, ContractError> {
            self.inner.append_batch(envelopes)
        }

        fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
            self.record_read();
            Ok(self.inner.recorded())
        }

        fn recorded_page(
            &self,
            cursor: Option<&CommitJournalReplayCursor>,
            limit: usize,
        ) -> Result<CommitJournalReplayPage, ContractError> {
            self.record_read();
            CommitJournal::recorded_page(&self.inner, cursor, limit)
        }

        fn recorded_page_for_aggregate(
            &self,
            scope: &CommitJournalAggregateScope,
            cursor: Option<&CommitJournalReplayCursor>,
            limit: usize,
        ) -> Result<CommitJournalReplayPage, ContractError> {
            self.record_read();
            CommitJournal::recorded_page_for_aggregate(&self.inner, scope, cursor, limit)
        }

        fn recorded_page_for_aggregate_event_types(
            &self,
            query: &CommitJournalAggregateEventTypeQuery,
            cursor: Option<&CommitJournalReplayCursor>,
            limit: usize,
        ) -> Result<CommitJournalReplayPage, ContractError> {
            self.record_read();
            CommitJournal::recorded_page_for_aggregate_event_types(
                &self.inner,
                query,
                cursor,
                limit,
            )
        }
    }

    struct AgentAssignmentTestAggregateStore {
        conversation: Option<NormalizedConversationRecord>,
    }

    impl ConversationAggregateStore for AgentAssignmentTestAggregateStore {
        fn load_conversation(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<Option<NormalizedConversationRecord>, ContractError> {
            Ok(self.conversation.clone())
        }

        fn load_members_page(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _cursor: Option<&ConversationMemberPageCursor>,
            _page_size: usize,
        ) -> Result<ConversationMemberPage, ContractError> {
            unsupported_test_store()
        }

        fn load_member(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
        ) -> Result<Option<ConversationMemberRecord>, ContractError> {
            unsupported_test_store()
        }

        fn load_member_by_id(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _member_id: i64,
        ) -> Result<Option<ConversationMemberRecord>, ContractError> {
            unsupported_test_store()
        }

        fn load_event_recipients_page(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _joined_before_or_at: &str,
            _cursor: Option<&ConversationMemberPageCursor>,
            _page_size: usize,
        ) -> Result<ConversationMemberPage, ContractError> {
            unsupported_test_store()
        }

        fn upsert_member(&self, _member: ConversationMemberRecord) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn remove_member(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _removed_at: &str,
        ) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn load_read_cursors_page(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _cursor: Option<&ReadCursorPageCursor>,
            _page_size: usize,
        ) -> Result<ReadCursorPage, ContractError> {
            unsupported_test_store()
        }

        fn load_read_cursor(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _member_id: i64,
        ) -> Result<Option<ReadCursorRecord>, ContractError> {
            unsupported_test_store()
        }

        fn upsert_read_cursor(&self, _cursor: ReadCursorRecord) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn load_high_watermark(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<u64, ContractError> {
            unsupported_test_store()
        }

        fn allocate_member_id(&self) -> Result<i64, ContractError> {
            unsupported_test_store()
        }

        fn conversation_exists(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<bool, ContractError> {
            unsupported_test_store()
        }
    }

    struct AgentAssignmentTestStore {
        records: Vec<ConversationAgentAssignmentRecord>,
    }

    impl AgentIntegrationStore for AgentAssignmentTestStore {
        fn replace_conversation_agents(
            &self,
            _command: ReplaceConversationAgentAssignments,
        ) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn list_conversation_agents(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            _conversation_id: &str,
            limit: usize,
        ) -> Result<Vec<ConversationAgentAssignmentRecord>, ContractError> {
            Ok(self.records.iter().take(limit).cloned().collect())
        }

        fn enqueue_dispatches(
            &self,
            _request: &AgentMentionDispatchRequest,
            _max_attempts: u32,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            unsupported_test_store()
        }

        fn claim_dispatches(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            _lease_owner: &str,
            _now: &str,
            _lease_expires_at: &str,
            _limit: usize,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            unsupported_test_store()
        }

        fn claim_global_dispatches(
            &self,
            _request: im_platform_contracts::GlobalAgentDispatchClaimRequest<'_>,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            unsupported_test_store()
        }

        fn resolve_binding(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            _conversation_id: &str,
            _agent_id: &str,
            _assignment_generation: u64,
        ) -> Result<Option<ConversationAgentBindingRecord>, ContractError> {
            unsupported_test_store()
        }

        fn save_binding(
            &self,
            _binding: ConversationAgentBindingRecord,
        ) -> Result<ConversationAgentBindingRecord, ContractError> {
            unsupported_test_store()
        }

        fn mark_dispatch_running(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _binding_id: &str,
            _agents_session_id: &str,
            _updated_at: &str,
        ) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn complete_dispatch(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _agents_turn_id: &str,
            _reply: AgentReplyCommitResult,
            _completed_at: &str,
        ) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn defer_dispatch_reconciliation(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _agents_turn_id: Option<&str>,
            _detail: &str,
            _next_attempt_at: &str,
            _updated_at: &str,
        ) -> Result<(), ContractError> {
            unsupported_test_store()
        }

        fn fail_dispatch(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _error_code: &str,
            _error_detail: &str,
            _next_attempt_at: &str,
            _updated_at: &str,
        ) -> Result<AgentDispatchStatus, ContractError> {
            unsupported_test_store()
        }
    }

    fn unsupported_test_store<T>() -> Result<T, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "operation is not used by normalized assignment tests".into(),
        ))
    }

    fn normalized_conversation(conversation_type: &str) -> NormalizedConversationRecord {
        NormalizedConversationRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: format!("c_normalized_agent_{conversation_type}"),
            conversation_type: conversation_type.into(),
            lifecycle_state: "active".into(),
            archived_at: None,
            archive_event_id: None,
            commit_seq: 5,
            member_epoch: 1,
            last_activity_at: "2026-07-24T00:00:00Z".into(),
            retention_until: None,
        }
    }

    fn normalized_assignment(
        conversation_id: &str,
        position: i32,
    ) -> ConversationAgentAssignmentRecord {
        ConversationAgentAssignmentRecord {
            tenant_id: 100001,
            organization_id: 0,
            conversation_id: conversation_id.into(),
            agent_id: format!("agent.im.normalized.{position}"),
            agent_revision_ref: Some(format!("revision.im.normalized.{position}.1")),
            assignment_source: AgentAssignmentSource::ConversationOverride,
            assignment_generation: 2,
            position,
            enabled: true,
            status: 0,
            source_aggregate_version: 4,
        }
    }

    fn assignment_runtime(
        conversation: Option<NormalizedConversationRecord>,
        records: Vec<ConversationAgentAssignmentRecord>,
    ) -> (
        ConversationRuntime<ReadCountingJournal>,
        ReadCountingJournal,
    ) {
        let conversation_type = conversation
            .as_ref()
            .map(|conversation| conversation.conversation_type.as_str())
            .unwrap_or("group");
        let conversation_id = conversation
            .as_ref()
            .map(|conversation| conversation.conversation_id.clone())
            .unwrap_or_else(|| "c_normalized_agent_group".into());
        let journal = ReadCountingJournal::default();
        let runtime = ConversationRuntime::new(journal.clone());
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id,
                creator_id: "42".into(),
                conversation_type: conversation_type.into(),
            })
            .expect("test conversation should be created");
        (
            runtime
                .with_aggregate_store(Arc::new(AgentAssignmentTestAggregateStore { conversation }))
                .with_agent_integration_store(Arc::new(AgentAssignmentTestStore { records })),
            journal,
        )
    }

    #[test]
    fn normalized_assignment_snapshot_refreshes_without_reading_the_journal() {
        let conversation = normalized_conversation("group");
        let records = vec![normalized_assignment(
            conversation.conversation_id.as_str(),
            0,
        )];
        let (runtime, journal) = assignment_runtime(Some(conversation.clone()), records);

        let snapshot = runtime
            .conversation_agent_assignments_snapshot(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
            )
            .expect("normalized assignment snapshot should load");

        assert_eq!(snapshot.generation, 2);
        assert_eq!(
            snapshot.source,
            ConversationAgentAssignmentSource::ConversationOverride
        );
        assert_eq!(snapshot.agents[0].agent_id, "agent.im.normalized.0");
        assert_eq!(journal.read_calls(), 0);
    }

    #[test]
    fn normalized_assignment_snapshot_fails_closed_for_missing_rows_and_store() {
        let conversation = normalized_conversation("group");
        let (runtime, journal) = assignment_runtime(Some(conversation.clone()), Vec::new());
        assert!(matches!(
            runtime.conversation_agent_assignments_snapshot(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
            ),
            Err(RuntimeError::Conflict(_))
        ));
        assert_eq!(journal.read_calls(), 0);

        let journal = ReadCountingJournal::default();
        let runtime = ConversationRuntime::new(journal.clone());
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: conversation.tenant_id.clone(),
                organization_id: conversation.organization_id.clone(),
                conversation_id: conversation.conversation_id.clone(),
                creator_id: "42".into(),
                conversation_type: "group".into(),
            })
            .expect("test group should be created");
        let runtime = runtime.with_aggregate_store(Arc::new(AgentAssignmentTestAggregateStore {
            conversation: Some(conversation.clone()),
        }));
        assert!(matches!(
            runtime.conversation_agent_assignments_snapshot(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
            ),
            Err(RuntimeError::Contract(ContractError::Unavailable(_)))
        ));
        assert_eq!(journal.read_calls(), 0);
    }

    #[test]
    fn normalized_assignment_snapshot_rejects_scope_generation_and_source_mismatch() {
        let conversation = normalized_conversation("group");
        let mut wrong_scope = normalized_assignment(conversation.conversation_id.as_str(), 0);
        wrong_scope.organization_id = 7;
        let (runtime, journal) = assignment_runtime(Some(conversation.clone()), vec![wrong_scope]);
        assert!(matches!(
            runtime.conversation_agent_assignments_snapshot(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
            ),
            Err(RuntimeError::Conflict(_))
        ));
        assert_eq!(journal.read_calls(), 0);

        for mutate in [
            |record: &mut ConversationAgentAssignmentRecord| record.assignment_generation = 3,
            |record: &mut ConversationAgentAssignmentRecord| {
                record.assignment_source = AgentAssignmentSource::DefaultPolicy
            },
        ] {
            let first = normalized_assignment(conversation.conversation_id.as_str(), 0);
            let mut second = normalized_assignment(conversation.conversation_id.as_str(), 1);
            mutate(&mut second);
            let (runtime, journal) =
                assignment_runtime(Some(conversation.clone()), vec![first, second]);
            assert!(matches!(
                runtime.conversation_agent_assignments_snapshot(
                    conversation.tenant_id.as_str(),
                    conversation.organization_id.as_str(),
                    conversation.conversation_id.as_str(),
                ),
                Err(RuntimeError::Conflict(_))
            ));
            assert_eq!(journal.read_calls(), 0);
        }
    }

    #[test]
    fn normalized_assignment_snapshot_rejects_non_group_conversations_without_journal_reads() {
        let conversation = normalized_conversation("direct");
        let records = vec![normalized_assignment(
            conversation.conversation_id.as_str(),
            0,
        )];
        let (runtime, journal) = assignment_runtime(Some(conversation.clone()), records);

        assert!(matches!(
            runtime.conversation_agent_assignments_snapshot(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
            ),
            Err(RuntimeError::ConversationTypeInvalid(_))
        ));
        assert_eq!(journal.read_calls(), 0);
    }
}
