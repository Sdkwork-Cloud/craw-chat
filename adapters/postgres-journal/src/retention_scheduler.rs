//! Background retention purge scheduler for PostgreSQL-backed IM deployments.
//!
//! Uses a PostgreSQL advisory lock so only one process purges expired rows per database
//! even when multiple service processes (gateway + ops) are running.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use im_platform_contracts::ContractError;
use tracing::{error, info, warn};

use crate::{
    postgres_pool_client, postgres_unavailable, purge_expired_retention_batch,
    PostgresJournalConfig, PostgresJournalPool, RetentionCleanupReport,
};
use crate::retention_metrics::RetentionPurgeMetrics;

const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const SCHEDULER_ENABLED_ENV: &str = "SDKWORK_IM_RETENTION_PURGE_SCHEDULER_ENABLED";
const INTERVAL_SECONDS_ENV: &str = "SDKWORK_IM_RETENTION_PURGE_INTERVAL_SECONDS";
const BATCH_SIZE_ENV: &str = "SDKWORK_IM_RETENTION_PURGE_BATCH_SIZE";
const MAX_BATCHES_PER_TICK_ENV: &str = "SDKWORK_IM_RETENTION_PURGE_MAX_BATCHES_PER_TICK";
const RETENTION_PURGE_ADVISORY_LOCK_KEY: i64 = 0x494D_5254;

const DEFAULT_INTERVAL_SECONDS: u64 = 3_600;
const DEFAULT_BATCH_SIZE: i64 = 500;
const DEFAULT_MAX_BATCHES_PER_TICK: u32 = 100;
const MIN_INTERVAL_SECONDS: u64 = 60;
const MAX_INTERVAL_SECONDS: u64 = 86_400;
const MAX_BATCH_SIZE: i64 = 5_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetentionPurgeSchedulerConfig {
    pub database_url: String,
    pub interval: Duration,
    pub batch_size: i64,
    pub max_batches_per_tick: u32,
}

/// Background retention scheduler owned by a dedicated OS thread.
///
/// The synchronous `postgres`/`r2d2` pool must be created and dropped off Tokio worker
/// threads; aborting a `tokio::spawn` task would drop the pool on a runtime worker and
/// panic inside the sync postgres driver.
pub struct RetentionPurgeSchedulerHandle {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl RetentionPurgeSchedulerHandle {
    pub fn shutdown(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for RetentionPurgeSchedulerHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl RetentionPurgeSchedulerConfig {
    pub fn from_env() -> Option<Self> {
        let database_url = std::env::var(DATABASE_URL_ENV)
            .ok()
            .filter(|value| !value.trim().is_empty())?;
        if !scheduler_enabled_from_env() {
            return None;
        }
        Some(Self {
            database_url,
            interval: Duration::from_secs(read_u64_env(
                INTERVAL_SECONDS_ENV,
                DEFAULT_INTERVAL_SECONDS,
                MIN_INTERVAL_SECONDS,
                MAX_INTERVAL_SECONDS,
            )),
            batch_size: read_i64_env(BATCH_SIZE_ENV, DEFAULT_BATCH_SIZE, 1, MAX_BATCH_SIZE),
            max_batches_per_tick: read_u64_env(
                MAX_BATCHES_PER_TICK_ENV,
                DEFAULT_MAX_BATCHES_PER_TICK as u64,
                1,
                10_000,
            ) as u32,
        })
    }
}

/// Spawns the retention purge scheduler when env configuration is present and enabled.
pub fn spawn_retention_purge_scheduler_from_env() -> Option<RetentionPurgeSchedulerHandle> {
    let config = RetentionPurgeSchedulerConfig::from_env()?;
    let metrics = RetentionPurgeMetrics::global();
    info!(
        target: "sdkwork.im",
        event = "im.retention_purge.scheduler_started",
        interval_seconds = config.interval.as_secs(),
        batch_size = config.batch_size,
        max_batches_per_tick = config.max_batches_per_tick,
        "retention purge scheduler started"
    );
    Some(spawn_retention_purge_scheduler_with_pool_bootstrap(config, metrics))
}

fn spawn_retention_purge_scheduler_with_pool_bootstrap(
    config: RetentionPurgeSchedulerConfig,
    metrics: Arc<RetentionPurgeMetrics>,
) -> RetentionPurgeSchedulerHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_thread = stop.clone();
    let thread = match std::thread::Builder::new()
        .name("im-retention-purge".into())
        .spawn(move || {
            let pool = match PostgresJournalConfig::new(config.database_url.clone()).connect_pool() {
                Ok(pool) => pool,
                Err(error) => {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.retention_purge.scheduler_start_failed",
                        error = %format!("{error:?}"),
                        "retention purge scheduler disabled because database pool could not be created"
                    );
                    return;
                }
            };
            retention_purge_scheduler_loop(config, pool, metrics, stop_for_thread);
        })
    {
        Ok(handle) => Some(handle),
        Err(error) => {
            error!(
                target: "sdkwork.im",
                event = "im.retention_purge.scheduler_spawn_failed",
                error = %error,
                "retention purge scheduler thread spawn failed; scheduler disabled for this process"
            );
            None
        }
    };
    RetentionPurgeSchedulerHandle { stop, thread }
}

pub fn spawn_retention_purge_scheduler(
    config: RetentionPurgeSchedulerConfig,
    pool: PostgresJournalPool,
    metrics: Arc<RetentionPurgeMetrics>,
) -> RetentionPurgeSchedulerHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_thread = stop.clone();
    let thread = match std::thread::Builder::new()
        .name("im-retention-purge".into())
        .spawn(move || retention_purge_scheduler_loop(config, pool, metrics, stop_for_thread))
    {
        Ok(handle) => Some(handle),
        Err(error) => {
            error!(
                target: "sdkwork.im",
                event = "im.retention_purge.scheduler_spawn_failed",
                error = %error,
                "retention purge scheduler thread spawn failed; scheduler disabled for this process"
            );
            None
        }
    };
    RetentionPurgeSchedulerHandle { stop, thread }
}

fn retention_purge_scheduler_loop(
    config: RetentionPurgeSchedulerConfig,
    pool: PostgresJournalPool,
    metrics: Arc<RetentionPurgeMetrics>,
    stop: Arc<AtomicBool>,
) {
    if let Err(error) = run_retention_purge_tick(&pool, &config, metrics.as_ref()) {
        metrics.record_failure();
        error!(
            target: "sdkwork.im",
            event = "im.retention_purge.tick_failed",
            error = %format!("{error:?}"),
            "retention purge tick failed"
        );
    }

    while !stop.load(Ordering::Relaxed) {
        if sleep_with_stop(config.interval, &stop) {
            break;
        }
        if let Err(error) = run_retention_purge_tick(&pool, &config, metrics.as_ref()) {
            metrics.record_failure();
            error!(
                target: "sdkwork.im",
                event = "im.retention_purge.tick_failed",
                error = %format!("{error:?}"),
                "retention purge tick failed"
            );
        }
    }
}

fn sleep_with_stop(duration: Duration, stop: &AtomicBool) -> bool {
    const SLICE: Duration = Duration::from_millis(200);
    let mut remaining = duration;
    while remaining > Duration::ZERO {
        if stop.load(Ordering::Relaxed) {
            return true;
        }
        let step = remaining.min(SLICE);
        std::thread::sleep(step);
        remaining = remaining.saturating_sub(step);
    }
    stop.load(Ordering::Relaxed)
}

fn run_retention_purge_tick(
    pool: &PostgresJournalPool,
    config: &RetentionPurgeSchedulerConfig,
    metrics: &RetentionPurgeMetrics,
) -> Result<(), ContractError> {
    let started = Instant::now();
    let lock = RetentionPurgeLock::try_acquire(pool)?;
    if !lock.acquired {
        metrics.record_skipped_lock();
        return Ok(());
    }
    let mut batches = 0_u32;
    let mut aggregate = RetentionCleanupReport::default();
    loop {
        let report = purge_expired_retention_batch(pool, Some(config.batch_size))?;
        aggregate.merge(&report);
        batches += 1;
        metrics.record_batch(&report, started.elapsed().as_micros().min(u64::MAX as u128) as u64);
        if report.is_empty() || batches >= config.max_batches_per_tick {
            break;
        }
    }
    if aggregate.total_deleted() > 0 {
        info!(
            target: "sdkwork.im",
            event = "im.retention_purge.tick_completed",
            batches,
            commit_journal_deleted = aggregate.commit_journal_deleted,
            conversation_messages_deleted = aggregate.conversation_messages_deleted,
            message_media_refs_deleted = aggregate.message_media_refs_deleted,
            outbox_events_deleted = aggregate.outbox_events_deleted,
            inbox_events_deleted = aggregate.inbox_events_deleted,
            projection_timeline_entries_deleted = aggregate.projection_timeline_entries_deleted,
            realtime_device_events_deleted = aggregate.realtime_device_events_deleted,
            duration_ms = started.elapsed().as_millis() as u64,
            "retention purge tick completed"
        );
    }
    Ok(())
}

struct RetentionPurgeLock {
    acquired: bool,
    pool: PostgresJournalPool,
}

impl RetentionPurgeLock {
    fn try_acquire(pool: &PostgresJournalPool) -> Result<Self, ContractError> {
        let pool = pool.clone();
        let pool_for_lock = pool.clone();
        let acquired = crate::run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool_for_lock, "retention purge lock")?;
            let row = client
                .query_one("SELECT pg_try_advisory_lock($1)", &[&RETENTION_PURGE_ADVISORY_LOCK_KEY])
                .map_err(|error| postgres_unavailable("retention purge lock acquire", error))?;
            Ok(row.get::<_, bool>(0))
        })?;
        Ok(Self { acquired, pool })
    }
}

impl Drop for RetentionPurgeLock {
    fn drop(&mut self) {
        if !self.acquired {
            return;
        }
        let pool = self.pool.clone();
        if let Err(error) = crate::run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "retention purge unlock")?;
            client
                .execute(
                    "SELECT pg_advisory_unlock($1)",
                    &[&RETENTION_PURGE_ADVISORY_LOCK_KEY],
                )
                .map_err(|error| postgres_unavailable("retention purge lock release", error))?;
            Ok(())
        }) {
            warn!(
                target: "sdkwork.im",
                event = "im.retention_purge.lock_release_failed",
                error = %format!("{error:?}"),
                "failed to release retention purge advisory lock"
            );
        }
    }
}

fn scheduler_enabled_from_env() -> bool {
    match std::env::var(SCHEDULER_ENABLED_ENV)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("0") | Some("false") | Some("no") | Some("off") => false,
        Some(_) => true,
        None => true,
    }
}

fn read_u64_env(name: &str, default: u64, min: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn read_i64_env(name: &str, default: i64, min: i64, max: i64) -> i64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

trait RetentionCleanupReportExt {
    fn merge(&mut self, other: &RetentionCleanupReport);
    fn is_empty(&self) -> bool;
    fn total_deleted(&self) -> u64;
}

impl RetentionCleanupReportExt for RetentionCleanupReport {
    fn merge(&mut self, other: &RetentionCleanupReport) {
        self.commit_journal_deleted += other.commit_journal_deleted;
        self.conversation_messages_deleted += other.conversation_messages_deleted;
        self.message_media_refs_deleted += other.message_media_refs_deleted;
        self.outbox_events_deleted += other.outbox_events_deleted;
        self.inbox_events_deleted += other.inbox_events_deleted;
        self.projection_timeline_entries_deleted += other.projection_timeline_entries_deleted;
        self.realtime_device_events_deleted += other.realtime_device_events_deleted;
    }

    fn is_empty(&self) -> bool {
        self.total_deleted() == 0
    }

    fn total_deleted(&self) -> u64 {
        self.commit_journal_deleted
            + self.conversation_messages_deleted
            + self.message_media_refs_deleted
            + self.outbox_events_deleted
            + self.inbox_events_deleted
            + self.projection_timeline_entries_deleted
            + self.realtime_device_events_deleted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_enabled_flag_parsing() {
        assert!(matches_env_flag("true"));
        assert!(matches_env_flag("1"));
        assert!(!matches_env_flag("false"));
        assert!(!matches_env_flag("off"));
    }

    fn matches_env_flag(value: &str) -> bool {
        match value.trim().to_ascii_lowercase().as_str() {
            "0" | "false" | "no" | "off" => false,
            _ => true,
        }
    }

    #[test]
    fn test_retention_cleanup_report_merge_and_empty() {
        let mut left = RetentionCleanupReport {
            commit_journal_deleted: 1,
            ..RetentionCleanupReport::default()
        };
        let right = RetentionCleanupReport {
            conversation_messages_deleted: 2,
            ..RetentionCleanupReport::default()
        };
        left.merge(&right);
        assert_eq!(left.total_deleted(), 3);
        assert!(!left.is_empty());
        assert!(RetentionCleanupReport::default().is_empty());
    }
}
