//! Durable lifecycle transitions for group Conversations.
//!
//! Archiving is intentionally separate from client-side hide/delete actions.
//! It is an Owner-only aggregate transition that blocks future group writes
//! and gives the Knowledgebase coordinator an immutable journal source event
//! for its archive outbox handoff.

use im_app_context::AppContext;
use im_domain_core::conversation::MembershipRole;
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};

use super::*;

const GROUP_ARCHIVE_EVENT_TYPE: &str = "conversation.group_archived";
const GROUP_ARCHIVE_PAYLOAD_SCHEMA: &str = "conversation.group_archived.v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveGroupConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub archived_by: String,
    pub idempotency_key: String,
}

impl ArchiveGroupConversationCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        idempotency_key: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            archived_by: auth.actor_id.clone(),
            idempotency_key,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveGroupConversationResult {
    pub conversation_id: String,
    pub event_id: String,
    pub archived_at: String,
    pub archived_by: String,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationGroupArchivedPayload {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub archived_by: String,
    pub archived_by_kind: String,
    pub archived_at: String,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn archive_group_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: String,
        idempotency_key: String,
    ) -> Result<ArchiveGroupConversationResult, RuntimeError> {
        self.archive_group_conversation_with_actor_kind(
            ArchiveGroupConversationCommand::from_auth_context(
                auth,
                conversation_id,
                idempotency_key,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn archive_group_conversation_with_actor_kind(
        &self,
        command: ArchiveGroupConversationCommand,
        actor_kind: &str,
    ) -> Result<ArchiveGroupConversationResult, RuntimeError> {
        validate_group_archive_command(&command, actor_kind)?;
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
            actor_kind,
            command.archived_by.as_str(),
        )?;

        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let mut state =
            write_runtime_state(&self.state, "conversation-runtime.state.group-archive");
        state.touch_conversation(scope_key.as_str());
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(command.conversation_id.clone()))?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::ConversationTypeInvalid(
                "group archive requires a group conversation".into(),
            ));
        }

        let actor = resolve_active_member_with_kind(
            conversation,
            command.archived_by.as_str(),
            actor_kind,
        )?;
        if !matches!(actor.role, MembershipRole::Owner) {
            return Err(RuntimeError::PermissionDenied(
                "only the group owner can archive a group conversation".into(),
            ));
        }
        if conversation.aggregate.is_archived() {
            return Ok(ArchiveGroupConversationResult {
                conversation_id: command.conversation_id,
                event_id: conversation
                    .aggregate
                    .archive_event_id()
                    .unwrap_or_default()
                    .to_owned(),
                archived_at: conversation
                    .aggregate
                    .archived_at()
                    .unwrap_or_default()
                    .to_owned(),
                archived_by: command.archived_by,
                applied: false,
            });
        }

        let ordering_seq = conversation
            .aggregate
            .commit_seq()
            .checked_add(1)
            .ok_or_else(|| {
                RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
            })?;
        let archived_at = conversation_timestamp();
        let event_id = group_archive_event_id(&command, actor_kind);
        let payload = ConversationGroupArchivedPayload {
            tenant_id: command.tenant_id.clone(),
            organization_id: command.organization_id.clone(),
            conversation_id: command.conversation_id.clone(),
            archived_by: command.archived_by.clone(),
            archived_by_kind: actor_kind.to_owned(),
            archived_at: archived_at.clone(),
        };
        let retention_class = conversation_retention_class(conversation);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            organization_id: command.organization_id.clone(),
            event_type: GROUP_ARCHIVE_EVENT_TYPE.into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq,
            causation_id: None,
            correlation_id: None,
            idempotency_key: Some(command.idempotency_key.clone()),
            actor: EventActor {
                actor_id: command.archived_by.clone(),
                actor_kind: actor_kind.to_owned(),
                actor_session_id: None,
            },
            occurred_at: archived_at.clone(),
            committed_at: archived_at.clone(),
            payload_schema: Some(GROUP_ARCHIVE_PAYLOAD_SCHEMA.into()),
            payload: runtime_json_string(&payload)?,
            retention_class,
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        conversation
            .aggregate
            .apply_archive(archived_at.clone(), event_id.clone(), ordering_seq);

        Ok(ArchiveGroupConversationResult {
            conversation_id: command.conversation_id,
            event_id,
            archived_at,
            archived_by: command.archived_by,
            applied: true,
        })
    }
}

/// Central guard for group mutations. Reads remain available for audit/history
/// flows, while message publishing and membership changes fail closed once the
/// immutable archive event has been committed.
pub(super) fn ensure_conversation_write_allowed(
    conversation: &ConversationState,
) -> Result<(), RuntimeError> {
    if conversation.aggregate.conversation_type() == "group" && conversation.aggregate.is_archived()
    {
        return Err(RuntimeError::Conflict(
            "group conversation is archived and no longer accepts writes".into(),
        ));
    }
    Ok(())
}

fn validate_group_archive_command(
    command: &ArchiveGroupConversationCommand,
    actor_kind: &str,
) -> Result<(), RuntimeError> {
    for (field, value, max_bytes) in [
        (
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        ),
        (
            "archivedBy",
            command.archived_by.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        ),
        (
            "idempotencyKey",
            command.idempotency_key.as_str(),
            CONVERSATION_MAX_REQUEST_KEY_BYTES,
        ),
        ("actorKind", actor_kind, CONVERSATION_MAX_KIND_BYTES),
    ] {
        validate_payload_size(field, value, max_bytes)?;
        if value.trim().is_empty() || value.trim() != value {
            return Err(RuntimeError::InvalidInput(format!("{field} is invalid")));
        }
    }
    Ok(())
}

fn group_archive_event_id(command: &ArchiveGroupConversationCommand, actor_kind: &str) -> String {
    let digest = sha256_hash(
        format!(
            "group-archive:{}:{}:{}:{}:{}:{}",
            command.tenant_id,
            command.organization_id,
            command.conversation_id,
            actor_kind,
            command.archived_by,
            command.idempotency_key,
        )
        .as_bytes(),
    );
    format!("evt_group_archived_{}", &digest[..24])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_archive_event_ids_are_idempotency_scoped_and_opaque() {
        let command = ArchiveGroupConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "g_example".into(),
            archived_by: "user1".into(),
            idempotency_key: "archive-request-1".into(),
        };
        let event_id = group_archive_event_id(&command, "user");
        assert!(event_id.starts_with("evt_group_archived_"));
        assert!(!event_id.contains("user1"));
        assert_eq!(event_id, group_archive_event_id(&command, "user"));
    }
}
