use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use im_domain_core::social::DirectChatStatus;
use im_domain_events::CommitEnvelope;
use im_domain_events::social::{
    DirectChatBoundPayload, FriendshipActivatedPayload, FriendshipRemovedPayload,
};
use im_time::{max_rfc3339_string, rfc3339_cmp};

use im_platform_contracts::normalize_realtime_organization_id;

use crate::client_route_sync::registered_client_routes_for_principal_kind;
use crate::model::ContactDirectChatBindingView;
use crate::{ContactView, TimelineProjectionService};

use super::projection::ProjectionError;
use super::scope::{
    ContactOwnerScopeKey, contact_owner_scope_key, encode_projection_key_segments,
    projection_organization_id_for_event,
};

#[derive(Default)]
pub(crate) struct ContactDirectChatBindingRuntimeStore {
    by_direct_chat_id: HashMap<ContactDirectChatBindingKey, ContactDirectChatBindingView>,
    direct_chat_id_by_conversation: HashMap<ContactConversationIndexKey, ContactDirectChatBindingKey>,
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
        if let Some(previous) = self.by_direct_chat_id.insert(binding_key.clone(), binding.clone()) {
            self.direct_chat_id_by_conversation.remove(
                &direct_chat_conversation_index_key_for_view(&previous),
            );
        }
        self.direct_chat_id_by_conversation
            .insert(conversation_key, binding_key);
    }

    pub(crate) fn extend(
        &mut self,
        bindings: impl IntoIterator<Item = ContactDirectChatBindingView>,
    ) {
        for binding in bindings {
            self.insert(binding);
        }
    }

    pub(crate) fn get_by_direct_chat_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        direct_chat_id: &str,
    ) -> Option<&ContactDirectChatBindingView> {
        self.by_direct_chat_id
            .get(&direct_chat_binding_key(
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
        let binding_key = self.direct_chat_id_by_conversation.get(
            &direct_chat_conversation_index_key(tenant_id, organization_id, conversation_id),
        )?;
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

    pub(crate) fn values(&self) -> impl Iterator<Item = &ContactDirectChatBindingView> {
        self.by_direct_chat_id.values()
    }

    pub(crate) fn clear(&mut self) {
        self.by_direct_chat_id.clear();
        self.direct_chat_id_by_conversation.clear();
    }
}

impl TimelineProjectionService {
    pub fn contacts(
        &self,
        tenant_id: &str,
        organization_id: &str,
        owner_user_id: &str,
    ) -> Vec<ContactView> {
        let scope = contact_runtime_scope(tenant_id, organization_id, owner_user_id);
        let items = self
            .lock_contact_store("contacts")
            .get(&scope)
            .map(|scope_contacts| scope_contacts.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        ordered_contact_views(items)
    }

    pub(super) fn apply_friendship_activated(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: FriendshipActivatedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
        let binding = payload
            .direct_chat_id
            .as_ref()
            .and_then(|direct_chat_id| {
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
    ) -> Result<(), ProjectionError> {
        let payload: FriendshipRemovedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
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
    ) -> Result<(), ProjectionError> {
        let payload: DirectChatBoundPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let organization_id = projection_organization_id_for_event(event);
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
        for (scope, scope_contacts) in contacts.iter_mut() {
            if scope.tenant_id != event.tenant_id || scope.organization_id != organization_id {
                continue;
            }
            for contact in scope_contacts.values_mut() {
                if contact.direct_chat_id.as_deref() == Some(payload.direct_chat_id.as_str()) {
                    contact.conversation_id = Some(payload.conversation_id.clone());
                    contact.last_interaction_at = max_rfc3339(
                        contact.last_interaction_at.as_str(),
                        payload.bound_at.as_str(),
                    )
                    .to_owned();
                }
            }
        }

        Ok(())
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
        let normalized_organization_id =
            normalize_realtime_organization_id(organization_id);
        let mut contacts = self.lock_contact_store("upsert_friendship_contact");
        let contact = contacts
            .entry(scope)
            .or_default()
            .entry(key)
            .or_insert_with(|| ContactView {
                tenant_id: tenant_id.to_owned(),
                organization_id: normalized_organization_id.clone(),
                owner_user_id: owner_user_id.to_owned(),
                target_user_id: target_user_id.to_owned(),
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
            crate::scope::projection_organization_id_for_event(event).as_str(),
            principal_id,
            "user",
        ) {
            self.append_client_route_sync_entry(
                event.tenant_id.as_str(),
                crate::scope::projection_organization_id_for_event(event).as_str(),
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
    ) -> MutexGuard<'_, HashMap<ContactOwnerScopeKey, HashMap<String, ContactView>>> {
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
    encode_projection_key_segments([contact_type, target_user_id])
}

pub(super) fn contact_snapshot_items(
    scope_contacts: &HashMap<String, ContactView>,
) -> Vec<ContactView> {
    ordered_contact_views(scope_contacts.values().cloned().collect::<Vec<_>>())
}

pub(super) fn contact_map_from_items(items: Vec<ContactView>) -> HashMap<String, ContactView> {
    items
        .into_iter()
        .map(|item| {
            (
                contact_entry_key(item.contact_type.as_str(), item.target_user_id.as_str()),
                item,
            )
        })
        .collect()
}

pub(super) fn ordered_contact_views(mut items: Vec<ContactView>) -> Vec<ContactView> {
    items.sort_by(|left, right| {
        rfc3339_cmp(
            right.last_interaction_at.as_str(),
            left.last_interaction_at.as_str(),
        )
        .then_with(|| left.target_user_id.cmp(&right.target_user_id))
    });
    items
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

fn direct_chat_binding_key_for_view(binding: &ContactDirectChatBindingView) -> ContactDirectChatBindingKey {
    direct_chat_binding_key(
        binding.tenant_id.as_deref().unwrap_or_default(),
        binding
            .organization_id
            .as_deref()
            .unwrap_or("default"),
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
        binding
            .organization_id
            .as_deref()
            .unwrap_or("default"),
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

fn lock_contacts_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
    operation: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!(
                "recovering poisoned projection-service {lock_name} lock during {operation}"
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
    fn test_max_rfc3339_compares_by_instant() {
        assert_eq!(
            max_rfc3339("2026-05-06T00:00:00Z", "2026-05-06T00:00:00.100Z"),
            "2026-05-06T00:00:00.100Z"
        );
    }

    #[test]
    fn test_ordered_contact_views_compares_last_interaction_by_rfc3339_instant() {
        let ordered = ordered_contact_views(vec![
            contact("1032", "2026-05-06T00:00:00.100Z"),
            contact("1033", "2026-05-06T00:00:00Z"),
        ]);

        assert_eq!(
            ordered
                .iter()
                .map(|contact| contact.target_user_id.as_str())
                .collect::<Vec<_>>(),
            vec!["1032", "1033"]
        );
    }
}
