//! Relays durable RTC outbox events to the embedded session-gateway realtime plane.

use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresOutboxStore};
use im_platform_contracts::{OutboxEventClaim, OutboxEventRecord, OutboxStore};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use session_gateway::RealtimeDeliveryRuntime;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::outbox_relay_common::{
    DEFAULT_OUTBOX_CLAIM_LEASE, discover_outbox_scopes, log_unexpected_aggregate_type,
    mark_missing_recipients,
};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_DATABASE_URL";
const RTC_OUTBOX_RELAY_POLL_MS_ENV: &str = "SDKWORK_IM_RTC_OUTBOX_RELAY_POLL_MS";
const RTC_OUTBOX_RELAY_TENANT_ID_ENV: &str = "SDKWORK_IM_RTC_OUTBOX_RELAY_TENANT_ID";
const RTC_OUTBOX_RELAY_ORGANIZATION_ID_ENV: &str = "SDKWORK_IM_RTC_OUTBOX_RELAY_ORGANIZATION_ID";
const RTC_OUTBOX_AGGREGATE_TYPE: &str = "rtc_session";
const DEFAULT_RTC_OUTBOX_RELAY_POLL_MS: u64 = 50;
const DEFAULT_RTC_OUTBOX_RELAY_TENANT_ID: &str = "100001";
const DEFAULT_RTC_OUTBOX_RELAY_ORGANIZATION_ID: &str = "default";
const DEFAULT_RTC_OUTBOX_RELAY_BATCH_SIZE: usize = 64;
const DEFAULT_RTC_OUTBOX_RELAY_SCOPE_LIMIT: usize = 32;
const RTC_OUTBOX_RELAY_WORKER_ID: &str = "rtc-outbox-relay";

pub struct RtcOutboxRelayHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl RtcOutboxRelayHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

pub fn spawn_rtc_outbox_relay_from_env(
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Option<RtcOutboxRelayHandle> {
    let outbox = resolve_rtc_outbox_store_from_env()?;
    let poll_interval = resolve_rtc_outbox_relay_poll_interval();
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task = tokio::spawn(async move {
        run_rtc_outbox_relay(outbox, realtime_runtime, poll_interval, shutdown_rx).await;
    });
    info!("rtc outbox relay started");
    Some(RtcOutboxRelayHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_rtc_outbox_store_from_env() -> Option<Arc<dyn OutboxStore>> {
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres {
            return PostgresJournalConfig::from_database_config(&config)
                .connect_pool()
                .ok()
                .map(|pool| {
                    Arc::new(PostgresOutboxStore::from_pool(pool)) as Arc<dyn OutboxStore>
                });
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

fn resolve_rtc_outbox_relay_poll_interval() -> Duration {
    let millis = std::env::var(RTC_OUTBOX_RELAY_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_RTC_OUTBOX_RELAY_POLL_MS);
    Duration::from_millis(millis)
}

fn resolve_rtc_outbox_relay_tenant_id() -> String {
    std::env::var(RTC_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_RTC_OUTBOX_RELAY_TENANT_ID.to_owned())
}

fn resolve_rtc_outbox_relay_organization_id() -> String {
    std::env::var(RTC_OUTBOX_RELAY_ORGANIZATION_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_RTC_OUTBOX_RELAY_ORGANIZATION_ID.to_owned())
}

fn resolve_rtc_outbox_relay_scopes(outbox: &Arc<dyn OutboxStore>) -> Vec<(String, String)> {
    if std::env::var(RTC_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return vec![(
            resolve_rtc_outbox_relay_tenant_id(),
            resolve_rtc_outbox_relay_organization_id(),
        )];
    }

    match discover_outbox_scopes(
        outbox.as_ref(),
        RTC_OUTBOX_RELAY_WORKER_ID,
        RTC_OUTBOX_AGGREGATE_TYPE,
        DEFAULT_RTC_OUTBOX_RELAY_SCOPE_LIMIT,
    ) {
        Ok(scopes) => scopes,
        Err(error) => {
            warn!(error = ?error, "rtc outbox relay scope discovery failed");
            Vec::new()
        }
    }
}

async fn run_rtc_outbox_relay(
    outbox: Arc<dyn OutboxStore>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        for (tenant_id, organization_id) in resolve_rtc_outbox_relay_scopes(&outbox) {
            match outbox.claim_pending(
                tenant_id.as_str(),
                organization_id.as_str(),
                RTC_OUTBOX_AGGREGATE_TYPE,
                DEFAULT_RTC_OUTBOX_RELAY_BATCH_SIZE,
                DEFAULT_OUTBOX_CLAIM_LEASE,
            ) {
                Ok(claims) => {
                    for claim in claims {
                        let event = &claim.event;
                        if event.aggregate_type != RTC_OUTBOX_AGGREGATE_TYPE {
                            log_unexpected_aggregate_type(event, RTC_OUTBOX_AGGREGATE_TYPE, "rtc");
                            continue;
                        }
                        relay_rtc_outbox_event(&realtime_runtime, &outbox, &claim);
                    }
                }
                Err(error) => {
                    warn!(
                        tenant_id = tenant_id.as_str(),
                        organization_id = organization_id.as_str(),
                        error = ?error,
                        "rtc outbox relay drain failed"
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

fn relay_rtc_outbox_event(
    realtime_runtime: &RealtimeDeliveryRuntime,
    outbox: &Arc<dyn OutboxStore>,
    claim: &OutboxEventClaim,
) {
    let event = &claim.event;
    let payload = build_realtime_payload(event);
    let recipients =
        rtc_realtime_recipients(event.event_type.as_str(), event.payload_json.as_str());
    if recipients.is_empty() {
        mark_missing_recipients(outbox, claim, "rtc", "recipientPrincipalIds");
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
            "rtc outbox relay publish failed"
        );
        let _ = outbox.mark_failed(claim, "rtc outbox relay publish failed");
        return;
    }

    if let Err(error) = outbox.mark_published(claim) {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            error = ?error,
            "rtc outbox relay mark_published failed"
        );
    }
}

fn build_realtime_payload(event: &OutboxEventRecord) -> String {
    let mut payload = serde_json::from_str::<serde_json::Value>(event.payload_json.as_str())
        .unwrap_or_else(|_| serde_json::json!(event.payload_json));
    // Strip server-internal routing fields before exposing the payload to
    // clients over WebSocket. `recipient_principal_ids` is injected by the
    // outbox enqueue path for recipient resolution and must not leak the
    // full participant list to each recipient (privacy: user IDs).
    if let Some(obj) = payload.as_object_mut() {
        obj.remove("recipient_principal_ids");
        obj.remove("recipientPrincipalIds");
    }
    serde_json::json!({
        "eventId": event.event_id,
        "eventType": event.event_type,
        "aggregateType": event.aggregate_type,
        "aggregateId": event.aggregate_id,
        "tenantId": event.tenant_id,
        "organizationId": event.organization_id,
        "payload": payload,
    })
    .to_string()
}

fn rtc_realtime_recipients(event_type: &str, payload_json: &str) -> Vec<(String, String)> {
    let payload = serde_json::from_str::<serde_json::Value>(payload_json).unwrap_or_default();
    im_domain_core::rtc_outbox::resolve_rtc_outbox_recipients(event_type, &payload)
}
