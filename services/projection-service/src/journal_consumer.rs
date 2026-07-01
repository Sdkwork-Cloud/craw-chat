use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{PostgresCommitJournal, PostgresJournalConfig};
use im_domain_events::CommitEnvelope;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_message::CommitJournal;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::{ProjectionRuntime, TimelineProjectionService};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const PROJECTION_JOURNAL_CONSUMER_POLL_MS_ENV: &str =
    "SDKWORK_IM_PROJECTION_JOURNAL_CONSUMER_POLL_MS";
const DEFAULT_PROJECTION_JOURNAL_CONSUMER_POLL_MS: u64 = 250;

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
        run_projection_journal_consumer(
            journal,
            service,
            runtime,
            poll_interval,
            shutdown_rx,
        )
        .await;
    });
    info!("projection journal consumer started");
    Some(ProjectionJournalConsumerHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_projection_commit_journal_from_env() -> Result<PostgresCommitJournal, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            return PostgresJournalConfig::from_database_config(&config)
                .connect()
                .map_err(|error| format!("postgres projection journal bootstrap failed: {error:?}"));
        }
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

async fn run_projection_journal_consumer(
    journal: PostgresCommitJournal,
    service: Arc<TimelineProjectionService>,
    runtime: Arc<ProjectionRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    let mut applied_event_ids = HashSet::new();
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        match journal.recorded() {
            Ok(events) => apply_journal_events(
                events.as_slice(),
                service.as_ref(),
                runtime.as_ref(),
                &mut applied_event_ids,
            ),
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
    runtime: &ProjectionRuntime,
    applied_event_ids: &mut HashSet<String>,
) {
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

    if applied_new {
        if let Err(error) = runtime.persist_durable_state() {
            warn!(error = %error, "projection journal consumer durable persist failed");
        }
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
