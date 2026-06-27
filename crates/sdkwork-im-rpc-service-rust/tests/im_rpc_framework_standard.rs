#[test]
fn im_rpc_sdk_manifest_declares_discovery_and_resilience() {
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json");
    let source = std::fs::read_to_string(&manifest_path).expect("read rpc sdk manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&source).expect("parse rpc sdk manifest");

    assert_eq!(manifest["kind"], "sdkwork.rpc.manifest");
    assert_eq!(
        manifest["discoveryServiceName"],
        "sdkwork-communication-app-rpc"
    );
    assert_eq!(manifest["defaultResilienceProfile"], "rpc-default");
    assert_eq!(manifest["sdkFamily"], "sdkwork-im-rpc-sdk");
}

#[test]
fn im_rpc_client_bootstrap_static_profile_requires_endpoint() {
    use sdkwork_im_rpc_service_rust::{
        IM_RPC_RESOLVER_PROFILE_ENV, IM_RPC_STATIC_ENDPOINT_ENV,
        build_im_rpc_name_resolver_from_env,
    };

    let _guard = env_test_lock();
    unsafe {
        std::env::set_var(IM_RPC_RESOLVER_PROFILE_ENV, "static");
        std::env::remove_var(IM_RPC_STATIC_ENDPOINT_ENV);
    }

    let error = build_im_rpc_name_resolver_from_env().expect_err("missing static endpoint");
    assert!(error.to_string().contains("SDKWORK_IM_RPC_STATIC_ENDPOINT"));

    unsafe {
        std::env::remove_var(IM_RPC_RESOLVER_PROFILE_ENV);
    }
}

#[test]
fn im_rpc_framework_bootstrap_skips_client_when_unconfigured() {
    use sdkwork_im_rpc_service_rust::initialize_im_rpc_framework_from_env;
    use sdkwork_rpc_framework_core::ResolverProfile;

    let _guard = env_test_lock();
    unsafe {
        std::env::remove_var("SDKWORK_IM_RPC_RESOLVER_PROFILE");
        std::env::remove_var("SDKWORK_IM_RPC_STATIC_ENDPOINT");
        std::env::remove_var("SDKWORK_IM_DISCOVERY_ENDPOINT");
    }

    let bootstrap = initialize_im_rpc_framework_from_env().expect("bootstrap");
    assert_eq!(bootstrap.resolver_profile, ResolverProfile::Static);
    assert_eq!(bootstrap.default_resilience_profile, "rpc-default");
    assert!(bootstrap.client_resolver.is_none());
}

#[test]
fn im_discovery_advertised_endpoint_uses_public_endpoint_when_configured() {
    use sdkwork_im_rpc_service_rust::{ImRpcServerConfig, im_discovery_config_from_env};

    let _guard = env_test_lock();
    unsafe {
        std::env::set_var("SDKWORK_IM_DISCOVERY_ENDPOINT", "http://127.0.0.1:19090");
    }
    let config = ImRpcServerConfig {
        bind_addr: "127.0.0.1:50051".to_string(),
        public_endpoint: Some("grpc://public.example:50051".to_string()),
        ..ImRpcServerConfig::local_default()
    };

    let discovery = im_discovery_config_from_env(&config).expect("discovery config");
    assert_eq!(discovery.advertised_endpoint, "grpc://public.example:50051");

    unsafe {
        std::env::remove_var("SDKWORK_IM_DISCOVERY_ENDPOINT");
    }
}

fn env_test_lock() -> std::sync::MutexGuard<'static, ()> {
    static ENV_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    ENV_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
