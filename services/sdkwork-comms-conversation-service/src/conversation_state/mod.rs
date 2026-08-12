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
mod event_apply;
mod event_fanout;
mod group_metadata;
mod inbox;
mod interactions;
mod list_page;
mod member_directory;
mod member_store;
mod message_delivery_index;
mod message_favorites;
mod message_visibilities;
mod model;
mod profile_resolver;
mod read_receipts;
mod received_message_index;
mod scope;
mod summary_updates;
mod timeline_cache;

use event_apply::{
    AgentHandoffStatusChangedConversationStatePayload, ConversationMemberRoleChangedPayload,
    handoff_view_from_state_payload,
};
use member_store::ConversationStateMemberRuntimeStore;
use model::ConversationCatalogEntry;
use received_message_index::ReceivedMessageIndex;
use scope::{
    ClientRouteFeedScopeKey, ClientRoutePrincipalScopeKey, ContactOwnerScopeKey, GroupScopeKey,
    conversation_state_organization_id_for_event, is_conversation_conversation_state_event_type,
    scope_key, scope_key_for_event_conversation, validate_conversation_conversation_state_envelope,
    validate_conversation_conversation_state_payload_scope,
};

pub use access::{ClientRouteSyncStateView, ConversationStateAccessError};
pub use bootstrap::{
    ConversationStateRuntime, build_conversation_state_runtime_from_env,
    try_init_conversation_state_runtime,
};
pub use client_route_sync::{ClientRouteSyncAckStateView, ClientRouteSyncFeedWindowQuery};
pub use embedded_bridge::refresh_conversation_cache;
pub use event_apply::ConversationStateError;
pub use http::{
    build_app, build_conversation_query_api_router, build_default_app, build_public_app,
    build_public_app_with_service, default_conversation_state_runtime,
    default_conversation_state_service,
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
pub use profile_resolver::{PostgresUserProfileResolver, ResolvedUserDisplay, UserProfileResolver};

pub const CONVERSATION_STATE_TIMELINE_DEFAULT_LIMIT: usize = 100;
pub const CONVERSATION_STATE_TIMELINE_MAX_LIMIT: usize = 1000;
pub const CONVERSATION_STATE_LIST_DEFAULT_LIMIT: usize = 100;
pub const CONVERSATION_STATE_LIST_MAX_LIMIT: usize =
    sdkwork_utils_rust::http_api::MAX_LIST_PAGE_SIZE as usize;

#[cfg(test)]
static CONVERSATION_STATE_TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
pub(crate) fn lock_conversation_state_test_environment() -> MutexGuard<'static, ()> {
    CONVERSATION_STATE_TEST_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
pub const CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT: usize = 100;
pub const CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT: usize = 1000;
pub const CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_RETAINED_EVENTS: usize =
    CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT;

pub use timeline_cache::{
    CONVERSATION_TIMELINE_CACHE_CAP_DEFAULT, CONVERSATION_TIMELINE_CACHE_CAP_MAX,
    resolve_memory_timeline_cap_from_env,
};

#[derive(Default)]
pub struct ConversationStateService {
    entries: Mutex<HashMap<String, BTreeMap<u64, TimelineViewEntry>>>,
    /// O(1) lookup: `tenant:org:msg:message_id` → `conversation_id`.
    message_conversation_index: Mutex<HashMap<String, String>>,
    summaries: Mutex<HashMap<String, ConversationSummaryView>>,
    members: Mutex<ConversationStateMemberRuntimeStore>,
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
    /// LRU access tracking for bounded conversation-state eviction: every
    /// apply/read touches the conversation so idle conversations can be
    /// evicted together with all of their derived indexes.
    conversation_last_access: Mutex<HashMap<String, u64>>,
    /// FIFO order for the bounded message delivery offer index.
    delivery_offer_order: Mutex<std::collections::VecDeque<String>>,
    timeline_cache: timeline_cache::TimelineCacheConfig,
    conversation_event_outbox:
        std::sync::OnceLock<std::sync::Arc<dyn im_platform_contracts::OutboxStore>>,
    agent_integration_store:
        std::sync::OnceLock<std::sync::Arc<dyn im_platform_contracts::AgentIntegrationStore>>,
    user_profile_resolver: std::sync::OnceLock<std::sync::Arc<dyn UserProfileResolver>>,
}

impl ConversationStateService {
    pub fn set_memory_timeline_cap(&self, memory_cap: usize) {
        self.timeline_cache.set_memory_timeline_cap(memory_cap);
    }

    pub fn memory_timeline_cap(&self) -> usize {
        self.timeline_cache.memory_timeline_cap()
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

    pub fn configure_user_profile_resolver(
        &self,
        resolver: std::sync::Arc<dyn UserProfileResolver>,
    ) {
        let _ = self.user_profile_resolver.set(resolver);
    }

    /// Resolves display attributes for a user principal from the IM user
    /// profile table; returns `None` when no resolver is configured, the
    /// principal is not a user, or the profile carries no nickname.
    pub(crate) fn resolve_user_display(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ResolvedUserDisplay> {
        if principal_kind != "user" {
            return None;
        }
        let resolver = self.user_profile_resolver.get()?;
        resolver.resolve_display(tenant_id, organization_id, principal_id)
    }
}

impl ConversationStateService {
    pub fn is_active_member_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> bool {
        lock_conversation_state_mutex(&self.members, "member store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .is_some_and(|scope_members| {
                scope_members.values().any(|member| {
                    member.principal_id == principal_id
                        && member.principal_kind == principal_kind
                        && member.is_active()
                })
            })
    }

    pub fn apply(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        if is_conversation_conversation_state_event_type(event.event_type.as_str()) {
            validate_conversation_conversation_state_envelope(event)?;
        }
        // Track access so idle conversations (and their derived indexes) can
        // be evicted; this keeps long-running processes bounded.
        self.touch_conversation(scope_key_for_event_conversation(
            event,
            event.scope_id.as_str(),
        ).as_str());
        

        match event.event_type.as_str() {
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
        }
    }

    /// Records conversation access so idle eviction can reclaim its indexes.
    pub(crate) fn touch_conversation(&self, scope_key: &str) {
        let now = monotonic_millis();
        lock_conversation_state_mutex(
            &self.conversation_last_access,
            "conversation last access",
        )
        .insert(scope_key.to_owned(), now);
    }

    /// Evicts the least recently used conversations (and every derived index
    /// keyed by their scope) so long-running processes stay bounded. Returns
    /// the number of conversations evicted.
    pub(crate) fn evict_idle_conversations(&self, max_conversations: usize) -> usize {
        if max_conversations == 0 {
            return 0;
        }
        let mut evicted = 0usize;
        let mut evicted_scope_keys = Vec::new();
        {
            let mut last_access = lock_conversation_state_mutex(
                &self.conversation_last_access,
                "conversation last access",
            );
            if last_access.len() <= max_conversations {
                return 0;
            }
            let mut entries: Vec<(String, u64)> = last_access.drain().collect();
            entries.sort_unstable_by_key(|(_, accessed_at)| *accessed_at);
            let excess = entries.len().saturating_sub(max_conversations);
            for (scope_key_value, accessed_at) in entries {
                if evicted_scope_keys.len() < excess {
                    evicted_scope_keys.push(scope_key_value);
                } else {
                    last_access.insert(scope_key_value, accessed_at);
                }
            }
        }
        if evicted_scope_keys.is_empty() {
            return 0;
        }
        for scope_key_value in evicted_scope_keys {
            self.evict_conversation_scope(&scope_key_value);
            evicted += 1;
        }
        evicted
    }

    fn evict_conversation_scope(&self, scope_key_value: &str) {
        lock_conversation_state_mutex(&self.entries, "entries evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.summaries, "summaries evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.conversations, "conversations evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.conversation_profiles, "profiles evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.conversation_preferences, "preferences evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.read_cursors, "read cursors evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.message_interactions, "message interactions evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.pinned_messages_index, "pinned messages evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.message_favorites, "favorites evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.message_favorites_index, "favorites index evict")
            .remove(scope_key_value);
        lock_conversation_state_mutex(&self.message_visibilities, "visibilities evict")
            .remove(scope_key_value);
        // message_conversation_index maps message ids to conversations; drop
        // every entry whose conversation matches the evicted scope.
        lock_conversation_state_mutex(&self.message_conversation_index, "message conversation index evict")
            .retain(|_, conversation_id| conversation_id != scope_key_value);
        // Delivery offers are keyed by `{scope_key}:{message_id}`; drop the
        // whole prefix range.
        lock_conversation_state_mutex(&self.message_delivery_offers, "delivery offers evict")
            .retain(|key, _| !key.starts_with(scope_key_value));
        lock_conversation_state_mutex(&self.delivery_offer_order, "delivery offer order evict")
            .retain(|key| !key.starts_with(scope_key_value));
        lock_conversation_state_mutex(&self.conversation_last_access, "last access evict")
            .remove(scope_key_value);
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
    ) -> Result<(), ConversationStateError> {
        let payload: AgentHandoffStatusChangedConversationStatePayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            event.tenant_id.as_str(),
            payload.state.conversation_id.as_str(),
        )?;
        let handoff_view = handoff_view_from_state_payload(&payload.state);
        let key = scope_key_for_event_conversation(event, payload.state.conversation_id.as_str());
        let mut summaries = lock_conversation_state_mutex(&self.summaries, "summary store");
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

    fn apply_message_posted(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let message: Message =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
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
        let organization_id = conversation_state_organization_id_for_event(event);
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

        let mut entries = lock_conversation_state_mutex(&self.entries, "conversation_state store");
        let timeline = entries.entry(key.clone()).or_default();
        timeline.insert(message_seq, entry);
        timeline_cache::trim_timeline_to_cap(timeline, self.memory_timeline_cap());
        drop(entries);
        lock_conversation_state_mutex(
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
        lock_conversation_state_mutex(&self.received_messages, "received message index")
            .append_message(
                key.as_str(),
                message_seq,
                sender_id.as_str(),
                sender_kind.as_str(),
            );

        let mut summaries = lock_conversation_state_mutex(&self.summaries, "summary store");
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

        lock_conversation_state_mutex(&self.members, "member store")
            .refresh_inbox_activity_for_scope(
                scope_key(
                    message.tenant_id.as_str(),
                    organization_id.as_str(),
                    message.conversation_id.as_str(),
                )
                .as_str(),
                last_message_at.as_str(),
            );

        self.fan_out_message_to_client_route_sync_feeds(event, &message);
        Ok(())
    }

    fn apply_message_edited(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let message: MessageEdited =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        )?;
        let organization_id = conversation_state_organization_id_for_event(event);
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
        Ok(())
    }

    fn apply_message_recalled(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let message: MessageRecalled =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        )?;
        let recalled_summary = Some("[recalled]".into());
        let organization_id = conversation_state_organization_id_for_event(event);
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
        Ok(())
    }

    fn apply_member_joined(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_conversation_state_mutex(&self.members, "member store")
            .insert_member(key.clone(), member.clone());

        let mut cursors = lock_conversation_state_mutex(&self.read_cursors, "cursor store");
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
            lock_conversation_state_mutex(&self.members, "member store")
                .refresh_inbox_activity_for_scope(key.as_str(), member.joined_at.as_str());
        }
        Ok(())
    }

    fn apply_member_role_changed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: ConversationMemberRoleChangedPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let member = payload.updated_member;
        validate_conversation_conversation_state_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_conversation_state_mutex(&self.members, "member store")
            .insert_member(key, member.clone());

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

    fn apply_member_removed(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_conversation_state_mutex(&self.members, "member store").remove_member(
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

    fn apply_member_left(&self, event: &CommitEnvelope) -> Result<(), ConversationStateError> {
        let member: ConversationMember =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            member.tenant_id.as_str(),
            member.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, member.conversation_id.as_str());
        lock_conversation_state_mutex(&self.members, "member store").remove_member(
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

    fn apply_read_cursor_updated(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let cursor: ConversationReadCursor =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_conversation_conversation_state_payload_scope(
            event,
            cursor.tenant_id.as_str(),
            cursor.conversation_id.as_str(),
        )?;
        let key = scope_key_for_event_conversation(event, cursor.conversation_id.as_str());
        let storage_key =
            read_cursor_storage_key(cursor.member_id.as_str(), cursor.device_id.as_deref());
        let mut cursors = lock_conversation_state_mutex(&self.read_cursors, "cursor store");
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
        lock_conversation_state_mutex(&self.entries, "conversation_state store")
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
    ) -> Result<TimelineWindowView, crate::conversation_state::event_apply::ConversationStateError>
    {
        let after_seq = after_seq.unwrap_or_default();
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let memory_timeline =
            lock_conversation_state_mutex(&self.entries, "conversation_state store")
                .get(scope.as_str())
                .cloned();
        Ok(memory_timeline
            .as_ref()
            .map(|timeline| timeline_cache::timeline_window_from_memory(timeline, after_seq, limit))
            .unwrap_or_else(|| {
                crate::conversation_state::list_page::seq_cursor_page(
                    Vec::new(),
                    limit,
                    None,
                    false,
                )
            }))
    }

    pub fn conversation_summary(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<ConversationSummaryView> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(summary) = lock_conversation_state_mutex(&self.summaries, "summary store")
            .get(scope.as_str())
            .cloned()
        {
            return Some(summary);
        }
        None
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
        let member = self.member_view_for_principal_kind(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            principal_kind,
        )?;
        let key = scope_key(tenant_id, organization_id, conversation_id);
        let scope_cursors = lock_conversation_state_mutex(&self.read_cursors, "cursor store")
            .get(key.as_str())
            .cloned();
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

        let unread_count =
            lock_conversation_state_mutex(&self.received_messages, "received message index")
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

    pub fn member_view_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        if let Some(member) = lock_conversation_state_mutex(&self.members, "member store")
            .member_for_principal_kind(scope.as_str(), principal_id, principal_kind)
            .cloned()
        {
            return Some(member);
        }
        None
    }
}

fn lock_conversation_state_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned conversation_state mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn monotonic_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
