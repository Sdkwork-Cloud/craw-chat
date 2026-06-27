//! Rust RPC service binding scaffold for the `sdkwork-im-rpc-sdk` family.

pub mod admission;
pub mod config;
pub mod deadline;
pub mod dispatcher;
pub mod error;
pub mod health;
pub mod metadata;
pub mod method_manifest;
pub mod rpc_client_bootstrap;
pub mod rpc_discovery;
pub mod rpc_framework_bootstrap;
pub mod rpc_server;
pub mod service_binding;
pub mod service_manifest;
pub mod tonic_service_adapters;

pub use admission::{
    admit_app_unary_request, admit_internal_unary_request, require_app_session_auth,
    require_idempotency_key, require_service_mtls_auth, resolve_service_identity,
};
pub use config::{ImRpcClientConfig, ImRpcServerConfig};
pub use deadline::RpcDeadline;
pub use dispatcher::{
    ImRpcBoxFuture, ImRpcBoxStream, ImRpcResponseStream, ImRpcRuntimeDispatcher,
    ImRpcStreamRequest, ImRpcStreamResponse, ImRpcUnaryRequest, ImRpcUnaryResponse,
    dispatch_server_stream_rpc, dispatch_unary_rpc,
};
pub use error::{ImRpcError, map_rpc_error_to_status};
pub use health::{ImRpcHealthService, build_im_rpc_health_server};
pub use metadata::{
    METADATA_ACCESS_TOKEN, METADATA_AUTHORIZATION, METADATA_CLIENT_VERSION,
    METADATA_IDEMPOTENCY_KEY, METADATA_REQUEST_HASH, METADATA_REQUEST_ID,
    METADATA_SERVICE_IDENTITY, METADATA_TRACEPARENT, RpcMetadata,
};
pub use method_manifest::RPC_METHOD_BINDINGS;
pub use rpc_client_bootstrap::{
    IM_DEFAULT_RESILIENCE_PROFILE, IM_RPC_RESOLVER_PROFILE_ENV, IM_RPC_RESOLVER_SUBJECT_ID_ENV,
    IM_RPC_STATIC_ENDPOINT_ENV, ImRpcNameResolverBootstrap, build_im_rpc_name_resolver_from_env,
    im_rpc_resolver_profile_from_env,
};
pub use rpc_discovery::{
    IM_DISCOVERY_ENDPOINT_ENV, IM_DISCOVERY_MANIFEST_REF, IM_DISCOVERY_SERVICE_NAME_DEFAULT,
    IM_DISCOVERY_SERVICE_NAME_ENV, im_discovery_config_from_env, register_im_discovery_instance,
};
pub use rpc_framework_bootstrap::{ImRpcFrameworkBootstrap, initialize_im_rpc_framework_from_env};
pub use rpc_server::serve_im_rpc_with_discovery;
pub use service_binding::{
    RpcMethodBinding, RpcMethodRuntimeAdapter, RpcRuntimeAdapter, RpcServiceBinding,
    bind_all_rpc_methods, bind_all_rpc_services,
};
pub use service_manifest::{RPC_SDK_FAMILY, RPC_SERVICE_BINDINGS};
pub use tonic_service_adapters::*;
pub use tonic_service_adapters::{
    GENERATED_TONIC_SERVICE_ADAPTER_COUNT, IM_RPC_SERVICE_KEYS, PresenceServiceAdapter,
    build_im_rpc_service_router, build_im_rpc_service_router_with_config,
    build_im_rpc_service_router_with_config_for_services,
};
