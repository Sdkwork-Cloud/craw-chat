use anyhow::{Context, Result};
use std::{
    env,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use url::Url;

const DEFAULT_RUNTIME_BIND_ADDR: &str = "127.0.0.1:0";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandaloneConfig {
    pub runtime_bind_addr: String,
    pub admin_proxy_target: String,
    pub portal_api_base_url: String,
    pub admin_sandbox_enabled: bool,
    pub admin_sandbox_storage_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Default)]
pub struct StandaloneConfigLoader;

impl StandaloneConfigLoader {
    pub fn from_env() -> Result<(Self, StandaloneConfig)> {
        Ok((
            Self,
            StandaloneConfig {
                runtime_bind_addr: resolve_runtime_bind_addr(),
                admin_proxy_target: resolve_admin_proxy_target()
                    .context("failed to resolve desktop admin proxy target")?,
                portal_api_base_url: resolve_portal_api_base_url()
                    .context("failed to resolve desktop portal api base url")?,
                admin_sandbox_enabled: resolve_admin_sandbox_enabled(),
                admin_sandbox_storage_file: resolve_admin_sandbox_storage_file(),
            },
        ))
    }
}

fn resolve_runtime_bind_addr() -> String {
    env::var("SDKWORK_DESKTOP_RUNTIME_BIND")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_RUNTIME_BIND_ADDR.to_owned())
}

fn resolve_admin_proxy_target() -> Result<String> {
    let candidate = ["SDKWORK_ADMIN_PROXY_TARGET", "SDKWORK_ADMIN_BIND"]
        .iter()
        .find_map(|key| env::var(key).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    match candidate {
        Some(value) => normalize_upstream_url(value.as_str()),
        None => Ok(String::new()),
    }
}

fn resolve_portal_api_base_url() -> Result<String> {
    for key in [
        "SDKWORK_IM_PORTAL_API_BASE_URL",
        "SDKWORK_PORTAL_API_BASE_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
    ] {
        if let Some(value) = env::var(key)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
        {
            return normalize_explicit_portal_api_base_url(key, value.as_str());
        }
    }

    if let Some(value) = env::var("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        return normalize_portal_api_base_url_from_bind_addr(
            "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
            value.as_str(),
        );
    }

    Ok("http://127.0.0.1:18079".to_owned())
}

fn normalize_upstream_url(value: &str) -> Result<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        anyhow::bail!("desktop admin proxy target cannot be empty");
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return Ok(trimmed.to_owned());
    }

    Ok(format!("http://{trimmed}"))
}

fn normalize_explicit_portal_api_base_url(env_name: &str, value: &str) -> Result<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        anyhow::bail!("{env_name} cannot be empty");
    }

    let url = Url::parse(trimmed)
        .with_context(|| format!("{env_name} must be an absolute http(s) url: {trimmed}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        anyhow::bail!("{env_name} must use http:// or https://");
    }

    let host = url
        .host_str()
        .with_context(|| format!("{env_name} must include a host"))?;
    if is_unspecified_host(host) {
        anyhow::bail!(
            "{env_name} must not use an unspecified bind host like 0.0.0.0 or ::; set a browser-reachable url"
        );
    }

    Ok(trimmed.to_owned())
}

fn normalize_portal_api_base_url_from_bind_addr(env_name: &str, value: &str) -> Result<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        anyhow::bail!("{env_name} cannot be empty when used as a portal api fallback");
    }

    if let Ok(socket_addr) = trimmed.parse::<SocketAddr>() {
        return Ok(format!(
            "http://{}:{}",
            loopback_safe_host_for_ip(socket_addr.ip()),
            socket_addr.port()
        ));
    }

    let mut url = Url::parse(normalize_upstream_url(trimmed)?.as_str())
        .with_context(|| format!("{env_name} must be a host:port or absolute url: {trimmed}"))?;

    let Some(host) = url.host_str().map(str::to_owned) else {
        anyhow::bail!("{env_name} must include a host");
    };
    if is_unspecified_host(host.as_str()) {
        let normalized_host = if matches!(url.host(), Some(url::Host::Ipv6(_))) {
            "::1"
        } else {
            "127.0.0.1"
        };
        url.set_host(Some(normalized_host))
            .expect("loopback replacement host should be valid");
    }

    Ok(url.to_string().trim_end_matches('/').to_owned())
}

fn is_unspecified_host(host: &str) -> bool {
    matches!(host, "0.0.0.0" | "::")
}

fn loopback_safe_host_for_ip(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".into(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "[::1]".into(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    }
}

fn resolve_admin_sandbox_enabled() -> bool {
    ["SDKWORK_ADMIN_SANDBOX", "SDKWORK_ADMIN_SANDBOX_MODE"]
        .iter()
        .find_map(|key| env::var(key).ok())
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn resolve_admin_sandbox_storage_file() -> Option<PathBuf> {
    env::var("SDKWORK_ADMIN_SANDBOX_STORAGE_FILE")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|error| error.into_inner())
    }

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = env::var(name).ok();
            unsafe {
                env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = env::var(name).ok();
            unsafe {
                env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => unsafe {
                    env::set_var(self.name, value);
                },
                None => unsafe {
                    env::remove_var(self.name);
                },
            }
        }
    }

    #[test]
    fn resolve_portal_api_base_url_prefers_explicit_url_and_falls_back_to_ingress_bind() {
        let _guard = env_guard();
        let _explicit = ScopedEnvVar::set(
            "SDKWORK_IM_PORTAL_API_BASE_URL",
            " https://portal-api.example.com/runtime-edge/ ",
        );
        let _application_public_http =
            ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
        let _bind = ScopedEnvVar::set(
            "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
            "127.0.0.1:19990",
        );
        assert_eq!(
            resolve_portal_api_base_url().expect("explicit portal api base url should resolve"),
            "https://portal-api.example.com/runtime-edge"
        );

        unsafe {
            env::remove_var("SDKWORK_IM_PORTAL_API_BASE_URL");
            env::set_var(
                "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
                " https://im.example.com/ ",
            );
        }
        assert_eq!(
            resolve_portal_api_base_url()
                .expect("application public HTTP URL should resolve as public portal api fallback"),
            "https://im.example.com"
        );

        unsafe {
            env::remove_var("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
            env::set_var(
                "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
                " https://im.example.com/api-edge/ ",
            );
        }
        assert_eq!(
            resolve_portal_api_base_url()
                .expect("application public HTTP URL should resolve as public portal api fallback"),
            "https://im.example.com/api-edge"
        );

        unsafe {
            env::remove_var("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
        }
        assert_eq!(
            resolve_portal_api_base_url().expect("ingress bind fallback should resolve"),
            "http://127.0.0.1:19990"
        );

        unsafe {
            env::set_var(
                "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
                "0.0.0.0:29990",
            );
        }
        assert_eq!(
            resolve_portal_api_base_url().expect("wildcard ipv4 bind should normalize"),
            "http://127.0.0.1:29990"
        );

        unsafe {
            env::set_var("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND", "[::]:39990");
        }
        assert_eq!(
            resolve_portal_api_base_url().expect("wildcard ipv6 bind should normalize"),
            "http://[::1]:39990"
        );
    }

    #[test]
    fn resolve_portal_api_base_url_rejects_unspecified_explicit_public_url() {
        let _guard = env_guard();
        let _explicit = ScopedEnvVar::set("SDKWORK_IM_PORTAL_API_BASE_URL", "http://0.0.0.0:18079");
        let _application_public_http =
            ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
        let _bind = ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");

        let error = resolve_portal_api_base_url()
            .expect_err("unspecified explicit public url should be rejected");
        assert!(error.to_string().contains("SDKWORK_IM_PORTAL_API_BASE_URL"));

        unsafe {
            env::remove_var("SDKWORK_IM_PORTAL_API_BASE_URL");
            env::set_var(
                "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
                "http://0.0.0.0:18079",
            );
        }

        let error = resolve_portal_api_base_url()
            .expect_err("unspecified application public HTTP URL should be rejected");
        assert!(error
            .to_string()
            .contains("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL"));
    }
}
