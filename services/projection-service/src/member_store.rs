use std::collections::{BTreeSet, HashMap};
use std::cmp::Ordering;

use im_domain_core::conversation::{ConversationMember, principal_member_key};

#[derive(Clone, Eq, PartialEq)]
struct InboxActivityEntry {
    activity_at: String,
    scope: String,
}

impl Ord for InboxActivityEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .activity_at
            .cmp(&self.activity_at)
            .then_with(|| self.scope.cmp(&other.scope))
    }
}

impl PartialOrd for InboxActivityEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub(crate) struct ProjectionMemberRuntimeStore {
    by_conversation: HashMap<String, HashMap<String, ConversationMember>>,
    conversation_members_by_typed_principal: HashMap<String, BTreeSet<String>>,
    inbox_activity_by_principal: HashMap<String, BTreeSet<InboxActivityEntry>>,
}

impl ProjectionMemberRuntimeStore {
    pub(crate) fn clear(&mut self) {
        self.by_conversation.clear();
        self.conversation_members_by_typed_principal.clear();
        self.inbox_activity_by_principal.clear();
    }

    pub(crate) fn get(&self, scope: &str) -> Option<&HashMap<String, ConversationMember>> {
        self.by_conversation.get(scope)
    }

    pub(crate) fn insert_member(&mut self, scope: String, member: ConversationMember) {
        let member_key =
            principal_member_key(member.principal_id.as_str(), member.principal_kind.as_str());
        let previous = self
            .by_conversation
            .entry(scope.clone())
            .or_default()
            .insert(member_key, member.clone());

        let mut affected_principals = Vec::new();
        if let Some(previous) = previous {
            affected_principals.push((
                previous.tenant_id,
                previous.principal_kind,
                previous.principal_id,
            ));
        }
        affected_principals.push((
            member.tenant_id.clone(),
            member.principal_kind.clone(),
            member.principal_id.clone(),
        ));
        affected_principals.sort();
        affected_principals.dedup();

        for (tenant_id, principal_kind, principal_id) in affected_principals {
            self.refresh_principal_scope(
                tenant_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                scope.as_str(),
            );
        }
    }

    pub(crate) fn remove_member(
        &mut self,
        scope: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let member_key = principal_member_key(principal_id, principal_kind);
        let removed = self
            .by_conversation
            .get_mut(scope)
            .and_then(|scope_members| scope_members.remove(member_key.as_str()));
        let remove_empty_scope = self
            .by_conversation
            .get(scope)
            .is_some_and(|scope_members| scope_members.is_empty());
        if remove_empty_scope {
            self.by_conversation.remove(scope);
        }

        if let Some(member) = removed.as_ref() {
            self.refresh_principal_scope(
                member.tenant_id.as_str(),
                member.principal_kind.as_str(),
                member.principal_id.as_str(),
                scope,
            );
        }

        removed
    }

    pub(crate) fn remove_conversation(&mut self, scope: &str) {
        let affected_principals = self
            .by_conversation
            .remove(scope)
            .map(|scope_members| {
                scope_members
                    .into_values()
                    .map(|member| (member.tenant_id, member.principal_kind, member.principal_id))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        for (tenant_id, principal_kind, principal_id) in affected_principals {
            self.refresh_principal_scope(
                tenant_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                scope,
            );
        }
    }

    pub(crate) fn member_for_principal_kind(
        &self,
        scope: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<&ConversationMember> {
        self.by_conversation.get(scope).and_then(|scope_members| {
            scope_members.get(principal_member_key(principal_id, principal_kind).as_str())
        })
    }

    pub(crate) fn active_member_scopes_for_principal_kind(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Vec<String> {
        self.inbox_scopes_ordered_for_principal(tenant_id, principal_kind, principal_id)
    }

    pub(crate) fn inbox_scopes_ordered_for_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Vec<String> {
        self.inbox_scope_entries_after_cursor(tenant_id, principal_kind, principal_id, None)
    }

    pub(crate) fn inbox_scope_entries_after_cursor(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        cursor: Option<(String, String)>,
    ) -> Vec<String> {
        let mut scopes = Vec::new();
        self.for_each_inbox_scope_after_cursor(
            tenant_id,
            principal_kind,
            principal_id,
            cursor,
            |scope| {
                scopes.push(scope.to_owned());
                true
            },
        );
        scopes
    }

    /// Iterate inbox scopes in activity order without materializing the full principal inbox.
    ///
    /// The visitor returns `false` to stop early. Returns `true` when every remaining scope was
    /// visited, `false` when iteration stopped early.
    pub(crate) fn for_each_inbox_scope_after_cursor(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        cursor: Option<(String, String)>,
        mut visitor: impl FnMut(&str) -> bool,
    ) -> bool {
        use std::collections::Bound::{Excluded, Unbounded};

        let typed_index_key =
            member_typed_principal_index_key(tenant_id, principal_kind, principal_id);
        let Some(entries) = self.inbox_activity_by_principal.get(typed_index_key.as_str()) else {
            return true;
        };

        if let Some((activity_at, scope)) = cursor {
            let cursor_entry = InboxActivityEntry { activity_at, scope };
            for entry in entries.range((Excluded(cursor_entry), Unbounded)) {
                if !visitor(entry.scope.as_str()) {
                    return false;
                }
            }
        } else {
            for entry in entries.iter() {
                if !visitor(entry.scope.as_str()) {
                    return false;
                }
            }
        }

        true
    }

    pub(crate) fn refresh_inbox_activity_for_scope(&mut self, scope: &str, activity_at: &str) {
        let Some(scope_members) = self.by_conversation.get(scope) else {
            return;
        };
        let active_members = scope_members
            .values()
            .filter(|member| member.is_active())
            .cloned()
            .collect::<Vec<_>>();
        for member in active_members {
            self.upsert_inbox_activity(
                member.tenant_id.as_str(),
                member.principal_kind.as_str(),
                member.principal_id.as_str(),
                scope,
                activity_at,
            );
        }
    }

    fn upsert_inbox_activity(
        &mut self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        scope: &str,
        activity_at: &str,
    ) {
        let typed_index_key =
            member_typed_principal_index_key(tenant_id, principal_kind, principal_id);
        let entries = self
            .inbox_activity_by_principal
            .entry(typed_index_key)
            .or_default();
        entries.retain(|entry| entry.scope != scope);
        entries.insert(InboxActivityEntry {
            activity_at: activity_at.to_owned(),
            scope: scope.to_owned(),
        });
    }

    fn remove_inbox_activity(
        &mut self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        scope: &str,
    ) {
        let typed_index_key =
            member_typed_principal_index_key(tenant_id, principal_kind, principal_id);
        let Some(entries) = self.inbox_activity_by_principal.get_mut(typed_index_key.as_str())
        else {
            return;
        };
        entries.retain(|entry| entry.scope != scope);
        if entries.is_empty() {
            self.inbox_activity_by_principal
                .remove(typed_index_key.as_str());
        }
    }

    fn refresh_principal_scope(
        &mut self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        scope: &str,
    ) {
        let has_active_member = self
            .by_conversation
            .get(scope)
            .is_some_and(|scope_members| {
                scope_members.values().any(|member| {
                    member.tenant_id == tenant_id
                        && member.principal_kind == principal_kind
                        && member.principal_id == principal_id
                        && member.is_active()
                })
            });
        let typed_index_key =
            member_typed_principal_index_key(tenant_id, principal_kind, principal_id);
        if has_active_member {
            self.conversation_members_by_typed_principal
                .entry(typed_index_key.clone())
                .or_default()
                .insert(scope.to_owned());
            let joined_at = self
                .by_conversation
                .get(scope)
                .and_then(|scope_members| {
                    scope_members
                        .get(principal_member_key(principal_id, principal_kind).as_str())
                })
                .map(|member| member.joined_at.as_str())
                .unwrap_or("1970-01-01T00:00:00.000Z")
                .to_owned();
            self.upsert_inbox_activity(
                tenant_id,
                principal_kind,
                principal_id,
                scope,
                joined_at.as_str(),
            );
            return;
        }

        if let Some(scopes) = self
            .conversation_members_by_typed_principal
            .get_mut(typed_index_key.as_str())
        {
            scopes.remove(scope);
            if scopes.is_empty() {
                self.conversation_members_by_typed_principal
                    .remove(typed_index_key.as_str());
            }
        }
        self.remove_inbox_activity(tenant_id, principal_kind, principal_id, scope);
    }
}

fn member_typed_principal_index_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    encode_member_index_key_segments([tenant_id, principal_kind, principal_id])
}

fn encode_member_index_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::conversation::{MembershipRole, MembershipState};
    use std::collections::BTreeMap;

    fn active_member(
        tenant_id: &str,
        conversation_id: &str,
        member_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> ConversationMember {
        ConversationMember {
            tenant_id: tenant_id.into(),
            conversation_id: conversation_id.into(),
            member_id: member_id.into(),
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
            role: MembershipRole::Member,
            state: MembershipState::Joined,
            invited_by: None,
            joined_at: "2026-04-12T00:00:00.000Z".into(),
            removed_at: None,
            attributes: BTreeMap::new(),
        }
    }

    #[test]
    fn test_member_principal_index_key_is_segment_safe() {
        let mut store = ProjectionMemberRuntimeStore::default();
        store.insert_member(
            "scope_a".into(),
            active_member("tenant:segment", "c_a", "cm_a", "user", "principal"),
        );
        store.insert_member(
            "scope_b".into(),
            active_member("tenant", "c_b", "cm_b", "segment:user", "principal"),
        );

        assert_eq!(
            store.active_member_scopes_for_principal_kind("tenant:segment", "user", "principal"),
            vec!["scope_a".to_owned()]
        );
        assert_eq!(
            store.active_member_scopes_for_principal_kind("tenant", "segment:user", "principal"),
            vec!["scope_b".to_owned()]
        );
    }
}
