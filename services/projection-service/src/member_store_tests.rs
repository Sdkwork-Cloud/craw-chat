//! White-box unit tests for projection member store.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "member_store_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

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
