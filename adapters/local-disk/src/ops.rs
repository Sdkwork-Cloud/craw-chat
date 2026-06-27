use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, ContractError, NotificationTaskRecord,
    NotificationTaskStore,
};

use crate::shared::{
    execution_scope_key, notification_recipient_scope_key, notification_scope_key,
    read_json_records_or_default, update_json_records,
};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct PersistedNotificationTaskRecords {
    by_notification: BTreeMap<String, NotificationTaskRecord>,
    tasks_by_recipient: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Clone, Debug)]
pub struct FileNotificationTaskStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileNotificationTaskStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<PersistedNotificationTaskRecords, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "notification task store")
    }
}

impl NotificationTaskStore for FileNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        Ok(self
            .read_records()?
            .by_notification
            .remove(notification_scope_key(tenant_id, notification_id).as_str()))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        let key =
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str());
        update_json_records(
            self.file_path.as_path(),
            "notification task store",
            move |records: &mut PersistedNotificationTaskRecords| {
                if let Some(previous) = records.by_notification.get(key.as_str()).cloned() {
                    remove_notification_recipient_index(
                        &mut records.tasks_by_recipient,
                        key.as_str(),
                        &previous,
                    );
                }
                let next = records
                    .by_notification
                    .remove(key.as_str())
                    .map(|previous| previous.merge_monotonic(record.clone()))
                    .unwrap_or(record);
                insert_notification_recipient_index(
                    &mut records.tasks_by_recipient,
                    key.as_str(),
                    &next,
                );
                records.by_notification.insert(key, next);
            },
        )
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("notification task file store lock should lock");
        let records = self.read_records()?;
        let task_keys = records
            .tasks_by_recipient
            .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
            .cloned()
            .unwrap_or_default();
        Ok(task_keys
            .into_iter()
            .filter_map(|task_key| records.by_notification.get(task_key.as_str()).cloned())
            .collect())
    }
}

#[derive(Clone, Debug)]
pub struct FileAutomationExecutionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileAutomationExecutionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, AutomationExecutionRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "automation execution store")
    }
}

impl AutomationExecutionStore for FileAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        let mut records = self.read_records()?;
        if let Some(record) = records.remove(
            execution_scope_key(tenant_id, principal_kind, principal_id, execution_id).as_str(),
        ) {
            return Ok(Some(record));
        }

        Ok(None)
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("automation execution file store lock should lock");
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        update_json_records(
            self.file_path.as_path(),
            "automation execution store",
            move |records: &mut BTreeMap<String, AutomationExecutionRecord>| {
                let next = records
                    .remove(key.as_str())
                    .map(|previous| previous.merge_monotonic(record.clone()))
                    .unwrap_or(record);
                records.insert(key, next);
            },
        )
    }
}

pub fn validate_notification_task_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: PersistedNotificationTaskRecords =
        read_json_records_or_default(file_path.as_ref(), "notification task store")?;
    Ok(())
}

pub fn validate_automation_execution_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, AutomationExecutionRecord> =
        read_json_records_or_default(file_path.as_ref(), "automation execution store")?;
    Ok(())
}

fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

fn insert_notification_recipient_index(
    index: &mut BTreeMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(notification_key.to_owned());
}

fn remove_notification_recipient_index(
    index: &mut BTreeMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}
