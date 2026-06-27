use serde::Deserialize;
use tauri::{AppHandle, Runtime};

use crate::tray::{hide_main_window_to_tray, main_window, show_main_window};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SdkworkChatPcWindowControlAction {
    Minimize,
    ToggleMaximize,
    CloseToTray,
    Show,
    StartDragging,
}

#[tauri::command]
pub fn sdkwork_chat_pc_window_control<R: Runtime>(
    app: AppHandle<R>,
    action: SdkworkChatPcWindowControlAction,
) -> Result<(), String> {
    let window = main_window(&app)?;

    match action {
        SdkworkChatPcWindowControlAction::Minimize => {
            window.minimize().map_err(|error| error.to_string())
        }
        SdkworkChatPcWindowControlAction::ToggleMaximize => {
            if window.is_maximized().map_err(|error| error.to_string())? {
                window.unmaximize().map_err(|error| error.to_string())
            } else {
                window.maximize().map_err(|error| error.to_string())
            }
        }
        SdkworkChatPcWindowControlAction::CloseToTray => hide_main_window_to_tray(&app),
        SdkworkChatPcWindowControlAction::Show => show_main_window(&app),
        SdkworkChatPcWindowControlAction::StartDragging => {
            window.start_dragging().map_err(|error| error.to_string())
        }
    }
}
