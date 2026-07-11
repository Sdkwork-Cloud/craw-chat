use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{
    JournalReplayCursor, PostgresCommitJournal, PostgresJournalConfig,
};
use im_domain_events::CommitEnvelope;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::{ProjectionError, ProjectionRuntime, TimelineProjectionService};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const PROJECTION_JOURNAL_CONSUMER_POLL_MS_ENV: &str =
    "SDKWORK_IM_PROJECTION_JOURNAL_CONSUMER_POLL_MS";
const DEFAULT_PROJECTION_JOURNAL_CONSUMER_POLL_MS: u64 = 250;

/// Maximum number of durable persist attempts before the consumer surrenders
/// for the current cycle. Each failure is followed by an exponentially growing
/// backoff drawn from [`PROJECTION_PERSIST_RETRY_BACKOFFS`]. The combined
/// worst-case sleep (50ms + 100ms = 150ms) stays below the default 250ms poll
/// interval so the consumer never falls behind because of retries.
const PROJECTION_PERSIST_RETRY_ATTEMPTS: usize = 3;

/// Backoff schedule between durable persist attempts. Index `attempt - 1` is
/// used after the `attempt`-th failure, so only the first `retry - 1` entries
/// are ever slept.
const PROJECTION_PERSIST_RETRY_BACKOFFS: &[Duration] =
    &[Duration::from_millis(50), Duration::from_millis(100)];

pub struct ProjectionJournalConsumerHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl ProjectionJournalConsumerHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

pub fn spawn_projection_journal_consumer_from_env(
    runtime: Arc<ProjectionRuntime>,
) -> Option<ProjectionJournalConsumerHandle> {
    let journal = resolve_projection_commit_journal_from_env().ok()?;
    let poll_interval = resolve_projection_journal_consumer_poll_interval();
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let service = runtime.service();
    let task = tokio::spawn(async move {
        run_projection_journal_consumer(journal, service, runtime, poll_interval, shutdown_rx)
            .await;
    });
    info!("projection journal consumer started");
    Some(ProjectionJournalConsumerHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_projection_commit_journal_from_env() -> Result<PostgresCommitJournal, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres
    {
        return PostgresJournalConfig::from_database_config(&config)
            .connect()
            .map_err(|error| format!("postgres projection journal bootstrap failed: {error:?}"));
    }

    if let Some(database_url) = std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        return PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| format!("postgres projection journal bootstrap failed: {error:?}"));
    }

    Err(format!(
        "projection journal consumer requires postgres journal: set IM database env or {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_projection_journal_consumer_poll_interval() -> Duration {
    let millis = std::env::var(PROJECTION_JOURNAL_CONSUMER_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_PROJECTION_JOURNAL_CONSUMER_POLL_MS);
    Duration::from_millis(millis)
}

const PROJECTION_JOURNAL_APPLIED_EVENT_DEDUP_CAPACITY: usize = 50_000;

struct BoundedAppliedEventDedup {
    order: VecDeque<String>,
    seen: HashSet<String>,
}

impl BoundedAppliedEventDedup {
    fn new() -> Self {
        Self {
            order: VecDeque::with_capacity(PROJECTION_JOURNAL_APPLIED_EVENT_DEDUP_CAPACITY),
            seen: HashSet::with_capacity(PROJECTION_JOURNAL_APPLIED_EVENT_DEDUP_CAPACITY),
        }
    }

    fn insert(&mut self, event_id: String) -> bool {
        if self.seen.contains(&event_id) {
            return false;
        }
        while self.order.len() >= PROJECTION_JOURNAL_APPLIED_EVENT_DEDUP_CAPACITY {
            let Some(evicted) = self.order.pop_front() else {
                break;
            };
            self.seen.remove(evicted.as_str());
        }
        self.seen.insert(event_id.clone());
        self.order.push_back(event_id);
        true
    }

    fn remove(&mut self, event_id: &str) {
        if !self.seen.remove(event_id) {
            return;
        }
        self.order.retain(|candidate| candidate != event_id);
    }
}

async fn run_projection_journal_consumer(
    journal: PostgresCommitJournal,
    service: Arc<TimelineProjectionService>,
    runtime: Arc<ProjectionRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    let mut applied_event_ids = BoundedAppliedEventDedup::new();
    let mut replay_cursor: Option<JournalReplayCursor> = None;
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        match journal.recorded_after(replay_cursor.as_ref()) {
            Ok((events, next_cursor)) if !events.is_empty() => {
                let applied_new =
                    apply_journal_events(&events, service.as_ref(), &mut applied_event_ids);
                if applied_new {
                    persist_durable_state_with_retry(runtime.as_ref()).await;
                }
                replay_cursor = next_cursor;
            }
            Ok(_) => {}
            Err(error) => {
                warn!(error = ?error, "projection journal consumer replay failed");
            }
        }

        tokio::select! {
            _ = shutdown.changed() => break,
            _ = tokio::time::sleep(poll_interval) => {}
        }
    }
}

fn apply_journal_events(
    events: &[CommitEnvelope],
    service: &TimelineProjectionService,
    applied_event_ids: &mut BoundedAppliedEventDedup,
) -> bool {
    let mut applied_new = false;
    for event in events {
        if !applied_event_ids.insert(event.event_id.clone()) {
            continue;
        }
        if let Err(error) = service.apply(event) {
            applied_event_ids.remove(event.event_id.as_str());
            warn!(
                event_id = %event.event_id,
                event_type = %event.event_type,
                error = %error,
                "projection journal consumer failed to apply event"
            );
            continue;
        }
        applied_new = true;
    }
    applied_new
}

/// Persist durable projection state with a bounded retry loop.
///
/// Transient Postgres hiccups (brief pool exhaustion, momentary network
/// blips) used to be swallowed by a single best-effort `warn!`, which let
/// memory and durable snapshots drift apart indefinitely. We now retry up to
/// [`PROJECTION_PERSIST_RETRY_ATTEMPTS`] times with escalating backoff. Because
/// `persist_durable_state` writes the current memory state (not deltas),
/// re-attempts are idempotent. When every attempt fails, the consumer keeps
/// advancing — the next cycle re-attempts the full accumulated state — but the
/// failure is logged with the attempt count so operators can spot drift.
async fn persist_durable_state_with_retry(runtime: &ProjectionRuntime) {
    let mut last_error: Option<ProjectionError> = None;
    for attempt in 1..=PROJECTION_PERSIST_RETRY_ATTEMPTS {
        match runtime.persist_durable_state() {
            Ok(()) => {
                if attempt > 1 {
                    info!(
                        attempt = attempt,
                        "projection journal consumer durable persist recovered after retry"
                    );
                }
                return;
            }
            Err(error) => {
                last_error = Some(error);
                if attempt < PROJECTION_PERSIST_RETRY_ATTEMPTS {
                    let backoff = PROJECTION_PERSIST_RETRY_BACKOFFS
                        .get(attempt - 1)
                        .copied()
                        .unwrap_or(Duration::from_millis(100));
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }
    if let Some(error) = last_error {
        warn!(
            error = %error,
            attempts = PROJECTION_PERSIST_RETRY_ATTEMPTS,
            "projection journal consumer durable persist failed after retries; \
             memory state advanced, durable snapshot will be retried next cycle"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_journal_consumer_poll_interval_has_default() {
        let interval = resolve_projection_journal_consumer_poll_interval();
        assert_eq!(interval, Duration::from_millis(250));
    }
}
