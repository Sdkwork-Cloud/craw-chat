use serde::Deserialize;
use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::{NotificationExt, PermissionState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkworkChatPcNotificationPayload {
    pub body: String,
    pub call_id: Option<String>,
    pub conversation_id: String,
    pub icon: Option<String>,
    pub kind: Option<String>,
    pub message_id: Option<String>,
    pub title: String,
    pub r#type: Option<String>,
}

fn permission_state_name(permission_state: PermissionState) -> &'static str {
    match permission_state {
        PermissionState::Granted => "granted",
        PermissionState::Denied => "denied",
        PermissionState::Prompt | PermissionState::PromptWithRationale => "default",
    }
}

fn is_supported_native_notification_icon(icon: &str) -> bool {
    let normalized = icon.trim().to_ascii_lowercase();
    !normalized.starts_with("http://")
        && !normalized.starts_with("https://")
        && !normalized.starts_with("data:")
        && !normalized.is_empty()
}

#[tauri::command]
pub fn sdkwork_chat_pc_notification_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<String, String> {
    app.notification()
        .permission_state()
        .map(permission_state_name)
        .map(str::to_owned)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn sdkwork_chat_pc_request_notification_permission<R: Runtime>(
    app: AppHandle<R>,
) -> Result<String, String> {
    app.notification()
        .request_permission()
        .map(permission_state_name)
        .map(str::to_owned)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn sdkwork_chat_pc_show_notification<R: Runtime>(
    app: AppHandle<R>,
    notification: SdkworkChatPcNotificationPayload,
) -> Result<(), String> {
    let conversation_id = notification.conversation_id;
    let call_id = notification.call_id;
    let kind = notification.kind.unwrap_or_else(|| "message".to_string());
    let message_id = notification.message_id;
    let notification_type = notification.r#type;

    let mut builder = app
        .notification()
        .builder()
        .title(notification.title)
        .body(notification.body)
        .group(conversation_id.clone())
        .extra("conversationId", conversation_id)
        .extra("kind", kind)
        .auto_cancel();

    if let Some(call_id) = call_id {
        builder = builder.extra("callId", call_id);
    }
    if let Some(message_id) = message_id {
        builder = builder.extra("messageId", message_id);
    }
    if let Some(notification_type) = notification_type {
        builder = builder.extra("type", notification_type);
    }

    if let Some(icon) = notification
        .icon
        .filter(|icon| is_supported_native_notification_icon(icon))
    {
        builder = builder.icon(icon);
    }

    builder.show().map_err(|error| error.to_string())
}
