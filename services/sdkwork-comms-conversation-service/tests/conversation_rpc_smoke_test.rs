//! In-process gRPC smoke tests for conversation app and internal RPC hosts.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use conversation_runtime::http::{AppState, bootstrap_conversation_app_state_from_env};
use conversation_runtime::internal_rpc_dispatch::{
    CONVERSATION_INTERNAL_RPC_SERVICE_KEYS, ConversationInternalRpcDispatcher,
};
use conversation_runtime::rpc_dispatch::{
    CONVERSATION_RPC_SERVICE_KEYS, ConversationRpcDispatcher, rpc_metadata_from_app_context,
};
use im_app_context::{build_signed_orchestration_context_headers, local_service_app_context};
use im_domain_core::room::game_move_schema_ref;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    CreateRoomRequest, EnterRoomRequest, RetrieveCurrentConversationMemberRequest,
    conversation_service_client::ConversationServiceClient, room_service_client::RoomServiceClient,
};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::internal::v1::{
    ConsumeGroupKnowledgebaseLaunchTicketRequest, CreateRoomRequest as InternalCreateRoomRequest,
    DispatchConversationMessageRequest, EnterRoomRequest as InternalEnterRoomRequest,
    group_knowledgebase_launch_ticket_service_client::GroupKnowledgebaseLaunchTicketServiceClient,
    message_dispatch_service_client::MessageDispatchServiceClient,
    room_orchestration_service_client::RoomOrchestrationServiceClient,
};
use sdkwork_im_rpc_service_rust::{
    ImRpcRuntimeDispatcher, ImRpcServerConfig, RpcMetadata,
    build_im_rpc_mtls_service_router_with_config_for_services,
    build_im_rpc_service_router_with_config_for_services,
};
use sdkwork_rpc_framework_core::{
    RpcCallerContextSigningKey, RpcCallerContextVerifier, RpcServiceIdentityPolicy,
};
use sdkwork_rpc_server::{RpcInternalServiceSecurity, RpcServerTlsConfig};
use tonic::Code;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Identity};

const KNOWLEDGEBASE_CALLER_CONTEXT_TEST_KEY: [u8; 32] = [41; 32];

fn ensure_test_rustls_provider() {
    static PROVIDER: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    PROVIDER.get_or_init(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

struct MtlsTestCertificates {
    _temp_dir: tempfile::TempDir,
    server_tls: RpcServerTlsConfig,
    ca_pem: String,
    knowledgebase_client_cert_pem: String,
    knowledgebase_client_key_pem: String,
}

struct RpcServerHandle {
    shutdown: tokio::sync::oneshot::Sender<()>,
    join: tokio::task::JoinHandle<()>,
}

static RPC_SMOKE_TEST_ENV: std::sync::OnceLock<()> = std::sync::OnceLock::new();

fn ensure_rpc_smoke_test_environment() {
    RPC_SMOKE_TEST_ENV.get_or_init(|| {
        // SAFETY: This integration-test binary needs a deterministic dev/test
        // environment before building conversation AppState. The value is set
        // once for the whole test process and is not mutated afterwards.
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
            std::env::set_var("SDKWORK_IM_ALLOW_ALL_PRINCIPALS", "true");
        }
    });
}

fn rpc_smoke_app_state() -> AppState {
    ensure_rpc_smoke_test_environment();
    bootstrap_conversation_app_state_from_env()
        .expect("conversation RPC smoke tests require a test AppState")
}

impl RpcServerHandle {
    async fn shutdown(self) {
        let _ = self.shutdown.send(());
        let _ = self.join.await;
    }
}

async fn start_in_process_rpc_server<D>(
    dispatcher: Arc<D>,
    service_keys: &[&str],
) -> (SocketAddr, RpcServerHandle)
where
    D: ImRpcRuntimeDispatcher + 'static,
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("test TCP listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let config = ImRpcServerConfig {
        bind_addr: addr.to_string(),
        enable_health: true,
        ..ImRpcServerConfig::local_default()
    };
    let router =
        build_im_rpc_service_router_with_config_for_services(&config, dispatcher, service_keys);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let join = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("in-process IM RPC server should run");
    });
    (
        addr,
        RpcServerHandle {
            shutdown: shutdown_tx,
            join,
        },
    )
}

fn issue_mtls_test_certificates() -> MtlsTestCertificates {
    use rcgen::{
        BasicConstraints, CertificateParams, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
        KeyUsagePurpose, SanType,
    };

    ensure_test_rustls_provider();
    let mut ca_params = CertificateParams::new(Vec::<String>::new()).expect("CA parameters");
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyCertSign,
    ];
    let ca_key = KeyPair::generate().expect("CA key");
    let ca_certificate = ca_params.self_signed(&ca_key).expect("CA certificate");
    let ca_pem = ca_certificate.pem();
    let issuer = Issuer::new(ca_params, ca_key);

    let server_key = KeyPair::generate().expect("server key");
    let mut server_params =
        CertificateParams::new(vec!["localhost".to_owned()]).expect("server parameters");
    server_params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ServerAuth);
    let server_certificate = server_params
        .signed_by(&server_key, &issuer)
        .expect("server certificate");

    let knowledgebase_key = KeyPair::generate().expect("Knowledgebase client key");
    let mut knowledgebase_params =
        CertificateParams::new(Vec::<String>::new()).expect("Knowledgebase client parameters");
    knowledgebase_params.subject_alt_names = vec![SanType::URI(
        "spiffe://sdkwork.internal/sdkwork/service/sdkwork-knowledgebase"
            .try_into()
            .expect("Knowledgebase SPIFFE URI"),
    )];
    knowledgebase_params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    let knowledgebase_certificate = knowledgebase_params
        .signed_by(&knowledgebase_key, &issuer)
        .expect("Knowledgebase client certificate");

    let temp_dir = tempfile::TempDir::new().expect("test TLS directory");
    let server_cert_path = temp_dir.path().join("server.crt");
    let server_key_path = temp_dir.path().join("server.key");
    let ca_path = temp_dir.path().join("ca.crt");
    std::fs::write(&server_cert_path, server_certificate.pem()).expect("server certificate file");
    std::fs::write(&server_key_path, server_key.serialize_pem()).expect("server key file");
    std::fs::write(&ca_path, &ca_pem).expect("CA certificate file");

    MtlsTestCertificates {
        _temp_dir: temp_dir,
        server_tls: RpcServerTlsConfig {
            server_cert_path,
            server_key_path,
            client_ca_certificate_path: Some(ca_path),
            client_auth_optional: false,
        },
        ca_pem,
        knowledgebase_client_cert_pem: knowledgebase_certificate.pem(),
        knowledgebase_client_key_pem: knowledgebase_key.serialize_pem(),
    }
}

fn knowledgebase_mtls_security() -> RpcInternalServiceSecurity {
    let signing_key =
        RpcCallerContextSigningKey::from_secret_bytes(KNOWLEDGEBASE_CALLER_CONTEXT_TEST_KEY)
            .expect("caller-context signing key");
    RpcInternalServiceSecurity::new(
        RpcServiceIdentityPolicy::new("sdkwork.internal", ["sdkwork-knowledgebase"])
            .expect("Knowledgebase service identity policy"),
        Some(
            RpcCallerContextVerifier::new("sdkwork-im", [("sdkwork-knowledgebase", signing_key)])
                .expect("Knowledgebase caller-context verifier"),
        ),
    )
}

async fn start_mtls_conversation_internal_rpc_server(
    dispatcher: Arc<ConversationInternalRpcDispatcher>,
    certificates: &MtlsTestCertificates,
) -> (SocketAddr, RpcServerHandle) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("mTLS test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("mTLS test listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let config = ImRpcServerConfig {
        bind_addr: addr.to_string(),
        enable_health: false,
        require_tls: true,
        require_mtls: true,
        ..ImRpcServerConfig::local_default()
    };
    let router = build_im_rpc_mtls_service_router_with_config_for_services(
        &config,
        dispatcher,
        CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
        &certificates.server_tls,
        &knowledgebase_mtls_security(),
    )
    .expect("mTLS conversation internal RPC router should build");
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let join = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("mTLS conversation internal RPC server should run");
    });
    (
        addr,
        RpcServerHandle {
            shutdown: shutdown_tx,
            join,
        },
    )
}

async fn knowledgebase_mtls_ticket_client(
    addr: SocketAddr,
    certificates: &MtlsTestCertificates,
) -> GroupKnowledgebaseLaunchTicketServiceClient<tonic::transport::Channel> {
    let endpoint = Endpoint::from_shared(format!("https://{addr}"))
        .expect("mTLS endpoint")
        .tls_config(
            ClientTlsConfig::new()
                .domain_name("localhost")
                .ca_certificate(Certificate::from_pem(certificates.ca_pem.as_bytes()))
                .identity(Identity::from_pem(
                    certificates.knowledgebase_client_cert_pem.as_bytes(),
                    certificates.knowledgebase_client_key_pem.as_bytes(),
                )),
        )
        .expect("mTLS client configuration");
    let channel = tokio::time::timeout(Duration::from_secs(5), endpoint.connect())
        .await
        .expect("mTLS client connect timeout")
        .expect("mTLS client connection");
    GroupKnowledgebaseLaunchTicketServiceClient::new(channel)
}

fn apply_rpc_metadata<T>(request: &mut Request<T>, metadata: &RpcMetadata) {
    let header_map = metadata.to_header_map();
    for key_and_value in header_map.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_and_value {
            request.metadata_mut().insert(key, value.clone());
        }
    }
}

fn apply_header_map_to_rpc_metadata<T>(request: &mut Request<T>, headers: &axum::http::HeaderMap) {
    for (name, value) in headers {
        let Ok(value) = value.to_str() else {
            continue;
        };
        let Ok(key) = name
            .as_str()
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
        else {
            continue;
        };
        let Ok(value) = MetadataValue::try_from(value) else {
            continue;
        };
        request.metadata_mut().insert(key, value);
    }
}

fn internal_service_metadata(idempotency_key: &str) -> RpcMetadata {
    RpcMetadata {
        service_identity: Some("sdkwork-game-runtime".into()),
        idempotency_key: Some(idempotency_key.into()),
        trace_id: Some("trace-rpc-smoke-internal".into()),
        ..RpcMetadata::default()
    }
}

#[tokio::test]
async fn test_app_room_service_create_enter_over_grpc() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_RPC_SERVICE_KEYS).await;

    let owner = local_service_app_context("100001", "1", "user", Some("d_owner"), ["*"]);
    let mut client = RoomServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room service client should connect");

    let mut create_request = Request::new(CreateRoomRequest {
        conversation_id: String::new(),
        room_id: "room_rpc_smoke_app".into(),
        room_kind: "game".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut create_request,
        &rpc_metadata_from_app_context(
            &owner,
            Some("idem-app-room-create".into()),
            Some("req-app-room-create".into()),
        ),
    );
    let create_response = client
        .create_room(create_request)
        .await
        .expect("rooms.create should succeed over app RPC");
    let create_body = create_response.into_inner();
    assert_eq!(
        create_body.room.as_ref().map(|room| room.room_id.as_str()),
        Some("room_rpc_smoke_app")
    );
    assert!(
        create_body
            .room
            .as_ref()
            .is_some_and(|room| room.conversation_id.starts_with("r_"))
    );

    let player = local_service_app_context("100001", "1040", "user", Some("d_player"), ["*"]);
    let mut enter_request = Request::new(EnterRoomRequest {
        room_id: "room_rpc_smoke_app".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut enter_request,
        &rpc_metadata_from_app_context(
            &player,
            Some("idem-app-room-enter".into()),
            Some("req-app-room-enter".into()),
        ),
    );
    let enter_response = client
        .enter_room(enter_request)
        .await
        .expect("rooms.enter should succeed over app RPC");
    assert!(
        enter_response.into_inner().member.is_some(),
        "enter room should return membership"
    );

    server.shutdown().await;
}

#[tokio::test]
async fn test_app_conversation_service_retrieves_current_member_over_grpc() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_RPC_SERVICE_KEYS).await;

    let owner = local_service_app_context("100001", "1", "user", Some("d_owner"), ["*"]);
    let mut room_client = RoomServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room service client should connect");
    let mut client = ConversationServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("conversation service client should connect");

    let mut create_request = Request::new(CreateRoomRequest {
        conversation_id: String::new(),
        room_id: "room_rpc_current_member".into(),
        room_kind: "game".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut create_request,
        &rpc_metadata_from_app_context(
            &owner,
            Some("idem-current-member-conversation-create".into()),
            Some("trace-current-member-conversation-create".into()),
        ),
    );
    let conversation_id = room_client
        .create_room(create_request)
        .await
        .expect("rooms.create should establish the owner membership")
        .into_inner()
        .room
        .expect("create room should return a room")
        .conversation_id;

    let mut retrieve_request = Request::new(RetrieveCurrentConversationMemberRequest {
        conversation_id: conversation_id.clone(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut retrieve_request,
        &rpc_metadata_from_app_context(
            &owner,
            None,
            Some("trace-current-conversation-member-retrieve".into()),
        ),
    );
    let member = client
        .retrieve_current_conversation_member(retrieve_request)
        .await
        .expect("current conversation member retrieval should succeed")
        .into_inner()
        .member
        .expect("current conversation member retrieval should return a member");
    assert_eq!(member.conversation_id, conversation_id);
    assert_eq!(member.user_id, "1");
    assert_eq!(member.principal_kind, "user");
    assert!(!member.member_id.is_empty());
    assert_eq!(member.tenant_id, "100001");
    assert!(!member.joined_at.is_empty());
    assert_eq!(member.role, "owner");
    assert_eq!(member.state, "joined");

    server.shutdown().await;
}

#[tokio::test]
async fn test_internal_room_orchestration_and_message_dispatch_over_grpc() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_INTERNAL_RPC_SERVICE_KEYS).await;

    let mut room_client = RoomOrchestrationServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room orchestration client should connect");
    let mut message_client = MessageDispatchServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("message dispatch client should connect");

    let mut create_request = Request::new(InternalCreateRoomRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        conversation_id: String::new(),
        room_id: "room_rpc_smoke_internal".into(),
        room_kind: "game".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut create_request,
        &internal_service_metadata("idem-internal-room-create"),
    );
    let create_response = room_client
        .create_room(create_request)
        .await
        .expect("internal.rooms.create should succeed");
    let conversation_id = create_response.into_inner().conversation_id;
    assert!(conversation_id.starts_with("r_"));

    let mut enter_request = Request::new(InternalEnterRoomRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        room_id: "room_rpc_smoke_internal".into(),
        principal_id: "1040".into(),
        principal_kind: "user".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut enter_request,
        &internal_service_metadata("idem-internal-room-enter"),
    );
    let enter_response = room_client
        .enter_room(enter_request)
        .await
        .expect("internal.rooms.enter should succeed");
    assert_eq!(enter_response.into_inner().conversation_id, conversation_id);

    let schema_ref = game_move_schema_ref("landlord.play");
    let mut dispatch_request = Request::new(DispatchConversationMessageRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        conversation_id: conversation_id.clone(),
        sender_id: "1040".into(),
        sender_kind: "user".into(),
        schema_ref: schema_ref.clone(),
        payload_json: r#"{"seat":1,"cards":["7S"]}"#.into(),
        encoding: "application/json".into(),
        client_msg_id: "move-rpc-smoke-1".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut dispatch_request,
        &internal_service_metadata("idem-internal-message-dispatch"),
    );
    let dispatch_response = message_client
        .dispatch_conversation_message(dispatch_request)
        .await
        .expect("internal.messages.dispatch should succeed");
    let message = dispatch_response
        .into_inner()
        .message
        .expect("dispatch should return stored message view");
    assert!(!message.message_id.is_empty());
    assert_eq!(message.conversation_id, conversation_id);
    assert_eq!(message.sender_user_id, "1040");

    server.shutdown().await;
}

#[tokio::test]
async fn test_app_rpc_host_rejects_service_mtls_metadata_without_dual_token() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_RPC_SERVICE_KEYS).await;

    let mut client = RoomServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room service client should connect");
    let mut request = Request::new(CreateRoomRequest {
        conversation_id: "c_rpc_smoke_reject".into(),
        room_id: "room_rpc_smoke_reject".into(),
        room_kind: "game".into(),
        metadata: None,
    });
    request.metadata_mut().insert(
        "x-sdkwork-service",
        MetadataValue::from_static("sdkwork-game-runtime"),
    );
    request
        .metadata_mut()
        .insert("idempotency-key", MetadataValue::from_static("idem-reject"));

    let error = client
        .create_room(request)
        .await
        .expect_err("app RPC host should reject missing dual-token app session");
    assert_eq!(error.code(), Code::Unauthenticated);

    server.shutdown().await;
}

#[tokio::test]
async fn test_internal_rpc_host_rejects_app_session_without_service_identity() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_INTERNAL_RPC_SERVICE_KEYS).await;

    let owner = local_service_app_context("100001", "1", "user", Some("d_owner"), ["*"]);
    let mut client = RoomOrchestrationServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room orchestration client should connect");
    let mut request = Request::new(InternalCreateRoomRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        conversation_id: "c_rpc_smoke_internal_reject".into(),
        room_id: "room_rpc_smoke_internal_reject".into(),
        room_kind: "game".into(),
        metadata: None,
    });
    apply_rpc_metadata(
        &mut request,
        &rpc_metadata_from_app_context(
            &owner,
            Some("idem-internal-reject".into()),
            Some("req-internal-reject".into()),
        ),
    );

    let error = client
        .create_room(request)
        .await
        .expect_err("internal RPC host should reject app-session metadata");
    assert_eq!(error.code(), Code::Unauthenticated);

    server.shutdown().await;
}

#[tokio::test]
async fn group_knowledgebase_ticket_rpc_rejects_spoofed_headers_without_verified_extensions() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_INTERNAL_RPC_SERVICE_KEYS).await;
    let mut client = GroupKnowledgebaseLaunchTicketServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("ticket RPC client should connect");
    let mut request = Request::new(ConsumeGroupKnowledgebaseLaunchTicketRequest {
        ticket: "gklt_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        metadata: None,
    });
    request.metadata_mut().insert(
        "x-sdkwork-service",
        MetadataValue::from_static("sdkwork-knowledgebase"),
    );
    request.metadata_mut().insert(
        "idempotency-key",
        MetadataValue::from_static("idem-spoofed-ticket-consume"),
    );
    request.metadata_mut().insert(
        "x-sdkwork-trace-id",
        MetadataValue::from_static("trace-spoofed-ticket-consume"),
    );
    let projected_headers = build_signed_orchestration_context_headers("100001", "0", "1", "user")
        .expect("test context headers should build");
    apply_header_map_to_rpc_metadata(&mut request, &projected_headers);

    let error = client
        .consume_group_knowledgebase_launch_ticket(request)
        .await
        .expect_err(
            "x-sdkwork-service and signed app-context headers must not substitute for verified mTLS extensions",
        );
    assert_eq!(error.code(), Code::Unauthenticated);

    server.shutdown().await;
}

#[tokio::test]
async fn group_knowledgebase_ticket_rpc_requires_signed_context_after_valid_mtls() {
    let state = rpc_smoke_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let certificates = issue_mtls_test_certificates();
    let (addr, server) =
        start_mtls_conversation_internal_rpc_server(dispatcher, &certificates).await;
    let mut client = knowledgebase_mtls_ticket_client(addr, &certificates).await;
    let mut request = Request::new(ConsumeGroupKnowledgebaseLaunchTicketRequest {
        ticket: "gklt_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
        metadata: None,
    });
    request.metadata_mut().insert(
        "x-sdkwork-service",
        MetadataValue::from_static("sdkwork-knowledgebase"),
    );
    request.metadata_mut().insert(
        "idempotency-key",
        MetadataValue::from_static("idem-mtls-missing-signed-context"),
    );
    let projected_headers = build_signed_orchestration_context_headers("100001", "0", "1", "user")
        .expect("test context headers should build");
    apply_header_map_to_rpc_metadata(&mut request, &projected_headers);

    let error = client
        .consume_group_knowledgebase_launch_ticket(request)
        .await
        .expect_err(
            "valid mTLS alone cannot consume a delegated user ticket without a signed caller context",
        );
    assert_eq!(error.code(), Code::Unauthenticated);

    server.shutdown().await;
}
