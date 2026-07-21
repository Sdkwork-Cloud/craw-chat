use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use fs4::fs_std::FileExt;
use im_platform_contracts::ContractError;
use serde::de::DeserializeOwned;

const MAX_JSON_STORE_BYTES: u64 = 32 * 1024 * 1024;

struct BoundedWriter<W> {
    inner: W,
    written: u64,
}

impl<W> BoundedWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner, written: 0 }
    }

    fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for BoundedWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        let next_size = self.written.saturating_add(buffer.len() as u64);
        if next_size > MAX_JSON_STORE_BYTES {
            return Err(std::io::Error::other(format!(
                "JSON store exceeds {MAX_JSON_STORE_BYTES} byte limit"
            )));
        }
        let written = self.inner.write(buffer)?;
        self.written = self.written.saturating_add(written as u64);
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub(super) fn read_json_records_or_default<T>(
    file_path: &Path,
    store_name: &str,
) -> Result<T, ContractError>
where
    T: DeserializeOwned + Default,
{
    with_store_file_lock(file_path, store_name, || {
        read_json_records_or_default_unlocked(file_path, store_name)
    })
}

pub(super) fn scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    im_platform_contracts::realtime_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    )
}

pub(super) fn scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

pub(super) fn parse_scope_key_parts(key: &str) -> Option<Vec<String>> {
    let mut parts = Vec::new();
    let bytes = key.as_bytes();
    let mut offset = 0;
    while offset < key.len() {
        let len_start = offset;
        while offset < key.len() && bytes[offset] != b':' {
            if !bytes[offset].is_ascii_digit() {
                return None;
            }
            offset += 1;
        }
        if offset == len_start || offset >= key.len() {
            return None;
        }
        let len = key[len_start..offset].parse::<usize>().ok()?;
        offset += 1;
        let value_end = offset.checked_add(len)?;
        if value_end > key.len() {
            return None;
        }
        parts.push(key[offset..value_end].to_owned());
        offset = value_end;
        if offset == key.len() {
            break;
        }
        if bytes[offset] != b'|' {
            return None;
        }
        offset += 1;
        if offset == key.len() {
            return None;
        }
    }
    Some(parts)
}

pub(super) fn notification_scope_key(
    tenant_id: &str,
    organization_id: &str,
    notification_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, organization_id, notification_id])
}

pub(super) fn notification_recipient_scope_key(
    tenant_id: &str,
    organization_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, organization_id, recipient_kind, recipient_id])
}

pub(super) fn principal_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    im_platform_contracts::realtime_principal_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
    )
}

pub(super) fn execution_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    scope_key_parts(&[
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        execution_id,
    ])
}

fn recover_pending_json_temp_file(file_path: &Path, store_name: &str) -> Result<(), ContractError> {
    let temp_path = temp_json_path(file_path);
    if !temp_path.exists() {
        return Ok(());
    }

    if file_path.exists() {
        return fs::remove_file(&temp_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to discard stale {store_name} temp file {}: {error}",
                temp_path.display()
            ))
        });
    }

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to recover {store_name} from temp file {} to {}: {error}",
            temp_path.display(),
            file_path.display()
        ))
    })
}

fn temp_json_path(file_path: &Path) -> PathBuf {
    file_path.with_extension("json.tmp")
}

fn lock_json_path(file_path: &Path) -> PathBuf {
    let mut file_name = file_path
        .file_name()
        .map(|value| value.to_os_string())
        .unwrap_or_else(|| OsString::from("store"));
    file_name.push(".lock");
    file_path
        .parent()
        .map(|parent| parent.join(&file_name))
        .unwrap_or_else(|| PathBuf::from(file_name))
}

pub(super) fn with_store_file_lock<T>(
    file_path: &Path,
    store_name: &str,
    operation: impl FnOnce() -> Result<T, ContractError>,
) -> Result<T, ContractError> {
    let lock_path = lock_json_path(file_path);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to create {store_name} lock dir {}: {error}",
                parent.display()
            ))
        })?;
    }
    let lock_file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to open {store_name} lock {}: {error}",
                lock_path.display()
            ))
        })?;
    lock_file.lock_exclusive().map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to lock {store_name} {}: {error}",
            lock_path.display()
        ))
    })?;

    let result = operation();
    if let Err(error) = lock_file.unlock() {
        return result.and_then(|_| {
            Err(ContractError::Unavailable(format!(
                "failed to unlock {store_name} {}: {error}",
                lock_path.display()
            )))
        });
    }

    result
}

pub(super) fn read_json_records_or_default_unlocked<T>(
    file_path: &Path,
    store_name: &str,
) -> Result<T, ContractError>
where
    T: DeserializeOwned + Default,
{
    recover_pending_json_temp_file(file_path, store_name)?;

    if !file_path.exists() {
        return Ok(T::default());
    }

    let file = File::open(file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    let file_size = file
        .metadata()
        .map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to inspect {store_name} {}: {error}",
                file_path.display()
            ))
        })?
        .len();
    if file_size > MAX_JSON_STORE_BYTES {
        return Err(ContractError::Unavailable(format!(
            "{store_name} {} exceeds {MAX_JSON_STORE_BYTES} byte limit",
            file_path.display()
        )));
    }
    let initial_capacity = usize::try_from(file_size).unwrap_or(0);
    let mut bytes = Vec::with_capacity(initial_capacity);
    file.take(MAX_JSON_STORE_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read {store_name} {}: {error}",
                file_path.display()
            ))
        })?;
    if bytes.len() as u64 > MAX_JSON_STORE_BYTES {
        return Err(ContractError::Unavailable(format!(
            "{store_name} {} grew beyond {MAX_JSON_STORE_BYTES} byte limit while reading",
            file_path.display()
        )));
    }
    if bytes.is_empty() {
        return Ok(T::default());
    }

    serde_json::from_slice(&bytes).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to parse {store_name} {}: {error}",
            file_path.display()
        ))
    })
}

pub(super) fn write_json_records_unlocked<T: serde::Serialize + ?Sized>(
    file_path: &Path,
    records: &T,
    store_name: &str,
) -> Result<(), ContractError> {
    let parent = file_path.parent().ok_or_else(|| {
        ContractError::Unavailable(format!(
            "{store_name} path has no parent: {}",
            file_path.display()
        ))
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} dir {}: {error}",
            parent.display()
        ))
    })?;

    let temp_path = temp_json_path(file_path);
    if temp_path.exists() {
        fs::remove_file(&temp_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to clear stale {store_name} temp file {}: {error}",
                temp_path.display()
            ))
        })?;
    }

    let temp_file = File::create(&temp_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    let mut writer = BoundedWriter::new(temp_file);
    if let Err(error) = serde_json::to_writer_pretty(&mut writer, records) {
        drop(writer);
        let _ = fs::remove_file(&temp_path);
        return Err(ContractError::Unavailable(format!(
            "failed to serialize bounded {store_name} {}: {error}",
            file_path.display()
        )));
    }
    if let Err(error) = writer.flush() {
        drop(writer);
        let _ = fs::remove_file(&temp_path);
        return Err(ContractError::Unavailable(format!(
            "failed to flush {store_name} temp file {}: {error}",
            temp_path.display()
        )));
    }
    let temp_file = writer.into_inner();
    if let Err(error) = temp_file.sync_all() {
        drop(temp_file);
        let _ = fs::remove_file(&temp_path);
        return Err(ContractError::Unavailable(format!(
            "failed to sync {store_name} temp file {}: {error}",
            temp_path.display()
        )));
    }
    drop(temp_file);

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to finalize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    Ok(())
}

pub(super) fn write_json_records<T: serde::Serialize + ?Sized>(
    file_path: &Path,
    records: &T,
    store_name: &str,
) -> Result<(), ContractError> {
    with_store_file_lock(file_path, store_name, || {
        write_json_records_unlocked(file_path, records, store_name)
    })
}

pub(super) fn update_json_records<T, R>(
    file_path: &Path,
    store_name: &str,
    apply: impl FnOnce(&mut T) -> R,
) -> Result<R, ContractError>
where
    T: DeserializeOwned + Default + serde::Serialize,
{
    with_store_file_lock(file_path, store_name, || {
        let mut records = read_json_records_or_default_unlocked(file_path, store_name)?;
        let result = apply(&mut records);
        write_json_records_unlocked(file_path, &records, store_name)?;
        Ok(result)
    })
}

pub(super) fn try_update_json_records<T, R>(
    file_path: &Path,
    store_name: &str,
    apply: impl FnOnce(&mut T) -> Result<R, ContractError>,
) -> Result<R, ContractError>
where
    T: DeserializeOwned + Default + serde::Serialize,
{
    with_store_file_lock(file_path, store_name, || {
        let mut records = read_json_records_or_default_unlocked(file_path, store_name)?;
        let result = apply(&mut records)?;
        write_json_records_unlocked(file_path, &records, store_name)?;
        Ok(result)
    })
}
