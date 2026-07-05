use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationInboxPeerView, ConversationMember,
    max_read_seq_for_member,
};

use crate::projection::latest_summary_activity_at;
use crate::{TimelineProjectionService, lock_projection_mutex};

fn decode_inbox_keyset_cursor(
    cursor: &str,
) -> Option<crate::model::InboxListCursor> {
    let wire: crate::model::InboxKeysetCursorWire = if cursor.contains('.') {
        let payload = crate::cursor_auth::decode_signed_projection_cursor(cursor).ok()?;
        serde_json::from_value(payload).ok()?
    } else {
        serde_json::from_str(cursor).ok()?
    };
    if wire.activity_at.trim().is_empty() || wire.scope.trim().is_empty() {
        return None;
    }
    Some(crate::model::InboxListCursor::Keyset {
        activity_at: wire.activity_at,
        scope: wire.scope,
    })
}

impl TimelineProjectionService {
    pub fn inbox_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<ConversationInboxEntry> {
        const INBOX_EXPORT_PAGE_SIZE: usize = 200;
        const INBOX_EXPORT_MAX_ITEMS: usize = 10_000;
        let mut items = Vec::new();
        let mut cursor = crate::model::InboxListCursor::Start;

        loop {
            if items.len() >= INBOX_EXPORT_MAX_ITEMS {
                break;
            }
            let page_limit = (INBOX_EXPORT_PAGE_SIZE)
                .min(INBOX_EXPORT_MAX_ITEMS.saturating_sub(items.len()));
            let window = self.inbox_window_for_principal_kind_filtered(
                tenant_id,
                principal_id,
                principal_kind,
                page_limit,
                cursor,
                |_| true,
            );
            items.extend(window.items);
            if !window.has_more {
                break;
            }
            let Some(next_cursor) = window.next_cursor else {
                break;
            };
            cursor = match decode_inbox_keyset_cursor(next_cursor.as_str()) {
                Some(next) => next,
                None => break,
            };
        }

        items
    }

    pub(crate) fn inbox_window_for_principal_kind_filtered<F>(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        limit: usize,
        cursor: crate::model::InboxListCursor,
        mut filter: F,
    ) -> crate::InboxWindowView
    where
        F: FnMut(&ConversationInboxEntry) -> bool,
    {
        let limit = limit.max(1);
        let mut window = Vec::with_capacity(limit.saturating_add(1).min(512));
        let mut last_scope: Option<String> = None;

        let keyset_cursor = match &cursor {
            crate::model::InboxListCursor::Keyset {
                activity_at,
                scope,
            } => Some((activity_at.clone(), scope.clone())),
            _ => None,
        };

        let mut visit_scope = |scope: &str| -> bool {
            let Some(entry) = self.build_inbox_entry_for_scope(
                tenant_id,
                principal_id,
                principal_kind,
                scope,
            ) else {
                return true;
            };
            if !filter(&entry) {
                return true;
            }
            window.push(entry);
            if window.len() <= limit {
                last_scope = Some(scope.to_owned());
            }
            window.len() <= limit
        };

        let exhausted = {
            let members = lock_projection_mutex(&self.members, "member store");
            match &cursor {
                crate::model::InboxListCursor::Offset(offset) => {
                    let mut skipped = 0usize;
                    members.for_each_inbox_scope_after_cursor(
                        tenant_id,
                        principal_kind,
                        principal_id,
                        None,
                        |scope| {
                            if skipped < *offset {
                                skipped += 1;
                                return true;
                            }
                            visit_scope(scope)
                        },
                    )
                }
                _ => members.for_each_inbox_scope_after_cursor(
                    tenant_id,
                    principal_kind,
                    principal_id,
                    keyset_cursor,
                    |scope| visit_scope(scope),
                ),
            }
        };

        let has_more = window.len() > limit || !exhausted;
        if window.len() > limit {
            window.truncate(limit);
        }
        let next_cursor = if has_more {
            window.last().and_then(|entry| {
                last_scope.as_ref().and_then(|scope| {
                    let payload = serde_json::json!({
                        "activityAt": entry.last_activity_at,
                        "scope": scope,
                    });
                    crate::cursor_auth::encode_signed_projection_cursor(&payload).ok()
                })
            })
        } else {
            None
        };
        crate::InboxWindowView {
            items: window,
            next_cursor,
            has_more,
        }
    }

    fn build_inbox_entry_for_scope(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope: &str,
    ) -> Option<ConversationInboxEntry> {
        let members = lock_projection_mutex(&self.members, "member store");
        let summaries = lock_projection_mutex(&self.summaries, "summary store");
        let cursors = lock_projection_mutex(&self.read_cursors, "cursor store");
        let received_messages =
            lock_projection_mutex(&self.received_messages, "received message index");
        let conversations = lock_projection_mutex(&self.conversations, "conversation store");

        let scope_members = members.get(scope)?;
        let member = scope_members.values().find(|member| {
            member.principal_id == principal_id
                && member.principal_kind == principal_kind
                && member.is_active()
                && member.tenant_id == tenant_id
        })?;
        let summary = summaries.get(scope);
        let conversation = conversations.get(scope);
        let conversation_type = conversation
            .map(|entry| entry.conversation_type.clone())
            .unwrap_or_else(|| "unknown".into());
        let peer = direct_inbox_peer_for_member(
            conversation_type.as_str(),
            scope_members.values(),
            member,
        );
        let display_name = peer.as_ref().and_then(|view| view.display_name.clone());
        let avatar_url = peer.as_ref().and_then(|view| view.avatar_url.clone());
        let display_source = display_name
            .as_ref()
            .map(|_| "member_projection".to_owned());
        let read_seq = max_read_seq_for_member(
            cursors
                .get(scope)
                .map(|scope_cursors| scope_cursors.values())
                .into_iter()
                .flatten(),
            member.member_id.as_str(),
        );
        let unread_count = received_messages.unread_count_after(
            scope,
            member.principal_id.as_str(),
            member.principal_kind.as_str(),
            read_seq,
        );

        Some(ConversationInboxEntry {
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
        })
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
