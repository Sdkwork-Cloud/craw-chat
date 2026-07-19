//! Durable agent-mention handoff owned by the conversation message plane.

use im_domain_core::conversation::ConversationAgentAssignment;
use im_domain_core::message::Message;
use im_domain_events::normalize_commit_organization_id;
use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE,
    AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA, AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
    AgentMentionDispatchRequest, AgentMentionDispatchTarget, CommitJournal, OutboxEventRecord,
    OutboxPublishStatus,
};

use super::*;

const AGENT_DISPATCH_ID_DIGEST_LEN: usize = 32;

pub(super) struct AgentMentionDispatchArtifacts {
    pub envelope: CommitEnvelope,
    pub outbox: Option<OutboxEventRecord>,
    pub request: AgentMentionDispatchRequest,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub(super) fn build_agent_mention_dispatch_artifacts(
        &self,
        organization_id: &str,
        message: &Message,
        causation_event_id: &str,
        assignments: &[ConversationAgentAssignment],
        ordering_seq: u64,
        retention_class: &str,
    ) -> Result<Option<AgentMentionDispatchArtifacts>, RuntimeError> {
        if assignments.is_empty() {
            return Ok(None);
        }
        let organization_id = normalize_commit_organization_id(organization_id);
        let assignment_generation = message
            .body
            .parts
            .iter()
            .filter_map(ContentPart::as_mention)
            .next()
            .map(|mention| mention.assignment_generation)
            .ok_or_else(|| {
                RuntimeError::Conflict(
                    "resolved agent mentions are missing a structured mention part".into(),
                )
            })?;
        let targets = assignments
            .iter()
            .map(|assignment| AgentMentionDispatchTarget {
                dispatch_id: deterministic_agent_dispatch_id(
                    organization_id.as_str(),
                    message,
                    assignment_generation,
                    assignment,
                ),
                agent_id: assignment.agent_id.clone(),
                revision_id: assignment.revision_id.clone(),
            })
            .collect::<Vec<_>>();
        let request = AgentMentionDispatchRequest {
            schema_version: AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
            tenant_id: message.tenant_id.clone(),
            organization_id: organization_id.clone(),
            conversation_id: message.conversation_id.clone(),
            message_id: message.message_id.clone(),
            message_seq: message.message_seq,
            causation_event_id: causation_event_id.to_owned(),
            sender_principal_id: message.sender.id.clone(),
            sender_principal_kind: message.sender.kind.clone(),
            assignment_generation,
            targets,
            body: message.body.clone(),
            requested_at: message.occurred_at.clone(),
        };
        request.validate().map_err(RuntimeError::from)?;
        let payload = runtime_json_string(&request)?;
        let event_id = deterministic_agent_dispatch_event_id(organization_id.as_str(), message);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: message.tenant_id.clone(),
            organization_id: organization_id.clone(),
            event_type: AGENT_MENTION_DISPATCH_EVENT_TYPE.into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: message.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: message.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                message.tenant_id.as_str(),
                message.conversation_id.as_str(),
            ),
            ordering_seq,
            causation_id: Some(causation_event_id.to_owned()),
            correlation_id: Some(causation_event_id.to_owned()),
            idempotency_key: Some(message.message_id.clone()),
            actor: EventActor {
                actor_id: message.sender.id.clone(),
                actor_kind: message.sender.kind.clone(),
                actor_session_id: message.sender.session_id.clone(),
            },
            occurred_at: message.occurred_at.clone(),
            committed_at: message
                .committed_at
                .clone()
                .unwrap_or_else(|| message.occurred_at.clone()),
            payload_schema: Some(AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA.into()),
            payload: payload.clone(),
            retention_class: retention_class.into(),
            audit_class: "default".into(),
        };
        let outbox = self.build_agent_mention_dispatch_outbox_record(
            organization_id.as_str(),
            message,
            event_id.as_str(),
            payload,
        )?;
        Ok(Some(AgentMentionDispatchArtifacts {
            envelope,
            outbox,
            request,
        }))
    }

    fn build_agent_mention_dispatch_outbox_record(
        &self,
        organization_id: &str,
        message: &Message,
        journal_event_id: &str,
        payload_json: String,
    ) -> Result<Option<OutboxEventRecord>, RuntimeError> {
        if self.outbox_store.is_none() {
            return Ok(None);
        }
        // The journal event id is deterministic for a source message. Derive
        // the outbox identity from it as well so a replayed command or a
        // recovery rebuild cannot enqueue a second logical dispatch with a
        // fresh Snowflake id.
        let outbox_id = deterministic_agent_dispatch_outbox_id(journal_event_id);
        let payload_hash = sha256_hash(payload_json.as_bytes());
        let now = message
            .committed_at
            .clone()
            .unwrap_or_else(|| message.occurred_at.clone());
        Ok(Some(OutboxEventRecord {
            tenant_id: message.tenant_id.clone(),
            organization_id: organization_id.to_owned(),
            outbox_id,
            aggregate_type: AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: message.conversation_id.clone(),
            event_id: format!("agent-dispatch:{journal_event_id}"),
            event_type: AGENT_MENTION_DISPATCH_EVENT_TYPE.into(),
            payload_json,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        }))
    }
}

fn deterministic_agent_dispatch_outbox_id(journal_event_id: &str) -> String {
    let seed = encode_conversation_key_segments(["agent-dispatch-outbox", journal_event_id]);
    let digest = sha256_hash(seed.as_bytes());
    format!("amd_ob_{}", &digest[..AGENT_DISPATCH_ID_DIGEST_LEN])
}

pub(super) fn deterministic_agent_dispatch_id(
    organization_id: &str,
    message: &Message,
    assignment_generation: u64,
    assignment: &ConversationAgentAssignment,
) -> String {
    let generation = assignment_generation.to_string();
    let seed = encode_conversation_key_segments([
        message.tenant_id.as_str(),
        organization_id,
        message.conversation_id.as_str(),
        message.message_id.as_str(),
        generation.as_str(),
        assignment.agent_id.as_str(),
        assignment.revision_id.as_deref().unwrap_or_default(),
    ]);
    let digest = sha256_hash(seed.as_bytes());
    format!("amd_{}", &digest[..AGENT_DISPATCH_ID_DIGEST_LEN])
}

pub(super) fn deterministic_agent_dispatch_event_id(
    organization_id: &str,
    message: &Message,
) -> String {
    let seed = encode_conversation_key_segments([
        message.tenant_id.as_str(),
        organization_id,
        message.conversation_id.as_str(),
        message.message_id.as_str(),
        AGENT_MENTION_DISPATCH_EVENT_TYPE,
    ]);
    let digest = sha256_hash(seed.as_bytes());
    format!(
        "evt_{}_agent_mentions",
        &digest[..AGENT_DISPATCH_ID_DIGEST_LEN]
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use im_domain_core::message::{MessageBody, MessageType, Sender};

    use super::*;

    fn source_message() -> Message {
        Message {
            tenant_id: "100001".into(),
            conversation_id: "g_dispatch".into(),
            message_id: "90001".into(),
            message_seq: 7,
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("member_1".into()),
                device_id: None,
                session_id: None,
                metadata: BTreeMap::new(),
            },
            message_type: MessageType::Standard,
            delivery_mode: "discrete".into(),
            client_msg_id: Some("client_dispatch".into()),
            stream_session_id: None,
            rtc_session_id: None,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: BTreeMap::new(),
                reply_to: None,
            },
            attributes: BTreeMap::new(),
            metadata: BTreeMap::new(),
            occurred_at: "2026-07-12T00:00:00Z".into(),
            committed_at: Some("2026-07-12T00:00:00Z".into()),
        }
    }

    #[test]
    fn dispatch_identity_is_stable_and_revision_scoped() {
        let message = source_message();
        let assignment = ConversationAgentAssignment::new(
            "agent.im.writer",
            Some("revision.im.writer.1".into()),
        );
        let first = deterministic_agent_dispatch_id("0", &message, 3, &assignment);
        let replay = deterministic_agent_dispatch_id("0", &message, 3, &assignment);
        let next_revision = deterministic_agent_dispatch_id(
            "0",
            &message,
            3,
            &ConversationAgentAssignment::new(
                "agent.im.writer",
                Some("revision.im.writer.2".into()),
            ),
        );
        let other_organization =
            deterministic_agent_dispatch_id("organization_b", &message, 3, &assignment);

        assert_eq!(first, replay);
        assert_ne!(first, next_revision);
        assert_ne!(first, other_organization);

        let event_id = deterministic_agent_dispatch_event_id("0", &message);
        assert_eq!(
            deterministic_agent_dispatch_outbox_id(event_id.as_str()),
            deterministic_agent_dispatch_outbox_id(event_id.as_str())
        );
        assert_ne!(
            deterministic_agent_dispatch_outbox_id(event_id.as_str()),
            deterministic_agent_dispatch_outbox_id("evt_other_agent_dispatch")
        );
    }
}
