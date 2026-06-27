#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RpcServiceBinding {
    pub service_key: &'static str,
    pub package: &'static str,
    pub service: &'static str,
    pub surface: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RpcMethodBinding {
    pub method_key: &'static str,
    pub package: &'static str,
    pub service: &'static str,
    pub method: &'static str,
    pub surface: &'static str,
    pub operation_id: &'static str,
    pub auth: &'static str,
    pub idempotency: &'static str,
    pub streaming: &'static str,
    pub owner: &'static str,
    pub compatibility: &'static str,
}

pub trait RpcRuntimeAdapter {
    type Error;

    fn register_service(&mut self, binding: &'static RpcServiceBinding) -> Result<(), Self::Error>;
}

pub trait RpcMethodRuntimeAdapter {
    type Error;

    fn register_method(&mut self, binding: &'static RpcMethodBinding) -> Result<(), Self::Error>;
}

pub fn bind_all_rpc_services<A>(adapter: &mut A) -> Result<(), A::Error>
where
    A: RpcRuntimeAdapter,
{
    for binding in crate::service_manifest::RPC_SERVICE_BINDINGS {
        adapter.register_service(binding)?;
    }
    Ok(())
}

pub fn bind_all_rpc_methods<A>(adapter: &mut A) -> Result<(), A::Error>
where
    A: RpcMethodRuntimeAdapter,
{
    for binding in crate::method_manifest::RPC_METHOD_BINDINGS {
        adapter.register_method(binding)?;
    }
    Ok(())
}
