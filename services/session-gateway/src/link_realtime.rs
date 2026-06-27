use std::sync::Arc;
use std::time::Duration;

use im_app_context::AppContext;
use im_domain_core::realtime::RealtimeEventWindow;
use sdkwork_im_ccp_control::{ControlFrame, ErrorFrame, HeartbeatFrame};
use sdkwork_im_ccp_core::{CcpEnvelope, CcpRoute, TransportBinding};
use sdkwork_im_runtime_link::{
    link_idle_timeout_goaway, session_disconnect_goaway, LinkBufferedPushDrainDriver,
    LinkBufferedPushDrainStatus, LinkBufferedPushFetchedWindow, LinkBufferedPushPlan,
    LinkGoAwayDirective, LinkOutboundQueueState, LinkSession, OutboundQueuePolicy, ResumeWindow,
};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::client_route_registration::ClientRouteRegistration;
use crate::link_business_contract::{validate_link_client_business_envelope, LinkClientBusinessFrame};
use crate::link_framing::{
    read_framed_envelope, write_framed_bytes, FramedStreamCcpCodec,
};
use crate::realtime::{RealtimeDeliveryRuntime, RealtimeRuntimeError, RealtimeSubscriptionItemInput, RealtimeWindowCheckpoint};
use crate::ApiError;

const REALTIME_HEARTBEAT_INTERVAL_SECS_ENV: &str = "SDKWORK_IM_REALTIME_HEARTBEAT_INTERVAL_SECS";
const REALTIME_HEARTBEAT_INTERVAL_DEFAULT_SECS: u64 = 30;
const REALTIME_IDLE_TIMEOUT_SECS_ENV: &str = "SDKWORK_IM_REALTIME_IDLE_TIMEOUT_SECS";
const REALTIME_IDLE_TIMEOUT_DEFAULT_SECS: u64 = 90;

fn resolve_heartbeat_interval() -> Duration {
    let secs = std::env::var(REALTIME_HEARTBEAT_INTERVAL_SECS_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(REALTIME_HEARTBEAT_INTERVAL_DEFAULT_SECS)
        .max(1);
    Duration::from_secs(secs)
}

fn resolve_idle_timeout() -> Duration {
    let secs = std::env::var(REALTIME_IDLE_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(REALTIME_IDLE_TIMEOUT_DEFAULT_SECS)
        .max(1);
    Duration::from_secs(secs)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamClientFrameEnvelope {
    #[serde(rename = "type")]
    frame_type: String,
    request_id: Option<String>,
    #[serde(default)]
    items: Vec<RealtimeSubscriptionItemInput>,
    after_seq: Option<u64>,
    limit: Option<usize>,
    acked_seq: Option<u64>,
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

fn map_api_error(error: ApiError) -> String {
    format!("{}: {}", error.code, error.message)
}

pub(crate) async fn serve_realtime_framed_session<R, W>(
    mut reader: R,
    mut writer: W,
    transport: TransportBinding,
    auth: AppContext,
    device_id: String,
    resume_after_seq: Option<u64>,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: ClientRouteRegistration,
) -> Result<(), String>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let tenant_id = auth.tenant_id.clone();
    let organization_id = auth.organization_id.clone();
    let principal_id = auth.actor_id.clone();
    let principal_kind = auth.actor_kind.clone();
    let authority = auth.ccp_authority();
    let route = CcpRoute::for_principal(
        tenant_id.clone(),
        principal_id.clone(),
        Some(device_id.clone()),
    );
    let ccp = FramedStreamCcpCodec::new(transport.clone());
    let expected_binding = transport.clone();

    if !ensure_framed_route_session(
        &route_owner,
        &auth,
        device_id.as_str(),
        &mut writer,
        &ccp,
        &route,
    )
    .await
    {
        return Ok(());
    }

    let mut route_epoch_receiver = route_owner
        .subscribe_active_client_route_epoch(&auth, device_id.as_str())
        .map_err(map_api_error)?;

    // The three setup calls below all perform blocking Postgres IO
    // (load checkpoint, load subscriptions, load window). Batch them into a
    // single spawn_blocking so only one blocking-thread hop is needed and
    // the async worker is free during the round-trips.
    let setup_runtime = Arc::clone(&runtime);
    let setup_tenant = tenant_id.clone();
    let setup_org = organization_id.clone();
    let setup_principal = principal_id.clone();
    let setup_kind = principal_kind.clone();
    let setup_device = device_id.clone();
    let (checkpoint, disconnect_generation) = tokio::task::spawn_blocking(
        move || -> Result<(RealtimeWindowCheckpoint, u64), String> {
            setup_runtime
                .ensure_client_route_state_for_principal_kind(
                    setup_tenant.as_str(),
                    setup_org.as_str(),
                    setup_principal.as_str(),
                    setup_kind.as_str(),
                    setup_device.as_str(),
                )
                .map_err(|error| error.message)?;
            let checkpoint = setup_runtime
                .window_checkpoint_for_principal_kind(
                    setup_tenant.as_str(),
                    setup_org.as_str(),
                    setup_principal.as_str(),
                    setup_kind.as_str(),
                    setup_device.as_str(),
                )
                .map_err(|error| error.message)?;
            let disconnect_generation = setup_runtime
                .disconnect_generation_for_principal_kind(
                    setup_tenant.as_str(),
                    setup_org.as_str(),
                    setup_principal.as_str(),
                    setup_kind.as_str(),
                    setup_device.as_str(),
                )
                .map_err(|error| error.message)?;
            Ok((checkpoint, disconnect_generation))
        },
    )
    .await
    .map_err(|join_error| format!("session setup blocking task failed: {join_error}"))??;

    let mut link_session = build_link_session(&auth, device_id.as_str());
    link_session.mark_authenticated();
    activate_link_session(&mut link_session, &checkpoint);
    let resume_after_seq = resume_after_seq.unwrap_or_else(|| {
        checkpoint
            .acked_through_seq
            .max(checkpoint.trimmed_through_seq)
    });
    let mut outbound_queue =
        link_session.start_outbound_queue(resume_after_seq, checkpoint.latest_realtime_seq);

    let mut seq_receiver = runtime
        .subscribe_client_route_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    let mut disconnect_receiver = runtime
        .subscribe_disconnect_signal_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    send_framed_realtime_connected(
        &mut writer,
        &ccp,
        &route,
        &auth,
        device_id.as_str(),
        &checkpoint,
        &authority.sender.sender_id(),
    )
    .await?;

    // Server-initiated heartbeat keeps the link alive through proxies/LBs
    // and surfaces silent peer disconnects via write failure. The idle
    // timeout tears down sessions that stop making progress so server
    // resources (route slots, subscription state) are reclaimed.
    let heartbeat_interval = resolve_heartbeat_interval();
    let idle_timeout = resolve_idle_timeout();
    let mut heartbeat_timer = tokio::time::interval(heartbeat_interval);
    // The first tick of tokio::time::interval completes immediately; consume
    // it so the first outbound heartbeat fires after `heartbeat_interval`
    // rather than right after `realtime.connected`.
    heartbeat_timer.tick().await;
    let mut last_activity = tokio::time::Instant::now();
    let mut heartbeat_seq: u64 = 0;

    if let Some(catchup_plan) = outbound_queue.plan_catchup() {
        if !ensure_framed_route_session(
            &route_owner,
            &auth,
            device_id.as_str(),
            &mut writer,
            &ccp,
            &route,
        )
        .await
        {
            return Ok(());
        }
        // list_events_for_principal_kind performs blocking Postgres IO; run
        // it on the blocking pool so the async worker stays free.
        let catchup_runtime = Arc::clone(&runtime);
        let catchup_tenant = tenant_id.clone();
        let catchup_org = organization_id.clone();
        let catchup_principal = principal_id.clone();
        let catchup_kind = principal_kind.clone();
        let catchup_device = device_id.clone();
        let catchup_after_seq = catchup_plan.after_seq;
        let catchup_batch_limit = catchup_plan.batch.limit;
        let catchup = tokio::task::spawn_blocking(move || {
            catchup_runtime.list_events_for_principal_kind(
                catchup_tenant.as_str(),
                catchup_org.as_str(),
                catchup_principal.as_str(),
                catchup_kind.as_str(),
                catchup_device.as_str(),
                catchup_after_seq,
                catchup_batch_limit,
            )
        })
        .await
        .map_err(|join_error| format!("catchup blocking task failed: {join_error}"))?
        .map_err(|error| error.message)?;
        if !catchup.items.is_empty() {
            let next_after_seq = catchup.next_after_seq;
            send_framed_event_window(
                &mut writer,
                &ccp,
                &route,
                "catchup",
                catchup,
            )
            .await?;
            let _ = outbound_queue.record_window_sent(catchup_plan.after_seq, next_after_seq);
        }
    }

    loop {
        tokio::select! {
            route_epoch_changed = route_epoch_receiver.changed() => {
                if route_epoch_changed.is_err() {
                    break;
                }
                if !ensure_framed_route_session(
                    &route_owner,
                    &auth,
                    device_id.as_str(),
                    &mut writer,
                    &ccp,
                    &route,
                )
                .await
                {
                    break;
                }
            }
            changed = seq_receiver.changed() => {
                if changed.is_err() {
                    break;
                }
                let latest_realtime_seq = *seq_receiver.borrow_and_update();
                let push_plan = outbound_queue.observe_latest_realtime_seq(latest_realtime_seq);
                if !drain_framed_buffered_push(
                    &mut writer,
                    &runtime,
                    &route_owner,
                    &auth,
                    tenant_id.as_str(),
                    organization_id.as_str(),
                    principal_id.as_str(),
                    principal_kind.as_str(),
                    device_id.as_str(),
                    &mut outbound_queue,
                    push_plan,
                    &ccp,
                    &route,
                )
                .await
                {
                    break;
                }
            }
            disconnect_changed = disconnect_receiver.changed() => {
                if disconnect_changed.is_err() {
                    break;
                }
                // disconnect_generation_for_principal_kind performs blocking
                // Postgres IO; run it on the blocking pool.
                let disconnect_runtime = Arc::clone(&runtime);
                let disconnect_tenant = tenant_id.clone();
                let disconnect_org = organization_id.clone();
                let disconnect_principal = principal_id.clone();
                let disconnect_kind = principal_kind.clone();
                let disconnect_device = device_id.clone();
                let current = tokio::task::spawn_blocking(move || {
                    disconnect_runtime.disconnect_generation_for_principal_kind(
                        disconnect_tenant.as_str(),
                        disconnect_org.as_str(),
                        disconnect_principal.as_str(),
                        disconnect_kind.as_str(),
                        disconnect_device.as_str(),
                    )
                })
                .await
                .map_err(|join_error| format!("disconnect check blocking task failed: {join_error}"))?
                .map_err(|error| error.message)?;
                if current != disconnect_generation {
                    send_framed_session_disconnect(&mut writer, &ccp, &route).await?;
                    break;
                }
            }
            // Server-initiated heartbeat: periodically send a heartbeat
            // frame to keep the connection alive through proxies/LBs and
            // to detect silent peer disconnects via write failure. The
            // same tick enforces the idle timeout so sessions that stop
            // making progress are reclaimed.
            _ = heartbeat_timer.tick() => {
                heartbeat_seq = heartbeat_seq.saturating_add(1);
                if send_framed_heartbeat(
                    &mut writer,
                    &ccp,
                    &route,
                    Some(heartbeat_seq),
                )
                .await
                .is_err()
                {
                    break;
                }
                if last_activity.elapsed() >= idle_timeout {
                    let _ = send_framed_goaway_and_close(
                        &mut writer,
                        &ccp,
                        &route,
                        &link_idle_timeout_goaway(),
                    )
                    .await;
                    break;
                }
            }
            read_result = read_framed_envelope(&mut reader, transport.clone()) => {
                // Any inbound traffic (heartbeat, ack, pull, etc.) resets
                // the idle timer — the peer is still alive.
                last_activity = tokio::time::Instant::now();
                match read_result {
                    Ok(envelope) => {
                        if envelope.binding != expected_binding {
                            return Err("stream link received unexpected binding envelope".into());
                        }
                        if !handle_framed_client_envelope(
                            &mut writer,
                            &runtime,
                            &route_owner,
                            &auth,
                            tenant_id.as_str(),
                            organization_id.as_str(),
                            principal_id.as_str(),
                            principal_kind.as_str(),
                            device_id.as_str(),
                            &mut outbound_queue,
                            &envelope,
                            &ccp,
                            &route,
                        )
                        .await
                        {
                            break;
                        }
                    }
                    Err(error) => {
                        if error.contains("early eof") || error.contains("connection") {
                            break;
                        }
                        return Err(error);
                    }
                }
            }
        }
    }

    // Release the route on the blocking thread pool — the release performs
    // blocking Redis/Postgres IO and the async worker should not be held
    // during connection teardown.
    let cleanup_auth = auth.clone();
    let cleanup_device_id = device_id.clone();
    let _ = tokio::task::spawn_blocking(move || {
        route_owner.release_active_client_route_if_current_session(&cleanup_auth, cleanup_device_id.as_str());
    })
    .await;
    Ok(())
}

async fn ensure_framed_route_session<W>(
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    device_id: &str,
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    // The route session check performs a blocking Redis/Postgres lookup.
    // Run it on the blocking thread pool so the async worker can service
    // other connections while the route store responds.
    let blocking_owner = route_owner.clone();
    let blocking_auth = auth.clone();
    let blocking_device_id = device_id.to_string();
    let result = tokio::task::spawn_blocking(move || {
        blocking_owner.ensure_active_client_route_current_session(&blocking_auth, &blocking_device_id)
    })
    .await;

    match result {
        Ok(Ok(())) => true,
        Ok(Err(error)) => {
            let frame = ControlFrame::Error(ErrorFrame {
                code: error.code.into(),
                message: error.message,
                retryable: false,
            });
            let _ = write_framed_bytes(
                writer,
                ccp.encode_control(route, &frame).unwrap_or_default().as_slice(),
            )
            .await;
            false
        }
        Err(join_error) => {
            let frame = ControlFrame::Error(ErrorFrame {
                code: "link_blocking_join_failed".into(),
                message: join_error.to_string(),
                retryable: false,
            });
            let _ = write_framed_bytes(
                writer,
                ccp.encode_control(route, &frame).unwrap_or_default().as_slice(),
            )
            .await;
            false
        }
    }
}

async fn send_framed_heartbeat<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    sequence: Option<u64>,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let frame = ControlFrame::Heartbeat(HeartbeatFrame { sequence });
    write_framed_bytes(
        writer,
        ccp.encode_control(route, &frame)?.as_slice(),
    )
    .await
}

async fn send_framed_realtime_connected<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    auth: &AppContext,
    device_id: &str,
    checkpoint: &RealtimeWindowCheckpoint,
    sender_id: &str,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let authority = auth.ccp_authority();
    let bytes = ccp.encode_business(
        route,
        "evt",
        "cc.realtime.connected.v1",
        json!({
            "type": "realtime.connected",
            "tenantId": auth.tenant_id,
            "principalId": auth.actor_id,
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
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_event_window<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    reason: &str,
    window: RealtimeEventWindow,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "evt",
        "cc.realtime.event.window.v1",
        json!({
            "type": "event.window",
            "requestId": Value::Null,
            "reason": reason,
            "window": window
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_session_disconnect<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    send_framed_goaway_and_close(writer, ccp, route, &session_disconnect_goaway()).await
}

async fn send_framed_goaway_and_close<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    directive: &LinkGoAwayDirective,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    send_framed_goaway(writer, ccp, route, directive).await?;
    writer
        .shutdown()
        .await
        .map_err(|error| format!("stream shutdown failed: {error}"))
}

enum FramedPushDrainError {
    Runtime(RealtimeRuntimeError),
    Fence,
    Send,
    JoinFailed(String),
}

struct FramedPushDrainDriver<'a, W> {
    writer: &'a mut W,
    runtime: &'a Arc<RealtimeDeliveryRuntime>,
    route_owner: &'a ClientRouteRegistration,
    auth: &'a AppContext,
    tenant_id: &'a str,
    organization_id: &'a str,
    principal_id: &'a str,
    principal_kind: &'a str,
    device_id: &'a str,
    ccp: &'a FramedStreamCcpCodec,
    route: &'a CcpRoute,
}

impl<W> LinkBufferedPushDrainDriver for FramedPushDrainDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    type Window = RealtimeEventWindow;
    type Error = FramedPushDrainError;

    async fn fetch_window(
        &mut self,
        after_seq: u64,
        limit: usize,
    ) -> Result<LinkBufferedPushFetchedWindow<Self::Window>, Self::Error> {
        self.ensure_current_route_session().await?;
        // list_events_for_principal_kind performs blocking Postgres IO.
        // Clone the owned data and run it on the blocking pool so the
        // async worker stays free to service other connections.
        let runtime = Arc::clone(self.runtime);
        let tenant = self.tenant_id.to_string();
        let org = self.organization_id.to_string();
        let principal = self.principal_id.to_string();
        let kind = self.principal_kind.to_string();
        let device = self.device_id.to_string();
        let window = tokio::task::spawn_blocking(move || {
            runtime.list_events_for_principal_kind(
                tenant.as_str(),
                org.as_str(),
                principal.as_str(),
                kind.as_str(),
                device.as_str(),
                after_seq,
                limit,
            )
        })
        .await
        .map_err(|e| FramedPushDrainError::JoinFailed(e.to_string()))?
        .map_err(FramedPushDrainError::Runtime)?;
        Ok(LinkBufferedPushFetchedWindow {
            next_after_seq: window.next_after_seq,
            is_empty: window.items.is_empty(),
            window,
        })
    }

    async fn send_window(&mut self, window: Self::Window) -> Result<(), Self::Error> {
        self.ensure_current_route_session().await?;
        send_framed_event_window(self.writer, self.ccp, self.route, "push", window)
            .await
            .map_err(|_| FramedPushDrainError::Send)
    }
}

impl<W> FramedPushDrainDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    async fn ensure_current_route_session(&self) -> Result<(), FramedPushDrainError> {
        let route_owner = self.route_owner.clone();
        let auth = self.auth.clone();
        let device_id = self.device_id.to_string();
        tokio::task::spawn_blocking(move || {
            route_owner.ensure_active_client_route_current_session(&auth, &device_id)
        })
        .await
        .map_err(|e| FramedPushDrainError::JoinFailed(e.to_string()))?
        .map_err(|_| FramedPushDrainError::Fence)
    }
}

async fn drain_framed_buffered_push<W>(
    writer: &mut W,
    runtime: &Arc<RealtimeDeliveryRuntime>,
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    push_plan: Option<LinkBufferedPushPlan>,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    let mut driver = FramedPushDrainDriver {
        writer,
        runtime,
        route_owner,
        auth,
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
        ccp,
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
            let _ = send_framed_goaway_and_close(writer, ccp, route, &directive).await;
            false
        }
        Err(FramedPushDrainError::Runtime(error)) => {
            let _ = send_framed_runtime_error(writer, ccp, route, None, &error).await;
            false
        }
        Err(FramedPushDrainError::Fence) => false,
        Err(FramedPushDrainError::Send) => false,
        Err(FramedPushDrainError::JoinFailed(message)) => {
            let _ = send_framed_runtime_error(
                writer,
                ccp,
                route,
                None,
                &RealtimeRuntimeError {
                    code: "push_drain_blocking_join_failed",
                    message,
                },
            )
            .await;
            false
        }
    }
}

async fn send_framed_goaway<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    directive: &LinkGoAwayDirective,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let frame = ControlFrame::GoAway(directive.frame.clone());
    write_framed_bytes(
        writer,
        ccp.encode_control(route, &frame)?.as_slice(),
    )
    .await
}

async fn send_framed_runtime_error<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    request_id: Option<String>,
    error: &RealtimeRuntimeError,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "error",
        "cc.realtime.error.v1",
        json!({
            "type": "error",
            "requestId": request_id,
            "code": error.code,
            "message": error.message
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_business_error<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    request_id: Option<String>,
    code: &str,
    message: impl Into<String>,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "error",
        "cc.realtime.error.v1",
        json!({
            "type": "error",
            "requestId": request_id,
            "code": code,
            "message": message.into()
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn handle_framed_client_envelope<W>(
    writer: &mut W,
    runtime: &Arc<RealtimeDeliveryRuntime>,
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    envelope: &CcpEnvelope,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    if envelope.kind == "heartbeat" {
        return true;
    }
    if envelope.kind == "goaway" {
        return false;
    }
    if matches!(
        envelope.kind.as_str(),
        "hello" | "hello_ack" | "auth_bind" | "auth_ok" | "session_resume" | "session_resumed"
    ) {
        let _ = send_framed_business_error(
            writer,
            ccp,
            route,
            None,
            "frame_type_unsupported",
            format!("unexpected post-auth control frame: {}", envelope.kind),
        )
        .await;
        return true;
    }

    let frame = match serde_json::from_str::<StreamClientFrameEnvelope>(envelope.payload.as_str()) {
        Ok(frame) => frame,
        Err(_) => {
            let _ = send_framed_business_error(
                writer,
                ccp,
                route,
                None,
                "invalid_frame",
                "frame must be valid json",
            )
            .await;
            return true;
        }
    };

    if let Err(message) = validate_link_client_business_envelope(
        envelope,
        &LinkClientBusinessFrame {
            frame_type: frame.frame_type.clone(),
            request_id: frame.request_id.clone(),
        },
    ) {
        let _ = send_framed_business_error(
            writer,
            ccp,
            route,
            frame.request_id.clone(),
            "invalid_frame",
            message,
        )
        .await;
        return true;
    }

    if !ensure_framed_route_session(route_owner, auth, device_id, writer, ccp, route).await {
        return false;
    }

    match frame.frame_type.as_str() {
        "subscriptions.sync" => {
            // sync_subscriptions_for_principal_kind performs blocking
            // Postgres/Redis IO. Run it on the blocking pool so the async
            // worker stays free to service other connections.
            let blocking_runtime = Arc::clone(runtime);
            let blocking_tenant = tenant_id.to_string();
            let blocking_org = organization_id.to_string();
            let blocking_principal = principal_id.to_string();
            let blocking_kind = principal_kind.to_string();
            let blocking_device = device_id.to_string();
            let blocking_items = frame.items;
            let result = tokio::task::spawn_blocking(move || {
                blocking_runtime.sync_subscriptions_for_principal_kind(
                    blocking_tenant.as_str(),
                    blocking_org.as_str(),
                    blocking_principal.as_str(),
                    blocking_kind.as_str(),
                    blocking_device.as_str(),
                    blocking_items,
                )
            })
            .await;
            match result {
                Ok(Ok(snapshot)) => {
                    let bytes = ccp
                        .encode_business(
                            route,
                            "evt",
                            "cc.realtime.subscriptions.synced.v1",
                            json!({
                                "type": "subscriptions.synced",
                                "requestId": frame.request_id,
                                "snapshot": snapshot
                            }),
                        )
                        .unwrap_or_default();
                    write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                }
                Ok(Err(error)) => {
                    let _ = send_framed_runtime_error(writer, ccp, route, frame.request_id, &error)
                        .await;
                    true
                }
                Err(join_error) => {
                    let _ = send_framed_runtime_error(
                        writer,
                        ccp,
                        route,
                        frame.request_id,
                        &RealtimeRuntimeError {
                            code: "subscriptions_blocking_join_failed",
                            message: join_error.to_string(),
                        },
                    )
                    .await;
                    true
                }
            }
        }
        "events.pull" => {
            let limit = frame.limit.unwrap_or(100).clamp(1, 500);
            let plan = outbound_queue.plan_pull(frame.after_seq, limit, outbound_queue.latest_realtime_seq());
            // list_events_for_principal_kind performs blocking Postgres IO.
            let blocking_runtime = Arc::clone(runtime);
            let blocking_tenant = tenant_id.to_string();
            let blocking_org = organization_id.to_string();
            let blocking_principal = principal_id.to_string();
            let blocking_kind = principal_kind.to_string();
            let blocking_device = device_id.to_string();
            let after_seq = plan.after_seq;
            let batch_limit = plan.batch.limit;
            let result = tokio::task::spawn_blocking(move || {
                blocking_runtime.list_events_for_principal_kind(
                    blocking_tenant.as_str(),
                    blocking_org.as_str(),
                    blocking_principal.as_str(),
                    blocking_kind.as_str(),
                    blocking_device.as_str(),
                    after_seq,
                    batch_limit,
                )
            })
            .await;
            match result {
                Ok(Ok(window)) => {
                    let next_after_seq = window.next_after_seq;
                    let _ = send_framed_event_window(writer, ccp, route, "pull", window).await;
                    let _ = outbound_queue.record_window_sent(plan.after_seq, next_after_seq);
                    true
                }
                Ok(Err(error)) => {
                    let _ = send_framed_runtime_error(writer, ccp, route, frame.request_id, &error)
                        .await;
                    true
                }
                Err(join_error) => {
                    let _ = send_framed_runtime_error(
                        writer,
                        ccp,
                        route,
                        frame.request_id,
                        &RealtimeRuntimeError {
                            code: "events_pull_blocking_join_failed",
                            message: join_error.to_string(),
                        },
                    )
                    .await;
                    true
                }
            }
        }
        "events.ack" => {
            match frame.acked_seq {
                Some(acked_seq) => {
                    // ack_events_for_principal_kind performs blocking Postgres IO.
                    let blocking_runtime = Arc::clone(runtime);
                    let blocking_tenant = tenant_id.to_string();
                    let blocking_org = organization_id.to_string();
                    let blocking_principal = principal_id.to_string();
                    let blocking_kind = principal_kind.to_string();
                    let blocking_device = device_id.to_string();
                    let result = tokio::task::spawn_blocking(move || {
                        blocking_runtime.ack_events_for_principal_kind(
                            blocking_tenant.as_str(),
                            blocking_org.as_str(),
                            blocking_principal.as_str(),
                            blocking_kind.as_str(),
                            blocking_device.as_str(),
                            acked_seq,
                        )
                    })
                    .await;
                    match result {
                        Ok(Ok(ack)) => {
                            let bytes = ccp
                                .encode_business(
                                    route,
                                    "evt",
                                    "cc.realtime.events.acked.v1",
                                    json!({
                                        "type": "events.acked",
                                        "requestId": frame.request_id,
                                        "ack": ack
                                    }),
                                )
                                .unwrap_or_default();
                            write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                        }
                        Ok(Err(error)) => {
                            let _ = send_framed_runtime_error(
                                writer,
                                ccp,
                                route,
                                frame.request_id,
                                &error,
                            )
                            .await;
                            true
                        }
                        Err(join_error) => {
                            let _ = send_framed_runtime_error(
                                writer,
                                ccp,
                                route,
                                frame.request_id,
                                &RealtimeRuntimeError {
                                    code: "events_ack_blocking_join_failed",
                                    message: join_error.to_string(),
                                },
                            )
                            .await;
                            true
                        }
                    }
                }
                None => {
                    let _ = send_framed_business_error(
                        writer,
                        ccp,
                        route,
                        frame.request_id,
                        "invalid_frame",
                        "events.ack requires ackedSeq",
                    )
                    .await;
                    true
                }
            }
        }
        _ => {
            let _ = send_framed_business_error(
                writer,
                ccp,
                route,
                frame.request_id,
                "frame_type_unsupported",
                format!("unsupported frame type: {}", frame.frame_type),
            )
            .await;
            true
        }
    }
}
