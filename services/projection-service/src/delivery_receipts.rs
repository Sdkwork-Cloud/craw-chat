use std::collections::BTreeSet;

use crate::model::{MessageDeliveryReceiptDeviceView, MessageDeliveryReceiptSummaryView};
use crate::{TimelineProjectionService, lock_projection_mutex};

pub(crate) const DELIVERY_RECEIPT_MAX_DEVICES: usize = 50;

impl TimelineProjectionService {
    pub(crate) fn delivery_receipt_summary_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        exclude_principal: Option<(&str, &str)>,
    ) -> MessageDeliveryReceiptSummaryView {
        let scope = crate::scope::scope_key(tenant_id, organization_id, conversation_id);
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
        let offers = self.message_delivery_offers_for_message(
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        );

        let active_member_count = members.len() as u64;
        let mut offered_members = BTreeSet::new();
        let mut delivered_devices = Vec::new();
        let mut delivered_members = BTreeSet::new();

        for offer in offers {
            let Some(member) = members.iter().find(|member| {
                member.principal_id == offer.principal_id
                    && member.principal_kind == offer.principal_kind
            }) else {
                continue;
            };
            if should_exclude_recipient(member, exclude_principal) {
                continue;
            }
            offered_members.insert(member.member_id.clone());
            let acked_through = self.client_route_sync_acked_through_for_device(
                tenant_id,
                organization_id,
                offer.principal_id.as_str(),
                offer.principal_kind.as_str(),
                offer.device_id.as_str(),
            );
            if acked_through >= offer.sync_seq {
                delivered_members.insert(member.member_id.clone());
                if delivered_devices.len() < DELIVERY_RECEIPT_MAX_DEVICES {
                    delivered_devices.push(MessageDeliveryReceiptDeviceView {
                        principal_id: member.principal_id.clone(),
                        principal_kind: member.principal_kind.clone(),
                        member_id: member.member_id.clone(),
                        device_id: offer.device_id.clone(),
                        sync_seq: offer.sync_seq,
                    });
                }
            }
        }

        delivered_devices.sort_by(|left, right| {
            right
                .sync_seq
                .cmp(&left.sync_seq)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });

        MessageDeliveryReceiptSummaryView {
            active_member_count,
            offered_count: offered_members.len() as u64,
            delivered_count: delivered_members.len() as u64,
            delivered_devices,
        }
    }
}

fn should_exclude_recipient(
    member: &im_domain_core::conversation::ConversationMember,
    exclude_principal: Option<(&str, &str)>,
) -> bool {
    let Some((principal_id, principal_kind)) = exclude_principal else {
        return false;
    };
    member.principal_id == principal_id && member.principal_kind == principal_kind
}

#[cfg(test)]
mod tests {
    use im_domain_events::CommitEnvelope;

    use super::*;

    fn member_joined(
        event_id: &str,
        member_id: &str,
        principal_id: &str,
        ordering_seq: u64,
    ) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "100001",
            "conversation.member_joined",
            "conversation",
            "c_dr",
            ordering_seq,
        )
        .with_payload(
            "conversation.member.v1",
            &format!(
                r#"{{
            "tenantId":"100001",
            "conversationId":"c_dr",
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

    fn message_posted(
        event_id: &str,
        message_id: &str,
        message_seq: u64,
        sender_id: &str,
        ordering_seq: u64,
    ) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "100001",
            "message.posted",
            "conversation",
            "c_dr",
            ordering_seq,
        )
        .with_payload(
            "message.posted.v1",
            &format!(
                r#"{{
            "tenantId":"100001",
            "conversationId":"c_dr",
            "messageId":"{message_id}",
            "messageSeq":{message_seq},
            "sender":{{
                "id":"{sender_id}",
                "kind":"user",
                "memberId":"cm_{sender_id}",
                "deviceId":"d_{sender_id}",
                "sessionId":"s_{sender_id}",
                "metadata":{{}}
            }},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_{message_id}",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{{"summary":"hello","parts":[{{"kind":"text","text":"hello"}}],"renderHints":{{}}}},
            "metadata":{{}},
            "attributes":{{}},
            "occurredAt":"2026-07-05T00:00:00Z",
            "committedAt":"2026-07-05T00:00:00Z"
        }}"#
            ),
        )
    }

    #[test]
    fn delivery_receipt_summary_counts_acked_device_offers() {
        let service = TimelineProjectionService::default();
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_conv",
                    "100001",
                    "conversation.created",
                    "conversation",
                    "c_dr",
                    0,
                )
                .with_payload(
                    "conversation.created.v1",
                    r#"{"conversationId":"c_dr","conversationType":"group","scenario":"standard","title":"Delivery receipt test","createdAt":"2026-07-05T00:00:00Z"}"#,
                ),
            )
            .expect("conversation created");
        service
            .apply(&member_joined("evt_join_a", "cm_a", "user_a", 1))
            .expect("member joined");
        service
            .apply(&member_joined("evt_join_b", "cm_b", "user_b", 2))
            .expect("member joined");
        service.register_client_route("100001", "default", "user_b", "d_phone");
        service.register_client_route("100001", "default", "user_b", "d_pad");
        service
            .apply(&message_posted("evt_msg", "m_dr_1", 1, "user_a", 3))
            .expect("message posted");

        let before_ack = service.delivery_receipt_summary_for_message(
            "100001",
            "default",
            "c_dr",
            "m_dr_1",
            Some(("user_a", "user")),
        );
        assert_eq!(before_ack.active_member_count, 2);
        assert_eq!(before_ack.offered_count, 1);
        assert_eq!(before_ack.delivered_count, 0);

        let sync_seq =
            service.latest_client_route_sync_seq("100001", "default", "user_b", "d_phone");
        assert!(sync_seq > 0);
        service.ack_client_route_sync_feed_for_principal_kind(
            "100001", "default", "user_b", "user", "d_phone", sync_seq,
        );

        let after_ack = service.delivery_receipt_summary_for_message(
            "100001",
            "default",
            "c_dr",
            "m_dr_1",
            Some(("user_a", "user")),
        );
        assert_eq!(after_ack.delivered_count, 1);
        assert_eq!(after_ack.delivered_devices.len(), 1);
        assert_eq!(after_ack.delivered_devices[0].device_id, "d_phone");
    }

    #[test]
    fn delivery_receipt_summary_treats_any_acked_device_as_member_delivered() {
        let service = TimelineProjectionService::default();
        service
            .apply(
                &CommitEnvelope::minimal(
                    "evt_conv_md",
                    "100001",
                    "conversation.created",
                    "conversation",
                    "c_dr",
                    0,
                )
                .with_payload(
                    "conversation.created.v1",
                    r#"{"conversationId":"c_dr","conversationType":"group","scenario":"standard","title":"Multi device delivery","createdAt":"2026-07-05T00:00:00Z"}"#,
                ),
            )
            .expect("conversation created");
        service
            .apply(&member_joined("evt_join_a", "cm_a", "user_a", 1))
            .expect("member joined");
        service
            .apply(&member_joined("evt_join_b", "cm_b", "user_b", 2))
            .expect("member joined");
        service.register_client_route("100001", "default", "user_b", "d_phone");
        service.register_client_route("100001", "default", "user_b", "d_pad");
        service
            .apply(&message_posted("evt_msg_md", "m_dr_2", 1, "user_a", 3))
            .expect("message posted");

        let phone_seq =
            service.latest_client_route_sync_seq("100001", "default", "user_b", "d_phone");
        let pad_seq = service.latest_client_route_sync_seq("100001", "default", "user_b", "d_pad");
        assert!(phone_seq > 0);
        assert!(pad_seq > 0);
        service.ack_client_route_sync_feed_for_principal_kind(
            "100001", "default", "user_b", "user", "d_pad", pad_seq,
        );

        let summary = service.delivery_receipt_summary_for_message(
            "100001",
            "default",
            "c_dr",
            "m_dr_2",
            Some(("user_a", "user")),
        );
        assert_eq!(summary.delivered_count, 1);
        assert_eq!(summary.delivered_devices.len(), 1);
        assert_eq!(summary.delivered_devices[0].device_id, "d_pad");
    }
}
