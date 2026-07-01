use crate::{ImRpcError, RpcMetadata, RpcMethodBinding};

pub fn require_app_session_auth(
    binding: &RpcMethodBinding,
    metadata: &RpcMetadata,
) -> Result<(), ImRpcError> {
    match binding.auth {
        "app-session" => {
            if metadata.authorization.as_deref().is_none()
                && metadata.access_token.as_deref().is_none()
            {
                return Err(ImRpcError::unauthenticated(
                    "app-session RPC requires authorization or access-token metadata",
                ));
            }
            Ok(())
        }
        "backend-admin" | "service-mtls" => Err(ImRpcError::permission_denied(format!(
            "RPC method `{}` requires `{}` auth and cannot be served by this host",
            binding.method_key, binding.auth
        ))),
        other => Err(ImRpcError::internal(format!(
            "unsupported RPC auth profile `{other}` for method `{}`",
            binding.method_key
        ))),
    }
}

pub fn require_service_mtls_auth(
    binding: &RpcMethodBinding,
    metadata: &RpcMetadata,
) -> Result<(), ImRpcError> {
    match binding.auth {
        "service-mtls" => {
            let identity = resolve_service_identity(metadata)?;
            if identity.is_none() {
                return Err(ImRpcError::unauthenticated(
                    "service-mtls RPC requires x-sdkwork-service metadata or Service authorization",
                ));
            }
            Ok(())
        }
        "app-session" | "backend-admin" => Err(ImRpcError::permission_denied(format!(
            "RPC method `{}` requires `{}` auth and cannot be served by this internal host",
            binding.method_key, binding.auth
        ))),
        other => Err(ImRpcError::internal(format!(
            "unsupported RPC auth profile `{other}` for method `{}`",
            binding.method_key
        ))),
    }
}

pub fn resolve_service_identity(metadata: &RpcMetadata) -> Result<Option<String>, ImRpcError> {
    if let Some(identity) = metadata
        .service_identity
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Ok(Some(identity.to_owned()));
    }

    // Collapsed nested if-let into a single chain
    if let Some(identity) = metadata.authorization.as_deref()
        .and_then(|auth| auth.strip_prefix("Service "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Ok(Some(identity.to_owned()));
    }

    Ok(None)
}

pub fn require_idempotency_key(
    binding: &RpcMethodBinding,
    metadata: &RpcMetadata,
) -> Result<(), ImRpcError> {
    if binding.idempotency == "required"
        && metadata
            .idempotency_key
            .as_deref()
            .is_none_or(str::is_empty)
    {
        return Err(ImRpcError::invalid_argument(format!(
            "RPC method `{}` requires idempotency-key metadata",
            binding.method_key
        )));
    }
    Ok(())
}

pub fn admit_app_unary_request(
    binding: &RpcMethodBinding,
    metadata: &RpcMetadata,
) -> Result<(), ImRpcError> {
    require_app_session_auth(binding, metadata)?;
    require_idempotency_key(binding, metadata)?;
    Ok(())
}

pub fn admit_internal_unary_request(
    binding: &RpcMethodBinding,
    metadata: &RpcMetadata,
) -> Result<(), ImRpcError> {
    require_service_mtls_auth(binding, metadata)?;
    require_idempotency_key(binding, metadata)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(auth: &'static str) -> RpcMethodBinding {
        RpcMethodBinding {
            method_key: "sdkwork.communication.internal.v1.MessageDispatchService/DispatchConversationMessage",
            package: "sdkwork.communication.internal.v1",
            service: "MessageDispatchService",
            method: "DispatchConversationMessage",
            surface: "internal",
            operation_id: "internal.messages.dispatch",
            auth,
            idempotency: "required",
            streaming: "unary",
            owner: "communication-internal",
            compatibility: "v1",
        }
    }

    #[test]
    fn service_mtls_auth_accepts_x_sdkwork_service_metadata() {
        let metadata = RpcMetadata {
            service_identity: Some("sdkwork-game-runtime".into()),
            idempotency_key: Some("idem-1".into()),
            ..RpcMetadata::default()
        };
        admit_internal_unary_request(&binding("service-mtls"), &metadata)
            .expect("service identity metadata should admit internal unary RPC");
    }

    #[test]
    fn service_mtls_auth_accepts_service_authorization_scheme() {
        let metadata = RpcMetadata {
            authorization: Some("Service sdkwork-im-cloud-gateway".into()),
            idempotency_key: Some("idem-1".into()),
            ..RpcMetadata::default()
        };
        admit_internal_unary_request(&binding("service-mtls"), &metadata)
            .expect("Service authorization should admit internal unary RPC");
    }

    #[test]
    fn app_session_auth_rejects_internal_methods_on_app_host() {
        let metadata = RpcMetadata {
            authorization: Some("Bearer user-token".into()),
            ..RpcMetadata::default()
        };
        let error = require_app_session_auth(&binding("service-mtls"), &metadata)
            .expect_err("internal auth profile must be rejected on app host");
        assert!(error.message().contains("service-mtls"));
    }
}
