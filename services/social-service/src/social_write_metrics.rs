//! Prometheus metrics for atomic Social journal and normalized state writes.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

static POSTGRES_ATOMIC_WRITE_FAILURES: OnceLock<AtomicU64> = OnceLock::new();

fn atomic_write_failure_counter() -> &'static AtomicU64 {
    POSTGRES_ATOMIC_WRITE_FAILURES.get_or_init(|| AtomicU64::new(0))
}

pub fn record_postgres_atomic_write_failures(count: u64) {
    if count > 0 {
        atomic_write_failure_counter().fetch_add(count, Ordering::Relaxed);
    }
}

pub fn postgres_atomic_write_failure_count() -> u64 {
    atomic_write_failure_counter().load(Ordering::Relaxed)
}

pub fn render_prometheus() -> String {
    format!(
        "# HELP im_social_postgres_atomic_write_failures_total Atomic journal and normalized Social state transactions that rolled back or were rejected as misconfigured\n\
         # TYPE im_social_postgres_atomic_write_failures_total counter\n\
         im_social_postgres_atomic_write_failures_total {}\n",
        postgres_atomic_write_failure_count()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_failure_counter_accumulates() {
        let before = postgres_atomic_write_failure_count();
        record_postgres_atomic_write_failures(1);
        record_postgres_atomic_write_failures(0);
        assert_eq!(postgres_atomic_write_failure_count(), before + 1);
    }
}
