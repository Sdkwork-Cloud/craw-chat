use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Mutex, MutexGuard};

use im_domain_core::conversation::{
    ClientRouteSyncFeedEntry, ConversationMember, ConversationReadCursor,
    ConversationReadCursorView, read_cursor_storage_key,
};
use im_domain_core::message::{Message, MessageEdited, MessageRecalled};
use im_domain_core::retention::retention_until_from_envelope;
use im_domain_events::CommitEnvelope;

mod access;
mod bootstrap;
mod client_route_sync;
mod contacts;
mod conversation_catalog;
mod conversation_personalization;
mod conversation_profile_events;
mod cursor_auth;
pub mod embedded_bridge;
pub mod http;
pub use http::build_integration_test_app;
mod delivery_receipts;
mod event_fanout;
mod group_metadata;
mod inbox;
mod interactions;
mod journal_consumer;
mod list_page;
mod member_directory;
mod member_store;
mod message_delivery_index;
mod message_favorites;
mod message_visibilities;
mod metadata_read_through;
mod metadata_tier;
mod model;
mod observability;
mod personalization_snapshot;
mod projection;
mod read_receipts;
mod received_message_index;
mod scope;
mod snapshot;
mod summary_updates;
mod timeline_tier;
mod update_delay;

use member_store::ProjectionMemberRuntimeStore;
use model::ConversationCatalogEntry;
use observability::ProjectionObservabilityState;
use projection::{
    AgentHandoffStatusChangedProjectionPayload, ConversationMemberRoleChangedPayload,
    handoff_view_from_state_payload,
};
use received_message_index::ReceivedMessageIndex;
use scope::{
    ClientRouteFeedScopeKey, ClientRoutePrincipalScopeKey, ContactOwnerScopeKey, GroupScopeKey,
    is_conversation_projection_event_type, projection_organization_id_for_event, scope_key,
    scope_key_for_event_conversation, tracked_live_projection_lag_scope_id,
    validate_conversation_projection_envelope, validate_conversation_projection_payload_scope,
};

pub use access::{ClientRouteSyncStateSnapshot, ProjectionAccessError};
pub use bootstrap::{
    ProjectionRuntime, build_projection_runtime_from_env, try_init_embedded_projection_runtime,
};
pub use client_route_sync::{ClientRouteSyncAckStateView, ClientRouteSyncFeedWindowQuery};
pub use embedded_bridge::try_apply_commit_envelope;
pub use http::{
    build_app, build_default_app, build_public_app, build_public_app_with_service,
    default_projection_runtime, default_projection_service,
};
pub use journal_consumer::{
    ProjectionJournalConsumerHandle, spawn_projection_journal_consumer_from_env,
};
pub use message_visibilities::TimelineWindowForPrincipalQuery;
pub use model::{
    ClientRouteSyncFeedWindowView, ContactView, ContactWindowView,
    ConversationMemberDirectoryEntry, ConversationPreferencesView, ConversationProfileView,
    ConversationSummaryView, DeleteMessageFavoriteResponse, FavoriteMessageRequest,
    FavoriteMessagesWindowView, InboxWindowView, InteractionActorView,
    MessageDeliveryReceiptDeviceView, MessageDeliveryReceiptSummaryView, MessageFavoriteView,
    MessageInteractionSummaryView, MessagePinView, MessageReactionCountView,
    MessageReadReceiptReaderView, MessageReadReceiptSummaryView, MessageSearchHitView,
    MessageSearchWindowView, MessageVisibilityMutationResult, NotificationRecipientView,
    RealtimeFanoutTarget, RegisteredClientRouteView, SummarySenderView, TimelineViewEntry,
    TimelineWindowView, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
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
pub const PROJECTION_LIST_MAX_LIMIT: usize =
    sdkwork_utils_rust::http_api::MAX_LIST_PAGE_SIZE as usize;

#[cfg(test)]
static PROJECTION_TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
pub(crate) fn lock_projection_test_environment() -> MutexGuard<'static, ()> {
    PROJECTION_TEST_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT: usize = 100;
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT: usize = 1000;
pub const PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_RETAINED_EVENTS: usize =
    PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT;

pub use timeline_tier::{
    PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT, PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED,
    resolve_memory_timeline_cap_from_env,
};

#[derive(Default)]
pub struct TimelineProjectionService {
    entries: Mutex<HashMap<String, BTreeMap<u64, TimelineViewEntry>>>,
    /// O(1) lookup: `tenant:org:msg:message_id` → `conversation_id`.
    message_conversation_index: Mutex<HashMap<String, String>>,
    summaries: Mutex<HashMap<String, ConversationSummaryView>>,
    members: Mutex<ProjectionMemberRuntimeStore>,
    read_cursors: Mutex<HashMap<String, HashMap<String, ConversationReadCursor>>>,
    received_messages: Mutex<ReceivedMessageIndex>,
    conversations: Mutex<HashMap<String, ConversationCatalogEntry>>,
    group_conversation_bindings:
        Mutex<HashMap<GroupScopeKey, group_metadata::GroupConversationBinding>>,
    contacts: Mutex<HashMap<ContactOwnerScopeKey, contacts::ContactScopeStore>>,
    direct_chat_bindings: Mutex<contacts::ContactDirectChatBindingRuntimeStore>,
    message_interactions:
        Mutex<HashMap<String, HashMap<String, interactions::StoredMessageInteractionSummary>>>,
    pinned_messages_index: Mutex<HashMap<String, BTreeSet<interactions::PinnedMessageIndexKey>>>,
    registered_client_routes:
        Mutex<HashMap<ClientRoutePrincipalScopeKey, HashMap<String, RegisteredClientRouteView>>>,
    client_route_sync_feeds:
        Mutex<HashMap<ClientRouteFeedScopeKey, BTreeMap<u64, ClientRouteSyncFeedEntry>>>,
    client_route_sync_sequences: Mutex<HashMap<ClientRouteFeedScopeKey, u64>>,
    client_route_sync_checkpoints:
        Mutex<HashMap<ClientRouteFeedScopeKey, client_route_sync::ClientRouteSyncCheckpoint>>,
    message_delivery_offers:
        Mutex<HashMap<String, Vec<message_delivery_index::MessageDeliveryDeviceOffer>>>,
    conversation_profiles: Mutex<HashMap<String, model::ConversationProfileView>>,
    conversation_preferences: Mutex<HashMap<String, model::ConversationPreferencesView>>,
    message_favorites: Mutex<HashMap<String, HashMap<String, model::MessageFavoriteView>>>,
    message_favorites_index:
        Mutex<HashMap<String, BTreeSet<message_favorites::MessageFavoriteIndexEntry>>>,
    message_visibilities:
        Mutex<HashMap<String, HashMap<String, model::MessageVisibilityMutationResult>>>,
    observability: Mutex<ProjectionObservabilityState>,
    timeline_tier: timeline_tier::TimelineTierConfig,
    metadata_tier: metadata_tier::MetadataTierConfig,
    conversation_event_outbox:
        std::sync::OnceLock<std::sync::Arc<dyn im_platform_contracts::OutboxStore>>,
    agent_integration_store:
        std::sync::OnceLock<std::sync::Arc<dyn im_platform_contracts::AgentIntegrationStore>>,
}

impl TimelineProjectionService {
    pub fn configure_durable_timeline(
        &self,
        store: std::sync::Arc<
            dyn sdkwork_im_contract_message::TimelineProjectionStore + Send + Sync,
        >,
        memory_cap: usize,
    ) {
        self.timeline_tier
            .configure_durable_timeline(store, memory_cap);
    }

    pub fn set_memory_timeline_cap(&self, memory_cap: usize) {
        self.timeline_tier.set_memory_timeline_cap(memory_cap);
    }

    pub fn memory_timeline_cap(&self) -> usize {
        self.timeline_tier.memory_timeline_cap()
    }

    pub fn configure_durable_metadata(
        &self,
        store: std::sync::Arc<dyn im_platform_contracts::MetadataStore + Send + Sync>,
    ) {
        self.metadata_tier.configure_durable_metadata(store);
    }

    pub fn durable_metadata_store(
        &self,
    ) -> Option<std::sync::Arc<dyn im_platform_contracts::MetadataStore + Send + Sync>> {
        self.metadata_tier.durable_metadata_store()
    }

    pub fn configure_conversation_event_outbox(
        &self,
        store: std::sync::Arc<dyn im_platform_contracts::OutboxStore>,
    ) {
        let _ = self.conversation_event_outbox.set(store);
    }

    pub fn configure_agent_integration_store(
        &self,
        store: std::sync::Arc<dyn im_platform_contracts::AgentIntegrationStore>,
    ) {
        let _ = self.agent_integration_store.set(store);
    }
}

impl TimelineProjectionService {
    pub fn reset_for_recovery(&self) {
        lock_projection_mutex(&self.entries, "projection store").clear();
        lock_projection_mutex(
            &self.message_conversation_index,
            "message conversation index",
        )
        .clear();
        lock_projection_mutex(&self.summaries, "summary store").clear();
        lock_projection_mutex(&self.members, "member store").clear();
        lock_projection_mutex(&self.read_cursors, "cursor store").clear();
        lock_projection_mutex(&self.received_messages, "received message index").clear();
        lock_projection_mutex(&self.conversations, "conversation store").clear();
        lock_projection_mutex(
            &self.group_conversation_bindings,
            "group conversation binding store",
        )
        .clear();
        lock_projection_mutex(&self.contacts, "contact store").clear();
        lock_projection_mutex(
            &self.direct_chat_bindings,
            "contact direct chat binding store",
        )
        .clear();
        lock_projection_mutex(&self.message_interactions, "message interaction store").clear();
        lock_projection_mutex(&self.pinned_messages_index, "pinned message index").clear();
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
        lock_projection_mutex(
            &self.client_route_sync_checkpoints,
            "client route sync checkpoint store",
        )
        .clear();
        lock_projection_mutex(
            &self.message_delivery_offers,
            "message delivery offer store",
        )
        .clear();
        lock_projection_mutex(&self.conversation_profiles, "conversation profile store").clear();
        lock_projection_mutex(
            &self.conversation_preferences,
            "conversation preferences store",
        )
        .clear();
        lock_projection_mutex(&self.message_favorites, "message favorites store").clear();
        lock_projection_mutex(&self.message_favorites_index, "message favorites index").clear();
        lock_projection_mutex(&self.message_visibilities, "message visibility store").clear();
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
        if is_conversation_projection_event_type(event.event_type.as_str()) {
            validate_conversation_projection_envelope(event)?;
        }
        let live_lag_scope_id = tracked_live_projection_lag_scope_id(event);
        if let Some(scope_id) = live_lag_scope_id.as_deref() {
            self.record_projection_live_lag_observed(scope_id, event.ordering_seq);
        }
        let result = match event.event_type.as_str() {
            "conversation.created" => self.apply_conversation_created(event),
            "conversation.agents_replaced" => self.apply_conversation_agents_replaced(event),
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
            "conversation.member_invitation_accepted" => self.apply_member_joined(event),
            "conversation.member_role_changed" => self.apply_member_role_changed(event),
            "conversation.member_removed" => self.apply_member_removed(event),
            "conversation.member_left" => self.apply_member_left(event),
            "conversation.read_cursor_updated" => self.apply_read_cursor_updated(event),
            "friendship.activated" => self.apply_friendship_activated(event),
            "friendship.removed" => self.apply_friendship_removed(event),
            "user_block.blocked" => self.apply_user_blocked(event),
            "user_block.released" => self.apply_user_block_released(event),
            "direct_chat.bound" => self.apply_direct_chat_bound(event),
            "group.created" => self.apply_group_created(event),
            "group.updated" => self.apply_group_updated(event),
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

    fn apply_agent_handoff_status_changed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: AgentHandoffStatusChangedProjectionPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        validate_conversation_projection_payload_scope(
            event,
            event.tenant_id.as_str(),
            payload.state.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        )?;
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
            reaction_counts: Vec::new(),
            pin: None,
        };

        let mut entries = lock_projection_mutex(&self.entries, "projection store");
        let timeline = entries.entry(key.clone()).or_default();
        timeline.insert(message_seq, entry);
        timeline_tier::trim_timeline_to_cap(timeline, self.memory_timeline_cap());
        drop(entries);
        lock_projection_mutex(
            &self.message_conversation_index,
            "message conversation index",
        )
        .insert(
            scope::message_lookup_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                message_id.as_str(),
            ),
            conversation_id.clone(),
        );
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
                last_message_at: Some(last_message_at.clone()),
                agent_handoff: existing_handoff,
            },
        );
        drop(summaries);

        lock_projection_mutex(&self.members, "member store").refresh_inbox_activity_for_scope(
            scope_key(
                message.tenant_id.as_str(),
                organization_id.as_str(),
                message.conversation_id.as_str(),
            )
            .as_str(),
            last_message_at.as_str(),
        );

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
        validate_conversation_projection_payload_scope(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_projection_mutex(&self.members, "member store")
            .insert_member(key.clone(), member.clone());

        let mut cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        cursors
            .entry(key.clone())
            .or_default()
            .entry(read_cursor_storage_key(member.member_id.as_str(), None))
            .or_insert_with(|| ConversationReadCursor {
                tenant_id: member.tenant_id.clone(),
                conversation_id: member.conversation_id.clone(),
                member_id: member.member_id.clone(),
                principal_id: member.principal_id.clone(),
                principal_kind: member.principal_kind.clone(),
                device_id: None,
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
        if member.is_active() {
            lock_projection_mutex(&self.members, "member store")
                .refresh_inbox_activity_for_scope(key.as_str(), member.joined_at.as_str());
        }
        Ok(())
    }

    fn apply_member_role_changed(&self, event: &CommitEnvelope) -> Result<(), ProjectionError> {
        let payload: ConversationMemberRoleChangedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let member = payload.updated_member;
        validate_conversation_projection_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
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
        validate_conversation_projection_payload_scope(
            event,
            cursor.tenant_id.as_str(),
            cursor.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, cursor.conversation_id.as_str());
        let storage_key =
            read_cursor_storage_key(cursor.member_id.as_str(), cursor.device_id.as_deref());
        let mut cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        cursors
            .entry(key)
            .or_default()
            .insert(storage_key, cursor.clone());
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
    ) -> Result<TimelineWindowView, crate::projection::ProjectionError> {
        let after_seq = after_seq.unwrap_or_default();
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let memory_timeline = lock_projection_mutex(&self.entries, "projection store")
            .get(scope.as_str())
            .cloned();
        match timeline_tier::resolve_timeline_window(
            &self.timeline_tier,
            memory_timeline.as_ref(),
            tenant_id,
            organization_id,
            conversation_id,
            after_seq,
            limit,
        ) {
            Ok(window) => Ok(window),
            Err(error) if crate::bootstrap::allows_in_memory_projection_fallback() => {
                tracing::warn!(
                    target: "sdkwork.im.projection.timeline",
                    event = "im.projection.timeline_durable_read_failed",
                    tenant_id = %tenant_id,
                    conversation_id = %conversation_id,
                    after_seq,
                    ?error,
                    "durable timeline read failed; falling back to in-memory window (development/test only)"
                );
                Ok(memory_timeline
                    .as_ref()
                    .map(|timeline| {
                        timeline_tier::timeline_window_from_memory(timeline, after_seq, limit)
                    })
                    .unwrap_or_else(|| {
                        crate::list_page::seq_cursor_page(Vec::new(), limit, None, false)
                    }))
            }
            Err(error) => Err(error),
        }
    }

    pub fn conversation_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<ConversationSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(summary) = lock_projection_mutex(&self.summaries, "summary store")
            .get(scope.as_str())
            .cloned()
        {
            return Some(summary);
        }
        self.load_summary_from_durable_store(scope.as_str())
    }

    pub fn read_cursor_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationReadCursorView> {
        self.read_cursor_for_principal_kind_and_device(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind,
            None,
        )
    }

    pub fn read_cursor_for_principal_kind_and_device(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: Option<&str>,
    ) -> Option<ConversationReadCursorView> {
        let member = self.member_snapshot_for_principal_kind(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind,
        )?;
        let key = scope_key(tenant_id, organization_id, conversation_id);
        let scope_cursors = lock_projection_mutex(&self.read_cursors, "cursor store")
            .get(key.as_str())
            .cloned()
            .or_else(|| self.load_read_cursors_from_durable_store(key.as_str()));
        let scope_cursors = scope_cursors?;
        let storage_key = read_cursor_storage_key(member.member_id.as_str(), device_id);
        let cursor = scope_cursors
            .get(storage_key.as_str())
            .or_else(|| {
                if device_id.is_some() {
                    scope_cursors.get(member.member_id.as_str())
                } else {
                    None
                }
            })
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
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(member) = lock_projection_mutex(&self.members, "member store")
            .member_for_principal_kind(scope.as_str(), principal_id, principal_kind)
            .cloned()
        {
            return Some(member);
        }
        self.load_member_from_durable_store(scope.as_str(), principal_id, principal_kind)
    }
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
