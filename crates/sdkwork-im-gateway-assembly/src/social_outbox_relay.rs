//! Relays durable social outbox events to the embedded session-gateway realtime plane.

use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresOutboxStore};
use im_platform_contracts::{OutboxEventRecord, OutboxStore};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use session_gateway::RealtimeDeliveryRuntime;
use social_service::SOCIAL_OUTBOX_AGGREGATE_TYPE;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::outbox_relay_common::{mark_missing_recipients, mark_unexpected_aggregate_type};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const SOCIAL_OUTBOX_RELAY_POLL_MS_ENV: &str = "SDKWORK_IM_SOCIAL_OUTBOX_RELAY_POLL_MS";
const SOCIAL_OUTBOX_RELAY_TENANT_ID_ENV: &str = "SDKWORK_IM_SOCIAL_OUTBOX_RELAY_TENANT_ID";
const SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_IM_SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID";
const DEFAULT_SOCIAL_OUTBOX_RELAY_POLL_MS: u64 = 50;
const DEFAULT_SOCIAL_OUTBOX_RELAY_TENANT_ID: &str = "100001";
const DEFAULT_SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID: &str = "default";
const DEFAULT_SOCIAL_OUTBOX_RELAY_BATCH_SIZE: usize = 64;
const DEFAULT_SOCIAL_OUTBOX_RELAY_SCOPE_LIMIT: usize = 32;

pub struct SocialOutboxRelayHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl SocialOutboxRelayHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

pub fn spawn_social_outbox_relay_from_env(
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Option<SocialOutboxRelayHandle> {
    let outbox = resolve_social_outbox_store_from_env()?;
    let poll_interval = resolve_social_outbox_relay_poll_interval();
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task = tokio::spawn(async move {
        run_social_outbox_relay(outbox, realtime_runtime, poll_interval, shutdown_rx).await;
    });
    info!("social outbox relay started");
    Some(SocialOutboxRelayHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_social_outbox_store_from_env() -> Option<Arc<dyn OutboxStore>> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            return PostgresJournalConfig::from_database_config(&config)
                .connect_pool()
                .ok()
                .map(|pool| {
                    Arc::new(PostgresOutboxStore::from_pool(pool)) as Arc<dyn OutboxStore>
                });
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

fn resolve_social_outbox_relay_poll_interval() -> Duration {
    let millis = std::env::var(SOCIAL_OUTBOX_RELAY_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_SOCIAL_OUTBOX_RELAY_POLL_MS);
    Duration::from_millis(millis)
}

fn resolve_social_outbox_relay_tenant_id() -> String {
    std::env::var(SOCIAL_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_SOCIAL_OUTBOX_RELAY_TENANT_ID.to_owned())
}

fn resolve_social_outbox_relay_organization_id() -> String {
    std::env::var(SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_SOCIAL_OUTBOX_RELAY_ORGANIZATION_ID.to_owned())
}

fn resolve_social_outbox_relay_scopes(outbox: &Arc<dyn OutboxStore>) -> Vec<(String, String)> {
    if std::env::var(SOCIAL_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return vec![(
            resolve_social_outbox_relay_tenant_id(),
            resolve_social_outbox_relay_organization_id(),
        )];
    }

    match outbox.list_pending_scopes(DEFAULT_SOCIAL_OUTBOX_RELAY_SCOPE_LIMIT) {
        Ok(scopes) => scopes,
        Err(error) => {
            warn!(error = ?error, "social outbox relay scope discovery failed");
            Vec::new()
        }
    }
}

async fn run_social_outbox_relay(
    outbox: Arc<dyn OutboxStore>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        for (tenant_id, organization_id) in resolve_social_outbox_relay_scopes(&outbox) {
            match outbox.drain_pending(
                tenant_id.as_str(),
                organization_id.as_str(),
                DEFAULT_SOCIAL_OUTBOX_RELAY_BATCH_SIZE,
            ) {
                Ok(events) => {
                    for event in events {
                        if event.aggregate_type != SOCIAL_OUTBOX_AGGREGATE_TYPE {
                            mark_unexpected_aggregate_type(
                                &outbox,
                                &event,
                                SOCIAL_OUTBOX_AGGREGATE_TYPE,
                                "social",
                            );
                            continue;
                        }
                        relay_social_outbox_event(&realtime_runtime, &outbox, &event);
                    }
                }
                Err(error) => {
                    warn!(
                        tenant_id = tenant_id.as_str(),
                        organization_id = organization_id.as_str(),
                        error = ?error,
                        "social outbox relay drain failed"
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

fn relay_social_outbox_event(
    realtime_runtime: &RealtimeDeliveryRuntime,
    outbox: &Arc<dyn OutboxStore>,
    event: &OutboxEventRecord,
) {
    let payload = build_realtime_payload(event);
    let recipients = social_realtime_recipients(event.payload_json.as_str());
    if recipients.is_empty() {
        mark_missing_recipients(outbox, event, "social", "recipientPrincipalIds");
        return;
    }

    if let Err(error) = realtime_runtime.publish_durable_user_scope_events_to_principals(
        event.tenant_id.as_str(),
        event.organization_id.as_str(),
        event.event_type.as_str(),
        payload,
        recipients,
    ) {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            event_type = event.event_type.as_str(),
            error = ?error,
            "social outbox relay publish failed"
        );
        let _ = outbox.mark_failed(
            event.tenant_id.as_str(),
            event.organization_id.as_str(),
            event.outbox_id.as_str(),
            "social outbox relay publish failed",
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
            "social outbox relay mark_published failed"
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

fn social_realtime_recipients(payload_json: &str) -> Vec<(String, String)> {
    let payload = serde_json::from_str::<serde_json::Value>(payload_json).unwrap_or_default();
    payload
        .get("recipientPrincipalIds")
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(|id| (id.to_owned(), "user".to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn social_realtime_recipients_reads_recipient_principal_ids() {
        let payload = serde_json::json!({
            "recipientPrincipalIds": ["u_alice", "u_bob"],
            "commitPayload": { "requesterUserId": "u_alice" },
        });
        let recipients = social_realtime_recipients(&payload.to_string());
        assert_eq!(
            recipients,
            vec![
                ("u_alice".to_owned(), "user".to_owned()),
                ("u_bob".to_owned(), "user".to_owned()),
            ]
        );
    }

    #[test]
    fn social_realtime_recipients_empty_when_field_missing() {
        let payload = serde_json::json!({
            "commitPayload": { "requesterUserId": "u_alice" },
        });
        assert!(social_realtime_recipients(&payload.to_string()).is_empty());
    }
}
