use std::fs;
use std::path::PathBuf;

use keyring::Entry;
use tauri::{AppHandle, Manager};

const KEYRING_SERVICE: &str = "com.sdkwork.im-pc";
const KEYRING_ACCOUNT: &str = "session:v1";
const LEGACY_SECURE_SESSION_STORE_PATH: &str = "secure-session.json";
const LEGACY_SECURE_SESSION_STORE_KEY: &str = "session";

fn session_keyring_entry() -> Result<Entry, String> {
    Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT)
        .map_err(|error| format!("open session keyring entry failed: {error}"))
}

fn legacy_session_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("resolve app data dir failed: {error}"))?;
    Ok(app_data_dir.join(LEGACY_SECURE_SESSION_STORE_PATH))
}

fn read_legacy_store_value(app: &AppHandle) -> Result<Option<String>, String> {
    let store_path = legacy_session_store_path(app)?;
    if !store_path.is_file() {
        return Ok(None);
    }

    let raw = fs::read_to_string(&store_path)
        .map_err(|error| format!("read legacy secure session store failed: {error}"))?;
    let parsed = serde_json::from_str::<serde_json::Value>(&raw)
        .map_err(|error| format!("parse legacy secure session store failed: {error}"))?;
    Ok(parsed
        .get(LEGACY_SECURE_SESSION_STORE_KEY)
        .and_then(|value| value.as_str().map(str::to_owned))
        .filter(|value| !value.trim().is_empty()))
}

fn remove_legacy_store_file(app: &AppHandle) -> Result<(), String> {
    let store_path = legacy_session_store_path(app)?;
    if store_path.is_file() {
        fs::remove_file(store_path)
            .map_err(|error| format!("remove legacy secure session store failed: {error}"))?;
    }
    Ok(())
}

fn migrate_legacy_store_value(app: &AppHandle) -> Result<Option<String>, String> {
    let legacy_value = read_legacy_store_value(app)?;
    let Some(value) = legacy_value else {
        return Ok(None);
    };

    session_keyring_entry()?.set_password(value.as_str()).map_err(|error| {
        format!("migrate legacy secure session store into keyring failed: {error}")
    })?;
    remove_legacy_store_file(app)?;
    Ok(Some(value))
}

#[tauri::command]
pub fn sdkwork_im_pc_session_read(app: AppHandle) -> Result<Option<String>, String> {
    let entry = session_keyring_entry()?;
    match entry.get_password() {
        Ok(value) if value.trim().is_empty() => Ok(None),
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => migrate_legacy_store_value(&app),
        Err(error) => Err(format!("read session from keyring failed: {error}")),
    }
}

#[tauri::command]
pub fn sdkwork_im_pc_session_write(app: AppHandle, value: String) -> Result<(), String> {
    session_keyring_entry()?
        .set_password(value.as_str())
        .map_err(|error| format!("write session to keyring failed: {error}"))?;
    remove_legacy_store_file(&app)?;
    Ok(())
}

#[tauri::command]
pub fn sdkwork_im_pc_session_clear(app: AppHandle) -> Result<(), String> {
    match session_keyring_entry()?.delete_password() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(error) => return Err(format!("clear session keyring entry failed: {error}")),
    }
    remove_legacy_store_file(&app)?;
    Ok(())
}
