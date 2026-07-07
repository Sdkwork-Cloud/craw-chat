use im_domain_core::conversation::ConversationReadCursor;
use im_domain_core::message::{ContentPart, Message};
use im_domain_events::CommitEnvelope;

use crate::client_route_sync::ClientRouteSyncEntryDraft;
use crate::model::NotificationRecipientView;
use crate::projection::AgentHandoffStatusChangedProjectionPayload;
use crate::{TimelineProjectionService, client_route_sync, scope};

impl TimelineProjectionService {
    pub(super) fn fan_out_message_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        message: &Message,
    ) {
        let (payload_schema, payload) = message_client_route_sync_payload(event, message);
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: message.tenant_id.clone(),
            organization_id: scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(message.conversation_id.clone()),
            message_id: Some(message.message_id.clone()),
            message_seq: Some(message.message_seq),
            member_id: message.sender.member_id.clone(),
            read_seq: None,
            last_read_message_id: None,
            actor_id: Some(message.sender.id.clone()),
            actor_kind: Some(message.sender.kind.clone()),
            actor_device_id: message.sender.device_id.clone(),
            summary: message.body.summary.clone(),
            payload_schema,
            payload,
            occurred_at: message
                .committed_at
                .clone()
                .unwrap_or_else(|| message.occurred_at.clone()),
        };

        for target in self.client_route_sync_fanout_targets_for_conversation(
            message.tenant_id.as_str(),
            scope::projection_organization_id_for_event(event).as_str(),
            message.conversation_id.as_str(),
            vec![NotificationRecipientView {
                principal_id: message.sender.id.clone(),
                principal_kind: message.sender.kind.clone(),
            }],
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }

    // These fanout helpers keep event and conversation identity fields explicit
    // because they bridge journal payloads into client-route-sync artifacts.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn fan_out_message_mutation_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        tenant_id: &str,
        conversation_id: &str,
        message_id: &str,
        message_seq: u64,
        actor_id: &str,
        actor_kind: &str,
        actor_device_id: Option<String>,
        summary: Option<String>,
    ) {
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: tenant_id.into(),
            organization_id: scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(conversation_id.into()),
            message_id: Some(message_id.into()),
            message_seq: Some(message_seq),
            member_id: None,
            read_seq: None,
            last_read_message_id: None,
            actor_id: Some(actor_id.into()),
            actor_kind: Some(actor_kind.into()),
            actor_device_id,
            summary,
            payload_schema: None,
            payload: None,
            occurred_at: event.committed_at.clone(),
        };

        for target in self.client_route_sync_fanout_targets_for_conversation(
            tenant_id,
            scope::projection_organization_id_for_event(event).as_str(),
            conversation_id,
            vec![NotificationRecipientView {
                principal_id: actor_id.into(),
                principal_kind: actor_kind.into(),
            }],
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }

    pub(super) fn fan_out_read_cursor_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        cursor: &ConversationReadCursor,
    ) {
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: cursor.tenant_id.clone(),
            organization_id: scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(cursor.conversation_id.clone()),
            message_id: None,
            message_seq: None,
            member_id: Some(cursor.member_id.clone()),
            read_seq: Some(cursor.read_seq),
            last_read_message_id: cursor.last_read_message_id.clone(),
            actor_id: Some(cursor.principal_id.clone()),
            actor_kind: Some(cursor.principal_kind.clone()),
            actor_device_id: None,
            summary: None,
            payload_schema: None,
            payload: None,
            occurred_at: cursor.updated_at.clone(),
        };

        for target in client_route_sync::realtime_fanout_targets_for_recipients(
            self,
            cursor.tenant_id.as_str(),
            scope::projection_organization_id_for_event(event).as_str(),
            vec![NotificationRecipientView {
                principal_id: cursor.principal_id.clone(),
                principal_kind: cursor.principal_kind.clone(),
            }],
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }

    pub(super) fn fan_out_agent_handoff_status_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        payload: &AgentHandoffStatusChangedProjectionPayload,
    ) {
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: event.tenant_id.clone(),
            organization_id: scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(payload.state.conversation_id.clone()),
            message_id: None,
            message_seq: None,
            member_id: None,
            read_seq: None,
            last_read_message_id: None,
            actor_id: Some(payload.changed_by.id.clone()),
            actor_kind: Some(payload.changed_by.kind.clone()),
            actor_device_id: None,
            summary: Some(payload.state.status.clone()),
            payload_schema: event.payload_schema.clone(),
            payload: Some(event.payload.clone()),
            occurred_at: payload.changed_at.clone(),
        };

        for target in self.client_route_sync_fanout_targets_for_conversation(
            event.tenant_id.as_str(),
            scope::projection_organization_id_for_event(event).as_str(),
            payload.state.conversation_id.as_str(),
            vec![NotificationRecipientView {
                principal_id: payload.changed_by.id.clone(),
                principal_kind: payload.changed_by.kind.clone(),
            }],
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn fan_out_member_governance_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        tenant_id: &str,
        conversation_id: &str,
        member_id: &str,
        affected_principal_id: &str,
        affected_principal_kind: &str,
        include_affected_principal_fallback: bool,
        occurred_at: &str,
    ) {
        let organization_id = scope::projection_organization_id_for_event(event);
        let include_fallback = include_affected_principal_fallback
            || client_route_sync::active_conversation_principal_recipients(
                self,
                tenant_id,
                organization_id.as_str(),
                conversation_id,
            )
            .is_empty();
        let fallback_recipients = if include_fallback {
            vec![NotificationRecipientView {
                principal_id: affected_principal_id.into(),
                principal_kind: affected_principal_kind.into(),
            }]
        } else {
            Vec::new()
        };
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: tenant_id.into(),
            organization_id: scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(conversation_id.into()),
            message_id: None,
            message_seq: None,
            member_id: Some(member_id.into()),
            read_seq: None,
            last_read_message_id: None,
            actor_id: Some(event.actor.actor_id.clone()),
            actor_kind: Some(event.actor.actor_kind.clone()),
            actor_device_id: None,
            summary: None,
            payload_schema: event.payload_schema.clone(),
            payload: Some(event.payload.clone()),
            occurred_at: occurred_at.into(),
        };

        for target in self.client_route_sync_fanout_targets_for_conversation(
            tenant_id,
            scope::projection_organization_id_for_event(event).as_str(),
            conversation_id,
            fallback_recipients,
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }
}

fn message_client_route_sync_payload(
    event: &CommitEnvelope,
    message: &Message,
) -> (Option<String>, Option<String>) {
    if !message_requires_client_route_sync_payload(message) {
        return (None, None);
    }

    (event.payload_schema.clone(), Some(event.payload.clone()))
}

fn message_requires_client_route_sync_payload(message: &Message) -> bool {
    message.rtc_session_id.is_some()
        || message.message_type == im_domain_core::message::MessageType::Signal
        || message
            .body
            .render_hints
            .get("channel")
            .is_some_and(|channel| channel == "rtc")
        || message.body.reply_to.is_some()
        || !message.body.render_hints.is_empty()
        || message
            .body
            .parts
            .iter()
            .any(|part| !matches!(part, ContentPart::Text(_)))
}
