use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

const DEFAULT_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18079";
const DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BASE_URL: &str = "http://127.0.0.1:3900";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GatewayRuntimeMode {
    Split,
    Unified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpstreamConfig {
    pub service_id: String,
    pub base_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebGatewayConfig {
    pub bind_addr: String,
    pub runtime_mode: GatewayRuntimeMode,
    pub strict_startup: bool,
    pub upstreams: Vec<ServiceUpstreamConfig>,
}

impl WebGatewayConfig {
    pub fn from_env() -> Self {
        let bind_addr = first_env_value(&[
            "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
            "SDKWORK_IM_WEB_GATEWAY_BIND",
        ])
        .unwrap_or_else(|| DEFAULT_GATEWAY_BIND_ADDR.to_owned());
        Self::with_bind_addr_and_runtime_mode(
            bind_addr,
            resolve_runtime_mode_from_env(),
        )
    }

    pub fn from_server_config_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read server config file {}: {error}",
                path.display()
            )
        })?;
        let bind_addr = parse_server_config_bind_addr(&content)
            .unwrap_or_else(|| DEFAULT_GATEWAY_BIND_ADDR.to_owned());
        Ok(Self::with_bind_addr_and_runtime_mode(
            bind_addr,
            GatewayRuntimeMode::Split,
        ))
    }

    pub fn upstream_base_url(&self, service_id: &str) -> Option<&str> {
        service_upstream_lookup(&self.upstreams, service_id)
    }

    fn with_bind_addr_and_runtime_mode(
        bind_addr: String,
        runtime_mode: GatewayRuntimeMode,
    ) -> Self {
        let upstreams = match runtime_mode {
            GatewayRuntimeMode::Unified => default_unified_process_upstreams(),
            GatewayRuntimeMode::Split => default_split_upstreams(),
        };
        Self {
            bind_addr,
            runtime_mode,
            strict_startup: false,
            upstreams,
        }
    }
}

/// IM foundation services embedded in standalone unified-process assembly.
/// These must not be HTTP-proxied to split-service ports in unified mode.
pub fn is_assembly_embedded_im_service(service_id: &str) -> bool {
    matches!(
        canonical_service_id(service_id),
        "session-gateway"
            | "governance-service"
            | "comms-conversation-service"
            | "conversation-runtime"
            | "projection-service"
            | "streaming-service"
            | "media-service"
            | "notification-service"
            | "automation-service"
            | "audit-service"
            | "ops-service"
            | "comms-social-service"
            | "comms-space-service"
    )
}

/// T1 commerce capability app-api authorities embedded by IM standalone gateway.
pub const COMMERCE_T1_APP_API_SERVICES: &[&str] = &[
    "sdkwork-account-app-api",
    "sdkwork-catalog-app-api",
    "sdkwork-inventory-app-api",
    "sdkwork-invoice-app-api",
    "sdkwork-membership-app-api",
    "sdkwork-merchandise-app-api",
    "sdkwork-order-app-api",
    "sdkwork-payment-app-api",
    "sdkwork-promotion-app-api",
    "sdkwork-shop-app-api",
];

pub fn is_commerce_t1_app_api_service(service_id: &str) -> bool {
    let canonical = canonical_service_id(service_id);
    COMMERCE_T1_APP_API_SERVICES.contains(&canonical)
}

/// Sibling dependency app APIs embedded by IM standalone gateway in unified-process mode.
pub fn is_standalone_embedded_dependency_service(service_id: &str) -> bool {
    matches!(
        canonical_service_id(service_id),
        "sdkwork-drive-app-api"
            | "sdkwork-knowledgebase-app-api"
            | "sdkwork-voice-app-api"
            | "sdkwork-agents-app-api"
            | "sdkwork-mail-app-api"
            | "sdkwork-notary-app-api"
            | "sdkwork-course-app-api"
    ) || is_commerce_t1_app_api_service(service_id)
}

fn first_env_value(names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        env::var(name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
}

pub fn resolve_runtime_mode_from_env() -> GatewayRuntimeMode {
    match first_env_value(&["SDKWORK_IM_SERVICE_LAYOUT"])
        .unwrap_or_else(|| "split-services".to_owned())
        .to_ascii_lowercase()
        .as_str()
    {
        "unified-process" => GatewayRuntimeMode::Unified,
        _ => GatewayRuntimeMode::Split,
    }
}

pub const GATEWAY_EMBED_REALTIME_PLANE_ENV: &str = "SDKWORK_IM_GATEWAY_EMBED_REALTIME_PLANE";

/// Returns true when the gateway process should embed session-gateway instead of HTTP-proxying.
pub fn should_embed_session_gateway(config: &WebGatewayConfig) -> bool {
    config.runtime_mode == GatewayRuntimeMode::Unified
        || resolve_gateway_embed_realtime_plane_from_env()
}

pub fn resolve_gateway_embed_realtime_plane_from_env() -> bool {
    first_env_value(&[GATEWAY_EMBED_REALTIME_PLANE_ENV])
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn parse_server_config_bind_addr(content: &str) -> Option<String> {
    let mut in_network_block = false;
    let mut in_server_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_network_block = false;
            in_server_block = trimmed == "[server]";
            continue;
        }

        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        if is_top_level {
            if trimmed == "network:" {
                in_network_block = true;
                in_server_block = false;
                continue;
            }
            if let Some(value) =
                parse_toml_key_value(trimmed, &["bind_address", "bind", "bindAddress"])
            {
                return Some(value);
            }
            in_network_block = false;
        }

        if in_network_block && trimmed.starts_with("bindAddress:") {
            let value = trimmed
                .trim_start_matches("bindAddress:")
                .trim()
                .trim_matches('"')
                .trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }

        if in_server_block
            && let Some(value) =
                parse_toml_key_value(trimmed, &["bind_address", "bind", "bindAddress"])
        {
            return Some(value);
        }
    }

    None
}

fn parse_toml_key_value(trimmed: &str, keys: &[&str]) -> Option<String> {
    let (key, raw_value) = trimmed.split_once('=')?;
    if !keys.iter().any(|candidate| key.trim() == *candidate) {
        return None;
    }
    let value = raw_value
        .split('#')
        .next()
        .unwrap_or(raw_value)
        .trim()
        .trim_matches('"')
        .trim_matches('\'');
    if value.is_empty() {
        return None;
    }
    Some(value.to_owned())
}

/// Upstreams for standalone unified-process: split-only foundation APIs and Appbase catalog.
/// IM foundation routes and standalone-embedded dependency APIs are served in-process.
pub fn default_unified_process_upstreams() -> Vec<ServiceUpstreamConfig> {
    let appbase_upstream = default_appbase_app_api_upstream();
    vec![service_upstream(
        "sdkwork-iam-app-api",
        appbase_upstream.as_str(),
    )]
}

pub fn default_split_upstreams() -> Vec<ServiceUpstreamConfig> {
    let appbase_upstream = default_appbase_app_api_upstream();
    let drive_upstream = default_drive_app_api_upstream();
    let notary_upstream = default_notary_app_api_upstream();
    let mail_upstream = default_mail_app_api_upstream();
    let community_upstream = default_community_app_api_upstream();
    let course_upstream = default_course_app_api_upstream();
    let knowledgebase_upstream = default_knowledgebase_app_api_upstream();
    let voice_upstream = default_voice_app_api_upstream();
    let mut upstreams = vec![
        service_upstream("sdkwork-iam-app-api", appbase_upstream.as_str()),
        service_upstream("session-gateway", "http://127.0.0.1:18080"),
        service_upstream("governance-service", "http://127.0.0.1:18081"),
        service_upstream("comms-conversation-service", "http://127.0.0.1:18082"),
        service_upstream("conversation-runtime", "http://127.0.0.1:18082"),
        service_upstream("projection-service", "http://127.0.0.1:18083"),
        service_upstream("streaming-service", "http://127.0.0.1:18084"),
        service_upstream("im-calls-service", "http://127.0.0.1:18085"),
        service_upstream("sdkwork-drive-app-api", drive_upstream.as_str()),
        service_upstream("sdkwork-notary-app-api", notary_upstream.as_str()),
        service_upstream("sdkwork-mail-app-api", mail_upstream.as_str()),
        service_upstream("sdkwork-community-app-api", community_upstream.as_str()),
        service_upstream("sdkwork-course-app-api", course_upstream.as_str()),
        service_upstream("sdkwork-knowledgebase-app-api", knowledgebase_upstream.as_str()),
        service_upstream("sdkwork-voice-app-api", voice_upstream.as_str()),
        service_upstream("media-service", "http://127.0.0.1:18086"),
        service_upstream("notification-service", "http://127.0.0.1:18087"),
        service_upstream("automation-service", "http://127.0.0.1:18088"),
        service_upstream("audit-service", "http://127.0.0.1:18089"),
        service_upstream("ops-service", "http://127.0.0.1:18091"),
        service_upstream("comms-social-service", "http://127.0.0.1:18092"),
        service_upstream("social-service", "http://127.0.0.1:18092"),
        service_upstream("comms-space-service", "http://127.0.0.1:18093"),
        service_upstream("space-service", "http://127.0.0.1:18093"),
    ];
    upstreams.extend(commerce_t1_split_upstreams());
    upstreams
}

fn commerce_t1_split_upstreams() -> Vec<ServiceUpstreamConfig> {
    COMMERCE_T1_APP_API_SERVICES
        .iter()
        .map(|service_id| {
            service_upstream(
                service_id,
                default_commerce_t1_app_api_upstream(service_id).as_str(),
            )
        })
        .collect()
}

fn default_commerce_t1_app_api_upstream(service_id: &str) -> String {
    explicit_commerce_t1_app_api_upstream(service_id)
        .unwrap_or_else(default_platform_api_gateway_base_url)
}

fn explicit_commerce_t1_app_api_upstream(service_id: &str) -> Option<String> {
    let capability = service_id
        .strip_prefix("sdkwork-")
        .and_then(|rest| rest.strip_suffix("-app-api"))
        .unwrap_or(service_id);
    let capability_env = capability.replace('-', "_").to_ascii_uppercase();
    first_env_value(&[
        &format!("SDKWORK_IM_{capability_env}_APP_API_UPSTREAM"),
        &format!("SDKWORK_{capability_env}_APP_API_UPSTREAM"),
        &format!("SDKWORK_{capability_env}_APP_API_BASE_URL"),
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

/// Resolves legacy gateway service ids to canonical communication capability ids.
pub fn canonical_service_id(service_id: &str) -> &str {
    match service_id {
        "social-service" => "comms-social-service",
        "space-service" => "comms-space-service",
        "conversation-runtime" => "comms-conversation-service",
        "web-gateway" => "sdkwork-im-cloud-gateway",
        other => other,
    }
}

fn service_upstream_lookup<'a>(
    upstreams: &'a [ServiceUpstreamConfig],
    service_id: &str,
) -> Option<&'a str> {
    let canonical = canonical_service_id(service_id);
    for candidate in [service_id, canonical] {
        if let Some(base_url) = upstreams
            .iter()
            .find(|upstream| upstream.service_id == candidate)
            .map(|upstream| upstream.base_url.as_str())
        {
            return Some(base_url);
        }
    }
    None
}

fn default_appbase_app_api_upstream() -> String {
    explicit_appbase_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_drive_app_api_upstream() -> String {
    explicit_drive_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_notary_app_api_upstream() -> String {
    explicit_notary_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_mail_app_api_upstream() -> String {
    explicit_mail_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_community_app_api_upstream() -> String {
    explicit_community_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_course_app_api_upstream() -> String {
    explicit_course_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_knowledgebase_app_api_upstream() -> String {
    explicit_knowledgebase_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_voice_app_api_upstream() -> String {
    explicit_voice_app_api_upstream().unwrap_or_else(default_platform_api_gateway_base_url)
}

fn default_platform_api_gateway_base_url() -> String {
    first_env_value(&[
        "SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL",
        "SDKWORK_API_CLOUD_GATEWAY_BASE_URL",
    ])
    .or_else(|| {
        first_env_value(&["SDKWORK_API_CLOUD_GATEWAY_BIND"])
            .map(|bind_addr| format!("http://{bind_addr}"))
    })
    .and_then(normalize_base_url)
    .unwrap_or_else(|| DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BASE_URL.to_owned())
}

fn normalize_base_url(value: String) -> Option<String> {
    let normalized = value.trim().trim_end_matches('/').to_owned();
    if normalized.is_empty() {
        return None;
    }
    Some(normalized)
}

fn explicit_appbase_app_api_upstream() -> Option<String> {
    env::var("SDKWORK_IM_APPBASE_APP_API_UPSTREAM")
        .or_else(|_| {
            env::var("SDKWORK_APPBASE_APP_API_BIND_ADDR")
                .map(|bind_addr| format!("http://{}", bind_addr.trim()))
        })
        .ok()
        .and_then(normalize_base_url)
}

fn explicit_drive_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_DRIVE_APP_API_UPSTREAM",
        "SDKWORK_DRIVE_APP_API_UPSTREAM",
        "SDKWORK_DRIVE_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_notary_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_NOTARY_APP_API_UPSTREAM",
        "SDKWORK_NOTARY_APP_API_UPSTREAM",
        "SDKWORK_NOTARY_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_mail_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_MAIL_APP_API_UPSTREAM",
        "SDKWORK_MAIL_APP_API_UPSTREAM",
        "SDKWORK_MAIL_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_community_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM",
        "SDKWORK_COMMUNITY_APP_API_UPSTREAM",
        "SDKWORK_COMMUNITY_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_course_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_COURSE_APP_API_UPSTREAM",
        "SDKWORK_COURSE_APP_API_UPSTREAM",
        "SDKWORK_COURSE_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_knowledgebase_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM",
        "SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM",
        "SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

fn explicit_voice_app_api_upstream() -> Option<String> {
    first_env_value(&[
        "SDKWORK_IM_VOICE_APP_API_UPSTREAM",
        "SDKWORK_VOICE_APP_API_UPSTREAM",
        "SDKWORK_VOICE_APP_API_BASE_URL",
    ])
    .map(|value| value.trim().trim_end_matches('/').to_owned())
    .filter(|value| !value.is_empty())
}

pub fn service_upstream(service_id: &str, base_url: &str) -> ServiceUpstreamConfig {
    ServiceUpstreamConfig {
        service_id: service_id.to_owned(),
        base_url: base_url.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{GatewayRuntimeMode, WebGatewayConfig};

    fn gateway_config_env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
                return;
            }

            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }

    fn unique_temp_root(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("sdkwork_im_cloud_gateway_config_{prefix}_{unique}"))
    }

    #[test]
    fn test_web_gateway_config_loads_bind_addr_from_server_yaml() {
        let temp_root = unique_temp_root("server_yaml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let server_yaml_path = temp_root.join("server.yaml");
        fs::write(
            &server_yaml_path,
            r#"instance:
  name: "default"

network:
  bindAddress: "127.0.0.1:28080"
"#,
        )
        .expect("server yaml should be written");

        let config = WebGatewayConfig::from_server_config_file(&server_yaml_path)
            .expect("server yaml should produce a gateway config");
        assert_eq!(config.bind_addr, "127.0.0.1:28080");

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_web_gateway_config_requires_server_yaml_when_loading_file_mode() {
        let temp_root = unique_temp_root("missing_server_yaml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let missing_path = temp_root.join("server.yaml");

        let error = WebGatewayConfig::from_server_config_file(&missing_path)
            .expect_err("missing config file should return an error");
        assert!(
            error.contains("server config file"),
            "missing config error should mention the server config file: {error}"
        );

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_web_gateway_config_loads_bind_addr_from_chat_toml() {
        let temp_root = unique_temp_root("chat_toml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let chat_toml_path = temp_root.join("chat.toml");
        fs::write(
            &chat_toml_path,
            r#"[runtime]
deployment_mode = "server"
app_code = "chat"

[server]
bind_address = "127.0.0.1:38080"
"#,
        )
        .expect("chat toml should be written");

        let config = WebGatewayConfig::from_server_config_file(&chat_toml_path)
            .expect("chat toml should produce a gateway config");
        assert_eq!(config.bind_addr, "127.0.0.1:38080");

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_application_public_ingress_bind_env_takes_precedence() {
        let _guard = gateway_config_env_guard();
        let _standard_bind = ScopedEnvVar::set(
            "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
            "127.0.0.1:39080",
        );
        let _legacy_bind = ScopedEnvVar::set("SDKWORK_IM_WEB_GATEWAY_BIND", "127.0.0.1:18079");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.bind_addr, "127.0.0.1:39080");
    }

    #[test]
    fn test_should_embed_session_gateway_when_unified_or_explicit_env() {
        let _guard = gateway_config_env_guard();
        let _embed = ScopedEnvVar::remove(super::GATEWAY_EMBED_REALTIME_PLANE_ENV);
        let _layout = ScopedEnvVar::set("SDKWORK_IM_SERVICE_LAYOUT", "unified-process");
        let unified = WebGatewayConfig::from_env();
        assert!(super::should_embed_session_gateway(&unified));

        let _layout = ScopedEnvVar::set("SDKWORK_IM_SERVICE_LAYOUT", "split-services");
        let split = WebGatewayConfig::from_env();
        assert!(!super::should_embed_session_gateway(&split));

        let _embed = ScopedEnvVar::set(super::GATEWAY_EMBED_REALTIME_PLANE_ENV, "true");
        assert!(super::should_embed_session_gateway(&split));
    }

    #[test]
    fn test_web_gateway_config_selects_unified_runtime_mode_from_service_layout_env() {
        let _guard = gateway_config_env_guard();
        let _layout = ScopedEnvVar::set("SDKWORK_IM_SERVICE_LAYOUT", "unified-process");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Unified);
    }

    #[test]
    fn test_resolve_runtime_mode_from_env_defaults_to_split() {
        let _guard = gateway_config_env_guard();
        let _layout = ScopedEnvVar::remove("SDKWORK_IM_SERVICE_LAYOUT");

        assert_eq!(
            super::resolve_runtime_mode_from_env(),
            GatewayRuntimeMode::Split
        );
    }

    #[test]
    fn test_web_gateway_config_defaults_to_split_upstreams() {
        let _guard = gateway_config_env_guard();
        let _platform_gateway = ScopedEnvVar::remove("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL");
        let _gateway_base_url = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BASE_URL");
        let _gateway_bind = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BIND");
        let _appbase_upstream = ScopedEnvVar::remove("SDKWORK_IM_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _drive_upstream = ScopedEnvVar::remove("SDKWORK_IM_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_upstream = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_base_url = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_BASE_URL");
        let _notary_upstream = ScopedEnvVar::remove("SDKWORK_IM_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_upstream = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_base_url = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_BASE_URL");
        let _catalog_upstream = ScopedEnvVar::remove("SDKWORK_IM_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_BASE_URL");
        let _mail_upstream = ScopedEnvVar::remove("SDKWORK_IM_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_upstream = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_base_url = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_BASE_URL");
        let _community_upstream = ScopedEnvVar::remove("SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_upstream = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_base_url = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_BASE_URL");
        let _course_upstream = ScopedEnvVar::remove("SDKWORK_IM_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_upstream = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_base_url = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_BASE_URL");
        let _knowledgebase_upstream = ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_upstream =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_base_url =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("session-gateway"),
            Some("http://127.0.0.1:18080")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-mail-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-community-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-course-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-knowledgebase-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("ops-service"),
            Some("http://127.0.0.1:18091")
        );
        assert_eq!(
            config.upstream_base_url("comms-social-service"),
            Some("http://127.0.0.1:18092")
        );
        assert_eq!(
            config.upstream_base_url("social-service"),
            Some("http://127.0.0.1:18092")
        );
        assert_eq!(
            config.upstream_base_url("comms-space-service"),
            Some("http://127.0.0.1:18093")
        );
        assert_eq!(config.upstream_base_url("interaction-service"), None);
    }

    #[test]
    fn test_web_gateway_config_unified_process_omits_split_foundation_upstreams() {
        let _guard = gateway_config_env_guard();
        let _layout = ScopedEnvVar::set("SDKWORK_IM_SERVICE_LAYOUT", "unified-process");
        let _platform_gateway = ScopedEnvVar::remove("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL");
        let _gateway_base_url = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BASE_URL");
        let _gateway_bind = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BIND");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Unified);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(config.upstream_base_url("sdkwork-drive-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-knowledgebase-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-catalog-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-mail-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-notary-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-community-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-course-app-api"), None);
        assert_eq!(config.upstream_base_url("session-gateway"), None);
        assert_eq!(config.upstream_base_url("comms-social-service"), None);
        assert_eq!(config.upstream_base_url("comms-space-service"), None);
        assert_eq!(config.upstream_base_url("projection-service"), None);
        assert!(super::is_assembly_embedded_im_service("social-service"));
        assert!(super::is_standalone_embedded_dependency_service("sdkwork-drive-app-api"));
        assert!(super::is_standalone_embedded_dependency_service(
            "sdkwork-knowledgebase-app-api"
        ));
        assert!(super::is_standalone_embedded_dependency_service("sdkwork-voice-app-api"));
        assert!(super::is_standalone_embedded_dependency_service(
            "sdkwork-catalog-app-api"
        ));
        assert!(super::is_standalone_embedded_dependency_service("sdkwork-mail-app-api"));
        assert!(super::is_standalone_embedded_dependency_service("sdkwork-notary-app-api"));
        assert!(super::is_standalone_embedded_dependency_service("sdkwork-course-app-api"));
    }

    #[test]
    fn test_web_gateway_config_uses_shared_gateway_base_url_for_platform_defaults() {
        let _guard = gateway_config_env_guard();
        let _platform_gateway = ScopedEnvVar::set(
            "SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL",
            "http://127.0.0.1:4900/",
        );
        let _gateway_base_url =
            ScopedEnvVar::set("SDKWORK_API_CLOUD_GATEWAY_BASE_URL", "http://127.0.0.1:5900");
        let _gateway_bind = ScopedEnvVar::set("SDKWORK_API_CLOUD_GATEWAY_BIND", "127.0.0.1:6900");
        let _appbase_upstream = ScopedEnvVar::remove("SDKWORK_IM_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _drive_upstream = ScopedEnvVar::remove("SDKWORK_IM_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_upstream = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_base_url = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_BASE_URL");
        let _notary_upstream = ScopedEnvVar::remove("SDKWORK_IM_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_upstream = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_base_url = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_BASE_URL");
        let _catalog_upstream = ScopedEnvVar::remove("SDKWORK_IM_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_BASE_URL");
        let _mail_upstream = ScopedEnvVar::remove("SDKWORK_IM_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_upstream = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_base_url = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_BASE_URL");
        let _community_upstream = ScopedEnvVar::remove("SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_upstream = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_base_url = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_BASE_URL");
        let _course_upstream = ScopedEnvVar::remove("SDKWORK_IM_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_upstream = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_base_url = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_BASE_URL");
        let _knowledgebase_upstream = ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_upstream =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_base_url =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-mail-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-community-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-course-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-knowledgebase-app-api"),
            Some("http://127.0.0.1:4900")
        );
    }

    #[test]
    fn test_web_gateway_config_derives_shared_gateway_base_url_from_gateway_bind() {
        let _guard = gateway_config_env_guard();
        let _platform_gateway = ScopedEnvVar::remove("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL");
        let _gateway_base_url = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BASE_URL");
        let _gateway_bind = ScopedEnvVar::set("SDKWORK_API_CLOUD_GATEWAY_BIND", "127.0.0.1:7900");
        let _appbase_upstream = ScopedEnvVar::remove("SDKWORK_IM_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _drive_upstream = ScopedEnvVar::remove("SDKWORK_IM_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_upstream = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_base_url = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_BASE_URL");
        let _notary_upstream = ScopedEnvVar::remove("SDKWORK_IM_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_upstream = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_base_url = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_BASE_URL");
        let _catalog_upstream = ScopedEnvVar::remove("SDKWORK_IM_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_BASE_URL");
        let _mail_upstream = ScopedEnvVar::remove("SDKWORK_IM_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_upstream = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_base_url = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_BASE_URL");
        let _community_upstream = ScopedEnvVar::remove("SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_upstream = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_base_url = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_BASE_URL");
        let _course_upstream = ScopedEnvVar::remove("SDKWORK_IM_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_upstream = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_base_url = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_BASE_URL");
        let _knowledgebase_upstream = ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_upstream =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_base_url =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL");

        let config = WebGatewayConfig::from_env();

        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-mail-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-community-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-course-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-knowledgebase-app-api"),
            Some("http://127.0.0.1:7900")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_drive_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _drive_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_DRIVE_APP_API_UPSTREAM",
            "http://127.0.0.1:28080/",
        );
        let _sdkwork_drive_upstream =
            ScopedEnvVar::set("SDKWORK_DRIVE_APP_API_UPSTREAM", "http://127.0.0.1:38080");
        let _sdkwork_drive_base_url =
            ScopedEnvVar::set("SDKWORK_DRIVE_APP_API_BASE_URL", "http://127.0.0.1:48080");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:28080")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_notary_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _notary_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_NOTARY_APP_API_UPSTREAM",
            "http://127.0.0.1:28092/",
        );
        let _sdkwork_notary_upstream =
            ScopedEnvVar::set("SDKWORK_NOTARY_APP_API_UPSTREAM", "http://127.0.0.1:38092");
        let _sdkwork_notary_base_url =
            ScopedEnvVar::set("SDKWORK_NOTARY_APP_API_BASE_URL", "http://127.0.0.1:48092");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:28092")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_catalog_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _catalog_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_CATALOG_APP_API_UPSTREAM",
            "http://127.0.0.1:28094/",
        );
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::set("SDKWORK_CATALOG_APP_API_UPSTREAM", "http://127.0.0.1:38094");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::set("SDKWORK_CATALOG_APP_API_BASE_URL", "http://127.0.0.1:48094");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:28094")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_mail_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _mail_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_MAIL_APP_API_UPSTREAM",
            "http://127.0.0.1:28096/",
        );
        let _sdkwork_mail_upstream =
            ScopedEnvVar::set("SDKWORK_MAIL_APP_API_UPSTREAM", "http://127.0.0.1:38096");
        let _sdkwork_mail_base_url =
            ScopedEnvVar::set("SDKWORK_MAIL_APP_API_BASE_URL", "http://127.0.0.1:48096");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-mail-app-api"),
            Some("http://127.0.0.1:28096")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_community_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _community_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM",
            "http://127.0.0.1:28098/",
        );
        let _sdkwork_community_upstream =
            ScopedEnvVar::set("SDKWORK_COMMUNITY_APP_API_UPSTREAM", "http://127.0.0.1:38098");
        let _sdkwork_community_base_url =
            ScopedEnvVar::set("SDKWORK_COMMUNITY_APP_API_BASE_URL", "http://127.0.0.1:48098");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-community-app-api"),
            Some("http://127.0.0.1:28098")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_course_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _course_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_COURSE_APP_API_UPSTREAM",
            "http://127.0.0.1:28100/",
        );
        let _sdkwork_course_upstream =
            ScopedEnvVar::set("SDKWORK_COURSE_APP_API_UPSTREAM", "http://127.0.0.1:38100");
        let _sdkwork_course_base_url =
            ScopedEnvVar::set("SDKWORK_COURSE_APP_API_BASE_URL", "http://127.0.0.1:48100");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-course-app-api"),
            Some("http://127.0.0.1:28100")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_voice_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _voice_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_VOICE_APP_API_UPSTREAM",
            "http://127.0.0.1:28103/",
        );
        let _sdkwork_voice_upstream =
            ScopedEnvVar::set("SDKWORK_VOICE_APP_API_UPSTREAM", "http://127.0.0.1:38103");
        let _sdkwork_voice_base_url =
            ScopedEnvVar::set("SDKWORK_VOICE_APP_API_BASE_URL", "http://127.0.0.1:48103");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-voice-app-api"),
            Some("http://127.0.0.1:28103")
        );
    }

    #[test]
    fn test_web_gateway_config_allows_knowledgebase_app_api_upstream_override() {
        let _guard = gateway_config_env_guard();
        let _knowledgebase_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:28102/",
        );
        let _sdkwork_knowledgebase_upstream = ScopedEnvVar::set(
            "SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:38102",
        );
        let _sdkwork_knowledgebase_base_url = ScopedEnvVar::set(
            "SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL",
            "http://127.0.0.1:48102",
        );

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-knowledgebase-app-api"),
            Some("http://127.0.0.1:28102")
        );
    }

    #[test]
    fn test_web_gateway_local_mode_alias_is_normalized_to_split_gateway_defaults() {
        let _guard = gateway_config_env_guard();
        let _platform_gateway = ScopedEnvVar::remove("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL");
        let _gateway_base_url = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BASE_URL");
        let _gateway_bind = ScopedEnvVar::remove("SDKWORK_API_CLOUD_GATEWAY_BIND");
        let _appbase_upstream = ScopedEnvVar::remove("SDKWORK_IM_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _drive_upstream = ScopedEnvVar::remove("SDKWORK_IM_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_upstream = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_UPSTREAM");
        let _sdkwork_drive_base_url = ScopedEnvVar::remove("SDKWORK_DRIVE_APP_API_BASE_URL");
        let _notary_upstream = ScopedEnvVar::remove("SDKWORK_IM_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_upstream = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_UPSTREAM");
        let _sdkwork_notary_base_url = ScopedEnvVar::remove("SDKWORK_NOTARY_APP_API_BASE_URL");
        let _catalog_upstream = ScopedEnvVar::remove("SDKWORK_IM_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_UPSTREAM");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::remove("SDKWORK_CATALOG_APP_API_BASE_URL");
        let _mail_upstream = ScopedEnvVar::remove("SDKWORK_IM_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_upstream = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_UPSTREAM");
        let _sdkwork_mail_base_url = ScopedEnvVar::remove("SDKWORK_MAIL_APP_API_BASE_URL");
        let _community_upstream = ScopedEnvVar::remove("SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_upstream = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_UPSTREAM");
        let _sdkwork_community_base_url = ScopedEnvVar::remove("SDKWORK_COMMUNITY_APP_API_BASE_URL");
        let _course_upstream = ScopedEnvVar::remove("SDKWORK_IM_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_upstream = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_UPSTREAM");
        let _sdkwork_course_base_url = ScopedEnvVar::remove("SDKWORK_COURSE_APP_API_BASE_URL");
        let _knowledgebase_upstream = ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_upstream =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM");
        let _sdkwork_knowledgebase_base_url =
            ScopedEnvVar::remove("SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:3900")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:3900")
        );
    }

    #[test]
    fn test_web_gateway_local_mode_alias_allows_explicit_drive_app_api_upstream() {
        let _guard = gateway_config_env_guard();
        let _appbase_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_APPBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:19090/",
        );
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _drive_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_DRIVE_APP_API_UPSTREAM",
            "http://127.0.0.1:28080/",
        );
        let _sdkwork_drive_upstream =
            ScopedEnvVar::set("SDKWORK_DRIVE_APP_API_UPSTREAM", "http://127.0.0.1:38080");
        let _sdkwork_drive_base_url =
            ScopedEnvVar::set("SDKWORK_DRIVE_APP_API_BASE_URL", "http://127.0.0.1:48080");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:19090")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-drive-app-api"),
            Some("http://127.0.0.1:28080")
        );
    }

    #[test]
    fn test_web_gateway_local_mode_alias_allows_explicit_notary_app_api_upstream() {
        let _guard = gateway_config_env_guard();
        let _appbase_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_APPBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:19090/",
        );
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _notary_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_NOTARY_APP_API_UPSTREAM",
            "http://127.0.0.1:28092/",
        );
        let _sdkwork_notary_upstream =
            ScopedEnvVar::set("SDKWORK_NOTARY_APP_API_UPSTREAM", "http://127.0.0.1:38092");
        let _sdkwork_notary_base_url =
            ScopedEnvVar::set("SDKWORK_NOTARY_APP_API_BASE_URL", "http://127.0.0.1:48092");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:19090")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-notary-app-api"),
            Some("http://127.0.0.1:28092")
        );
    }

    #[test]
    fn test_web_gateway_local_mode_alias_allows_explicit_catalog_app_api_upstream() {
        let _guard = gateway_config_env_guard();
        let _appbase_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_APPBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:19090/",
        );
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");
        let _catalog_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_CATALOG_APP_API_UPSTREAM",
            "http://127.0.0.1:28094/",
        );
        let _sdkwork_catalog_upstream =
            ScopedEnvVar::set("SDKWORK_CATALOG_APP_API_UPSTREAM", "http://127.0.0.1:38094");
        let _sdkwork_catalog_base_url =
            ScopedEnvVar::set("SDKWORK_CATALOG_APP_API_BASE_URL", "http://127.0.0.1:48094");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:19090")
        );
        assert_eq!(
            config.upstream_base_url("sdkwork-catalog-app-api"),
            Some("http://127.0.0.1:28094")
        );
    }

    #[test]
    fn test_web_gateway_local_mode_alias_still_uses_appbase_split_override() {
        let _guard = gateway_config_env_guard();
        let _appbase_upstream = ScopedEnvVar::remove("SDKWORK_IM_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr =
            ScopedEnvVar::set("SDKWORK_APPBASE_APP_API_BIND_ADDR", "127.0.0.1:28090");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:28090")
        );
    }
}
