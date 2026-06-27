use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationInboxPeerView, ConversationMember,
};

use crate::projection::latest_summary_activity_at;
use crate::{TimelineProjectionService, lock_projection_mutex};

impl TimelineProjectionService {
    pub fn inbox_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<ConversationInboxEntry> {
        let members = lock_projection_mutex(&self.members, "member store");
        let summaries = lock_projection_mutex(&self.summaries, "summary store");
        let cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        let received_messages =
            lock_projection_mutex(&self.received_messages, "received message index");
        let conversations = lock_projection_mutex(&self.conversations, "conversation store");
        let mut items = Vec::new();

        for scope in
            members.active_member_scopes_for_principal_kind(tenant_id, principal_kind, principal_id)
        {
            let Some(scope_members) = members.get(scope.as_str()) else {
                continue;
            };
            for member in scope_members.values().filter(|member| {
                member.principal_id == principal_id
                    && member.principal_kind == principal_kind
                    && member.is_active()
                    && member.tenant_id == tenant_id
            }) {
                let summary = summaries.get(scope.as_str());
                let cursor = cursors
                    .get(scope.as_str())
                    .and_then(|scope_cursors| scope_cursors.get(member.member_id.as_str()));
                let conversation = conversations.get(scope.as_str());
                let conversation_type = conversation
                    .map(|entry| entry.conversation_type.clone())
                    .unwrap_or_else(|| "unknown".into());
                let peer = direct_inbox_peer_for_member(
                    &conversation_type,
                    scope_members.values(),
                    member,
                );
                let display_name = peer.as_ref().and_then(|view| view.display_name.clone());
                let avatar_url = peer.as_ref().and_then(|view| view.avatar_url.clone());
                let display_source = display_name
                    .as_ref()
                    .map(|_| "member_projection".to_owned());
                let read_seq = cursor.map(|view| view.read_seq).unwrap_or_default();
                let unread_count = received_messages.unread_count_after(
                    scope.as_str(),
                    member.principal_id.as_str(),
                    member.principal_kind.as_str(),
                    read_seq,
                );

                items.push(ConversationInboxEntry {
                    tenant_id: member.tenant_id.clone(),
                    principal_id: member.principal_id.clone(),
                    member_id: member.member_id.clone(),
                    conversation_id: member.conversation_id.clone(),
                    conversation_type,
                    message_count: summary.map(|view| view.message_count).unwrap_or_default(),
                    last_message_id: summary.and_then(|view| view.last_message_id.clone()),
                    last_message_seq: summary
                        .map(|view| view.last_message_seq)
                        .unwrap_or_default(),
                    last_sender_id: summary.and_then(|view| view.last_sender_id.clone()),
                    last_sender_kind: summary.and_then(|view| view.last_sender_kind.clone()),
                    last_summary: summary.and_then(|view| view.last_summary.clone()),
                    unread_count,
                    last_activity_at: summary
                        .and_then(latest_summary_activity_at)
                        .or_else(|| conversation.map(|entry| entry.created_at.clone()))
                        .unwrap_or_else(|| member.joined_at.clone()),
                    display_name,
                    avatar_url,
                    display_source,
                    peer,
                    preferences: None,
                    agent_handoff: summary.and_then(|view| view.agent_handoff.clone()),
                });
            }
        }

        items.sort_by(|left, right| right.last_activity_at.cmp(&left.last_activity_at));
        items
    }
}

fn direct_inbox_peer_for_member<'a>(
    conversation_type: &str,
    scope_members: impl Iterator<Item = &'a ConversationMember>,
    member: &ConversationMember,
) -> Option<ConversationInboxPeerView> {
    if !matches!(conversation_type, "single" | "direct") {
        return None;
    }

    let candidates = scope_members
        .filter(|candidate| {
            candidate.tenant_id == member.tenant_id
                && candidate.conversation_id == member.conversation_id
                && candidate.is_active()
                && (candidate.principal_id != member.principal_id
                    || candidate.principal_kind != member.principal_kind)
        })
        .collect::<Vec<_>>();
    candidates
        .iter()
        .copied()
        .find(|candidate| candidate.principal_kind == "user")
        .or_else(|| candidates.first().copied())
        .map(conversation_member_to_inbox_peer)
}

fn conversation_member_to_inbox_peer(member: &ConversationMember) -> ConversationInboxPeerView {
    ConversationInboxPeerView {
        principal_kind: member.principal_kind.clone(),
        principal_id: member.principal_id.clone(),
        user_id: if member.principal_kind == "user" {
            Some(member.principal_id.clone())
        } else {
            None
        },
        chat_id: pick_member_attribute(&member.attributes, &["chatId", "chat_id"]),
        display_name: pick_member_attribute(&member.attributes, &["displayName", "display_name"]),
        avatar_url: pick_member_attribute(
            &member.attributes,
            &["avatarUrl", "avatar_url", "avatar"],
        ),
        relationship_state: pick_member_attribute(
            &member.attributes,
            &["relationshipState", "relationship_state"],
        ),
    }
}

fn pick_member_attribute(
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
