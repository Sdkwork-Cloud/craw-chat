use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use im_domain_core::social::DirectChatStatus;
use im_domain_events::CommitEnvelope;
use im_domain_events::social::{
    DirectChatBoundPayload, FriendshipActivatedPayload, FriendshipRemovedPayload,
    UserBlockReleasedPayload, UserBlockedPayload,
};
use im_time::{max_rfc3339_string, rfc3339_cmp};
use sdkwork_utils_rust::SdkWorkPageData;

use im_platform_contracts::normalize_realtime_organization_id;

use crate::conversation_state::client_route_sync::registered_client_routes_for_principal_kind;
use crate::conversation_state::model::ContactDirectChatBindingView;
use crate::conversation_state::model::ContactListCursor;
use crate::conversation_state::{ContactView, ConversationStateService};

use super::event_apply::ConversationStateError;
use super::lock_conversation_state_mutex;
use super::scope::{
    ContactOwnerScopeKey, contact_owner_scope_key, conversation_state_organization_id_for_event,
    encode_conversation_state_key_segments, scope_key,
};

#[derive(Default)]
pub(crate) struct ContactDirectChatBindingRuntimeStore {
    by_direct_chat_id: HashMap<ContactDirectChatBindingKey, ContactDirectChatBindingView>,
    direct_chat_id_by_conversation:
        HashMap<ContactConversationIndexKey, ContactDirectChatBindingKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ContactDirectChatBindingKey {
    tenant_id: String,
    organization_id: String,
    direct_chat_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ContactConversationIndexKey {
    tenant_id: String,
    organization_id: String,
    conversation_id: String,
}

impl ContactDirectChatBindingRuntimeStore {
    pub(crate) fn insert(&mut self, binding: ContactDirectChatBindingView) {
        let binding_key = direct_chat_binding_key_for_view(&binding);
        let conversation_key = direct_chat_conversation_index_key_for_view(&binding);
        if let Some(previous) = self
            .by_direct_chat_id
            .insert(binding_key.clone(), binding.clone())
        {
            self.direct_chat_id_by_conversation
                .remove(&direct_chat_conversation_index_key_for_view(&previous));
        }
        self.direct_chat_id_by_conversation
            .insert(conversation_key, binding_key);
    }

    pub(crate) fn get_by_direct_chat_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        direct_chat_id: &str,
    ) -> Option<&ContactDirectChatBindingView> {
        self.by_direct_chat_id.get(&direct_chat_binding_key(
            tenant_id,
            organization_id,
            direct_chat_id,
        ))
    }

    pub(crate) fn get_by_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<&ContactDirectChatBindingView> {
        let binding_key =
            self.direct_chat_id_by_conversation
                .get(&direct_chat_conversation_index_key(
                    tenant_id,
                    organization_id,
                    conversation_id,
                ))?;
        self.by_direct_chat_id.get(binding_key)
    }

    pub(crate) fn archive_by_direct_chat_id(
        &mut self,
        tenant_id: &str,
        organization_id: &str,
        direct_chat_id: &str,
        archived_at: &str,
    ) {
        if let Some(binding) = self.by_direct_chat_id.get_mut(&direct_chat_binding_key(
            tenant_id,
            organization_id,
            direct_chat_id,
        )) {
            binding.status = DirectChatStatus::Archived;
            binding.updated_at = Some(archived_at.to_owned());
        }
    }
}

/// Per-owner contact index with incrementally maintained sort order for paginated reads.
#[derive(Default, Clone)]
pub(super) struct ContactScopeStore {
    by_key: HashMap<String, ContactView>,
    ordered_keys: Vec<String>,
}

impl ContactScopeStore {
    pub fn is_empty(&self) -> bool {
        self.by_key.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&ContactView> {
        self.by_key.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut ContactView> {
        self.by_key.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<ContactView> {
        let removed = self.by_key.remove(key)?;
        self.ordered_keys.retain(|entry| entry != key);
        Some(removed)
    }

    pub fn values(&self) -> impl Iterator<Item = &ContactView> {
        self.by_key.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut ContactView> {
        self.by_key.values_mut()
    }

    pub fn ensure_contact<F>(&mut self, key: String, init: F) -> &mut ContactView
    where
        F: FnOnce() -> ContactView,
    {
        if !self.by_key.contains_key(key.as_str()) {
            self.by_key.insert(key.clone(), init());
            self.rebuild_order();
        }
        self.by_key
            .get_mut(key.as_str())
            .expect("contact entry must exist after ensure")
    }

    pub fn rebuild_order(&mut self) {
        self.ordered_keys = self.by_key.keys().cloned().collect();
        self.ordered_keys.sort_by(|left_key, right_key| {
            let left = &self.by_key[left_key];
            let right = &self.by_key[right_key];
            rfc3339_cmp(
                right.last_interaction_at.as_str(),
                left.last_interaction_at.as_str(),
            )
            .then_with(|| left.target_user_id.cmp(&right.target_user_id))
        });
    }

    pub fn ordered_window_with_cursor(
        &self,
        cursor: Option<ContactListCursor>,
        limit: usize,
    ) -> (Vec<ContactView>, bool) {
        let mut window = Vec::with_capacity(limit.saturating_add(1));
        let keyset_cursor = match cursor {
            Some(ContactListCursor::Keyset {
                last_interaction_at,
                target_user_id,
            }) => Some((last_interaction_at, target_user_id)),
            _ => None,
        };
        for key in self.ordered_keys.iter() {
            let Some(view) = self.by_key.get(key) else {
                continue;
            };
            if let Some(cursor_pair) = keyset_cursor.as_ref()
                && !contact_entry_after_keyset_cursor(view, cursor_pair)
            {
                continue;
            }
            window.push(view.clone());
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

    pub fn ordered_items(&self) -> Vec<ContactView> {
        self.ordered_keys
            .iter()
            .filter_map(|key| self.by_key.get(key).cloned())
            .collect()
    }

    #[cfg(test)]
    pub fn from_items(items: Vec<ContactView>) -> Self {
        let mut store = Self::default();
        for item in items {
            let key = contact_entry_key(item.contact_type.as_str(), item.target_user_id.as_str());
            store.by_key.insert(key, item);
        }
        store.rebuild_order();
        store
    }
}

impl ConversationStateService {
    pub fn contacts(
        &self,
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
    ) -> Vec<ContactView> {
        let scope = contact_runtime_scope(tenant_id, organization_id, owner_user_id);
        let mut items = self
            .lock_contact_store("contacts")
            .get(&scope)
            .map(ContactScopeStore::ordered_items)
            .unwrap_or_default();
        self.enrich_contact_profiles(tenant_id, organization_id, &mut items);
        items
    }

    pub(crate) fn contact_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
        limit: usize,
        cursor: crate::conversation_state::model::ContactListCursor,
        search_query: Option<&str>,
    ) -> SdkWorkPageData<crate::conversation_state::ContactView> {
        let scope = contact_runtime_scope(tenant_id, organization_id, owner_user_id);
        let requested_query = search_query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_lowercase);
        let scan_batch_size = if requested_query.is_some() {
            limit
                .saturating_mul(8)
                .max(limit.saturating_add(1))
                .min(512)
        } else {
            limit.saturating_add(1)
        };
        let mut scan_cursor = match cursor {
            crate::conversation_state::model::ContactListCursor::Start => None,
            other => Some(other),
        };
        let mut items = Vec::with_capacity(limit.saturating_add(1));
        let mut last_returned_cursor: Option<(String, String)> = None;
        let mut exhausted = false;

        while items.len() <= limit && !exhausted {
            let (mut batch, batch_has_more) = self
                .lock_contact_store("contact_window")
                .get(&scope)
                .map(|scope_store| {
                    scope_store.ordered_window_with_cursor(scan_cursor.clone(), scan_batch_size)
                })
                .unwrap_or_default();
            exhausted = !batch_has_more;
            let Some(last_scanned) = batch.last() else {
                exhausted = true;
                break;
            };
            scan_cursor = Some(
                crate::conversation_state::model::ContactListCursor::Keyset {
                    last_interaction_at: last_scanned.last_interaction_at.clone(),
                    target_user_id: last_scanned.target_user_id.clone(),
                },
            );
            self.enrich_contact_profiles(tenant_id, organization_id, &mut batch);
            for contact in batch {
                if requested_query
                    .as_ref()
                    .is_some_and(|query| !contact_matches_query(&contact, query))
                {
                    continue;
                }
                items.push(contact);
                if items.len() <= limit {
                    let contact = items.last().expect("contact was just appended");
                    last_returned_cursor = Some((
                        contact.last_interaction_at.clone(),
                        contact.target_user_id.clone(),
                    ));
                }
                if items.len() > limit {
                    break;
                }
            }
        }

        let has_more = items.len() > limit || !exhausted;
        if items.len() > limit {
            items.truncate(limit);
        }
        let next_cursor = if has_more {
            last_returned_cursor
                .as_ref()
                .and_then(|(last_interaction_at, target_user_id)| {
                    let payload = serde_json::json!({
                        "lastInteractionAt": last_interaction_at,
                        "targetUserId": target_user_id,
                    });
                    crate::conversation_state::cursor_auth::encode_signed_conversation_state_cursor(
                        &payload,
                    )
                    .ok()
                })
        } else {
            None
        };
        super::list_page::cursor_page(items, limit, next_cursor, has_more)
    }

    fn enrich_contact_profiles(
        &self,
        tenant_id: &str,
        organization_id: &str,
        contacts: &mut [ContactView],
    ) {
        let members = lock_conversation_state_mutex(&self.members, "member store");
        for contact in contacts {
            let Some(conversation_id) = contact.conversation_id.as_deref() else {
                continue;
            };
            let scope = scope_key(tenant_id, organization_id, conversation_id);
            let Some(member) = members.member_for_principal_kind(
                scope.as_str(),
                contact.target_user_id.as_str(),
                "user",
            ) else {
                continue;
            };
            contact.display_name =
                contact_member_attribute(&member.attributes, &["displayName", "display_name"]);
            contact.avatar_url = contact_member_attribute(
                &member.attributes,
                &["avatarUrl", "avatar_url", "avatar"],
            );
            contact.chat_id = contact_member_attribute(&member.attributes, &["chatId", "chat_id"]);
        }
    }

    pub(super) fn apply_friendship_activated(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: FriendshipActivatedPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let organization_id = conversation_state_organization_id_for_event(event);
        let binding = payload.direct_chat_id.as_ref().and_then(|direct_chat_id| {
            self.direct_chat_binding(
                event.tenant_id.as_str(),
                organization_id.as_str(),
                direct_chat_id.as_str(),
            )
        });

        self.upsert_friendship_contact(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.user_low_id.as_str(),
            payload.user_high_id.as_str(),
            &payload,
            binding.as_ref(),
        );
        self.upsert_friendship_contact(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.user_high_id.as_str(),
            payload.user_low_id.as_str(),
            &payload,
            binding.as_ref(),
        );
        self.fan_out_friendship_activated_to_client_route_sync_feeds(event, &payload);

        Ok(())
    }

    pub(super) fn apply_friendship_removed(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: FriendshipRemovedPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let organization_id = conversation_state_organization_id_for_event(event);
        self.archive_friendship_direct_chat(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.friendship_id.as_str(),
            payload.removed_at.as_str(),
        );

        self.remove_friendship_contact(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.user_low_id.as_str(),
            payload.user_high_id.as_str(),
            payload.friendship_id.as_str(),
        );
        self.remove_friendship_contact(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.user_high_id.as_str(),
            payload.user_low_id.as_str(),
            payload.friendship_id.as_str(),
        );
        self.fan_out_friendship_removed_to_client_route_sync_feeds(event, &payload);

        Ok(())
    }

    pub(super) fn apply_direct_chat_bound(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: DirectChatBoundPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let organization_id = conversation_state_organization_id_for_event(event);
        let binding = ContactDirectChatBindingView {
            tenant_id: Some(event.tenant_id.clone()),
            organization_id: Some(organization_id.clone()),
            direct_chat_id: payload.direct_chat_id.clone(),
            conversation_id: payload.conversation_id.clone(),
            bound_at: payload.bound_at.clone(),
            status: DirectChatStatus::Active,
            updated_at: Some(payload.bound_at.clone()),
        };
        self.lock_direct_chat_bindings("apply_direct_chat_bound")
            .insert(binding.clone());

        let mut contacts = self.lock_contact_store("apply_direct_chat_bound");
        let mut touched_scopes = Vec::new();
        for (scope, scope_contacts) in contacts.iter_mut() {
            if scope.tenant_id != event.tenant_id || scope.organization_id != organization_id {
                continue;
            }
            let mut touched = false;
            for contact in scope_contacts.values_mut() {
                if contact.direct_chat_id.as_deref() == Some(payload.direct_chat_id.as_str()) {
                    contact.conversation_id = Some(payload.conversation_id.clone());
                    contact.last_interaction_at = max_rfc3339(
                        contact.last_interaction_at.as_str(),
                        payload.bound_at.as_str(),
                    )
                    .to_owned();
                    touched = true;
                }
            }
            if touched {
                touched_scopes.push(scope.clone());
            }
        }
        for scope in touched_scopes {
            if let Some(scope_contacts) = contacts.get_mut(&scope) {
                scope_contacts.rebuild_order();
            }
        }

        Ok(())
    }

    pub(super) fn apply_user_blocked(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: UserBlockedPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let organization_id = conversation_state_organization_id_for_event(event);
        self.mark_friendship_contacts_blocked(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.blocker_user_id.as_str(),
            payload.blocked_user_id.as_str(),
            payload.effective_at.as_str(),
        );
        Ok(())
    }

    pub(super) fn apply_user_block_released(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: UserBlockReleasedPayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        let organization_id = conversation_state_organization_id_for_event(event);
        self.restore_friendship_contacts_after_block_release(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.blocker_user_id.as_str(),
            payload.blocked_user_id.as_str(),
            payload.released_at.as_str(),
        );
        Ok(())
    }

    fn mark_friendship_contacts_blocked(
        &self,
        tenant_id: &str,
        organization_id: &str,
        blocker_user_id: &str,
        blocked_user_id: &str,
        blocked_at: &str,
    ) {
        let scope = contact_runtime_scope(tenant_id, organization_id, blocker_user_id);
        let key = contact_entry_key("friendship", blocked_user_id);
        let mut contacts = self.lock_contact_store("mark_friendship_contacts_blocked");
        if let Some(scope_contacts) = contacts.get_mut(&scope)
            && let Some(contact) = scope_contacts.get_mut(key.as_str())
        {
            contact.relationship_state = "blocked".into();
            contact.last_interaction_at =
                max_rfc3339(contact.last_interaction_at.as_str(), blocked_at).to_owned();
            scope_contacts.rebuild_order();
        }
    }

    fn restore_friendship_contacts_after_block_release(
        &self,
        tenant_id: &str,
        organization_id: &str,
        blocker_user_id: &str,
        blocked_user_id: &str,
        released_at: &str,
    ) {
        let scope = contact_runtime_scope(tenant_id, organization_id, blocker_user_id);
        let key = contact_entry_key("friendship", blocked_user_id);
        let mut contacts =
            self.lock_contact_store("restore_friendship_contacts_after_block_release");
        if let Some(scope_contacts) = contacts.get_mut(&scope)
            && let Some(contact) = scope_contacts.get_mut(key.as_str())
            && contact.relationship_state == "blocked"
            && !contact.friendship_id.trim().is_empty()
        {
            contact.relationship_state = "active".into();
            contact.last_interaction_at =
                max_rfc3339(contact.last_interaction_at.as_str(), released_at).to_owned();
            scope_contacts.rebuild_order();
        }
    }

    fn upsert_friendship_contact(
        &self,
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
        payload: &FriendshipActivatedPayload,
        binding: Option<&ContactDirectChatBindingView>,
    ) {
        let scope = contact_runtime_scope(tenant_id, organization_id, owner_user_id);
        let key = contact_entry_key("friendship", target_user_id);
        let normalized_organization_id = normalize_realtime_organization_id(organization_id);
        let mut contacts = self.lock_contact_store("upsert_friendship_contact");
        let scope_store = contacts.entry(scope).or_default();
        {
            let contact = scope_store.ensure_contact(key.clone(), || ContactView {
                tenant_id: tenant_id.to_owned(),
                organization_id: normalized_organization_id.clone(),
                owner_user_id: owner_user_id.to_owned(),
                target_user_id: target_user_id.to_owned(),
                display_name: None,
                avatar_url: None,
                chat_id: None,
                contact_type: "friendship".into(),
                relationship_state: "active".into(),
                friendship_id: payload.friendship_id.clone(),
                direct_chat_id: payload.direct_chat_id.clone(),
                conversation_id: None,
                established_at: payload.established_at.clone(),
                last_interaction_at: payload.established_at.clone(),
            });

            contact.relationship_state = "active".into();
            contact.friendship_id = payload.friendship_id.clone();
            contact.direct_chat_id = payload
                .direct_chat_id
                .clone()
                .or_else(|| contact.direct_chat_id.clone());
            contact.established_at = std::cmp::min(
                contact.established_at.clone(),
                payload.established_at.clone(),
            );
            contact.last_interaction_at = max_rfc3339(
                contact.last_interaction_at.as_str(),
                payload.established_at.as_str(),
            )
            .to_owned();

            if let Some(binding) = binding {
                contact.conversation_id = Some(binding.conversation_id.clone());
                contact.last_interaction_at = max_rfc3339(
                    contact.last_interaction_at.as_str(),
                    binding.bound_at.as_str(),
                )
                .to_owned();
            }
        }
        scope_store.rebuild_order();
    }

    fn remove_friendship_contact(
        &self,
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
        friendship_id: &str,
    ) {
        let scope = contact_runtime_scope(tenant_id, organization_id, owner_user_id);
        let key = contact_entry_key("friendship", target_user_id);
        let mut contacts = self.lock_contact_store("remove_friendship_contact");
        let mut remove_scope = false;
        if let Some(scope_contacts) = contacts.get_mut(&scope) {
            if scope_contacts
                .get(key.as_str())
                .is_some_and(|contact| contact.friendship_id == friendship_id)
            {
                scope_contacts.remove(key.as_str());
            }
            remove_scope = scope_contacts.is_empty();
        }
        if remove_scope {
            contacts.remove(&scope);
        }
    }

    fn archive_friendship_direct_chat(
        &self,
        tenant_id: &str,
        organization_id: &str,
        friendship_id: &str,
        archived_at: &str,
    ) {
        let targets = {
            let contacts = self.lock_contact_store("archive_friendship_direct_chat");
            contacts
                .iter()
                .filter_map(|(scope, scope_contacts)| {
                    if scope.tenant_id != tenant_id || scope.organization_id != organization_id {
                        return None;
                    }

                    scope_contacts
                        .values()
                        .find(|contact| contact.friendship_id == friendship_id)
                        .and_then(|contact| {
                            contact.direct_chat_id.as_ref().map(|direct_chat_id| {
                                (
                                    scope.tenant_id.clone(),
                                    scope.organization_id.clone(),
                                    direct_chat_id.clone(),
                                )
                            })
                        })
                })
                .collect::<Vec<_>>()
        };

        if targets.is_empty() {
            return;
        }

        let mut bindings = self.lock_direct_chat_bindings("archive_friendship_direct_chat");
        for (tenant_id, organization_id, direct_chat_id) in targets {
            bindings.archive_by_direct_chat_id(
                tenant_id.as_str(),
                organization_id.as_str(),
                direct_chat_id.as_str(),
                archived_at,
            );
        }
    }

    fn direct_chat_binding(
        &self,
        tenant_id: &str,
        organization_id: &str,
        direct_chat_id: &str,
    ) -> Option<ContactDirectChatBindingView> {
        self.lock_direct_chat_bindings("direct_chat_binding")
            .get_by_direct_chat_id(tenant_id, organization_id, direct_chat_id)
            .cloned()
    }

    fn fan_out_friendship_activated_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        payload: &FriendshipActivatedPayload,
    ) {
        self.fan_out_friendship_to_client_route_sync_feeds(
            event,
            payload.user_low_id.as_str(),
            payload.user_high_id.as_str(),
            payload.initiator_user_id.as_str(),
            payload.established_at.as_str(),
        );
        self.fan_out_friendship_to_client_route_sync_feeds(
            event,
            payload.user_high_id.as_str(),
            payload.user_low_id.as_str(),
            payload.initiator_user_id.as_str(),
            payload.established_at.as_str(),
        );
    }

    fn fan_out_friendship_removed_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        payload: &FriendshipRemovedPayload,
    ) {
        self.fan_out_friendship_to_client_route_sync_feeds(
            event,
            payload.user_low_id.as_str(),
            payload.user_high_id.as_str(),
            payload.removed_by_user_id.as_str(),
            payload.removed_at.as_str(),
        );
        self.fan_out_friendship_to_client_route_sync_feeds(
            event,
            payload.user_high_id.as_str(),
            payload.user_low_id.as_str(),
            payload.removed_by_user_id.as_str(),
            payload.removed_at.as_str(),
        );
    }

    fn fan_out_friendship_to_client_route_sync_feeds(
        &self,
        event: &CommitEnvelope,
        principal_id: &str,
        peer_user_id: &str,
        actor_id: &str,
        occurred_at: &str,
    ) {
        for device in registered_client_routes_for_principal_kind(
            self,
            event.tenant_id.as_str(),
            crate::conversation_state::scope::conversation_state_organization_id_for_event(event)
                .as_str(),
            principal_id,
            "user",
        ) {
            self.append_client_route_sync_entry(
                event.tenant_id.as_str(),
                crate::conversation_state::scope::conversation_state_organization_id_for_event(
                    event,
                )
                .as_str(),
                principal_id,
                "user",
                device.device_id.as_str(),
                |sync_seq| im_domain_core::conversation::ClientRouteSyncFeedEntry {
                    tenant_id: event.tenant_id.clone(),
                    principal_id: principal_id.into(),
                    device_id: device.device_id.clone(),
                    sync_seq,
                    origin_event_id: event.event_id.clone(),
                    origin_event_type: event.event_type.clone(),
                    conversation_id: None,
                    message_id: None,
                    message_seq: None,
                    member_id: None,
                    read_seq: None,
                    last_read_message_id: None,
                    actor_id: Some(actor_id.into()),
                    actor_kind: Some("user".into()),
                    actor_device_id: None,
                    summary: Some(peer_user_id.into()),
                    payload_schema: event.payload_schema.clone(),
                    payload: Some(event.payload.clone()),
                    occurred_at: occurred_at.into(),
                },
            );
        }
    }

    fn lock_contact_store(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, HashMap<ContactOwnerScopeKey, ContactScopeStore>> {
        lock_contacts_mutex(&self.contacts, "contact store", operation)
    }

    fn lock_direct_chat_bindings(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, ContactDirectChatBindingRuntimeStore> {
        lock_contacts_mutex(
            &self.direct_chat_bindings,
            "contact direct chat binding store",
            operation,
        )
    }
}

pub(super) fn contact_runtime_scope(
    tenant_id: &str,
    organization_id: &str,
    owner_user_id: &str,
) -> ContactOwnerScopeKey {
    contact_owner_scope_key(tenant_id, organization_id, owner_user_id)
}

pub(super) fn contact_entry_key(contact_type: &str, target_user_id: &str) -> String {
    encode_conversation_state_key_segments([contact_type, target_user_id])
}

fn direct_chat_binding_key(
    tenant_id: &str,
    organization_id: &str,
    direct_chat_id: &str,
) -> ContactDirectChatBindingKey {
    ContactDirectChatBindingKey {
        tenant_id: tenant_id.to_owned(),
        organization_id: normalize_realtime_organization_id(organization_id),
        direct_chat_id: direct_chat_id.to_owned(),
    }
}

fn direct_chat_binding_key_for_view(
    binding: &ContactDirectChatBindingView,
) -> ContactDirectChatBindingKey {
    direct_chat_binding_key(
        binding.tenant_id.as_deref().unwrap_or_default(),
        binding.organization_id.as_deref().unwrap_or("default"),
        binding.direct_chat_id.as_str(),
    )
}

fn direct_chat_conversation_index_key(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
) -> ContactConversationIndexKey {
    ContactConversationIndexKey {
        tenant_id: tenant_id.to_owned(),
        organization_id: normalize_realtime_organization_id(organization_id),
        conversation_id: conversation_id.to_owned(),
    }
}

fn direct_chat_conversation_index_key_for_view(
    binding: &ContactDirectChatBindingView,
) -> ContactConversationIndexKey {
    direct_chat_conversation_index_key(
        binding.tenant_id.as_deref().unwrap_or_default(),
        binding.organization_id.as_deref().unwrap_or("default"),
        binding.conversation_id.as_str(),
    )
}

fn max_rfc3339<'a>(left: &'a str, right: &'a str) -> &'a str {
    if max_rfc3339_string(left.to_owned(), right.to_owned()) == left {
        left
    } else {
        right
    }
}

fn contact_entry_after_keyset_cursor(contact: &ContactView, cursor: &(String, String)) -> bool {
    use std::cmp::Ordering;

    match rfc3339_cmp(cursor.0.as_str(), contact.last_interaction_at.as_str()) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => contact.target_user_id.as_str() > cursor.1.as_str(),
    }
}

fn contact_member_attribute(
    attributes: &std::collections::BTreeMap<String, String>,
    keys: &[&str],
) -> Option<String> {
    keys.iter().find_map(|key| {
        attributes
            .get(*key)
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    })
}

fn contact_matches_query(contact: &ContactView, query: &str) -> bool {
    contact.target_user_id.to_lowercase().contains(query)
        || contact
            .display_name
            .as_ref()
            .is_some_and(|value| value.to_lowercase().contains(query))
        || contact
            .chat_id
            .as_ref()
            .is_some_and(|value| value.to_lowercase().contains(query))
}

fn lock_contacts_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
    operation: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!(
                "recovering poisoned conversation_state-service {lock_name} lock during {operation}"
            );
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contact(target_user_id: &str, last_interaction_at: &str) -> ContactView {
        ContactView {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            owner_user_id: "1".into(),
            target_user_id: target_user_id.into(),
            display_name: None,
            avatar_url: None,
            chat_id: None,
            contact_type: "friendship".into(),
            relationship_state: "active".into(),
            friendship_id: format!("fs_{target_user_id}"),
            direct_chat_id: None,
            conversation_id: None,
            established_at: last_interaction_at.into(),
            last_interaction_at: last_interaction_at.into(),
        }
    }

    #[test]
    fn test_contact_keyset_window_paginates_without_offset_scan() {
        let store = ContactScopeStore::from_items(vec![
            contact("1033", "2026-05-06T00:00:00Z"),
            contact("1032", "2026-05-06T00:00:00.100Z"),
            contact("1031", "2026-05-06T00:00:00.200Z"),
        ]);

        let (first_page, has_more) =
            store.ordered_window_with_cursor(Some(ContactListCursor::Start), 2);
        assert!(has_more);
        assert_eq!(first_page.len(), 2);
        assert_eq!(first_page[0].target_user_id, "1031");
        assert_eq!(first_page[1].target_user_id, "1032");

        let cursor = ContactListCursor::Keyset {
            last_interaction_at: first_page[1].last_interaction_at.clone(),
            target_user_id: first_page[1].target_user_id.clone(),
        };
        let (second_page, has_more) = store.ordered_window_with_cursor(Some(cursor), 2);
        assert!(!has_more);
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page[0].target_user_id, "1033");
    }

    #[test]
    fn contact_window_concurrent_reads_do_not_deadlock() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let service = Arc::new(ConversationStateService::default());
        let barrier = Arc::new(Barrier::new(8));
        let handles = (0..8)
            .map(|_| {
                let service = Arc::clone(&service);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    barrier.wait();
                    for _ in 0..32 {
                        let _window = service.contact_window(
                            "100001",
                            "default",
                            "user_contact_deadlock",
                            20,
                            crate::conversation_state::model::ContactListCursor::Start,
                            None,
                        );
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle
                .join()
                .expect("concurrent contact reads must not deadlock conversation_state mutexes");
        }
    }

    #[test]
    fn test_max_rfc3339_compares_by_instant() {
        assert_eq!(
            max_rfc3339("2026-05-06T00:00:00Z", "2026-05-06T00:00:00.100Z"),
            "2026-05-06T00:00:00.100Z"
        );
    }

    #[test]
    fn test_ordered_contact_views_compares_last_interaction_by_rfc3339_instant() {
        let ordered = ContactScopeStore::from_items(vec![
            contact("1032", "2026-05-06T00:00:00.100Z"),
            contact("1033", "2026-05-06T00:00:00Z"),
        ])
        .ordered_items();

        assert_eq!(
            ordered
                .iter()
                .map(|contact| contact.target_user_id.as_str())
                .collect::<Vec<_>>(),
            vec!["1032", "1033"]
        );
    }

    #[test]
    fn test_user_block_conversation_state_marks_and_restores_friendship_contacts() {
        use im_domain_events::social::{UserBlockReleasedPayload, UserBlockedPayload};

        let service = ConversationStateService::default();
        let friendship_payload = FriendshipActivatedPayload {
            friendship_id: "fs_block".to_owned(),
            user_low_id: "1".to_owned(),
            user_high_id: "2".to_owned(),
            initiator_user_id: "1".to_owned(),
            direct_chat_id: None,
            established_at: "2026-05-06T00:00:00Z".to_owned(),
        };
        let mut friendship_event = CommitEnvelope::minimal(
            "evt_friendship",
            "100001",
            "friendship.activated",
            "social",
            "fs_block",
            1,
        );
        friendship_event.payload = serde_json::to_string(&friendship_payload).expect("payload");
        service
            .apply_friendship_activated(&friendship_event)
            .expect("friendship projects");

        let blocked_payload = UserBlockedPayload {
            block_id: "blk_1".to_owned(),
            blocker_user_id: "1".to_owned(),
            blocked_user_id: "2".to_owned(),
            scope: "all".to_owned(),
            direct_chat_id: None,
            expires_at: None,
            effective_at: "2026-05-07T00:00:00.000Z".to_owned(),
        };
        let mut blocked_event = CommitEnvelope::minimal(
            "evt_block",
            "100001",
            "user_block.blocked",
            "social",
            "blk_1",
            2,
        );
        blocked_event.payload = serde_json::to_string(&blocked_payload).expect("payload");
        service
            .apply_user_blocked(&blocked_event)
            .expect("block projects");

        let blocker_contact = service
            .contacts("100001", "default", "1")
            .into_iter()
            .find(|contact| contact.target_user_id == "2")
            .expect("blocker contact");
        assert_eq!(blocker_contact.relationship_state, "blocked");

        let blocked_user_contact = service
            .contacts("100001", "default", "2")
            .into_iter()
            .find(|contact| contact.target_user_id == "1")
            .expect("blocked user contact");
        assert_eq!(blocked_user_contact.relationship_state, "active");

        let released_payload = UserBlockReleasedPayload {
            block_id: "blk_1".to_owned(),
            blocker_user_id: "1".to_owned(),
            blocked_user_id: "2".to_owned(),
            released_at: "2026-05-08T00:00:00.000Z".to_owned(),
            scope: None,
            direct_chat_id: None,
            expires_at: None,
            effective_at: None,
        };
        let mut released_event = CommitEnvelope::minimal(
            "evt_release",
            "100001",
            "user_block.released",
            "social",
            "blk_1",
            3,
        );
        released_event.payload = serde_json::to_string(&released_payload).expect("payload");
        service
            .apply_user_block_released(&released_event)
            .expect("block release projects");

        let restored_contact = service
            .contacts("100001", "default", "1")
            .into_iter()
            .find(|contact| contact.target_user_id == "2")
            .expect("restored contact");
        assert_eq!(restored_contact.relationship_state, "active");
    }
}
