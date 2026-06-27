use im_domain_core::social::{
    BlockScope, DirectChat, DirectChatStatus, ExternalConnection, ExternalConnectionKind,
    ExternalConnectionStatus, ExternalMemberLink, ExternalMemberLinkStatus, FriendRequest,
    FriendRequestStatus, Friendship, FriendshipEvent, FriendshipEventType, FriendshipStatus,
    NormalizedActorPair, NormalizedUserPair, SharedChannelPolicy, SharedChannelPolicyStatus,
    SocialInvariantError, UserBlock, UserBlockStatus, ensure_cross_tenant_connection,
};
use serde_json::Value;

#[test]
fn test_normalized_user_pair_orders_ids_and_rejects_self_pair() {
    let pair = NormalizedUserPair::try_new("user_b", "user_a").expect("pair should normalize");

    assert_eq!(pair.user_low_id, "user_a");
    assert_eq!(pair.user_high_id, "user_b");
    assert!(matches!(
        NormalizedUserPair::try_new("user_a", "user_a"),
        Err(SocialInvariantError::IdenticalPair { .. })
    ));
}

#[test]
fn test_normalized_actor_pair_exposes_stable_pair_hash() {
    let pair = NormalizedActorPair::try_new("actor_z", "actor_a").expect("pair should normalize");

    assert_eq!(pair.left_actor_id, "actor_a");
    assert_eq!(pair.right_actor_id, "actor_z");
    assert_eq!(pair.pair_hash, "actor_a:actor_z");
}

#[test]
fn test_social_contracts_serialize_expected_shape() {
    let friendship = Friendship {
        tenant_id: "100001".into(),
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        initiator_user_id: "user_a".into(),
        status: FriendshipStatus::Active,
        established_at: Some("2026-04-10T09:01:00Z".into()),
        updated_at: "2026-04-10T09:01:00Z".into(),
    };
    let request = FriendRequest {
        tenant_id: "100001".into(),
        request_id: "fr_demo".into(),
        requester_user_id: "user_a".into(),
        target_user_id: "user_b".into(),
        status: FriendRequestStatus::Pending,
        request_message: Some("hello".into()),
        expired_at: Some("2026-04-17T09:00:00Z".into()),
        created_at: "2026-04-10T09:00:00Z".into(),
        updated_at: "2026-04-10T09:00:00Z".into(),
    };
    let event = FriendshipEvent {
        tenant_id: "100001".into(),
        event_id: "fse_demo".into(),
        friendship_id: "fs_demo".into(),
        event_type: FriendshipEventType::Accepted,
        operator_user_id: Some("user_b".into()),
        reason: None,
        occurred_at: "2026-04-10T09:01:00Z".into(),
    };
    let block = UserBlock {
        tenant_id: "100001".into(),
        block_id: "blk_demo".into(),
        blocker_user_id: "user_a".into(),
        blocked_user_id: "user_b".into(),
        scope: BlockScope::All,
        status: UserBlockStatus::Active,
        direct_chat_id: Some("dc_demo".into()),
        expires_at: None,
        created_at: "2026-04-10T09:02:00Z".into(),
        updated_at: "2026-04-10T09:02:00Z".into(),
    };
    let direct_chat = DirectChat {
        tenant_id: "100001".into(),
        direct_chat_id: "dc_demo".into(),
        left_actor_id: "actor_a".into(),
        right_actor_id: "actor_b".into(),
        pair_hash: "actor_a:actor_b".into(),
        status: DirectChatStatus::Active,
        conversation_id: Some("c_demo".into()),
        created_at: "2026-04-10T09:03:00Z".into(),
        updated_at: "2026-04-10T09:03:00Z".into(),
    };

    let friendship_value = serde_json::to_value(friendship).expect("friendship should serialize");
    let request_value = serde_json::to_value(request).expect("request should serialize");
    let event_value = serde_json::to_value(event).expect("event should serialize");
    let block_value = serde_json::to_value(block).expect("block should serialize");
    let direct_chat_value =
        serde_json::to_value(direct_chat).expect("direct chat should serialize");

    assert_eq!(
        friendship_value["userLowId"],
        Value::String("user_a".into())
    );
    assert_eq!(friendship_value["status"], Value::String("active".into()));
    assert_eq!(
        request_value["requestMessage"],
        Value::String("hello".into())
    );
    assert_eq!(event_value["eventType"], Value::String("accepted".into()));
    assert_eq!(block_value["scope"], Value::String("all".into()));
    assert_eq!(
        direct_chat_value["pairHash"],
        Value::String("actor_a:actor_b".into())
    );
}

#[test]
fn test_social_status_helpers_reflect_active_truth() {
    assert!(FriendshipStatus::Active.is_active());
    assert!(!FriendshipStatus::Removed.is_active());
    assert!(UserBlockStatus::Active.is_active());
    assert!(!UserBlockStatus::Released.is_active());
    assert!(DirectChatStatus::Active.is_active());
    assert!(!DirectChatStatus::Archived.is_active());
}

#[test]
fn test_external_collaboration_contracts_serialize_expected_shape() {
    let external_connection = ExternalConnection {
        tenant_id: "100001".into(),
        connection_id: "ec_demo".into(),
        external_tenant_id: "t_partner".into(),
        external_org_name: Some("Partner Org".into()),
        connection_kind: ExternalConnectionKind::SharedChannel,
        status: ExternalConnectionStatus::Active,
        established_at: "2026-04-10T13:00:00Z".into(),
        updated_at: "2026-04-10T13:00:00Z".into(),
    };
    let external_member_link = ExternalMemberLink {
        tenant_id: "100001".into(),
        link_id: "eml_demo".into(),
        connection_id: "ec_demo".into(),
        local_actor_id: "actor_alice".into(),
        local_actor_kind: "user".into(),
        external_member_id: "partner::bob".into(),
        external_display_name: Some("Bob Partner".into()),
        status: ExternalMemberLinkStatus::Active,
        linked_at: "2026-04-10T13:01:00Z".into(),
        updated_at: "2026-04-10T13:01:00Z".into(),
    };
    let shared_channel_policy = SharedChannelPolicy {
        tenant_id: "100001".into(),
        policy_id: "scp_demo".into(),
        connection_id: "ec_demo".into(),
        channel_id: "ch_demo".into(),
        conversation_id: Some("c_demo".into()),
        policy_version: 1,
        history_visibility: "shared".into(),
        status: SharedChannelPolicyStatus::Active,
        applied_at: "2026-04-10T13:02:00Z".into(),
        updated_at: "2026-04-10T13:02:00Z".into(),
    };

    let external_connection_value =
        serde_json::to_value(external_connection).expect("external connection should serialize");
    let external_member_link_value =
        serde_json::to_value(external_member_link).expect("external member link should serialize");
    let shared_channel_policy_value = serde_json::to_value(shared_channel_policy)
        .expect("shared channel policy should serialize");

    assert_eq!(
        external_connection_value["connectionKind"],
        Value::String("shared_channel".into())
    );
    assert_eq!(
        external_member_link_value["externalMemberId"],
        Value::String("partner::bob".into())
    );
    assert_eq!(
        external_member_link_value["localActorKind"],
        Value::String("user".into())
    );
    assert_eq!(
        shared_channel_policy_value["historyVisibility"],
        Value::String("shared".into())
    );
    assert_eq!(
        shared_channel_policy_value["policyVersion"],
        Value::Number(1u64.into())
    );
}

#[test]
fn test_external_collaboration_status_helpers_and_cross_tenant_invariant() {
    assert!(ExternalConnectionStatus::Active.is_active());
    assert!(!ExternalConnectionStatus::Revoked.is_active());
    assert!(ExternalMemberLinkStatus::Active.is_active());
    assert!(!ExternalMemberLinkStatus::Revoked.is_active());
    assert!(SharedChannelPolicyStatus::Active.is_active());
    assert!(!SharedChannelPolicyStatus::Suspended.is_active());

    assert!(ensure_cross_tenant_connection("100001", "t_partner").is_ok());
    assert!(matches!(
        ensure_cross_tenant_connection("100001", "100001"),
        Err(SocialInvariantError::IdenticalPair { .. })
    ));
}
