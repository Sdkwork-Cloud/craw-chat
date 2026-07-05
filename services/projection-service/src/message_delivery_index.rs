use crate::scope::{client_route_feed_scope_key, scope_key};
use crate::{lock_projection_mutex, TimelineProjectionService};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MessageDeliveryDeviceOffer {
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
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

impl TimelineProjectionService {
    pub(crate) fn record_message_delivery_offer(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        sync_seq: u64,
    ) {
        let key = message_delivery_index_key(
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        );
        let mut offers =
            lock_projection_mutex(&self.message_delivery_offers, "message delivery offer store");
        let scope_offers = offers.entry(key).or_default();
        if let Some(existing) = scope_offers.iter_mut().find(|offer| {
            offer.principal_id == principal_id
                && offer.principal_kind == principal_kind
                && offer.device_id == device_id
        }) {
            existing.sync_seq = existing.sync_seq.max(sync_seq);
            return;
        }
        scope_offers.push(MessageDeliveryDeviceOffer {
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
            device_id: device_id.into(),
            sync_seq,
        });
    }

    pub(crate) fn message_delivery_offers_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Vec<MessageDeliveryDeviceOffer> {
        let key = message_delivery_index_key(
            tenant_id,
            organization_id,
            conversation_id,
            message_id,
        );
        lock_projection_mutex(&self.message_delivery_offers, "message delivery offer store")
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
        lock_projection_mutex(
            &self.client_route_sync_checkpoints,
            "client route sync checkpoint store",
        )
        .get(&scope)
        .map(|checkpoint| checkpoint.acked_through_sync_seq)
        .unwrap_or_default()
    }
}
