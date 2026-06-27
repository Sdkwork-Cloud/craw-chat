use std::collections::BTreeMap;

use im_domain_core::conversation::{
    ClientRouteSyncFeedEntry, ConversationActorView, ConversationAgentHandoffView,
    ConversationInboxEntry, ConversationInboxPeerView, ConversationInboxPreferencesView,
    ConversationMember, ConversationReadCursor, MembershipRole, MembershipState,
};
use im_domain_core::media::{DriveReference, MediaKind, MediaResource, MediaSource};
use im_domain_core::message::{
    ContentPart, DataPart, MediaPart, Message, MessageBody, MessageEdited, MessageLocatorIndex,
    MessageRecalled, MessageType, SDKWORK_IM_CUSTOM_MESSAGE_SCHEMA_PREFIX,
    SDKWORK_IM_MESSAGE_SCHEMA_AGENT, SDKWORK_IM_MESSAGE_SCHEMA_AI_IMAGE,
    SDKWORK_IM_MESSAGE_SCHEMA_AI_VIDEO, SDKWORK_IM_MESSAGE_SCHEMA_CARD,
    SDKWORK_IM_MESSAGE_SCHEMA_CONTACT, SDKWORK_IM_MESSAGE_SCHEMA_LINK,
    SDKWORK_IM_MESSAGE_SCHEMA_LOCATION, SDKWORK_IM_MESSAGE_SCHEMA_MUSIC,
    SDKWORK_IM_MESSAGE_SCHEMA_STICKER, SDKWORK_IM_MESSAGE_SCHEMA_VOICE, Sender,
};
use im_domain_core::presence::{
    PresenceClientView, PresenceResumeView, PresenceSnapshotView, PresenceStatus,
};
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscription,
    RealtimeSubscriptionSnapshot,
};
use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent, RtcSignalSender};
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use serde_json::{Value, json};

#[test]
fn test_message_body_serializes_content_parts_with_expected_shape() {
    let message = Message {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        message_id: "m_demo".into(),
        message_seq: 1,
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::from([("role".into(), "owner".into())]),
        },
        message_type: MessageType::Standard,
        delivery_mode: "discrete".into(),
        client_msg_id: Some("client_demo".into()),
        stream_session_id: None,
        rtc_session_id: None,
        body: MessageBody {
            summary: Some("hello".into()),
            parts: vec![
                ContentPart::text("hello"),
                ContentPart::media(MediaPart {
                    drive: DriveReference {
                        drive_uri: "drive://spaces/space_app_upload_demo/nodes/node_image_demo"
                            .into(),
                        space_id: "space_app_upload_demo".into(),
                        node_id: "node_image_demo".into(),
                        node_version: Some("1".into()),
                    },
                    media_role: Some("attachment".into()),
                    resource: MediaResource {
                        id: Some("node_image_demo".into()),
                        kind: MediaKind::Image,
                        source: MediaSource::Drive,
                        url: None,
                        public_url: None,
                        uri: Some(
                            "drive://spaces/space_app_upload_demo/nodes/node_image_demo".into(),
                        ),
                        object_blob_id: None,
                        file_name: Some("demo.png".into()),
                        mime_type: Some("image/png".into()),
                        size_bytes: Some("42".into()),
                        checksum: None,
                        width: None,
                        height: None,
                        duration_seconds: None,
                        alt_text: None,
                        title: Some("poster".into()),
                        poster: None,
                        thumbnails: None,
                        variants: None,
                        access: None,
                        ai: None,
                        metadata: Some(BTreeMap::from([("origin".into(), "test".into())])),
                    },
                }),
            ],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        attributes: BTreeMap::new(),
        metadata: BTreeMap::new(),
        occurred_at: "2026-04-05T10:00:00Z".into(),
        committed_at: Some("2026-04-05T10:00:01Z".into()),
    };

    let value = serde_json::to_value(message).expect("message should serialize");

    assert_eq!(value["messageType"], Value::String("standard".into()));
    assert_eq!(value["sender"]["id"], Value::String("1".into()));
    assert_eq!(value["sender"]["kind"], Value::String("user".into()));
    assert_eq!(value["sender"]["memberId"], Value::String("cm_demo".into()));
    assert_eq!(
        value["sender"]["metadata"]["role"],
        Value::String("owner".into())
    );
    assert_eq!(
        value["body"]["parts"][0]["kind"],
        Value::String("text".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["kind"],
        Value::String("media".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["kind"],
        Value::String("image".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["source"],
        Value::String("drive".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["drive"]["driveUri"],
        Value::String("drive://spaces/space_app_upload_demo/nodes/node_image_demo".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["drive"]["spaceId"],
        Value::String("space_app_upload_demo".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["drive"]["nodeId"],
        Value::String("node_image_demo".into())
    );
    assert!(
        value["body"]["parts"][1].get("mediaAssetId").is_none(),
        "message media parts must reference Drive, not legacy mediaAssetId"
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["mimeType"],
        Value::String("image/png".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["fileName"],
        Value::String("demo.png".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["sizeBytes"],
        Value::String("42".into())
    );
    assert_eq!(
        value["body"]["parts"][1]["resource"]["metadata"]["origin"],
        json!("test")
    );
}

#[test]
fn test_media_content_part_requires_drive_reference_at_domain_boundary() {
    let body = json!({
        "summary": null,
        "renderHints": {},
        "parts": [
            {
                "kind": "media",
                "resource": {
                    "id": "node_image_demo",
                    "kind": "image",
                    "source": "drive",
                    "uri": "drive://spaces/space_app_upload_demo/nodes/node_image_demo"
                }
            }
        ]
    });

    let error = serde_json::from_value::<MessageBody>(body)
        .expect_err("media content parts must require DriveReference");
    assert!(
        error.to_string().contains("drive"),
        "missing drive error should identify the DriveReference field: {error}"
    );
}

#[test]
fn test_media_resource_serializes_drive_backed_profile_without_storage_internals() {
    let resource = MediaResource {
        id: Some("node_01HR6P7ZJQ4A7M2CKA9F0P6R7S".into()),
        kind: MediaKind::Image,
        source: MediaSource::Drive,
        url: None,
        public_url: None,
        uri: Some(
            "drive://spaces/space_app_upload_01/nodes/node_01HR6P7ZJQ4A7M2CKA9F0P6R7S".into(),
        ),
        object_blob_id: Some("objv_01HR6P7ZJQ4A7M2CKA9F0P6R7S_1".into()),
        file_name: Some("demo.png".into()),
        mime_type: Some("image/png".into()),
        size_bytes: Some("424242".into()),
        checksum: None,
        width: Some(1280),
        height: Some(720),
        duration_seconds: None,
        alt_text: Some("demo image".into()),
        title: Some("demo".into()),
        poster: None,
        thumbnails: None,
        variants: None,
        access: None,
        ai: None,
        metadata: Some(BTreeMap::from([(
            "drive".into(),
            json!({
                "spaceId": "space_app_upload_01",
                "nodeId": "node_01HR6P7ZJQ4A7M2CKA9F0P6R7S",
                "spaceType": "app_upload",
                "nodeVersion": "1"
            }),
        )])),
    };

    let value = serde_json::to_value(resource).expect("MediaResource should serialize");

    assert_eq!(
        value["uri"],
        Value::String(
            "drive://spaces/space_app_upload_01/nodes/node_01HR6P7ZJQ4A7M2CKA9F0P6R7S".into()
        )
    );
    assert_eq!(
        value["metadata"]["drive"]["spaceId"],
        Value::String("space_app_upload_01".into())
    );
    for forbidden in ["bucketId", "objectKey", "objectVersion"] {
        assert!(
            value.get(forbidden).is_none(),
            "MediaResource must not serialize storage-internal field {forbidden}"
        );
    }
}

#[test]
fn test_media_resource_rejects_storage_internal_identity_fields() {
    let legacy = json!({
        "id": "node_legacy",
        "kind": "image",
        "source": "drive",
        "uri": "drive://spaces/space_app_upload_01/nodes/node_legacy",
        "bucketId": "media-assets",
        "objectKey": "tenant/100001/node_legacy/demo.png",
        "objectVersion": "1"
    });

    assert!(
        serde_json::from_value::<MediaResource>(legacy).is_err(),
        "MediaResource must reject bucketId/objectKey/objectVersion because Drive owns storage facts"
    );
}

#[test]
fn test_media_resource_rejects_object_storage_source_vocabulary() {
    let legacy = json!({
        "id": "node_legacy",
        "kind": "image",
        "source": "object_storage",
        "uri": "drive://spaces/space_app_upload_01/nodes/node_legacy"
    });

    assert!(
        serde_json::from_value::<MediaResource>(legacy).is_err(),
        "MediaResource.source must use Drive vocabulary instead of storage implementation names"
    );
}

#[test]
fn test_media_part_rejects_legacy_media_asset_id() {
    let legacy = json!({
        "resource": {
            "id": "node_legacy",
            "kind": "image",
            "source": "drive",
            "uri": "drive://spaces/space_app_upload_01/nodes/node_legacy"
        },
        "mediaAssetId": "ma_legacy",
        "mediaRole": "attachment"
    });

    assert!(
        serde_json::from_value::<MediaPart>(legacy).is_err(),
        "MediaPart must reject legacy mediaAssetId and use drive references"
    );
}

#[test]
fn test_media_resource_rejects_legacy_field_aliases() {
    let legacy = json!({
        "uuid": "res_legacy",
        "type": "image",
        "mimeType": "image/png",
        "size": 42,
        "name": "legacy.png",
        "extension": "png",
        "prompt": "legacy prompt"
    });

    assert!(
        serde_json::from_value::<MediaResource>(legacy).is_err(),
        "MediaResource must not accept legacy uuid/type/size/name/extension/prompt aliases"
    );
}

#[test]
fn test_message_locator_index_is_segment_safe_for_delimiter_bearing_ids() {
    let mut index = MessageLocatorIndex::default();

    index.register("tenant:a", "b", "c_left");
    index.register("tenant", "a:b", "c_right");

    assert_eq!(index.conversation_id("tenant:a", "b"), Some("c_left"));
    assert_eq!(index.conversation_id("tenant", "a:b"), Some("c_right"));
}

#[test]
fn test_stream_session_serializes_lifecycle_fields() {
    let session = StreamSession {
        tenant_id: "100001".into(),
        stream_id: "st_demo".into(),
        owner_principal_id: "1".into(),
        owner_principal_kind: "user".into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "conversation".into(),
        scope_id: "c_demo".into(),
        durability_class: StreamDurabilityClass::DurableSession,
        ordering_scope: "stream".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        state: StreamSessionState::Opened,
        last_frame_seq: 3,
        last_checkpoint_seq: Some(2),
        result_message_id: None,
        complete_frame_seq: None,
        abort_frame_seq: None,
        abort_reason: None,
        opened_at: "2026-04-05T10:00:00Z".into(),
        closed_at: None,
        expires_at: None,
    };

    let value = serde_json::to_value(session).expect("stream session should serialize");

    assert_eq!(
        value["durabilityClass"],
        Value::String("durableSession".into())
    );
    assert_eq!(value["ownerPrincipalId"], Value::String("1".into()));
    assert_eq!(value["ownerPrincipalKind"], Value::String("user".into()));
    assert_eq!(value["state"], Value::String("opened".into()));
}

#[test]
fn test_stream_frame_serializes_transport_shape() {
    let frame = StreamFrame {
        tenant_id: "100001".into(),
        stream_id: "st_demo".into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "conversation".into(),
        scope_id: "c_demo".into(),
        frame_seq: 1,
        frame_type: "delta".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        encoding: "json".into(),
        payload: r#"{"delta":"hello"}"#.into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        attributes: BTreeMap::from([("topic".into(), "llm".into())]),
        occurred_at: "2026-04-05T10:00:05Z".into(),
    };

    let value = serde_json::to_value(frame).expect("stream frame should serialize");

    assert_eq!(value["streamId"], Value::String("st_demo".into()));
    assert_eq!(value["frameSeq"], Value::Number(1.into()));
    assert_eq!(value["frameType"], Value::String("delta".into()));
    assert_eq!(value["encoding"], Value::String("json".into()));
    assert_eq!(value["sender"]["id"], Value::String("1".into()));
    assert_eq!(value["attributes"]["topic"], Value::String("llm".into()));
}

#[test]
fn test_rtc_session_serializes_signal_binding_fields() {
    let session = RtcSession {
        tenant_id: "100001".into(),
        rtc_session_id: "rtc_demo".into(),
        conversation_id: Some("c_demo".into()),
        rtc_mode: "voice".into(),
        initiator_id: "1".into(),
        initiator_kind: "user".into(),
        provider_plugin_id: Some("rtc-volcengine".into()),
        provider_session_id: Some("volcengine:rtc_demo".into()),
        access_endpoint: Some("wss://rtc.volcengine.local/session".into()),
        provider_region: Some("cn-beijing".into()),
        state: RtcSessionState::Started,
        signaling_stream_id: Some("st_demo".into()),
        artifact_message_id: None,
        started_at: "2026-04-05T10:00:00Z".into(),
        ended_at: None,
    };

    let value = serde_json::to_value(session).expect("rtc session should serialize");

    assert_eq!(value["rtcMode"], Value::String("voice".into()));
    assert_eq!(value["initiatorKind"], Value::String("user".into()));
    assert_eq!(value["state"], Value::String("started".into()));
}

#[test]
fn test_stream_and_rtc_session_identity_kind_fields_are_required() {
    let stream_source = include_str!("../src/stream.rs");
    let rtc_source = include_str!("../src/rtc.rs");

    assert!(
        stream_source.contains("pub owner_principal_id: String,")
            && stream_source.contains("pub owner_principal_kind: String,"),
        "stream sessions must persist an explicit owner principal id and kind"
    );
    assert!(
        rtc_source.contains("pub initiator_kind: String,")
            && !rtc_source.contains("pub use sdkwork_communication_rtc_service"),
        "IM-owned rtc sessions must persist an explicit initiator kind without re-exporting sdkwork-rtc call signaling contracts"
    );

    for forbidden_symbol in [
        "pub owner_principal_id: Option<String>",
        "pub owner_principal_kind: Option<String>",
        "pub initiator_kind: Option<String>",
    ] {
        assert!(
            !stream_source.contains(forbidden_symbol) && !rtc_source.contains(forbidden_symbol),
            "session identity kind fields must not be optional: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_rtc_signal_event_serializes_signal_transport_shape() {
    let signal = RtcSignalEvent {
        tenant_id: "100001".into(),
        rtc_session_id: "rtc_demo".into(),
        signal_seq: 1,
        conversation_id: Some("c_demo".into()),
        rtc_mode: "voice".into(),
        signal_type: "rtc.offer".into(),
        schema_ref: Some("webrtc.offer.v1".into()),
        payload: r#"{"sdp":"demo"}"#.into(),
        sender: RtcSignalSender {
            id: "1".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        signaling_stream_id: Some("st_demo".into()),
        occurred_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(signal).expect("rtc signal event should serialize");

    assert_eq!(value["rtcSessionId"], Value::String("rtc_demo".into()));
    assert_eq!(value["signalSeq"], Value::Number(1.into()));
    assert_eq!(value["signalType"], Value::String("rtc.offer".into()));
    assert_eq!(value["schemaRef"], Value::String("webrtc.offer.v1".into()));
    assert_eq!(value["sender"]["id"], Value::String("1".into()));
    assert_eq!(value["signalingStreamId"], Value::String("st_demo".into()));
}

#[test]
fn test_realtime_subscription_snapshot_serializes_shape() {
    let snapshot = RealtimeSubscriptionSnapshot {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        items: vec![RealtimeSubscription {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
            subscribed_at: "2026-04-05T10:10:00Z".into(),
        }],
        synced_at: "2026-04-05T10:10:00Z".into(),
    };

    let value = serde_json::to_value(snapshot).expect("realtime snapshot should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(
        value["items"][0]["scopeType"],
        Value::String("conversation".into())
    );
    assert_eq!(
        value["items"][0]["eventTypes"][0],
        Value::String("message.posted".into())
    );
}

#[test]
fn test_realtime_event_window_serializes_shape() {
    let window = RealtimeEventWindow {
        device_id: "d_pad".into(),
        items: vec![RealtimeEvent {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            realtime_seq: 1,
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_type: "message.posted".into(),
            delivery_class: "ephemeral".into(),
            payload: r#"{"messageId":"msg_c_demo_1"}"#.into(),
            occurred_at: "2026-04-05T10:10:01Z".into(),
        }],
        next_after_seq: Some(1),
        has_more: false,
        acked_through_seq: 0,
        trimmed_through_seq: 0,
    };

    let value = serde_json::to_value(window).expect("realtime event window should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(value["items"][0]["realtimeSeq"], Value::Number(1.into()));
    assert_eq!(
        value["items"][0]["eventType"],
        Value::String("message.posted".into())
    );
    assert_eq!(value["nextAfterSeq"], Value::Number(1.into()));
    assert_eq!(value["ackedThroughSeq"], Value::Number(0.into()));
    assert_eq!(value["trimmedThroughSeq"], Value::Number(0.into()));
}

#[test]
fn test_realtime_ack_state_serializes_checkpoint_shape() {
    let ack = RealtimeAckState {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        acked_through_seq: 12,
        trimmed_through_seq: 12,
        retained_event_count: 3,
        acked_at: "2026-04-05T10:10:02Z".into(),
    };

    let value = serde_json::to_value(ack).expect("realtime ack state should serialize");

    assert_eq!(value["deviceId"], Value::String("d_pad".into()));
    assert_eq!(value["ackedThroughSeq"], Value::Number(12.into()));
    assert_eq!(value["trimmedThroughSeq"], Value::Number(12.into()));
    assert_eq!(value["retainedEventCount"], Value::Number(3.into()));
}

#[test]
fn test_conversation_member_serializes_membership_shape() {
    let member = ConversationMember {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        member_id: "cm_demo".into(),
        principal_id: "1".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        state: MembershipState::Joined,
        invited_by: Some("u_admin".into()),
        joined_at: "2026-04-05T10:00:00Z".into(),
        removed_at: None,
        attributes: BTreeMap::from([("source".into(), "bootstrap".into())]),
    };

    let value = serde_json::to_value(member).expect("conversation member should serialize");

    assert_eq!(value["memberId"], Value::String("cm_demo".into()));
    assert_eq!(value["principalId"], Value::String("1".into()));
    assert_eq!(value["principalKind"], Value::String("user".into()));
    assert_eq!(value["role"], Value::String("owner".into()));
    assert_eq!(value["state"], Value::String("joined".into()));
    assert_eq!(value["invitedBy"], Value::String("u_admin".into()));
    assert_eq!(
        value["attributes"]["source"],
        Value::String("bootstrap".into())
    );
}

#[test]
fn test_conversation_read_cursor_serializes_cursor_shape() {
    let cursor = ConversationReadCursor {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        member_id: "cm_demo".into(),
        principal_id: "1".into(),
        principal_kind: "user".into(),
        read_seq: 12,
        last_read_message_id: Some("msg_c_demo_12".into()),
        updated_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(cursor).expect("read cursor should serialize");

    assert_eq!(value["memberId"], Value::String("cm_demo".into()));
    assert_eq!(value["principalId"], Value::String("1".into()));
    assert_eq!(value["principalKind"], Value::String("user".into()));
    assert_eq!(value["readSeq"], Value::Number(12.into()));
    assert_eq!(
        value["lastReadMessageId"],
        Value::String("msg_c_demo_12".into())
    );
    assert_eq!(
        value["updatedAt"],
        Value::String("2026-04-05T10:00:10Z".into())
    );
}

#[test]
fn test_conversation_inbox_entry_serializes_inbox_shape() {
    let entry = ConversationInboxEntry {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        member_id: "cm_demo".into(),
        conversation_id: "c_demo".into(),
        conversation_type: "group".into(),
        message_count: 12,
        last_message_id: Some("msg_c_demo_12".into()),
        last_message_seq: 12,
        last_sender_id: Some("u_other".into()),
        last_sender_kind: Some("user".into()),
        last_summary: Some("hello".into()),
        unread_count: 3,
        last_activity_at: "2026-04-05T10:00:10Z".into(),
        display_name: Some("Alice Project Lead".into()),
        avatar_url: Some("https://cdn.example.test/alice.png".into()),
        display_source: Some("contact_remark".into()),
        peer: Some(ConversationInboxPeerView {
            principal_kind: "user".into(),
            principal_id: "u_alice".into(),
            user_id: Some("u_alice".into()),
            chat_id: Some("alice-chat-id".into()),
            display_name: Some("Alice Chen".into()),
            avatar_url: Some("https://cdn.example.test/alice.png".into()),
            relationship_state: Some("active".into()),
        }),
        preferences: Some(ConversationInboxPreferencesView {
            is_pinned: true,
            is_muted: false,
            is_marked_unread: true,
            is_hidden: false,
        }),
        agent_handoff: Some(ConversationAgentHandoffView {
            status: "accepted".into(),
            source: ConversationActorView {
                id: "ag_source".into(),
                kind: "agent".into(),
            },
            target: ConversationActorView {
                id: "1".into(),
                kind: "user".into(),
            },
            handoff_session_id: "hs_demo".into(),
            handoff_reason: Some("manual_escalation".into()),
            accepted_at: Some("2026-04-05T10:00:09Z".into()),
            accepted_by: Some(ConversationActorView {
                id: "1".into(),
                kind: "user".into(),
            }),
            resolved_at: None,
            resolved_by: None,
            closed_at: None,
            closed_by: None,
        }),
    };

    let value = serde_json::to_value(entry).expect("inbox entry should serialize");

    assert_eq!(value["conversationId"], Value::String("c_demo".into()));
    assert_eq!(value["conversationType"], Value::String("group".into()));
    assert_eq!(value["messageCount"], Value::Number(12.into()));
    assert_eq!(
        value["lastMessageId"],
        Value::String("msg_c_demo_12".into())
    );
    assert_eq!(value["lastSenderId"], Value::String("u_other".into()));
    assert_eq!(value["unreadCount"], Value::Number(3.into()));
    assert_eq!(
        value["agentHandoff"]["status"],
        Value::String("accepted".into())
    );
    assert_eq!(
        value["agentHandoff"]["source"]["id"],
        Value::String("ag_source".into())
    );
    assert_eq!(
        value["lastActivityAt"],
        Value::String("2026-04-05T10:00:10Z".into())
    );
    assert_eq!(
        value["displayName"],
        Value::String("Alice Project Lead".into())
    );
    assert_eq!(
        value["avatarUrl"],
        Value::String("https://cdn.example.test/alice.png".into())
    );
    assert_eq!(
        value["displaySource"],
        Value::String("contact_remark".into())
    );
    assert_eq!(
        value["peer"]["principalId"],
        Value::String("u_alice".into())
    );
    assert_eq!(
        value["peer"]["displayName"],
        Value::String("Alice Chen".into())
    );
    assert_eq!(
        value["peer"]["relationshipState"],
        Value::String("active".into())
    );
    assert_eq!(value["preferences"]["isPinned"], Value::Bool(true));
    assert_eq!(value["preferences"]["isMarkedUnread"], Value::Bool(true));
}

#[test]
fn test_client_route_sync_feed_entry_serializes_sync_shape() {
    let entry = ClientRouteSyncFeedEntry {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_demo".into(),
        sync_seq: 2,
        origin_event_id: "evt_demo".into(),
        origin_event_type: "message.posted".into(),
        conversation_id: Some("c_demo".into()),
        message_id: Some("msg_c_demo_2".into()),
        message_seq: Some(2),
        member_id: None,
        read_seq: None,
        last_read_message_id: None,
        actor_id: Some("u_other".into()),
        actor_kind: Some("user".into()),
        actor_device_id: Some("d_other".into()),
        summary: Some("hello".into()),
        payload_schema: Some("message.posted.v1".into()),
        payload: Some(r#"{"messageId":"msg_c_demo_2"}"#.into()),
        occurred_at: "2026-04-05T10:00:10Z".into(),
    };

    let value = serde_json::to_value(entry).expect("client route sync feed entry should serialize");

    assert_eq!(value["principalId"], Value::String("1".into()));
    assert_eq!(value["deviceId"], Value::String("d_demo".into()));
    assert_eq!(value["syncSeq"], Value::Number(2.into()));
    assert_eq!(
        value["originEventType"],
        Value::String("message.posted".into())
    );
    assert_eq!(value["messageId"], Value::String("msg_c_demo_2".into()));
    assert_eq!(value["actorKind"], Value::String("user".into()));
    assert_eq!(value["actorDeviceId"], Value::String("d_other".into()));
    assert_eq!(value["summary"], Value::String("hello".into()));
    assert_eq!(
        value["payloadSchema"],
        Value::String("message.posted.v1".into())
    );
    assert_eq!(
        value["payload"],
        Value::String(r#"{"messageId":"msg_c_demo_2"}"#.into())
    );
}

#[test]
fn test_presence_resume_view_serializes_presence_snapshot_shape() {
    let view = PresenceResumeView {
        tenant_id: "100001".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: "d_demo".into(),
        resume_required: true,
        resume_from_sync_seq: 3,
        latest_sync_seq: 5,
        resumed_at: "2026-04-05T10:00:20Z".into(),
        presence: PresenceSnapshotView {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            current_device_id: Some("d_demo".into()),
            devices: vec![
                PresenceClientView {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_demo".into(),
                    platform: None,
                    session_id: Some("s_demo".into()),
                    status: PresenceStatus::Online,
                    last_sync_seq: 5,
                    last_resume_at: Some("2026-04-05T10:00:20Z".into()),
                    last_seen_at: Some("2026-04-05T10:00:20Z".into()),
                },
                PresenceClientView {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: "d_pad".into(),
                    platform: None,
                    session_id: Some("s_pad".into()),
                    status: PresenceStatus::Offline,
                    last_sync_seq: 2,
                    last_resume_at: Some("2026-04-05T09:50:00Z".into()),
                    last_seen_at: Some("2026-04-05T09:51:00Z".into()),
                },
            ],
        },
    };

    let value = serde_json::to_value(view).expect("device session resume view should serialize");

    assert_eq!(value["deviceId"], Value::String("d_demo".into()));
    assert_eq!(value["resumeRequired"], Value::Bool(true));
    assert_eq!(value["resumeFromSyncSeq"], Value::Number(3.into()));
    assert_eq!(value["latestSyncSeq"], Value::Number(5.into()));
    assert_eq!(
        value["presence"]["currentDeviceId"],
        Value::String("d_demo".into())
    );
    assert_eq!(
        value["presence"]["devices"][0]["status"],
        Value::String("online".into())
    );
    assert_eq!(
        value["presence"]["devices"][1]["status"],
        Value::String("offline".into())
    );
}

#[test]
fn test_message_mutation_payloads_serialize_stable_shape() {
    let edited = MessageEdited {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        message_id: "msg_c_demo_1".into(),
        message_seq: 1,
        body: MessageBody {
            summary: Some("edited".into()),
            parts: vec![ContentPart::text("edited")],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        editor: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        edited_at: "2026-04-05T10:00:30Z".into(),
    };
    let recalled = MessageRecalled {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        message_id: "msg_c_demo_1".into(),
        message_seq: 1,
        recalled_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: Some("cm_demo".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        recalled_at: "2026-04-05T10:00:40Z".into(),
    };

    let edited_value = serde_json::to_value(edited).expect("edited payload should serialize");
    let recalled_value = serde_json::to_value(recalled).expect("recalled payload should serialize");

    assert_eq!(
        edited_value["messageId"],
        Value::String("msg_c_demo_1".into())
    );
    assert_eq!(edited_value["messageSeq"], Value::Number(1.into()));
    assert_eq!(
        edited_value["editor"]["deviceId"],
        Value::String("d_demo".into())
    );
    assert_eq!(
        edited_value["body"]["summary"],
        Value::String("edited".into())
    );

    assert_eq!(
        recalled_value["recalledBy"]["sessionId"],
        Value::String("s_demo".into())
    );
    assert_eq!(
        recalled_value["recalledAt"],
        Value::String("2026-04-05T10:00:40Z".into())
    );
}

#[test]
fn test_message_body_derives_summary_for_rich_structured_message_schemas() {
    let cases = [
        (
            SDKWORK_IM_MESSAGE_SCHEMA_LOCATION,
            json!({
                "name": "The Bund",
                "latitude": 31.2400,
                "longitude": 121.4900
            }),
            "Location: The Bund",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_LINK,
            json!({
                "title": "Realtime architecture",
                "url": "https://example.com/realtime"
            }),
            "Link: Realtime architecture",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_CARD,
            json!({
                "title": "Support escalation"
            }),
            "Card: Support escalation",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_MUSIC,
            json!({
                "title": "Ambient Focus",
                "url": "https://example.com/music"
            }),
            "Music: Ambient Focus",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_CONTACT,
            json!({
                "displayName": "Alice"
            }),
            "Contact: Alice",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_STICKER,
            json!({
                "stickerId": "sticker_wave"
            }),
            "Sticker",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_VOICE,
            json!({
                "durationSeconds": 7
            }),
            "Voice message",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_AGENT,
            json!({
                "agentId": "agent_sales_router",
                "agentName": "Sales Router"
            }),
            "Agent: Sales Router",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_AI_IMAGE,
            json!({
                "prompt": "A skyline at sunset"
            }),
            "AI image generated",
        ),
        (
            SDKWORK_IM_MESSAGE_SCHEMA_AI_VIDEO,
            json!({
                "prompt": "Launch teaser"
            }),
            "AI video generated",
        ),
    ];

    for (schema_ref, payload, expected) in cases {
        let body = MessageBody {
            summary: None,
            parts: vec![ContentPart::Data(DataPart {
                schema_ref: schema_ref.into(),
                encoding: "application/json".into(),
                payload: payload.to_string(),
            })],
            render_hints: BTreeMap::new(),
            reply_to: None,
        };

        assert_eq!(body.derived_summary().as_deref(), Some(expected));
    }

    let custom_body = MessageBody {
        summary: None,
        parts: vec![ContentPart::Data(DataPart {
            schema_ref: format!("{SDKWORK_IM_CUSTOM_MESSAGE_SCHEMA_PREFIX}workflow.approval"),
            encoding: "application/json".into(),
            payload: json!({
                "approvalId": "approval_demo"
            })
            .to_string(),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };

    assert_eq!(
        custom_body.derived_summary().as_deref(),
        Some("Custom: workflow.approval")
    );
}

#[test]
fn test_message_body_prefers_structured_semantics_and_media_signal_fallbacks() {
    let rich_body = MessageBody {
        summary: None,
        parts: vec![
            ContentPart::text("caption that should not become the summary"),
            ContentPart::Data(DataPart {
                schema_ref: SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
                encoding: "application/json".into(),
                payload: json!({
                    "name": "West Lake",
                    "latitude": 30.2528,
                    "longitude": 120.1551
                })
                .to_string(),
            }),
        ],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    assert_eq!(
        rich_body.derived_summary().as_deref(),
        Some("Location: West Lake")
    );

    let media_body = MessageBody {
        summary: None,
        parts: vec![ContentPart::media(MediaPart {
            drive: DriveReference {
                drive_uri: "drive://spaces/space_app_upload_demo/nodes/node_image_demo".into(),
                space_id: "space_app_upload_demo".into(),
                node_id: "node_image_demo".into(),
                node_version: None,
            },
            media_role: Some("attachment".into()),
            resource: MediaResource {
                id: Some("node_image_demo".into()),
                kind: MediaKind::Image,
                source: MediaSource::Drive,
                url: None,
                public_url: None,
                uri: Some("drive://spaces/space_app_upload_demo/nodes/node_image_demo".into()),
                object_blob_id: None,
                file_name: Some("demo.png".into()),
                mime_type: Some("image/png".into()),
                size_bytes: None,
                checksum: None,
                width: None,
                height: None,
                duration_seconds: None,
                alt_text: None,
                title: None,
                poster: None,
                thumbnails: None,
                variants: None,
                access: None,
                ai: None,
                metadata: None,
            },
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    assert_eq!(media_body.derived_summary().as_deref(), Some("Image"));

    let signal_body = MessageBody {
        summary: None,
        parts: vec![ContentPart::Signal(im_domain_core::message::SignalPart {
            signal_type: "rtc.offer".into(),
            schema_ref: Some("webrtc.offer.v1".into()),
            payload: json!({
                "sdp": "demo"
            })
            .to_string(),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    assert_eq!(signal_body.derived_summary().as_deref(), Some("rtc.offer"));
}

#[test]
fn test_message_body_with_derived_summary_preserves_explicit_summary_and_normalizes_blank() {
    let explicit = MessageBody {
        summary: Some("Pinned place".into()),
        parts: vec![ContentPart::Data(DataPart {
            schema_ref: SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
            encoding: "application/json".into(),
            payload: json!({
                "name": "The Bund"
            })
            .to_string(),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    }
    .with_derived_summary();
    assert_eq!(explicit.summary.as_deref(), Some("Pinned place"));

    let normalized = MessageBody {
        summary: Some("   ".into()),
        parts: vec![ContentPart::Data(DataPart {
            schema_ref: SDKWORK_IM_MESSAGE_SCHEMA_CARD.into(),
            encoding: "application/json".into(),
            payload: json!({
                "title": "Escalation runbook"
            })
            .to_string(),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    }
    .with_derived_summary();
    assert_eq!(
        normalized.summary.as_deref(),
        Some("Card: Escalation runbook")
    );
}
