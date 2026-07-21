use crate::RpcDeadline;

pub const DEFAULT_RPC_MAX_DECODING_MESSAGE_SIZE: usize = 4 * 1024 * 1024;
pub const DEFAULT_RPC_MAX_ENCODING_MESSAGE_SIZE: usize = 16 * 1024 * 1024;
pub const RPC_MESSAGE_SIZE_HARD_MAX: usize = 64 * 1024 * 1024;
const RPC_SERVICE_MESH_MTLS_VERIFIED_ENV: &str = "SDKWORK_IM_RPC_SERVICE_MESH_MTLS_VERIFIED";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcServerConfig {
    pub bind_addr: String,
    pub public_endpoint: Option<String>,
    pub enable_health: bool,
    pub enable_reflection: bool,
    pub require_tls: bool,
    pub require_mtls: bool,
    pub enable_grpc_web: bool,
    pub default_deadline: RpcDeadline,
    pub max_decoding_message_size: usize,
    pub max_encoding_message_size: usize,
}

impl ImRpcServerConfig {
    pub fn local_default() -> Self {
        Self {
            bind_addr: "127.0.0.1:50051".to_owned(),
            public_endpoint: Some("http://127.0.0.1:50051".to_owned()),
            enable_health: true,
            enable_reflection: false,
            require_tls: false,
            require_mtls: false,
            enable_grpc_web: false,
            default_deadline: RpcDeadline::default(),
            max_decoding_message_size: DEFAULT_RPC_MAX_DECODING_MESSAGE_SIZE,
            max_encoding_message_size: DEFAULT_RPC_MAX_ENCODING_MESSAGE_SIZE,
        }
    }

    pub fn validate_message_sizes(&self) -> Result<(), String> {
        validate_message_size("max_decoding_message_size", self.max_decoding_message_size)?;
        validate_message_size("max_encoding_message_size", self.max_encoding_message_size)
    }

    pub fn validate_plaintext_listener(&self) -> Result<(), String> {
        self.validate_plaintext_listener_for_environment(
            im_app_context::is_production_like_im_environment(),
            service_mesh_mtls_verified(),
        )
    }

    fn validate_plaintext_listener_for_environment(
        &self,
        production_like: bool,
        mesh_mtls_verified: bool,
    ) -> Result<(), String> {
        self.validate_message_sizes()?;
        if self.require_tls || self.require_mtls {
            return Err(
                "plaintext RPC router cannot satisfy require_tls/require_mtls; use the mTLS router"
                    .into(),
            );
        }
        let bind_addr = self
            .bind_addr
            .parse::<std::net::SocketAddr>()
            .map_err(|error| format!("invalid RPC bind address `{}`: {error}", self.bind_addr))?;
        if production_like && !bind_addr.ip().is_loopback() && !mesh_mtls_verified {
            return Err(format!(
                "production-like non-loopback plaintext RPC requires {RPC_SERVICE_MESH_MTLS_VERIFIED_ENV}=true or the mTLS router"
            ));
        }
        Ok(())
    }
}

impl Default for ImRpcServerConfig {
    fn default() -> Self {
        Self::local_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcClientConfig {
    pub endpoint: String,
    pub require_tls: bool,
    pub require_mtls: bool,
    pub default_deadline: RpcDeadline,
    pub max_decoding_message_size: usize,
    pub max_encoding_message_size: usize,
}

impl ImRpcClientConfig {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            require_tls: false,
            require_mtls: false,
            default_deadline: RpcDeadline::default(),
            max_decoding_message_size: DEFAULT_RPC_MAX_DECODING_MESSAGE_SIZE,
            max_encoding_message_size: DEFAULT_RPC_MAX_ENCODING_MESSAGE_SIZE,
        }
    }
}

fn validate_message_size(name: &str, value: usize) -> Result<(), String> {
    if value == 0 || value > RPC_MESSAGE_SIZE_HARD_MAX {
        return Err(format!(
            "{name} must be between 1 and {RPC_MESSAGE_SIZE_HARD_MAX} bytes, actual={value}"
        ));
    }
    Ok(())
}

fn service_mesh_mtls_verified() -> bool {
    std::env::var(RPC_SERVICE_MESH_MTLS_VERIFIED_ENV)
        .ok()
        .map(|value| value.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unbounded_or_zero_message_sizes() {
        let mut config = ImRpcServerConfig::local_default();
        config.max_encoding_message_size = usize::MAX;
        assert!(config.validate_message_sizes().is_err());
        config.max_encoding_message_size = DEFAULT_RPC_MAX_ENCODING_MESSAGE_SIZE;
        config.max_decoding_message_size = 0;
        assert!(config.validate_message_sizes().is_err());
    }

    #[test]
    fn production_non_loopback_plaintext_requires_verified_mesh_mtls() {
        let mut config = ImRpcServerConfig::local_default();
        config.bind_addr = "0.0.0.0:50051".into();
        assert!(
            config
                .validate_plaintext_listener_for_environment(true, false)
                .is_err()
        );
        assert!(
            config
                .validate_plaintext_listener_for_environment(true, true)
                .is_ok()
        );
    }

    #[test]
    fn production_loopback_plaintext_is_allowed_for_same_host_composition() {
        let config = ImRpcServerConfig::local_default();
        assert!(
            config
                .validate_plaintext_listener_for_environment(true, false)
                .is_ok()
        );
    }
}
