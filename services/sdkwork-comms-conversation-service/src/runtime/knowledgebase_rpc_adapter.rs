//! Generated Knowledgebase RPC adapter for the IM group lifecycle port.
//!
//! This module owns only transport conversion, service caller credentials, and
//! response validation. Group lifecycle authorization and persistence remain in
//! the conversation runtime; the generated Knowledgebase SDK owns the RPC wire.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use im_domain_core::conversation::MembershipRole;
use sdkwork_knowledgebase_rpc_sdk_rust::sdkwork::{
    common::v1::RequestMetadata,
    intelligence::internal::v1::{
        ArchiveGroupKnowledgeSpaceRequest as RpcArchiveGroupKnowledgeSpaceRequest,
        ArchiveGroupKnowledgeSpaceResponse as RpcArchiveGroupKnowledgeSpaceResponse,
        EnsureGroupKnowledgeSpaceRequest as RpcEnsureGroupKnowledgeSpaceRequest,
        EnsureGroupKnowledgeSpaceResponse as RpcEnsureGroupKnowledgeSpaceResponse,
        GroupKnowledgeSpaceLifecycle as RpcGroupKnowledgeSpaceLifecycle,
        GroupKnowledgeSpaceLifecycleState as RpcGroupKnowledgeSpaceLifecycleState,
        GroupKnowledgeSpaceMember as RpcGroupKnowledgeSpaceMember,
        GroupKnowledgeSpaceMemberRole as RpcGroupKnowledgeSpaceMemberRole,
        SynchronizeGroupKnowledgeSpaceMembersRequest as RpcSynchronizeGroupKnowledgeSpaceMembersRequest,
        SynchronizeGroupKnowledgeSpaceMembersResponse as RpcSynchronizeGroupKnowledgeSpaceMembersResponse,
        group_knowledge_space_lifecycle_service_client::GroupKnowledgeSpaceLifecycleServiceClient,
    },
};
use sdkwork_rpc_client::{
    GrpcChannelConfig, RpcServiceCredentialProvider, RpcTlsConfig,
    SignedRpcServiceCredentialProvider, connect_grpc_channel_with_config,
};
use sdkwork_rpc_framework_core::{
    RpcCallerActorKind, RpcCallerContext, RpcCallerContextSigner, RpcCallerContextSigningKey,
    RpcFrameworkError,
};
use sdkwork_utils_rust::sha256_hash;
use tokio::sync::OnceCell;
use tonic::{Code, Status, transport::Channel};
use url::Url;

use super::knowledgebase::GroupKnowledgebaseArchiveDeliveryState;
use super::{
    ArchiveGroupKnowledgebaseRequest, EnsureGroupKnowledgebaseRequest, EnsuredGroupKnowledgebase,
    GroupKnowledgebaseMembership, GroupKnowledgebasePort, GroupKnowledgebasePortError,
    GroupKnowledgebaseScope, SynchronizeGroupKnowledgebaseMembersRequest,
};

const IM_SERVICE_ID: &str = "sdkwork-im";
const KNOWLEDGEBASE_SERVICE_ID: &str = "sdkwork-knowledgebase";
const MIN_REQUEST_TIMEOUT_MS: u64 = 100;
const MAX_REQUEST_TIMEOUT_MS: u64 = 60_000;
const MAX_CREDENTIAL_TTL_SECONDS: u64 = 300;
const RPC_CLIENT_VERSION: &str = "sdkwork-im";
const ENSURE_OPERATION: &str = "ensure";
const SYNCHRONIZE_OPERATION: &str = "synchronize-members";
const ARCHIVE_OPERATION: &str = "archive";

/// Bootstrap-owned configuration for the IM -> Knowledgebase lifecycle client.
///
/// The configuration is constructed from process configuration at the runtime
/// edge. This adapter never reads the environment or logs the endpoint,
/// credentials, roster, or signed caller context.
#[derive(Clone, Debug)]
pub(crate) struct KnowledgebaseGroupLifecycleRpcPortConfig {
    endpoint: String,
    tls: RpcTlsConfig,
    caller_context_signing_key: RpcCallerContextSigningKey,
    credential_ttl: Duration,
    request_timeout: Duration,
}

impl KnowledgebaseGroupLifecycleRpcPortConfig {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        endpoint: impl Into<String>,
        server_ca_certificate_path: impl Into<std::path::PathBuf>,
        client_certificate_path: impl Into<std::path::PathBuf>,
        client_private_key_path: impl Into<std::path::PathBuf>,
        tls_domain: impl Into<String>,
        caller_context_signing_key: RpcCallerContextSigningKey,
        credential_ttl: Duration,
        request_timeout: Duration,
    ) -> Result<Self, KnowledgebaseGroupLifecycleRpcAdapterError> {
        let endpoint = validate_secure_endpoint(endpoint.into())?;
        let tls_domain = tls_domain.into().trim().to_owned();
        if tls_domain.is_empty() {
            return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
                "Knowledgebase RPC TLS domain is required".to_owned(),
            ));
        }

        let credential_ttl_seconds = credential_ttl.as_secs();
        if credential_ttl.is_zero()
            || credential_ttl.subsec_nanos() != 0
            || credential_ttl_seconds > MAX_CREDENTIAL_TTL_SECONDS
        {
            return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
                "Knowledgebase RPC caller-context credential TTL must be a whole number of seconds between 1 and 300"
                    .to_owned(),
            ));
        }

        let request_timeout_ms = u64::try_from(request_timeout.as_millis()).unwrap_or(u64::MAX);
        if !(MIN_REQUEST_TIMEOUT_MS..=MAX_REQUEST_TIMEOUT_MS).contains(&request_timeout_ms) {
            return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
                format!(
                    "Knowledgebase RPC request timeout must be between {MIN_REQUEST_TIMEOUT_MS} and {MAX_REQUEST_TIMEOUT_MS} milliseconds"
                ),
            ));
        }

        let tls = RpcTlsConfig::server_verified()
            .with_server_ca(server_ca_certificate_path)
            .with_client_identity(client_certificate_path, client_private_key_path)
            .with_domain(tls_domain);
        tls.validate()
            .map_err(|_| KnowledgebaseGroupLifecycleRpcAdapterError::RpcFramework)?;

        Ok(Self {
            endpoint,
            tls,
            caller_context_signing_key,
            credential_ttl,
            request_timeout,
        })
    }
}

/// Internal port backed by the generated Knowledgebase lifecycle RPC client.
/// A successful construction validates every local security input. The first
/// `ensure_delivery_ready` call creates the mutually authenticated channel
/// before the outbox relay can lease durable work.
#[derive(Clone)]
pub(crate) struct KnowledgebaseGroupLifecycleRpcPort {
    config: KnowledgebaseGroupLifecycleRpcPortConfig,
    credential_provider: SignedRpcServiceCredentialProvider,
    client: Arc<OnceCell<GroupKnowledgeSpaceLifecycleServiceClient<Channel>>>,
}

impl KnowledgebaseGroupLifecycleRpcPort {
    pub(crate) fn new(
        config: KnowledgebaseGroupLifecycleRpcPortConfig,
    ) -> Result<Self, KnowledgebaseGroupLifecycleRpcAdapterError> {
        let signer =
            RpcCallerContextSigner::new(IM_SERVICE_ID, config.caller_context_signing_key.clone())
                .map_err(|_| KnowledgebaseGroupLifecycleRpcAdapterError::RpcFramework)?;
        let credential_provider =
            SignedRpcServiceCredentialProvider::new(signer, config.credential_ttl)
                .map_err(|_| KnowledgebaseGroupLifecycleRpcAdapterError::RpcFramework)?;

        Ok(Self {
            config,
            credential_provider,
            client: Arc::new(OnceCell::new()),
        })
    }

    async fn client(
        &self,
    ) -> Result<GroupKnowledgeSpaceLifecycleServiceClient<Channel>, GroupKnowledgebasePortError>
    {
        let config = self.config.clone();
        let client: &GroupKnowledgeSpaceLifecycleServiceClient<Channel> = self
            .client
            .get_or_try_init(|| async move {
                let channel = connect_grpc_channel_with_config(
                    config.endpoint.as_str(),
                    &GrpcChannelConfig {
                        connect_timeout: config.request_timeout,
                        tls: Some(config.tls),
                        ..GrpcChannelConfig::default()
                    },
                )
                .await?;
                Ok::<GroupKnowledgeSpaceLifecycleServiceClient<Channel>, RpcFrameworkError>(
                    GroupKnowledgeSpaceLifecycleServiceClient::new(channel),
                )
            })
            .await
            .map_err(|_| GroupKnowledgebasePortError::Unavailable)?;
        Ok(client.clone())
    }

    fn signed_request<T>(
        &self,
        message: T,
        scope: &GroupKnowledgebaseScope,
        source_event_id: &str,
    ) -> Result<tonic::Request<T>, GroupKnowledgebasePortError> {
        let caller_context = rpc_caller_context(scope, source_event_id)?;
        let credential = self
            .credential_provider
            .issue(caller_context)
            .map_err(|_| GroupKnowledgebasePortError::Rejected)?;
        let mut request = tonic::Request::new(message);
        request.set_timeout(self.config.request_timeout);
        credential
            .apply_to(request.metadata_mut())
            .map_err(|_| GroupKnowledgebasePortError::Rejected)?;
        Ok(request)
    }
}

#[async_trait]
impl GroupKnowledgebasePort for KnowledgebaseGroupLifecycleRpcPort {
    async fn ensure_delivery_ready(&self) -> Result<(), GroupKnowledgebasePortError> {
        let _ = self.client().await?;
        Ok(())
    }

    async fn ensure_group_knowledgebase(
        &self,
        command: EnsureGroupKnowledgebaseRequest,
    ) -> Result<EnsuredGroupKnowledgebase, GroupKnowledgebasePortError> {
        let rpc_command = RpcEnsureGroupKnowledgeSpaceRequest {
            conversation_id: command.scope.conversation_id.clone(),
            group_name: command.group_name.clone(),
            source_event_id: command.source_event_id.clone(),
            provisioning_idempotency_key: command.idempotency_key.clone(),
            membership_epoch: canonical_nonnegative_i64_decimal(command.membership_epoch)?,
            members: rpc_members(&command.members)?,
            metadata: Some(request_metadata(
                command.source_event_id.as_str(),
                ENSURE_OPERATION,
            )),
        };
        let request = self.signed_request(
            rpc_command,
            &command.scope,
            command.source_event_id.as_str(),
        )?;
        let mut client = self.client().await?;
        let response = tokio::time::timeout(
            self.config.request_timeout,
            client.ensure_group_knowledge_space(request),
        )
        .await
        .map_err(|_| GroupKnowledgebasePortError::Unavailable)?
        .map_err(map_rpc_status)?
        .into_inner();

        ensured_from_response(&command, response)
    }

    async fn synchronize_group_members(
        &self,
        command: SynchronizeGroupKnowledgebaseMembersRequest,
    ) -> Result<(), GroupKnowledgebasePortError> {
        let rpc_command = RpcSynchronizeGroupKnowledgeSpaceMembersRequest {
            conversation_id: command.scope.conversation_id.clone(),
            group_name: super::knowledgebase::group_knowledgebase_initial_group_name(
                command.scope.conversation_id.as_str(),
            ),
            source_event_id: command.source_event_id.clone(),
            knowledgebase_binding_id: canonical_positive_i64_decimal(
                command.knowledgebase_binding_id,
            )?,
            knowledgebase_binding_uuid: command.knowledgebase_binding_uuid.clone(),
            knowledge_space_id: canonical_positive_i64_decimal(command.knowledge_space_id)?,
            knowledge_space_uuid: command.knowledge_space_uuid.clone(),
            membership_epoch: canonical_nonnegative_i64_decimal(command.membership_epoch)?,
            upstream_link_generation: canonical_nonnegative_i64_decimal(
                command.upstream_link_generation,
            )?,
            members: rpc_members(&command.members)?,
            metadata: Some(request_metadata(
                command.source_event_id.as_str(),
                SYNCHRONIZE_OPERATION,
            )),
        };
        let request = self.signed_request(
            rpc_command,
            &command.scope,
            command.source_event_id.as_str(),
        )?;
        let mut client = self.client().await?;
        let response = tokio::time::timeout(
            self.config.request_timeout,
            client.synchronize_group_knowledge_space_members(request),
        )
        .await
        .map_err(|_| GroupKnowledgebasePortError::Unavailable)?
        .map_err(map_rpc_status)?
        .into_inner();

        synchronize_response_is_acceptable(&command, response)
    }

    async fn archive_group_knowledgebase(
        &self,
        command: ArchiveGroupKnowledgebaseRequest,
    ) -> Result<GroupKnowledgebaseArchiveDeliveryState, GroupKnowledgebasePortError> {
        let rpc_command = RpcArchiveGroupKnowledgeSpaceRequest {
            conversation_id: command.scope.conversation_id.clone(),
            source_event_id: command.source_event_id.clone(),
            knowledgebase_binding_id: canonical_positive_i64_decimal(
                command.knowledgebase_binding_id,
            )?,
            knowledgebase_binding_uuid: command.knowledgebase_binding_uuid.clone(),
            knowledge_space_id: canonical_positive_i64_decimal(command.knowledge_space_id)?,
            knowledge_space_uuid: command.knowledge_space_uuid.clone(),
            membership_epoch: canonical_nonnegative_i64_decimal(command.membership_epoch)?,
            upstream_link_generation: canonical_nonnegative_i64_decimal(
                command.upstream_link_generation,
            )?,
            archived_by: command.archived_by.clone(),
            metadata: Some(request_metadata(
                command.source_event_id.as_str(),
                ARCHIVE_OPERATION,
            )),
        };
        let request = self.signed_request(
            rpc_command,
            &command.scope,
            command.source_event_id.as_str(),
        )?;
        let mut client = self.client().await?;
        let response = tokio::time::timeout(
            self.config.request_timeout,
            client.archive_group_knowledge_space(request),
        )
        .await
        .map_err(|_| GroupKnowledgebasePortError::Unavailable)?
        .map_err(map_rpc_status)?
        .into_inner();

        archive_state_from_response(&command, response)
    }
}

fn validate_secure_endpoint(
    endpoint: String,
) -> Result<String, KnowledgebaseGroupLifecycleRpcAdapterError> {
    let endpoint = endpoint.trim().to_owned();
    if endpoint.is_empty() {
        return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
            "Knowledgebase RPC endpoint is required".to_owned(),
        ));
    }
    let parsed = Url::parse(endpoint.as_str()).map_err(|_| {
        KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
            "Knowledgebase RPC endpoint must be an absolute URL".to_owned(),
        )
    })?;
    if !matches!(parsed.scheme(), "https" | "grpcs")
        || parsed.host_str().is_none()
        || !matches!(parsed.path(), "" | "/")
    {
        return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
            "Knowledgebase RPC endpoint must use https:// or grpcs:// with a host and no path"
                .to_owned(),
        ));
    }
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(
            "Knowledgebase RPC endpoint must not contain credentials, query parameters, or fragments"
                .to_owned(),
        ));
    }
    Ok(endpoint)
}

fn rpc_caller_context(
    scope: &GroupKnowledgebaseScope,
    source_event_id: &str,
) -> Result<RpcCallerContext, GroupKnowledgebasePortError> {
    if !is_canonical_positive_signed_i64(scope.tenant_id.as_str())
        || !is_canonical_positive_signed_i64(scope.organization_id.as_str())
        || scope.conversation_id.trim().is_empty()
        || source_event_id.trim().is_empty()
    {
        return Err(GroupKnowledgebasePortError::Rejected);
    }
    let correlation_id = rpc_correlation_id(source_event_id);
    RpcCallerContext::builder()
        .tenant_id(scope.tenant_id.clone())
        .organization_id(scope.organization_id.clone())
        .actor_id(IM_SERVICE_ID)
        .actor_kind(RpcCallerActorKind::Service)
        .request_id(correlation_id.clone())
        .trace_id(correlation_id.clone())
        .idempotency_key(correlation_id)
        .audience_service_id(KNOWLEDGEBASE_SERVICE_ID)
        .build()
        .map_err(|_| GroupKnowledgebasePortError::Rejected)
}

fn request_metadata(source_event_id: &str, operation: &str) -> RequestMetadata {
    let correlation_id = rpc_correlation_id(source_event_id);
    RequestMetadata {
        trace_id: correlation_id.clone(),
        traceparent: String::new(),
        idempotency_key: correlation_id,
        request_hash: sha256_hash(
            format!("sdkwork-im:group-knowledgebase:{operation}:{source_event_id}").as_bytes(),
        ),
        client_version: RPC_CLIENT_VERSION.to_owned(),
    }
}

fn rpc_correlation_id(source_event_id: &str) -> String {
    format!("gkb-{}", sha256_hash(source_event_id.as_bytes()))
}

fn rpc_members(
    members: &[GroupKnowledgebaseMembership],
) -> Result<Vec<RpcGroupKnowledgeSpaceMember>, GroupKnowledgebasePortError> {
    members
        .iter()
        .map(|member| {
            if member.principal_kind != "user" || member.principal_id.trim().is_empty() {
                return Err(GroupKnowledgebasePortError::Rejected);
            }
            let role = match member.role {
                MembershipRole::Owner => RpcGroupKnowledgeSpaceMemberRole::Owner,
                MembershipRole::Admin => RpcGroupKnowledgeSpaceMemberRole::Admin,
                MembershipRole::Member => RpcGroupKnowledgeSpaceMemberRole::Member,
                MembershipRole::Guest => RpcGroupKnowledgeSpaceMemberRole::Guest,
            };
            Ok(RpcGroupKnowledgeSpaceMember {
                actor_id: member.principal_id.clone(),
                role: role as i32,
            })
        })
        .collect()
}

fn ensured_from_response(
    command: &EnsureGroupKnowledgebaseRequest,
    response: RpcEnsureGroupKnowledgeSpaceResponse,
) -> Result<EnsuredGroupKnowledgebase, GroupKnowledgebasePortError> {
    let lifecycle = response
        .lifecycle
        .ok_or(GroupKnowledgebasePortError::Unavailable)?;
    if lifecycle_state(&lifecycle)? != RpcGroupKnowledgeSpaceLifecycleState::Active {
        return Err(GroupKnowledgebasePortError::Conflict);
    }
    let membership_epoch = parse_nonnegative_signed_i64(lifecycle.membership_epoch.as_str())?;
    if membership_epoch != command.membership_epoch {
        return Err(GroupKnowledgebasePortError::Conflict);
    }

    Ok(EnsuredGroupKnowledgebase {
        knowledge_space_id: parse_positive_signed_i64(lifecycle.knowledge_space_id.as_str())?,
        knowledge_space_uuid: required_response_identifier(lifecycle.knowledge_space_uuid)?,
        knowledgebase_binding_id: parse_positive_signed_i64(
            lifecycle.knowledgebase_binding_id.as_str(),
        )?,
        knowledgebase_binding_uuid: required_response_identifier(
            lifecycle.knowledgebase_binding_uuid,
        )?,
        provisioning_operation_id: None,
        membership_epoch,
    })
}

fn synchronize_response_is_acceptable(
    command: &SynchronizeGroupKnowledgebaseMembersRequest,
    response: RpcSynchronizeGroupKnowledgeSpaceMembersResponse,
) -> Result<(), GroupKnowledgebasePortError> {
    let lifecycle = response
        .lifecycle
        .ok_or(GroupKnowledgebasePortError::Unavailable)?;
    validate_response_target(
        &lifecycle,
        command.knowledge_space_id,
        command.knowledge_space_uuid.as_str(),
        command.knowledgebase_binding_id,
        command.knowledgebase_binding_uuid.as_str(),
    )?;
    validate_response_progress(
        &lifecycle,
        command.membership_epoch,
        command.upstream_link_generation,
    )?;

    match lifecycle_state(&lifecycle)? {
        RpcGroupKnowledgeSpaceLifecycleState::Active
        | RpcGroupKnowledgeSpaceLifecycleState::Provisioning
        | RpcGroupKnowledgeSpaceLifecycleState::Archived
        | RpcGroupKnowledgeSpaceLifecycleState::Deleted => Ok(()),
        RpcGroupKnowledgeSpaceLifecycleState::Archiving => {
            Err(GroupKnowledgebasePortError::Unavailable)
        }
        RpcGroupKnowledgeSpaceLifecycleState::Failed
        | RpcGroupKnowledgeSpaceLifecycleState::Unspecified => {
            Err(GroupKnowledgebasePortError::Conflict)
        }
    }
}

fn archive_state_from_response(
    command: &ArchiveGroupKnowledgebaseRequest,
    response: RpcArchiveGroupKnowledgeSpaceResponse,
) -> Result<GroupKnowledgebaseArchiveDeliveryState, GroupKnowledgebasePortError> {
    let lifecycle = response
        .lifecycle
        .ok_or(GroupKnowledgebasePortError::Unavailable)?;
    validate_response_target(
        &lifecycle,
        command.knowledge_space_id,
        command.knowledge_space_uuid.as_str(),
        command.knowledgebase_binding_id,
        command.knowledgebase_binding_uuid.as_str(),
    )?;
    validate_response_progress(
        &lifecycle,
        command.membership_epoch,
        command.upstream_link_generation,
    )?;

    match lifecycle_state(&lifecycle)? {
        RpcGroupKnowledgeSpaceLifecycleState::Archiving => {
            Ok(GroupKnowledgebaseArchiveDeliveryState::Archiving)
        }
        RpcGroupKnowledgeSpaceLifecycleState::Archived => {
            Ok(GroupKnowledgebaseArchiveDeliveryState::Archived)
        }
        RpcGroupKnowledgeSpaceLifecycleState::Deleted => {
            Ok(GroupKnowledgebaseArchiveDeliveryState::Deleted)
        }
        RpcGroupKnowledgeSpaceLifecycleState::Provisioning
        | RpcGroupKnowledgeSpaceLifecycleState::Active
        | RpcGroupKnowledgeSpaceLifecycleState::Failed
        | RpcGroupKnowledgeSpaceLifecycleState::Unspecified => {
            Err(GroupKnowledgebasePortError::Conflict)
        }
    }
}

fn validate_response_target(
    lifecycle: &RpcGroupKnowledgeSpaceLifecycle,
    expected_space_id: i64,
    expected_space_uuid: &str,
    expected_binding_id: i64,
    expected_binding_uuid: &str,
) -> Result<(), GroupKnowledgebasePortError> {
    if parse_positive_signed_i64(lifecycle.knowledge_space_id.as_str())? != expected_space_id
        || lifecycle.knowledge_space_uuid != expected_space_uuid
        || parse_positive_signed_i64(lifecycle.knowledgebase_binding_id.as_str())?
            != expected_binding_id
        || lifecycle.knowledgebase_binding_uuid != expected_binding_uuid
    {
        return Err(GroupKnowledgebasePortError::Conflict);
    }
    Ok(())
}

fn validate_response_progress(
    lifecycle: &RpcGroupKnowledgeSpaceLifecycle,
    expected_membership_epoch: u64,
    expected_upstream_link_generation: u64,
) -> Result<(), GroupKnowledgebasePortError> {
    let membership_epoch = parse_nonnegative_signed_i64(lifecycle.membership_epoch.as_str())?;
    let upstream_link_generation =
        parse_nonnegative_signed_i64(lifecycle.upstream_link_generation.as_str())?;
    if membership_epoch < expected_membership_epoch
        || upstream_link_generation < expected_upstream_link_generation
    {
        return Err(GroupKnowledgebasePortError::Conflict);
    }
    Ok(())
}

fn lifecycle_state(
    lifecycle: &RpcGroupKnowledgeSpaceLifecycle,
) -> Result<RpcGroupKnowledgeSpaceLifecycleState, GroupKnowledgebasePortError> {
    RpcGroupKnowledgeSpaceLifecycleState::try_from(lifecycle.lifecycle_state)
        .map_err(|_| GroupKnowledgebasePortError::Unavailable)
}

fn canonical_positive_i64_decimal(value: i64) -> Result<String, GroupKnowledgebasePortError> {
    if value <= 0 {
        return Err(GroupKnowledgebasePortError::Rejected);
    }
    Ok(value.to_string())
}

fn canonical_nonnegative_i64_decimal(value: u64) -> Result<String, GroupKnowledgebasePortError> {
    let value = i64::try_from(value).map_err(|_| GroupKnowledgebasePortError::Rejected)?;
    Ok(value.to_string())
}

fn parse_positive_signed_i64(value: &str) -> Result<i64, GroupKnowledgebasePortError> {
    let parsed = parse_nonnegative_signed_i64(value)?;
    i64::try_from(parsed)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(GroupKnowledgebasePortError::Unavailable)
}

fn parse_nonnegative_signed_i64(value: &str) -> Result<u64, GroupKnowledgebasePortError> {
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(GroupKnowledgebasePortError::Unavailable);
    }
    let parsed = value
        .parse::<i64>()
        .map_err(|_| GroupKnowledgebasePortError::Unavailable)?;
    u64::try_from(parsed).map_err(|_| GroupKnowledgebasePortError::Unavailable)
}

fn required_response_identifier(value: String) -> Result<String, GroupKnowledgebasePortError> {
    if value.trim().is_empty() {
        return Err(GroupKnowledgebasePortError::Unavailable);
    }
    Ok(value)
}

fn is_canonical_positive_signed_i64(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('0')
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.parse::<i64>().is_ok_and(|parsed| parsed > 0)
}

fn map_rpc_status(status: Status) -> GroupKnowledgebasePortError {
    match status.code() {
        Code::InvalidArgument
        | Code::OutOfRange
        | Code::Unauthenticated
        | Code::PermissionDenied => GroupKnowledgebasePortError::Rejected,
        Code::AlreadyExists | Code::Aborted | Code::FailedPrecondition | Code::NotFound => {
            GroupKnowledgebasePortError::Conflict
        }
        Code::Cancelled
        | Code::DeadlineExceeded
        | Code::ResourceExhausted
        | Code::Unavailable
        | Code::Unknown
        | Code::Internal
        | Code::DataLoss
        | Code::Unimplemented => GroupKnowledgebasePortError::Unavailable,
        Code::Ok => GroupKnowledgebasePortError::Unavailable,
    }
}

#[derive(Debug)]
pub(crate) enum KnowledgebaseGroupLifecycleRpcAdapterError {
    Configuration(String),
    RpcFramework,
}

impl std::fmt::Display for KnowledgebaseGroupLifecycleRpcAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Configuration(message) => write!(formatter, "{message}"),
            Self::RpcFramework => {
                formatter.write_str("Knowledgebase RPC framework configuration failed")
            }
        }
    }
}

impl std::error::Error for KnowledgebaseGroupLifecycleRpcAdapterError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope() -> GroupKnowledgebaseScope {
        GroupKnowledgebaseScope {
            tenant_id: "100001".to_owned(),
            organization_id: "200001".to_owned(),
            conversation_id: "g-knowledgebase-rpc".to_owned(),
        }
    }

    fn member(role: MembershipRole) -> GroupKnowledgebaseMembership {
        GroupKnowledgebaseMembership {
            principal_id: "42".to_owned(),
            principal_kind: "user".to_owned(),
            role,
        }
    }

    fn ensure_command() -> EnsureGroupKnowledgebaseRequest {
        EnsureGroupKnowledgebaseRequest {
            scope: scope(),
            group_name: "Group g-knowledgebase-rpc".to_owned(),
            idempotency_key: "provision-1".to_owned(),
            source_event_id: "ensure-event-1".to_owned(),
            membership_epoch: 3,
            members: vec![member(MembershipRole::Owner)],
        }
    }

    fn lifecycle(state: RpcGroupKnowledgeSpaceLifecycleState) -> RpcGroupKnowledgeSpaceLifecycle {
        RpcGroupKnowledgeSpaceLifecycle {
            knowledgebase_binding_id: "801".to_owned(),
            knowledgebase_binding_uuid: "binding-801".to_owned(),
            knowledge_space_id: "701".to_owned(),
            knowledge_space_uuid: "space-701".to_owned(),
            lifecycle_state: state as i32,
            membership_epoch: "3".to_owned(),
            upstream_link_generation: "5".to_owned(),
        }
    }

    #[test]
    fn service_caller_context_is_scoped_to_im_and_never_to_an_end_user() {
        let context = rpc_caller_context(&scope(), "source-event-1").expect("caller context");
        assert_eq!(context.actor_id, IM_SERVICE_ID);
        assert_eq!(context.actor_kind, RpcCallerActorKind::Service);
        assert_eq!(context.audience_service_id, KNOWLEDGEBASE_SERVICE_ID);
        assert_eq!(context.tenant_id, "100001");
        assert_eq!(context.organization_id, "200001");
        assert!(context.session_id.is_none());
        assert_eq!(context.request_id, rpc_correlation_id("source-event-1"));
        assert_eq!(
            context.idempotency_key.as_deref(),
            Some(context.request_id.as_str())
        );
    }

    #[test]
    fn group_members_are_mapped_only_for_user_principals() {
        let mapped = rpc_members(&[
            member(MembershipRole::Owner),
            member(MembershipRole::Admin),
            member(MembershipRole::Member),
            member(MembershipRole::Guest),
        ])
        .expect("user members");
        assert_eq!(
            mapped[0].role,
            RpcGroupKnowledgeSpaceMemberRole::Owner as i32
        );
        assert_eq!(
            mapped[1].role,
            RpcGroupKnowledgeSpaceMemberRole::Admin as i32
        );
        assert_eq!(
            mapped[2].role,
            RpcGroupKnowledgeSpaceMemberRole::Member as i32
        );
        assert_eq!(
            mapped[3].role,
            RpcGroupKnowledgeSpaceMemberRole::Guest as i32
        );

        let mut unsupported = member(MembershipRole::Member);
        unsupported.principal_kind = "bot".to_owned();
        assert_eq!(
            rpc_members(&[unsupported]),
            Err(GroupKnowledgebasePortError::Rejected)
        );
    }

    #[test]
    fn ensure_requires_an_active_matching_epoch_response() {
        let command = ensure_command();
        let response = RpcEnsureGroupKnowledgeSpaceResponse {
            lifecycle: Some(lifecycle(RpcGroupKnowledgeSpaceLifecycleState::Active)),
            metadata: None,
        };
        let ensured = ensured_from_response(&command, response).expect("active ensure response");
        assert_eq!(ensured.knowledge_space_id, 701);
        assert_eq!(ensured.knowledgebase_binding_id, 801);
        assert_eq!(ensured.membership_epoch, 3);

        let response = RpcEnsureGroupKnowledgeSpaceResponse {
            lifecycle: Some(lifecycle(
                RpcGroupKnowledgeSpaceLifecycleState::Provisioning,
            )),
            metadata: None,
        };
        assert_eq!(
            ensured_from_response(&command, response),
            Err(GroupKnowledgebasePortError::Conflict)
        );
    }

    #[test]
    fn sync_treats_terminal_archive_states_as_a_stale_success() {
        let command = SynchronizeGroupKnowledgebaseMembersRequest {
            scope: scope(),
            knowledge_space_id: 701,
            knowledge_space_uuid: "space-701".to_owned(),
            knowledgebase_binding_id: 801,
            knowledgebase_binding_uuid: "binding-801".to_owned(),
            upstream_link_generation: 5,
            membership_epoch: 3,
            source_event_id: "sync-event-1".to_owned(),
            members: vec![member(MembershipRole::Owner)],
        };
        let response = RpcSynchronizeGroupKnowledgeSpaceMembersResponse {
            lifecycle: Some(lifecycle(RpcGroupKnowledgeSpaceLifecycleState::Archived)),
            metadata: None,
        };
        assert!(synchronize_response_is_acceptable(&command, response).is_ok());

        let response = RpcSynchronizeGroupKnowledgeSpaceMembersResponse {
            lifecycle: Some(lifecycle(RpcGroupKnowledgeSpaceLifecycleState::Archiving)),
            metadata: None,
        };
        assert_eq!(
            synchronize_response_is_acceptable(&command, response),
            Err(GroupKnowledgebasePortError::Unavailable)
        );
    }

    #[test]
    fn responses_must_preserve_the_immutable_target_fence() {
        let command = ArchiveGroupKnowledgebaseRequest {
            scope: scope(),
            knowledge_space_id: 701,
            knowledge_space_uuid: "space-701".to_owned(),
            knowledgebase_binding_id: 801,
            knowledgebase_binding_uuid: "binding-801".to_owned(),
            membership_epoch: 3,
            upstream_link_generation: 5,
            source_event_id: "archive-event-1".to_owned(),
            archived_by: "42".to_owned(),
        };
        let mut mismatched = lifecycle(RpcGroupKnowledgeSpaceLifecycleState::Archived);
        mismatched.knowledge_space_uuid = "other-space".to_owned();
        let response = RpcArchiveGroupKnowledgeSpaceResponse {
            lifecycle: Some(mismatched),
            metadata: None,
        };
        assert_eq!(
            archive_state_from_response(&command, response),
            Err(GroupKnowledgebasePortError::Conflict)
        );
    }

    #[test]
    fn rejects_plaintext_or_ambiguous_endpoints() {
        for endpoint in [
            "http://knowledgebase.internal:7443",
            "grpc://knowledgebase.internal:7443",
            "https://knowledgebase.internal:7443/path",
            "knowledgebase.internal",
        ] {
            assert!(matches!(
                validate_secure_endpoint(endpoint.to_owned()),
                Err(KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(_))
            ));
        }
        assert_eq!(
            validate_secure_endpoint("grpcs://knowledgebase.internal:7443".to_owned())
                .expect("secure endpoint"),
            "grpcs://knowledgebase.internal:7443"
        );
    }

    #[test]
    fn rpc_status_mapping_never_exposes_remote_status_messages() {
        assert_eq!(
            map_rpc_status(Status::permission_denied("sensitive reason")),
            GroupKnowledgebasePortError::Rejected
        );
        assert_eq!(
            map_rpc_status(Status::unavailable("sensitive reason")),
            GroupKnowledgebasePortError::Unavailable
        );
        assert_eq!(
            map_rpc_status(Status::failed_precondition("sensitive reason")),
            GroupKnowledgebasePortError::Conflict
        );
    }
}
