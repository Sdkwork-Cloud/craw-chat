use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

const DEFAULT_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18079";
const DEFAULT_SDKWORK_API_CLOUD_GATEWAY_BASE_URL: &str = "http://127.0.0.1:3900";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GatewayRuntimeMode {
    SingleIngress,
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
        Self::with_bind_addr(bind_addr)
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
        Ok(Self::with_bind_addr(bind_addr))
    }

    pub fn upstream_base_url(&self, service_id: &str) -> Option<&str> {
        service_upstream_lookup(&self.upstreams, service_id)
    }

    fn with_bind_addr(bind_addr: String) -> Self {
        Self {
            bind_addr,
            runtime_mode: GatewayRuntimeMode::SingleIngress,
            strict_startup: false,
            upstreams: default_single_ingress_upstreams(),
        }
    }
}

/// IM foundation services embedded in the single-ingress application assembly.
/// These must not be HTTP-proxied to legacy per-service loopback ports.
pub fn is_assembly_embedded_im_service(service_id: &str) -> bool {
    matches!(
        canonical_service_id(service_id),
        "session-gateway"
            | "im-calls-service"
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
            | "portal-service"
            | "comms-social-service"
            | "comms-space-service"
    )
}

/// T1 commerce capability app-api authorities embedded by IM standalone assembly.
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

/// Sibling dependency app APIs embedded by IM standalone assembly in single-ingress mode.
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
    GatewayRuntimeMode::SingleIngress
}

/// Returns true when the gateway process should embed session-gateway instead of HTTP-proxying.
pub fn should_embed_session_gateway(config: &WebGatewayConfig) -> bool {
    let _ = config;
    true
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

/// Application-plane bridge to the embedded platform gateway IAM surface.
pub fn default_single_ingress_upstreams() -> Vec<ServiceUpstreamConfig> {
    let appbase_upstream = default_appbase_app_api_upstream();
    vec![service_upstream(
        "sdkwork-iam-app-api",
        appbase_upstream.as_str(),
    )]
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
    default_platform_api_gateway_base_url()
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
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{GatewayRuntimeMode, WebGatewayConfig};

    const GATEWAY_ENV_KEYS: &[&str] = &[
        "SDKWORK_IM_DEPLOYMENT_PROFILE",
        "SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL",
        "SDKWORK_API_CLOUD_GATEWAY_BASE_URL",
        "SDKWORK_API_CLOUD_GATEWAY_BIND",
    ];

    fn env_lock() -> MutexGuard<'static, ()> {
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

    fn clear_gateway_env() -> Vec<ScopedEnvVar> {
        GATEWAY_ENV_KEYS
            .iter()
            .map(|&name| ScopedEnvVar::remove(name))
            .collect()
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
deployment_profile = "standalone"
runtime_target = "server"
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
        let _lock = env_lock();
        let _clear = clear_gateway_env();
        let _standard_bind = ScopedEnvVar::set(
            "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
            "127.0.0.1:39080",
        );
        let _legacy_bind = ScopedEnvVar::set("SDKWORK_IM_WEB_GATEWAY_BIND", "127.0.0.1:18079");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.bind_addr, "127.0.0.1:39080");
    }

    #[test]
    fn test_should_embed_session_gateway_is_single_ingress_default() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();
        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::SingleIngress);
        assert!(super::should_embed_session_gateway(&config));
    }

    #[test]
    fn test_resolve_runtime_mode_from_env_is_single_ingress_for_all_profiles() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();

        assert_eq!(
            super::resolve_runtime_mode_from_env(),
            GatewayRuntimeMode::SingleIngress
        );

        for profile in ["standalone", "cloud"] {
            let _profile = ScopedEnvVar::set("SDKWORK_IM_DEPLOYMENT_PROFILE", profile);

            assert_eq!(
                super::resolve_runtime_mode_from_env(),
                GatewayRuntimeMode::SingleIngress,
                "profile {profile} must not select another runtime mode"
            );

            let config = WebGatewayConfig::from_env();
            assert_eq!(config.runtime_mode, GatewayRuntimeMode::SingleIngress);
        }
    }

    #[test]
    fn test_web_gateway_config_defaults_to_single_ingress_upstreams() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::SingleIngress);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:3900")
        );
        for service_id in [
            "session-gateway",
            "comms-conversation-service",
            "conversation-runtime",
            "projection-service",
            "ops-service",
            "social-service",
            "comms-social-service",
            "comms-space-service",
            "sdkwork-drive-app-api",
            "sdkwork-notary-app-api",
            "sdkwork-catalog-app-api",
            "sdkwork-order-app-api",
            "sdkwork-mail-app-api",
            "sdkwork-community-app-api",
            "sdkwork-course-app-api",
            "sdkwork-knowledgebase-app-api",
            "sdkwork-voice-app-api",
            "sdkwork-agents-app-api",
        ] {
            assert_eq!(
                config.upstream_base_url(service_id),
                None,
                "{service_id} must not get a default sidecar upstream"
            );
        }

        for upstream in &config.upstreams {
            for retired_suffix in ["080", "082", "091", "092", "093"] {
                let retired_port = format!("18{retired_suffix}");
                assert!(
                    !upstream.base_url.contains(retired_port.as_str()),
                    "{} must not use retired sidecar port {retired_port}",
                    upstream.service_id
                );
            }
        }
        assert!(super::is_assembly_embedded_im_service("social-service"));
        assert!(super::is_assembly_embedded_im_service(
            "conversation-runtime"
        ));
        assert!(super::is_standalone_embedded_dependency_service(
            "sdkwork-drive-app-api"
        ));
        assert!(super::is_standalone_embedded_dependency_service(
            "sdkwork-catalog-app-api"
        ));
        assert!(super::is_standalone_embedded_dependency_service(
            "sdkwork-agents-app-api"
        ));
    }

    #[test]
    fn test_web_gateway_config_uses_platform_gateway_root_for_iam_only() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();
        let _platform_gateway = ScopedEnvVar::set(
            "SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL",
            "http://127.0.0.1:4900/",
        );
        let _gateway_base_url = ScopedEnvVar::set(
            "SDKWORK_API_CLOUD_GATEWAY_BASE_URL",
            "http://127.0.0.1:5900",
        );
        let _gateway_bind = ScopedEnvVar::set("SDKWORK_API_CLOUD_GATEWAY_BIND", "127.0.0.1:6900");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::SingleIngress);
        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:4900")
        );
        assert_eq!(config.upstream_base_url("sdkwork-drive-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-notary-app-api"), None);
        assert_eq!(config.upstream_base_url("sdkwork-catalog-app-api"), None);
    }

    #[test]
    fn test_web_gateway_config_derives_iam_gateway_root_from_gateway_bind() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();
        let _gateway_bind = ScopedEnvVar::set("SDKWORK_API_CLOUD_GATEWAY_BIND", "127.0.0.1:7900");

        let config = WebGatewayConfig::from_env();

        assert_eq!(
            config.upstream_base_url("sdkwork-iam-app-api"),
            Some("http://127.0.0.1:7900")
        );
        assert_eq!(config.upstream_base_url("sdkwork-mail-app-api"), None);
    }

    #[test]
    fn test_web_gateway_config_ignores_retired_appbase_explicit_upstream() {
        let _lock = env_lock();
        let _clear = clear_gateway_env();
        let _appbase_upstream = ScopedEnvVar::set(
            "SDKWORK_IM_APPBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:19090/",
        );
        let _appbase_bind_addr =
            ScopedEnvVar::set("SDKWORK_APPBASE_APP_API_BIND_ADDR", "127.0.0.1:28090");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::SingleIngress);
        assert_eq!(config.upstream_base_url("sdkwork-iam-app-api"), Some("http://127.0.0.1:3900"));
    }

}
