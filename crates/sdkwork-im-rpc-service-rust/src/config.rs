use crate::RpcDeadline;

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
            max_decoding_message_size: 4 * 1024 * 1024,
            max_encoding_message_size: usize::MAX,
        }
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
            max_decoding_message_size: 4 * 1024 * 1024,
            max_encoding_message_size: usize::MAX,
        }
    }
}
