use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::ContractError;
use im_storage_contracts::{StorageDomainSnapshot, StorageDomainSnapshotStore};

use crate::shared::{read_json_records_or_default, write_json_records};

#[derive(Clone, Debug)]
pub struct FileStorageDomainSnapshotStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileStorageDomainSnapshotStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("storage domain snapshot file store lock should lock");
        Ok(self.read_records()?.get(domain).cloned())
    }

    fn read_records(&self) -> Result<BTreeMap<String, StorageDomainSnapshot>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "storage domain snapshot store")
    }

    fn write_records(
        &self,
        records: &BTreeMap<String, StorageDomainSnapshot>,
    ) -> Result<(), ContractError> {
        write_json_records(
            self.file_path.as_path(),
            records,
            "storage domain snapshot store",
        )
    }
}

impl StorageDomainSnapshotStore for FileStorageDomainSnapshotStore {
    fn load_snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        self.snapshot(domain)
    }

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("storage domain snapshot file store lock should lock");
        let mut records = self.read_records()?;
        records.insert(snapshot.catalog.domain.clone(), snapshot);
        self.write_records(&records)
    }
}

pub fn validate_storage_domain_snapshot_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, StorageDomainSnapshot> =
        read_json_records_or_default(file_path.as_ref(), "storage domain snapshot store")?;
    Ok(())
}
