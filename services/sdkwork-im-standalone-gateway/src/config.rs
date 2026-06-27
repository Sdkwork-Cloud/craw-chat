use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayFileConfig {
    pub service: ServiceSection,
    pub server: ServerSection,
    pub cors: CorsSection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceSection {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSection {
    pub bind: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorsSection {
    #[serde(default)]
    pub allow_any_origin: bool,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedGatewayConfig {
    pub service_name: String,
    pub environment: String,
    pub bind: String,
    pub allow_any_origin: bool,
    pub allowed_origins: Vec<String>,
}

pub fn load_gateway_config(config_path: &Path) -> Result<GatewayFileConfig, String> {
    let raw = fs::read_to_string(config_path).map_err(|error| {
        format!(
            "failed to read standalone gateway config {}: {error}",
            config_path.display()
        )
    })?;
    toml::from_str(&raw).map_err(|error| {
        format!(
            "failed to parse standalone gateway config {}: {error}",
            config_path.display()
        )
    })
}

pub fn resolve_gateway_config(
    file_config: GatewayFileConfig,
) -> Result<ResolvedGatewayConfig, String> {
    let environment = file_config.service.environment.trim().to_ascii_lowercase();
    let is_production = environment == "production" || environment == "prod";

    // Fail-closed: reject allow_any_origin in production to prevent CSRF attacks.
    // If no explicit origins are configured in production, fall back to a safe
    // default and emit a warning rather than allowing all origins.
    let allow_any_origin = if is_production && file_config.cors.allow_any_origin {
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.cors_any_origin_blocked",
            environment = %environment,
            "CORS allow_any_origin=true is not permitted in production — falling back to explicit origins"
        );
        false
    } else {
        file_config.cors.allow_any_origin
    };

    let allowed_origins = if is_production && file_config.cors.allowed_origins.is_empty() && !allow_any_origin {
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.cors_no_explicit_origins",
            environment = %environment,
            "No explicit CORS origins configured in production — set SDKWORK_IM_BROWSER_ORIGINS or config [cors].allowed_origins"
        );
        // Use the same default origins as the cloud gateway as a safety net.
        vec![
            "https://api.sdkwork.com".to_owned(),
            "https://api-dev.sdkwork.com".to_owned(),
        ]
    } else {
        file_config.cors.allowed_origins
    };

    Ok(ResolvedGatewayConfig {
        service_name: file_config.service.name,
        environment: file_config.service.environment,
        bind: read_env_override("SDKWORK_IM_STANDALONE_GATEWAY_BIND")
            .or_else(|| read_env_override("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND"))
            .unwrap_or(file_config.server.bind),
        allow_any_origin,
        allowed_origins,
    })
}

fn read_env_override(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn resolve_config_path(args: &[String]) -> Result<String, String> {
    if let Ok(path) = std::env::var("SDKWORK_IM_STANDALONE_GATEWAY_CONFIG") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    for (index, arg) in args.iter().enumerate() {
        if arg == "--config" {
            let value = args
                .get(index + 1)
                .ok_or_else(|| "--config requires a path".to_string())?;
            return Ok(value.clone());
        }
        if arg == "-h" || arg == "--help" {
            println!("Usage: sdkwork-im-standalone-gateway [--config <path>]");
            println!("Embed IAM and IM application ingress on one standalone bind.");
            std::process::exit(0);
        }
    }

    let environment = std::env::var("SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "development".to_string());

    Ok(format!(
        "configs/sdkwork-im-standalone-gateway.{environment}.toml"
    ))
}
