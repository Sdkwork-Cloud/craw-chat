//! White-box unit tests for received message index.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "received_message_index_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::collections::BTreeMap;

use im_domain_core::message::{MessageBody, MessageType, Sender};

use crate::TimelineViewEntry;

use super::ReceivedMessageIndex;

fn timeline_entry(message_seq: u64, sender_id: &str) -> TimelineViewEntry {
    TimelineViewEntry {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        message_id: format!("m_{message_seq}"),
        message_seq,
        summary: Some(format!("message {message_seq}")),
        sender: Sender {
            id: sender_id.into(),
            kind: "user".into(),
            member_id: Some(format!("cm_{sender_id}")),
            device_id: None,
            session_id: None,
            metadata: BTreeMap::new(),
        },
        body: MessageBody {
            summary: Some(format!("message {message_seq}")),
            parts: Vec::new(),
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        message_type: MessageType::Standard,
        delivery_mode: "discrete".into(),
        client_msg_id: None,
        stream_session_id: None,
        rtc_session_id: None,
        occurred_at: "2026-04-05T10:00:01Z".into(),
        committed_at: Some("2026-04-05T10:00:01Z".into()),
        retention_until: None,
    }
}

#[test]
fn test_received_message_index_counts_only_messages_received_after_read_seq() {
    let mut index = ReceivedMessageIndex::default();
    index.append_message("scope", 2, "1", "user");
    index.append_message("scope", 1, "1017", "user");
    index.append_message("scope", 2, "1", "user");

    assert_eq!(index.unread_count_after("scope", "1", "user", 0), 1);
    assert_eq!(index.unread_count_after("scope", "1", "user", 1), 0);
    assert_eq!(index.unread_count_after("scope", "1017", "user", 0), 1);
    assert_eq!(index.unread_count_after("scope", "1017", "user", 1), 1);
    assert_eq!(index.unread_count_after("scope", "1017", "user", 2), 0);
}

#[test]
fn test_received_message_index_rebuilds_conversation_from_timeline_and_members() {
    let mut index = ReceivedMessageIndex::default();
    let timeline = BTreeMap::from([
        (1, timeline_entry(1, "1")),
        (2, timeline_entry(2, "1017")),
        (3, timeline_entry(3, "1")),
    ]);

    index.rebuild_conversation("scope", &timeline);

    assert_eq!(index.unread_count_after("scope", "1", "user", 0), 1);
    assert_eq!(index.unread_count_after("scope", "1017", "user", 0), 2);
    assert_eq!(index.unread_count_after("scope", "1017", "user", 1), 1);
}
