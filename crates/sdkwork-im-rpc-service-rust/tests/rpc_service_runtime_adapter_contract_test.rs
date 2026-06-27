use std::sync::{Arc, Mutex};

use prost::Message;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    CreatePresenceHeartbeatRequest, CreatePresenceHeartbeatResponse,
    presence_service_client::PresenceServiceClient, presence_service_server::PresenceService,
};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::backend::v3::{
    RetrieveHealthRequest, RetrieveHealthResponse,
    communication_ops_service_client::CommunicationOpsServiceClient,
};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::internal::v1::{
    RetrieveRuntimeTopologyRequest, RetrieveRuntimeTopologyResponse,
    runtime_topology_service_client::RuntimeTopologyServiceClient,
};
use sdkwork_im_rpc_service_rust::{
    GENERATED_TONIC_SERVICE_ADAPTER_COUNT, ImRpcClientConfig, ImRpcError, ImRpcRuntimeDispatcher,
    ImRpcServerConfig, ImRpcStreamRequest, ImRpcStreamResponse, ImRpcUnaryRequest,
    ImRpcUnaryResponse, PresenceServiceAdapter, RPC_METHOD_BINDINGS, RPC_SERVICE_BINDINGS,
    RpcDeadline, RpcMetadata, build_im_rpc_service_router, build_im_rpc_service_router_with_config,
    map_rpc_error_to_status,
};
use tonic::Code;
use tonic::metadata::MetadataMap;

#[derive(Clone, PartialEq, Eq, prost::Message)]
struct EmptyMessage {}

#[test]
fn test_im_rpc_server_and_client_configs_have_distributed_defaults() {
    let server = ImRpcServerConfig::local_default();

    assert_eq!(server.bind_addr, "127.0.0.1:50051");
    assert!(server.enable_health);
    assert!(!server.enable_reflection);
    assert!(!server.require_tls);
    assert!(!server.require_mtls);
    assert_eq!(server.default_deadline, RpcDeadline::from_millis(30_000));
    assert_eq!(server.max_decoding_message_size, 4 * 1024 * 1024);

    let client = ImRpcClientConfig::new("http://127.0.0.1:50051");

    assert_eq!(client.endpoint, "http://127.0.0.1:50051");
    assert!(!client.require_tls);
    assert!(!client.require_mtls);
    assert_eq!(client.default_deadline, RpcDeadline::from_millis(30_000));
    assert_eq!(client.max_encoding_message_size, usize::MAX);
}

#[test]
fn test_rpc_metadata_extracts_standard_sdkwork_keys() {
    let mut metadata = MetadataMap::new();
    metadata.insert("authorization", "Bearer app-token".parse().unwrap());
    metadata.insert("access-token", "access-token-value".parse().unwrap());
    metadata.insert("x-request-id", "request-1".parse().unwrap());
    metadata.insert("traceparent", "00-trace-span-01".parse().unwrap());
    metadata.insert("idempotency-key", "idem-1".parse().unwrap());
    metadata.insert("x-request-hash", "hash-1".parse().unwrap());
    metadata.insert(
        "x-sdkwork-client-version",
        "sdkwork-im-rpc-sdk-rust/1.0.3".parse().unwrap(),
    );

    let parsed = RpcMetadata::from_metadata_map(&metadata).expect("metadata should parse");

    assert_eq!(parsed.authorization.as_deref(), Some("Bearer app-token"));
    assert_eq!(parsed.access_token.as_deref(), Some("access-token-value"));
    assert_eq!(parsed.request_id.as_deref(), Some("request-1"));
    assert_eq!(parsed.traceparent.as_deref(), Some("00-trace-span-01"));
    assert_eq!(parsed.idempotency_key.as_deref(), Some("idem-1"));
    assert_eq!(parsed.request_hash.as_deref(), Some("hash-1"));
    assert_eq!(
        parsed.client_version.as_deref(),
        Some("sdkwork-im-rpc-sdk-rust/1.0.3")
    );
}

#[test]
fn test_rpc_error_mapper_uses_standard_grpc_status_codes() {
    assert_eq!(
        map_rpc_error_to_status(ImRpcError::unauthenticated("missing token")).code(),
        Code::Unauthenticated
    );
    assert_eq!(
        map_rpc_error_to_status(ImRpcError::permission_denied("forbidden")).code(),
        Code::PermissionDenied
    );
    assert_eq!(
        map_rpc_error_to_status(ImRpcError::invalid_argument("bad request")).code(),
        Code::InvalidArgument
    );
    assert_eq!(
        map_rpc_error_to_status(ImRpcError::unavailable("node draining")).code(),
        Code::Unavailable
    );
    assert_eq!(
        map_rpc_error_to_status(ImRpcError::internal("runtime failed")).code(),
        Code::Internal
    );
}

#[test]
fn test_generated_tonic_service_adapter_count_matches_manifest() {
    assert_eq!(
        GENERATED_TONIC_SERVICE_ADAPTER_COUNT,
        RPC_SERVICE_BINDINGS.len()
    );
}

#[tokio::test]
async fn test_presence_service_adapter_delegates_unary_call_to_runtime_dispatcher() {
    #[derive(Clone, Default)]
    struct FakeDispatcher {
        requests: Arc<Mutex<Vec<CapturedRequest>>>,
    }

    #[derive(Clone, Debug)]
    struct CapturedRequest {
        method_key: &'static str,
        operation_id: &'static str,
        authorization: Option<String>,
        request_bytes: Vec<u8>,
    }

    impl ImRpcRuntimeDispatcher for FakeDispatcher {
        fn dispatch_unary(
            &self,
            request: ImRpcUnaryRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>
        {
            self.requests.lock().unwrap().push(CapturedRequest {
                method_key: request.binding.method_key,
                operation_id: request.binding.operation_id,
                authorization: request.metadata.authorization.clone(),
                request_bytes: request.request_bytes.clone(),
            });

            Box::pin(async {
                let response = CreatePresenceHeartbeatResponse::default();
                ImRpcUnaryResponse::from_message(response)
            })
        }
    }

    let dispatcher = FakeDispatcher::default();
    let service = PresenceServiceAdapter::new(Arc::new(dispatcher.clone()));
    let mut request = tonic::Request::new(CreatePresenceHeartbeatRequest {
        device_id: "device-1".to_string(),
        status: "online".to_string(),
        metadata: None,
    });
    request
        .metadata_mut()
        .insert("authorization", "Bearer app-token".parse().unwrap());

    let response = PresenceService::create_presence_heartbeat(&service, request)
        .await
        .expect("presence heartbeat should dispatch");

    assert!(response.into_inner().presence.is_none());

    let captured = dispatcher.requests.lock().unwrap().clone();
    assert_eq!(captured.len(), 1);
    assert_eq!(
        captured[0].method_key,
        "sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat"
    );
    assert_eq!(captured[0].operation_id, "presence.heartbeat.create");
    assert_eq!(
        captured[0].authorization.as_deref(),
        Some("Bearer app-token")
    );

    let decoded_request =
        CreatePresenceHeartbeatRequest::decode(captured[0].request_bytes.as_slice())
            .expect("dispatcher should receive encoded proto request");
    assert_eq!(decoded_request.device_id, "device-1");
    assert_eq!(decoded_request.status, "online");
}

#[tokio::test]
async fn test_generated_grpc_server_and_client_smoke_for_presence_unary_call() {
    #[derive(Clone, Default)]
    struct FakeDispatcher {
        requests: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ImRpcRuntimeDispatcher for FakeDispatcher {
        fn dispatch_unary(
            &self,
            request: ImRpcUnaryRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>
        {
            self.requests
                .lock()
                .unwrap()
                .push(request.binding.method_key);

            Box::pin(async move {
                if request.binding.method_key
                    == "sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat"
                {
                    return ImRpcUnaryResponse::from_message(
                        CreatePresenceHeartbeatResponse::default(),
                    );
                }

                Err(ImRpcError::unimplemented(format!(
                    "test dispatcher does not implement {}",
                    request.binding.method_key
                )))
            })
        }
    }

    let dispatcher = Arc::new(FakeDispatcher::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("test TCP listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let router = build_im_rpc_service_router(dispatcher.clone());

    let server_task = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("in-process IM RPC server should run");
    });

    let mut client = PresenceServiceClient::connect(format!("http://{addr}"))
        .await
        .expect("generated RPC client should connect to in-process server");
    let response = client
        .create_presence_heartbeat(CreatePresenceHeartbeatRequest {
            device_id: "transport-device-1".to_owned(),
            status: "online".to_owned(),
            metadata: None,
        })
        .await
        .expect("generated client should call generated server adapter");

    assert!(response.into_inner().presence.is_none());

    shutdown_tx
        .send(())
        .expect("test shutdown signal should be delivered");
    server_task.await.expect("server task should join");

    assert_eq!(
        dispatcher.requests.lock().unwrap().as_slice(),
        [RPC_METHOD_BINDINGS[0].method_key]
    );
}

#[tokio::test]
async fn test_generated_grpc_server_routes_app_backend_and_internal_services() {
    #[derive(Clone, Default)]
    struct FakeDispatcher {
        requests: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ImRpcRuntimeDispatcher for FakeDispatcher {
        fn dispatch_unary(
            &self,
            request: ImRpcUnaryRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>
        {
            self.requests
                .lock()
                .unwrap()
                .push(request.binding.method_key);

            Box::pin(async move {
                match request.binding.method_key {
                    "sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat" => {
                        ImRpcUnaryResponse::from_message(CreatePresenceHeartbeatResponse::default())
                    }
                    "sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveHealth" => {
                        ImRpcUnaryResponse::from_message(RetrieveHealthResponse::default())
                    }
                    "sdkwork.communication.internal.v1.RuntimeTopologyService/RetrieveRuntimeTopology" => {
                        ImRpcUnaryResponse::from_message(RetrieveRuntimeTopologyResponse::default())
                    }
                    method_key => Err(ImRpcError::unimplemented(format!(
                        "test dispatcher does not implement {method_key}"
                    ))),
                }
            })
        }
    }

    let dispatcher = Arc::new(FakeDispatcher::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("test TCP listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let router = build_im_rpc_service_router(dispatcher.clone());

    let server_task = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("in-process IM RPC server should run");
    });

    let endpoint = format!("http://{addr}");
    let mut app_client = PresenceServiceClient::connect(endpoint.clone())
        .await
        .expect("generated app RPC client should connect");
    app_client
        .create_presence_heartbeat(CreatePresenceHeartbeatRequest {
            device_id: "transport-device-1".to_owned(),
            status: "online".to_owned(),
            metadata: None,
        })
        .await
        .expect("app RPC service should route through generated adapter");

    let mut backend_client = CommunicationOpsServiceClient::connect(endpoint.clone())
        .await
        .expect("generated backend RPC client should connect");
    backend_client
        .retrieve_health(RetrieveHealthRequest { metadata: None })
        .await
        .expect("backend RPC service should route through generated adapter");

    let mut internal_client = RuntimeTopologyServiceClient::connect(endpoint)
        .await
        .expect("generated internal RPC client should connect");
    internal_client
        .retrieve_runtime_topology(RetrieveRuntimeTopologyRequest { metadata: None })
        .await
        .expect("internal RPC service should route through generated adapter");

    shutdown_tx
        .send(())
        .expect("test shutdown signal should be delivered");
    server_task.await.expect("server task should join");

    assert_eq!(
        dispatcher.requests.lock().unwrap().as_slice(),
        [
            "sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat",
            "sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveHealth",
            "sdkwork.communication.internal.v1.RuntimeTopologyService/RetrieveRuntimeTopology",
        ]
    );
}

#[tokio::test]
async fn test_configured_grpc_server_registers_health_service_for_all_im_rpc_services() {
    #[derive(Clone, Default)]
    struct FakeDispatcher;

    impl ImRpcRuntimeDispatcher for FakeDispatcher {
        fn dispatch_unary(
            &self,
            request: ImRpcUnaryRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>
        {
            let _ = request;
            Box::pin(async { Err(ImRpcError::unimplemented("not used by health check")) })
        }
    }

    let config = ImRpcServerConfig {
        enable_health: true,
        ..ImRpcServerConfig::local_default()
    };
    let dispatcher = Arc::new(FakeDispatcher);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("test TCP listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let router = build_im_rpc_service_router_with_config(&config, dispatcher);

    let server_task = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("in-process IM RPC server should run");
    });

    let health_channel = tonic::transport::Endpoint::from_shared(format!("http://{addr}"))
        .expect("health endpoint should be parseable")
        .connect()
        .await
        .expect("generated health client should connect");
    let mut health_client = tonic_health::pb::health_client::HealthClient::new(health_channel);
    let response = health_client
        .check(tonic_health::pb::HealthCheckRequest {
            service: "sdkwork.communication.app.v3.PresenceService".to_owned(),
        })
        .await
        .expect("configured IM RPC server should expose grpc health checks");

    assert_eq!(
        response.into_inner().status,
        tonic_health::ServingStatus::Serving as i32
    );

    shutdown_tx
        .send(())
        .expect("test shutdown signal should be delivered");
    server_task.await.expect("server task should join");
}

#[tokio::test]
async fn test_generated_grpc_server_routes_every_manifest_method_path() {
    #[derive(Clone, Default)]
    struct FakeDispatcher {
        unary_requests: Arc<Mutex<Vec<&'static str>>>,
        stream_requests: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ImRpcRuntimeDispatcher for FakeDispatcher {
        fn dispatch_unary(
            &self,
            request: ImRpcUnaryRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>
        {
            self.unary_requests
                .lock()
                .unwrap()
                .push(request.binding.method_key);

            Box::pin(async { ImRpcUnaryResponse::from_message(EmptyMessage::default()) })
        }

        fn dispatch_server_stream(
            &self,
            request: ImRpcStreamRequest,
        ) -> sdkwork_im_rpc_service_rust::ImRpcBoxFuture<
            Result<
                sdkwork_im_rpc_service_rust::ImRpcBoxStream<
                    Result<ImRpcStreamResponse, ImRpcError>,
                >,
                ImRpcError,
            >,
        > {
            self.stream_requests
                .lock()
                .unwrap()
                .push(request.binding.method_key);

            Box::pin(async {
                let response = ImRpcStreamResponse::from_message(EmptyMessage::default())?;
                let stream = tokio_stream::iter([Ok(response)]);
                Ok(Box::pin(stream)
                    as sdkwork_im_rpc_service_rust::ImRpcBoxStream<
                        Result<ImRpcStreamResponse, ImRpcError>,
                    >)
            })
        }
    }

    let dispatcher = Arc::new(FakeDispatcher::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test TCP listener should bind");
    let addr = listener
        .local_addr()
        .expect("test TCP listener should expose local address");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let router = build_im_rpc_service_router(dispatcher.clone());

    let server_task = tokio::spawn(async move {
        router
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("in-process IM RPC server should run");
    });

    let channel = tonic::transport::Endpoint::from_shared(format!("http://{addr}"))
        .expect("test RPC endpoint should parse")
        .connect()
        .await
        .expect("test RPC client should connect");

    for binding in RPC_METHOD_BINDINGS {
        let path: tonic::codegen::http::uri::PathAndQuery = format!("/{}", binding.method_key)
            .parse()
            .expect("manifest method key should form a gRPC path");
        let mut client = tonic::client::Grpc::new(channel.clone());
        let codec = tonic_prost::ProstCodec::<EmptyMessage, EmptyMessage>::default();
        client
            .ready()
            .await
            .unwrap_or_else(|error| panic!("RPC client should be ready: {error}"));

        match binding.streaming {
            "unary" => {
                client
                    .unary(tonic::Request::new(EmptyMessage::default()), path, codec)
                    .await
                    .unwrap_or_else(|error| {
                        panic!(
                            "unary RPC method {} should route: {error}",
                            binding.method_key
                        )
                    });
            }
            "server" => {
                let mut stream = client
                    .server_streaming(tonic::Request::new(EmptyMessage::default()), path, codec)
                    .await
                    .unwrap_or_else(|error| {
                        panic!(
                            "server-streaming RPC method {} should route: {error}",
                            binding.method_key
                        )
                    })
                    .into_inner();
                stream.message().await.unwrap_or_else(|error| {
                    panic!(
                        "server-streaming RPC method {} should decode first item: {error}",
                        binding.method_key
                    )
                });
            }
            other => panic!(
                "unsupported streaming mode in IM RPC manifest for {}: {other}",
                binding.method_key
            ),
        }
    }

    shutdown_tx
        .send(())
        .expect("test shutdown signal should be delivered");
    server_task.await.expect("server task should join");

    let unary_requests = dispatcher.unary_requests.lock().unwrap().clone();
    let stream_requests = dispatcher.stream_requests.lock().unwrap().clone();
    assert_eq!(
        unary_requests,
        RPC_METHOD_BINDINGS
            .iter()
            .filter(|binding| binding.streaming == "unary")
            .map(|binding| binding.method_key)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        stream_requests,
        RPC_METHOD_BINDINGS
            .iter()
            .filter(|binding| binding.streaming == "server")
            .map(|binding| binding.method_key)
            .collect::<Vec<_>>()
    );
}
