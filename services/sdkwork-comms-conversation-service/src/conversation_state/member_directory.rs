use crate::conversation_state::{ConversationMemberDirectoryEntry, ConversationStateService};
use im_domain_core::conversation::MembershipRole;
use sdkwork_utils_rust::{SdkWorkPageData, cursor_window_page_info};

use super::model::MemberDirectoryListCursor;
use super::scope_key;

#[cfg(test)]
use im_time::rfc3339_cmp;

impl ConversationStateService {
    pub fn member_directory(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<ConversationMemberDirectoryEntry> {
        let mut items = super::lock_conversation_state_mutex(&self.members, "member store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.tenant_id == tenant_id && member.is_active())
                    .map(ConversationMemberDirectoryEntry::from_member)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        items.sort_by(|left, right| {
            member_directory_role_rank(&left.role)
                .cmp(&member_directory_role_rank(&right.role))
                .then_with(|| left.joined_at.cmp(&right.joined_at))
                .then_with(|| left.principal_id.cmp(&right.principal_id))
        });
        self.enrich_directory_display(organization_id, &mut items);
        items
    }

    pub(crate) fn member_directory_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        page_size: usize,
        cursor: MemberDirectoryListCursor,
    ) -> Result<
        SdkWorkPageData<ConversationMemberDirectoryEntry>,
        crate::conversation_state::event_apply::ConversationStateError,
    > {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let keyset_cursor = match cursor {
            MemberDirectoryListCursor::Start => None,
            MemberDirectoryListCursor::Keyset {
                role_rank,
                joined_at,
                principal_id,
            } => Some((role_rank, joined_at, principal_id)),
        };
        let (members, has_more) =
            super::lock_conversation_state_mutex(&self.members, "member store")
                .collect_member_directory_window(
                    scope.as_str(),
                    tenant_id,
                    keyset_cursor,
                    page_size,
                );
        let mut items = members
            .into_iter()
            .map(|member| ConversationMemberDirectoryEntry::from_member(&member))
            .collect::<Vec<_>>();
        self.enrich_directory_display(organization_id, &mut items);
        let next_cursor = if has_more {
            items
                .last()
                .map(|entry| {
                    let payload = serde_json::json!({
                        "roleRank": member_directory_role_rank(&entry.role),
                        "joinedAt": entry.joined_at,
                        "principalId": entry.principal_id,
                    });
                    crate::conversation_state::cursor_auth::encode_conversation_state_list_cursor(
                        &payload,
                    )
                })
                .transpose()?
        } else {
            None
        };
        Ok(SdkWorkPageData {
            items,
            page_info: cursor_window_page_info(Some(page_size), next_cursor, has_more),
        })
    }

    /// Fills missing `displayName`/`avatarUrl` member attributes from the IM
    /// user profile table for user principals (read-time enrichment only; the
    /// in-memory member store is never mutated).
    fn enrich_directory_display(
        &self,
        organization_id: &str,
        entries: &mut [ConversationMemberDirectoryEntry],
    ) {
        for entry in entries {
            if entry.principal_kind != "user"
                || entry.attributes.contains_key("displayName")
                || entry.attributes.contains_key("display_name")
            {
                continue;
            }
            let Some(display) = self.resolve_user_display(
                &entry.tenant_id,
                organization_id,
                &entry.principal_id,
                "user",
            ) else {
                continue;
            };
            entry
                .attributes
                .entry("displayName".to_owned())
                .or_insert(display.display_name);
            if let Some(avatar_url) = display.avatar_url {
                entry
                    .attributes
                    .entry("avatarUrl".to_owned())
                    .or_insert(avatar_url);
            }
        }
    }
}

#[cfg(test)]
pub(super) fn member_directory_window_slice(
    items: Vec<ConversationMemberDirectoryEntry>,
    cursor: Option<MemberDirectoryListCursor>,
    limit: usize,
) -> (Vec<ConversationMemberDirectoryEntry>, bool) {
    let mut window = Vec::with_capacity(limit.saturating_add(1));
    let keyset_cursor = match cursor {
        Some(MemberDirectoryListCursor::Keyset {
            role_rank,
            joined_at,
            principal_id,
        }) => Some((role_rank, joined_at, principal_id)),
        _ => None,
    };
    for entry in items {
        if let Some((role_rank, joined_at, principal_id)) = keyset_cursor.as_ref()
            && !member_entry_after_keyset_cursor(&entry, *role_rank, joined_at, principal_id)
        {
            continue;
        }
        window.push(entry);
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

#[cfg(test)]
fn member_entry_after_keyset_cursor(
    entry: &ConversationMemberDirectoryEntry,
    role_rank: u8,
    joined_at: &str,
    principal_id: &str,
) -> bool {
    use std::cmp::Ordering;

    match member_directory_role_rank(&entry.role).cmp(&role_rank) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => match rfc3339_cmp(joined_at, entry.joined_at.as_str()) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => entry.principal_id.as_str() > principal_id,
        },
    }
}

pub(super) fn member_directory_role_rank(role: &MembershipRole) -> u8 {
    match role {
        MembershipRole::Owner => 0,
        MembershipRole::Admin => 1,
        MembershipRole::Member => 2,
        MembershipRole::Guest => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::conversation::{MembershipRole, MembershipState};

    fn member(
        principal_id: &str,
        role: MembershipRole,
        joined_at: &str,
    ) -> ConversationMemberDirectoryEntry {
        ConversationMemberDirectoryEntry {
            tenant_id: "100001".into(),
            conversation_id: "c1".into(),
            member_id: format!("m_{principal_id}"),
            principal_id: principal_id.into(),
            principal_kind: "user".into(),
            role,
            state: MembershipState::Joined,
            invited_by: None,
            joined_at: joined_at.into(),
            removed_at: None,
            attributes: Default::default(),
        }
    }

    #[test]
    fn member_directory_keyset_window_paginates_without_offset_scan() {
        let items = vec![
            member("u1", MembershipRole::Owner, "2026-05-06T00:00:00.000Z"),
            member("u2", MembershipRole::Member, "2026-05-06T00:00:00.100Z"),
            member("u3", MembershipRole::Member, "2026-05-06T00:00:00.200Z"),
        ];
        let (first_page, has_more) = member_directory_window_slice(items.clone(), None, 2);
        assert!(has_more);
        assert_eq!(first_page.len(), 2);
        assert_eq!(first_page[0].principal_id, "u1");
        assert_eq!(first_page[1].principal_id, "u2");

        let cursor = Some(MemberDirectoryListCursor::Keyset {
            role_rank: member_directory_role_rank(&first_page[1].role),
            joined_at: first_page[1].joined_at.clone(),
            principal_id: first_page[1].principal_id.clone(),
        });
        let (second_page, has_more) = member_directory_window_slice(items, cursor, 2);
        assert!(!has_more);
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page[0].principal_id, "u3");
    }
}
