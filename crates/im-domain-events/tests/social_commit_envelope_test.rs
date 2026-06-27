use im_domain_events::{
    AggregateType, CommitEnvelope, EventActor,
    social::{
        DirectChatBoundPayload, ExternalMemberLinkBoundPayload, FriendRequestAcceptedPayload,
        FriendRequestCanceledPayload, FriendRequestDeclinedPayload, FriendRequestSubmittedPayload,
        FriendshipActivatedPayload, FriendshipRemovedPayload, SocialCommitEnvelopeInput,
        SocialEventType, UserBlockedPayload, social_commit_envelope,
    },
};
use serde_json::Value;

#[test]
fn test_social_aggregate_types_expose_stable_wire_values() {
    assert_eq!(
        AggregateType::FriendRequest.as_wire_value(),
        "friend_request"
    );
    assert_eq!(AggregateType::Friendship.as_wire_value(), "friendship");
    assert_eq!(AggregateType::UserBlock.as_wire_value(), "user_block");
    assert_eq!(AggregateType::DirectChat.as_wire_value(), "direct_chat");
}

#[test]
fn test_social_event_type_exposes_wire_and_schema_values() {
    assert_eq!(
        SocialEventType::FriendRequestSubmitted.as_wire_value(),
        "friend_request.submitted"
    );
    assert_eq!(
        SocialEventType::FriendRequestAccepted.as_wire_value(),
        "friend_request.accepted"
    );
    assert_eq!(
        SocialEventType::FriendRequestDeclined.as_wire_value(),
        "friend_request.declined"
    );
    assert_eq!(
        SocialEventType::FriendRequestCanceled.as_wire_value(),
        "friend_request.canceled"
    );
    assert_eq!(
        SocialEventType::UserBlocked.as_wire_value(),
        "user_block.blocked"
    );
    assert_eq!(
        SocialEventType::FriendshipRemoved.as_wire_value(),
        "friendship.removed"
    );
    assert_eq!(
        SocialEventType::FriendshipActivated.payload_schema(),
        "social.friendship.activated.v1"
    );
    assert_eq!(
        SocialEventType::DirectChatBound.as_wire_value(),
        "direct_chat.bound"
    );
}

#[test]
fn test_social_payloads_serialize_expected_shape() {
    let request = FriendRequestSubmittedPayload {
        request_id: "fr_demo".into(),
        requester_user_id: "user_a".into(),
        target_user_id: "user_b".into(),
        request_message: None,
        requested_at: "2026-04-10T10:00:00Z".into(),
    };
    let accepted = FriendRequestAcceptedPayload {
        request_id: "fr_demo".into(),
        accepted_by_user_id: "user_b".into(),
        accepted_at: "2026-04-10T10:00:30Z".into(),
    };
    let declined = FriendRequestDeclinedPayload {
        request_id: "fr_demo".into(),
        declined_by_user_id: "user_b".into(),
        declined_at: "2026-04-10T10:00:45Z".into(),
    };
    let canceled = FriendRequestCanceledPayload {
        request_id: "fr_demo".into(),
        canceled_by_user_id: "user_a".into(),
        canceled_at: "2026-04-10T10:00:50Z".into(),
    };
    let friendship = FriendshipActivatedPayload {
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        initiator_user_id: "user_a".into(),
        direct_chat_id: Some("dc_demo".into()),
        established_at: "2026-04-10T10:01:00Z".into(),
    };
    let removed = FriendshipRemovedPayload {
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        removed_by_user_id: "user_b".into(),
        removed_at: "2026-04-10T10:01:30Z".into(),
    };
    let blocked = UserBlockedPayload {
        block_id: "blk_demo".into(),
        blocker_user_id: "user_a".into(),
        blocked_user_id: "user_b".into(),
        scope: "all".into(),
        direct_chat_id: Some("dc_demo".into()),
        expires_at: None,
        effective_at: "2026-04-10T10:02:00Z".into(),
    };
    let bound = DirectChatBoundPayload {
        direct_chat_id: "dc_demo".into(),
        conversation_id: "c_demo".into(),
        left_actor_id: "actor_a".into(),
        right_actor_id: "actor_b".into(),
        pair_hash: "actor_a:actor_b".into(),
        bound_at: "2026-04-10T10:03:00Z".into(),
    };
    let external_member_link = ExternalMemberLinkBoundPayload {
        link_id: "eml_demo".into(),
        connection_id: "ec_demo".into(),
        local_actor_id: "actor_a".into(),
        local_actor_kind: "user".into(),
        external_member_id: "partner::alice".into(),
        external_display_name: Some("Alice Partner".into()),
        linked_at: "2026-04-10T10:04:00Z".into(),
    };

    let request_value = serde_json::to_value(request).expect("request should serialize");
    let accepted_value = serde_json::to_value(accepted).expect("accepted should serialize");
    let declined_value = serde_json::to_value(declined).expect("declined should serialize");
    let canceled_value = serde_json::to_value(canceled).expect("canceled should serialize");
    let friendship_value = serde_json::to_value(friendship).expect("friendship should serialize");
    let removed_value = serde_json::to_value(removed).expect("removed payload should serialize");
    let blocked_value = serde_json::to_value(blocked).expect("block should serialize");
    let bound_value = serde_json::to_value(bound).expect("bound should serialize");
    let external_member_link_value =
        serde_json::to_value(external_member_link).expect("external member link should serialize");

    assert_eq!(request_value["requestId"], Value::String("fr_demo".into()));
    assert_eq!(
        accepted_value["acceptedByUserId"],
        Value::String("user_b".into())
    );
    assert_eq!(
        declined_value["declinedByUserId"],
        Value::String("user_b".into())
    );
    assert_eq!(
        canceled_value["canceledByUserId"],
        Value::String("user_a".into())
    );
    assert_eq!(
        friendship_value["directChatId"],
        Value::String("dc_demo".into())
    );
    assert_eq!(
        removed_value["removedByUserId"],
        Value::String("user_b".into())
    );
    assert_eq!(blocked_value["scope"], Value::String("all".into()));
    assert_eq!(
        bound_value["conversationId"],
        Value::String("c_demo".into())
    );
    assert_eq!(
        external_member_link_value["localActorKind"],
        Value::String("user".into())
    );
}

#[test]
fn test_social_commit_envelope_builds_social_defaults() {
    let payload = serde_json::to_string(&FriendshipActivatedPayload {
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        initiator_user_id: "user_a".into(),
        direct_chat_id: Some("dc_demo".into()),
        established_at: "2026-04-10T10:01:00Z".into(),
    })
    .expect("payload should serialize");

    let envelope = social_commit_envelope(SocialCommitEnvelopeInput {
        event_id: "evt_demo",
        tenant_id: "100001",
        organization_id: "org_a",
        aggregate_type: AggregateType::Friendship,
        aggregate_id: "fs_demo",
        event_type: SocialEventType::FriendshipActivated,
        ordering_seq: 7,
        actor: EventActor {
            actor_id: "user_a".into(),
            actor_kind: "user".into(),
            actor_session_id: Some("s_demo".into()),
        },
        occurred_at: "2026-04-10T10:01:00Z",
        committed_at: "2026-04-10T10:01:01Z",
        payload: &payload,
    });

    assert_eq!(envelope.aggregate_type, AggregateType::Friendship);
    assert_eq!(envelope.event_type, "friendship.activated");
    assert_eq!(
        envelope.payload_schema,
        Some("social.friendship.activated.v1".into())
    );
    assert_eq!(envelope.scope_type, "friendship");
    assert_eq!(envelope.scope_id, "fs_demo");
    assert_eq!(
        envelope.ordering_key,
        CommitEnvelope::ordering_key("100001", "fs_demo")
    );
    assert_eq!(envelope.audit_class, "social");
    assert_eq!(envelope.retention_class, "standard");
    assert_eq!(envelope.normalized_organization_id(), "org_a");
}
