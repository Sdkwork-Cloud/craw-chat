use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    ContractError, TimelineProjectionBatch, TimelineProjectionRecord, TimelineProjectionStore,
};

use crate::shared::{read_json_records_or_default, update_json_records};

#[derive(Clone, Debug)]
pub struct FileTimelineProjectionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileTimelineProjectionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn entries(&self, tenant_id: &str, timeline_scope: &str) -> Vec<(u64, String)> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        self.read_records()
            .expect("timeline projection store should parse")
            .get(timeline_projection_scope_key(tenant_id, timeline_scope).as_str())
            .map(|items| {
                items
                    .iter()
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn read_records(&self) -> Result<BTreeMap<String, BTreeMap<u64, String>>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "timeline projection store")
    }
}

impl TimelineProjectionStore for FileTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "timeline projection store",
            |records: &mut BTreeMap<String, BTreeMap<u64, String>>| {
                records
                    .entry(timeline_projection_scope_key(tenant_id, timeline_scope))
                    .or_default()
                    .insert(message_seq, payload.to_string());
            },
        )
    }

    fn load_timeline(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(self.entries(tenant_id, timeline_scope))
    }

    fn upsert_timeline_entries(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "timeline projection store",
            |stored: &mut BTreeMap<String, BTreeMap<u64, String>>| {
                let scope_entries = stored
                    .entry(timeline_projection_scope_key(tenant_id, timeline_scope))
                    .or_default();
                for record in records {
                    scope_entries.insert(record.message_seq, record.payload.clone());
                }
            },
        )
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("timeline projection file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "timeline projection store",
            |stored: &mut BTreeMap<String, BTreeMap<u64, String>>| {
                for batch in batches {
                    let scope_entries = stored
                        .entry(timeline_projection_scope_key(
                            batch.tenant_id.as_str(),
                            batch.timeline_scope.as_str(),
                        ))
                        .or_default();
                    for record in &batch.records {
                        scope_entries.insert(record.message_seq, record.payload.clone());
                    }
                }
            },
        )
    }
}

pub fn validate_timeline_projection_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, BTreeMap<u64, String>> =
        read_json_records_or_default(file_path.as_ref(), "timeline projection store")?;
    Ok(())
}

fn timeline_projection_scope_key(tenant_id: &str, timeline_scope: &str) -> String {
    let mut encoded = String::new();
    for segment in [tenant_id, timeline_scope] {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}
