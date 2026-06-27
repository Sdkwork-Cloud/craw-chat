use conversation_runtime::{
    ConversationRuntime, CreateRoomCommand, EnterRoomCommand, InMemoryJournal, PostMessageCommand,
};
use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};

#[test]
fn test_game_room_create_enter_and_post_message() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    runtime
        .create_room_with_creator_kind(
            CreateRoomCommand {
                tenant_id: "100001".into(),
                organization_id: "org_a".into(),
                conversation_id: "c_game_room_flow".into(),
                room_id: "room_game_flow".into(),
                room_kind: "game".into(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("game room create should succeed");

    runtime
        .enter_room_with_principal_kind(
            EnterRoomCommand {
                tenant_id: "100001".into(),
                organization_id: "org_a".into(),
                room_id: "room_game_flow".into(),
                principal_id: "1040".into(),
                principal_kind: "user".into(),
            },
            "user",
        )
        .expect("player should enter game room");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "org_a".into(),
            conversation_id: "c_game_room_flow".into(),
            sender: Sender {
                id: "1040".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_player".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("move-1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("play".into()),
                parts: vec![ContentPart::Data(im_domain_core::message::DataPart {
                    schema_ref: im_domain_core::room::game_move_schema_ref("landlord.play"),
                    encoding: "application/json".into(),
                    payload: r#"{"seat":1,"cards":["7S"]}"#.into(),
                })],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("game move message should post");

    assert_eq!(posted.message_seq, 1);
}

#[test]
fn test_live_room_message_rate_limit_rejects_burst_posts() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());
    runtime
        .create_room_with_creator_kind(
            CreateRoomCommand {
                tenant_id: "100001".into(),
                organization_id: "org_a".into(),
                conversation_id: "c_live_room_rate".into(),
                room_id: "room_live_rate".into(),
                room_kind: "live".into(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("live room create should succeed");

    unsafe {
        std::env::set_var("SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT", "2");
    }

    let post = |client_msg_id: &str| {
        runtime.post_message(PostMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "org_a".into(),
            conversation_id: "c_live_room_rate".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some(client_msg_id.into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hi".into()),
                parts: vec![ContentPart::text("hi")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
    };

    post("live-1").expect("first live message should post");
    post("live-2").expect("second live message should post");
    let burst = post("live-3").expect_err("third live message should hit rate limit");
    assert!(
        format!("{burst:?}").contains("rate limit"),
        "expected rate limit error, got: {burst:?}"
    );

    unsafe {
        std::env::remove_var("SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT");
    }
}
