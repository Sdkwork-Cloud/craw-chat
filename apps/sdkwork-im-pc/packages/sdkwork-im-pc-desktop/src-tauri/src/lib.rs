mod notification;
mod offline_store;
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
            session_store::sdkwork_im_pc_session_clear,
            offline_store::sdkwork_im_pc_offline_init,
            offline_store::sdkwork_im_pc_offline_upsert_conversations,
            offline_store::sdkwork_im_pc_offline_list_conversations,
            offline_store::sdkwork_im_pc_offline_upsert_messages,
            offline_store::sdkwork_im_pc_offline_list_messages,
            offline_store::sdkwork_im_pc_offline_get_sync_cursor,
            offline_store::sdkwork_im_pc_offline_set_sync_cursor,
            offline_store::sdkwork_im_pc_offline_enqueue_pending_send,
            offline_store::sdkwork_im_pc_offline_list_pending_sends,
            offline_store::sdkwork_im_pc_offline_claim_pending_sends,
            offline_store::sdkwork_im_pc_offline_release_pending_send_claim,
            offline_store::sdkwork_im_pc_offline_delete_pending_send,
            offline_store::sdkwork_im_pc_offline_quarantine_pending_send,
            offline_store::sdkwork_im_pc_offline_purge_principal_cache
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Sdkwork IM PC");
}
