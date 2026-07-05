use im_domain_core::conversation::{best_read_cursor_for_member_at_seq, ConversationReadCursor};

use crate::model::{MessageReadReceiptReaderView, MessageReadReceiptSummaryView};
use crate::scope::scope_key;
use crate::{lock_projection_mutex, TimelineProjectionService};

pub(crate) const READ_RECEIPT_MAX_READERS: usize = 50;

impl TimelineProjectionService {
    pub(crate) fn read_receipt_summary_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_seq: u64,
        exclude_principal: Option<(&str, &str)>,
    ) -> MessageReadReceiptSummaryView {
        let scope = scope_key(tenant_id, organization_id, conversation_id);
        let members = lock_projection_mutex(&self.members, "member store")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.is_active())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let cursors = lock_projection_mutex(&self.read_cursors, "cursor store")
            .get(scope.as_str())
            .cloned()
            .unwrap_or_default();

        let active_member_count = members.len() as u64;
        let mut readers = Vec::new();
        for member in members {
            if should_exclude_reader(&member, exclude_principal) {
                continue;
            }
            let Some(cursor) =
                best_read_cursor_for_member_at_seq(cursors.values(), member.member_id.as_str(), message_seq)
            else {
                continue;
            };
            readers.push(reader_view_from_cursor(&member, cursor));
            if readers.len() >= READ_RECEIPT_MAX_READERS {
                break;
            }
        }
        readers.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
        });

        MessageReadReceiptSummaryView {
            active_member_count,
            read_count: readers.len() as u64,
            readers,
        }
    }

    pub(crate) fn timeline_message_sender(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Option<(String, String)> {
        lock_projection_mutex(&self.entries, "projection store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .and_then(|entries| {
                entries.values().find_map(|entry| {
                    if entry.message_id == message_id {
                        Some((entry.sender.id.clone(), entry.sender.kind.clone()))
                    } else {
                        None
                    }
                })
            })
    }
}

fn should_exclude_reader(
    member: &im_domain_core::conversation::ConversationMember,
    exclude_principal: Option<(&str, &str)>,
) -> bool {
    let Some((principal_id, principal_kind)) = exclude_principal else {
        return false;
    };
    member.principal_id == principal_id && member.principal_kind == principal_kind
}

fn reader_view_from_cursor(
    member: &im_domain_core::conversation::ConversationMember,
    cursor: &ConversationReadCursor,
) -> MessageReadReceiptReaderView {
    MessageReadReceiptReaderView {
        principal_id: member.principal_id.clone(),
        principal_kind: member.principal_kind.clone(),
        member_id: member.member_id.clone(),
        read_seq: cursor.read_seq,
        updated_at: cursor.updated_at.clone(),
    }
}

#[cfg(test)]
mod tests {
    use im_domain_events::CommitEnvelope;

    use super::*;

    fn member_joined(event_id: &str, member_id: &str, principal_id: &str, ordering_seq: u64) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "100001",
            "conversation.member_joined",
            "conversation",
            "c_rr",
            ordering_seq,
        )
        .with_payload(
            "conversation.member.v1",
            &format!(
                r#"{{
            "tenantId":"100001",
            "conversationId":"c_rr",
            "memberId":"{member_id}",
            "principalId":"{principal_id}",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-07-05T00:00:00Z",
            "removedAt":null,
            "attributes":{{}}
        }}"#
            ),
        )
    }

    fn read_cursor_updated(
        member_id: &str,
        principal_id: &str,
        read_seq: u64,
        ordering_seq: u64,
        device_id: Option<&str>,
    ) -> CommitEnvelope {
        let device_json = device_id
            .map(|value| format!(r#","deviceId":"{value}""#))
            .unwrap_or_default();
        CommitEnvelope::minimal(
            &format!("evt_cursor_{member_id}_{ordering_seq}"),
            "100001",
            "conversation.read_cursor_updated",
            "conversation",
            "c_rr",
            ordering_seq,
        )
        .with_payload(
            "conversation.read_cursor.v1",
            &format!(
                r#"{{
            "tenantId":"100001",
            "conversationId":"c_rr",
            "memberId":"{member_id}",
            "principalId":"{principal_id}",
            "principalKind":"user",
            "readSeq":{read_seq},
            "lastReadMessageId":"m_rr_{read_seq}",
            "updatedAt":"2026-07-05T00:00:01Z"{device_json}
        }}"#
            ),
        )
    }

    #[test]
    fn read_receipt_summary_counts_members_at_or_beyond_message_seq() {
        let service = TimelineProjectionService::default();
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_conv",
                    "100001",
                    "conversation.created",
                    "conversation",
                    "c_rr",
                    0,
                )
                .with_payload(
                    "conversation.created.v1",
                    r#"{"conversationId":"c_rr","conversationType":"group","scenario":"standard","title":"Read receipt test","createdAt":"2026-07-05T00:00:00Z"}"#,
                ),
            )
            .expect("conversation created");
        service
            .apply(&member_joined("evt_join_a", "cm_a", "user_a", 1))
            .expect("member joined");
        service
            .apply(&member_joined("evt_join_b", "cm_b", "user_b", 2))
            .expect("member joined");
        service
            .apply(&read_cursor_updated("cm_a", "user_a", 2, 3, None))
            .expect("cursor updated");

        let excluding_sender = service.read_receipt_summary_for_message(
            "100001",
            "default",
            "c_rr",
            2,
            Some(("user_a", "user")),
        );
        assert_eq!(excluding_sender.active_member_count, 2);
        assert_eq!(excluding_sender.read_count, 0);

        let including_sender = service.read_receipt_summary_for_message(
            "100001",
            "default",
            "c_rr",
            2,
            None,
        );
        assert_eq!(including_sender.read_count, 1);
        assert_eq!(including_sender.readers[0].principal_id, "user_a");
    }

    #[test]
    fn read_receipt_summary_uses_max_read_seq_across_member_devices() {
        let service = TimelineProjectionService::default();
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_conv_md",
                    "100001",
                    "conversation.created",
                    "conversation",
                    "c_rr",
                    0,
                )
                .with_payload(
                    "conversation.created.v1",
                    r#"{"conversationId":"c_rr","conversationType":"group","scenario":"standard","title":"Multi device read receipt","createdAt":"2026-07-05T00:00:00Z"}"#,
                ),
            )
            .expect("conversation created");
        service
            .apply(&member_joined("evt_join_a_md", "cm_a", "user_a", 10))
            .expect("member joined");
        service
            .apply(&member_joined("evt_join_b_md", "cm_b", "user_b", 11))
            .expect("member joined");
        service
            .apply(&read_cursor_updated("cm_a", "user_a", 1, 12, Some("phone")))
            .expect("phone cursor updated");
        service
            .apply(&read_cursor_updated("cm_a", "user_a", 3, 13, Some("desktop")))
            .expect("desktop cursor updated");

        let summary = service.read_receipt_summary_for_message(
            "100001",
            "default",
            "c_rr",
            3,
            Some(("user_b", "user")),
        );
        assert_eq!(summary.read_count, 1);
        assert_eq!(summary.readers[0].read_seq, 3);
    }
}
