use crate::{ConversationMemberDirectoryEntry, TimelineProjectionService};
use im_domain_core::conversation::MembershipRole;
use im_time::rfc3339_cmp;
use sdkwork_utils_rust::{cursor_window_page_info, SdkWorkPageData};

use super::model::MemberDirectoryListCursor;
use super::scope_key;

impl TimelineProjectionService {
    pub fn member_directory(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<ConversationMemberDirectoryEntry> {
        let mut items = super::lock_projection_mutex(&self.members, "member store")
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
        items
    }

    pub(crate) fn member_directory_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        page_size: usize,
        cursor: MemberDirectoryListCursor,
    ) -> SdkWorkPageData<ConversationMemberDirectoryEntry> {
        let list_cursor = match cursor {
            MemberDirectoryListCursor::Start => None,
            other => Some(other),
        };
        let (items, has_more) = member_directory_window_slice(
            self.member_directory(tenant_id, organization_id, conversation_id),
            list_cursor,
            page_size,
        );
        let next_cursor = if has_more {
            items.last().and_then(|entry| {
                let payload = serde_json::json!({
                    "roleRank": member_directory_role_rank(&entry.role),
                    "joinedAt": entry.joined_at,
                    "principalId": entry.principal_id,
                });
                crate::cursor_auth::encode_signed_projection_cursor(&payload).ok()
            })
        } else {
            None
        };
        SdkWorkPageData {
            items,
            page_info: cursor_window_page_info(Some(page_size), next_cursor, has_more),
        }
    }
}

pub(super) fn member_directory_window_slice(
    items: Vec<ConversationMemberDirectoryEntry>,
    cursor: Option<MemberDirectoryListCursor>,
    limit: usize,
) -> (Vec<ConversationMemberDirectoryEntry>, bool) {
    let mut window = Vec::with_capacity(limit.saturating_add(1));
    let legacy_offset = matches!(cursor, Some(MemberDirectoryListCursor::Offset(_)));
    let offset = match cursor {
        Some(MemberDirectoryListCursor::Offset(value)) => value,
        _ => 0,
    };
    let keyset_cursor = match cursor {
        Some(MemberDirectoryListCursor::Keyset {
            role_rank,
            joined_at,
            principal_id,
        }) => Some((role_rank, joined_at, principal_id)),
        _ => None,
    };
    let mut skipped = 0usize;
    for entry in items {
        if let Some((role_rank, joined_at, principal_id)) = keyset_cursor.as_ref()
            && !member_entry_after_keyset_cursor(&entry, *role_rank, joined_at, principal_id)
        {
            continue;
        }
        if legacy_offset && skipped < offset {
            skipped += 1;
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

    fn member(principal_id: &str, role: MembershipRole, joined_at: &str) -> ConversationMemberDirectoryEntry {
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
        let (first_page, has_more) =
            member_directory_window_slice(items.clone(), None, 2);
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
