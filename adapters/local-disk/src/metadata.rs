use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{ContractError, MetadataSnapshotRecord, MetadataStore};

use crate::shared::{
    parse_scope_key_parts, read_json_records_or_default, scope_key_parts, update_json_records,
};

#[derive(Clone, Debug)]
pub struct FileMetadataStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileMetadataStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn snapshot(&self, scope: &str, key: &str) -> Option<String> {
        let _guard = self
            .io_lock
            .lock()
            .expect("metadata file store lock should lock");
        self.read_records()
            .expect("metadata store should parse")
            .get(snapshot_key(scope, key).as_str())
            .cloned()
    }

    pub fn scopes_for_key(&self, key: &str) -> Vec<String> {
        let _guard = self
            .io_lock
            .lock()
            .expect("metadata file store lock should lock");
        let mut scopes = self
            .read_records()
            .expect("metadata store should parse")
            .keys()
            .filter_map(|stored_key| {
                let parts = parse_scope_key_parts(stored_key)?;
                if parts.len() == 2 && parts[1] == key {
                    Some(parts[0].clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        scopes.sort();
        scopes.dedup();
        scopes
    }

    fn read_records(&self) -> Result<BTreeMap<String, String>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "metadata store")
    }
}

impl MetadataStore for FileMetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("metadata file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "metadata store",
            |records: &mut BTreeMap<String, String>| {
                records.insert(snapshot_key(scope, key), value.to_string());
            },
        )
    }

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
        Ok(self.snapshot(scope, key))
    }

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("metadata file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "metadata store",
            |records: &mut BTreeMap<String, String>| {
                for snapshot in snapshots {
                    records.insert(
                        snapshot_key(snapshot.scope.as_str(), snapshot.key.as_str()),
                        snapshot.value.clone(),
                    );
                }
            },
        )
    }
}

pub fn validate_metadata_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, String> =
        read_json_records_or_default(file_path.as_ref(), "metadata store")?;
    Ok(())
}

fn snapshot_key(scope: &str, key: &str) -> String {
    scope_key_parts(&[scope, key])
}
