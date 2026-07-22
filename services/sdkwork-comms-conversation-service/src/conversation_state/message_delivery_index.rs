use crate::conversation_state::scope::{client_route_feed_scope_key, scope_key};
use crate::conversation_state::{ConversationStateService, lock_conversation_state_mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MessageDeliveryDeviceOffer {
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
    pub sync_seq: u64,
}

pub(crate) struct MessageDeliveryOfferCommand<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub conversation_id: &'a str,
    pub message_id: &'a str,
    pub principal_id: &'a str,
    pub principal_kind: &'a str,
    pub device_id: &'a str,
    pub sync_seq: u64,
}

fn message_delivery_index_key(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    message_id: &str,
) -> String {
    format!(
        "{}:{}",
        scope_key(tenant_id, organization_id, conversation_id),
        message_id
    )
}

impl ConversationStateService {
    pub(crate) fn record_message_delivery_offer(&self, command: MessageDeliveryOfferCommand<'_>) {
        let key = message_delivery_index_key(
            command.tenant_id,
            command.organization_id,
            command.conversation_id,
            command.message_id,
        );
        let mut offers = lock_conversation_state_mutex(
            &self.message_delivery_offers,
            "message delivery offer store",
        );
        let scope_offers = offers.entry(key).or_default();
        if let Some(existing) = scope_offers.iter_mut().find(|offer| {
            offer.principal_id == command.principal_id
                && offer.principal_kind == command.principal_kind
                && offer.device_id == command.device_id
        }) {
            existing.sync_seq = existing.sync_seq.max(command.sync_seq);
            return;
        }
        scope_offers.push(MessageDeliveryDeviceOffer {
            principal_id: command.principal_id.into(),
            principal_kind: command.principal_kind.into(),
            device_id: command.device_id.into(),
            sync_seq: command.sync_seq,
        });
    }

    pub(crate) fn message_delivery_offers_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Vec<MessageDeliveryDeviceOffer> {
        let key =
            message_delivery_index_key(tenant_id, organization_id, conversation_id, message_id);
        lock_conversation_state_mutex(
            &self.message_delivery_offers,
            "message delivery offer store",
        )
        .get(key.as_str())
        .cloned()
        .unwrap_or_default()
    }

    pub(crate) fn client_route_sync_acked_through_for_device(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> u64 {
        let scope = client_route_feed_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        lock_conversation_state_mutex(
            &self.client_route_sync_checkpoints,
            "client route sync checkpoint store",
        )
        .get(&scope)
        .map(|checkpoint| checkpoint.acked_through_sync_seq)
        .unwrap_or_default()
    }
}
