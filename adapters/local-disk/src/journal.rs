use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{CommitEnvelope, CommitJournal, CommitPosition, ContractError};
use serde::{Deserialize, Serialize};

use crate::shared::with_store_file_lock;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CommitJournalIndex {
    last_offset: u64,
    file_size_bytes: u64,
}

#[derive(Clone, Debug)]
pub struct FileCommitJournal {
    partition: Arc<String>,
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileCommitJournal {
    pub fn new(partition: impl Into<String>, file_path: impl Into<PathBuf>) -> Self {
        Self {
            partition: Arc::new(partition.into()),
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    pub fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        self.read_events()
    }

    fn read_events(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        with_store_file_lock(self.file_path.as_path(), "commit journal", || {
            self.read_events_unlocked()
        })
    }

    fn read_events_unlocked(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        read_commit_journal_json_lines_unlocked(self.file_path.as_path(), "commit journal")
    }

    fn append_events_unlocked(
        &self,
        envelopes: &[CommitEnvelope],
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Ok(Vec::new());
        }

        recover_pending_commit_journal_temp_file(self.file_path.as_path(), "commit journal")?;
        let parent = self.file_path.parent().ok_or_else(|| {
            ContractError::Unavailable(format!(
                "commit journal path has no parent: {}",
                self.file_path.display()
            ))
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to create commit journal dir {}: {error}",
                parent.display()
            ))
        })?;

        let current_offset =
            load_commit_journal_offset_unlocked(self.file_path.as_path(), "commit journal")?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.file_path.as_path())
            .map_err(|error| {
                ContractError::Unavailable(format!(
                    "failed to open append-only commit journal {}: {error}",
                    self.file_path.display()
                ))
            })?;

        let mut positions = Vec::with_capacity(envelopes.len());
        for (index, envelope) in envelopes.iter().enumerate() {
            serde_json::to_writer(&mut file, envelope).map_err(|error| {
                ContractError::Unavailable(format!(
                    "failed to serialize commit journal event {}: {error}",
                    envelope.event_id
                ))
            })?;
            file.write_all(b"\n").map_err(|error| {
                ContractError::Unavailable(format!(
                    "failed to append commit journal event {} to {}: {error}",
                    envelope.event_id,
                    self.file_path.display()
                ))
            })?;
            positions.push(CommitPosition::new(
                self.partition.as_str(),
                current_offset + index as u64 + 1,
            ));
        }
        file.sync_all().map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to sync append-only commit journal {}: {error}",
                self.file_path.display()
            ))
        })?;
        let file_size_bytes = file
            .metadata()
            .map_err(|error| {
                ContractError::Unavailable(format!(
                    "failed to stat append-only commit journal {}: {error}",
                    self.file_path.display()
                ))
            })?
            .len();
        write_commit_journal_index_unlocked(
            self.file_path.as_path(),
            &CommitJournalIndex {
                last_offset: current_offset + envelopes.len() as u64,
                file_size_bytes,
            },
        )?;
        Ok(positions)
    }
}

impl CommitJournal for FileCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        with_store_file_lock(self.file_path.as_path(), "commit journal", || {
            let mut positions = self.append_events_unlocked(std::slice::from_ref(&envelope))?;
            positions.pop().ok_or_else(|| {
                ContractError::Unavailable("commit journal append produced no position".into())
            })
        })
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("commit journal file store lock should lock");
        with_store_file_lock(self.file_path.as_path(), "commit journal", || {
            self.append_events_unlocked(envelopes.as_slice())
        })
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        FileCommitJournal::recorded(self)
    }
}

pub fn read_commit_journal_file(
    file_path: impl AsRef<Path>,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    with_store_file_lock(file_path.as_ref(), "commit journal", || {
        read_commit_journal_json_lines_unlocked(file_path.as_ref(), "commit journal")
    })
}

pub fn validate_commit_journal_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: Vec<CommitEnvelope> = read_commit_journal_file(file_path)?;
    Ok(())
}

fn read_commit_journal_json_lines_unlocked(
    file_path: &Path,
    store_name: &str,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    recover_pending_commit_journal_temp_file(file_path, store_name)?;

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    if file
        .metadata()
        .map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to stat {store_name} {}: {error}",
                file_path.display()
            ))
        })?
        .len()
        == 0
    {
        return Ok(Vec::new());
    }

    let reader = BufReader::new(file);
    let mut events = Vec::new();
    for (line_index, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read {store_name} {} line {}: {error}",
                file_path.display(),
                line_index + 1
            ))
        })?;
        if line.trim().is_empty() {
            return Err(ContractError::Unavailable(format!(
                "failed to parse {store_name} {} line {}: blank JSON Lines record",
                file_path.display(),
                line_index + 1
            )));
        }
        let event = serde_json::from_str::<CommitEnvelope>(line.as_str()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to parse {store_name} {} line {} as JSON Lines commit event: {error}",
                file_path.display(),
                line_index + 1
            ))
        })?;
        events.push(event);
    }

    Ok(events)
}

fn load_commit_journal_offset_unlocked(
    file_path: &Path,
    store_name: &str,
) -> Result<u64, ContractError> {
    let file_size_bytes = journal_file_size(file_path, store_name)?;
    let index_path = commit_journal_index_path(file_path);
    if index_path.exists() {
        let index = fs::read(index_path.as_path())
            .ok()
            .and_then(|payload| serde_json::from_slice::<CommitJournalIndex>(&payload).ok());
        if let Some(index) = index
            && index.file_size_bytes == file_size_bytes
            && (file_size_bytes > 0 || index.last_offset == 0)
        {
            return Ok(index.last_offset);
        }
    }

    rebuild_commit_journal_index_unlocked(file_path, store_name)
}

fn rebuild_commit_journal_index_unlocked(
    file_path: &Path,
    store_name: &str,
) -> Result<u64, ContractError> {
    let last_offset = read_commit_journal_json_lines_unlocked(file_path, store_name)?.len() as u64;
    let file_size_bytes = journal_file_size(file_path, store_name)?;
    write_commit_journal_index_unlocked(
        file_path,
        &CommitJournalIndex {
            last_offset,
            file_size_bytes,
        },
    )?;
    Ok(last_offset)
}

fn journal_file_size(file_path: &Path, store_name: &str) -> Result<u64, ContractError> {
    match fs::metadata(file_path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(error) => Err(ContractError::Unavailable(format!(
            "failed to stat {store_name} {}: {error}",
            file_path.display()
        ))),
    }
}

fn write_commit_journal_index_unlocked(
    file_path: &Path,
    index: &CommitJournalIndex,
) -> Result<(), ContractError> {
    let index_path = commit_journal_index_path(file_path);
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to create commit journal index dir {}: {error}",
                parent.display()
            ))
        })?;
    }
    let temp_path = sibling_path_with_suffix(index_path.as_path(), ".tmp");
    let payload = serde_json::to_vec(index).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to serialize commit journal index {}: {error}",
            index_path.display()
        ))
    })?;
    let mut temp_file = File::create(temp_path.as_path()).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create commit journal index temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    temp_file.write_all(payload.as_slice()).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to write commit journal index temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    temp_file.sync_all().map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to sync commit journal index temp file {}: {error}",
            temp_path.display()
        ))
    })?;
    drop(temp_file);
    fs::rename(temp_path.as_path(), index_path.as_path()).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to finalize commit journal index {}: {error}",
            index_path.display()
        ))
    })?;
    Ok(())
}

fn recover_pending_commit_journal_temp_file(
    file_path: &Path,
    store_name: &str,
) -> Result<(), ContractError> {
    let temp_path = file_path.with_extension("json.tmp");
    if !temp_path.exists() {
        return Ok(());
    }

    if file_path.exists() {
        return fs::remove_file(temp_path.as_path()).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to discard stale {store_name} temp file {}: {error}",
                temp_path.display()
            ))
        });
    }

    fs::rename(temp_path.as_path(), file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to recover {store_name} from temp file {} to {}: {error}",
            temp_path.display(),
            file_path.display()
        ))
    })
}

fn commit_journal_index_path(file_path: &Path) -> PathBuf {
    sibling_path_with_suffix(file_path, ".index")
}

fn sibling_path_with_suffix(file_path: &Path, suffix: &str) -> PathBuf {
    let mut file_name = file_path
        .file_name()
        .map(|value| value.to_os_string())
        .unwrap_or_else(|| OsString::from("commit-journal"));
    file_name.push(suffix);
    file_path
        .parent()
        .map(|parent| parent.join(&file_name))
        .unwrap_or_else(|| PathBuf::from(file_name))
}
