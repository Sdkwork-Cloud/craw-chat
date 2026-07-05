use std::collections::{BTreeSet, HashMap};

use im_domain_core::conversation::ConversationMember;
use sdkwork_utils_rust::{offset_limit_page_from_iter, OffsetLimitPage};

use super::support::decode_conversation_scope_key;
use super::{encode_conversation_key_segments, ConversationState, RuntimeState};

#[derive(Default)]
pub(super) struct ActorInboxRuntimeStore {
    conversation_ids_by_actor: HashMap<String, BTreeSet<String>>,
}

impl ActorInboxRuntimeStore {
    fn actor_key(
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> String {
        encode_conversation_key_segments([
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
        ])
    }

    pub(super) fn sync_member(&mut self, organization_id: &str, member: &ConversationMember) {
        let actor_key = Self::actor_key(
            member.tenant_id.as_str(),
            organization_id,
            member.principal_kind.as_str(),
            member.principal_id.as_str(),
        );
        if member.is_active() {
            self.conversation_ids_by_actor
                .entry(actor_key)
                .or_default()
                .insert(member.conversation_id.clone());
            return;
        }

        let Some(conversation_ids) = self.conversation_ids_by_actor.get_mut(actor_key.as_str())
        else {
            return;
        };
        conversation_ids.remove(member.conversation_id.as_str());
        if conversation_ids.is_empty() {
            self.conversation_ids_by_actor.remove(actor_key.as_str());
        }
    }

    pub(super) fn rebuild_for_conversation(
        &mut self,
        organization_id: &str,
        conversation: &ConversationState,
    ) {
        for member in conversation.roster.members().values() {
            self.sync_member(organization_id, member);
        }
    }

    pub(super) fn purge_conversation(
        &mut self,
        organization_id: &str,
        conversation: &ConversationState,
    ) {
        for member in conversation.roster.members().values() {
            if !member.is_active() {
                continue;
            }
            let actor_key = Self::actor_key(
                member.tenant_id.as_str(),
                organization_id,
                member.principal_kind.as_str(),
                member.principal_id.as_str(),
            );
            let Some(conversation_ids) = self.conversation_ids_by_actor.get_mut(actor_key.as_str())
            else {
                continue;
            };
            conversation_ids.remove(member.conversation_id.as_str());
            if conversation_ids.is_empty() {
                self.conversation_ids_by_actor.remove(actor_key.as_str());
            }
        }
    }

    pub(super) fn page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        offset: usize,
        limit: usize,
    ) -> OffsetLimitPage<String> {
        let actor_key = Self::actor_key(tenant_id, organization_id, principal_kind, principal_id);
        let Some(conversation_ids) = self.conversation_ids_by_actor.get(actor_key.as_str()) else {
            return OffsetLimitPage {
                items: Vec::new(),
                next_cursor: None,
                has_more: false,
            };
        };
        offset_limit_page_from_iter(conversation_ids.iter().cloned(), limit, offset)
    }
}

impl RuntimeState {
    pub(super) fn sync_actor_inbox_member(
        &mut self,
        organization_id: &str,
        member: &ConversationMember,
    ) {
        self.actor_inbox.sync_member(organization_id, member);
    }

    pub(super) fn sync_actor_inbox_members(
        &mut self,
        organization_id: &str,
        members: &[ConversationMember],
    ) {
        for member in members {
            self.actor_inbox.sync_member(organization_id, member);
        }
    }

    pub(super) fn rebuild_actor_inbox_for_conversation(
        &mut self,
        organization_id: &str,
        conversation: &ConversationState,
    ) {
        self.actor_inbox
            .rebuild_for_conversation(organization_id, conversation);
    }

    pub(super) fn rebuild_all_actor_inboxes(&mut self) {
        let scope_keys: Vec<String> = self.conversations.keys().cloned().collect();
        self.actor_inbox = ActorInboxRuntimeStore::default();
        for scope_key in scope_keys {
            let Some((_, organization_id, _)) = decode_conversation_scope_key(scope_key.as_str()) else {
                continue;
            };
            let Some(conversation) = self.conversations.get(scope_key.as_str()) else {
                continue;
            };
            self.actor_inbox
                .rebuild_for_conversation(organization_id.as_str(), conversation);
        }
    }

    pub(super) fn actor_inbox_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        offset: usize,
        limit: usize,
    ) -> sdkwork_utils_rust::OffsetLimitPage<String> {
        self.actor_inbox.page(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            offset,
            limit,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::conversation::{
        ConversationMember, MembershipRole, MembershipState,
    };

    fn sample_member(conversation_id: &str, principal_id: &str) -> ConversationMember {
        ConversationMember {
            tenant_id: "100001".into(),
            conversation_id: conversation_id.into(),
            member_id: format!("m_{principal_id}"),
            principal_id: principal_id.into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            state: MembershipState::Joined,
            invited_by: None,
            joined_at: "2026-01-01T00:00:00.000Z".into(),
            removed_at: None,
            attributes: Default::default(),
        }
    }

    #[test]
    fn actor_inbox_pages_active_conversations_without_scanning_all_state() {
        let mut store = ActorInboxRuntimeStore::default();
        store.sync_member("default", &sample_member("c_1", "alice"));
        store.sync_member("default", &sample_member("c_2", "alice"));
        store.sync_member("default", &sample_member("c_3", "bob"));

        let page = store.page("100001", "default", "user", "alice", 0, 1);
        assert_eq!(page.items.len(), 1);
        assert!(page.has_more);
        assert_eq!(page.next_cursor.as_deref(), Some("1"));

        let second = store.page(
            "100001",
            "default",
            "user",
            "alice",
            1,
            1,
        );
        assert_eq!(second.items.len(), 1);
        assert!(!second.has_more);
    }
}
