use serde::Deserialize;
use tauri::{AppHandle, Manager, Runtime, Url, WebviewUrl, WebviewWindowBuilder};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenKnowledgeWindowRequest {
    pub url: String,
    pub title: Option<String>,
    pub label: Option<String>,
}

fn sanitize_window_label(raw: &str) -> String {
    let mut label = raw
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();

    if label.is_empty() {
        label.push_str("knowledge-host");
    }

    if !label.starts_with("knowledge-") {
        label = format!("knowledge-{label}");
    }

    label
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "[::1]" | "::1")
}

fn resolve_main_window_host<R: Runtime>(app: &AppHandle<R>) -> Option<String> {
    app.get_webview_window("main")
        .and_then(|window| window.url().ok())
        .and_then(|url| url.host().map(|host| host.to_string()))
}

fn is_allowed_knowledge_window_host(host: &str, main_window_host: Option<&str>) -> bool {
    if is_loopback_host(host) {
        return true;
    }

    main_window_host.is_some_and(|main_host| host.eq_ignore_ascii_case(main_host))
}

fn validate_knowledge_window_url<R: Runtime>(app: &AppHandle<R>, url: &Url) -> Result<(), String> {
    let scheme = url.scheme();
    if scheme != "http" && scheme != "https" {
        return Err("knowledge window url must use http or https".to_string());
    }

    let host = url
        .host_str()
        .ok_or_else(|| "knowledge window url must include a host".to_string())?;

    let main_window_host = resolve_main_window_host(app);
    if !is_allowed_knowledge_window_host(host, main_window_host.as_deref()) {
        return Err(format!(
            "knowledge window url host is not allowed: {host}"
        ));
    }

    Ok(())
}

#[tauri::command]
pub fn sdkwork_chat_pc_open_knowledge_window<R: Runtime>(
    app: AppHandle<R>,
    request: OpenKnowledgeWindowRequest,
) -> Result<(), String> {
    let label = sanitize_window_label(request.label.as_deref().unwrap_or("knowledge-host"));
    if let Some(existing) = app.get_webview_window(&label) {
        existing
            .show()
            .map_err(|error| format!("show existing knowledge window failed: {error}"))?;
        existing
            .set_focus()
            .map_err(|error| format!("focus existing knowledge window failed: {error}"))?;
        return Ok(());
    }

    let url = Url::parse(request.url.trim())
        .map_err(|error| format!("invalid knowledge window url: {error}"))?;
    validate_knowledge_window_url(&app, &url)?;
    let title = request
        .title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Knowledge Base");

    WebviewWindowBuilder::new(&app, label, WebviewUrl::External(url))
        .title(title)
        .inner_size(1280.0, 860.0)
        .min_inner_size(960.0, 640.0)
        .center()
        .resizable(true)
        .decorations(true)
        .focused(true)
        .visible(true)
        .build()
        .map_err(|error| format!("create knowledge window failed: {error}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        is_allowed_knowledge_window_host, is_loopback_host, sanitize_window_label,
    };

    #[test]
    fn sanitize_window_label_prefixes_knowledge() {
        assert_eq!(sanitize_window_label("group-1"), "knowledge-group-1");
        assert_eq!(sanitize_window_label("knowledge-host"), "knowledge-host");
    }

    #[test]
    fn loopback_hosts_are_allowed() {
        assert!(is_loopback_host("localhost"));
        assert!(is_loopback_host("127.0.0.1"));
        assert!(is_loopback_host("[::1]"));
    }

    #[test]
    fn main_window_host_is_allowed() {
        assert!(is_allowed_knowledge_window_host(
            "127.0.0.1",
            Some("127.0.0.1")
        ));
        assert!(is_allowed_knowledge_window_host(
            "im.example.com",
            Some("im.example.com")
        ));
        assert!(!is_allowed_knowledge_window_host(
            "evil.example.com",
            Some("127.0.0.1")
        ));
    }
}
