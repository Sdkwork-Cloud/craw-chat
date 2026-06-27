#[test]
fn test_local_disk_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "adapters/local-disk/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_local_disk_state_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileStreamStateStore {",
        "pub struct FilePresenceStateStore {",
        "impl StreamStateStore for FileStreamStateStore {",
        "impl PresenceStateStore for FilePresenceStateStore {",
        "pub fn validate_stream_state_store_file(",
        "pub fn validate_presence_state_store_file(",
        "fn stream_scope_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep state-store symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_disk_ops_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileNotificationTaskStore {",
        "pub struct FileAutomationExecutionStore {",
        "impl NotificationTaskStore for FileNotificationTaskStore {",
        "impl AutomationExecutionStore for FileAutomationExecutionStore {",
        "pub fn validate_notification_task_store_file(",
        "pub fn validate_automation_execution_store_file(",
        "fn notification_scope_key(",
        "fn execution_scope_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep ops-store symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_disk_does_not_own_rtc_state_store_surface() {
    let state_source = include_str!("../src/state.rs");
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileRtcStateStore",
        "impl RtcStateStore for FileRtcStateStore",
        "pub fn validate_rtc_state_store_file",
        "RtcStateRecord",
        "RtcStateStore",
    ] {
        assert!(
            !state_source.contains(forbidden_symbol) && !lib_source.contains(forbidden_symbol),
            "RTC state store must be owned by sdkwork-rtc, not local-disk: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_disk_presence_state_store_uses_persisted_indexes_for_hot_paths() {
    let source = include_str!("../src/state.rs").replace("\r\n", "\n");

    assert!(
        source.contains("struct PersistedPresenceStateRecords {"),
        "local disk presence store should persist an indexed state object, not a flat record map"
    );
    assert!(
        source.contains("by_device: BTreeMap<String, PresenceStateRecord>"),
        "local disk presence store should keep device records by encoded device key"
    );
    assert!(
        source.contains("presence_by_principal: BTreeMap<String, BTreeSet<String>>"),
        "local disk presence store should persist a tenant/principal-kind/principal-id -> device-key index"
    );
    assert!(
        source.contains("online_by_seen_at: BTreeMap<String, BTreeSet<String>>"),
        "local disk presence store should persist a last-seen -> device-key index for expiration jobs"
    );
    assert!(
        !source.contains("read_records()?\n            .into_values()\n            .filter"),
        "local disk presence hot-path listings must not full-scan all persisted device records"
    );
}

#[test]
fn test_local_disk_notification_task_store_uses_persisted_recipient_index() {
    let source = include_str!("../src/ops.rs").replace("\r\n", "\n");

    assert!(
        source.contains("struct PersistedNotificationTaskRecords {"),
        "local disk notification task store should persist an indexed state object, not a flat record map"
    );
    assert!(
        source.contains("by_notification: BTreeMap<String, NotificationTaskRecord>"),
        "local disk notification task store should keep tasks by encoded notification key"
    );
    assert!(
        source.contains("tasks_by_recipient: BTreeMap<String, BTreeSet<String>>"),
        "local disk notification task store should persist a tenant/recipient-kind/recipient-id -> notification-key index"
    );
    assert!(
        source.contains("notification_recipient_scope_key("),
        "notification task index must include recipient_kind in its scope key"
    );
    assert!(
        !source.contains("read_records()?\n            .into_values()\n            .filter"),
        "local disk notification recipient listing must not full-scan all persisted tasks"
    );
}

#[test]
fn test_local_disk_metadata_and_projection_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileMetadataStore {",
        "pub struct FileTimelineProjectionStore {",
        "impl MetadataStore for FileMetadataStore {",
        "impl TimelineProjectionStore for FileTimelineProjectionStore {",
        "pub fn validate_metadata_store_file(",
        "pub fn validate_timeline_projection_store_file(",
        "fn snapshot_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep metadata/projection-store symbol: {forbidden_symbol}"
        );
    }
}
