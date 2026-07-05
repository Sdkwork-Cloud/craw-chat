//! Relays durable conversation outbox events to the embedded session-gateway realtime plane.

use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresOutboxStore};
use im_platform_contracts::{OutboxEventRecord, OutboxStore, RealtimeEventRecipient};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use session_gateway::RealtimeDeliveryRuntime;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::outbox_relay_common::{mark_missing_recipients, mark_unexpected_aggregate_type};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const CONVERSATION_OUTBOX_RELAY_POLL_MS_ENV: &str = "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_POLL_MS";
const CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV: &str =
    "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_TENANT_ID";
const CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID";
const CONVERSATION_OUTBOX_AGGREGATE_TYPE: &str = "conversation";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_POLL_MS: u64 = 50;
const DEFAULT_CONVERSATION_OUTBOX_RELAY_TENANT_ID: &str = "100001";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID: &str = "default";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_BATCH_SIZE: usize = 64;
const DEFAULT_CONVERSATION_OUTBOX_RELAY_SCOPE_LIMIT: usize = 32;

pub struct ConversationOutboxRelayHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl ConversationOutboxRelayHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

pub fn spawn_conversation_outbox_relay_from_env(
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Option<ConversationOutboxRelayHandle> {
    let outbox = resolve_conversation_outbox_store_from_env()?;
    let poll_interval = resolve_conversation_outbox_relay_poll_interval();
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task = tokio::spawn(async move {
        run_conversation_outbox_relay(outbox, realtime_runtime, poll_interval, shutdown_rx).await;
    });
    info!("conversation outbox relay started");
    Some(ConversationOutboxRelayHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_conversation_outbox_store_from_env() -> Option<Arc<dyn OutboxStore>> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            return PostgresJournalConfig::from_database_config(&config)
                .connect_pool()
                .ok()
                .map(|pool| Arc::new(PostgresOutboxStore::from_pool(pool)) as Arc<dyn OutboxStore>);
        }
    }

    let database_url = std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())?;
    PostgresJournalConfig::new(database_url)
        .connect_pool()
        .ok()
        .map(|pool| Arc::new(PostgresOutboxStore::from_pool(pool)) as Arc<dyn OutboxStore>)
}

fn resolve_conversation_outbox_relay_poll_interval() -> Duration {
    let millis = std::env::var(CONVERSATION_OUTBOX_RELAY_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_CONVERSATION_OUTBOX_RELAY_POLL_MS);
    Duration::from_millis(millis)
}

fn resolve_conversation_outbox_relay_tenant_id() -> String {
    std::env::var(CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CONVERSATION_OUTBOX_RELAY_TENANT_ID.to_owned())
}

fn resolve_conversation_outbox_relay_organization_id() -> String {
    std::env::var(CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID.to_owned())
}

fn resolve_conversation_outbox_relay_scopes(
    outbox: &Arc<dyn OutboxStore>,
) -> Vec<(String, String)> {
    if std::env::var(CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return vec![(
            resolve_conversation_outbox_relay_tenant_id(),
            resolve_conversation_outbox_relay_organization_id(),
        )];
    }

    match outbox.list_pending_scopes(DEFAULT_CONVERSATION_OUTBOX_RELAY_SCOPE_LIMIT) {
        Ok(scopes) => scopes,
        Err(error) => {
            warn!(error = ?error, "conversation outbox relay scope discovery failed");
            Vec::new()
        }
    }
}

async fn run_conversation_outbox_relay(
    outbox: Arc<dyn OutboxStore>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        for (tenant_id, organization_id) in resolve_conversation_outbox_relay_scopes(&outbox) {
            match outbox.drain_pending(
                tenant_id.as_str(),
                organization_id.as_str(),
                DEFAULT_CONVERSATION_OUTBOX_RELAY_BATCH_SIZE,
            ) {
                Ok(events) => {
                    for event in events {
                        if event.aggregate_type != CONVERSATION_OUTBOX_AGGREGATE_TYPE {
                            mark_unexpected_aggregate_type(
                                &outbox,
                                &event,
                                CONVERSATION_OUTBOX_AGGREGATE_TYPE,
                                "conversation",
                            );
                            continue;
                        }
                        relay_conversation_outbox_event(&realtime_runtime, &outbox, &event);
                    }
                }
                Err(error) => {
                    warn!(
                        tenant_id = tenant_id.as_str(),
                        organization_id = organization_id.as_str(),
                        error = ?error,
                        "conversation outbox relay drain failed"
                    );
                }
            }
        }

        tokio::select! {
            _ = shutdown.changed() => break,
            _ = tokio::time::sleep(poll_interval) => {}
        }
    }
}

fn relay_conversation_outbox_event(
    realtime_runtime: &RealtimeDeliveryRuntime,
    outbox: &Arc<dyn OutboxStore>,
    event: &OutboxEventRecord,
) {
    let payload = build_realtime_payload(event);
    let recipients =
        conversation_realtime_recipients(event.event_type.as_str(), event.payload_json.as_str());
    if recipients.is_empty() {
        mark_missing_recipients(
            outbox,
            event,
            "conversation",
            "recipientPrincipalIds",
        );
        return;
    }

    let recipient_views = recipients
        .into_iter()
        .map(|(principal_id, principal_kind)| {
            RealtimeEventRecipient::new(principal_id, principal_kind)
        })
        .collect::<Vec<_>>();
    let publish_result =
        im_platform_contracts::RealtimeEventPublisher::publish_durable_scope_event_to_recipients(
            realtime_runtime,
        event.tenant_id.as_str(),
        event.organization_id.as_str(),
        "conversation",
        event.aggregate_id.as_str(),
        event.event_type.as_str(),
        payload,
        recipient_views,
    );

    if let Err(error) = publish_result {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            event_type = event.event_type.as_str(),
            error = ?error,
            "conversation outbox relay publish failed"
        );
        let _ = outbox.mark_failed(
            event.tenant_id.as_str(),
            event.organization_id.as_str(),
            event.outbox_id.as_str(),
            "conversation outbox relay publish failed",
        );
        return;
    }

    if let Err(error) = outbox.mark_published(
        event.tenant_id.as_str(),
        event.organization_id.as_str(),
        event.outbox_id.as_str(),
    ) {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            error = ?error,
            "conversation outbox relay mark_published failed"
        );
    }
}

fn build_realtime_payload(event: &OutboxEventRecord) -> String {
    serde_json::json!({
        "eventId": event.event_id,
        "eventType": event.event_type,
        "aggregateType": event.aggregate_type,
        "aggregateId": event.aggregate_id,
        "tenantId": event.tenant_id,
        "organizationId": event.organization_id,
        "payload": serde_json::from_str::<serde_json::Value>(event.payload_json.as_str())
            .unwrap_or_else(|_| serde_json::json!(event.payload_json)),
    })
    .to_string()
}

fn conversation_realtime_recipients(
    event_type: &str,
    payload_json: &str,
) -> Vec<(String, String)> {
    let payload = serde_json::from_str::<serde_json::Value>(payload_json).unwrap_or_default();
    if let Some(ids) = payload
        .get("recipientPrincipalIds")
        .and_then(|value| value.as_array())
    {
        let kinds = payload
            .get("recipientPrincipalKinds")
            .and_then(|value| value.as_array());
        return ids
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                let id = value.as_str()?;
                let kind = kinds
                    .and_then(|items| items.get(index))
                    .and_then(|item| item.as_str())
                    .unwrap_or("user");
                Some((id.to_owned(), kind.to_owned()))
            })
            .collect();
    }

    match event_type {
        "message.posted" | "message.edited" | "message.recalled" => Vec::new(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_realtime_recipients_returns_empty_when_recipient_ids_missing() {
        let payload = serde_json::json!({
            "conversationId": "c_1",
            "messageId": "m_1",
        });
        let recipients = conversation_realtime_recipients(
            "message.posted",
            &payload.to_string(),
        );
        assert!(recipients.is_empty());
    }

    #[test]
    fn conversation_realtime_recipients_reads_recipient_principal_ids() {
        let payload = serde_json::json!({
            "recipientPrincipalIds": ["u_alice", "u_bob"],
            "recipientPrincipalKinds": ["user", "device"],
            "conversationId": "c_1",
        });
        let recipients = conversation_realtime_recipients(
            "message.posted",
            &payload.to_string(),
        );
        assert_eq!(
            recipients,
            vec![
                ("u_alice".to_owned(), "user".to_owned()),
                ("u_bob".to_owned(), "device".to_owned()),
            ]
        );
    }
}
