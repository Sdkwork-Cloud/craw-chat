//! In-process gRPC smoke tests for conversation app and internal RPC hosts.

use std::net::SocketAddr;
use std::sync::Arc;

use conversation_runtime::http::default_app_state;
use conversation_runtime::internal_rpc_dispatch::{
    ConversationInternalRpcDispatcher, CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
};
use conversation_runtime::rpc_dispatch::{
    rpc_metadata_from_app_context, ConversationRpcDispatcher, CONVERSATION_RPC_SERVICE_KEYS,
};
use im_app_context::local_service_app_context;
use im_domain_core::room::game_move_schema_ref;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    CreateRoomRequest, EnterRoomRequest, room_service_client::RoomServiceClient,
};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::internal::v1::{
    DispatchConversationMessageRequest, OrchestrateCreateRoomRequest, OrchestrateEnterRoomRequest,
    message_dispatch_service_client::MessageDispatchServiceClient,
    room_orchestration_service_client::RoomOrchestrationServiceClient,
};
use sdkwork_im_rpc_service_rust::{
    build_im_rpc_service_router_with_config_for_services, ImRpcRuntimeDispatcher,
    ImRpcServerConfig, RpcMetadata,
};
use tonic::Code;
use tonic::metadata::MetadataValue;
use tonic::Request;

struct RpcServerHandle {
    shutdown: tokio::sync::oneshot::Sender<()>,
    join: tokio::task::JoinHandle<()>,
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

fn apply_rpc_metadata<T>(request: &mut Request<T>, metadata: &RpcMetadata) {
    let header_map = metadata.to_header_map();
    for key_and_value in header_map.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_and_value {
            request.metadata_mut().insert(key, value.clone());
        }
    }
}

fn internal_service_metadata(idempotency_key: &str) -> RpcMetadata {
    RpcMetadata {
        service_identity: Some("sdkwork-game-runtime".into()),
        idempotency_key: Some(idempotency_key.into()),
        request_id: Some("rpc-smoke-internal".into()),
        ..RpcMetadata::default()
    }
}

#[tokio::test]
async fn test_app_room_service_create_enter_over_grpc() {
    let state = default_app_state();
    let dispatcher = Arc::new(ConversationRpcDispatcher::from_app_state(state));
    let (addr, server) =
        start_in_process_rpc_server(dispatcher, CONVERSATION_RPC_SERVICE_KEYS).await;

    let owner = local_service_app_context("100001", "1", "user", Some("d_owner"), ["*"]);
    let mut client = RoomServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room service client should connect");

    let mut create_request = Request::new(CreateRoomRequest {
        conversation_id: "c_rpc_smoke_app".into(),
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
        enter_response
            .into_inner()
            .member
            .is_some(),
        "enter room should return membership"
    );

    server.shutdown().await;
}

#[tokio::test]
async fn test_internal_room_orchestration_and_message_dispatch_over_grpc() {
    let state = default_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let (addr, server) = start_in_process_rpc_server(
        dispatcher,
        CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
    )
    .await;

    let mut room_client = RoomOrchestrationServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room orchestration client should connect");
    let mut message_client = MessageDispatchServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("message dispatch client should connect");

    let mut create_request = Request::new(OrchestrateCreateRoomRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        conversation_id: "c_rpc_smoke_internal".into(),
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
    assert_eq!(
        create_response.into_inner().conversation_id,
        "c_rpc_smoke_internal"
    );

    let mut enter_request = Request::new(OrchestrateEnterRoomRequest {
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
    assert_eq!(
        enter_response.into_inner().conversation_id,
        "c_rpc_smoke_internal"
    );

    let schema_ref = game_move_schema_ref("landlord.play");
    let mut dispatch_request = Request::new(DispatchConversationMessageRequest {
        tenant_id: "100001".into(),
        organization_id: "org_a".into(),
        conversation_id: "c_rpc_smoke_internal".into(),
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
    assert_eq!(message.conversation_id, "c_rpc_smoke_internal");
    assert_eq!(message.sender_user_id, "1040");

    server.shutdown().await;
}

#[tokio::test]
async fn test_app_rpc_host_rejects_service_mtls_metadata_without_dual_token() {
    let state = default_app_state();
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
    let state = default_app_state();
    let dispatcher = Arc::new(ConversationInternalRpcDispatcher::from_app_state(state));
    let (addr, server) = start_in_process_rpc_server(
        dispatcher,
        CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
    )
    .await;

    let owner = local_service_app_context("100001", "1", "user", Some("d_owner"), ["*"]);
    let mut client = RoomOrchestrationServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("room orchestration client should connect");
    let mut request = Request::new(OrchestrateCreateRoomRequest {
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
