use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::Bound::{Excluded, Unbounded};

use im_domain_core::message::{
    MessagePinned, MessageReactionAdded, MessageReactionRemoved, MessageUnpinned,
    ReactionActorIdentity,
};
use im_domain_events::CommitEnvelope;
use serde::{Deserialize, Serialize};

use crate::client_route_sync::ClientRouteSyncEntryDraft;
use crate::model::{InteractionActorView, MessagePinView, MessageReadReceiptSummaryView};
use crate::scope::{
    projection_organization_id_for_event, scope_key, scope_key_for_event_conversation,
    validate_conversation_projection_payload_scope,
};
use crate::{
    MessageInteractionSummaryView, MessageReactionCountView, RealtimeFanoutTarget,
    TimelineProjectionService,
};

use super::projection::ProjectionError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) struct PinnedMessageIndexKey {
    pinned_at: Reverse<String>,
    message_seq: Reverse<u64>,
    message_id: String,
}

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
    /// Batch-enrich timeline entries with inline interaction data (reactions, pin).
    ///
    /// This method looks up the in-memory interaction store for the given conversation
    /// scope and populates `reaction_counts` and `pin` on each entry. When the in-memory
    /// store is empty, it triggers a durable-store read-through to hydrate the cache
    /// before enrichment, ensuring reactions/pins recorded before a replica restart
    /// are still served inline.
    ///
    /// Unlike [`message_interaction_summary`], this method does **not** enrich
    /// per-principal read/delivery receipts — those remain available through the
    /// single-message `interaction_summary` endpoint for on-demand detail views.
    pub fn enrich_timeline_entries_with_interactions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        entries: &mut [crate::model::TimelineViewEntry],
    ) {
        if entries.is_empty() {
            return;
        }
        let scope = scope_key(tenant_id, organization_id, conversation_id);

        // Check whether the in-memory interaction store already has data for this scope.
        let need_durable_fallback = {
            let store = super::lock_projection_mutex(
                &self.message_interactions,
                "message interaction store",
            );
            store.get(scope.as_str()).is_none()
        };

        // If the in-memory store is empty, trigger a durable-store read-through.
        // `load_message_interaction_from_durable_store` loads the full conversation
        // interaction snapshot and hydrates the in-memory cache as a side effect.
        if need_durable_fallback {
            let first_message_id = entries[0].message_id.as_str();
            let _ = self.load_message_interaction_from_durable_store(&scope, first_message_id);
        }

        // Read the (possibly hydrated) in-memory interaction store.
        let scope_items = {
            let store = super::lock_projection_mutex(
                &self.message_interactions,
                "message interaction store",
            );
            store.get(scope.as_str()).cloned()
        };

        let Some(scope_items) = scope_items else {
            return;
        };

        for entry in entries.iter_mut() {
            if let Some(stored) = scope_items.get(entry.message_id.as_str()) {
                entry.reaction_counts = stored
                    .reactions
                    .iter()
                    .map(
                        |(reaction_key, actor_ids)| crate::MessageReactionCountView {
                            reaction_key: reaction_key.clone(),
                            count: actor_ids.len() as u64,
                        },
                    )
                    .collect::<Vec<_>>();
                entry.pin = stored.pin.as_ref().map(|pin| crate::model::MessagePinView {
                    pinned_by: pin.pinned_by.clone(),
                    pinned_at: pin.pinned_at.clone(),
                });
            }
        }
    }

    pub fn message_interaction_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Option<MessageInteractionSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let view = if let Some(view) =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store")
                .get(scope.as_str())
                .and_then(|scope_items| scope_items.get(message_id))
                .map(stored_interaction_to_view)
        {
            view
        } else if let Some(view) =
            self.load_message_interaction_from_durable_store(scope.as_str(), message_id)
        {
            view
        } else {
            let message_seq =
                self.timeline_message_seq(tenant_id, organization_id, conversation_id, message_id)?;
            MessageInteractionSummaryView {
                tenant_id: tenant_id.into(),
                conversation_id: conversation_id.into(),
                message_id: message_id.into(),
                message_seq,
                total_reaction_count: 0,
                reaction_counts: Vec::new(),
                pin: None,
                read_receipt: MessageReadReceiptSummaryView::default(),
                delivery_receipt: Default::default(),
            }
        };
        Some(self.enrich_interaction_summary_with_receipts(
            view,
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        ))
    }

    /// Read-through fallback: load the full conversation message-interactions
    /// snapshot from the durable metadata store, hydrate the in-memory
    /// interaction store, then return the matching interaction view. This
    /// ensures reactions/pins recorded before an in-memory prune or replica
    /// restart are still served without a 404. Returns `None` when the durable
    /// store is not configured, the snapshot is absent, the load fails
    /// (warn-logged), or the snapshot does not contain `message_id`.
    fn load_message_interaction_from_durable_store(
        &self,
        scope: &str,
        message_id: &str,
    ) -> Option<MessageInteractionSummaryView> {
        let store = self.durable_metadata_store()?;
        let interactions =
            match crate::snapshot::load_metadata_snapshot::<Vec<StoredMessageInteractionSummary>>(
                store.as_ref(),
                scope,
                crate::snapshot::MESSAGE_INTERACTIONS_KEY,
            ) {
                Ok(Some(interactions)) => interactions,
                Ok(None) => return None,
                Err(error) => {
                    tracing::warn!(
                        target: "sdkwork.im.projection.read_through",
                        event = "im.projection.interactions_durable_read_failed",
                        scope = %scope,
                        error = %error,
                        "durable metadata read-through failed for message interactions",
                    );
                    return None;
                }
            };
        let hydrated_map = interaction_map_from_items(interactions);
        let view = hydrated_map.get(message_id).map(stored_interaction_to_view);
        if !hydrated_map.is_empty() {
            super::lock_projection_mutex(&self.message_interactions, "message interaction store")
                .insert(scope.to_owned(), hydrated_map);
        }
        view
    }

    pub(crate) fn enrich_interaction_summary_with_receipts(
        &self,
        mut view: MessageInteractionSummaryView,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> MessageInteractionSummaryView {
        view = self.enrich_interaction_summary_with_read_receipt(
            view,
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        );
        self.enrich_interaction_summary_with_delivery_receipt(
            view,
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        )
    }

    pub(crate) fn enrich_interaction_summary_with_delivery_receipt(
        &self,
        mut view: MessageInteractionSummaryView,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> MessageInteractionSummaryView {
        let exclude_sender =
            self.timeline_message_sender(tenant_id, organization_id, conversation_id, message_id);
        view.delivery_receipt = self.delivery_receipt_summary_for_message(
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
            exclude_sender
                .as_ref()
                .map(|(principal_id, principal_kind)| {
                    (principal_id.as_str(), principal_kind.as_str())
                }),
        );
        view
    }

    pub(crate) fn enrich_interaction_summary_with_read_receipt(
        &self,
        mut view: MessageInteractionSummaryView,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> MessageInteractionSummaryView {
        let exclude_sender =
            self.timeline_message_sender(tenant_id, organization_id, conversation_id, message_id);
        view.read_receipt = self.read_receipt_summary_for_message(
            tenant_id,
            organization_id,
            conversation_id,
            view.message_seq,
            exclude_sender
                .as_ref()
                .map(|(principal_id, principal_kind)| {
                    (principal_id.as_str(), principal_kind.as_str())
                }),
        );
        view
    }

    pub fn pinned_messages(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<MessageInteractionSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let index_keys = {
            let index =
                super::lock_projection_mutex(&self.pinned_messages_index, "pinned message index");
            index.get(scope.as_str()).cloned().unwrap_or_default()
        };
        let scope_items = {
            let store = super::lock_projection_mutex(
                &self.message_interactions,
                "message interaction store",
            );
            store.get(scope.as_str()).cloned().unwrap_or_default()
        };
        index_keys
            .iter()
            .filter_map(|key| {
                scope_items
                    .get(key.message_id.as_str())
                    .filter(|item| item.pin.is_some())
                    .map(stored_interaction_to_view)
            })
            .collect()
    }

    pub(crate) fn pinned_messages_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        page_size: usize,
        cursor: crate::model::PinnedMessagesListCursor,
    ) -> sdkwork_utils_rust::SdkWorkPageData<MessageInteractionSummaryView> {
        use sdkwork_utils_rust::cursor_window_page_info;
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let list_cursor = match cursor {
            crate::model::PinnedMessagesListCursor::Start => None,
            other => Some(other),
        };
        let index =
            super::lock_projection_mutex(&self.pinned_messages_index, "pinned message index");
        let store =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store");
        let index_keys = index.get(scope.as_str()).cloned().unwrap_or_default();
        let scope_items = store.get(scope.as_str()).cloned().unwrap_or_default();
        drop(index);
        drop(store);

        let (items, has_more) = pinned_messages_window_slice_from_index(
            &index_keys,
            &scope_items,
            list_cursor,
            page_size,
        );
        let next_cursor = if has_more {
            items.last().and_then(|item| {
                let pin = item.pin.as_ref()?;
                let payload = serde_json::json!({
                    "pinnedAt": pin.pinned_at,
                    "messageSeq": item.message_seq,
                    "messageId": item.message_id,
                });
                crate::cursor_auth::encode_signed_projection_cursor(&payload).ok()
            })
        } else {
            None
        };
        sdkwork_utils_rust::SdkWorkPageData {
            items,
            page_info: cursor_window_page_info(Some(page_size), next_cursor, has_more),
        }
    }

    pub(super) fn apply_message_reaction_added(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let reaction: MessageReactionAdded =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        validate_conversation_projection_payload_scope(
            event,
            reaction.tenant_id.as_str(),
            reaction.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            reaction.tenant_id.as_str(),
            reaction.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            pin.tenant_id.as_str(),
            pin.conversation_id.as_str(),
        )?;
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

        self.register_pinned_message_index(
            scope_key(
                pin.tenant_id.as_str(),
                organization_id.as_str(),
                pin.conversation_id.as_str(),
            )
            .as_str(),
            pin.message_id.as_str(),
            pin.message_seq,
            pin.pinned_at.as_str(),
        );

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
        validate_conversation_projection_payload_scope(
            event,
            pin.tenant_id.as_str(),
            pin.conversation_id.as_str(),
        )?;
        let organization_id = projection_organization_id_for_event(event);
        let scope = scope_key(
            pin.tenant_id.as_str(),
            organization_id.as_str(),
            pin.conversation_id.as_str(),
        );
        let removed_pin_at =
            super::lock_projection_mutex(&self.message_interactions, "message interaction store")
                .get(scope.as_str())
                .and_then(|scope_items| scope_items.get(pin.message_id.as_str()))
                .and_then(|stored| stored.pin.as_ref().map(|summary| summary.pinned_at.clone()));
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

        if let Some(pinned_at) = removed_pin_at.as_deref() {
            self.unregister_pinned_message_index(
                scope.as_str(),
                pin.message_id.as_str(),
                pin.message_seq,
                pinned_at,
            );
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

    fn register_pinned_message_index(
        &self,
        scope: &str,
        message_id: &str,
        message_seq: u64,
        pinned_at: &str,
    ) {
        super::lock_projection_mutex(&self.pinned_messages_index, "pinned message index")
            .entry(scope.to_owned())
            .or_default()
            .insert(PinnedMessageIndexKey {
                pinned_at: Reverse(pinned_at.to_owned()),
                message_seq: Reverse(message_seq),
                message_id: message_id.to_owned(),
            });
    }

    fn unregister_pinned_message_index(
        &self,
        scope: &str,
        message_id: &str,
        message_seq: u64,
        pinned_at: &str,
    ) {
        let mut index_store =
            super::lock_projection_mutex(&self.pinned_messages_index, "pinned message index");
        if let Some(index) = index_store.get_mut(scope) {
            index.remove(&PinnedMessageIndexKey {
                pinned_at: Reverse(pinned_at.to_owned()),
                message_seq: Reverse(message_seq),
                message_id: message_id.to_owned(),
            });
            if index.is_empty() {
                index_store.remove(scope);
            }
        }
    }

    pub(super) fn rebuild_pinned_messages_index_for_scope(
        &self,
        scope: &str,
        scope_items: &HashMap<String, StoredMessageInteractionSummary>,
    ) {
        let mut index = BTreeSet::new();
        for stored in scope_items.values() {
            let Some(pin) = stored.pin.as_ref() else {
                continue;
            };
            index.insert(PinnedMessageIndexKey {
                pinned_at: Reverse(pin.pinned_at.clone()),
                message_seq: Reverse(stored.message_seq),
                message_id: stored.message_id.clone(),
            });
        }
        let mut pinned_index =
            super::lock_projection_mutex(&self.pinned_messages_index, "pinned message index");
        if index.is_empty() {
            pinned_index.remove(scope);
        } else {
            pinned_index.insert(scope.to_owned(), index);
        }
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

pub(super) fn pinned_messages_window_slice_from_index(
    index_keys: &BTreeSet<PinnedMessageIndexKey>,
    scope_items: &HashMap<String, StoredMessageInteractionSummary>,
    cursor: Option<crate::model::PinnedMessagesListCursor>,
    limit: usize,
) -> (Vec<MessageInteractionSummaryView>, bool) {
    let limit = limit.max(1);
    let mut window = Vec::with_capacity(limit.saturating_add(1));
    let legacy_offset = matches!(
        cursor,
        Some(crate::model::PinnedMessagesListCursor::Offset(_))
    );
    let offset = match cursor {
        Some(crate::model::PinnedMessagesListCursor::Offset(value)) => value,
        _ => 0,
    };
    let index_iter: Box<dyn Iterator<Item = &PinnedMessageIndexKey>> = match cursor {
        Some(crate::model::PinnedMessagesListCursor::Keyset {
            pinned_at,
            message_seq,
            message_id,
        }) => {
            let cursor_key = PinnedMessageIndexKey {
                pinned_at: Reverse(pinned_at),
                message_seq: Reverse(message_seq),
                message_id: message_id.clone(),
            };
            Box::new(index_keys.range((Excluded(cursor_key), Unbounded)))
        }
        _ => Box::new(index_keys.iter()),
    };
    let mut skipped = 0usize;
    for key in index_iter {
        let Some(stored) = scope_items.get(key.message_id.as_str()) else {
            continue;
        };
        if stored.pin.is_none() {
            continue;
        }
        if legacy_offset && skipped < offset {
            skipped += 1;
            continue;
        }
        window.push(stored_interaction_to_view(stored));
        if window.len() > limit {
            break;
        }
    }
    let has_more = window.len() > limit;
    if has_more {
        window.truncate(limit);
    }
    (window, has_more)
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
        read_receipt: MessageReadReceiptSummaryView::default(),
        delivery_receipt: Default::default(),
    }
}
