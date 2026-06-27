mod notification;
mod qr_code;
mod session_store;
mod tray;
mod window_control;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tray::ensure_tray(app.handle()).map_err(Box::<dyn std::error::Error>::from)?;
            Ok(())
        })
        .on_window_event(tray::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            window_control::sdkwork_chat_pc_window_control,
            notification::sdkwork_chat_pc_notification_permission,
            notification::sdkwork_chat_pc_request_notification_permission,
            notification::sdkwork_chat_pc_show_notification,
            qr_code::sdkwork_chat_pc_decode_qr_code_image,
            qr_code::sdkwork_chat_pc_decode_qr_code_rgba,
            session_store::sdkwork_im_pc_session_read,
            session_store::sdkwork_im_pc_session_write,
            session_store::sdkwork_im_pc_session_clear
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Sdkwork IM PC");
}
