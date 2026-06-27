use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use futures_util::{SinkExt, StreamExt};
use sdkwork_im_ccp_binding_ws::{CCP_WS_SUBPROTOCOL, WsBinding, WsBindingMessage, WsOpcode};
use sdkwork_im_ccp_codec::CcpCodec;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{AuthBindFrame, ControlFrame, HelloFrame};
use sdkwork_im_ccp_core::{
    CapabilitySet, CcpEnvelope, CcpRoute, ProtocolVersion, TransportBinding,
};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::ClientRequestBuilder;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

mod test_env;

type RealtimeWsSocket = tokio_tungstenite::WebSocketStream<
    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
>;

async fn spawn_server(app: Router) -> (test_env::DevTestEnvironment, String, tokio::task::JoinHandle<()>) {
    let env = test_env::dev_test_environment();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (
        env,
        format!("127.0.0.1:{}", address.port()),
        handle,
    )
}

fn build_permissive_realtime_app() -> Router {
    session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests()),
    )
}

async fn next_message(
    socket: &mut RealtimeWsSocket,
) -> Message {
    timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should stay open")
        .expect("websocket frame should decode")
}

fn encode_ccp_text_frame(
    schema: &str,
    kind: &str,
    payload: serde_json::Value,
) -> tokio_tungstenite::tungstenite::Message {
    encode_ccp_text_frame_with_route(schema, kind, None, payload)
}

fn encode_ccp_text_frame_with_route(
    schema: &str,
    kind: &str,
    route: Option<CcpRoute>,
    payload: serde_json::Value,
) -> tokio_tungstenite::tungstenite::Message {
    let codec = JsonEnvelopeCodec::new();
    let binding = WsBinding::new();
    let envelope = CcpEnvelope::new(
        ProtocolVersion::new("ccp", 1, 0),
        TransportBinding::Ws1,
        kind,
        schema,
        None,
        route,
        Vec::<String>::new(),
        None,
        payload.to_string(),
    );
    let message = binding
        .encode(&envelope, &codec)
        .expect("ccp envelope should encode");
    match message.opcode {
        WsOpcode::Text => Message::Text(
            String::from_utf8(message.payload)
                .expect("ccp text payload should stay utf8")
                .into(),
        ),
        WsOpcode::Binary => Message::Binary(message.payload.into()),
    }
}

fn decode_ccp_envelope(message: Message) -> CcpEnvelope {
    let codec = JsonEnvelopeCodec::new();
    let binding = WsBinding::new();
    let binding_message = match message {
        Message::Text(text) => WsBindingMessage {
            protocol_id: TransportBinding::Ws1.protocol_id(),
            content_type: codec.content_type(),
            opcode: WsOpcode::Text,
            payload: text.to_string().into_bytes(),
        },
        Message::Binary(bytes) => WsBindingMessage {
            protocol_id: TransportBinding::Ws1.protocol_id(),
            content_type: codec.content_type(),
            opcode: WsOpcode::Binary,
            payload: bytes.to_vec(),
        },
        other => panic!("expected CCP websocket frame, got {other:?}"),
    };
    binding
        .decode(&binding_message, &codec)
        .expect("ccp websocket frame should decode")
}

fn envelope_payload_json(envelope: &CcpEnvelope) -> serde_json::Value {
    serde_json::from_str(envelope.payload.as_str()).expect("ccp payload should be valid json")
}

fn assert_policy_close_with_reason(message: Message, reason: &str) {
    match message {
        Message::Close(Some(frame)) => {
            assert_eq!(
                frame.code,
                tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Policy
            );
            assert_eq!(frame.reason.as_str(), reason);
        }
        other => panic!("expected policy close frame, got {other:?}"),
    }
}

fn assert_connection_closed_after_oversized_message(
    message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
) {
    match message {
        Some(Ok(Message::Close(_))) | Some(Err(_)) => {}
        Some(Ok(other)) => {
            panic!("expected websocket close after oversized message, got {other:?}")
        }
        None => panic!("expected websocket close after oversized message"),
    }
}

fn test_auth_token(
    tenant_id: &str,
    principal_id: &str,
    actor_kind: &str,
    session_id: &str,
) -> String {
    json!({
        "tenant_id": tenant_id,
        "login_scope": "TENANT",
        "user_id": principal_id,
        "session_id": session_id,
        "app_id": "sdkwork-im",
        "auth_level": "password",
        "subject_type": actor_kind
    })
    .to_string()
}

fn test_access_token(
    tenant_id: &str,
    principal_id: &str,
    actor_kind: &str,
    session_id: &str,
    device_id: &str,
) -> String {
    json!({
        "tenant_id": tenant_id,
        "login_scope": "TENANT",
        "user_id": principal_id,
        "session_id": session_id,
        "app_id": "sdkwork-im",
        "environment": "dev",
        "deployment_mode": "saas",
        "auth_level": "password",
        "actor_id": principal_id,
        "actor_kind": actor_kind,
        "device_id": device_id,
        "data_scope": ["tenant"],
        "permission_scope": ["*"],
        "subject_type": actor_kind
    })
    .to_string()
}

fn insert_test_dual_token_headers(
    headers: &mut tokio_tungstenite::tungstenite::http::HeaderMap,
    tenant_id: &str,
    principal_id: &str,
    actor_kind: &str,
    session_id: &str,
    device_id: &str,
) {
    headers.insert(
        tokio_tungstenite::tungstenite::http::header::AUTHORIZATION,
        format!(
            "Bearer {}",
            test_auth_token(tenant_id, principal_id, actor_kind, session_id)
        )
        .parse()
        .expect("auth token header should parse"),
    );
    headers.insert(
        "Access-Token",
        test_access_token(tenant_id, principal_id, actor_kind, session_id, device_id)
            .parse()
            .expect("access token header should parse"),
    );
}

fn authenticated_ccp_request(url: String) -> ClientRequestBuilder {
    ClientRequestBuilder::new(url.parse().expect("websocket url should parse"))
        .with_sub_protocol(CCP_WS_SUBPROTOCOL)
        .with_header(
            "authorization",
            format!(
                "Bearer {}",
                test_auth_token("100001", "1", "user", "s_pad")
            ),
        )
        .with_header(
            "Access-Token",
            test_access_token("100001", "1", "user", "s_pad", "d_pad"),
        )
}

fn authenticated_json_request(url: String) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url
        .into_client_request()
        .expect("websocket request should build");
    insert_test_dual_token_headers(
        request.headers_mut(),
        "100001",
        "1",
        "user",
        "s_pad",
        "d_pad",
    );
    request
}

async fn connect_ccp_realtime_socket(address: &str) -> RealtimeWsSocket {
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));
    connect_async(request)
        .await
        .expect("ccp websocket connection should succeed")
        .0
}

async fn complete_ccp_realtime_handshake(socket: &mut RealtimeWsSocket) -> serde_json::Value {
    complete_ccp_realtime_handshake_with_capabilities(socket, &["payload.json"]).await
}

async fn complete_ccp_realtime_handshake_with_capabilities(
    socket: &mut RealtimeWsSocket,
    capabilities: &[&str],
) -> serde_json::Value {
    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(capabilities.iter().copied()),
                trace_id: None,
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    envelope_payload_json(&connected)
}

async fn next_ccp_business_payload(socket: &mut RealtimeWsSocket) -> serde_json::Value {
    let envelope = decode_ccp_envelope(next_message(socket).await);
    envelope_payload_json(&envelope)
}

async fn send_ccp_business_frame(
    socket: &mut RealtimeWsSocket,
    schema: &str,
    kind: &str,
    payload: serde_json::Value,
) {
    socket
        .send(encode_ccp_text_frame(schema, kind, payload))
        .await
        .expect("ccp business frame should send");
}

async fn send_ccp_business_command(
    socket: &mut RealtimeWsSocket,
    schema: &str,
    payload: serde_json::Value,
) {
    send_ccp_business_frame(socket, schema, "cmd", payload).await;
}

async fn assert_ccp_overload_disconnect(socket: &mut RealtimeWsSocket) {
    let goaway = decode_ccp_envelope(next_message(socket).await);
    assert_eq!(goaway.schema, "cc.control.goaway.v1");
    let goaway_payload = envelope_payload_json(&goaway);
    assert_eq!(goaway_payload["type"], "go_away");
    assert_eq!(
        goaway_payload["data"]["code"].as_str(),
        Some("LINK_OVERLOAD")
    );

    let close = next_message(socket).await;
    match close {
        Message::Close(Some(frame)) => {
            assert_eq!(
                frame.code,
                tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(
                    session_gateway::REALTIME_OVERLOAD_CLOSE_CODE,
                )
            );
            assert_eq!(
                frame.reason.as_str(),
                session_gateway::REALTIME_OVERLOAD_CLOSE_REASON
            );
        }
        other => panic!("expected close frame after goaway, got {other:?}"),
    }
}

#[tokio::test]
async fn test_realtime_websocket_binds_http_control_semantics() {
    let app = build_permissive_realtime_app();
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], "d_pad");
    assert_eq!(connected["actor"]["id"], "1");
    assert_eq!(connected["actor"]["kind"], "user");
    assert_eq!(connected["sender"]["principalId"], "1");
    assert_eq!(connected["sender"]["deviceId"], "d_pad");
    assert_eq!(connected["sender"]["sessionId"], "s_pad");
    assert_eq!(connected["sender"]["senderId"], "1:d_pad");
    assert_eq!(connected["ackedThroughSeq"], 0);
    assert_eq!(connected["trimmedThroughSeq"], 0);
    assert_eq!(connected["latestRealtimeSeq"], 0);

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.subscriptions.sync.v1",
        json!({
            "type":"subscriptions.sync",
            "requestId":"req_sync_1",
            "items":[
                {
                    "scopeType":"conversation",
                    "scopeId":"c_demo",
                    "eventTypes":["message.posted"]
                }
            ]
        }),
    )
    .await;

    let synced = next_ccp_business_payload(&mut socket).await;
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["requestId"], "req_sync_1");
    assert_eq!(synced["snapshot"]["deviceId"], "d_pad");
    assert_eq!(synced["snapshot"]["items"][0]["scopeType"], "conversation");
    assert_eq!(synced["snapshot"]["items"][0]["scopeId"], "c_demo");

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"req_pull_1",
            "afterSeq":0,
            "limit":10
        }),
    )
    .await;

    let window = next_ccp_business_payload(&mut socket).await;
    assert_eq!(window["type"], "event.window");
    assert_eq!(window["requestId"], "req_pull_1");
    assert_eq!(window["reason"], "pull");
    assert_eq!(window["window"]["deviceId"], "d_pad");
    assert_eq!(window["window"]["items"].as_array().unwrap().len(), 0);
    assert_eq!(window["window"]["ackedThroughSeq"], 0);
    assert_eq!(window["window"]["trimmedThroughSeq"], 0);

    send_ccp_business_frame(
        &mut socket,
        "cc.realtime.events.ack.v1",
        "ack",
        json!({
            "type":"events.ack",
            "requestId":"req_ack_1",
            "ackedSeq":0
        }),
    )
    .await;

    let acked = next_ccp_business_payload(&mut socket).await;
    assert_eq!(acked["type"], "events.acked");
    assert_eq!(acked["requestId"], "req_ack_1");
    assert_eq!(acked["ack"]["deviceId"], "d_pad");
    assert_eq!(acked["ack"]["ackedThroughSeq"], 0);
    assert_eq!(acked["ack"]["trimmedThroughSeq"], 0);
    assert_eq!(acked["ack"]["retainedEventCount"], 0);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_oversized_request_id() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let _connected = complete_ccp_realtime_handshake(&mut socket).await;

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"r".repeat(1024),
            "afterSeq":0,
            "limit":10
        }),
    )
    .await;

    let error = next_ccp_business_payload(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert!(error["requestId"].is_null());
    assert_eq!(error["code"], "payload_too_large");
    assert!(
        error["message"]
            .as_str()
            .expect("message should be a string")
            .contains("requestId"),
        "error should point to requestId payload guard, got: {error:?}"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_oversized_frame_type() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let _connected = complete_ccp_realtime_handshake(&mut socket).await;

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"x".repeat(1024),
            "requestId":"req_oversized_type_1"
        }),
    )
    .await;

    let error = next_ccp_business_payload(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert_eq!(error["requestId"], "req_oversized_type_1");
    assert_eq!(error["code"], "payload_too_large");
    assert!(
        error["message"]
            .as_str()
            .expect("message should be a string")
            .contains("type"),
        "error should point to type payload guard, got: {error:?}"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_connection_for_oversized_raw_message() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let _connected = complete_ccp_realtime_handshake(&mut socket).await;

    socket
        .send(Message::Text("x".repeat(700_000).into()))
        .await
        .expect("oversized websocket message should send");

    let next = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("oversized websocket message should trigger a close");
    assert_connection_closed_after_oversized_message(next);

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames() {
    let app = build_permissive_realtime_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-1".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.kind, "control");
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");
    let hello_ack_payload = envelope_payload_json(&hello_ack);
    assert_eq!(hello_ack_payload["type"], "hello_ack");
    assert_eq!(hello_ack_payload["data"]["accepted"], true);

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.kind, "control");
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");
    let auth_ok_payload = envelope_payload_json(&auth_ok);
    assert_eq!(auth_ok_payload["type"], "auth_ok");
    assert_eq!(auth_ok_payload["data"]["tenant_id"], "100001");
    assert_eq!(auth_ok_payload["data"]["principal_id"], "1");
    assert_eq!(auth_ok_payload["data"]["device_id"], "d_pad");
    assert_eq!(auth_ok_payload["data"]["session_id"], "s_pad");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.kind, "evt");
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");
    assert_eq!(connected_payload["deviceId"], "d_pad");
    assert_eq!(connected_payload["actor"]["id"], "1");
    assert_eq!(connected_payload["sender"]["senderId"], "1:d_pad");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.subscriptions.sync.v1",
            "cmd",
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_sync_ccp_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_demo",
                        "eventTypes":["message.posted"]
                    }
                ]
            }),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(synced.schema, "cc.realtime.subscriptions.synced.v1");
    let synced_payload = envelope_payload_json(&synced);
    assert_eq!(synced_payload["type"], "subscriptions.synced");
    assert_eq!(synced_payload["requestId"], "req_sync_ccp_1");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_ccp_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("event pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_ccp_1");
    assert_eq!(window_payload["reason"], "pull");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.ack.v1",
            "ack",
            json!({
                "type":"events.ack",
                "requestId":"req_ack_ccp_1",
                "ackedSeq":0
            }),
        ))
        .await
        .expect("ack frame should send");

    let acked = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(acked.schema, "cc.realtime.events.acked.v1");
    let acked_payload = envelope_payload_json(&acked);
    assert_eq!(acked_payload["type"], "events.acked");
    assert_eq!(acked_payload["requestId"], "req_ack_ccp_1");
    assert_eq!(acked_payload["ack"]["deviceId"], "d_pad");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_control_kind_after_handshake() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-invalid-kind".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_wrong_kind_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("wrong-kind business frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_wrong_kind_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("kind"),
        "error should explain CCP kind mismatch, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_invalid_kind_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_after_invalid_kind_1");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_wrong_schema_after_handshake() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-invalid-schema".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.ack.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_wrong_schema_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("wrong-schema business frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_wrong_schema_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("schema"),
        "error should explain CCP schema mismatch, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_invalid_schema_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(
        window_payload["requestId"],
        "req_pull_after_invalid_schema_1"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_accepts_ccp_heartbeat_control_frame_after_handshake() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-heartbeat".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.heartbeat.v1",
            "control",
            serde_json::to_value(ControlFrame::Heartbeat(
                sdkwork_im_ccp_control::HeartbeatFrame { sequence: Some(1) },
            ))
            .expect("heartbeat frame should serialize"),
        ))
        .await
        .expect("heartbeat frame should send");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_heartbeat_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("pull frame after heartbeat should send");

    let first_response = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(first_response.kind, "evt");
    assert_eq!(first_response.schema, "cc.realtime.event.window.v1");
    let first_response_payload = envelope_payload_json(&first_response);
    assert_eq!(first_response_payload["type"], "event.window");
    assert_eq!(
        first_response_payload["requestId"],
        "req_pull_after_heartbeat_1"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_client_route_metadata() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-client-route".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame_with_route(
            "cc.realtime.events.pull.v1",
            "cmd",
            Some(CcpRoute::new(
                "t_forged",
                Some("1137".into()),
                Some("d_forged".into()),
            )),
            json!({
                "type":"events.pull",
                "requestId":"req_pull_client_route_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("client-route pull frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_client_route_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("route"),
        "error should explain client route metadata is forbidden, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_client_route_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_after_client_route_1");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_handshake_starts_without_hello() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("out-of-order auth bind frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_HELLO_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_after_ccp_handshake_protocol_close() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_runtime_and_presence(
        cluster.clone(),
        Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests()),
        Arc::new(session_gateway::PresenceRuntime::default()),
    );
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("out-of-order auth bind frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["data"]["code"], "CCP_HELLO_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released after handshake protocol close");

    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .is_none(),
        "ccp handshake failure must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_after_client_close() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    socket
        .close(None)
        .await
        .expect("client close should send successfully");

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released after client websocket close");

    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .is_none(),
        "closed websocket must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_when_upgrade_client_disconnects_before_socket_handoff()
 {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());
    let (_env, address, handle) = spawn_server(app).await;

    let mut stream = tokio::net::TcpStream::connect(address.as_str())
        .await
        .expect("raw tcp connection should succeed");
    let auth_token = test_auth_token("100001", "1", "user", "s_pad");
    let access_token = test_access_token("100001", "1", "user", "s_pad", "d_pad");
    let upgrade_request = format!(
        "GET /im/v3/api/realtime/ws HTTP/1.1\r\n\
Host: {address}\r\n\
Connection: Upgrade\r\n\
Upgrade: websocket\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Protocol: {CCP_WS_SUBPROTOCOL}\r\n\
Authorization: Bearer {auth_token}\r\n\
Access-Token: {access_token}\r\n\
\r\n"
    );
    stream
        .write_all(upgrade_request.as_bytes())
        .await
        .expect("upgrade request should write");
    stream
        .shutdown()
        .await
        .expect("client shutdown should succeed");
    drop(stream);

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released when upgrade client disconnects before websocket handoff");

    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .is_none(),
        "aborted websocket upgrade must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_hello_uses_wrong_schema() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-wrong-schema".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("wrong-schema hello frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_SCHEMA_INCOMPATIBLE");
    assert!(
        error_payload["data"]["message"]
            .as_str()
            .expect("message should be a string")
            .contains("schema"),
        "error should explain CCP control schema mismatch, got: {error_payload:?}"
    );

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_handshake_receives_hello_twice() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let hello_message = encode_ccp_text_frame(
        "cc.control.hello.v1",
        "control",
        serde_json::to_value(ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["payload.json"]),
            trace_id: Some("trace-hello-order".into()),
        }))
        .expect("hello frame should serialize"),
    );

    socket
        .send(hello_message.clone())
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(hello_message)
        .await
        .expect("duplicate hello frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_AUTH_BIND_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_pushes_live_business_frames_over_ccp_subprotocol() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-live-push-ccp".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.subscriptions.sync.v1",
            "cmd",
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_live_push_ccp_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_demo",
                        "eventTypes":["message.posted"]
                    }
                ]
            }),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(synced.schema, "cc.realtime.subscriptions.synced.v1");
    let synced_payload = envelope_payload_json(&synced);
    assert_eq!(synced_payload["type"], "subscriptions.synced");

    runtime
        .publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "messageId": "msg_ccp_push_1",
                "summary": "hello ccp push"
            })
            .to_string(),
            vec!["d_pad".into()],
        )
        .expect("live publish should succeed");

    let pushed_window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(pushed_window.schema, "cc.realtime.event.window.v1");
    let pushed_payload = envelope_payload_json(&pushed_window);
    assert_eq!(pushed_payload["type"], "event.window");
    assert_eq!(pushed_payload["reason"], "push");
    assert_eq!(pushed_payload["window"]["deviceId"], "d_pad");
    assert_eq!(
        pushed_payload["window"]["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        pushed_payload["window"]["items"][0]["eventType"],
        "message.posted"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_skips_session_resume_when_capability_not_negotiated() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_ccp_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-no-resume".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.kind, "control");
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");
    let hello_ack_payload = envelope_payload_json(&hello_ack);
    assert_eq!(hello_ack_payload["type"], "hello_ack");
    assert_eq!(hello_ack_payload["data"]["accepted"], true);
    assert_eq!(
        hello_ack_payload["data"]["capabilities"]["items"],
        json!(["payload.json"])
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "1".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
                auth_token: None,
                access_token: None,
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.kind, "control");
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");
    let auth_ok_payload = envelope_payload_json(&auth_ok);
    assert_eq!(auth_ok_payload["type"], "auth_ok");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.kind, "evt");
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");
    assert_eq!(connected_payload["deviceId"], "d_pad");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_uses_runtime_link_queue_owner_limits_for_catchup_and_pull() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=520 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let catchup = next_ccp_business_payload(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["hasMore"], true);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"req_pull_backpressure_1",
            "afterSeq":0,
            "limit":999
        }),
    )
    .await;

    let pull = next_ccp_business_payload(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_backpressure_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["hasMore"], true);
    assert_eq!(pull["window"]["nextAfterSeq"], 512);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_degrades_live_push_to_pull_only_when_runtime_link_detects_overload()
 {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=900 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let catchup = next_ccp_business_payload(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    runtime
        .publish_scope_event_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "index": 901
            })
            .to_string(),
            vec!["d_pad".into()],
        )
        .expect("overload publish should succeed");

    assert!(
        timeout(Duration::from_millis(250), socket.next())
            .await
            .is_err(),
        "live push should degrade to pull-only under overload backlog"
    );

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"req_pull_after_overload_1",
            "afterSeq":128,
            "limit":999
        }),
    )
    .await;

    let pull = next_ccp_business_payload(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_after_overload_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_clamps_stale_pull_replay_when_backlog_is_still_over_hard_limit() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=900 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let catchup = next_ccp_business_payload(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["hasMore"], true);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"req_pull_stale_replay_overload_1",
            "afterSeq":0,
            "limit":999
        }),
    )
    .await;

    let pull = next_ccp_business_payload(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_stale_replay_overload_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["items"][0]["realtimeSeq"], 129);
    assert_eq!(pull["window"]["hasMore"], true);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit()
{
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=700 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let catchup = next_ccp_business_payload(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    send_ccp_business_command(
        &mut socket,
        "cc.realtime.events.pull.v1",
        json!({
            "type":"events.pull",
            "requestId":"req_pull_recovery_1",
            "afterSeq":128,
            "limit":999
        }),
    )
    .await;

    let pull = next_ccp_business_payload(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_recovery_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let recovered_push = next_ccp_business_payload(&mut socket).await;
    assert_eq!(recovered_push["type"], "event.window");
    assert_eq!(recovered_push["reason"], "push");
    assert_eq!(
        recovered_push["window"]["items"].as_array().unwrap().len(),
        60
    );
    assert_eq!(recovered_push["window"]["hasMore"], false);
    assert_eq!(recovered_push["window"]["nextAfterSeq"], 700);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_when_runtime_link_detects_extreme_overload_backlog() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::permissive_for_tests());
    runtime
        .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("client route state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "100001",
                "default",
                "1",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=1200 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
    let (_env, address, handle) = spawn_server(app).await;
    let mut socket = connect_ccp_realtime_socket(&address).await;
    let connected = complete_ccp_realtime_handshake(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["trimmedThroughSeq"], 200);

    let catchup = next_ccp_business_payload(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["items"][0]["realtimeSeq"], 201);
    assert_eq!(catchup["window"]["nextAfterSeq"], 328);

    for index in 1201..=1353 {
        runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("extreme overload publish should succeed");
    }

    assert_ccp_overload_disconnect(&mut socket).await;

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_legacy_json_upgrade_by_default() {
    let app = session_gateway::build_app();
    let (_env, address, handle) = spawn_server(app).await;
    let request = authenticated_json_request(format!("ws://{address}/im/v3/api/realtime/ws"));

    let error = connect_async(request)
        .await
        .expect_err("legacy plain-json websocket upgrade should be rejected by default");

    assert!(
        error.to_string().contains("400")
            || error.to_string().to_ascii_lowercase().contains("reject"),
        "upgrade failure should surface as HTTP rejection, got {error}"
    );

    handle.abort();
    let _ = handle.await;
}
