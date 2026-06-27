use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{
    menu::MenuBuilder,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime, Window, WindowEvent,
};

const MAIN_WINDOW_LABEL: &str = "main";
const TRAY_ICON_ID: &str = "sdkwork_chat_pc_main_tray";
const TRAY_MENU_CHAT_ID: &str = "sdkwork_chat_pc_tray_chat";
const TRAY_MENU_CALL_ID: &str = "sdkwork_chat_pc_tray_call";
const TRAY_MENU_SETTINGS_ID: &str = "sdkwork_chat_pc_tray_settings";
const TRAY_MENU_QUIT_ID: &str = "sdkwork_chat_pc_tray_quit";
const TRAY_EVENT_OPEN_CHAT: &str = "sdkwork-im-pc://tray/open-chat";
const TRAY_EVENT_SHOW_ACTIVE_CALL: &str = "sdkwork-im-pc://tray/show-active-call";
const TRAY_EVENT_OPEN_SETTINGS: &str = "sdkwork-im-pc://tray/open-settings";

static IS_EXITING: AtomicBool = AtomicBool::new(false);

pub fn main_window<R: Runtime>(app: &AppHandle<R>) -> Result<tauri::WebviewWindow<R>, String> {
    app.get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "main window is unavailable".to_string())
}

pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let window = main_window(app)?;
    let _ = window.unminimize();
    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())?;
    Ok(())
}

pub fn hide_main_window_to_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    ensure_tray(app)?;
    main_window(app)?.hide().map_err(|error| error.to_string())
}

fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or_else(|| "default window icon is unavailable".to_string())?;
    let menu = MenuBuilder::new(app)
        .text(TRAY_MENU_CHAT_ID, "\u{804a}\u{5929}")
        .text(TRAY_MENU_CALL_ID, "\u{663e}\u{793a}\u{901a}\u{8bdd}")
        .text(TRAY_MENU_SETTINGS_ID, "\u{8bbe}\u{7f6e}")
        .separator()
        .text(TRAY_MENU_QUIT_ID, "\u{9000}\u{51fa}")
        .build()
        .map_err(|error| error.to_string())?;

    TrayIconBuilder::with_id(TRAY_ICON_ID)
        .menu(&menu)
        .icon(icon)
        .tooltip("Sdkwork IM PC")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_tray_menu_event(app, event))
        .on_tray_icon_event(|tray, event| handle_tray_icon_event(tray.app_handle(), event))
        .build(app)
        .map_err(|error| error.to_string())?;

    Ok(())
}

pub fn ensure_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    if app.tray_by_id(TRAY_ICON_ID).is_some() {
        return Ok(());
    }

    create_tray(app)
}

fn handle_tray_icon_event<R: Runtime>(app: &AppHandle<R>, event: TrayIconEvent) {
    if matches!(
        event,
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        }
    ) {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_CHAT, ());
    }
}

fn handle_tray_menu_event<R: Runtime>(app: &AppHandle<R>, event: tauri::menu::MenuEvent) {
    if event.id() == TRAY_MENU_CHAT_ID {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_CHAT, ());
    } else if event.id() == TRAY_MENU_CALL_ID {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_SHOW_ACTIVE_CALL, ());
    } else if event.id() == TRAY_MENU_SETTINGS_ID {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_SETTINGS, ());
    } else if event.id() == TRAY_MENU_QUIT_ID {
        quit_app(app);
    }
}

fn quit_app<R: Runtime>(app: &AppHandle<R>) {
    IS_EXITING.store(true, Ordering::SeqCst);
    if let Ok(window) = main_window(app) {
        let _ = window.close();
    }
    app.exit(0);
}

pub fn handle_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    if window.label() != MAIN_WINDOW_LABEL {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        if IS_EXITING.load(Ordering::SeqCst) {
            return;
        }

        api.prevent_close();
        let app = window.app_handle();
        if ensure_tray(app).is_ok() {
            let _ = window.hide();
        }
    }
}
