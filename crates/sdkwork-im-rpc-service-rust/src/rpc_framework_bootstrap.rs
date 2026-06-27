use sdkwork_rpc_framework_core::{ResolverProfile, RpcFrameworkError};

use crate::rpc_client_bootstrap::{
    IM_DEFAULT_RESILIENCE_PROFILE, IM_RPC_RESOLVER_PROFILE_ENV, IM_RPC_STATIC_ENDPOINT_ENV,
    ImRpcNameResolverBootstrap, build_im_rpc_name_resolver_from_env,
    im_rpc_resolver_profile_from_env,
};

/// RPC framework bootstrap inventory for IM RPC hosts.
#[derive(Debug)]
pub struct ImRpcFrameworkBootstrap {
    pub resolver_profile: ResolverProfile,
    pub default_resilience_profile: &'static str,
    pub client_resolver: Option<ImRpcNameResolverBootstrap>,
}

impl ImRpcFrameworkBootstrap {
    pub async fn verify_client_resolution(&self) -> Result<(), RpcFrameworkError> {
        let Some(client) = &self.client_resolver else {
            return Ok(());
        };

        let _endpoint = client.resolve_primary_endpoint().await?;
        Ok(())
    }

    pub async fn connect_primary_rpc_channel(
        &self,
    ) -> Result<Option<tonic::transport::Channel>, RpcFrameworkError> {
        match &self.client_resolver {
            Some(client) => Ok(Some(client.connect_primary_channel().await?)),
            None => Ok(None),
        }
    }
}

/// `initialize-rpc-framework` stage for IM RPC hosts.
pub fn initialize_im_rpc_framework_from_env() -> Result<ImRpcFrameworkBootstrap, RpcFrameworkError>
{
    let resolver_profile = im_rpc_resolver_profile_from_env();
    let client_resolver = if should_initialize_client_resolver(resolver_profile) {
        Some(build_im_rpc_name_resolver_from_env()?)
    } else {
        None
    };

    Ok(ImRpcFrameworkBootstrap {
        resolver_profile,
        default_resilience_profile: IM_DEFAULT_RESILIENCE_PROFILE,
        client_resolver,
    })
}

fn should_initialize_client_resolver(profile: ResolverProfile) -> bool {
    if std::env::var(IM_RPC_RESOLVER_PROFILE_ENV).is_ok() {
        return true;
    }

    match profile {
        ResolverProfile::Discovery | ResolverProfile::Composite => true,
        ResolverProfile::Static | ResolverProfile::StaticComposite => static_endpoint_configured(),
    }
}

fn static_endpoint_configured() -> bool {
    std::env::var(IM_RPC_STATIC_ENDPOINT_ENV)
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_client_inventory_when_no_resolver_env_is_configured() {
        unsafe {
            std::env::remove_var(IM_RPC_RESOLVER_PROFILE_ENV);
            std::env::remove_var(IM_RPC_STATIC_ENDPOINT_ENV);
            std::env::remove_var("SDKWORK_IM_DISCOVERY_ENDPOINT");
        }

        let bootstrap = initialize_im_rpc_framework_from_env().expect("bootstrap");
        assert_eq!(bootstrap.resolver_profile, ResolverProfile::Static);
        assert_eq!(bootstrap.default_resilience_profile, "rpc-default");
        assert!(bootstrap.client_resolver.is_none());
    }
}
