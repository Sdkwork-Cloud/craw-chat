use std::panic::{self, AssertUnwindSafe};
use std::sync::Mutex;

use im_app_context::AppContext;
use im_domain_core::notification::NotificationStatus;
use sdkwork_im_contract_notification::{NotificationTaskRecord, NotificationTaskStore};

use super::*;
use crate::state::RuntimeMemoryNotificationTaskStore;

fn notification_task_record(
    notification_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
    status: NotificationStatus,
    dispatched_at: Option<&str>,
    failure_reason: Option<&str>,
    updated_at: &str,
) -> NotificationTaskRecord {
    NotificationTaskRecord {
        tenant_id: "100001".into(),
        notification_id: notification_id.into(),
        task: NotificationTask {
            tenant_id: "100001".into(),
            notification_id: notification_id.into(),
            source_event_id: format!("evt_{notification_id}"),
            source_event_type: "message.posted".into(),
            category: "message.new".into(),
            channel: "inapp".into(),
            recipient_id: recipient_id.into(),
            recipient_kind: recipient_kind.to_owned(),
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

fn demo_auth_context() -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        device_id: Some("d_demo".into()),
    }
}

fn poison_mutex<T>(mutex: &Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

#[test]
fn test_notification_runtime_uses_recipient_index_for_listing() {
    let source = include_str!("state.rs").replace("\r\n", "\n");

    assert!(
        source.contains("tasks_by_recipient: HashMap<String, BTreeSet<String>>"),
        "notification runtime should maintain a tenant/recipient-kind/recipient-id task index"
    );
    assert!(
        source.contains("notification_keys_for_recipient("),
        "list_notifications should read notification keys from the runtime recipient index"
    );
    assert!(
        !source.contains(".iter()\n            .filter(|(key, task)| {\n                key.starts_with(prefix.as_str()) && notification_visible_to_actor(task, auth)\n            })"),
        "list_notifications must not full-scan the runtime task cache"
    );
}

#[test]
fn test_list_notifications_recovers_from_poisoned_tasks_lock() {
    let runtime = NotificationRuntime::default();
    let auth = demo_auth_context();
    poison_mutex(&runtime.tasks);

    let result = panic::catch_unwind(AssertUnwindSafe(|| runtime.list_notifications(&auth)));
    assert!(
        result.is_ok(),
        "list_notifications should not panic when tasks lock is poisoned"
    );
    let list_result = result.expect("panic status should be captured");
    assert!(
        list_result.is_ok(),
        "list_notifications should recover from poisoned tasks lock"
    );
}

#[test]
fn test_get_notification_recovers_from_poisoned_tasks_lock() {
    let runtime = NotificationRuntime::default();
    let auth = demo_auth_context();
    poison_mutex(&runtime.tasks);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        runtime.get_notification(&auth, "ntf_missing")
    }));
    assert!(
        result.is_ok(),
        "get_notification should not panic when tasks lock is poisoned"
    );
    let get_result = result.expect("panic status should be captured");
    assert!(
        get_result.is_err(),
        "get_notification should return controlled error after lock recovery"
    );
}

#[test]
fn test_list_notifications_recovers_from_poisoned_restored_recipients_lock() {
    let runtime = NotificationRuntime::default();
    let auth = demo_auth_context();
    poison_mutex(&runtime.restored_recipients);

    let result = panic::catch_unwind(AssertUnwindSafe(|| runtime.list_notifications(&auth)));
    assert!(
        result.is_ok(),
        "list_notifications should not panic when restored-recipient lock is poisoned"
    );
    let list_result = result.expect("panic status should be captured");
    assert!(
        list_result.is_ok(),
        "list_notifications should recover from poisoned restored-recipient lock"
    );
}

#[test]
fn test_runtime_memory_task_store_load_recovers_from_poisoned_lock() {
    let store = RuntimeMemoryNotificationTaskStore::default();
    poison_mutex(&store.state);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        store.load_task("100001", "ntf_poison_store")
    }));
    assert!(
        result.is_ok(),
        "notification task store load should not panic when lock is poisoned"
    );
    let load_result = result.expect("panic status should be captured");
    assert!(
        load_result.is_ok(),
        "notification task store load should recover from poisoned lock"
    );
}

#[test]
fn test_runtime_memory_task_store_uses_recipient_kind_index_for_listing() {
    let source = include_str!("state.rs").replace("\r\n", "\n");

    assert!(
        source.contains("tasks_by_recipient: HashMap<String, BTreeSet<String>>"),
        "runtime notification task store should maintain a tenant/recipient-kind/recipient-id index"
    );
    assert!(
        source.contains("notification_recipient_scope_key("),
        "runtime notification task store should include recipient_kind in its index key"
    );
}

#[test]
fn test_runtime_memory_task_store_lists_only_matching_recipient_kind() {
    let store = RuntimeMemoryNotificationTaskStore::default();
    store
        .save_task(notification_task_record(
            "ntf_user",
            "user",
            "shared_id",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("user notification save should succeed");
    store
        .save_task(notification_task_record(
            "ntf_system",
            "system",
            "shared_id",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:03.000Z"),
            None,
            "2026-05-06T00:00:03.000Z",
        ))
        .expect("system notification save should succeed");

    let listed = store
        .list_tasks_for_recipient("100001", "user", "shared_id")
        .expect("recipient listing should succeed");

    assert_eq!(
        listed
            .iter()
            .map(|record| record.notification_id.as_str())
            .collect::<Vec<_>>(),
        vec!["ntf_user"]
    );
}

#[test]
fn test_runtime_memory_task_store_rejects_stale_status_regression_writes() {
    let store = RuntimeMemoryNotificationTaskStore::default();
    store
        .save_task(notification_task_record(
            "ntf_demo",
            "user",
            "1",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("current notification save should succeed");
    store
        .save_task(notification_task_record(
            "ntf_demo",
            "user",
            "1",
            NotificationStatus::Requested,
            None,
            None,
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("stale notification save should not fail the caller");

    let restored = store
        .load_task("100001", "ntf_demo")
        .expect("notification load should succeed")
        .expect("notification should be present");
    assert_eq!(restored.task.status, NotificationStatus::Dispatched);
    assert_eq!(
        restored.task.dispatched_at.as_deref(),
        Some("2026-05-06T00:00:02.000Z")
    );
    assert_eq!(restored.updated_at, "2026-05-06T00:00:02.000Z");
}
