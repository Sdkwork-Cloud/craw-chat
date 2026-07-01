use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use futures_util::{SinkExt, StreamExt};
use im_adapters_postgres_realtime::{
    PostgresRealtimeCheckpointStore, PostgresRealtimeConfig, PostgresRealtimeEventWindowStore,
    PostgresRealtimePool, PostgresRealtimeSubscriptionStore,
};
use serde_json::json;
use session_gateway::{RealtimeClusterBridge, RealtimeDeliveryRuntime};
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL";
const CORE_SCHEMA_SQL: &str = include_str!(
    "../../../database/ddl/baseline/postgres/0001_im_baseline.sql"
);
const TENANT_ID_PREFIX: &str = "t_ws_pg_drill";
const PRINCIPAL_ID_PREFIX: &str = "1132";
const SESSION_ID: &str = "s_ws_pg_drill";
const DEVICE_ID_PREFIX: &str = "d_ws_pg_drill";
const CONVERSATION_ID_PREFIX: &str = "c_ws_pg_drill";

#[tokio::test]
async fn test_postgres_realtime_websocket_recovers_and_trims_across_runtime_restart() {
    let Some(database_url) = std::env::var(POSTGRES_TEST_DATABASE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        eprintln!(
            "skipping session-gateway PostgreSQL realtime websocket live drill because {POSTGRES_TEST_DATABASE_URL_ENV} is not set"
        );
        return;
    };

    let config = PostgresRealtimeConfig::new(database_url)
        .with_pool_max_size(4)
        .with_pool_min_idle(0);
    let pool = config
        .connect_pool()
        .expect("PostgreSQL realtime websocket live drill pool should connect");
    apply_core_schema(&pool);

    let suffix = unique_suffix();
    let tenant_id = format!("{TENANT_ID_PREFIX}_{suffix}");
    let principal_id = format!("{PRINCIPAL_ID_PREFIX}_{suffix}");
    let device_id = format!("{DEVICE_ID_PREFIX}_{suffix}");
    let conversation_id = format!("{CONVERSATION_ID_PREFIX}_{suffix}");

    let initial_runtime = Arc::new(runtime_for_pool(pool.clone()));
    let initial_app = realtime_app(initial_runtime.clone());
    let (initial_address, initial_handle) = spawn_server(initial_app).await;
    let mut socket = connect_legacy_json_socket(
        initial_address.as_str(),
        &tenant_id,
        &principal_id,
        &device_id,
    )
    .await;

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["ackedThroughSeq"], 0);
    assert_eq!(connected["trimmedThroughSeq"], 0);
    assert_eq!(connected["latestRealtimeSeq"], 0);

    send_sync_subscription(
        &mut socket,
        "req_sync_pg_ws_1",
        conversation_id.as_str(),
        vec!["message.posted"],
    )
    .await;
    let synced = next_text_json(&mut socket).await;
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["snapshot"]["deviceId"], device_id);
    assert_eq!(synced["snapshot"]["items"][0]["scopeId"], conversation_id);

    let first_delivery = initial_runtime
        .publish_scope_event_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            "conversation",
            conversation_id.as_str(),
            "message.posted",
            json!({
                "messageId": format!("m_ws_pg_{suffix}_1"),
                "summary": "first live websocket PostgreSQL event"
            })
            .to_string(),
            vec![device_id.clone()],
        )
        .expect("PostgreSQL runtime should publish live websocket event");
    assert_eq!(first_delivery, 1);

    let pushed = next_text_json(&mut socket).await;
    assert_event_window(
        &pushed,
        "push",
        &[1],
        &[format!("m_ws_pg_{suffix}_1").as_str()],
    );

    socket
        .close(None)
        .await
        .expect("first websocket should close cleanly");
    initial_handle.abort();
    let _ = initial_handle.await;

    let offline_delivery = initial_runtime
        .publish_scope_event_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            "conversation",
            conversation_id.as_str(),
            "message.posted",
            json!({
                "messageId": format!("m_ws_pg_{suffix}_2"),
                "summary": "offline event before runtime rebuild"
            })
            .to_string(),
            vec![device_id.clone()],
        )
        .expect("PostgreSQL runtime should persist offline websocket event");
    assert_eq!(offline_delivery, 1);

    let rebuilt_runtime = Arc::new(runtime_for_pool(pool.clone()));
    let rebuilt_app = realtime_app(rebuilt_runtime.clone());
    let (rebuilt_address, rebuilt_handle) = spawn_server(rebuilt_app).await;
    let mut resumed_socket = connect_legacy_json_socket(
        rebuilt_address.as_str(),
        &tenant_id,
        &principal_id,
        &device_id,
    )
    .await;

    let reconnected = next_text_json(&mut resumed_socket).await;
    assert_eq!(reconnected["type"], "realtime.connected");
    assert_eq!(reconnected["ackedThroughSeq"], 0);
    assert_eq!(reconnected["trimmedThroughSeq"], 0);
    assert_eq!(reconnected["latestRealtimeSeq"], 2);

    let catchup = next_text_json(&mut resumed_socket).await;
    assert_event_window(
        &catchup,
        "catchup",
        &[1, 2],
        &[
            format!("m_ws_pg_{suffix}_1").as_str(),
            format!("m_ws_pg_{suffix}_2").as_str(),
        ],
    );

    send_ack(&mut resumed_socket, "req_ack_pg_ws_1", 2).await;
    let acked = next_text_json(&mut resumed_socket).await;
    assert_eq!(acked["type"], "events.acked");
    assert_eq!(acked["ack"]["ackedThroughSeq"], 2);
    assert_eq!(acked["ack"]["trimmedThroughSeq"], 2);
    assert_eq!(acked["ack"]["retainedEventCount"], 0);

    resumed_socket
        .close(None)
        .await
        .expect("resumed websocket should close cleanly");
    rebuilt_handle.abort();
    let _ = rebuilt_handle.await;

    let after_ack_runtime = Arc::new(runtime_for_pool(pool.clone()));
    let after_ack_app = realtime_app(after_ack_runtime);
    let (after_ack_address, after_ack_handle) = spawn_server(after_ack_app).await;
    let mut after_ack_socket = connect_legacy_json_socket(
        after_ack_address.as_str(),
        &tenant_id,
        &principal_id,
        &device_id,
    )
    .await;

    let after_ack_connected = next_text_json(&mut after_ack_socket).await;
    assert_eq!(after_ack_connected["type"], "realtime.connected");
    assert_eq!(after_ack_connected["ackedThroughSeq"], 2);
    assert_eq!(after_ack_connected["trimmedThroughSeq"], 2);
    assert_eq!(after_ack_connected["latestRealtimeSeq"], 2);

    assert!(
        timeout(Duration::from_millis(300), after_ack_socket.next())
            .await
            .is_err(),
        "reconnect after ack/trim must not replay already trimmed events"
    );

    send_pull(&mut after_ack_socket, "req_pull_pg_ws_after_trim_1", 0, 10).await;
    let post_trim_pull = next_text_json(&mut after_ack_socket).await;
    assert_eq!(post_trim_pull["type"], "event.window");
    assert_eq!(post_trim_pull["reason"], "pull");
    assert_eq!(post_trim_pull["window"]["ackedThroughSeq"], 2);
    assert_eq!(post_trim_pull["window"]["trimmedThroughSeq"], 2);
    assert_eq!(
        post_trim_pull["window"]["items"]
            .as_array()
            .expect("post-trim window items should be an array")
            .len(),
        0
    );

    after_ack_socket
        .close(None)
        .await
        .expect("post-trim websocket should close cleanly");
    after_ack_handle.abort();
    let _ = after_ack_handle.await;
}

fn realtime_app(runtime: Arc<RealtimeDeliveryRuntime>) -> Router {
    session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(RealtimeClusterBridge::default()),
        runtime,
    )
}

fn runtime_for_pool(pool: PostgresRealtimePool) -> RealtimeDeliveryRuntime {
    RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        Arc::new(PostgresRealtimeCheckpointStore::from_pool(pool.clone())),
        Arc::new(PostgresRealtimeSubscriptionStore::from_pool(pool.clone())),
        Arc::new(PostgresRealtimeEventWindowStore::from_pool(pool)),
    )
}

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("127.0.0.1:{}", address.port()), handle)
}

fn test_auth_token(tenant_id: &str, principal_id: &str) -> String {
    json!({
        "tenant_id": tenant_id,
        "login_scope": "TENANT",
        "user_id": principal_id,
        "session_id": SESSION_ID,
        "app_id": "sdkwork-im",
        "auth_level": "password",
        "subject_type": "user"
    })
    .to_string()
}

fn test_access_token(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    json!({
        "tenant_id": tenant_id,
        "login_scope": "TENANT",
        "user_id": principal_id,
        "session_id": SESSION_ID,
        "app_id": "sdkwork-im",
        "environment": "dev",
        "deployment_mode": "saas",
        "auth_level": "password",
        "actor_id": principal_id,
        "actor_kind": "user",
        "device_id": device_id,
        "data_scope": ["tenant"],
        "permission_scope": ["*"],
        "subject_type": "user"
    })
    .to_string()
}

fn insert_test_dual_token_headers(
    headers: &mut tokio_tungstenite::tungstenite::http::HeaderMap,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) {
    headers.insert(
        tokio_tungstenite::tungstenite::http::header::AUTHORIZATION,
        format!("Bearer {}", test_auth_token(tenant_id, principal_id))
            .parse()
            .expect("auth token header should parse"),
    );
    headers.insert(
        "Access-Token",
        test_access_token(tenant_id, principal_id, device_id)
            .parse()
            .expect("access token header should parse"),
    );
}

async fn connect_legacy_json_socket(
    address: &str,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let mut request = format!("ws://{address}/im/v3/api/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    insert_test_dual_token_headers(request.headers_mut(), tenant_id, principal_id, device_id);

    connect_async(request)
        .await
        .expect("websocket connection should succeed")
        .0
}

async fn send_sync_subscription(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    request_id: &str,
    conversation_id: &str,
    event_types: Vec<&str>,
) {
    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":request_id,
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":conversation_id,
                        "eventTypes":event_types
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscription sync frame should send");
}

async fn send_ack(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    request_id: &str,
    acked_seq: u64,
) {
    socket
        .send(Message::Text(
            json!({
                "type":"events.ack",
                "requestId":request_id,
                "ackedSeq":acked_seq
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("ack frame should send");
}

async fn send_pull(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    request_id: &str,
    after_seq: u64,
    limit: usize,
) {
    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":request_id,
                "afterSeq":after_seq,
                "limit":limit
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("pull frame should send");
}

async fn next_text_json(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> serde_json::Value {
    let message = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should stay open")
        .expect("websocket frame should decode");
    match message {
        Message::Text(text) => serde_json::from_str(text.as_str())
            .expect("websocket text frame should contain valid json"),
        other => panic!("expected text frame, got {other:?}"),
    }
}

fn assert_event_window(
    frame: &serde_json::Value,
    reason: &str,
    expected_sequences: &[u64],
    expected_message_ids: &[&str],
) {
    assert_eq!(frame["type"], "event.window");
    assert_eq!(frame["reason"], reason);
    let items = frame["window"]["items"]
        .as_array()
        .expect("event window items should be an array");
    assert_eq!(items.len(), expected_sequences.len());
    assert_eq!(expected_sequences.len(), expected_message_ids.len());
    for ((item, expected_seq), expected_message_id) in items
        .iter()
        .zip(expected_sequences.iter())
        .zip(expected_message_ids.iter())
    {
        assert_eq!(item["realtimeSeq"], *expected_seq);
        let payload: serde_json::Value = serde_json::from_str(
            item["payload"]
                .as_str()
                .expect("event payload should be serialized JSON"),
        )
        .expect("event payload should decode as JSON");
        assert_eq!(payload["messageId"], *expected_message_id);
    }
}

fn apply_core_schema(pool: &PostgresRealtimePool) {
    let pool = pool.clone();
    std::thread::spawn(move || {
        let mut client = pool.get().expect("PostgreSQL schema client should connect");
        client
            .batch_execute(CORE_SCHEMA_SQL)
            .expect("core PostgreSQL schema should apply before websocket live drill");
    })
    .join()
    .expect("PostgreSQL schema worker thread should not panic");
}

fn unique_suffix() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
        .to_string()
}
