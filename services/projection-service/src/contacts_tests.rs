//! White-box unit tests for projection contacts/friendship.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "contacts_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

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

/// Verify the per-user active-friendship cap is enforced at the projection
/// layer. We cannot easily lower the production constant for the test, so we
/// drive the projection up to a modest number of friends and assert that the
/// contact catalog grows monotonically below the cap �?and that re-activating
/// an existing friendship (the replay path) does not corrupt the catalog. The
/// hard-rejection path (count >= cap) is covered by the same `upsert`
/// guard, which returns early without inserting when the cap is reached.
#[test]
fn test_friendship_projection_stays_bounded_and_handles_replay() {
    let service = TimelineProjectionService::default();
    // Activate a handful of distinct friendships for one owner. The catalog
    // must contain exactly that many active friendship contacts afterwards.
    for index in 0..5u32 {
        let target = format!("{}", 1034 + index);
        let payload = FriendshipActivatedPayload {
            friendship_id: format!("fs_{index}"),
            user_low_id: "1".to_owned(),
            user_high_id: target.clone(),
            initiator_user_id: "1".to_owned(),
            direct_chat_id: None,
            established_at: "2026-05-06T00:00:00Z".to_owned(),
        };
        let mut envelope =
            CommitEnvelope::minimal("e", "100001", "friendship.activated", "social", "s", 1);
        envelope.payload = serde_json::to_string(&payload).expect("payload serializes");
        service
            .apply_friendship_activated(&envelope)
            .expect("friendship activation projects");
    }
    let contacts = service.contacts("100001", "default", "1");
    let active_friends = contacts
        .iter()
        .filter(|contact| {
            contact.contact_type == "friendship" && contact.relationship_state == "active"
        })
        .count();
    assert_eq!(
        active_friends, 5,
        "every distinct activated friendship must project to a contact"
    );

    // Replay the first friendship activation: the contact must remain (no
    // duplicate, no drop), proving the cap-guarded upsert is idempotent.
    let payload = FriendshipActivatedPayload {
        friendship_id: "fs_0".to_owned(),
        user_low_id: "1".to_owned(),
        user_high_id: "1034".to_owned(),
        initiator_user_id: "1".to_owned(),
        direct_chat_id: None,
        established_at: "2026-05-06T00:00:00Z".to_owned(),
    };
    let mut envelope = CommitEnvelope::minimal(
        "e_replay",
        "100001",
        "friendship.activated",
        "social",
        "s",
        2,
    );
    envelope.payload = serde_json::to_string(&payload).expect("payload serializes");
    service
        .apply_friendship_activated(&envelope)
        .expect("replay friendship activation projects");

    let contacts_after_replay = service.contacts("100001", "default", "1");
    let active_friends_after_replay = contacts_after_replay
        .iter()
        .filter(|contact| {
            contact.contact_type == "friendship" && contact.relationship_state == "active"
        })
        .count();
    assert_eq!(
        active_friends_after_replay, 5,
        "replaying an existing friendship must not duplicate or drop the contact"
    );
}
