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

impl Drop for ProjectionJournalConsumerHandle {
    fn drop(&mut self) {
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
                if let Ok(applied_events) =
                    apply_journal_events(&events, service.as_ref(), &mut applied_event_ids)
                {
                    match persist_durable_state_with_retry(runtime.as_ref(), &applied_events).await
                    {
                        Ok(()) => replay_cursor = next_cursor,
                        Err(error) => {
                            for event in &applied_events {
                                applied_event_ids.remove(event.event_id.as_str());
                            }
                            warn!(
                                error = %error,
                                event_count = applied_events.len(),
                                "projection journal consumer durable persist failed; replay cursor remains unchanged"
                            );
                        }
                    }
                }
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
) -> Result<Vec<CommitEnvelope>, ()> {
    let mut applied_events: Vec<CommitEnvelope> = Vec::with_capacity(events.len());
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
                "projection journal consumer failed to apply event; replay cursor remains unchanged"
            );
            for applied_event in &applied_events {
                applied_event_ids.remove(applied_event.event_id.as_str());
            }
            return Err(());
        }
        applied_events.push((*event).clone());
    }
    Ok(applied_events)
}

/// Persist only dirty scopes from one journal batch with bounded retries.
/// The caller advances its cursor only after this function succeeds.
async fn persist_durable_state_with_retry(
    runtime: &ProjectionRuntime,
    events: &[CommitEnvelope],
) -> Result<(), ProjectionError> {
    let mut last_error: Option<ProjectionError> = None;
    for attempt in 1..=PROJECTION_PERSIST_RETRY_ATTEMPTS {
        match runtime.persist_durable_state_for_events(events) {
            Ok(()) => {
                if attempt > 1 {
                    info!(
                        attempt = attempt,
                        "projection journal consumer durable persist recovered after retry"
                    );
                }
                return Ok(());
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
    Err(last_error.unwrap_or_else(|| {
        ProjectionError::InvalidEvent(
            "projection durable persist retry attempts must be greater than zero".into(),
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;

    #[test]
    fn projection_journal_consumer_poll_interval_has_default() {
        let interval = resolve_projection_journal_consumer_poll_interval();
        assert_eq!(interval, Duration::from_millis(250));
    }

    #[test]
    fn failed_batch_releases_all_dedup_entries_for_retry() {
        let service = TimelineProjectionService::default();
        let mut dedup = BoundedAppliedEventDedup::new();
        let accepted = CommitEnvelope::minimal(
            "event-accepted",
            "tenant-1",
            "projection.noop",
            "conversation",
            "conversation-1",
            1,
        );
        let mut rejected = CommitEnvelope::minimal(
            "event-rejected",
            "tenant-1",
            "message.posted",
            "conversation",
            "conversation-1",
            2,
        );
        rejected.payload = "not-json".into();

        assert!(apply_journal_events(&[accepted, rejected], &service, &mut dedup).is_err());
        assert!(dedup.insert("event-accepted".into()));
        assert!(dedup.insert("event-rejected".into()));
    }

    #[test]
    fn successful_batch_returns_only_newly_applied_events() {
        let service = TimelineProjectionService::default();
        let mut dedup = BoundedAppliedEventDedup::new();
        let event = CommitEnvelope::minimal(
            "event-1",
            "tenant-1",
            "projection.noop",
            "conversation",
            "conversation-1",
            1,
        );

        let first = apply_journal_events(std::slice::from_ref(&event), &service, &mut dedup)
            .expect("first apply should succeed");
        let duplicate = apply_journal_events(&[event], &service, &mut dedup)
            .expect("duplicate apply should succeed without work");

        assert_eq!(first.len(), 1);
        assert!(duplicate.is_empty());
    }
}
