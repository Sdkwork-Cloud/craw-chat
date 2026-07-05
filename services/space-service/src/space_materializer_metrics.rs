//! Prometheus metrics for space supplemental Postgres materialization and journal ordering.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

static POSTGRES_MATERIALIZATION_FAILURES: OnceLock<AtomicU64> = OnceLock::new();
static POSTGRES_JOURNAL_APPEND_FAILURES_AFTER_MATERIALIZE: OnceLock<AtomicU64> = OnceLock::new();

fn failure_counter() -> &'static AtomicU64 {
    POSTGRES_MATERIALIZATION_FAILURES.get_or_init(|| AtomicU64::new(0))
}

fn journal_append_failure_counter() -> &'static AtomicU64 {
    POSTGRES_JOURNAL_APPEND_FAILURES_AFTER_MATERIALIZE.get_or_init(|| AtomicU64::new(0))
}

pub fn record_postgres_materialization_failures(count: u64) {
    if count > 0 {
        failure_counter().fetch_add(count, Ordering::Relaxed);
    }
}

pub fn record_postgres_journal_append_failures_after_materialize(count: u64) {
    if count > 0 {
        journal_append_failure_counter().fetch_add(count, Ordering::Relaxed);
    }
}

pub fn postgres_materialization_failure_count() -> u64 {
    failure_counter().load(Ordering::Relaxed)
}

pub fn postgres_journal_append_failure_after_materialize_count() -> u64 {
    journal_append_failure_counter().load(Ordering::Relaxed)
}

pub fn render_prometheus() -> String {
    format!(
        "# HELP im_space_postgres_materialization_failures_total Supplemental postgres writes that failed before journal append (write rejected; journal unchanged)\n\
         # TYPE im_space_postgres_materialization_failures_total counter\n\
         im_space_postgres_materialization_failures_total {}\n\
         # HELP im_space_postgres_journal_append_failures_after_materialize_total Journal append failures after successful postgres materialize (compensating rollback attempted)\n\
         # TYPE im_space_postgres_journal_append_failures_after_materialize_total counter\n\
         im_space_postgres_journal_append_failures_after_materialize_total {}\n",
        postgres_materialization_failure_count(),
        postgres_journal_append_failure_after_materialize_count()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialization_failure_counter_accumulates() {
        let before = postgres_materialization_failure_count();
        record_postgres_materialization_failures(2);
        record_postgres_materialization_failures(0);
        assert_eq!(postgres_materialization_failure_count(), before + 2);
    }

    #[test]
    fn journal_append_failure_counter_accumulates() {
        let before = postgres_journal_append_failure_after_materialize_count();
        record_postgres_journal_append_failures_after_materialize(1);
        record_postgres_journal_append_failures_after_materialize(0);
        assert_eq!(
            postgres_journal_append_failure_after_materialize_count(),
            before + 1
        );
    }
}
