use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationInboxPeerView, ConversationInboxPreferencesView,
    ConversationMember, max_read_seq_for_member,
};
use sdkwork_utils_rust::SdkWorkPageData;

use super::list_page;

use crate::conversation_state::event_apply::latest_summary_activity_at;
use crate::conversation_state::member_store::ConversationStateMemberRuntimeStore;
use crate::conversation_state::{ConversationStateService, lock_conversation_state_mutex};

/// Member conversation_state fields captured under a short `members` lock so inbox reads
/// never hold `members` while acquiring summary/cursor/received/conversation locks
/// (write paths take `received` → `summaries` → `members`).
struct InboxMemberContext {
    member: ConversationMember,
    scope_member_views: Vec<ConversationMember>,
}

fn non_empty_owned(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

pub(crate) struct InboxWindowQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_id: &'a str,
    pub principal_kind: &'a str,
    pub limit: usize,
    pub cursor: crate::conversation_state::model::InboxListCursor,
}

fn decode_inbox_keyset_cursor(
    cursor: &str,
) -> Option<crate::conversation_state::model::InboxListCursor> {
    let wire: crate::conversation_state::model::InboxKeysetCursorWire = if cursor.contains('.') {
        let payload =
            crate::conversation_state::cursor_auth::decode_signed_conversation_state_cursor(cursor)
                .ok()?;
        serde_json::from_value(payload).ok()?
    } else {
        serde_json::from_str(cursor).ok()?
    };
    if wire.activity_at.trim().is_empty() || wire.scope.trim().is_empty() {
        return None;
    }
    Some(crate::conversation_state::model::InboxListCursor::Keyset {
        activity_at: wire.activity_at,
        scope: wire.scope,
    })
}

impl ConversationStateService {
    /// Export/sync helper: pages through the inbox index until `INBOX_EXPORT_MAX_ITEMS`.
    /// Interactive HTTP list APIs must use `inbox_window_for_principal_kind_filtered` directly.
    pub fn inbox_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<
        Vec<ConversationInboxEntry>,
        crate::conversation_state::event_apply::ConversationStateError,
    > {
        const INBOX_EXPORT_PAGE_SIZE: usize = 200;
        const INBOX_EXPORT_MAX_ITEMS: usize = 10_000;
        let mut items = Vec::new();
        let mut cursor = crate::conversation_state::model::InboxListCursor::Start;

        loop {
            if items.len() >= INBOX_EXPORT_MAX_ITEMS {
                break;
            }
            let page_limit =
                (INBOX_EXPORT_PAGE_SIZE).min(INBOX_EXPORT_MAX_ITEMS.saturating_sub(items.len()));
            let window = self.inbox_window_for_principal_kind_filtered(
                InboxWindowQuery {
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    limit: page_limit,
                    cursor,
                },
                |_| true,
            )?;
            items.extend(window.items);
            if window.page_info.has_more != Some(true) {
                break;
            }
            let Some(next_cursor) = window.page_info.next_cursor else {
                break;
            };
            cursor = match decode_inbox_keyset_cursor(next_cursor.as_str()) {
                Some(next) => next,
                None => break,
            };
        }

        Ok(items)
    }

    pub(crate) fn inbox_window_for_principal_kind_filtered<F>(
        &self,
        query: InboxWindowQuery<'_>,
        mut filter: F,
    ) -> Result<
        SdkWorkPageData<ConversationInboxEntry>,
        crate::conversation_state::event_apply::ConversationStateError,
    >
    where
        F: FnMut(&ConversationInboxEntry) -> bool,
    {
        let limit = query.limit.max(1);
        let mut window = Vec::with_capacity(limit.saturating_add(1).min(512));
        let mut last_returned_cursor: Option<(String, String)> = None;
        let mut scan_cursor = match &query.cursor {
            crate::conversation_state::model::InboxListCursor::Keyset { activity_at, scope } => {
                Some((activity_at.clone(), scope.clone()))
            }
            _ => None,
        };
        let scan_batch_size = limit
            .saturating_mul(8)
            .max(limit.saturating_add(1))
            .min(512);
        let mut exhausted = false;

        while window.len() <= limit && !exhausted {
            let mut scanned_entries = Vec::with_capacity(scan_batch_size);
            exhausted = {
                let members = lock_conversation_state_mutex(&self.members, "member store");
                members.for_each_inbox_activity_after_cursor(
                    query.tenant_id,
                    query.organization_id,
                    query.principal_kind,
                    query.principal_id,
                    scan_cursor.clone(),
                    |activity_at, scope| {
                        scanned_entries.push((activity_at.to_owned(), scope.to_owned()));
                        scanned_entries.len() < scan_batch_size
                    },
                )
            };

            if scanned_entries.is_empty() {
                exhausted = true;
                break;
            }

            scan_cursor = scanned_entries.last().cloned();
            for (activity_at, scope) in scanned_entries {
                let Some(entry) = self.build_inbox_entry_for_scope(
                    query.tenant_id,
                    query.organization_id,
                    query.principal_id,
                    query.principal_kind,
                    scope.as_str(),
                ) else {
                    continue;
                };
                if !filter(&entry) {
                    continue;
                }
                window.push(entry);
                if window.len() <= limit {
                    last_returned_cursor = Some((activity_at, scope));
                }
                if window.len() > limit {
                    break;
                }
            }
        }

        let has_more = window.len() > limit || !exhausted;
        if window.len() > limit {
            window.truncate(limit);
        }
        let next_cursor = if has_more {
            window.last().and_then(|_| {
                last_returned_cursor
                    .as_ref()
                    .and_then(|(activity_at, scope)| {
                        let payload = serde_json::json!({
                            "activityAt": activity_at,
                            "scope": scope,
                        });
                        crate::conversation_state::cursor_auth::encode_conversation_state_list_cursor(&payload).ok()
                    })
            })
        } else {
            None
        };
        if has_more && next_cursor.is_none() {
            return Err(
                crate::conversation_state::event_apply::ConversationStateError::InvalidEvent(
                    "failed to encode inbox list cursor".into(),
                ),
            );
        }
        Ok(list_page::cursor_page(window, limit, next_cursor, has_more))
    }

    fn build_inbox_entry_for_scope(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope: &str,
    ) -> Option<ConversationInboxEntry> {
        let member_context = {
            let members = lock_conversation_state_mutex(&self.members, "member store");
            let scope_members = members.get(scope)?;
            let member = scope_members.values().find(|member| {
                member.principal_id == principal_id
                    && member.principal_kind == principal_kind
                    && member.is_active()
                    && member.tenant_id == tenant_id
            })?;
            Some(InboxMemberContext {
                member: member.clone(),
                scope_member_views: scope_members.values().cloned().collect(),
            })
        };
        self.build_inbox_entry_from_member_context(scope, organization_id, member_context?)
    }

    fn build_inbox_entry_from_member_context(
        &self,
        scope: &str,
        organization_id: &str,
        member_context: InboxMemberContext,
    ) -> Option<ConversationInboxEntry> {
        let InboxMemberContext {
            member,
            scope_member_views,
        } = member_context;
        // Snapshot each store under a single lock to avoid AB-BA deadlocks with journal writers.
        let conversation = {
            let conversations =
                lock_conversation_state_mutex(&self.conversations, "conversation store");
            conversations.get(scope).cloned()
        };
        let summary = {
            let summaries = lock_conversation_state_mutex(&self.summaries, "summary store");
            summaries.get(scope).cloned()
        };
        let read_seq = {
            let cursors = lock_conversation_state_mutex(&self.read_cursors, "cursor store");
            max_read_seq_for_member(
                cursors
                    .get(scope)
                    .map(|scope_cursors| scope_cursors.values())
                    .into_iter()
                    .flatten(),
                member.member_id.as_str(),
            )
        };
        let unread_count = {
            let received_messages =
                lock_conversation_state_mutex(&self.received_messages, "received message index");
            received_messages.unread_count_after(
                scope,
                member.principal_id.as_str(),
                member.principal_kind.as_str(),
                read_seq,
            )
        };

        let conversation_type = conversation
            .as_ref()
            .map(|entry| entry.conversation_type.clone())
            .unwrap_or_else(|| "unknown".into());
        let conversation_profile = if conversation_type.eq_ignore_ascii_case("group") {
            lock_conversation_state_mutex(&self.conversation_profiles, "conversation profile store")
                .get(scope)
                .cloned()
        } else {
            None
        };
        let mut peer = direct_inbox_peer_for_member(
            conversation_type.as_str(),
            scope_member_views.iter(),
            &member,
        );
        // Resolve missing peer display attributes from the IM user profile
        // table so direct chats show names instead of raw principal IDs.
        if let Some(view) = peer.as_mut()
            && view.display_name.is_none()
            && view.principal_kind == "user"
            && let Some(display) =
                self.resolve_user_display(&member.tenant_id, organization_id, &view.principal_id, "user")
        {
            view.display_name = Some(display.display_name);
            if view.avatar_url.is_none() {
                view.avatar_url = display.avatar_url;
            }
        }
        let profile_display_name = conversation_profile
            .as_ref()
            .and_then(|profile| non_empty_owned(profile.display_name.as_str()));
        let catalog_display_name = conversation
            .as_ref()
            .and_then(|entry| entry.title.as_deref())
            .and_then(non_empty_owned);
        let peer_display_name = peer.as_ref().and_then(|view| view.display_name.clone());
        let (display_name, display_source) = if let Some(display_name) = profile_display_name {
            (Some(display_name), Some("conversation_profile".to_owned()))
        } else if conversation_type.eq_ignore_ascii_case("group") {
            catalog_display_name.map_or((None, None), |display_name| {
                (Some(display_name), Some("conversation_catalog".to_owned()))
            })
        } else {
            let display_source = peer_display_name
                .as_ref()
                .map(|_| "member_conversation_state".to_owned());
            (peer_display_name, display_source)
        };
        let avatar_url = conversation_profile
            .as_ref()
            .and_then(|profile| non_empty_owned(profile.avatar_url.as_str()))
            .or_else(|| peer.as_ref().and_then(|view| view.avatar_url.clone()));
        let conversation_preferences = self.conversation_preferences(
            member.tenant_id.as_str(),
            organization_id,
            member.conversation_id.as_str(),
            member.principal_kind.as_str(),
            member.principal_id.as_str(),
        );
        let preferences = Some(ConversationInboxPreferencesView {
            is_pinned: conversation_preferences.is_pinned,
            is_muted: conversation_preferences.is_muted,
            is_marked_unread: conversation_preferences.is_marked_unread,
            is_hidden: conversation_preferences.is_hidden,
        });

        Some(ConversationInboxEntry {
            tenant_id: member.tenant_id.clone(),
            principal_id: member.principal_id.clone(),
            member_id: member.member_id.clone(),
            conversation_id: member.conversation_id.clone(),
            conversation_type,
            message_count: summary
                .as_ref()
                .map(|view| view.message_count)
                .unwrap_or_default(),
            last_message_id: summary
                .as_ref()
                .and_then(|view| view.last_message_id.clone()),
            last_message_seq: summary
                .as_ref()
                .map(|view| view.last_message_seq)
                .unwrap_or_default(),
            last_sender_id: summary
                .as_ref()
                .and_then(|view| view.last_sender_id.clone()),
            last_sender_kind: summary
                .as_ref()
                .and_then(|view| view.last_sender_kind.clone()),
            last_summary: summary.as_ref().and_then(|view| view.last_summary.clone()),
            unread_count,
            last_activity_at: summary
                .as_ref()
                .and_then(latest_summary_activity_at)
                .or_else(|| conversation.as_ref().map(|entry| entry.created_at.clone()))
                .unwrap_or_else(|| member.joined_at.clone()),
            display_name,
            avatar_url,
            display_source,
            peer,
            preferences,
            agent_handoff: summary.as_ref().and_then(|view| view.agent_handoff.clone()),
        })
    }

    #[allow(dead_code)]
    fn build_inbox_entry_for_scope_with_members(
        &self,
        members: &ConversationStateMemberRuntimeStore,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope: &str,
    ) -> Option<ConversationInboxEntry> {
        let scope_members = members.get(scope)?;
        let member = scope_members.values().find(|member| {
            member.principal_id == principal_id
                && member.principal_kind == principal_kind
                && member.is_active()
                && member.tenant_id == tenant_id
        })?;
        self.build_inbox_entry_from_member_context(
            scope,
            organization_id,
            InboxMemberContext {
                member: member.clone(),
                scope_member_views: scope_members.values().cloned().collect(),
            },
        )
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

#[cfg(test)]
mod deadlock_regression_tests {
    use std::sync::{Arc, Barrier};
    use std::thread;

    use im_domain_events::CommitEnvelope;

    use super::*;

    const TEST_CURSOR_SECRET_ENV: &str = "SDKWORK_IM_CONVERSATION_STATE_CURSOR_HS256_SECRET";

    struct TestEnvGuard {
        _lock: std::sync::MutexGuard<'static, ()>,
        previous: Option<String>,
    }

    impl TestEnvGuard {
        fn set_cursor_secret() -> Self {
            let lock = crate::conversation_state::lock_conversation_state_test_environment();
            let previous = std::env::var(TEST_CURSOR_SECRET_ENV).ok();
            unsafe {
                std::env::set_var(
                    TEST_CURSOR_SECRET_ENV,
                    "conversation_state-service-inbox-test-cursor-secret-32-bytes",
                );
            }
            Self {
                _lock: lock,
                previous,
            }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(value) => std::env::set_var(TEST_CURSOR_SECRET_ENV, value),
                    None => std::env::remove_var(TEST_CURSOR_SECRET_ENV),
                }
            }
        }
    }

    fn seed_inbox_scope(service: &ConversationStateService) {
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_conv_inbox_deadlock",
                    "100001",
                    "conversation.created",
                    "conversation",
                    "c_inbox_deadlock",
                    0,
                )
                .with_payload(
                    "conversation.created.v1",
                    r#"{"conversationId":"c_inbox_deadlock","conversationType":"group","scenario":"standard","title":"Inbox deadlock regression","createdAt":"2026-07-06T00:00:00Z"}"#,
                ),
            )
            .expect("conversation created");
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_member_inbox_deadlock",
                    "100001",
                    "conversation.member_joined",
                    "conversation",
                    "c_inbox_deadlock",
                    1,
                )
                .with_payload(
                    "conversation.member.v1",
                    r#"{
                        "tenantId":"100001",
                        "conversationId":"c_inbox_deadlock",
                        "memberId":"cm_inbox_deadlock",
                        "principalId":"user_inbox_deadlock",
                        "principalKind":"user",
                        "role":"member",
                        "state":"joined",
                        "invitedBy":null,
                        "joinedAt":"2026-07-06T00:00:00Z",
                        "removedAt":null,
                        "attributes":{}
                    }"#,
                ),
            )
            .expect("member joined");
    }

    fn seed_filtered_inbox_scope(
        service: &ConversationStateService,
        conversation_id: &str,
        conversation_type: &str,
        joined_at: &str,
        ordering_seq: u64,
    ) {
        let created_payload = format!(
            r#"{{
                "conversationId":"{conversation_id}",
                "conversationType":"{conversation_type}",
                "scenario":"standard",
                "title":"Filtered inbox regression",
                "createdAt":"{joined_at}"
            }}"#
        );
        service
            .apply(
                &CommitEnvelope::minimal(
                    format!("evt_created_{conversation_id}").as_str(),
                    "100001",
                    "conversation.created",
                    "conversation",
                    conversation_id,
                    ordering_seq,
                )
                .with_payload("conversation.created.v1", created_payload.as_str()),
            )
            .expect("conversation created");
        let member_payload = format!(
            r#"{{
                "tenantId":"100001",
                "conversationId":"{conversation_id}",
                "memberId":"cm_{conversation_id}",
                "principalId":"user_filtered_inbox",
                "principalKind":"user",
                "role":"member",
                "state":"joined",
                "invitedBy":null,
                "joinedAt":"{joined_at}",
                "removedAt":null,
                "attributes":{{}}
            }}"#
        );
        service
            .apply(
                &CommitEnvelope::minimal(
                    format!("evt_member_{conversation_id}").as_str(),
                    "100001",
                    "conversation.member_joined",
                    "conversation",
                    conversation_id,
                    ordering_seq + 1,
                )
                .with_payload("conversation.member.v1", member_payload.as_str()),
            )
            .expect("member joined");
    }

    #[test]
    fn inbox_window_filter_scans_until_matching_page_is_full() {
        let service = ConversationStateService::default();
        for index in 0..10 {
            seed_filtered_inbox_scope(
                &service,
                format!("c_direct_filter_{index}").as_str(),
                "direct",
                format!("2026-07-08T00:00:{index:02}Z").as_str(),
                index * 2,
            );
        }
        seed_filtered_inbox_scope(
            &service,
            "g_filtered_after_directs",
            "group",
            "2026-07-01T00:00:00Z",
            100,
        );

        let window = service
            .inbox_window_for_principal_kind_filtered(
                InboxWindowQuery {
                    tenant_id: "100001",
                    organization_id: "0",
                    principal_id: "user_filtered_inbox",
                    principal_kind: "user",
                    limit: 1,
                    cursor: crate::conversation_state::model::InboxListCursor::Start,
                },
                |entry| entry.conversation_type == "group",
            )
            .expect("filtered inbox window");

        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].conversation_id, "g_filtered_after_directs");
        assert_eq!(window.page_info.has_more, Some(false));
    }

    #[test]
    fn inbox_search_uses_latest_group_profile_and_scans_past_newer_nonmatches() {
        let service = ConversationStateService::default();
        for index in 0..10 {
            seed_filtered_inbox_scope(
                &service,
                format!("c_direct_search_{index}").as_str(),
                "direct",
                format!("2026-07-08T00:00:{index:02}Z").as_str(),
                index * 2,
            );
        }
        seed_filtered_inbox_scope(
            &service,
            "g_search_by_profile",
            "group",
            "2026-07-01T00:00:00Z",
            100,
        );
        service.update_conversation_profile(
            "100001",
            "0",
            "g_search_by_profile",
            "user",
            "group_owner",
            crate::conversation_state::UpdateConversationProfileRequest {
                display_name: Some("Commercial Launch Team".into()),
                avatar_url: None,
                notice: None,
            },
        );
        let auth = im_app_context::local_service_app_context(
            "100001",
            "user_filtered_inbox",
            "user",
            None,
            ["*"],
        );

        let window = service
            .inbox_window_from_auth_context_filtered(
                &auth,
                Some(1),
                None,
                Some("group"),
                Some("launch team"),
            )
            .expect("search latest group profile");

        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].conversation_id, "g_search_by_profile");
        assert_eq!(
            window.items[0].display_name.as_deref(),
            Some("Commercial Launch Team")
        );
        assert_eq!(window.page_info.has_more, Some(false));
    }

    #[test]
    fn inbox_search_cursor_pages_matching_groups_without_duplicates() {
        let _cursor_secret = TestEnvGuard::set_cursor_secret();
        let service = ConversationStateService::default();
        for (index, conversation_id) in ["g_search_cursor_new", "g_search_cursor_old"]
            .into_iter()
            .enumerate()
        {
            seed_filtered_inbox_scope(
                &service,
                conversation_id,
                "group",
                format!("2026-07-0{}T00:00:00Z", 8 - index).as_str(),
                index as u64 * 2,
            );
            service.update_conversation_profile(
                "100001",
                "0",
                conversation_id,
                "user",
                "group_owner",
                crate::conversation_state::UpdateConversationProfileRequest {
                    display_name: Some(format!("Search Cursor Result {index}")),
                    avatar_url: None,
                    notice: None,
                },
            );
        }
        let auth = im_app_context::local_service_app_context(
            "100001",
            "user_filtered_inbox",
            "user",
            None,
            ["*"],
        );

        let first = service
            .inbox_window_from_auth_context_filtered(
                &auth,
                Some(1),
                None,
                Some("group"),
                Some("search cursor result"),
            )
            .expect("first search page");
        let next_cursor = first
            .page_info
            .next_cursor
            .as_deref()
            .expect("first search page cursor");
        let second = service
            .inbox_window_from_auth_context_filtered(
                &auth,
                Some(1),
                Some(next_cursor),
                Some("group"),
                Some("search cursor result"),
            )
            .expect("second search page");

        assert_eq!(first.items.len(), 1);
        assert_eq!(first.page_info.has_more, Some(true));
        assert_eq!(second.items.len(), 1);
        assert_ne!(
            first.items[0].conversation_id,
            second.items[0].conversation_id
        );
        assert_eq!(second.page_info.has_more, Some(false));
    }

    #[test]
    fn inbox_search_rejects_queries_longer_than_256_characters() {
        let service = ConversationStateService::default();
        let auth = im_app_context::local_service_app_context(
            "100001",
            "user_filtered_inbox",
            "user",
            None,
            ["*"],
        );
        let query = "q".repeat(257);

        let error = service
            .inbox_window_from_auth_context_filtered(
                &auth,
                Some(20),
                None,
                Some("group"),
                Some(query.as_str()),
            )
            .expect_err("oversized inbox search query must be rejected");

        assert_eq!(error.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(error.code(), "payload_too_large");
    }

    #[test]
    fn group_inbox_uses_latest_conversation_profile_for_every_member() {
        let service = ConversationStateService::default();
        seed_inbox_scope(&service);

        service.update_conversation_profile(
            "100001",
            "0",
            "c_inbox_deadlock",
            "user",
            "group_owner",
            crate::conversation_state::UpdateConversationProfileRequest {
                display_name: Some("Renamed by owner".into()),
                avatar_url: Some("https://cdn.example.test/renamed-group.png".into()),
                notice: None,
            },
        );

        let window = service
            .inbox_window_for_principal_kind_filtered(
                InboxWindowQuery {
                    tenant_id: "100001",
                    organization_id: "0",
                    principal_id: "user_inbox_deadlock",
                    principal_kind: "user",
                    limit: 20,
                    cursor: crate::conversation_state::model::InboxListCursor::Start,
                },
                |_| true,
            )
            .expect("member inbox window");

        let entry = window.items.first().expect("group inbox entry");
        assert_eq!(entry.display_name.as_deref(), Some("Renamed by owner"));
        assert_eq!(
            entry.avatar_url.as_deref(),
            Some("https://cdn.example.test/renamed-group.png")
        );
        assert_eq!(
            entry.display_source.as_deref(),
            Some("conversation_profile")
        );
    }

    #[test]
    fn inbox_window_concurrent_reads_do_not_deadlock() {
        let service = Arc::new(ConversationStateService::default());
        seed_inbox_scope(service.as_ref());

        let barrier = Arc::new(Barrier::new(8));
        let handles = (0..8)
            .map(|_| {
                let service = Arc::clone(&service);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    barrier.wait();
                    for _ in 0..32 {
                        let _window = service
                            .inbox_window_for_principal_kind_filtered(
                                InboxWindowQuery {
                                    tenant_id: "100001",
                                    organization_id: "0",
                                    principal_id: "user_inbox_deadlock",
                                    principal_kind: "user",
                                    limit: 20,
                                    cursor:
                                        crate::conversation_state::model::InboxListCursor::Start,
                                },
                                |_| true,
                            )
                            .expect("inbox window");
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle
                .join()
                .expect("concurrent inbox reads must not deadlock conversation_state mutexes");
        }
    }

    #[test]
    fn inbox_window_from_auth_context_returns_without_reentrant_member_lock() {
        use std::sync::mpsc;
        use std::time::Duration;

        let service = Arc::new(ConversationStateService::default());
        seed_inbox_scope(service.as_ref());
        let auth = im_app_context::local_service_app_context(
            "100001",
            "user_inbox_deadlock",
            "user",
            None,
            ["*"],
        );
        let (tx, rx) = mpsc::channel();
        let worker_service = Arc::clone(&service);

        std::thread::spawn(move || {
            let result = worker_service.inbox_window_from_auth_context(&auth, Some(20), None);
            let _ = tx.send(result.map(|window| window.items.len()));
        });

        let result = rx
            .recv_timeout(Duration::from_secs(1))
            .expect("inbox auth-context read must not remain blocked on conversation_state locks");
        assert_eq!(result.expect("inbox auth-context read should succeed"), 1);
    }
}
