use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Unbounded};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, ContractError, NotificationTaskListCursor,
    NotificationTaskRecord, NotificationTaskStore,
};

use crate::shared::{
    execution_scope_key, notification_recipient_scope_key, notification_scope_key,
    read_json_records_or_default, try_update_json_records,
};

const MAX_FILE_STORE_RECORDS: usize = 50_000;

fn lock_file_store(io_lock: &Mutex<()>) -> MutexGuard<'_, ()> {
    match io_lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct PersistedNotificationTaskRecords {
    by_notification: BTreeMap<String, NotificationTaskRecord>,
    tasks_by_recipient: BTreeMap<String, BTreeSet<PersistedNotificationTaskSortEntry>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
struct PersistedNotificationTaskSortEntry {
    sort_key: Reverse<(String, String)>,
    notification_key: String,
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
        let records: PersistedNotificationTaskRecords =
            read_json_records_or_default(self.file_path.as_path(), "notification task store")?;
        validate_notification_record_capacity(&records)?;
        Ok(records)
    }
}

impl NotificationTaskStore for FileNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let _guard = lock_file_store(&self.io_lock);
        Ok(self
            .read_records()?
            .by_notification
            .remove(notification_scope_key(tenant_id, organization_id, notification_id).as_str()))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let _guard = lock_file_store(&self.io_lock);
        let key = notification_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.notification_id.as_str(),
        );
        try_update_json_records(
            self.file_path.as_path(),
            "notification task store",
            move |records: &mut PersistedNotificationTaskRecords| {
                validate_notification_record_capacity(records)?;
                if !records.by_notification.contains_key(key.as_str())
                    && records.by_notification.len() >= MAX_FILE_STORE_RECORDS
                {
                    return Err(ContractError::Unavailable(format!(
                        "notification task file store exceeds {MAX_FILE_STORE_RECORDS} record limit"
                    )));
                }
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
                validate_notification_record_capacity(records)
            },
        )
    }

    fn list_tasks_for_recipient_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        cursor: Option<&NotificationTaskListCursor>,
        page_size: usize,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let _guard = lock_file_store(&self.io_lock);
        let records = self.read_records()?;
        let entries = records.tasks_by_recipient.get(
            notification_recipient_scope_key(
                tenant_id,
                organization_id,
                recipient_kind,
                recipient_id,
            )
            .as_str(),
        );
        let Some(entries) = entries else {
            return Ok(Vec::new());
        };
        let cursor_entry = cursor.map(|value| PersistedNotificationTaskSortEntry {
            sort_key: Reverse((value.updated_at.clone(), value.notification_id.clone())),
            notification_key: notification_scope_key(
                tenant_id,
                organization_id,
                value.notification_id.as_str(),
            ),
        });
        let iter: Box<dyn Iterator<Item = &PersistedNotificationTaskSortEntry> + '_> =
            match cursor_entry.as_ref() {
                Some(entry) => Box::new(entries.range((Excluded(entry), Unbounded))),
                None => Box::new(entries.iter()),
            };
        Ok(iter
            .take(page_size.saturating_add(1))
            .filter_map(|entry| {
                records
                    .by_notification
                    .get(entry.notification_key.as_str())
                    .cloned()
            })
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
        let records: BTreeMap<String, AutomationExecutionRecord> =
            read_json_records_or_default(self.file_path.as_path(), "automation execution store")?;
        validate_automation_record_capacity(&records)?;
        Ok(records)
    }
}

impl AutomationExecutionStore for FileAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        let _guard = lock_file_store(&self.io_lock);
        let mut records = self.read_records()?;
        if let Some(record) = records.remove(
            execution_scope_key(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                execution_id,
            )
            .as_str(),
        ) {
            return Ok(Some(record));
        }

        Ok(None)
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let _guard = lock_file_store(&self.io_lock);
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        try_update_json_records(
            self.file_path.as_path(),
            "automation execution store",
            move |records: &mut BTreeMap<String, AutomationExecutionRecord>| {
                validate_automation_record_capacity(records)?;
                if !records.contains_key(key.as_str()) && records.len() >= MAX_FILE_STORE_RECORDS {
                    return Err(ContractError::Unavailable(format!(
                        "automation execution file store exceeds {MAX_FILE_STORE_RECORDS} record limit"
                    )));
                }
                let next = records
                    .remove(key.as_str())
                    .map(|previous| previous.merge_monotonic(record.clone()))
                    .unwrap_or(record);
                records.insert(key, next);
                validate_automation_record_capacity(records)
            },
        )
    }
}

pub fn validate_notification_task_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let records: PersistedNotificationTaskRecords =
        read_json_records_or_default(file_path.as_ref(), "notification task store")?;
    validate_notification_record_capacity(&records)
}

pub fn validate_automation_execution_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let records: BTreeMap<String, AutomationExecutionRecord> =
        read_json_records_or_default(file_path.as_ref(), "automation execution store")?;
    validate_automation_record_capacity(&records)
}

fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

fn insert_notification_recipient_index(
    index: &mut BTreeMap<String, BTreeSet<PersistedNotificationTaskSortEntry>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(PersistedNotificationTaskSortEntry {
            sort_key: Reverse((record.updated_at.clone(), record.notification_id.clone())),
            notification_key: notification_key.to_owned(),
        });
}

fn remove_notification_recipient_index(
    index: &mut BTreeMap<String, BTreeSet<PersistedNotificationTaskSortEntry>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(&PersistedNotificationTaskSortEntry {
            sort_key: Reverse((record.updated_at.clone(), record.notification_id.clone())),
            notification_key: notification_key.to_owned(),
        });
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

fn validate_notification_record_capacity(
    records: &PersistedNotificationTaskRecords,
) -> Result<(), ContractError> {
    let index_entries = records
        .tasks_by_recipient
        .values()
        .try_fold(0_usize, |count, entries| count.checked_add(entries.len()))
        .ok_or_else(|| {
            ContractError::Unavailable("notification task file index size overflow".into())
        })?;
    if records.by_notification.len() > MAX_FILE_STORE_RECORDS
        || index_entries > MAX_FILE_STORE_RECORDS
    {
        return Err(ContractError::Unavailable(format!(
            "notification task file store exceeds {MAX_FILE_STORE_RECORDS} record limit"
        )));
    }
    Ok(())
}

fn validate_automation_record_capacity(
    records: &BTreeMap<String, AutomationExecutionRecord>,
) -> Result<(), ContractError> {
    if records.len() > MAX_FILE_STORE_RECORDS {
        return Err(ContractError::Unavailable(format!(
            "automation execution file store exceeds {MAX_FILE_STORE_RECORDS} record limit"
        )));
    }
    Ok(())
}
