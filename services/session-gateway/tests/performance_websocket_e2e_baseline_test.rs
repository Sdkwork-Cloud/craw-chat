use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use futures_util::{SinkExt, StreamExt};
use im_adapters_local_memory::{
    MemoryRealtimeCheckpointStore, MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use sdkwork_im_ccp_binding_ws::{CCP_WS_SUBPROTOCOL, WsBinding, WsBindingMessage, WsOpcode};
use sdkwork_im_ccp_codec::CcpCodec;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{AuthBindFrame, ControlFrame, HelloFrame};
use sdkwork_im_ccp_core::{CapabilitySet, CcpEnvelope, ProtocolVersion, TransportBinding};
use serde::Deserialize;
use serde_json::{Value, json};
use session_gateway::{RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeRuntimeError};
use tokio::net::TcpListener;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::ClientRequestBuilder;

mod test_env;

const TENANT_ID: &str = "t_step11_ws_e2e";
const PRINCIPAL_ID: &str = "1131";
const PRINCIPAL_KIND: &str = "user";
const CONVERSATION_ID: &str = "c_step11_ws_e2e";
const EVENT_TYPE: &str = "message.posted";
const SESSION_ID: &str = "s_step11_ws_e2e";
const NODE_ID: &str = "session_gateway_local_1";

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Step11WebsocketE2eBaseline {
    profile: String,
    tier: String,
    websocket_e2e: WebsocketE2eBaseline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebsocketE2eBaseline {
    connected_device_count: usize,
    live_message_count: usize,
    backlog_message_count: usize,
    expected_fanout_per_live_message: usize,
    expected_capacity_trimmed_event_count: u64,
    ack_checkpoint_seq: u64,
    min_connect_success_permille: u64,
    min_live_fanout_success_permille: u64,
    max_connect_p95_ms: f64,
    max_subscribe_p95_ms: f64,
    max_live_push_p95_ms: f64,
    max_ack_p95_ms: f64,
    max_disconnect_recovery_ms: f64,
    max_backlog_restore_ms: f64,
    max_cluster_handoff_ms: f64,
}

struct TestServer {
    address: String,
    handle: tokio::task::JoinHandle<()>,
}

impl TestServer {
    fn websocket_url(&self) -> String {
        format!("ws://{}/im/v3/api/realtime/ws", self.address)
    }

    async fn shutdown(self) {
        self.handle.abort();
        let _ = self.handle.await;
    }
}

struct ConnectedDevice {
    device_id: String,
    socket: WsStream,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn websocket_e2e_baseline_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-cp11-5-websocket-e2e-baseline.json")
}

fn step11_catalog_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json")
}

fn operator_doc_path() -> PathBuf {
    workspace_root()
        .join("docs")
        .join("\u{90E8}\u{7F72}")
        .join("\u{6027}\u{80FD}\u{4E0E}\u{707E}\u{5907}\u{6F14}\u{7EC3}\u{573A}\u{666F}.md")
}

fn read_operator_doc() -> String {
    let doc_path = operator_doc_path();
    let doc_bytes = fs::read(&doc_path).unwrap_or_else(|err| {
        panic!(
            "missing Step 11 operator doc: {} ({err})",
            doc_path.display()
        );
    });
    String::from_utf8_lossy(&doc_bytes).into_owned()
}

fn load_websocket_e2e_baseline() -> Step11WebsocketE2eBaseline {
    let path = websocket_e2e_baseline_path();
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing IM websocket E2E baseline: {}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid IM websocket E2E baseline: {}", path.display()))
}

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("websocket E2E performance gate operation should succeed")
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn percentile_ms(samples: &[f64], percentile: f64) -> f64 {
    assert!(!samples.is_empty(), "percentile samples must not be empty");
    let mut ordered = samples.to_vec();
    ordered.sort_by(|left, right| {
        left.partial_cmp(right)
            .expect("latency samples should be comparable")
    });
    let rank = ((ordered.len() as f64) * percentile).ceil() as usize;
    ordered[rank.saturating_sub(1).min(ordered.len() - 1)]
}

fn message_payload(index: usize) -> String {
    json!({
        "messageId": format!("msg_step11_ws_e2e_{index:04}"),
        "index": index
    })
    .to_string()
}

async fn spawn_server(app: Router) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    TestServer {
        address: format!("127.0.0.1:{}", address.port()),
        handle,
    }
}

async fn next_message(socket: &mut WsStream) -> Message {
    timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should stay open")
        .expect("websocket frame should decode")
}

fn encode_ccp_text_frame(schema: &str, kind: &str, payload: Value) -> Message {
    let codec = JsonEnvelopeCodec::new();
    let binding = WsBinding::new();
    let envelope = CcpEnvelope::new(
        ProtocolVersion::new("ccp", 1, 0),
        TransportBinding::Ws1,
        kind,
        schema,
        None,
        None,
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

fn envelope_payload_json(envelope: CcpEnvelope) -> Value {
    serde_json::from_str(envelope.payload.as_str()).expect("ccp payload should be valid json")
}

async fn connect_ccp_device(url: &str, device_id: &str) -> (ConnectedDevice, f64) {
    let request = ClientRequestBuilder::new(url.parse().expect("websocket url should parse"))
        .with_sub_protocol(CCP_WS_SUBPROTOCOL)
        .with_header(
            "authorization",
            format!(
                "Bearer {}",
                json!({
                    "tenant_id": TENANT_ID,
                    "login_scope": "TENANT",
                    "user_id": PRINCIPAL_ID,
                    "session_id": SESSION_ID,
                    "app_id": "sdkwork-im",
                    "auth_level": "password",
                    "subject_type": PRINCIPAL_KIND
                })
            ),
        )
        .with_header(
            "Access-Token",
            json!({
                "tenant_id": TENANT_ID,
                "login_scope": "TENANT",
                "user_id": PRINCIPAL_ID,
                "session_id": SESSION_ID,
                "app_id": "sdkwork-im",
                "environment": "dev",
                "deployment_mode": "saas",
                "auth_level": "password",
                "actor_id": PRINCIPAL_ID,
                "actor_kind": PRINCIPAL_KIND,
                "device_id": device_id,
                "data_scope": ["tenant"],
                "permission_scope": ["*"],
                "subject_type": PRINCIPAL_KIND
            })
            .to_string(),
        );

    let started = Instant::now();
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
                trace_id: Some(format!("trace-step11-ws-e2e-{device_id}")),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.kind, "control");
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: PRINCIPAL_ID.into(),
                device_id: Some(device_id.into()),
                session_id: Some(SESSION_ID.into()),
                actor_kind: PRINCIPAL_KIND.into(),
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

    let connected = envelope_payload_json(decode_ccp_envelope(next_message(&mut socket).await));
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], device_id);
    (
        ConnectedDevice {
            device_id: device_id.into(),
            socket,
        },
        started.elapsed().as_secs_f64() * 1000.0,
    )
}

async fn sync_subscription(device: &mut ConnectedDevice, request_id: &str) -> f64 {
    let started = Instant::now();
    device
        .socket
        .send(encode_ccp_text_frame(
            "cc.realtime.subscriptions.sync.v1",
            "cmd",
            json!({
                "type": "subscriptions.sync",
                "requestId": request_id,
                "items": [
                    {
                        "scopeType": "conversation",
                        "scopeId": CONVERSATION_ID,
                        "eventTypes": [EVENT_TYPE]
                    }
                ]
            }),
        ))
        .await
        .expect("subscription sync frame should send");
    let synced = envelope_payload_json(decode_ccp_envelope(next_message(&mut device.socket).await));
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["requestId"], request_id);
    started.elapsed().as_secs_f64() * 1000.0
}

async fn ack_device(device: &mut ConnectedDevice, request_id: &str, acked_seq: u64) -> f64 {
    let started = Instant::now();
    device
        .socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.ack.v1",
            "ack",
            json!({
                "type": "events.ack",
                "requestId": request_id,
                "ackedSeq": acked_seq
            }),
        ))
        .await
        .expect("ack frame should send");
    let acked = envelope_payload_json(decode_ccp_envelope(next_message(&mut device.socket).await));
    assert_eq!(acked["type"], "events.acked");
    assert_eq!(acked["requestId"], request_id);
    assert_eq!(acked["ack"]["ackedThroughSeq"], acked_seq);
    started.elapsed().as_secs_f64() * 1000.0
}

async fn pull_client_route_window(
    device: &mut ConnectedDevice,
    request_id: &str,
    after_seq: u64,
    limit: usize,
) -> (Value, f64) {
    let started = Instant::now();
    device
        .socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type": "events.pull",
                "requestId": request_id,
                "afterSeq": after_seq,
                "limit": limit
            }),
        ))
        .await
        .expect("pull frame should send");
    let window = envelope_payload_json(decode_ccp_envelope(next_message(&mut device.socket).await));
    assert_eq!(window["type"], "event.window");
    assert_eq!(window["requestId"], request_id);
    assert_eq!(window["reason"], "pull");
    (window, started.elapsed().as_secs_f64() * 1000.0)
}

fn publish_message(
    runtime: &RealtimeDeliveryRuntime,
    index: usize,
    candidate_device_ids: Vec<String>,
) -> usize {
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(index),
        candidate_device_ids,
    ))
}

fn publish_cluster_device_message(
    cluster: &RealtimeClusterBridge,
    device_id: &str,
    index: usize,
) -> usize {
    let result = cluster.publish_client_route_event_for_principal_kind(
        NODE_ID,
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        device_id,
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(index),
    );
    assert_eq!(result.route_state, "resolved");
    assert_eq!(result.delivery_error_code, None);
    result.delivered
}

async fn next_push_window(device: &mut ConnectedDevice) -> (Value, f64) {
    let started = Instant::now();
    let pushed = envelope_payload_json(decode_ccp_envelope(next_message(&mut device.socket).await));
    assert_eq!(pushed["type"], "event.window");
    assert_eq!(pushed["reason"], "push");
    (pushed, started.elapsed().as_secs_f64() * 1000.0)
}

fn append_window_realtime_seqs(window: &Value, restored_seqs: &mut Vec<u64>) {
    let items = window["window"]["items"]
        .as_array()
        .expect("window items should be an array");
    for item in items {
        restored_seqs.push(
            item["realtimeSeq"]
                .as_u64()
                .expect("window realtimeSeq should be u64"),
        );
    }
}

async fn next_catchup_window(device: &mut ConnectedDevice) -> Value {
    let catchup =
        envelope_payload_json(decode_ccp_envelope(next_message(&mut device.socket).await));
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    catchup
}

async fn wait_until_routes_released(cluster: &RealtimeClusterBridge, device_ids: &[&str]) {
    timeout(Duration::from_secs(5), async {
        loop {
            if device_ids.iter().all(|device_id| {
                cluster
                    .resolve_client_route_for_principal_kind(
                        TENANT_ID,
                        "default",
                        PRINCIPAL_ID,
                        PRINCIPAL_KIND,
                        device_id,
                    )
                    .is_none()
            }) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("closed websocket routes should be released before reconnect drill");
}

#[test]
fn test_step11_websocket_e2e_baseline_config_is_frozen() {
    let baseline = load_websocket_e2e_baseline();
    assert_eq!(baseline.profile, "session-gateway");
    assert_eq!(baseline.tier, "CI Smoke Tier");
    assert!(baseline.websocket_e2e.connected_device_count >= 3);
    assert!(baseline.websocket_e2e.live_message_count >= 12);
    assert!(baseline.websocket_e2e.backlog_message_count > 1_000);
    assert_eq!(
        baseline.websocket_e2e.expected_fanout_per_live_message,
        baseline.websocket_e2e.connected_device_count
    );
    assert!(baseline.websocket_e2e.expected_capacity_trimmed_event_count > 0);
    assert!(baseline.websocket_e2e.ack_checkpoint_seq > 0);
    assert_eq!(baseline.websocket_e2e.min_connect_success_permille, 1_000);
    assert_eq!(
        baseline.websocket_e2e.min_live_fanout_success_permille,
        1_000
    );
    assert!(baseline.websocket_e2e.max_connect_p95_ms > 0.0);
    assert!(baseline.websocket_e2e.max_subscribe_p95_ms > 0.0);
    assert!(baseline.websocket_e2e.max_live_push_p95_ms > 0.0);
    assert!(baseline.websocket_e2e.max_ack_p95_ms > 0.0);
    assert!(baseline.websocket_e2e.max_disconnect_recovery_ms > 0.0);
    assert!(baseline.websocket_e2e.max_backlog_restore_ms > 0.0);
    assert!(baseline.websocket_e2e.max_cluster_handoff_ms > 0.0);

    let catalog_path = step11_catalog_path();
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    for required_text in [
        "\"family\": \"im-websocket-e2e\"",
        "services/session-gateway/tests/performance_websocket_e2e_baseline_test.rs",
        "services/session-gateway/tests/websocket_smoke_test.rs",
        "tools/perf/step-11-cp11-5-websocket-e2e-baseline.json",
    ] {
        assert!(
            catalog.contains(required_text),
            "Step 11 catalog must reference IM websocket E2E asset {required_text}"
        );
    }

    let doc = read_operator_doc();
    for required_text in [
        "`im-websocket-e2e`",
        "STEP11_WEBSOCKET_E2E",
        "step-11-cp11-5-websocket-e2e-baseline.json",
        "performance_websocket_e2e_baseline_test.rs",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 11 operator doc must reference IM websocket E2E marker {required_text}"
        );
    }
}

#[tokio::test]
async fn test_step11_websocket_e2e_quant_gate_emits_thresholded_metrics() {
    let _env = test_env::dev_test_environment();
    let baseline = load_websocket_e2e_baseline();
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let runtime = Arc::new(
        RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
            checkpoint_store.clone(),
            subscription_store.clone(),
            event_window_store.clone(),
        ),
    );
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_and_runtime(cluster.clone(), runtime.clone());
    let server = spawn_server(app).await;

    let device_ids = ["d_primary", "d_mobile", "d_tablet"];
    assert_eq!(
        baseline.websocket_e2e.connected_device_count,
        device_ids.len()
    );
    let mut devices = Vec::with_capacity(device_ids.len());
    let mut connect_latencies_ms = Vec::with_capacity(device_ids.len());
    for device_id in device_ids {
        let (device, latency_ms) =
            connect_ccp_device(server.websocket_url().as_str(), device_id).await;
        connect_latencies_ms.push(latency_ms);
        devices.push(device);
    }
    let connect_success_permille =
        ((devices.len() * 1_000) / baseline.websocket_e2e.connected_device_count) as u64;
    assert!(connect_success_permille >= baseline.websocket_e2e.min_connect_success_permille);

    let mut subscribe_latencies_ms = Vec::with_capacity(devices.len());
    for device in &mut devices {
        subscribe_latencies_ms.push(
            sync_subscription(device, format!("req_sync_{}", device.device_id).as_str()).await,
        );
    }

    let candidate_device_ids = devices
        .iter()
        .map(|device| device.device_id.clone())
        .chain(["d_other_conversation".to_owned(), "d_missing".to_owned()])
        .collect::<Vec<_>>();
    let mut push_latencies_ms = Vec::new();
    let mut live_delivered_event_count = 0usize;
    for index in 1..=baseline.websocket_e2e.live_message_count {
        assert_eq!(
            publish_message(&runtime, index, candidate_device_ids.clone()),
            baseline.websocket_e2e.expected_fanout_per_live_message
        );
        for device in &mut devices {
            let (pushed, latency_ms) = next_push_window(device).await;
            assert_eq!(
                pushed["window"]["deviceId"],
                Value::String(device.device_id.clone())
            );
            let items = pushed["window"]["items"]
                .as_array()
                .expect("push window items should be an array");
            assert_eq!(items.len(), 1);
            assert_eq!(items[0]["realtimeSeq"], index as u64);
            live_delivered_event_count += items.len();
            push_latencies_ms.push(latency_ms);
        }
    }
    let expected_live_delivered_event_count = baseline.websocket_e2e.live_message_count
        * baseline.websocket_e2e.expected_fanout_per_live_message;
    let live_fanout_success_permille =
        ((live_delivered_event_count * 1_000) / expected_live_delivered_event_count) as u64;
    assert!(
        live_fanout_success_permille >= baseline.websocket_e2e.min_live_fanout_success_permille
    );

    let mut ack_latencies_ms = Vec::new();
    let ack_seq = baseline.websocket_e2e.ack_checkpoint_seq;
    for device in &mut devices {
        ack_latencies_ms.push(
            ack_device(
                device,
                format!("req_ack_{}", device.device_id).as_str(),
                ack_seq,
            )
            .await,
        );
    }

    let disconnect_started = Instant::now();
    for device in &mut devices {
        device
            .socket
            .close(None)
            .await
            .expect("websocket close should send");
    }
    wait_until_routes_released(&cluster, &device_ids).await;
    let disconnect_recovery_ms = disconnect_started.elapsed().as_secs_f64() * 1000.0;

    for index in (baseline.websocket_e2e.live_message_count + 1)
        ..=baseline.websocket_e2e.backlog_message_count
    {
        assert_eq!(
            publish_message(&runtime, index, vec!["d_primary".into()]),
            1
        );
    }

    let backlog_restore_started = Instant::now();
    let (mut reconnected, reconnect_connect_ms) =
        connect_ccp_device(server.websocket_url().as_str(), "d_primary").await;
    connect_latencies_ms.push(reconnect_connect_ms);

    let catchup_window = next_catchup_window(&mut reconnected).await;
    let catchup_items = catchup_window["window"]["items"]
        .as_array()
        .expect("catchup window items should be an array");
    assert!(!catchup_items.is_empty());
    assert_eq!(catchup_items[0]["realtimeSeq"], 51);
    let mut restored_seqs = Vec::new();
    append_window_realtime_seqs(&catchup_window, &mut restored_seqs);

    let (backlog_tail_window, backlog_pull_ms) =
        pull_client_route_window(&mut reconnected, "req_pull_backlog_primary", 0, 1_000).await;
    let backlog_restore_ms = backlog_restore_started.elapsed().as_secs_f64() * 1000.0;
    append_window_realtime_seqs(&backlog_tail_window, &mut restored_seqs);
    while restored_seqs.last().copied().unwrap_or_default()
        < baseline.websocket_e2e.backlog_message_count as u64
    {
        let buffered_window = envelope_payload_json(decode_ccp_envelope(
            next_message(&mut reconnected.socket).await,
        ));
        assert_eq!(buffered_window["type"], "event.window");
        assert_eq!(buffered_window["reason"], "push");
        append_window_realtime_seqs(&buffered_window, &mut restored_seqs);
    }
    assert_eq!(restored_seqs.len(), 1_000);
    assert_eq!(restored_seqs.first().copied(), Some(51));
    assert_eq!(
        restored_seqs.last().copied(),
        Some(baseline.websocket_e2e.backlog_message_count as u64)
    );
    for pair in restored_seqs.windows(2) {
        assert_eq!(pair[1], pair[0] + 1);
    }
    assert_eq!(catchup_window["window"]["ackedThroughSeq"], ack_seq);
    assert_eq!(catchup_window["window"]["trimmedThroughSeq"], 50);
    assert_eq!(backlog_tail_window["window"]["ackedThroughSeq"], ack_seq);
    assert_eq!(backlog_tail_window["window"]["trimmedThroughSeq"], 50);

    let persisted_checkpoint = checkpoint_store
        .checkpoint(TENANT_ID, "default", PRINCIPAL_KIND, PRINCIPAL_ID, "d_primary")
        .expect("persisted checkpoint should exist");
    assert_eq!(
        persisted_checkpoint.capacity_trimmed_event_count,
        baseline.websocket_e2e.expected_capacity_trimmed_event_count
    );

    let runtime_b = Arc::new(
        RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
            checkpoint_store.clone(),
            subscription_store,
            event_window_store,
        ),
    );
    cluster.bind_node_runtime("node_b", runtime_b.clone());
    let handoff_started = Instant::now();
    cluster
        .mark_node_draining(NODE_ID)
        .expect("source node should enter draining");
    cluster
        .migrate_node_routes(NODE_ID, "node_b")
        .expect("source node routes should migrate");
    let cluster_handoff_ms = handoff_started.elapsed().as_secs_f64() * 1000.0;

    let route = cluster
        .resolve_client_route_for_principal_kind(
            TENANT_ID,
            "default",
            PRINCIPAL_ID,
            PRINCIPAL_KIND,
            "d_primary",
        )
        .expect("migrated primary route should resolve");
    assert_eq!(route.owner_node_id, "node_b");
    assert_eq!(
        publish_cluster_device_message(&cluster, "d_primary", 1_251),
        1
    );
    let target_window = expect_ok(runtime_b.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        baseline.websocket_e2e.backlog_message_count as u64,
        10,
    ));
    assert_eq!(target_window.items.len(), 1);
    assert_eq!(
        target_window.items[0].realtime_seq,
        baseline.websocket_e2e.backlog_message_count as u64 + 1
    );
    let target_payload: Value = serde_json::from_str(target_window.items[0].payload.as_str())
        .expect("target handoff payload should be json");
    assert_eq!(target_payload["messageId"], "msg_step11_ws_e2e_1251");

    let connect_p95_ms = percentile_ms(&connect_latencies_ms, 0.95);
    let subscribe_p95_ms = percentile_ms(&subscribe_latencies_ms, 0.95);
    let live_push_p95_ms = percentile_ms(&push_latencies_ms, 0.95);
    let ack_p95_ms = percentile_ms(&ack_latencies_ms, 0.95);

    assert!(
        connect_p95_ms <= baseline.websocket_e2e.max_connect_p95_ms,
        "connect p95 {}ms exceeded baseline {}ms",
        round3(connect_p95_ms),
        baseline.websocket_e2e.max_connect_p95_ms
    );
    assert!(
        subscribe_p95_ms <= baseline.websocket_e2e.max_subscribe_p95_ms,
        "subscribe p95 {}ms exceeded baseline {}ms",
        round3(subscribe_p95_ms),
        baseline.websocket_e2e.max_subscribe_p95_ms
    );
    assert!(
        live_push_p95_ms <= baseline.websocket_e2e.max_live_push_p95_ms,
        "live push p95 {}ms exceeded baseline {}ms",
        round3(live_push_p95_ms),
        baseline.websocket_e2e.max_live_push_p95_ms
    );
    assert!(
        ack_p95_ms <= baseline.websocket_e2e.max_ack_p95_ms,
        "ack p95 {}ms exceeded baseline {}ms",
        round3(ack_p95_ms),
        baseline.websocket_e2e.max_ack_p95_ms
    );
    assert!(
        disconnect_recovery_ms <= baseline.websocket_e2e.max_disconnect_recovery_ms,
        "disconnect recovery {}ms exceeded baseline {}ms",
        round3(disconnect_recovery_ms),
        baseline.websocket_e2e.max_disconnect_recovery_ms
    );
    assert!(
        backlog_restore_ms <= baseline.websocket_e2e.max_backlog_restore_ms,
        "backlog restore {}ms exceeded baseline {}ms",
        round3(backlog_restore_ms),
        baseline.websocket_e2e.max_backlog_restore_ms
    );
    assert!(
        cluster_handoff_ms <= baseline.websocket_e2e.max_cluster_handoff_ms,
        "cluster handoff {}ms exceeded baseline {}ms",
        round3(cluster_handoff_ms),
        baseline.websocket_e2e.max_cluster_handoff_ms
    );

    println!(
        "STEP11_WEBSOCKET_E2E {}",
        json!({
            "scenario": "im-websocket-e2e",
            "profile": baseline.profile,
            "tier": baseline.tier,
            "connectedDeviceCount": devices.len(),
            "liveMessageCount": baseline.websocket_e2e.live_message_count,
            "backlogMessageCount": baseline.websocket_e2e.backlog_message_count,
            "connectSuccessPermille": connect_success_permille,
            "liveFanoutSuccessPermille": live_fanout_success_permille,
            "liveDeliveredEventCount": live_delivered_event_count,
            "expectedLiveDeliveredEventCount": expected_live_delivered_event_count,
            "capacityTrimmedEventCount": persisted_checkpoint.capacity_trimmed_event_count,
            "connectP95Ms": round3(connect_p95_ms),
            "subscribeP95Ms": round3(subscribe_p95_ms),
            "livePushP95Ms": round3(live_push_p95_ms),
            "ackP95Ms": round3(ack_p95_ms),
            "disconnectRecoveryMs": round3(disconnect_recovery_ms),
            "backlogPullMs": round3(backlog_pull_ms),
            "backlogRestoreMs": round3(backlog_restore_ms),
            "clusterHandoffMs": round3(cluster_handoff_ms),
            "thresholds": {
                "maxConnectP95Ms": baseline.websocket_e2e.max_connect_p95_ms,
                "maxSubscribeP95Ms": baseline.websocket_e2e.max_subscribe_p95_ms,
                "maxLivePushP95Ms": baseline.websocket_e2e.max_live_push_p95_ms,
                "maxAckP95Ms": baseline.websocket_e2e.max_ack_p95_ms,
                "maxDisconnectRecoveryMs": baseline.websocket_e2e.max_disconnect_recovery_ms,
                "maxBacklogRestoreMs": baseline.websocket_e2e.max_backlog_restore_ms,
                "maxClusterHandoffMs": baseline.websocket_e2e.max_cluster_handoff_ms
            }
        })
    );

    reconnected
        .socket
        .close(None)
        .await
        .expect("reconnected websocket close should send");
    server.shutdown().await;
}
