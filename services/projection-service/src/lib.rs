use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Mutex, MutexGuard};

use im_domain_core::conversation::{
    ClientRouteSyncFeedEntry, ConversationMember, ConversationReadCursor,
    ConversationReadCursorView,
};
use im_domain_core::message::{ContentPart, Message, MessageEdited, MessageRecalled};
use im_domain_core::retention::{is_retention_expired, retention_until_from_envelope};
use im_domain_events::CommitEnvelope;
use im_time::utc_now_rfc3339_millis;

mod access;
mod bootstrap;
pub mod embedded_bridge;
mod client_route_sync;
mod journal_consumer;
mod contacts;
mod conversation_personalization;
pub mod http;
mod inbox;
mod interactions;
mod member_directory;
mod member_store;
mod message_favorites;
mod model;
mod observability;
mod projection;
mod received_message_index;
mod scope;
mod snapshot;
mod summary_updates;
mod update_delay;

use client_route_sync::ClientRouteSyncEntryDraft;
use member_store::ProjectionMemberRuntimeStore;
use model::ConversationCatalogEntry;
use observability::ProjectionObservabilityState;
use projection::{
    AgentHandoffStatusChangedProjectionPayload, ConversationCreatedPayload,
    ConversationMemberRoleChangedPayload, ConversationPolicyAppliedProjectionPayload,
    handoff_view_from_created_payload, handoff_view_from_state_payload,
};
use received_message_index::ReceivedMessageIndex;
use scope::{
    ClientRouteFeedScopeKey, ClientRoutePrincipalScopeKey, ContactOwnerScopeKey,
    projection_organization_id_for_event, scope_key, scope_key_for_event,
    scope_key_for_event_conversation, tracked_live_projection_lag_scope_id,
};

pub use access::{ClientRouteSyncStateSnapshot, ProjectionAccessError};
pub use bootstrap::{build_projection_runtime_from_env, ProjectionRuntime};
pub use embedded_bridge::try_apply_commit_envelope;
pub use journal_consumer::{
    spawn_projection_journal_consumer_from_env, ProjectionJournalConsumerHandle,
};
pub use http::{
    build_app, build_default_app, build_public_app, build_public_app_with_service,
    default_projection_runtime, default_projection_service,
};
pub use model::{
    ClientRouteSyncFeedWindowView, ContactView, ContactWindowView,
    ConversationMemberDirectoryEntry, ConversationPreferencesView, ConversationProfileView,
    ConversationSummaryView, DeleteMessageFavoriteResponse, FavoriteMessageRequest,
    FavoriteMessagesWindowView, InboxWindowView, InteractionActorView,
    MessageFavoriteView, MessageInteractionSummaryView, MessagePinView, MessageReactionCountView,
    NotificationRecipientView, RealtimeFanoutTarget, RegisteredClientRouteView, SummarySenderView,
    TimelineViewEntry, TimelineWindowView, UpdateConversationPreferencesRequest,
    UpdateConversationProfileRequest,
};
pub use observability::{
    ProjectionLagItemView, ProjectionLogView, ProjectionOperationMetricView,
    ProjectionPlaneMetricsView, ProjectionPlaneObservabilityView, ProjectionReplayMetricsView,
    ProjectionTraceView, ProjectionUpdateDelayView,
};
pub use projection::ProjectionError;

pub const PROJECTION_TIMELINE_DEFAULT_LIMIT: usize = 100;
pub const PROJECTION_TIMELINE_MAX_LIMIT: usize = 1000;
pub const PROJECTION_LIST_DEFAULT_LIMIT: usize = 100;
pub const PROJECTION_LIST_MAX_LIMIT: usize = 1000;
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT: usize = 100;
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT: usize = 1000;
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_RETAINED_EVENTS: usize =
    PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT;

#[derive(Default)]
pub struct TimelineProjectionService {
    entries: Mutex<HashMap<String, BTreeMap<u64, TimelineViewEntry>>>,
    summaries: Mutex<HashMap<String, ConversationSummaryView>>,
    members: Mutex<ProjectionMemberRuntimeStore>,
    read_cursors: Mutex<HashMap<String, HashMap<String, ConversationReadCursor>>>,
    received_messages: Mutex<ReceivedMessageIndex>,
    conversations: Mutex<HashMap<String, ConversationCatalogEntry>>,
    contacts: Mutex<HashMap<ContactOwnerScopeKey, HashMap<String, ContactView>>>,
    direct_chat_bindings: Mutex<contacts::ContactDirectChatBindingRuntimeStore>,
    message_interactions:
        Mutex<HashMap<String, HashMap<String, interactions::StoredMessageInteractionSummary>>>,
    registered_client_routes:
        Mutex<HashMap<ClientRoutePrincipalScopeKey, HashMap<String, RegisteredClientRouteView>>>,
    client_route_sync_feeds:
        Mutex<HashMap<ClientRouteFeedScopeKey, BTreeMap<u64, ClientRouteSyncFeedEntry>>>,
    client_route_sync_sequences: Mutex<HashMap<ClientRouteFeedScopeKey, u64>>,
    conversation_profiles: Mutex<HashMap<String, model::ConversationProfileView>>,
    conversation_preferences: Mutex<HashMap<String, model::ConversationPreferencesView>>,
    message_favorites:
        Mutex<HashMap<String, HashMap<String, model::MessageFavoriteView>>>,
    observability: Mutex<ProjectionObservabilityState>,
}

impl TimelineProjectionService {
    pub fn reset_for_recovery(&self) {
        lock_projection_mutex(&self.entries, "projection store").clear();
        lock_projection_mutex(&self.summaries, "summary store").clear();
        lock_projection_mutex(&self.members, "member store").clear();
        lock_projection_mutex(&self.read_cursors, "cursor store").clear();
        lock_projection_mutex(&self.received_messages, "received message index").clear();
        lock_projection_mutex(&self.conversations, "conversation store").clear();
        lock_projection_mutex(&self.contacts, "contact store").clear();
        lock_projection_mutex(
            &self.direct_chat_bindings,
            "contact direct chat binding store",
        )
        .clear();
        lock_projection_mutex(&self.message_interactions, "message interaction store").clear();
        lock_projection_mutex(
            &self.registered_client_routes,
            "registered client route store",
        )
        .clear();
        lock_projection_mutex(
            &self.client_route_sync_feeds,
            "client route sync feed store",
        )
        .clear();
        lock_projection_mutex(
            &self.client_route_sync_sequences,
            "client route sync sequence store",
        )
        .clear();
        lock_projection_mutex(&self.conversation_profiles, "conversation profile store").clear();
        lock_projection_mutex(
            &self.conversation_preferences,
            "conversation preferences store",
        )
        .clear();
        lock_projection_mutex(&self.message_favorites, "message favorites store").clear();
        *lock_projection_mutex(&self.observability, "projection observability store") =
            ProjectionObservabilityState::default();
    }

    pub fn is_active_member_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> bool {
        lock_projection_mutex(&self.members, "member store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .is_some_and(|scope_members| {
                scope_members.values().any(|member| {
                    member.principal_id == principal_id
                        && member.principal_kind == principal_kind
                        && member.is_active()
                })
            })
    }

    pub fn apply(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let live_lag_scope_id = tracked_live_projection_lag_scope_id(event);
        if let Some(scope_id) = live_lag_scope_id.as_deref() {
            self.record_projection_live_lag_observed(scope_id, event.ordering_seq);
        }
        let result = match event.event_type.as_str() {
            "conversation.created" => self.apply_conversation_created(event),
            "conversation.policy_applied" => self.apply_conversation_policy_applied(event),
            "conversation.agent_handoff_status_changed" => {
                self.apply_agent_handoff_status_changed(event)
            }
            "message.posted" => self.apply_message_posted(event),
            "message.edited" => self.apply_message_edited(event),
            "message.recalled" => self.apply_message_recalled(event),
            "message.reaction_added" => self.apply_message_reaction_added(event),
            "message.reaction_removed" => self.apply_message_reaction_removed(event),
            "message.pin_added" => self.apply_message_pinned(event),
            "message.pin_removed" => self.apply_message_unpinned(event),
            "conversation.member_joined" => self.apply_member_joined(event),
            "conversation.member_role_changed" => self.apply_member_role_changed(event),
            "conversation.member_removed" => self.apply_member_removed(event),
            "conversation.member_left" => self.apply_member_left(event),
            "conversation.read_cursor_updated" => self.apply_read_cursor_updated(event),
            "friendship.activated" => self.apply_friendship_activated(event),
            "friendship.removed" => self.apply_friendship_removed(event),
            "direct_chat.bound" => self.apply_direct_chat_bound(event),
            _ => Ok(()),
        };

        if result.is_ok()
            && let Some(scope_id) = live_lag_scope_id.as_deref()
        {
            self.record_projection_live_lag_committed(scope_id, event.ordering_seq);
        }

        result
    }

    pub fn client_route_sync_fanout_targets_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        fallback_recipients: Vec<NotificationRecipientView>,
    ) -> Vec<RealtimeFanoutTarget> {
        client_route_sync::client_route_sync_fanout_targets_for_conversation(
            self,
            tenant_id,
            organization_id,
            conversation_id,
            fallback_recipients,
        )
    }

    fn apply_conversation_created(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let payload: ConversationCreatedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let handoff_view = handoff_view_from_created_payload(&payload)?;
        let key = scope_key_for_event(event);
        lock_projection_mutex(&self.conversations, "conversation store").insert(
            key.clone(),
            ConversationCatalogEntry {
                conversation_type: payload.conversation_type,
                created_at: event.committed_at.clone(),
                history_visibility: "joined".into(),
            },
        );
        let conversation_id = event.aggregate_id.clone();
        let tenant_id = event.tenant_id.clone();
        let mut summaries = lock_projection_mutex(&self.summaries, "summary store");
        let summary = summaries
            .entry(key)
            .or_insert_with(|| ConversationSummaryView {
                tenant_id: tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                message_count: 0,
                last_message_id: None,
                last_message_seq: 0,
                last_sender_id: None,
                last_sender_kind: None,
                last_sender: None,
                last_summary: None,
                last_message_at: None,
                agent_handoff: None,
            });
        if handoff_view.is_some() {
            summary.agent_handoff = handoff_view;
        }
        Ok(())
    }

    fn apply_conversation_policy_applied(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: ConversationPolicyAppliedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key_for_event(event);
        let mut conversations = lock_projection_mutex(&self.conversations, "conversation store");
        let entry = conversations.entry(key.clone()).or_insert_with(|| ConversationCatalogEntry {
            conversation_type: "unknown".into(),
            created_at: event.committed_at.clone(),
            history_visibility: payload.history_visibility.clone(),
        });
        entry.history_visibility = payload.history_visibility;
        if im_domain_core::retention::retention_is_indefinite(
            im_domain_core::retention::retention_class_from_policy_ref(
                payload.retention_policy_ref.as_str(),
            )
            .as_str(),
        ) {
            let mut entries = lock_projection_mutex(&self.entries, "projection store");
            if let Some(timeline) = entries.get_mut(key.as_str()) {
                for entry in timeline.values_mut() {
                    entry.retention_until = None;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn history_visibility_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> String {
        lock_projection_mutex(&self.conversations, "conversation store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .map(|entry| entry.history_visibility.clone())
            .unwrap_or_else(|| "joined".into())
    }

    fn apply_agent_handoff_status_changed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: AgentHandoffStatusChangedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let handoff_view = handoff_view_from_state_payload(&payload.state);
        let key = scope_key_for_event_conversation(event, payload.state.conversation_id.as_str());
        let mut summaries = lock_projection_mutex(&self.summaries, "summary store");
        let summary = summaries
            .entry(key)
            .or_insert_with(|| ConversationSummaryView {
                tenant_id: event.tenant_id.clone(),
                conversation_id: payload.state.conversation_id.clone(),
                message_count: 0,
                last_message_id: None,
                last_message_seq: 0,
                last_sender_id: None,
                last_sender_kind: None,
                last_sender: None,
                last_summary: None,
                last_message_at: None,
                agent_handoff: None,
            });
        summary.agent_handoff = Some(handoff_view);
        drop(summaries);
        self.fan_out_agent_handoff_status_to_client_route_sync_feeds(event, &payload);
        Ok(())
    }

    fn apply_message_posted(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: Message =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let tenant_id = message.tenant_id.clone();
        let conversation_id = message.conversation_id.clone();
        let message_id = message.message_id.clone();
        let message_seq = message.message_seq;
        let summary = message.body.summary.clone();
        let sender_id = message.sender.id.clone();
        let sender_kind = message.sender.kind.clone();
        let last_message_at = message
            .committed_at
            .clone()
            .unwrap_or_else(|| message.occurred_at.clone());
        let organization_id = projection_organization_id_for_event(event);
        let key = scope_key(
            tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id.as_str(),
        );
        let entry = TimelineViewEntry {
            tenant_id: tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            message_id: message_id.clone(),
            message_seq,
            summary: summary.clone(),
            sender: message.sender.clone(),
            body: message.body.clone(),
            message_type: message.message_type.clone(),
            delivery_mode: message.delivery_mode.clone(),
            client_msg_id: message.client_msg_id.clone(),
            stream_session_id: message.stream_session_id.clone(),
            rtc_session_id: message.rtc_session_id.clone(),
            occurred_at: message.occurred_at.clone(),
            committed_at: message.committed_at.clone(),
            retention_until: retention_until_from_envelope(
                event.retention_class.as_str(),
                message.occurred_at.as_str(),
            ),
        };

        let mut entries = lock_projection_mutex(&self.entries, "projection store");
        entries.entry(key.clone()).or_default().insert(message_seq, entry);
        drop(entries);
        lock_projection_mutex(&self.received_messages, "received message index").append_message(
            key.as_str(),
            message_seq,
            sender_id.as_str(),
            sender_kind.as_str(),
        );

        let mut summaries = lock_projection_mutex(&self.summaries, "summary store");
        let existing_handoff = summaries
            .get(key.as_str())
            .and_then(|view| view.agent_handoff.clone());
        summaries.insert(
            key,
            ConversationSummaryView {
                tenant_id,
                conversation_id,
                message_count: message_seq,
                last_message_id: Some(message_id),
                last_message_seq: message_seq,
                last_sender_id: Some(sender_id.clone()),
                last_sender_kind: Some(sender_kind.clone()),
                last_sender: Some(SummarySenderView {
                    id: sender_id,
                    kind: sender_kind,
                }),
                last_summary: summary,
                last_message_at: Some(last_message_at),
                agent_handoff: existing_handoff,
            },
        );
        drop(summaries);

        self.fan_out_message_to_client_route_sync_feeds(event, &message);
        self.record_projection_update_delay_for_scope(
            "message.posted",
            scope_key(
                message.tenant_id.as_str(),
                organization_id.as_str(),
                message.conversation_id.as_str(),
            )
            .as_str(),
            message
                .committed_at
                .as_deref()
                .unwrap_or(message.occurred_at.as_str()),
        );
        Ok(())
    }

    fn apply_message_edited(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: MessageEdited =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        self.update_timeline_summary(
            message.tenant_id.as_str(),
            organization_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.body.summary.clone(),
        );
        self.update_conversation_summary_if_last(
            message.tenant_id.as_str(),
            organization_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.body.summary.clone(),
            message.edited_at.clone(),
        );
        self.fan_out_message_mutation_to_client_route_sync_feeds(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.message_seq,
            message.editor.id.as_str(),
            message.editor.kind.as_str(),
            message.editor.device_id.clone(),
            message.body.summary,
        );
        self.record_projection_update_delay_for_scope(
            "message.edited",
            scope_key_for_event_conversation(event, message.conversation_id.as_str()).as_str(),
            message.edited_at.as_str(),
        );
        Ok(())
    }

    fn apply_message_recalled(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let message: MessageRecalled =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let recalled_summary = Some("[recalled]".into());
        let organization_id = projection_organization_id_for_event(event);
        self.update_timeline_summary(
            message.tenant_id.as_str(),
            organization_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            recalled_summary.clone(),
        );
        self.update_conversation_summary_if_last(
            message.tenant_id.as_str(),
            organization_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            recalled_summary.clone(),
            message.recalled_at.clone(),
        );
        self.fan_out_message_mutation_to_client_route_sync_feeds(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
            message.message_id.as_str(),
            message.message_seq,
            message.recalled_by.id.as_str(),
            message.recalled_by.kind.as_str(),
            message.recalled_by.device_id.clone(),
            recalled_summary,
        );
        self.record_projection_update_delay_for_scope(
            "message.recalled",
            scope_key_for_event_conversation(event, message.conversation_id.as_str()).as_str(),
            message.recalled_at.as_str(),
        );
        Ok(())
    }

    fn apply_member_joined(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_projection_mutex(&self.members, "member store")
            .insert_member(key.clone(), member.clone());

        let mut cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        cursors
            .entry(key)
            .or_default()
            .entry(member.member_id.clone())
            .or_insert_with(|| ConversationReadCursor {
                tenant_id: member.tenant_id.clone(),
                conversation_id: member.conversation_id.clone(),
                member_id: member.member_id.clone(),
                principal_id: member.principal_id.clone(),
                principal_kind: member.principal_kind.clone(),
                read_seq: 0,
                last_read_message_id: None,
                updated_at: member.joined_at.clone(),
            });
        drop(cursors);

        self.fan_out_member_governance_to_client_route_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
            false,
            member.joined_at.as_str(),
        );
        Ok(())
    }

    fn apply_member_role_changed(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let payload: ConversationMemberRoleChangedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let member = payload.updated_member;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_projection_mutex(&self.members, "member store").insert_member(key, member.clone());

        self.fan_out_member_governance_to_client_route_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
            false,
            event.committed_at.as_str(),
        );
        Ok(())
    }

    fn apply_member_removed(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_projection_mutex(&self.members, "member store").remove_member(
            key.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
        );

        self.fan_out_member_governance_to_client_route_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
            true,
            member
                .removed_at
                .as_deref()
                .unwrap_or(event.committed_at.as_str()),
        );
        Ok(())
    }

    fn apply_member_left(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_projection_mutex(&self.members, "member store").remove_member(
            key.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
        );

        self.fan_out_member_governance_to_client_route_sync_feeds(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
            member.member_id.as_str(),
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
            true,
            member
                .removed_at
                .as_deref()
                .unwrap_or(event.committed_at.as_str()),
        );
        Ok(())
    }

    fn apply_read_cursor_updated(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let cursor: ConversationReadCursor =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let key = scope_key_for_event_conversation(event, cursor.conversation_id.as_str());
        let mut cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        cursors
            .entry(key)
            .or_default()
            .insert(cursor.member_id.clone(), cursor.clone());
        drop(cursors);

        self.fan_out_read_cursor_to_client_route_sync_feeds(event, &cursor);
        Ok(())
    }

    pub fn timeline(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<TimelineViewEntry> {
        lock_projection_mutex(&self.entries, "projection store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .map(|entries| entries.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn timeline_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> TimelineWindowView {
        let after_seq = after_seq.unwrap_or_default();
        let now = utc_now_rfc3339_millis();
        let entries = lock_projection_mutex(&self.entries, "projection store");
        let Some(timeline) = entries.get(scope_key(tenant_id, organization_id, conversation_id).as_str()) else {
            return TimelineWindowView {
                items: Vec::new(),
                next_after_seq: None,
                has_more: false,
            };
        };
        let mut window = timeline
            .range((Excluded(after_seq), Unbounded))
            .map(|(_, entry)| entry)
            .filter(|entry| !is_retention_expired(entry.retention_until.as_deref(), now.as_str()))
            .take(limit.saturating_add(1))
            .cloned()
            .collect::<Vec<_>>();
        let has_more = window.len() > limit;
        if has_more {
            window.truncate(limit);
        }
        let next_after_seq = window.last().map(|entry| entry.message_seq);

        TimelineWindowView {
            items: window,
            next_after_seq,
            has_more,
        }
    }

    pub fn conversation_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<ConversationSummaryView> {
        lock_projection_mutex(&self.summaries, "summary store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .cloned()
    }

    pub fn read_cursor_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationReadCursorView> {
        let member = self.member_snapshot_for_principal_kind(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind,
        )?;
        let key = scope_key(tenant_id, organization_id, conversation_id);
        let cursor = lock_projection_mutex(&self.read_cursors, "cursor store")
            .get(key.as_str())
            .and_then(|scope_cursors| scope_cursors.get(member.member_id.as_str()))
            .cloned()?;

        let unread_count = lock_projection_mutex(&self.received_messages, "received message index")
            .unread_count_after(
                key.as_str(),
                member.principal_id.as_str(),
                member.principal_kind.as_str(),
                cursor.read_seq,
            );

        Some(ConversationReadCursorView::from_cursor(
            &cursor,
            unread_count,
        ))
    }

    pub fn member_snapshot_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        lock_projection_mutex(&self.members, "member store")
            .member_for_principal_kind(
                scope_key(tenant_id, organization_id, conversation_id).as_str(),
                principal_id,
                principal_kind,
            )
            .cloned()
    }

    fn fan_out_message_to_client_route_sync_feeds(
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
    fn fan_out_message_mutation_to_client_route_sync_feeds(
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

    fn fan_out_read_cursor_to_client_route_sync_feeds(
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

    fn fan_out_agent_handoff_status_to_client_route_sync_feeds(
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
    fn fan_out_member_governance_to_client_route_sync_feeds(
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

fn lock_projection_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned projection mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests;
