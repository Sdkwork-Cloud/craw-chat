use im_domain_core::notification::NotificationTask;
use im_time::{max_optional_rfc3339_string, max_rfc3339_string, rfc3339_cmp};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationTaskRecord {
    pub tenant_id: String,
    pub notification_id: String,
    pub task: NotificationTask,
    pub updated_at: String,
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
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError>;

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError>;

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError>;
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
