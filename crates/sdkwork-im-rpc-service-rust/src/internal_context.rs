//! Authoritative tenant scope resolution for internal RPC (RPC_SPEC §13.1).

use axum::http::HeaderMap;
use im_app_context::{
    AppContext, AppContextError, resolve_orchestration_app_context_from_projection_headers,
};
use sdkwork_rpc_framework_core::{
    RpcCallerActorKind, VerifiedRpcCallerContext, VerifiedRpcServiceIdentity,
};
use sdkwork_rpc_server::{
    require_verified_rpc_caller_context, require_verified_rpc_service_identity,
};
use tonic::{Request, Status};

use crate::{ImRpcError, RpcMetadata, RpcMethodBinding, resolve_service_identity};

/// The one internal operation that exchanges a user-scoped capability and
/// therefore requires a caller context that is cryptographically bound to the
/// mTLS peer. Other legacy internal operations remain on their existing
/// migration path and must not accidentally opt into this strict profile.
pub const GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_CONSUME_OPERATION_ID: &str =
    "internal.groupKnowledgebaseLaunchTickets.consume";

/// Framework-verified request data retained before `tonic::Request::into_inner`
/// consumes extensions. The fields are private so a runtime dispatcher cannot
/// manufacture a trusted caller from request metadata or protobuf payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedInternalRpcContext {
    service_identity: VerifiedRpcServiceIdentity,
    caller_context: VerifiedRpcCallerContext,
}

impl VerifiedInternalRpcContext {
    pub fn from_tonic_request<T>(request: &Request<T>) -> Result<Self, Status> {
        Ok(Self {
            service_identity: require_verified_rpc_service_identity(request)?.clone(),
            caller_context: require_verified_rpc_caller_context(request)?.clone(),
        })
    }

    pub fn service_identity(&self) -> &VerifiedRpcServiceIdentity {
        &self.service_identity
    }

    pub fn caller_context(&self) -> &VerifiedRpcCallerContext {
        &self.caller_context
    }
}

/// User principal context derived only from [`VerifiedInternalRpcContext`].
/// It deliberately does not preserve arbitrary app-context projection fields:
/// ticket consumption needs only the delegated identity that was signed by the
/// verified Knowledgebase mTLS peer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedDelegatedUserContext {
    pub service_identity: String,
    pub app_context: AppContext,
    pub request_id: String,
    pub trace_id: String,
    pub idempotency_key: String,
}

pub fn requires_verified_delegated_user_context(binding: &RpcMethodBinding) -> bool {
    binding.operation_id == GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_CONSUME_OPERATION_ID
}

pub fn resolve_verified_delegated_user_context(
    verified: &VerifiedInternalRpcContext,
) -> Result<VerifiedDelegatedUserContext, ImRpcError> {
    let caller = verified.caller_context();
    if !matches!(caller.actor_kind, RpcCallerActorKind::User) {
        return Err(ImRpcError::permission_denied(
            "group knowledgebase launch ticket consumption requires a delegated user caller",
        ));
    }
    let session_id = caller
        .session_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .filter(|value| value.len() <= 256)
        .filter(|value| value.bytes().all(|byte| byte.is_ascii_graphic()))
        .ok_or_else(|| {
            ImRpcError::unauthenticated(
                "group knowledgebase launch ticket consumption requires a verified user session",
            )
        })?
        .to_owned();
    let trace_id = caller
        .trace_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ImRpcError::invalid_argument(
                "group knowledgebase launch ticket consumption requires a signed trace id",
            )
        })?
        .to_owned();
    let idempotency_key = caller
        .idempotency_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ImRpcError::invalid_argument(
                "group knowledgebase launch ticket consumption requires a signed idempotency key",
            )
        })?
        .to_owned();
    let actor_id = caller.actor_id.clone();
    Ok(VerifiedDelegatedUserContext {
        service_identity: verified.service_identity().service_id.clone(),
        app_context: AppContext {
            tenant_id: caller.tenant_id.clone(),
            organization_id: caller.organization_id.clone(),
            user_id: actor_id.clone(),
            session_id: Some(session_id),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            actor_id,
            actor_kind: "user".into(),
            device_id: None,
        },
        request_id: caller.request_id.clone(),
        trace_id,
        idempotency_key,
    })
}

/// Server-resolved tenant scope for internal orchestration RPC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternalOrchestrationContext {
    pub tenant_id: String,
    pub organization_id: String,
    pub service_identity: String,
    pub app_context: AppContext,
}

pub fn resolve_internal_orchestration_context(
    metadata: &RpcMetadata,
) -> Result<InternalOrchestrationContext, ImRpcError> {
    let service_identity = resolve_service_identity(metadata)?.ok_or_else(|| {
        ImRpcError::unauthenticated(
            "internal RPC requires x-sdkwork-service metadata or Service authorization",
        )
    })?;
    let headers = orchestration_headers_from_rpc_metadata(metadata);
    let app_context = resolve_orchestration_app_context_from_projection_headers(&headers)
        .map_err(map_app_context_error)?;
    Ok(InternalOrchestrationContext {
        tenant_id: app_context.tenant_id.clone(),
        organization_id: app_context.organization_id.clone(),
        service_identity,
        app_context,
    })
}

pub fn assert_body_scope_matches_authoritative_context(
    authoritative: &InternalOrchestrationContext,
    body_tenant_id: &str,
    body_organization_id: &str,
) -> Result<(), ImRpcError> {
    let body_tenant_id = body_tenant_id.trim();
    let body_organization_id = if body_organization_id.trim().is_empty() {
        "0"
    } else {
        body_organization_id.trim()
    };
    if body_tenant_id != authoritative.tenant_id.as_str() {
        return Err(ImRpcError::permission_denied(format!(
            "request body tenant_id `{body_tenant_id}` does not match authoritative tenant `{}`",
            authoritative.tenant_id
        )));
    }
    if body_organization_id != authoritative.organization_id.as_str() {
        return Err(ImRpcError::permission_denied(format!(
            "request body organization_id `{body_organization_id}` does not match authoritative organization `{}`",
            authoritative.organization_id
        )));
    }
    Ok(())
}

pub fn orchestration_headers_from_rpc_metadata(metadata: &RpcMetadata) -> HeaderMap {
    metadata.to_orchestration_header_map()
}

fn map_app_context_error(error: AppContextError) -> ImRpcError {
    match error.code() {
        "app_context_auth_token_missing" | "app_context_access_token_missing" => {
            ImRpcError::unauthenticated(error.message())
        }
        "app_context_invalid" | "app_context_jwt_invalid" => {
            ImRpcError::invalid_argument(error.message())
        }
        _ => ImRpcError::permission_denied(error.message()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_app_context::build_signed_orchestration_projection_headers;

    #[test]
    fn body_tenant_must_match_authoritative_context() {
        let authoritative = InternalOrchestrationContext {
            tenant_id: "100001".into(),
            organization_id: "org_a".into(),
            service_identity: "sdkwork-game-runtime".into(),
            app_context: im_app_context::local_service_app_context(
                "100001",
                "1",
                "service",
                None,
                ["*"],
            ),
        };
        assert!(
            assert_body_scope_matches_authoritative_context(&authoritative, "100001", "org_a",)
                .is_ok()
        );
        assert!(
            assert_body_scope_matches_authoritative_context(&authoritative, "100002", "org_a",)
                .is_err()
        );
    }

    #[test]
    fn orchestration_context_resolves_from_projection_headers() {
        let headers =
            build_signed_orchestration_projection_headers("100001", "org_a", "1040", "user")
                .expect("orchestration headers should build in test env");
        let metadata = RpcMetadata::from_orchestration_http_headers(
            &headers,
            Some("sdkwork-game-runtime".into()),
            Some("idem-1".into()),
            Some("trace-1".into()),
        );
        let resolved = resolve_internal_orchestration_context(&metadata)
            .expect("orchestration context should resolve");
        assert_eq!(resolved.tenant_id, "100001");
        assert_eq!(resolved.organization_id, "org_a");
        assert_eq!(resolved.service_identity, "sdkwork-game-runtime");
    }
}
