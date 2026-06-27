use crate::service_binding::RpcMethodBinding;

pub const RPC_METHOD_BINDINGS: &[RpcMethodBinding] =
    include!(concat!(env!("OUT_DIR"), "/rpc_method_bindings.rs"));
