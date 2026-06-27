use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

use im_domain_core::conversation::ClientRouteSyncFeedEntry;
use im_platform_contracts::normalize_realtime_organization_id;

use crate::model::{NotificationRecipientView, RealtimeFanoutTarget, RegisteredClientRouteView};
use crate::scope::{
    client_route_feed_scope_key, client_route_principal_scope_key, registered_client_route_at,
    scope_key, ClientRoutePrincipalScopeKey,
};
use crate::{TimelineProjectionService, lock_projection_mutex};

#[derive(Clone, Debug)]
pub(crate) struct ClientRouteSyncEntryDraft {
    pub(crate) tenant_id: String,
    pub(crate) organization_id: String,
    pub(crate) origin_event_id: String,
    pub(crate) origin_event_type: String,
    pub(crate) conversation_id: Option<String>,
    pub(crate) message_id: Option<String>,
    pub(crate) message_seq: Option<u64>,
    pub(crate) member_id: Option<String>,
    pub(crate) read_seq: Option<u64>,
    pub(crate) last_read_message_id: Option<String>,
    pub(crate) actor_id: Option<String>,
    pub(crate) actor_kind: Option<String>,
    pub(crate) actor_device_id: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) payload_schema: Option<String>,
    pub(crate) payload: Option<String>,
    pub(crate) occurred_at: String,
}

impl ClientRouteSyncEntryDraft {
    pub(crate) fn build_for_target(
        &self,
        target: &RealtimeFanoutTarget,
        sync_seq: u64,
    ) -> ClientRouteSyncFeedEntry {
        ClientRouteSyncFeedEntry {
            tenant_id: self.tenant_id.clone(),
            principal_id: target.principal_id.clone(),
            device_id: target.device_id.clone(),
            sync_seq,
            origin_event_id: self.origin_event_id.clone(),
            origin_event_type: self.origin_event_type.clone(),
            conversation_id: self.conversation_id.clone(),
            message_id: self.message_id.clone(),
            message_seq: self.message_seq,
            member_id: self.member_id.clone(),
            read_seq: self.read_seq,
            last_read_message_id: self.last_read_message_id.clone(),
            actor_id: self.actor_id.clone(),
            actor_kind: self.actor_kind.clone(),
            actor_device_id: self.actor_device_id.clone(),
            summary: self.summary.clone(),
            payload_schema: self.payload_schema.clone(),
            payload: self.payload.clone(),
            occurred_at: self.occurred_at.clone(),
        }
    }
}

pub(crate) fn register_client_route_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> RegisteredClientRouteView {
    register_client_route_with_principal_kind(
        service,
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
    )
}

pub(crate) fn register_client_route_default_user(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    device_id: &str,
) -> RegisteredClientRouteView {
    register_client_route_for_principal_kind(
        service,
        tenant_id,
        organization_id,
        principal_id,
        "user",
        device_id,
    )
}

fn register_client_route_with_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> RegisteredClientRouteView {
    let organization_id = normalize_realtime_organization_id(organization_id);
    let device = RegisteredClientRouteView {
        tenant_id: tenant_id.into(),
        organization_id: organization_id.clone(),
        principal_id: principal_id.into(),
        principal_kind: principal_kind.into(),
        device_id: device_id.into(),
        registered_at: registered_client_route_at(),
    };
    let scope = client_route_principal_scope_key(
        tenant_id,
        organization_id.as_str(),
        principal_kind,
        principal_id,
    );
    lock_projection_mutex(
        &service.registered_client_routes,
        "registered client route store",
    )
    .entry(scope)
    .or_default()
    .insert(device_id.into(), device.clone());
    lock_projection_mutex(
        &service.client_route_sync_feeds,
        "client route sync feed store",
    )
    .entry(client_route_feed_scope_key(
        tenant_id,
        organization_id.as_str(),
        principal_kind,
        principal_id,
        device_id,
    ))
    .or_default();
    lock_projection_mutex(
        &service.client_route_sync_sequences,
        "client route sync sequence store",
    )
    .entry(client_route_feed_scope_key(
        tenant_id,
        organization_id.as_str(),
        principal_kind,
        principal_id,
        device_id,
    ))
    .or_insert(0);
    device
}

pub(crate) fn registered_client_routes_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Vec<RegisteredClientRouteView> {
    let registered_client_routes = lock_projection_mutex(
        &service.registered_client_routes,
        "registered client route store",
    );
    let mut devices_by_id = BTreeMap::new();

    if let Some(typed_devices) = registered_client_routes.get(&client_route_principal_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
    )) {
        for device in typed_devices.values() {
            devices_by_id.insert(device.device_id.clone(), device.clone());
        }
    }

    let mut devices = devices_by_id.into_values().collect::<Vec<_>>();
    devices.sort_by(|left, right| left.device_id.cmp(&right.device_id));
    devices
}

pub(crate) fn client_route_sync_feed_window_for_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    after_seq: Option<u64>,
    limit: usize,
) -> super::ClientRouteSyncFeedWindowView {
    let min_seq = after_seq.unwrap_or_default();
    let scope = client_route_feed_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    );
    let feeds = lock_projection_mutex(
        &service.client_route_sync_feeds,
        "client route sync feed store",
    );
    let Some(feed) = feeds.get(&scope) else {
        return super::ClientRouteSyncFeedWindowView {
            items: Vec::new(),
            next_after_seq: None,
            has_more: false,
            trimmed_through_seq: 0,
        };
    };

    let trimmed_through_seq = client_route_sync_feed_trimmed_through_seq(feed);
    let mut items = Vec::with_capacity(limit.min(feed.len()));
    let mut has_more = false;
    let mut next_after_seq = None;
    for (sync_seq, entry) in feed.range((Excluded(min_seq), Unbounded)) {
        if entry
            .conversation_id
            .as_deref()
            .is_some_and(|conversation_id| {
                service.is_archived_direct_chat_conversation(
                    tenant_id,
                    organization_id,
                    conversation_id,
                )
            })
        {
            next_after_seq = Some(*sync_seq);
            continue;
        }
        if items.len() == limit {
            has_more = true;
            break;
        }
        items.push(entry.clone());
        next_after_seq = Some(*sync_seq);
    }

    super::ClientRouteSyncFeedWindowView {
        items,
        next_after_seq,
        has_more,
        trimmed_through_seq,
    }
}

pub(crate) fn latest_client_route_sync_seq_for_principal_kind(
    service: &TimelineProjectionService,
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
        &service.client_route_sync_sequences,
        "client route sync sequence store",
    )
    .get(&scope)
    .copied()
    .unwrap_or_default()
}

pub(crate) fn active_conversation_principal_recipients(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
) -> Vec<NotificationRecipientView> {
    let scope = scope_key(tenant_id, organization_id, conversation_id);
    let mut recipients = lock_projection_mutex(&service.members, "member store")
        .get(scope.as_str())
        .map(|scope_members| {
            scope_members
                .values()
                .filter(|member| member.is_active())
                .map(|member| NotificationRecipientView {
                    principal_id: member.principal_id.clone(),
                    principal_kind: member.principal_kind.clone(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    recipients.sort();
    recipients.dedup();
    recipients
}

pub(crate) fn realtime_fanout_targets_for_recipients(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    recipients: impl IntoIterator<Item = NotificationRecipientView>,
) -> Vec<RealtimeFanoutTarget> {
    let mut targets = recipients
        .into_iter()
        .flat_map(|recipient| {
            registered_client_routes_for_principal_kind(
                service,
                tenant_id,
                organization_id,
                recipient.principal_id.as_str(),
                recipient.principal_kind.as_str(),
            )
            .into_iter()
            .map(move |device| RealtimeFanoutTarget {
                principal_id: recipient.principal_id.clone(),
                principal_kind: recipient.principal_kind.clone(),
                device_id: device.device_id,
            })
        })
        .collect::<Vec<_>>();
    targets.sort_by(|left, right| {
        left.principal_id
            .cmp(&right.principal_id)
            .then_with(|| left.principal_kind.cmp(&right.principal_kind))
            .then_with(|| left.device_id.cmp(&right.device_id))
    });
    targets
}

pub(crate) fn client_route_sync_fanout_targets_for_conversation(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    fallback_recipients: Vec<NotificationRecipientView>,
) -> Vec<RealtimeFanoutTarget> {
    let mut recipients = active_conversation_principal_recipients(
        service,
        tenant_id,
        organization_id,
        conversation_id,
    );
    for fallback in fallback_recipients {
        if !recipients.iter().any(|item| item == &fallback) {
            recipients.push(fallback);
        }
    }
    realtime_fanout_targets_for_recipients(service, tenant_id, organization_id, recipients)
}

impl TimelineProjectionService {
    pub fn register_client_route(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> RegisteredClientRouteView {
        register_client_route_default_user(self, tenant_id, organization_id, principal_id, device_id)
    }

    pub fn register_client_route_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> RegisteredClientRouteView {
        self::register_client_route_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub fn client_route_sync_feed_window_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> super::ClientRouteSyncFeedWindowView {
        self::client_route_sync_feed_window_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            after_seq,
            limit,
        )
    }

    pub fn registered_client_routes(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
    ) -> Vec<RegisteredClientRouteView> {
        self::registered_client_routes_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            "user",
        )
    }

    pub fn registered_client_routes_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<RegisteredClientRouteView> {
        self::registered_client_routes_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
        )
    }

    pub fn latest_client_route_sync_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> u64 {
        self::latest_client_route_sync_seq_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            "user",
            device_id,
        )
    }

    pub fn latest_client_route_sync_seq_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> u64 {
        self::latest_client_route_sync_seq_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn append_principal_client_route_sync_event(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        origin_event_id: &str,
        origin_event_type: &str,
        conversation_id: Option<String>,
        message_id: Option<String>,
        message_seq: Option<u64>,
        actor_device_id: Option<String>,
        summary: Option<String>,
        payload_schema: Option<String>,
        payload: Option<String>,
        occurred_at: String,
    ) {
        for device in registered_client_routes_for_principal_kind(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
        ) {
            self.append_client_route_sync_entry(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device.device_id.as_str(),
                |sync_seq| ClientRouteSyncFeedEntry {
                    tenant_id: tenant_id.into(),
                    principal_id: principal_id.into(),
                    device_id: device.device_id.clone(),
                    sync_seq,
                    origin_event_id: origin_event_id.into(),
                    origin_event_type: origin_event_type.into(),
                    conversation_id: conversation_id.clone(),
                    message_id: message_id.clone(),
                    message_seq,
                    member_id: None,
                    read_seq: None,
                    last_read_message_id: None,
                    actor_id: Some(principal_id.into()),
                    actor_kind: Some(principal_kind.into()),
                    actor_device_id: actor_device_id.clone(),
                    summary: summary.clone(),
                    payload_schema: payload_schema.clone(),
                    payload: payload.clone(),
                    occurred_at: occurred_at.clone(),
                },
            );
        }
    }

    pub(crate) fn append_client_route_sync_entry<F>(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        build: F,
    ) where
        F: FnOnce(u64) -> ClientRouteSyncFeedEntry,
    {
        let scope = client_route_feed_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let sync_seq = {
            let mut sequences = lock_projection_mutex(
                &self.client_route_sync_sequences,
                "client route sync sequence store",
            );
            let entry = sequences.entry(scope.clone()).or_insert(0);
            *entry += 1;
            *entry
        };

        let mut feeds = lock_projection_mutex(
            &self.client_route_sync_feeds,
            "client route sync feed store",
        );
        let feed = feeds.entry(scope).or_default();
        feed.insert(sync_seq, build(sync_seq));
        while feed.len() > super::PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_RETAINED_EVENTS {
            feed.pop_first();
        }
    }

    pub(crate) fn append_client_route_sync_draft(
        &self,
        target: &RealtimeFanoutTarget,
        draft: &ClientRouteSyncEntryDraft,
    ) {
        let principal_kind = resolved_client_route_scope_principal_kind(
            self,
            draft.tenant_id.as_str(),
            draft.organization_id.as_str(),
            target.principal_id.as_str(),
            target.principal_kind.as_str(),
            target.device_id.as_str(),
        );
        self.append_client_route_sync_entry(
            draft.tenant_id.as_str(),
            draft.organization_id.as_str(),
            target.principal_id.as_str(),
            principal_kind.as_str(),
            target.device_id.as_str(),
            |sync_seq| draft.build_for_target(target, sync_seq),
        );
    }
}

fn client_route_sync_feed_trimmed_through_seq(
    feed: &BTreeMap<u64, ClientRouteSyncFeedEntry>,
) -> u64 {
    feed.first_key_value()
        .map(|(first_retained_seq, _)| first_retained_seq.saturating_sub(1))
        .unwrap_or_default()
}

fn resolved_client_route_scope_principal_kind(
    service: &TimelineProjectionService,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    let registered_client_routes = lock_projection_mutex(
        &service.registered_client_routes,
        "registered client route store",
    );
    if scope_contains_client_route(
        &registered_client_routes,
        &client_route_principal_scope_key(tenant_id, organization_id, principal_kind, principal_id),
        device_id,
    ) {
        return principal_kind.to_owned();
    }

    principal_kind.to_owned()
}

fn scope_contains_client_route(
    registered_client_routes: &std::collections::HashMap<
        ClientRoutePrincipalScopeKey,
        std::collections::HashMap<String, RegisteredClientRouteView>,
    >,
    scope: &ClientRoutePrincipalScopeKey,
    device_id: &str,
) -> bool {
    registered_client_routes
        .get(scope)
        .is_some_and(|devices| devices.contains_key(device_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registered_client_routes_isolate_organizations() {
        let service = TimelineProjectionService::default();
        register_client_route_for_principal_kind(
            &service,
            "100001",
            "org_a",
            "1",
            "user",
            "d_pad",
        );
        register_client_route_for_principal_kind(
            &service,
            "100001",
            "org_b",
            "1",
            "user",
            "d_pad",
        );

        let org_a_devices = registered_client_routes_for_principal_kind(
            &service, "100001", "org_a", "1", "user",
        );
        let org_b_devices = registered_client_routes_for_principal_kind(
            &service, "100001", "org_b", "1", "user",
        );

        assert_eq!(org_a_devices.len(), 1);
        assert_eq!(org_b_devices.len(), 1);
        assert_eq!(org_a_devices[0].organization_id, "org_a");
        assert_eq!(org_b_devices[0].organization_id, "org_b");
    }
}
