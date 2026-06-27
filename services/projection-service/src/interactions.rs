use std::collections::{BTreeMap, BTreeSet, HashMap};

use im_domain_core::message::{
    MessagePinned, MessageReactionAdded, MessageReactionRemoved, MessageUnpinned,
    ReactionActorIdentity,
};
use im_domain_events::CommitEnvelope;
use serde::{Deserialize, Serialize};

use crate::client_route_sync::ClientRouteSyncEntryDraft;
use crate::model::{InteractionActorView, MessagePinView};
use crate::scope::{projection_organization_id_for_event, scope_key, scope_key_for_event_conversation};
use crate::{
    MessageInteractionSummaryView, MessageReactionCountView, RealtimeFanoutTarget,
    TimelineProjectionService,
};

use super::projection::ProjectionError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct StoredMessagePinSummary {
    pub(super) pinned_by: InteractionActorView,
    pub(super) pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct StoredMessageInteractionSummary {
    pub(super) tenant_id: String,
    pub(super) conversation_id: String,
    pub(super) message_id: String,
    pub(super) message_seq: u64,
    pub(super) reactions: BTreeMap<String, BTreeSet<ReactionActorIdentity>>,
    pub(super) pin: Option<StoredMessagePinSummary>,
}

struct MessageInteractionFanoutContext {
    tenant_id: String,
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    actor: RealtimeFanoutTarget,
    summary: Option<String>,
    occurred_at: String,
}

impl TimelineProjectionService {
    pub fn message_interaction_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Option<MessageInteractionSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(view) =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store")
                .get(scope.as_str())
                .and_then(|scope_items| scope_items.get(message_id))
                .map(stored_interaction_to_view)
        {
            return Some(view);
        }

        let message_seq =
            self.timeline_message_seq(tenant_id, organization_id, conversation_id, message_id)?;
        Some(MessageInteractionSummaryView {
            tenant_id: tenant_id.into(),
            conversation_id: conversation_id.into(),
            message_id: message_id.into(),
            message_seq,
            total_reaction_count: 0,
            reaction_counts: Vec::new(),
            pin: None,
        })
    }

    pub fn pinned_messages(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<MessageInteractionSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let mut items =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store")
                .get(scope.as_str())
                .map(|scope_items| {
                    scope_items
                        .values()
                        .filter(|item| item.pin.is_some())
                        .map(stored_interaction_to_view)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
        items.sort_by(|left, right| {
            right
                .pin
                .as_ref()
                .map(|pin| pin.pinned_at.as_str())
                .cmp(&left.pin.as_ref().map(|pin| pin.pinned_at.as_str()))
                .then_with(|| right.message_seq.cmp(&left.message_seq))
                .then_with(|| left.message_id.cmp(&right.message_id))
        });
        items
    }

    pub(super) fn apply_message_reaction_added(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let reaction: MessageReactionAdded =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        let changed = self.upsert_message_interaction(
            reaction.tenant_id.as_str(),
            organization_id.as_str(),
            reaction.conversation_id.as_str(),
            reaction.message_id.as_str(),
            reaction.message_seq,
            |stored| {
                stored
                    .reactions
                    .entry(reaction.reaction_key.clone())
                    .or_default()
                    .insert(ReactionActorIdentity::from_sender(&reaction.reacted_by))
            },
        );
        if !changed {
            return Ok(());
        }

        self.fan_out_message_interaction_to_client_route_sync_feeds(
            event,
            MessageInteractionFanoutContext {
                tenant_id: reaction.tenant_id.clone(),
                conversation_id: reaction.conversation_id.clone(),
                message_id: reaction.message_id.clone(),
                message_seq: reaction.message_seq,
                actor: RealtimeFanoutTarget {
                    principal_id: reaction.reacted_by.id.clone(),
                    principal_kind: reaction.reacted_by.kind.clone(),
                    device_id: reaction.reacted_by.device_id.clone().unwrap_or_default(),
                },
                summary: Some(format!("reacted with {}", reaction.reaction_key)),
                occurred_at: reaction.reacted_at.clone(),
            },
        );
        self.record_projection_update_delay_for_scope(
            "message.reaction_added",
            scope_key_for_event_conversation(event, reaction.conversation_id.as_str()).as_str(),
            reaction.reacted_at.as_str(),
        );
        Ok(())
    }

    pub(super) fn apply_message_reaction_removed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let reaction: MessageReactionRemoved =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        let changed = self.mutate_existing_message_interaction(
            reaction.tenant_id.as_str(),
            organization_id.as_str(),
            reaction.conversation_id.as_str(),
            reaction.message_id.as_str(),
            |stored| {
                let Some(actor_ids) = stored.reactions.get_mut(reaction.reaction_key.as_str())
                else {
                    return false;
                };
                let changed =
                    actor_ids.remove(&ReactionActorIdentity::from_sender(&reaction.removed_by));
                if actor_ids.is_empty() {
                    stored.reactions.remove(reaction.reaction_key.as_str());
                }
                changed
            },
        );
        if !changed {
            return Ok(());
        }

        self.fan_out_message_interaction_to_client_route_sync_feeds(
            event,
            MessageInteractionFanoutContext {
                tenant_id: reaction.tenant_id.clone(),
                conversation_id: reaction.conversation_id.clone(),
                message_id: reaction.message_id.clone(),
                message_seq: reaction.message_seq,
                actor: RealtimeFanoutTarget {
                    principal_id: reaction.removed_by.id.clone(),
                    principal_kind: reaction.removed_by.kind.clone(),
                    device_id: reaction.removed_by.device_id.clone().unwrap_or_default(),
                },
                summary: Some(format!("removed reaction {}", reaction.reaction_key)),
                occurred_at: reaction.removed_at.clone(),
            },
        );
        self.record_projection_update_delay_for_scope(
            "message.reaction_removed",
            scope_key_for_event_conversation(event, reaction.conversation_id.as_str()).as_str(),
            reaction.removed_at.as_str(),
        );
        Ok(())
    }

    pub(super) fn apply_message_pinned(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let pin: MessagePinned =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        let changed = self.upsert_message_interaction(
            pin.tenant_id.as_str(),
            organization_id.as_str(),
            pin.conversation_id.as_str(),
            pin.message_id.as_str(),
            pin.message_seq,
            |stored| {
                if stored.pin.is_some() {
                    return false;
                }
                stored.pin = Some(StoredMessagePinSummary {
                    pinned_by: InteractionActorView {
                        id: pin.pinned_by.id.clone(),
                        kind: pin.pinned_by.kind.clone(),
                    },
                    pinned_at: pin.pinned_at.clone(),
                });
                true
            },
        );
        if !changed {
            return Ok(());
        }

        self.fan_out_message_interaction_to_client_route_sync_feeds(
            event,
            MessageInteractionFanoutContext {
                tenant_id: pin.tenant_id.clone(),
                conversation_id: pin.conversation_id.clone(),
                message_id: pin.message_id.clone(),
                message_seq: pin.message_seq,
                actor: RealtimeFanoutTarget {
                    principal_id: pin.pinned_by.id.clone(),
                    principal_kind: pin.pinned_by.kind.clone(),
                    device_id: pin.pinned_by.device_id.clone().unwrap_or_default(),
                },
                summary: Some("pinned message".into()),
                occurred_at: pin.pinned_at.clone(),
            },
        );
        self.record_projection_update_delay_for_scope(
            "message.pin_added",
            scope_key_for_event_conversation(event, pin.conversation_id.as_str()).as_str(),
            pin.pinned_at.as_str(),
        );
        Ok(())
    }

    pub(super) fn apply_message_unpinned(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let pin: MessageUnpinned =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        let changed = self.mutate_existing_message_interaction(
            pin.tenant_id.as_str(),
            organization_id.as_str(),
            pin.conversation_id.as_str(),
            pin.message_id.as_str(),
            |stored| stored.pin.take().is_some(),
        );
        if !changed {
            return Ok(());
        }

        self.fan_out_message_interaction_to_client_route_sync_feeds(
            event,
            MessageInteractionFanoutContext {
                tenant_id: pin.tenant_id.clone(),
                conversation_id: pin.conversation_id.clone(),
                message_id: pin.message_id.clone(),
                message_seq: pin.message_seq,
                actor: RealtimeFanoutTarget {
                    principal_id: pin.unpinned_by.id.clone(),
                    principal_kind: pin.unpinned_by.kind.clone(),
                    device_id: pin.unpinned_by.device_id.clone().unwrap_or_default(),
                },
                summary: Some("unpinned message".into()),
                occurred_at: pin.unpinned_at.clone(),
            },
        );
        self.record_projection_update_delay_for_scope(
            "message.pin_removed",
            scope_key_for_event_conversation(event, pin.conversation_id.as_str()).as_str(),
            pin.unpinned_at.as_str(),
        );
        Ok(())
    }

    fn timeline_message_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Option<u64> {
        super::lock_projection_mutex(&self.entries, "projection store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .and_then(|entries| {
                entries
                    .values()
                    .find(|entry| entry.message_id == message_id)
                    .map(|entry| entry.message_seq)
            })
    }

    fn upsert_message_interaction<F>(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        message_seq: u64,
        mutate: F,
    ) -> bool
    where
        F: FnOnce(&mut StoredMessageInteractionSummary) -> bool,
    {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let mut store =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store");
        let changed = mutate(
            store
                .entry(scope)
                .or_default()
                .entry(message_id.into())
                .or_insert_with(|| StoredMessageInteractionSummary {
                    tenant_id: tenant_id.into(),
                    conversation_id: conversation_id.into(),
                    message_id: message_id.into(),
                    message_seq,
                    reactions: BTreeMap::new(),
                    pin: None,
                }),
        );
        drop(store);
        self.prune_message_interaction(tenant_id, organization_id, conversation_id, message_id);
        changed
    }

    fn mutate_existing_message_interaction<F>(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        mutate: F,
    ) -> bool
    where
        F: FnOnce(&mut StoredMessageInteractionSummary) -> bool,
    {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let mut store =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store");
        let changed = store
            .get_mut(scope.as_str())
            .and_then(|scope_items| scope_items.get_mut(message_id))
            .is_some_and(mutate);
        drop(store);
        self.prune_message_interaction(tenant_id, organization_id, conversation_id, message_id);
        changed
    }

    fn prune_message_interaction(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let mut store =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store");
        let remove_scope = if let Some(scope_items) = store.get_mut(scope.as_str()) {
            let remove_item = scope_items
                .get(message_id)
                .is_some_and(|item| item.reactions.is_empty() && item.pin.is_none());
            if remove_item {
                scope_items.remove(message_id);
            }
            scope_items.is_empty()
        } else {
            false
        };
        if remove_scope {
            store.remove(scope.as_str());
        }
    }

    fn fan_out_message_interaction_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        context: MessageInteractionFanoutContext,
    ) {
        let MessageInteractionFanoutContext {
            tenant_id,
            conversation_id,
            message_id,
            message_seq,
            actor,
            summary,
            occurred_at,
        } = context;
        let actor_kind = actor.principal_kind.clone();
        let draft = ClientRouteSyncEntryDraft {
            tenant_id: tenant_id.clone(),
            organization_id: crate::scope::projection_organization_id_for_event(event),
            origin_event_id: event.event_id.clone(),
            origin_event_type: event.event_type.clone(),
            conversation_id: Some(conversation_id.clone()),
            message_id: Some(message_id),
            message_seq: Some(message_seq),
            member_id: None,
            read_seq: None,
            last_read_message_id: None,
            actor_id: Some(actor.principal_id.clone()),
            actor_kind: Some(actor_kind.clone()),
            actor_device_id: if actor.device_id.is_empty() {
                None
            } else {
                Some(actor.device_id.clone())
            },
            summary,
            payload_schema: event.payload_schema.clone(),
            payload: Some(event.payload.clone()),
            occurred_at,
        };

        for target in self.client_route_sync_fanout_targets_for_conversation(
            tenant_id.as_str(),
            crate::scope::projection_organization_id_for_event(event).as_str(),
            conversation_id.as_str(),
            vec![crate::NotificationRecipientView {
                principal_id: actor.principal_id,
                principal_kind: actor_kind,
            }],
        ) {
            self.append_client_route_sync_draft(&target, &draft);
        }
    }
}

pub(super) fn interaction_snapshot_items(
    scope_items: &HashMap<String, StoredMessageInteractionSummary>,
) -> Vec<StoredMessageInteractionSummary> {
    let mut items = scope_items.values().cloned().collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.message_seq
            .cmp(&right.message_seq)
            .then_with(|| left.message_id.cmp(&right.message_id))
    });
    items
}

pub(super) fn interaction_map_from_items(
    items: Vec<StoredMessageInteractionSummary>,
) -> HashMap<String, StoredMessageInteractionSummary> {
    items
        .into_iter()
        .map(|item| (item.message_id.clone(), item))
        .collect()
}

fn stored_interaction_to_view(
    stored: &StoredMessageInteractionSummary,
) -> MessageInteractionSummaryView {
    let reaction_counts = stored
        .reactions
        .iter()
        .map(|(reaction_key, actor_ids)| MessageReactionCountView {
            reaction_key: reaction_key.clone(),
            count: actor_ids.len() as u64,
        })
        .collect::<Vec<_>>();
    MessageInteractionSummaryView {
        tenant_id: stored.tenant_id.clone(),
        conversation_id: stored.conversation_id.clone(),
        message_id: stored.message_id.clone(),
        message_seq: stored.message_seq,
        total_reaction_count: reaction_counts.iter().map(|item| item.count).sum(),
        reaction_counts,
        pin: stored.pin.as_ref().map(|pin| MessagePinView {
            pinned_by: pin.pinned_by.clone(),
            pinned_at: pin.pinned_at.clone(),
        }),
    }
}
