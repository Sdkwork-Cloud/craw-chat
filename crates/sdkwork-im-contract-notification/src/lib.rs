use im_domain_core::notification::NotificationTask;
use im_time::{max_optional_rfc3339_string, max_rfc3339_string, rfc3339_cmp};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationTaskRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub notification_id: String,
    pub task: NotificationTask,
    pub updated_at: String,
    /// Delivery attempt count (worker metadata; 0 for fresh requests).
    #[serde(default)]
    pub attempt_count: u32,
    /// Lease/retry gate instant (RFC 3339). Tasks become claimable when the
    /// current time reaches this value.
    #[serde(default = "default_available_at")]
    pub available_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationTaskListCursor {
    pub updated_at: String,
    pub notification_id: String,
}

/// Typed delivery-worker claim request.
///
/// Carries the batch limit and the worker clock instant so every
/// cross-organization notification claim is attributable and audit-logged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobalNotificationTaskClaimRequest {
    pub limit: usize,
    pub now: String,
}

impl GlobalNotificationTaskClaimRequest {
    pub fn try_new(limit: usize, now: String) -> Result<Self, ContractError> {
        if limit == 0 {
            return Err(ContractError::Invalid(
                "notification worker claim limit must be positive".into(),
            ));
        }
        if now.trim().is_empty() {
            return Err(ContractError::Invalid(
                "notification worker claim clock must be a non-empty RFC 3339 instant".into(),
            ));
        }
        Ok(Self { limit, now })
    }
}

impl NotificationTaskRecord {
    pub fn merge_monotonic(self, next: Self) -> Self {
        let mut selected = if notification_task_record_precedes(&self, &next) {
            next.clone()
        } else {
            self.clone()
        };

        selected.updated_at = max_rfc3339_string(self.updated_at, next.updated_at);
        selected.task.dispatched_at = max_optional_rfc3339_string(
            selected.task.dispatched_at,
            max_optional_rfc3339_string(self.task.dispatched_at, next.task.dispatched_at),
        );
        if selected.task.failure_reason.is_none() {
            selected.task.failure_reason = self.task.failure_reason.or(next.task.failure_reason);
        }
        selected
    }
}

pub trait NotificationTaskStore: Send + Sync {
    fn load_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError>;

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError>;

    fn list_tasks_for_recipient_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        cursor: Option<&NotificationTaskListCursor>,
        page_size: usize,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError>;

    /// Claims up to `limit` requested notification tasks whose lease has
    /// expired (`available_at <= now`). Implementations MUST serialize
    /// concurrent claims (`FOR UPDATE SKIP LOCKED` in PostgreSQL) so no task
    /// is claimed by two workers. Claimed tasks receive a fresh lease.
    fn claim_tasks(
        &self,
        limit: usize,
        now: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError>;

    /// Marks a claimed task as dispatched. A task that is no longer
    /// `requested` (already dispatched or failed) is a no-op success so
    /// duplicate worker completions never regress state.
    fn complete_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
        dispatched_at: &str,
    ) -> Result<(), ContractError>;

    /// Records a delivery failure: increments the attempt count, applies
    /// exponential backoff to `available_at`, and dead-letters the task as
    /// `failed` once attempts reach the store's configured maximum.
    fn fail_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
        failure_reason: &str,
        now: &str,
    ) -> Result<(), ContractError>;
}

fn default_organization_id() -> String {
    "0".to_owned()
}

fn default_available_at() -> String {
    im_time::utc_now_rfc3339_millis()
}

/// Env override for the delivery dead-letter attempt cap.
pub const NOTIFICATION_MAX_ATTEMPTS_ENV: &str = "SDKWORK_IM_NOTIFICATION_MAX_ATTEMPTS";
pub const NOTIFICATION_MAX_ATTEMPTS_DEFAULT: u32 = 10;
/// Env override for the claim lease duration in seconds.
pub const NOTIFICATION_CLAIM_LEASE_SECS_ENV: &str = "SDKWORK_IM_NOTIFICATION_CLAIM_LEASE_SECS";
pub const NOTIFICATION_CLAIM_LEASE_SECS_DEFAULT: i64 = 60;
/// Backoff ceiling (seconds) shared by every notification task store.
const NOTIFICATION_BACKOFF_MAX_SECS: i64 = 300;

/// Delivery attempt cap shared by every `NotificationTaskStore` worker.
pub fn resolve_notification_max_attempts() -> u32 {
    std::env::var(NOTIFICATION_MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_MAX_ATTEMPTS_DEFAULT)
}

/// Claim lease duration in seconds shared by every `NotificationTaskStore`
/// worker. A claimed task whose lease expires becomes claimable again.
pub fn resolve_notification_claim_lease_secs() -> i64 {
    std::env::var(NOTIFICATION_CLAIM_LEASE_SECS_ENV)
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_CLAIM_LEASE_SECS_DEFAULT)
}

/// Exponential backoff in seconds for the next delivery attempt
/// (`min(300, 2^min(attempt_count, 8))`), matching the durable outbox policy.
pub fn notification_backoff_secs(attempt_count: u32) -> i64 {
    let exponent = attempt_count.min(8);
    let secs = 1i64.checked_shl(exponent).unwrap_or(i64::MAX);
    secs.min(NOTIFICATION_BACKOFF_MAX_SECS)
}

fn notification_task_record_precedes(
    left: &NotificationTaskRecord,
    right: &NotificationTaskRecord,
) -> bool {
    notification_status_group_rank(&left.task.status)
        .cmp(&notification_status_group_rank(&right.task.status))
        .then_with(|| rfc3339_cmp(left.updated_at.as_str(), right.updated_at.as_str()))
        .then_with(|| {
            notification_status_tie_rank(&left.task.status)
                .cmp(&notification_status_tie_rank(&right.task.status))
        })
        .is_lt()
}

fn notification_status_group_rank(status: &im_domain_core::notification::NotificationStatus) -> u8 {
    match status {
        im_domain_core::notification::NotificationStatus::Requested => 0,
        im_domain_core::notification::NotificationStatus::Dispatched
        | im_domain_core::notification::NotificationStatus::Failed => 1,
    }
}

fn notification_status_tie_rank(status: &im_domain_core::notification::NotificationStatus) -> u8 {
    match status {
        im_domain_core::notification::NotificationStatus::Requested => 0,
        im_domain_core::notification::NotificationStatus::Dispatched => 1,
        im_domain_core::notification::NotificationStatus::Failed => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::notification::{NotificationStatus, NotificationTask};

    fn notification_task_record(
        status: NotificationStatus,
        dispatched_at: Option<&str>,
        failure_reason: Option<&str>,
        updated_at: &str,
    ) -> NotificationTaskRecord {
        NotificationTaskRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            notification_id: "ntf_demo".into(),
            task: NotificationTask {
                tenant_id: "100001".into(),
                notification_id: "ntf_demo".into(),
                source_event_id: "evt_demo".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1".into(),
                recipient_kind: "user".into(),
                status,
                title: Some("hello".into()),
                body: Some("world".into()),
                payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                requested_at: "2026-05-06T00:00:00.000Z".into(),
                dispatched_at: dispatched_at.map(str::to_owned),
                failure_reason: failure_reason.map(str::to_owned),
            },
            updated_at: updated_at.into(),
            attempt_count: 0,
            available_at: "2026-05-06T00:00:00.000Z".into(),
        }
    }

    #[test]
    fn test_notification_task_record_merge_rejects_stale_status_regression() {
        let current = notification_task_record(
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        );
        let stale = notification_task_record(
            NotificationStatus::Requested,
            None,
            None,
            "2026-05-06T00:00:01.000Z",
        );

        let merged = current.merge_monotonic(stale);

        assert_eq!(merged.task.status, NotificationStatus::Dispatched);
        assert_eq!(
            merged.task.dispatched_at.as_deref(),
            Some("2026-05-06T00:00:02.000Z")
        );
        assert_eq!(merged.updated_at, "2026-05-06T00:00:02.000Z");
    }

    #[test]
    fn test_notification_task_record_merge_compares_rfc3339_by_instant() {
        let whole_second = notification_task_record(
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:00Z"),
            None,
            "2026-05-06T00:00:00Z",
        );
        let later_fraction = notification_task_record(
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:00.100Z"),
            None,
            "2026-05-06T00:00:00.100Z",
        );

        let merged = whole_second.merge_monotonic(later_fraction);

        assert_eq!(merged.task.status, NotificationStatus::Dispatched);
        assert_eq!(
            merged.task.dispatched_at.as_deref(),
            Some("2026-05-06T00:00:00.100Z")
        );
        assert_eq!(merged.updated_at, "2026-05-06T00:00:00.100Z");
    }
}
