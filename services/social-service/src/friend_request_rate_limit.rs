//! Per-user daily friend request rate limiting (SECURITY_SPEC abuse protection).
//!
//! Counts are incremented only after a friend request is durably committed so
//! failed/idempotent retries do not consume quota. When Postgres supplemental
//! stores are configured, daily counts are read from `im_friend_requests` so
//! multi-instance social-service processes share one authoritative quota.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use im_adapters_social_postgres::friend_request_store::FriendRequestStore;
use im_app_context::is_production_like_im_environment;

const FRIEND_REQUEST_DAILY_LIMIT_ENV: &str = "SDKWORK_IM_FRIEND_REQUEST_DAILY_LIMIT";
const DEFAULT_FRIEND_REQUEST_DAILY_LIMIT: u32 = 50;

pub(crate) fn is_production_like_environment() -> bool {
    is_production_like_im_environment()
}

#[derive(Debug)]
pub(crate) struct FriendRequestRateLimitError {
    pub(crate) message: String,
    pub(crate) retry_after_seconds: u64,
}

#[derive(Default)]
struct FriendRequestDailyLimiter {
    counts: HashMap<String, u32>,
    day_key: String,
}

impl FriendRequestDailyLimiter {
    fn check_allowed(&mut self, tenant_id: &str, user_id: &str) -> Result<(), FriendRequestRateLimitError> {
        self.roll_day_if_needed();
        let limit = resolve_friend_request_daily_limit();
        let key = format!("{tenant_id}:{user_id}");
        let current = self.counts.get(&key).copied().unwrap_or(0);
        if current >= limit {
            return Err(rate_limit_exceeded_error(limit));
        }
        Ok(())
    }

    fn record_submitted(&mut self, tenant_id: &str, user_id: &str) {
        self.roll_day_if_needed();
        let key = format!("{tenant_id}:{user_id}");
        let entry = self.counts.entry(key).or_insert(0);
        *entry = entry.saturating_add(1);
    }

    fn roll_day_if_needed(&mut self) {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        if self.day_key != today {
            self.day_key = today;
            self.counts.clear();
        }
    }
}

fn resolve_friend_request_daily_limit() -> u32 {
    std::env::var(FRIEND_REQUEST_DAILY_LIMIT_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_FRIEND_REQUEST_DAILY_LIMIT)
}

fn utc_day_bounds_rfc3339() -> (String, String) {
    let now = chrono::Utc::now();
    let start = now.date_naive().and_hms_opt(0, 0, 0).expect("midnight should exist");
    let end = (now.date_naive() + chrono::Days::new(1))
        .and_hms_opt(0, 0, 0)
        .expect("midnight should exist");
    (start.and_utc().to_rfc3339(), end.and_utc().to_rfc3339())
}

fn seconds_until_next_utc_day() -> u64 {
    let now = chrono::Utc::now();
    let tomorrow = (now.date_naive() + chrono::Days::new(1))
        .and_hms_opt(0, 0, 0)
        .expect("midnight should exist");
    (tomorrow.and_utc() - now).num_seconds().max(1) as u64
}

fn rate_limit_exceeded_error(limit: u32) -> FriendRequestRateLimitError {
    FriendRequestRateLimitError {
        message: format!("friend request daily limit exceeded ({limit} requests per day)"),
        retry_after_seconds: seconds_until_next_utc_day(),
    }
}

fn check_postgres_friend_request_rate_allowed(
    store: &dyn FriendRequestStore,
    tenant_id: &str,
    organization_id: &str,
    requester_user_id: &str,
) -> Result<(), FriendRequestRateLimitError> {
    let limit = resolve_friend_request_daily_limit();
    let (start_inclusive, end_exclusive) = utc_day_bounds_rfc3339();
    let current = store
        .count_by_requester_created_between(
            tenant_id,
            organization_id,
            requester_user_id,
            start_inclusive.as_str(),
            end_exclusive.as_str(),
        )
        .map_err(|error| FriendRequestRateLimitError {
            message: format!("friend request rate limit store unavailable: {error:?}"),
            retry_after_seconds: 60,
        })?;
    if current >= limit as i64 {
        return Err(rate_limit_exceeded_error(limit));
    }
    Ok(())
}

static FRIEND_REQUEST_RATE_LIMITER: LazyLock<Mutex<FriendRequestDailyLimiter>> =
    LazyLock::new(|| {
        Mutex::new(FriendRequestDailyLimiter {
            counts: HashMap::new(),
            day_key: String::new(),
        })
    });

pub(crate) fn check_friend_request_rate_allowed(
    tenant_id: &str,
    organization_id: &str,
    requester_user_id: &str,
    postgres_store: Option<&dyn FriendRequestStore>,
) -> Result<(), FriendRequestRateLimitError> {
    if postgres_store.is_none() && is_production_like_environment() {
        return Err(FriendRequestRateLimitError {
            message: "friend request rate limit store is required in production-like environments"
                .to_owned(),
            retry_after_seconds: 60,
        });
    }
    if let Some(store) = postgres_store {
        return check_postgres_friend_request_rate_allowed(
            store,
            tenant_id,
            organization_id,
            requester_user_id,
        );
    }

    FRIEND_REQUEST_RATE_LIMITER
        .lock()
        .map_err(|_| FriendRequestRateLimitError {
            message: "friend request rate limiter unavailable".to_owned(),
            retry_after_seconds: 60,
        })?
        .check_allowed(tenant_id, requester_user_id)
}

pub(crate) fn record_friend_request_submitted(
    tenant_id: &str,
    requester_user_id: &str,
    postgres_authority: bool,
) {
    if postgres_authority {
        return;
    }
    if let Ok(mut limiter) = FRIEND_REQUEST_RATE_LIMITER.lock() {
        limiter.record_submitted(tenant_id, requester_user_id);
    }
}

#[cfg(test)]
pub(crate) fn reset_friend_request_rate_limiter_for_tests() {
    if let Ok(mut limiter) = FRIEND_REQUEST_RATE_LIMITER.lock() {
        limiter.counts.clear();
        limiter.day_key = chrono::Utc::now().format("%Y-%m-%d").to_string();
    }
}

#[cfg(test)]
static TEST_ENV_LOCK: std::sync::LazyLock<std::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(()));

#[cfg(test)]
pub(crate) fn social_service_test_env_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_ENV_LOCK.lock().expect("social service test env lock")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            unsafe {
                if let Some(value) = self.previous.as_ref() {
                    std::env::set_var(self.name, value);
                } else {
                    std::env::remove_var(self.name);
                }
            }
        }
    }

    #[test]
    fn friend_request_rate_limit_enforces_daily_cap_after_successful_records() {
        let _env_guard = crate::friend_request_rate_limit::social_service_test_env_lock();
        reset_friend_request_rate_limiter_for_tests();
        let _im_env = ScopedEnvVar::set("SDKWORK_IM_ENVIRONMENT", "test");
        let _limit = ScopedEnvVar::set(FRIEND_REQUEST_DAILY_LIMIT_ENV, "2");
        let tenant = "100001";
        let user = "u_rate_limit_test";

        check_friend_request_rate_allowed(tenant, "default", user, None).expect("first check");
        record_friend_request_submitted(tenant, user, false);
        check_friend_request_rate_allowed(tenant, "default", user, None).expect("second check");
        record_friend_request_submitted(tenant, user, false);
        let error = check_friend_request_rate_allowed(tenant, "default", user, None)
            .expect_err("third check should exceed daily cap");
        assert!(error.message.contains("daily limit exceeded"));
        assert!(error.retry_after_seconds > 0);
    }

    #[test]
    fn failed_submission_does_not_consume_rate_limit_quota() {
        let _env_guard = crate::friend_request_rate_limit::social_service_test_env_lock();
        reset_friend_request_rate_limiter_for_tests();
        let _im_env = ScopedEnvVar::set("SDKWORK_IM_ENVIRONMENT", "test");
        let _limit = ScopedEnvVar::set(FRIEND_REQUEST_DAILY_LIMIT_ENV, "1");
        let tenant = "100001";
        let user = "u_rate_limit_no_record";

        check_friend_request_rate_allowed(tenant, "default", user, None).expect("first check should pass");
        check_friend_request_rate_allowed(tenant, "default", user, None)
            .expect("second check should still pass without record");
    }
}
