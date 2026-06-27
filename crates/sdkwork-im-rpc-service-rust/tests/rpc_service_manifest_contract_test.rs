use std::collections::HashSet;
use std::convert::Infallible;

use sdkwork_im_rpc_service_rust::{
    RPC_METHOD_BINDINGS, RPC_SDK_FAMILY, RPC_SERVICE_BINDINGS, RpcMethodBinding,
    RpcMethodRuntimeAdapter, RpcRuntimeAdapter, RpcServiceBinding, bind_all_rpc_methods,
    bind_all_rpc_services,
};

#[test]
fn test_rpc_service_manifest_declares_im_rpc_sdk_family() {
    assert_eq!(RPC_SDK_FAMILY, "sdkwork-im-rpc-sdk");
}

#[test]
fn test_rpc_service_manifest_lists_unique_standard_services() {
    assert_eq!(RPC_SERVICE_BINDINGS.len(), 22);

    let mut service_keys = HashSet::new();
    for binding in RPC_SERVICE_BINDINGS {
        assert!(
            matches!(binding.surface, "app" | "backend" | "internal"),
            "unexpected RPC surface for {}: {}",
            binding.service_key,
            binding.surface,
        );
        assert!(
            service_keys.insert(binding.service_key),
            "duplicated RPC service binding: {}",
            binding.service_key,
        );
    }
}

#[test]
fn test_bind_all_rpc_services_registers_every_manifest_service() {
    #[derive(Default)]
    struct FakeRpcRuntimeAdapter {
        registered_service_keys: Vec<&'static str>,
    }

    impl RpcRuntimeAdapter for FakeRpcRuntimeAdapter {
        type Error = Infallible;

        fn register_service(
            &mut self,
            binding: &'static RpcServiceBinding,
        ) -> Result<(), Self::Error> {
            self.registered_service_keys.push(binding.service_key);
            Ok(())
        }
    }

    let mut adapter = FakeRpcRuntimeAdapter::default();
    bind_all_rpc_services(&mut adapter).expect("fake adapter registration cannot fail");

    assert_eq!(
        adapter.registered_service_keys.len(),
        RPC_SERVICE_BINDINGS.len()
    );
    assert_eq!(
        adapter.registered_service_keys,
        RPC_SERVICE_BINDINGS
            .iter()
            .map(|binding| binding.service_key)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_rpc_method_manifest_lists_unique_standard_methods() {
    assert_eq!(RPC_METHOD_BINDINGS.len(), 158);

    let mut method_keys = HashSet::new();
    for binding in RPC_METHOD_BINDINGS {
        assert!(
            matches!(binding.surface, "app" | "backend" | "internal"),
            "unexpected RPC surface for {}: {}",
            binding.method_key,
            binding.surface,
        );
        assert!(
            matches!(binding.idempotency, "none" | "optional" | "required"),
            "unexpected idempotency for {}: {}",
            binding.method_key,
            binding.idempotency,
        );
        assert!(
            matches!(binding.streaming, "unary" | "server" | "client" | "bidi"),
            "unexpected streaming mode for {}: {}",
            binding.method_key,
            binding.streaming,
        );
        assert_eq!(
            binding.method_key,
            format!("{}.{}/{}", binding.package, binding.service, binding.method),
            "RPC method key must be package.service/method for {}",
            binding.operation_id,
        );
        assert!(
            binding.operation_id.contains('.'),
            "RPC method {} must declare a dotted operation id",
            binding.method_key,
        );
        assert!(
            !binding.auth.is_empty(),
            "RPC method {} must declare auth metadata",
            binding.method_key,
        );
        assert!(
            !binding.owner.is_empty(),
            "RPC method {} must declare owner metadata",
            binding.method_key,
        );
        assert!(
            !binding.compatibility.is_empty(),
            "RPC method {} must declare compatibility metadata",
            binding.method_key,
        );
        assert!(
            method_keys.insert(binding.method_key),
            "duplicated RPC method binding: {}",
            binding.method_key,
        );
    }

    for required_method_key in [
        "sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat",
        "sdkwork.communication.app.v3.RealtimeService/WatchRealtimeEvents",
        "sdkwork.communication.app.v3.ConversationService/CreateConversation",
        "sdkwork.communication.app.v3.MessageService/CreateConversationMessage",
        "sdkwork.communication.app.v3.RoomService/CreateRoom",
        "sdkwork.communication.app.v3.StreamService/WatchStreamFrames",
        "sdkwork.communication.app.v3.CallService/WatchCallSignals",
        "sdkwork.communication.backend.v3.RealtimeNodeAdminService/MigrateRealtimeNodeRoutes",
        "sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairSharedChannelSync",
        "sdkwork.communication.internal.v1.RouteLeaseService/ClaimRouteLease",
        "sdkwork.communication.internal.v1.DomainEventRelayService/WatchDomainEvents",
    ] {
        assert!(
            method_keys.contains(required_method_key),
            "RPC method manifest must bind {required_method_key}",
        );
    }
}

#[test]
fn test_bind_all_rpc_methods_registers_every_manifest_method() {
    #[derive(Default)]
    struct FakeRpcMethodRuntimeAdapter {
        registered_method_keys: Vec<&'static str>,
    }

    impl RpcMethodRuntimeAdapter for FakeRpcMethodRuntimeAdapter {
        type Error = Infallible;

        fn register_method(
            &mut self,
            binding: &'static RpcMethodBinding,
        ) -> Result<(), Self::Error> {
            self.registered_method_keys.push(binding.method_key);
            Ok(())
        }
    }

    let mut adapter = FakeRpcMethodRuntimeAdapter::default();
    bind_all_rpc_methods(&mut adapter).expect("fake adapter registration cannot fail");

    assert_eq!(
        adapter.registered_method_keys.len(),
        RPC_METHOD_BINDINGS.len()
    );
    assert_eq!(
        adapter.registered_method_keys,
        RPC_METHOD_BINDINGS
            .iter()
            .map(|binding| binding.method_key)
            .collect::<Vec<_>>()
    );
}
