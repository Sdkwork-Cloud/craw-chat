use std::sync::Arc;
use std::time::Duration;

use axum::body::Bytes;
use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket, close_code};
use futures_util::StreamExt;
use im_app_context::AppContext;
use im_domain_core::realtime::RealtimeEventWindow;
use sdkwork_im_ccp_binding_ws::{WsBinding, WsBindingMessage, WsOpcode};
use sdkwork_im_ccp_codec::CcpCodec;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{AuthOkFrame, ControlFrame, ErrorFrame, HeartbeatFrame};
use sdkwork_im_ccp_core::{CcpEnvelope, CcpRoute, ProtocolVersion, TransportBinding};
use sdkwork_im_runtime_link::{
    LINK_WEBSOCKET_SUBPROTOCOL, LinkBufferedPushDrainDriver, LinkBufferedPushDrainStatus,
    LinkBufferedPushFetchedWindow, LinkBufferedPushPlan, LinkGoAwayDirective,
    LinkOutboundQueueState, LinkSession, OutboundQueuePolicy,
    REALTIME_OVERLOAD_CLOSE_CODE as RUNTIME_LINK_REALTIME_OVERLOAD_CLOSE_CODE,
    REALTIME_OVERLOAD_CLOSE_REASON as RUNTIME_LINK_REALTIME_OVERLOAD_CLOSE_REASON, ResumeWindow,
    SESSION_DISCONNECT_CLOSE_CODE as RUNTIME_LINK_SESSION_DISCONNECT_CLOSE_CODE,
    SESSION_DISCONNECT_CLOSE_REASON as RUNTIME_LINK_SESSION_DISCONNECT_CLOSE_REASON,
    session_disconnect_goaway,
};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::watch;
use tokio::time::{interval, timeout};

use crate::{
    RealtimeDeliveryRuntime, RealtimeEventWindowQuery, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput,
    client_route_registration::ClientRouteRegistration,
    link_business_contract::{LinkClientBusinessFrame, validate_link_client_business_envelope},
    realtime::RealtimeWindowCheckpoint,
    trace_identity::new_server_trace_id,
};

pub const CCP_WEBSOCKET_SUBPROTOCOL: &str = LINK_WEBSOCKET_SUBPROTOCOL;
pub const SESSION_DISCONNECT_CLOSE_CODE: u16 = RUNTIME_LINK_SESSION_DISCONNECT_CLOSE_CODE;
pub const SESSION_DISCONNECT_CLOSE_REASON: &str = RUNTIME_LINK_SESSION_DISCONNECT_CLOSE_REASON;
pub const REALTIME_OVERLOAD_CLOSE_CODE: u16 = RUNTIME_LINK_REALTIME_OVERLOAD_CLOSE_CODE;
pub const REALTIME_OVERLOAD_CLOSE_REASON: &str = RUNTIME_LINK_REALTIME_OVERLOAD_CLOSE_REASON;
const CCP_PROTOCOL_ERROR_CLOSE_REASON: &str = "ccp.protocol_error";
const REALTIME_MAX_WEBSOCKET_FRAME_TYPE_BYTES: usize = 64;
const ROUTE_CHANGE_CLOSE_GRACE_MS: u64 = 250; // Increased from 25ms to give clients more time (P2-3 fix)

// Heartbeat configuration constants for WebSocket connections
// These mirror the TCP/QUIC link heartbeat settings in link_realtime.rs
const WEBSOCKET_HEARTBEAT_INTERVAL_SECS_ENV: &str = "SDKWORK_IM_WEBSOCKET_HEARTBEAT_INTERVAL_SECS";
const WEBSOCKET_HEARTBEAT_INTERVAL_DEFAULT_SECS: u64 = 30;
const WEBSOCKET_IDLE_TIMEOUT_SECS_ENV: &str = "SDKWORK_IM_WEBSOCKET_IDLE_TIMEOUT_SECS";
const WEBSOCKET_IDLE_TIMEOUT_DEFAULT_SECS: u64 = 90; // 3x heartbeat interval for fault tolerance

fn resolve_websocket_heartbeat_interval() -> Duration {
    let secs = std::env::var(WEBSOCKET_HEARTBEAT_INTERVAL_SECS_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(WEBSOCKET_HEARTBEAT_INTERVAL_DEFAULT_SECS)
        .max(1);
    Duration::from_secs(secs)
}

fn resolve_websocket_idle_timeout() -> Duration {
    let secs = std::env::var(WEBSOCKET_IDLE_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(WEBSOCKET_IDLE_TIMEOUT_DEFAULT_SECS)
        .max(1);
    Duration::from_secs(secs)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealtimeWebsocketMode {
    LegacyJson,
    CcpJson,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientFrameEnvelope {
    #[serde(rename = "type")]
    frame_type: String,
    #[serde(default)]
    items: Vec<RealtimeSubscriptionItemInput>,
    after_seq: Option<u64>,
    limit: Option<usize>,
    acked_seq: Option<u64>,
    #[serde(default)]
    nack_through_seq: Option<u64>,
}

#[derive(Debug)]
struct ClientFrameDecodeError {
    message: String,
}

impl ClientFrameDecodeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug)]
enum DecodedClientFrame {
    Business(ClientFrameEnvelope),
    Heartbeat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealtimeRouteOwnerError {
    pub code: &'static str,
    pub message: String,
}

impl RealtimeRouteOwnerError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

pub trait RealtimeRouteOwner: Send + Sync {
    fn ensure_active_client_route_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), RealtimeRouteOwnerError>;

    fn subscribe_active_client_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRouteOwnerError>;

    fn release_active_client_route_if_current_session(&self, auth: &AppContext, device_id: &str);

    /// Clone the owner into an owned trait object so route-session checks can
    /// be moved into `spawn_blocking` without borrowing the WebSocket task.
    fn boxed_clone(&self) -> Box<dyn RealtimeRouteOwner + Send + Sync>;
}

#[derive(Clone, Copy, Debug, Default)]
struct CcpWebsocketRuntime {
    binding: WsBinding,
    codec: JsonEnvelopeCodec,
}

fn websocket_payload_too_large(
    field: &'static str,
    max_bytes: usize,
    actual_bytes: usize,
) -> RealtimeRuntimeError {
    RealtimeRuntimeError {
        code: "payload_too_large",
        message: format!(
            "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
        ),
    }
}

fn validate_client_frame_type(frame: &ClientFrameEnvelope) -> Result<(), RealtimeRuntimeError> {
    if frame.frame_type.len() > REALTIME_MAX_WEBSOCKET_FRAME_TYPE_BYTES {
        return Err(websocket_payload_too_large(
            "type",
            REALTIME_MAX_WEBSOCKET_FRAME_TYPE_BYTES,
            frame.frame_type.len(),
        ));
    }
    Ok(())
}

fn validate_ccp_client_business_envelope(
    envelope: &CcpEnvelope,
    frame: &ClientFrameEnvelope,
) -> Result<(), ClientFrameDecodeError> {
    validate_link_client_business_envelope(
        envelope,
        &LinkClientBusinessFrame {
            frame_type: frame.frame_type.clone(),
        },
    )
    .map_err(ClientFrameDecodeError::new)
}

fn validate_ccp_control_envelope(
    envelope: &CcpEnvelope,
    frame: &ControlFrame,
) -> Result<(), String> {
    let expected_schema = control_schema(frame);
    if envelope.schema != expected_schema {
        return Err(format!(
            "control frame schema mismatch: expected {}, got {}",
            expected_schema, envelope.schema
        ));
    }
    Ok(())
}

fn ccp_client_route_metadata_error() -> String {
    "client websocket frames must not supply ccp route metadata".into()
}

impl RealtimeRouteOwner for ClientRouteRegistration {
    fn ensure_active_client_route_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), RealtimeRouteOwnerError> {
        self.ensure_active_client_route_current_session(auth, device_id)
            .map_err(|error| RealtimeRouteOwnerError::new(error.code, error.message))
    }

    fn subscribe_active_client_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRouteOwnerError> {
        self.subscribe_active_client_route_epoch(auth, device_id)
            .map_err(|error| RealtimeRouteOwnerError::new(error.code, error.message))
    }

    fn release_active_client_route_if_current_session(&self, auth: &AppContext, device_id: &str) {
        self.release_active_client_route_if_current_session(auth, device_id);
    }

    fn boxed_clone(&self) -> Box<dyn RealtimeRouteOwner + Send + Sync> {
        Box::new(self.clone())
    }
}

impl CcpWebsocketRuntime {
    fn decode_message(&self, message: Message) -> Result<CcpEnvelope, String> {
        let binding_message = match message {
            Message::Text(text) => WsBindingMessage {
                protocol_id: TransportBinding::Ws1.protocol_id(),
                content_type: self.codec.content_type(),
                opcode: WsOpcode::Text,
                payload: text.to_string().into_bytes(),
            },
            Message::Binary(bytes) => WsBindingMessage {
                protocol_id: TransportBinding::Ws1.protocol_id(),
                content_type: self.codec.content_type(),
                opcode: WsOpcode::Binary,
                payload: bytes.to_vec(),
            },
            Message::Ping(_) | Message::Pong(_) => {
                return Err("ccp control/business frames must use text or binary messages".into());
            }
            Message::Close(_) => return Err("websocket closed before CCP frame arrived".into()),
        };
        let envelope = self
            .binding
            .decode(&binding_message, &self.codec)
            .map_err(|error| error.message().to_owned())?;
        if envelope.protocol.family != "ccp" || envelope.protocol.major != 1 {
            return Err(format!(
                "unsupported CCP protocol: {}",
                envelope.protocol.wire_id()
            ));
        }
        if envelope.binding != TransportBinding::Ws1 {
            return Err("unsupported websocket binding".into());
        }
        Ok(envelope)
    }

    async fn send_envelope(
        &self,
        socket: &mut WebSocket,
        envelope: &CcpEnvelope,
    ) -> Result<(), axum::Error> {
        let message = self
            .binding
            .encode(envelope, &self.codec)
            .map_err(axum::Error::new)?;
        match message.opcode {
            WsOpcode::Text => {
                socket
                    .send(Message::Text(
                        String::from_utf8(message.payload)
                            .expect("json ccp payload should remain utf8")
                            .into(),
                    ))
                    .await
            }
            WsOpcode::Binary => socket.send(Message::Binary(message.payload.into())).await,
        }
    }

    async fn send_control_frame(
        &self,
        socket: &mut WebSocket,
        route: &CcpRoute,
        frame: &ControlFrame,
    ) -> Result<(), axum::Error> {
        let trace_id = new_server_trace_id();
        let envelope = CcpEnvelope::new(
            ccp_protocol_version(),
            TransportBinding::Ws1,
            "control",
            control_schema(frame),
            None,
            Some(route.clone()),
            std::iter::empty::<String>(),
            Some(trace_id),
            serde_json::to_string(frame).expect("control frame should serialize"),
        );
        self.send_envelope(socket, &envelope).await
    }

    async fn send_business_payload(
        &self,
        socket: &mut WebSocket,
        route: &CcpRoute,
        kind: &str,
        schema: &str,
        trace_id: String,
        payload: Value,
    ) -> Result<(), axum::Error> {
        let envelope = CcpEnvelope::new(
            ccp_protocol_version(),
            TransportBinding::Ws1,
            kind,
            schema,
            None,
            Some(route.clone()),
            std::iter::empty::<String>(),
            Some(trace_id),
            payload.to_string(),
        );
        self.send_envelope(socket, &envelope).await
    }
}

pub async fn serve_realtime_websocket<R: RealtimeRouteOwner>(
    socket: WebSocket,
    auth: AppContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: R,
    wire_mode: RealtimeWebsocketMode,
    frame_rate_limiter: crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
) {
    let tenant_id = auth.tenant_id.clone();
    let principal_id = auth.actor_id.clone();
    let principal_kind = auth.actor_kind.clone();
    let authority = auth.ccp_authority();
    let route = CcpRoute::for_principal(
        tenant_id.clone(),
        principal_id.clone(),
        Some(device_id.clone()),
    );
    let ccp_runtime = CcpWebsocketRuntime::default();
    let sender_id = authority.sender.sender_id();
    let mut socket = socket;
    if !ensure_current_route_session_or_close(&mut socket, &route_owner, &auth, device_id.as_str())
        .await
    {
        return;
    }
    // `subscribe_active_client_route_epoch` performs blocking Redis/Postgres
    // IO via `route_store.lookup`; run it on the blocking pool so the async
    // worker stays free.
    let subscribe_epoch_owner = route_owner.boxed_clone();
    let subscribe_epoch_auth = auth.clone();
    let subscribe_epoch_device = device_id.clone();
    let mut route_epoch_receiver = match tokio::task::spawn_blocking(move || {
        subscribe_epoch_owner
            .subscribe_active_client_route_epoch(&subscribe_epoch_auth, &subscribe_epoch_device)
    })
    .await
    {
        Ok(Ok(receiver)) => receiver,
        Ok(Err(_)) => return,
        Err(join_error) => {
            tracing::error!(
                target: "sdkwork.im.session_gateway",
                tenant_id = %tenant_id,
                principal_id = %principal_id,
                device_id = %device_id,
                error = %join_error,
                "subscribe_active_client_route_epoch blocking task panicked"
            );
            return;
        }
    };

    // The three setup calls below all perform blocking Postgres IO
    // (ensure_client_route_state loads checkpoint/subscriptions/window,
    // window_checkpoint reads the latest sequence, disconnect_generation reads
    // the fence store). Batch them into a single `spawn_blocking` so only one
    // blocking-thread hop is needed and the async worker stays free during the
    // round-trips. Mirrors the pattern in `link_realtime.rs`.
    let setup_runtime = Arc::clone(&runtime);
    let setup_tenant = tenant_id.clone();
    let setup_org = auth.organization_id.clone();
    let setup_principal = principal_id.clone();
    let setup_kind = principal_kind.clone();
    let setup_device = device_id.clone();
    let setup_result = tokio::task::spawn_blocking(
        move || -> Result<(RealtimeWindowCheckpoint, u64), RealtimeRuntimeError> {
            setup_runtime.ensure_client_route_state_for_principal_kind(
                setup_tenant.as_str(),
                setup_org.as_str(),
                setup_principal.as_str(),
                setup_kind.as_str(),
                setup_device.as_str(),
            )?;
            let checkpoint = setup_runtime.window_checkpoint_for_principal_kind(
                setup_tenant.as_str(),
                setup_org.as_str(),
                setup_principal.as_str(),
                setup_kind.as_str(),
                setup_device.as_str(),
            )?;
            let disconnect_generation = setup_runtime.disconnect_generation_for_principal_kind(
                setup_tenant.as_str(),
                setup_org.as_str(),
                setup_principal.as_str(),
                setup_kind.as_str(),
                setup_device.as_str(),
            )?;
            Ok((checkpoint, disconnect_generation))
        },
    )
    .await;
    let (checkpoint, disconnect_generation) = match setup_result {
        Ok(Ok(values)) => values,
        Ok(Err(error)) => {
            let _ =
                send_initial_runtime_error(&mut socket, wire_mode, &ccp_runtime, &route, &error)
                    .await;
            return;
        }
        Err(join_error) => {
            tracing::error!(
                target: "sdkwork.im.session_gateway",
                tenant_id = %tenant_id,
                principal_id = %principal_id,
                device_id = %device_id,
                error = %join_error,
                "session setup blocking task panicked"
            );
            let _ = send_initial_runtime_error(
                &mut socket,
                wire_mode,
                &ccp_runtime,
                &route,
                &RealtimeRuntimeError {
                    code: "session_setup_failed",
                    message: format!("session setup blocking task failed: {join_error}"),
                },
            )
            .await;
            return;
        }
    };

    let mut link_session = build_link_session(&auth, device_id.as_str());
    let mut resume_after_seq = checkpoint
        .acked_through_seq
        .max(checkpoint.trimmed_through_seq);

    // `subscribe_client_route_for_principal_kind` and
    // `subscribe_disconnect_signal_for_principal_kind` both invoke
    // `ensure_client_route_state_internal`, which performs blocking Postgres
    // IO when the principal's in-memory state has been evicted. Batch them
    // into a single `spawn_blocking` so the async worker stays free.
    let subscribe_runtime = Arc::clone(&runtime);
    let subscribe_tenant = tenant_id.clone();
    let subscribe_org = auth.organization_id.clone();
    let subscribe_principal = principal_id.clone();
    let subscribe_kind = principal_kind.clone();
    let subscribe_device = device_id.clone();
    let subscribe_result = tokio::task::spawn_blocking(
        move || -> Result<(watch::Receiver<u64>, watch::Receiver<u64>), RealtimeRuntimeError> {
            let receiver = subscribe_runtime.subscribe_client_route_for_principal_kind(
                subscribe_tenant.as_str(),
                subscribe_org.as_str(),
                subscribe_principal.as_str(),
                subscribe_kind.as_str(),
                subscribe_device.as_str(),
            )?;
            let disconnect_receiver = subscribe_runtime
                .subscribe_disconnect_signal_for_principal_kind(
                    subscribe_tenant.as_str(),
                    subscribe_org.as_str(),
                    subscribe_principal.as_str(),
                    subscribe_kind.as_str(),
                    subscribe_device.as_str(),
                )?;
            Ok((receiver, disconnect_receiver))
        },
    )
    .await;
    let (mut receiver, mut disconnect_receiver) = match subscribe_result {
        Ok(Ok(values)) => values,
        Ok(Err(error)) => {
            let _ =
                send_initial_runtime_error(&mut socket, wire_mode, &ccp_runtime, &route, &error)
                    .await;
            return;
        }
        Err(join_error) => {
            tracing::error!(
                target: "sdkwork.im.session_gateway",
                tenant_id = %tenant_id,
                principal_id = %principal_id,
                device_id = %device_id,
                error = %join_error,
                "session subscribe blocking task panicked"
            );
            let _ = send_initial_runtime_error(
                &mut socket,
                wire_mode,
                &ccp_runtime,
                &route,
                &RealtimeRuntimeError {
                    code: "session_subscribe_failed",
                    message: format!("session subscribe blocking task failed: {join_error}"),
                },
            )
            .await;
            return;
        }
    };

    if wire_mode == RealtimeWebsocketMode::CcpJson {
        let handshake_context = CcpHandshakeContext {
            ccp_runtime: &ccp_runtime,
            route: &route,
            checkpoint: &checkpoint,
            route_owner: &route_owner,
            auth: &auth,
            device_id: device_id.as_str(),
        };
        let Some(negotiated_after_seq) = complete_ccp_handshake(
            &mut socket,
            &mut link_session,
            &mut route_epoch_receiver,
            handshake_context,
        )
        .await
        else {
            return;
        };
        resume_after_seq = negotiated_after_seq;
    }
    if wire_mode == RealtimeWebsocketMode::LegacyJson {
        link_session.mark_authenticated();
    }
    activate_link_session(&mut link_session, &checkpoint);
    let mut outbound_queue =
        link_session.start_outbound_queue(resume_after_seq, checkpoint.latest_realtime_seq);

    if !ensure_current_route_session_or_close(&mut socket, &route_owner, &auth, device_id.as_str())
        .await
    {
        return;
    }
    if send_business_payload(
        &mut socket,
        wire_mode,
        &ccp_runtime,
        &route,
        "evt",
        "cc.realtime.connected.v1",
        json!({
            "type": "realtime.connected",
            "tenantId": tenant_id,
            "principalId": principal_id,
            "deviceId": device_id,
            "actor": {
                "id": authority.actor.actor_id,
                "kind": authority.actor.actor_kind
            },
            "sender": {
                "principalId": authority.sender.principal_id,
                "deviceId": authority.sender.device_id,
                "sessionId": authority.sender.session_id,
                "senderId": sender_id
            },
            "ackedThroughSeq": checkpoint.acked_through_seq,
            "trimmedThroughSeq": checkpoint.trimmed_through_seq,
            "latestRealtimeSeq": checkpoint.latest_realtime_seq
        }),
    )
    .await
    .is_err()
    {
        return;
    }

    if let Some(catchup_plan) = outbound_queue.plan_catchup() {
        if !ensure_current_route_session_or_close(
            &mut socket,
            &route_owner,
            &auth,
            device_id.as_str(),
        )
        .await
        {
            return;
        }
        // `list_events_for_principal_kind` performs blocking Postgres IO;
        // run it on the blocking pool so the async worker stays free.
        let catchup_runtime = Arc::clone(&runtime);
        let catchup_tenant = auth.tenant_id.clone();
        let catchup_org = auth.organization_id.clone();
        let catchup_principal = auth.actor_id.clone();
        let catchup_kind = auth.actor_kind.clone();
        let catchup_device = device_id.clone();
        let catchup_after_seq = catchup_plan.after_seq;
        let catchup_batch_limit = catchup_plan.batch.limit;
        let catchup = match tokio::task::spawn_blocking(move || {
            catchup_runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                tenant_id: catchup_tenant.as_str(),
                organization_id: catchup_org.as_str(),
                principal_id: catchup_principal.as_str(),
                principal_kind: catchup_kind.as_str(),
                device_id: catchup_device.as_str(),
                after_seq: catchup_after_seq,
                limit: catchup_batch_limit,
            })
        })
        .await
        {
            Ok(Ok(catchup)) => catchup,
            Ok(Err(error)) => {
                let _ =
                    send_runtime_error(&mut socket, wire_mode, &ccp_runtime, &route, &error).await;
                return;
            }
            Err(join_error) => {
                tracing::error!(
                    target: "sdkwork.im.session_gateway",
                    tenant_id = %tenant_id,
                    principal_id = %principal_id,
                    device_id = %device_id,
                    error = %join_error,
                    "session catchup blocking task panicked"
                );
                let _ = send_runtime_error(
                    &mut socket,
                    wire_mode,
                    &ccp_runtime,
                    &route,
                    &RealtimeRuntimeError {
                        code: "session_catchup_failed",
                        message: format!("session catchup blocking task failed: {join_error}"),
                    },
                )
                .await;
                return;
            }
        };
        if !catchup.items.is_empty() {
            let next_after_seq = catchup.next_after_seq;
            if !ensure_current_route_session_or_close(
                &mut socket,
                &route_owner,
                &auth,
                device_id.as_str(),
            )
            .await
            {
                return;
            }
            if send_business_payload(
                &mut socket,
                wire_mode,
                &ccp_runtime,
                &route,
                "evt",
                "cc.realtime.event.window.v1",
                json!({
                    "type": "event.window",
                    "reason": "catchup",
                    "window": catchup
                }),
            )
            .await
            .is_err()
            {
                return;
            }
            let _ = outbound_queue.record_window_sent(catchup_plan.after_seq, next_after_seq);
        }
    }

    // Initialize heartbeat timer and idle timeout tracking for WebSocket connections
    // This mirrors the TCP/QUIC implementation in link_realtime.rs (P0-2 fix)
    let heartbeat_interval = resolve_websocket_heartbeat_interval();
    let idle_timeout = resolve_websocket_idle_timeout();
    let mut heartbeat_timer = interval(heartbeat_interval);
    heartbeat_timer.tick().await; // Skip the initial immediate tick
    let mut heartbeat_seq: u64 = 0;
    let mut last_activity = tokio::time::Instant::now();

    loop {
        tokio::select! {
            // Server-initiated heartbeat: periodically send a heartbeat frame to keep
            // the connection alive through proxies/LBs and to detect silent peer disconnects.
            // Also enforces idle timeout so sessions that stop making progress are reclaimed.
            _ = heartbeat_timer.tick() => {
                heartbeat_seq = heartbeat_seq.saturating_add(1);
                // In CCP mode, send a proper CCP Heartbeat control frame
                if wire_mode == RealtimeWebsocketMode::CcpJson {
                    let heartbeat_frame = ControlFrame::Heartbeat(HeartbeatFrame {
                        sequence: Some(heartbeat_seq),
                    });
                    if ccp_runtime
                        .send_control_frame(&mut socket, &route, &heartbeat_frame)
                        .await
                        .is_err()
                    {
                        tracing::debug!(
                            tenant_id = %tenant_id,
                            principal_id = %principal_id,
                            device_id = %device_id,
                            "WebSocket heartbeat send failed, closing connection"
                        );
                        break;
                    }
                } else {
                    // In Legacy JSON mode, use WebSocket Ping/Pong for heartbeat
                    if socket.send(Message::Ping(Bytes::new())).await.is_err() {
                        tracing::debug!(
                            tenant_id = %tenant_id,
                            principal_id = %principal_id,
                            device_id = %device_id,
                            "WebSocket ping send failed, closing connection"
                        );
                        break;
                    }
                }
                // Check idle timeout - if no activity for the timeout period, close connection
                if last_activity.elapsed() >= idle_timeout {
                    tracing::warn!(
                        tenant_id = %tenant_id,
                        principal_id = %principal_id,
                        device_id = %device_id,
                        idle_timeout_secs = idle_timeout.as_secs(),
                        "WebSocket connection idle timeout, closing"
                    );
                    let _ = socket
                        .send(Message::Close(Some(CloseFrame {
                            code: close_code::NORMAL,
                            reason: Utf8Bytes::from_static("idle_timeout"),
                        })))
                        .await;
                    break;
                }
            }
            route_epoch_changed = route_epoch_receiver.changed() => {
                // Reset activity timer on any route epoch change
                last_activity = tokio::time::Instant::now();
                if route_epoch_changed.is_err() {
                    break;
                }
                if !handle_route_epoch_change(
                    &mut socket,
                    &runtime,
                    &route_owner,
                    &auth,
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    device_id.as_str(),
                    &mut outbound_queue,
                    wire_mode,
                    &ccp_runtime,
                    &route,
                    &frame_rate_limiter,
                )
                .await
                {
                    break;
                }
            }
            changed = receiver.changed() => {
                // Reset activity timer on any realtime event
                last_activity = tokio::time::Instant::now();
                if changed.is_err() {
                    break;
                }

                let latest_realtime_seq = *receiver.borrow_and_update();
                let push_plan = outbound_queue.observe_latest_realtime_seq(latest_realtime_seq);
                if !drain_runtime_owned_buffered_push(
                    &mut socket,
                    runtime.as_ref(),
                    &route_owner,
                    &auth,
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    device_id.as_str(),
                    &mut outbound_queue,
                    push_plan,
                    wire_mode,
                    &ccp_runtime,
                    &route,
                )
                .await
                {
                    break;
                }
            }
            disconnect_changed = disconnect_receiver.changed() => {
                // Reset activity timer on disconnect signal
                last_activity = tokio::time::Instant::now();
                if disconnect_changed.is_err() {
                    break;
                }
                // `disconnect_generation_for_principal_kind` performs blocking
                // Postgres IO; run it on the blocking pool so the async worker
                // stays free.
                let disconnect_runtime = Arc::clone(&runtime);
                let disconnect_tenant = auth.tenant_id.clone();
                let disconnect_org = auth.organization_id.clone();
                let disconnect_principal = auth.actor_id.clone();
                let disconnect_kind = auth.actor_kind.clone();
                let disconnect_device = device_id.clone();
                let current_disconnect_generation = match tokio::task::spawn_blocking(move || {
                    disconnect_runtime.disconnect_generation_for_principal_kind(
                        disconnect_tenant.as_str(),
                        disconnect_org.as_str(),
                        disconnect_principal.as_str(),
                        disconnect_kind.as_str(),
                        disconnect_device.as_str(),
                    )
                })
                .await
                {
                    Ok(Ok(disconnect_generation)) => disconnect_generation,
                    Ok(Err(error)) => {
                        let _ = send_runtime_error(
                            &mut socket,
                            wire_mode,
                            &ccp_runtime,
                            &route,
                            &error,
                        )
                        .await;
                        break;
                    }
                    Err(join_error) => {
                        tracing::error!(
                            target: "sdkwork.im.session_gateway",
                            tenant_id = %tenant_id,
                            principal_id = %principal_id,
                            device_id = %device_id,
                            error = %join_error,
                            "disconnect_generation blocking task panicked (disconnect branch)"
                        );
                        let _ = send_runtime_error(
                            &mut socket,
                            wire_mode,
                            &ccp_runtime,
                            &route,
                            &RealtimeRuntimeError {
                                code: "disconnect_generation_failed",
                                message: format!(
                                    "disconnect_generation blocking task failed: {join_error}"
                                ),
                            },
                        )
                        .await;
                        break;
                    }
                };
                if current_disconnect_generation != disconnect_generation
                {
                    if !ensure_current_route_session_or_close(
                        &mut socket,
                        &route_owner,
                        &auth,
                        device_id.as_str(),
                    )
                    .await
                    {
                        break;
                    }
                    send_session_disconnect_signal(
                        &mut socket,
                        wire_mode,
                        &ccp_runtime,
                        &route,
                    )
                    .await;
                    break;
                }
            }
            message = socket.next() => {
                let Some(message) = message else {
                    break;
                };
                let Ok(message) = message else {
                    break;
                };
                if matches!(message, Message::Text(_) | Message::Binary(_)) {
                    last_activity = tokio::time::Instant::now();
                }
                // `disconnect_generation_for_principal_kind` performs blocking
                // Postgres IO; run it on the blocking pool so the async worker
                // stays free.
                let disconnect_runtime = Arc::clone(&runtime);
                let disconnect_tenant = auth.tenant_id.clone();
                let disconnect_org = auth.organization_id.clone();
                let disconnect_principal = auth.actor_id.clone();
                let disconnect_kind = auth.actor_kind.clone();
                let disconnect_device = device_id.clone();
                let current_disconnect_generation = match tokio::task::spawn_blocking(move || {
                    disconnect_runtime.disconnect_generation_for_principal_kind(
                        disconnect_tenant.as_str(),
                        disconnect_org.as_str(),
                        disconnect_principal.as_str(),
                        disconnect_kind.as_str(),
                        disconnect_device.as_str(),
                    )
                })
                .await
                {
                    Ok(Ok(disconnect_generation)) => disconnect_generation,
                    Ok(Err(error)) => {
                        let _ = send_runtime_error(
                            &mut socket,
                            wire_mode,
                            &ccp_runtime,
                            &route,
                            &error,
                        )
                        .await;
                        break;
                    }
                    Err(join_error) => {
                        tracing::error!(
                            target: "sdkwork.im.session_gateway",
                            tenant_id = %tenant_id,
                            principal_id = %principal_id,
                            device_id = %device_id,
                            error = %join_error,
                            "disconnect_generation blocking task panicked (message branch)"
                        );
                        let _ = send_runtime_error(
                            &mut socket,
                            wire_mode,
                            &ccp_runtime,
                            &route,
                            &RealtimeRuntimeError {
                                code: "disconnect_generation_failed",
                                message: format!(
                                    "disconnect_generation blocking task failed: {join_error}"
                                ),
                            },
                        )
                        .await;
                        break;
                    }
                };
                if current_disconnect_generation != disconnect_generation
                {
                    if !ensure_current_route_session_or_close(
                        &mut socket,
                        &route_owner,
                        &auth,
                        device_id.as_str(),
                    )
                    .await
                    {
                        break;
                    }
                    send_session_disconnect_signal(
                        &mut socket,
                        wire_mode,
                        &ccp_runtime,
                        &route,
                    )
                    .await;
                    break;
                }

                let keep_open = handle_client_message(
                    &mut socket,
                    &runtime,
                    &route_owner,
                    &auth,
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    device_id.as_str(),
                    &mut outbound_queue,
                    message,
                    wire_mode,
                    &ccp_runtime,
                    &route,
                    &frame_rate_limiter,
                )
                .await;
                if !keep_open {
                    break;
                }
            }
        }
    }
    link_session.mark_draining();
}

struct CcpHandshakeContext<'a> {
    ccp_runtime: &'a CcpWebsocketRuntime,
    route: &'a CcpRoute,
    checkpoint: &'a RealtimeWindowCheckpoint,
    route_owner: &'a dyn RealtimeRouteOwner,
    auth: &'a AppContext,
    device_id: &'a str,
}

#[allow(clippy::too_many_arguments)]
async fn handle_route_epoch_change(
    socket: &mut WebSocket,
    runtime: &RealtimeDeliveryRuntime,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    frame_rate_limiter: &crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
) -> bool {
    match timeout(
        Duration::from_millis(ROUTE_CHANGE_CLOSE_GRACE_MS),
        socket.next(),
    )
    .await
    {
        Ok(Some(Ok(message))) => {
            let keep_open = handle_client_message(
                socket,
                runtime,
                route_owner,
                auth,
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
                outbound_queue,
                message,
                wire_mode,
                ccp_runtime,
                route,
                frame_rate_limiter,
            )
            .await;
            if !keep_open {
                return false;
            }
            ensure_current_route_session_or_close(socket, route_owner, auth, device_id).await
        }
        Ok(Some(Err(_))) | Ok(None) => false,
        Err(_) => ensure_current_route_session_or_close(socket, route_owner, auth, device_id).await,
    }
}

async fn complete_ccp_handshake(
    socket: &mut WebSocket,
    link_session: &mut LinkSession,
    route_epoch_receiver: &mut watch::Receiver<u64>,
    context: CcpHandshakeContext<'_>,
) -> Option<u64> {
    if !ensure_current_route_session_or_close(
        socket,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        return None;
    }

    let hello = match receive_next_control_frame(
        socket,
        context.ccp_runtime,
        context.route,
        route_epoch_receiver,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        Ok(frame) => frame,
        Err(()) => return None,
    };
    let hello = match hello {
        ControlFrame::Hello(frame) => frame,
        other => {
            let _ = send_control_error_and_close(
                socket,
                context.ccp_runtime,
                context.route,
                "CCP_HELLO_REQUIRED",
                format!("expected hello frame, got {}", other.frame_type()),
            )
            .await;
            return None;
        }
    };

    let hello_ack = match link_session.negotiate_hello(&hello) {
        Ok(hello_ack) => hello_ack,
        Err(error) => {
            let _ = send_control_error_and_close(
                socket,
                context.ccp_runtime,
                context.route,
                error.code(),
                error.message(),
            )
            .await;
            return None;
        }
    };
    let resume_negotiated = hello_ack.capabilities.supports("session.resume");
    if !ensure_current_route_session_or_close(
        socket,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        return None;
    }
    if context
        .ccp_runtime
        .send_control_frame(socket, context.route, &ControlFrame::HelloAck(hello_ack))
        .await
        .is_err()
    {
        return None;
    }

    let auth_bind = match receive_next_control_frame(
        socket,
        context.ccp_runtime,
        context.route,
        route_epoch_receiver,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        Ok(frame) => frame,
        Err(()) => return None,
    };
    let auth_bind = match auth_bind {
        ControlFrame::AuthBind(frame) => frame,
        other => {
            let _ = send_control_error_and_close(
                socket,
                context.ccp_runtime,
                context.route,
                "CCP_AUTH_BIND_REQUIRED",
                format!("expected auth_bind frame, got {}", other.frame_type()),
            )
            .await;
            return None;
        }
    };

    if !ensure_current_route_session_or_close(
        socket,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        return None;
    }
    if !link_session.matches_auth_bind(
        auth_bind.principal_id.as_str(),
        auth_bind.actor_kind.as_str(),
        auth_bind.device_id.as_deref(),
        auth_bind.session_id.as_deref(),
    ) {
        let _ = send_control_error(
            socket,
            context.ccp_runtime,
            context.route,
            "CCP_AUTH_FAILED",
            "auth_bind does not match authenticated context",
        )
        .await;
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: close_code::POLICY,
                reason: Utf8Bytes::from_static("ccp.auth_failed"),
            })))
            .await;
        return None;
    }

    let auth_ok = ControlFrame::AuthOk(AuthOkFrame {
        tenant_id: link_session.tenant_id.clone(),
        principal_id: link_session.principal_id.clone(),
        actor_kind: link_session.actor_kind.clone(),
        device_id: Some(link_session.device_id.clone()),
        session_id: link_session.session_id.clone(),
    });
    if context
        .ccp_runtime
        .send_control_frame(socket, context.route, &auth_ok)
        .await
        .is_err()
    {
        return None;
    }
    link_session.mark_authenticated();

    if !resume_negotiated {
        return Some(context.checkpoint.acked_through_seq);
    }

    let session_resume = match receive_next_control_frame(
        socket,
        context.ccp_runtime,
        context.route,
        route_epoch_receiver,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        Ok(frame) => frame,
        Err(()) => return None,
    };
    let session_resume = match session_resume {
        ControlFrame::SessionResume(frame) => frame,
        other => {
            let _ = send_control_error_and_close(
                socket,
                context.ccp_runtime,
                context.route,
                "CCP_SESSION_RESUME_REQUIRED",
                format!("expected session_resume frame, got {}", other.frame_type()),
            )
            .await;
            return None;
        }
    };

    let directive = match link_session.negotiate_session_resume(
        &session_resume,
        context.checkpoint.latest_realtime_seq,
        context.checkpoint.acked_through_seq,
    ) {
        Ok(directive) => directive,
        Err(error) => {
            let _ = send_control_error_and_close(
                socket,
                context.ccp_runtime,
                context.route,
                error.code(),
                error.message(),
            )
            .await;
            return None;
        }
    };
    let catchup_after_seq = directive
        .catchup_after_seq
        .max(context.checkpoint.trimmed_through_seq);
    let session_resumed = ControlFrame::SessionResumed(directive.frame);
    if !ensure_current_route_session_or_close(
        socket,
        context.route_owner,
        context.auth,
        context.device_id,
    )
    .await
    {
        return None;
    }
    if context
        .ccp_runtime
        .send_control_frame(socket, context.route, &session_resumed)
        .await
        .is_err()
    {
        return None;
    }

    Some(catchup_after_seq)
}

async fn receive_next_control_frame(
    socket: &mut WebSocket,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    route_epoch_receiver: &mut watch::Receiver<u64>,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    device_id: &str,
) -> Result<ControlFrame, ()> {
    loop {
        tokio::select! {
            route_epoch_changed = route_epoch_receiver.changed() => {
                if route_epoch_changed.is_err() {
                    return Err(());
                }
                if route_owner
                    .ensure_active_client_route_current_session(auth, device_id)
                    .is_err()
                {
                    let _ = timeout(
                        Duration::from_millis(ROUTE_CHANGE_CLOSE_GRACE_MS),
                        socket.next(),
                    )
                    .await;
                }
                if !ensure_current_route_session_or_close(socket, route_owner, auth, device_id).await {
                    return Err(());
                }
            }
            next_message = socket.next() => {
                let Some(message) = next_message else {
                    return Err(());
                };
                let Ok(message) = message else {
                    return Err(());
                };
                if !ensure_current_route_session_or_close(socket, route_owner, auth, device_id).await {
                    return Err(());
                }
                match message {
                    Message::Ping(payload) => {
                        if socket.send(Message::Pong(payload)).await.is_err() {
                            return Err(());
                        }
                    }
                    Message::Pong(_) => {}
                    Message::Close(frame) => {
                        let _ = socket.send(Message::Close(frame)).await;
                        return Err(());
                    }
                    Message::Text(_) | Message::Binary(_) => {
                        let envelope = match ccp_runtime.decode_message(message) {
                            Ok(envelope) => envelope,
                            Err(error) => {
                                let _ = send_control_error_and_close(
                                    socket,
                                    ccp_runtime,
                                    route,
                                    "CCP_SCHEMA_INCOMPATIBLE",
                                    error,
                                )
                                .await;
                                return Err(());
                            }
                        };
                        if envelope.route.is_some() {
                            let _ = send_control_error_and_close(
                                socket,
                                ccp_runtime,
                                route,
                                "CCP_SCHEMA_INCOMPATIBLE",
                                ccp_client_route_metadata_error(),
                            )
                            .await;
                            return Err(());
                        }
                        if envelope.kind != "control" {
                            let _ = send_control_error_and_close(
                                socket,
                                ccp_runtime,
                                route,
                                "CCP_CONTROL_REQUIRED",
                                format!("expected control envelope, got kind {}", envelope.kind),
                            )
                            .await;
                            return Err(());
                        }
                        let control: ControlFrame = match serde_json::from_str(envelope.payload.as_str()) {
                            Ok(frame) => frame,
                            Err(error) => {
                                let _ = send_control_error_and_close(
                                    socket,
                                    ccp_runtime,
                                    route,
                                    "CCP_SCHEMA_INCOMPATIBLE",
                                    format!("control payload decode failed: {error}"),
                                )
                                .await;
                                return Err(());
                            }
                        };
                        if let Err(error) = validate_ccp_control_envelope(&envelope, &control) {
                            let _ = send_control_error_and_close(
                                socket,
                                ccp_runtime,
                                route,
                                "CCP_SCHEMA_INCOMPATIBLE",
                                error,
                            )
                            .await;
                            return Err(());
                        }
                        return Ok(control);
                    }
                }
            }
        }
    }
}

async fn send_session_disconnect_signal(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
) {
    let directive = session_disconnect_goaway();
    send_link_goaway_and_close(socket, wire_mode, ccp_runtime, route, &directive).await;
}

async fn send_link_goaway_and_close(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    directive: &LinkGoAwayDirective,
) {
    if wire_mode == RealtimeWebsocketMode::CcpJson {
        let frame = ControlFrame::GoAway(directive.frame.clone());
        if ccp_runtime
            .send_control_frame(socket, route, &frame)
            .await
            .is_err()
        {
            return;
        }
    }
    let _ = socket
        .send(session_disconnect_close_message(directive))
        .await;
}

fn session_disconnect_close_message(directive: &LinkGoAwayDirective) -> Message {
    Message::Close(Some(CloseFrame {
        code: directive.close_code,
        reason: Utf8Bytes::from_static(directive.close_reason),
    }))
}

#[derive(Debug)]
enum BufferedPushDrainError {
    Runtime(RealtimeRuntimeError),
    Fence(&'static str),
    Send,
}

struct BufferedPushDrainDriver<'a> {
    socket: &'a mut WebSocket,
    runtime: &'a RealtimeDeliveryRuntime,
    route_owner: &'a dyn RealtimeRouteOwner,
    auth: &'a AppContext,
    tenant_id: &'a str,
    organization_id: &'a str,
    principal_id: &'a str,
    principal_kind: &'a str,
    device_id: &'a str,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &'a CcpWebsocketRuntime,
    route: &'a CcpRoute,
}

impl LinkBufferedPushDrainDriver for BufferedPushDrainDriver<'_> {
    type Window = RealtimeEventWindow;
    type Error = BufferedPushDrainError;

    async fn fetch_window(
        &mut self,
        after_seq: u64,
        limit: usize,
    ) -> Result<LinkBufferedPushFetchedWindow<Self::Window>, Self::Error> {
        self.ensure_current_route_session().await?;
        // `list_events_for_principal_kind` performs blocking Postgres IO; run
        // it on the blocking pool so the async worker stays free.
        let runtime = self.runtime.clone();
        let tenant_id = self.tenant_id.to_owned();
        let organization_id = self.organization_id.to_owned();
        let principal_id = self.principal_id.to_owned();
        let principal_kind = self.principal_kind.to_owned();
        let device_id = self.device_id.to_owned();
        let window = match tokio::task::spawn_blocking(move || {
            runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                tenant_id: tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                principal_id: principal_id.as_str(),
                principal_kind: principal_kind.as_str(),
                device_id: device_id.as_str(),
                after_seq,
                limit,
            })
        })
        .await
        {
            Ok(Ok(window)) => window,
            Ok(Err(error)) => return Err(BufferedPushDrainError::Runtime(error)),
            Err(join_error) => {
                return Err(BufferedPushDrainError::Runtime(RealtimeRuntimeError {
                    code: "list_events_failed",
                    message: format!("list_events blocking task failed: {join_error}"),
                }));
            }
        };
        let next_after_seq = window.next_after_seq;
        let is_empty = window.items.is_empty();
        Ok(LinkBufferedPushFetchedWindow {
            window,
            next_after_seq,
            is_empty,
        })
    }

    async fn send_window(&mut self, window: Self::Window) -> Result<(), Self::Error> {
        self.ensure_current_route_session().await?;
        send_business_payload(
            self.socket,
            self.wire_mode,
            self.ccp_runtime,
            self.route,
            "evt",
            "cc.realtime.event.window.v1",
            json!({
                "type": "event.window",
                "reason": "push",
                "window": window
            }),
        )
        .await
        .map_err(|_| BufferedPushDrainError::Send)
    }
}

impl BufferedPushDrainDriver<'_> {
    // Takes `&mut self` (not `&self`) so the returned future is `Send`:
    // `BufferedPushDrainDriver` is `Send` but not `Sync` (it holds a
    // `&mut WebSocket`, and axum's `WebSocket` is not `Sync`). A shared
    // `&self` borrow held across `.await` would require `Sync`; a mutable
    // borrow only requires `Send`.
    async fn ensure_current_route_session(&mut self) -> Result<(), BufferedPushDrainError> {
        // `ensure_active_client_route_current_session` may perform blocking
        // Redis/Postgres IO via `route_store.lookup`; run it on the blocking
        // pool so the async worker stays free.
        let blocking_owner = self.route_owner.boxed_clone();
        let blocking_auth = self.auth.clone();
        let blocking_device_id = self.device_id.to_string();
        match tokio::task::spawn_blocking(move || {
            blocking_owner
                .ensure_active_client_route_current_session(&blocking_auth, &blocking_device_id)
        })
        .await
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(error)) => Err(BufferedPushDrainError::Fence(error.code)),
            Err(join_error) => {
                tracing::error!(
                    target: "sdkwork.im.session_gateway",
                    error = %join_error,
                    "route session blocking task panicked (BufferedPushDrainDriver)"
                );
                Err(BufferedPushDrainError::Fence("route_session_check_failed"))
            }
        }
    }
}

// The websocket message loop is a boundary adapter that needs the full runtime,
// queue, transport, and routing context visible while decoding client frames.
#[allow(clippy::too_many_arguments)]
async fn handle_client_message(
    socket: &mut WebSocket,
    runtime: &RealtimeDeliveryRuntime,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    message: Message,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    frame_rate_limiter: &crate::websocket_frame_rate_limit::WebsocketFrameRateLimiter,
) -> bool {
    match message {
        Message::Text(_) | Message::Binary(_) => {
            let principal_key = format!("{tenant_id}:{principal_kind}:{principal_id}");
            // `check_frame` may perform blocking Redis IO; run it on the
            // blocking pool so the async worker stays free.
            let rate_limiter = frame_rate_limiter.clone();
            let rate_principal_key = principal_key.clone();
            if let Err(error) = match tokio::task::spawn_blocking(move || {
                rate_limiter.check_frame(rate_principal_key.as_str())
            })
            .await
            {
                Ok(Ok(())) => Ok(()),
                Ok(Err(error)) => Err(error),
                Err(join_error) => Err(crate::ApiError {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    code: "websocket_frame_rate_limiter_unavailable",
                    message: format!(
                        "websocket frame rate limiter blocking task failed: {join_error}"
                    ),
                }),
            } {
                let runtime_error = RealtimeRuntimeError {
                    code: error.code,
                    message: error.message,
                };
                let _ =
                    send_runtime_error(socket, wire_mode, ccp_runtime, route, &runtime_error).await;
                return false;
            }
            let decoded = match decode_client_frame(message, wire_mode, ccp_runtime) {
                Ok(frame) => frame,
                Err(error) => {
                    let _ = send_business_error(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "invalid_frame",
                        error.message,
                    )
                    .await;
                    return true;
                }
            };
            let DecodedClientFrame::Business(frame) = decoded else {
                return true;
            };
            if let Err(error) = validate_client_frame_type(&frame) {
                let _ = send_runtime_error(socket, wire_mode, ccp_runtime, route, &error).await;
                return true;
            }
            if !ensure_current_route_session_for_request_or_close(
                socket,
                route_owner,
                auth,
                device_id,
                wire_mode,
                ccp_runtime,
                route,
            )
            .await
            {
                return false;
            }

            match frame.frame_type.as_str() {
                "subscriptions.sync" => {
                    // `sync_subscriptions_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let sync_runtime = runtime.clone();
                    let sync_tenant = tenant_id.to_owned();
                    let sync_org = organization_id.to_owned();
                    let sync_principal = principal_id.to_owned();
                    let sync_kind = principal_kind.to_owned();
                    let sync_device = device_id.to_owned();
                    let sync_items = frame.items.clone();
                    let snapshot = match tokio::task::spawn_blocking(move || {
                        sync_runtime.sync_subscriptions_for_principal_kind(
                            sync_tenant.as_str(),
                            sync_org.as_str(),
                            sync_principal.as_str(),
                            sync_kind.as_str(),
                            sync_device.as_str(),
                            sync_items,
                        )
                    })
                    .await
                    {
                        Ok(Ok(snapshot)) => snapshot,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "sync_subscriptions blocking task panicked"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "sync_subscriptions_failed",
                                    message: format!(
                                        "sync_subscriptions blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    let _ = send_business_payload(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "evt",
                        "cc.realtime.subscriptions.synced.v1",
                        json!({
                            "type": "subscriptions.synced",
                            "snapshot": snapshot
                        }),
                    )
                    .await;
                    true
                }
                "events.pull" => {
                    let limit = frame.limit.unwrap_or(100);
                    if limit == 0 {
                        let _ = send_business_error(
                            socket,
                            wire_mode,
                            ccp_runtime,
                            route,
                            "limit_invalid",
                            "limit must be greater than 0",
                        )
                        .await;
                        return true;
                    }

                    // `window_checkpoint_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let checkpoint_runtime = runtime.clone();
                    let checkpoint_tenant = tenant_id.to_owned();
                    let checkpoint_org = organization_id.to_owned();
                    let checkpoint_principal = principal_id.to_owned();
                    let checkpoint_kind = principal_kind.to_owned();
                    let checkpoint_device = device_id.to_owned();
                    let latest_realtime_seq = match tokio::task::spawn_blocking(move || {
                        checkpoint_runtime.window_checkpoint_for_principal_kind(
                            checkpoint_tenant.as_str(),
                            checkpoint_org.as_str(),
                            checkpoint_principal.as_str(),
                            checkpoint_kind.as_str(),
                            checkpoint_device.as_str(),
                        )
                    })
                    .await
                    {
                        Ok(Ok(checkpoint)) => checkpoint.latest_realtime_seq,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "window_checkpoint blocking task panicked (events.pull)"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "window_checkpoint_failed",
                                    message: format!(
                                        "window_checkpoint blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    let pull_plan =
                        outbound_queue.plan_pull(frame.after_seq, limit, latest_realtime_seq);
                    // `list_events_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let list_runtime = runtime.clone();
                    let list_tenant = tenant_id.to_owned();
                    let list_org = organization_id.to_owned();
                    let list_principal = principal_id.to_owned();
                    let list_kind = principal_kind.to_owned();
                    let list_device = device_id.to_owned();
                    let list_after_seq = pull_plan.after_seq;
                    let list_batch_limit = pull_plan.batch.limit;
                    let window = match tokio::task::spawn_blocking(move || {
                        list_runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                            tenant_id: list_tenant.as_str(),
                            organization_id: list_org.as_str(),
                            principal_id: list_principal.as_str(),
                            principal_kind: list_kind.as_str(),
                            device_id: list_device.as_str(),
                            after_seq: list_after_seq,
                            limit: list_batch_limit,
                        })
                    })
                    .await
                    {
                        Ok(Ok(window)) => window,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "list_events blocking task panicked (events.pull)"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "list_events_failed",
                                    message: format!(
                                        "list_events blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    let next_after_seq = window.next_after_seq;
                    if send_business_payload(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "evt",
                        "cc.realtime.event.window.v1",
                        json!({
                            "type": "event.window",
                            "reason": "pull",
                            "window": window
                        }),
                    )
                    .await
                    .is_err()
                    {
                        return false;
                    }
                    let recovery_plan =
                        outbound_queue.record_window_sent(pull_plan.after_seq, next_after_seq);
                    if !drain_runtime_owned_buffered_push(
                        socket,
                        runtime,
                        route_owner,
                        auth,
                        tenant_id,
                        organization_id,
                        principal_id,
                        principal_kind,
                        device_id,
                        outbound_queue,
                        recovery_plan,
                        wire_mode,
                        ccp_runtime,
                        route,
                    )
                    .await
                    {
                        return false;
                    }
                    true
                }
                "events.nack" => {
                    let Some(nack_through_seq) = frame.nack_through_seq.or(frame.after_seq) else {
                        let _ = send_business_error(
                            socket,
                            wire_mode,
                            ccp_runtime,
                            route,
                            "nack_through_seq_missing",
                            "nackThroughSeq or afterSeq is required",
                        )
                        .await;
                        return true;
                    };
                    let limit = frame.limit.unwrap_or(100);
                    if limit == 0 {
                        let _ = send_business_error(
                            socket,
                            wire_mode,
                            ccp_runtime,
                            route,
                            "limit_invalid",
                            "limit must be greater than 0",
                        )
                        .await;
                        return true;
                    }

                    // `window_checkpoint_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let checkpoint_runtime = runtime.clone();
                    let checkpoint_tenant = tenant_id.to_owned();
                    let checkpoint_org = organization_id.to_owned();
                    let checkpoint_principal = principal_id.to_owned();
                    let checkpoint_kind = principal_kind.to_owned();
                    let checkpoint_device = device_id.to_owned();
                    let latest_realtime_seq = match tokio::task::spawn_blocking(move || {
                        checkpoint_runtime.window_checkpoint_for_principal_kind(
                            checkpoint_tenant.as_str(),
                            checkpoint_org.as_str(),
                            checkpoint_principal.as_str(),
                            checkpoint_kind.as_str(),
                            checkpoint_device.as_str(),
                        )
                    })
                    .await
                    {
                        Ok(Ok(checkpoint)) => checkpoint.latest_realtime_seq,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "window_checkpoint blocking task panicked (events.nack)"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "window_checkpoint_failed",
                                    message: format!(
                                        "window_checkpoint blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    let nack_plan = outbound_queue.plan_nack_replay(
                        nack_through_seq,
                        limit,
                        latest_realtime_seq,
                    );
                    // `list_events_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let list_runtime = runtime.clone();
                    let list_tenant = tenant_id.to_owned();
                    let list_org = organization_id.to_owned();
                    let list_principal = principal_id.to_owned();
                    let list_kind = principal_kind.to_owned();
                    let list_device = device_id.to_owned();
                    let list_after_seq = nack_plan.after_seq;
                    let list_batch_limit = nack_plan.batch.limit;
                    let window = match tokio::task::spawn_blocking(move || {
                        list_runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                            tenant_id: list_tenant.as_str(),
                            organization_id: list_org.as_str(),
                            principal_id: list_principal.as_str(),
                            principal_kind: list_kind.as_str(),
                            device_id: list_device.as_str(),
                            after_seq: list_after_seq,
                            limit: list_batch_limit,
                        })
                    })
                    .await
                    {
                        Ok(Ok(window)) => window,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "list_events blocking task panicked (events.nack)"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "list_events_failed",
                                    message: format!(
                                        "list_events blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    let next_after_seq = window.next_after_seq;
                    if send_business_payload(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "evt",
                        "cc.realtime.event.window.v1",
                        json!({
                            "type": "event.window",
                            "reason": "nack",
                            "window": window
                        }),
                    )
                    .await
                    .is_err()
                    {
                        return false;
                    }
                    let recovery_plan =
                        outbound_queue.record_window_sent(nack_plan.after_seq, next_after_seq);
                    if !drain_runtime_owned_buffered_push(
                        socket,
                        runtime,
                        route_owner,
                        auth,
                        tenant_id,
                        organization_id,
                        principal_id,
                        principal_kind,
                        device_id,
                        outbound_queue,
                        recovery_plan,
                        wire_mode,
                        ccp_runtime,
                        route,
                    )
                    .await
                    {
                        return false;
                    }
                    true
                }
                "events.ack" => {
                    let Some(acked_seq) = frame.acked_seq else {
                        let _ = send_business_error(
                            socket,
                            wire_mode,
                            ccp_runtime,
                            route,
                            "acked_seq_missing",
                            "ackedSeq is required",
                        )
                        .await;
                        return true;
                    };

                    // `ack_events_for_principal_kind` performs blocking
                    // Postgres IO; run it on the blocking pool so the async
                    // worker stays free.
                    let ack_runtime = runtime.clone();
                    let ack_tenant = tenant_id.to_owned();
                    let ack_org = organization_id.to_owned();
                    let ack_principal = principal_id.to_owned();
                    let ack_kind = principal_kind.to_owned();
                    let ack_device = device_id.to_owned();
                    let ack_seq = acked_seq;
                    let ack = match tokio::task::spawn_blocking(move || {
                        ack_runtime.ack_events_for_principal_kind(
                            ack_tenant.as_str(),
                            ack_org.as_str(),
                            ack_principal.as_str(),
                            ack_kind.as_str(),
                            ack_device.as_str(),
                            ack_seq,
                        )
                    })
                    .await
                    {
                        Ok(Ok(ack)) => ack,
                        Ok(Err(error)) => {
                            let _ =
                                send_runtime_error(socket, wire_mode, ccp_runtime, route, &error)
                                    .await;
                            return true;
                        }
                        Err(join_error) => {
                            tracing::error!(
                                target: "sdkwork.im.session_gateway",
                                tenant_id = %tenant_id,
                                principal_id = %principal_id,
                                device_id = %device_id,
                                error = %join_error,
                                "ack_events blocking task panicked"
                            );
                            let _ = send_runtime_error(
                                socket,
                                wire_mode,
                                ccp_runtime,
                                route,
                                &RealtimeRuntimeError {
                                    code: "ack_events_failed",
                                    message: format!(
                                        "ack_events blocking task failed: {join_error}"
                                    ),
                                },
                            )
                            .await;
                            return true;
                        }
                    };
                    outbound_queue.record_client_ack(ack.acked_through_seq);
                    let _ = send_business_payload(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "ack",
                        "cc.realtime.events.acked.v1",
                        json!({
                            "type": "events.acked",
                            "ack": ack
                        }),
                    )
                    .await;
                    true
                }
                _ => {
                    let _ = send_business_error(
                        socket,
                        wire_mode,
                        ccp_runtime,
                        route,
                        "frame_type_unsupported",
                        format!("unsupported frame type: {}", frame.frame_type),
                    )
                    .await;
                    true
                }
            }
        }
        Message::Ping(payload) => {
            let principal_key = format!("{tenant_id}:{principal_kind}:{principal_id}");
            // `check_frame` may perform blocking Redis IO; run it on the
            // blocking pool so the async worker stays free.
            let rate_limiter = frame_rate_limiter.clone();
            let rate_principal_key = principal_key.clone();
            let rate_result = tokio::task::spawn_blocking(move || {
                rate_limiter.check_frame(rate_principal_key.as_str())
            })
            .await;
            match rate_result {
                Ok(Ok(())) => socket.send(Message::Pong(payload)).await.is_ok(),
                Ok(Err(_)) => false,
                Err(join_error) => {
                    tracing::error!(
                        target: "sdkwork.im.session_gateway",
                        tenant_id = %tenant_id,
                        principal_id = %principal_id,
                        device_id = %device_id,
                        error = %join_error,
                        "frame_rate_limiter blocking task panicked (Ping branch)"
                    );
                    false
                }
            }
        }
        Message::Pong(_) => true,
        Message::Close(frame) => {
            let _ = socket.send(Message::Close(frame)).await;
            false
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn drain_runtime_owned_buffered_push(
    socket: &mut WebSocket,
    runtime: &RealtimeDeliveryRuntime,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    push_plan: Option<LinkBufferedPushPlan>,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
) -> bool {
    let mut driver = BufferedPushDrainDriver {
        socket,
        runtime,
        route_owner,
        auth,
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
        wire_mode,
        ccp_runtime,
        route,
    };

    match outbound_queue
        .drain_buffered_push_windows(push_plan, &mut driver)
        .await
    {
        Ok(LinkBufferedPushDrainStatus::Drained) | Ok(LinkBufferedPushDrainStatus::PullOnly) => {
            true
        }
        Ok(LinkBufferedPushDrainStatus::Disconnect(directive)) => {
            send_link_goaway_and_close(socket, wire_mode, ccp_runtime, route, &directive).await;
            false
        }
        Err(BufferedPushDrainError::Runtime(error)) => {
            let _ = send_runtime_error(socket, wire_mode, ccp_runtime, route, &error).await;
            false
        }
        Err(BufferedPushDrainError::Fence(code)) => {
            close_policy_with_reason(socket, code).await;
            false
        }
        Err(BufferedPushDrainError::Send) => false,
    }
}

fn decode_client_frame(
    message: Message,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
) -> Result<DecodedClientFrame, ClientFrameDecodeError> {
    match wire_mode {
        RealtimeWebsocketMode::LegacyJson => match message {
            Message::Text(text) => serde_json::from_str(text.as_str())
                .map(DecodedClientFrame::Business)
                .map_err(|_| ClientFrameDecodeError::new("frame must be valid json")),
            Message::Binary(_) => Err(ClientFrameDecodeError::new(
                "binary websocket frames are not supported",
            )),
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) => Err(
                ClientFrameDecodeError::new("unexpected websocket control message"),
            ),
        },
        RealtimeWebsocketMode::CcpJson => {
            let envelope = ccp_runtime
                .decode_message(message)
                .map_err(ClientFrameDecodeError::new)?;
            if envelope.kind == "control"
                && let Ok(control) = serde_json::from_str::<ControlFrame>(envelope.payload.as_str())
            {
                if envelope.route.is_some() {
                    return Err(ClientFrameDecodeError::new(
                        ccp_client_route_metadata_error(),
                    ));
                }
                validate_ccp_control_envelope(&envelope, &control)
                    .map_err(ClientFrameDecodeError::new)?;
                return match control {
                    ControlFrame::Heartbeat(_) => Ok(DecodedClientFrame::Heartbeat),
                    other => Err(ClientFrameDecodeError::new(format!(
                        "unexpected ccp control frame after handshake: {}",
                        other.frame_type()
                    ))),
                };
            }
            let frame: ClientFrameEnvelope = serde_json::from_str(envelope.payload.as_str())
                .map_err(|error| {
                    ClientFrameDecodeError::new(format!("ccp payload must be valid json: {error}"))
                })?;
            if envelope.route.is_some() {
                return Err(ClientFrameDecodeError::new(
                    ccp_client_route_metadata_error(),
                ));
            }
            validate_ccp_client_business_envelope(&envelope, &frame)?;
            Ok(DecodedClientFrame::Business(frame))
        }
    }
}

async fn send_initial_runtime_error(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    error: &RealtimeRuntimeError,
) -> Result<(), axum::Error> {
    match wire_mode {
        RealtimeWebsocketMode::LegacyJson => {
            send_json(
                socket,
                json!({
                    "type": "error",
                    "traceId": new_server_trace_id(),
                    "code": error.code,
                    "message": error.message
                }),
            )
            .await
        }
        RealtimeWebsocketMode::CcpJson => {
            send_control_error(
                socket,
                ccp_runtime,
                route,
                error.code,
                error.message.as_str(),
            )
            .await
        }
    }
}

async fn send_control_error(
    socket: &mut WebSocket,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<(), axum::Error> {
    let frame = ControlFrame::Error(ErrorFrame {
        code: code.into(),
        message: message.into(),
        retryable: false,
    });
    ccp_runtime.send_control_frame(socket, route, &frame).await
}

async fn send_control_error_and_close(
    socket: &mut WebSocket,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<(), axum::Error> {
    let send_result = send_control_error(socket, ccp_runtime, route, code, message).await;
    let close_result = socket
        .send(Message::Close(Some(CloseFrame {
            code: close_code::POLICY,
            reason: Utf8Bytes::from_static(CCP_PROTOCOL_ERROR_CLOSE_REASON),
        })))
        .await;
    send_result?;
    close_result
}

async fn ensure_current_route_session_or_close(
    socket: &mut WebSocket,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    device_id: &str,
) -> bool {
    // `ensure_active_client_route_current_session` may perform blocking
    // Redis/Postgres IO via `route_store.lookup`; run it on the blocking
    // pool so the async worker stays free.
    let blocking_owner = route_owner.boxed_clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.to_string();
    let result = tokio::task::spawn_blocking(move || {
        blocking_owner
            .ensure_active_client_route_current_session(&blocking_auth, &blocking_device_id)
    })
    .await;
    match result {
        Ok(Ok(())) => true,
        Ok(Err(error)) => {
            close_policy_with_reason(socket, error.code).await;
            false
        }
        Err(join_error) => {
            tracing::error!(
                target: "sdkwork.im.session_gateway",
                error = %join_error,
                "route session blocking task panicked (ensure_current_route_session_or_close)"
            );
            close_policy_with_reason(socket, "route_session_check_failed").await;
            false
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn ensure_current_route_session_for_request_or_close(
    socket: &mut WebSocket,
    route_owner: &dyn RealtimeRouteOwner,
    auth: &AppContext,
    device_id: &str,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
) -> bool {
    // `ensure_active_client_route_current_session` may perform blocking
    // Redis/Postgres IO via `route_store.lookup`; run it on the blocking
    // pool so the async worker stays free.
    let blocking_owner = route_owner.boxed_clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.to_string();
    let result = tokio::task::spawn_blocking(move || {
        blocking_owner
            .ensure_active_client_route_current_session(&blocking_auth, &blocking_device_id)
    })
    .await;
    match result {
        Ok(Ok(())) => true,
        Ok(Err(error)) => {
            let _ = send_business_error(
                socket,
                wire_mode,
                ccp_runtime,
                route,
                error.code,
                error.message.clone(),
            )
            .await;
            close_policy_with_reason(socket, error.code).await;
            false
        }
        Err(join_error) => {
            tracing::error!(
                target: "sdkwork.im.session_gateway",
                error = %join_error,
                "route session blocking task panicked (ensure_current_route_session_for_request_or_close)"
            );
            let _ = send_business_error(
                socket,
                wire_mode,
                ccp_runtime,
                route,
                "route_session_check_failed",
                format!("route session blocking task failed: {join_error}"),
            )
            .await;
            close_policy_with_reason(socket, "route_session_check_failed").await;
            false
        }
    }
}

async fn close_policy_with_reason(socket: &mut WebSocket, reason: &'static str) {
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: close_code::POLICY,
            reason: Utf8Bytes::from_static(reason),
        })))
        .await;
}

async fn send_business_error(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<(), axum::Error> {
    let code = code.into();
    let message = message.into();
    send_business_payload(
        socket,
        wire_mode,
        ccp_runtime,
        route,
        "error",
        "cc.realtime.error.v1",
        json!({
            "type": "error",
            "code": code,
            "message": message
        }),
    )
    .await
}

async fn send_runtime_error(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    error: &RealtimeRuntimeError,
) -> Result<(), axum::Error> {
    send_business_error(
        socket,
        wire_mode,
        ccp_runtime,
        route,
        error.code,
        error.message.as_str(),
    )
    .await
}

async fn send_business_payload(
    socket: &mut WebSocket,
    wire_mode: RealtimeWebsocketMode,
    ccp_runtime: &CcpWebsocketRuntime,
    route: &CcpRoute,
    kind: &str,
    schema: &str,
    payload: Value,
) -> Result<(), axum::Error> {
    let trace_id = new_server_trace_id();
    let payload = payload_with_trace_id(payload, trace_id.as_str());
    match wire_mode {
        RealtimeWebsocketMode::LegacyJson => send_json(socket, payload).await,
        RealtimeWebsocketMode::CcpJson => {
            ccp_runtime
                .send_business_payload(socket, route, kind, schema, trace_id, payload)
                .await
        }
    }
}

fn payload_with_trace_id(mut payload: Value, trace_id: &str) -> Value {
    if let Value::Object(fields) = &mut payload {
        fields.insert("traceId".to_owned(), Value::String(trace_id.to_owned()));
    }
    payload
}

fn ccp_protocol_version() -> ProtocolVersion {
    ProtocolVersion::new("ccp", 1, 0)
}

fn control_schema(frame: &ControlFrame) -> &'static str {
    match frame {
        ControlFrame::Hello(_) => "cc.control.hello.v1",
        ControlFrame::HelloAck(_) => "cc.control.hello_ack.v1",
        ControlFrame::AuthBind(_) => "cc.control.auth_bind.v1",
        ControlFrame::AuthOk(_) => "cc.control.auth_ok.v1",
        ControlFrame::SessionResume(_) => "cc.control.session_resume.v1",
        ControlFrame::SessionResumed(_) => "cc.control.session_resumed.v1",
        ControlFrame::Heartbeat(_) => "cc.control.heartbeat.v1",
        ControlFrame::GoAway(_) => "cc.control.goaway.v1",
        ControlFrame::Error(_) => "cc.control.error.v1",
    }
}

fn build_link_session(auth: &AppContext, device_id: &str) -> LinkSession {
    LinkSession::new(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id,
        auth.session_id.as_deref(),
        OutboundQueuePolicy::realtime_default(),
    )
}

fn activate_link_session(session: &mut LinkSession, checkpoint: &RealtimeWindowCheckpoint) {
    session.activate(ResumeWindow::new(
        checkpoint.latest_realtime_seq,
        checkpoint.acked_through_seq,
    ));
}

async fn send_json(socket: &mut WebSocket, value: Value) -> Result<(), axum::Error> {
    socket.send(Message::Text(value.to_string().into())).await
}

#[cfg(test)]
#[path = "websocket_tests.rs"]
mod tests;
