use std::sync::Arc;

use sdkwork_rpc_discovery::{
    DiscoveryInstanceConfig, DiscoveryInstanceLifecycle, RegistrationMetadataInput,
    build_registration_metadata, default_instance_id, grpc_advertised_endpoint,
};

use crate::ImRpcServerConfig;

pub const IM_DISCOVERY_ENDPOINT_ENV: &str = "SDKWORK_IM_DISCOVERY_ENDPOINT";
pub const IM_DISCOVERY_NAMESPACE_ENV: &str = "SDKWORK_IM_DISCOVERY_NAMESPACE";
pub const IM_DISCOVERY_ENVIRONMENT_ENV: &str = "SDKWORK_IM_DISCOVERY_ENVIRONMENT";
pub const IM_DISCOVERY_SERVICE_NAME_ENV: &str = "SDKWORK_IM_DISCOVERY_SERVICE_NAME";
pub const IM_DISCOVERY_INSTANCE_ID_ENV: &str = "SDKWORK_IM_DISCOVERY_INSTANCE_ID";
pub const IM_DISCOVERY_LEASE_TTL_SECONDS_ENV: &str = "SDKWORK_IM_DISCOVERY_LEASE_TTL_SECONDS";
pub const IM_DISCOVERY_SUBJECT_ID_ENV: &str = "SDKWORK_IM_DISCOVERY_SUBJECT_ID";

pub const IM_DISCOVERY_SERVICE_NAME_DEFAULT: &str = "sdkwork-communication-app-rpc";
pub const IM_DISCOVERY_MANIFEST_REF: &str =
    "sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json";

pub fn im_discovery_config_from_env(
    server_config: &ImRpcServerConfig,
) -> Option<DiscoveryInstanceConfig> {
    let discovery_endpoint = std::env::var(IM_DISCOVERY_ENDPOINT_ENV).ok()?;
    if discovery_endpoint.trim().is_empty() {
        return None;
    }

    let service_name = std::env::var(IM_DISCOVERY_SERVICE_NAME_ENV)
        .unwrap_or_else(|_| IM_DISCOVERY_SERVICE_NAME_DEFAULT.to_string());
    let instance_id = std::env::var(IM_DISCOVERY_INSTANCE_ID_ENV)
        .unwrap_or_else(|_| default_instance_id(&service_name));
    let lease_ttl_seconds = std::env::var(IM_DISCOVERY_LEASE_TTL_SECONDS_ENV)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(30);
    let subject_id = std::env::var(IM_DISCOVERY_SUBJECT_ID_ENV)
        .unwrap_or_else(|_| "sdkwork-im-session-gateway-rpc".to_string());

    let metadata = build_registration_metadata(RegistrationMetadataInput {
        rpc_surface: "app",
        sdk_family: "sdkwork-im-rpc-sdk",
        domain: "communication",
        proto_packages: &[
            "sdkwork.communication.app.v3",
            "sdkwork.communication.backend.v3",
            "sdkwork.communication.internal.v1",
        ],
        operation_manifest_ref: IM_DISCOVERY_MANIFEST_REF,
        deployment_profile: std::env::var("SDKWORK_IM_DEPLOYMENT_PROFILE")
            .ok()
            .as_deref(),
        runtime_target: std::env::var("SDKWORK_IM_RUNTIME_TARGET").ok().as_deref(),
    });

    Some(DiscoveryInstanceConfig {
        discovery_endpoint,
        namespace: std::env::var(IM_DISCOVERY_NAMESPACE_ENV)
            .unwrap_or_else(|_| "sdkwork".to_string()),
        environment: std::env::var(IM_DISCOVERY_ENVIRONMENT_ENV)
            .or_else(|_| std::env::var("SDKWORK_IM_ENVIRONMENT"))
            .unwrap_or_else(|_| "development".to_string()),
        service_name,
        instance_id,
        advertised_endpoint: im_advertised_endpoint(server_config),
        protocol: "grpc".to_string(),
        version: std::env::var("SDKWORK_IM_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
        region: std::env::var("SDKWORK_IM_REGION").unwrap_or_else(|_| "local".to_string()),
        zone: std::env::var("SDKWORK_IM_ZONE").unwrap_or_else(|_| "local".to_string()),
        lease_ttl_seconds,
        subject_id,
        metadata,
        revision_cas_on_register: true,
        expected_revision: None,
    })
}

pub async fn register_im_discovery_instance(
    server_config: &ImRpcServerConfig,
) -> Result<
    Option<Arc<sdkwork_rpc_discovery::DiscoveryInstanceHandle>>,
    sdkwork_rpc_discovery::DiscoveryRegistrationError,
> {
    let Some(config) = im_discovery_config_from_env(server_config) else {
        return Ok(None);
    };

    let handle = DiscoveryInstanceLifecycle::register(config).await?;
    Ok(Some(Arc::new(handle)))
}

fn im_advertised_endpoint(server_config: &ImRpcServerConfig) -> String {
    if let Some(public_endpoint) = server_config
        .public_endpoint
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        if public_endpoint.starts_with("grpc://") || public_endpoint.starts_with("grpcs://") {
            return public_endpoint.to_string();
        }
        if let Some(rest) = public_endpoint.strip_prefix("http://") {
            return format!("grpc://{rest}");
        }
        if let Some(rest) = public_endpoint.strip_prefix("https://") {
            return format!("grpcs://{rest}");
        }
        return grpc_advertised_endpoint(public_endpoint);
    }

    grpc_advertised_endpoint(&server_config.bind_addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advertised_endpoint_prefers_public_grpc_endpoint() {
        let config = ImRpcServerConfig {
            bind_addr: "127.0.0.1:50051".to_string(),
            public_endpoint: Some("grpc://public.example:50051".to_string()),
            ..ImRpcServerConfig::local_default()
        };

        assert_eq!(
            im_advertised_endpoint(&config),
            "grpc://public.example:50051"
        );
    }

    #[test]
    fn advertised_endpoint_converts_http_public_endpoint_to_grpc() {
        let config = ImRpcServerConfig {
            bind_addr: "127.0.0.1:50051".to_string(),
            public_endpoint: Some("http://public.example:50051".to_string()),
            ..ImRpcServerConfig::local_default()
        };

        assert_eq!(
            im_advertised_endpoint(&config),
            "grpc://public.example:50051"
        );
    }
}
