use std::collections::{BTreeMap, HashMap};

use im_domain_core::conversation::principal_member_key;

use crate::TimelineViewEntry;

#[derive(Default)]
pub(crate) struct ReceivedMessageIndex {
    by_conversation: HashMap<String, ConversationReceivedMessageIndex>,
}

#[derive(Default)]
struct ConversationReceivedMessageIndex {
    message_seqs: Vec<u64>,
    sent_message_seqs_by_principal: HashMap<String, Vec<u64>>,
}

impl ReceivedMessageIndex {
    pub(crate) fn clear(&mut self) {
        self.by_conversation.clear();
    }

    pub(crate) fn append_message(
        &mut self,
        scope: &str,
        message_seq: u64,
        sender_id: &str,
        sender_kind: &str,
    ) {
        let conversation = self.by_conversation.entry(scope.to_owned()).or_default();
        insert_sorted_unique(&mut conversation.message_seqs, message_seq);
        insert_sorted_unique(
            conversation
                .sent_message_seqs_by_principal
                .entry(principal_member_key(sender_id, sender_kind))
                .or_default(),
            message_seq,
        );
    }

    pub(crate) fn rebuild_conversation(
        &mut self,
        scope: &str,
        timeline: &BTreeMap<u64, TimelineViewEntry>,
    ) {
        self.remove_conversation(scope);
        for message in timeline.values() {
            self.append_message(
                scope,
                message.message_seq,
                message.sender.id.as_str(),
                message.sender.kind.as_str(),
            );
        }
    }

    pub(crate) fn remove_conversation(&mut self, scope: &str) {
        self.by_conversation.remove(scope);
    }

    pub(crate) fn unread_count_after(
        &self,
        scope: &str,
        principal_id: &str,
        principal_kind: &str,
        read_seq: u64,
    ) -> u64 {
        let Some(conversation) = self.by_conversation.get(scope) else {
            return 0;
        };
        let sent_message_seqs = conversation
            .sent_message_seqs_by_principal
            .get(principal_member_key(principal_id, principal_kind).as_str())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let message_count_after_read = count_after(conversation.message_seqs.as_slice(), read_seq);
        let self_sent_count_after_read = count_after(sent_message_seqs, read_seq);

        message_count_after_read.saturating_sub(self_sent_count_after_read) as u64
    }
}

fn insert_sorted_unique(values: &mut Vec<u64>, value: u64) {
    match values.binary_search(&value) {
        Ok(_) => {}
        Err(index) => values.insert(index, value),
    }
}

fn count_after(values: &[u64], read_seq: u64) -> usize {
    values
        .len()
        .saturating_sub(values.partition_point(|seq| *seq <= read_seq))
}

#[cfg(test)]
mod tests {
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
}
