use std::sync::Arc;
use std::time::Duration;

use im_app_context::AppContext;
use im_domain_core::realtime::RealtimeEventWindow;
use sdkwork_im_ccp_control::{ControlFrame, ErrorFrame, HeartbeatFrame};
use sdkwork_im_ccp_core::{CcpEnvelope, CcpRoute, TransportBinding};
use sdkwork_im_runtime_link::{
    LinkBufferedPushDrainDriver, LinkBufferedPushDrainStatus, LinkBufferedPushFetchedWindow,
    LinkBufferedPushPlan, LinkGoAwayDirective, LinkOutboundQueueState, LinkSession,
    OutboundQueuePolicy, ResumeWindow, link_idle_timeout_goaway, session_disconnect_goaway,
};
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::ApiError;
use crate::client_route_registration::ClientRouteRegistration;
use crate::link_business_contract::{
    LinkClientBusinessFrame, validate_link_client_business_envelope,
};
use crate::link_framing::{FramedStreamCcpCodec, read_framed_envelope, write_framed_bytes};
use crate::realtime::{
    RealtimeDeliveryRuntime, RealtimeEventWindowQuery, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput, RealtimeWindowCheckpoint,
};

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

pub(crate) struct RealtimeFramedSessionInput {
    pub transport: TransportBinding,
    pub auth: AppContext,
    pub device_id: String,
    pub resume_after_seq: Option<u64>,
    pub runtime: Arc<RealtimeDeliveryRuntime>,
    pub route_owner: ClientRouteRegistration,
}

#[derive(Clone, Copy)]
struct FramedRealtimeSessionContext<'a> {
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

pub(crate) async fn serve_realtime_framed_session<R, W>(
    mut reader: R,
    mut writer: W,
    session: RealtimeFramedSessionInput,
) -> Result<(), String>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let RealtimeFramedSessionInput {
        transport,
        auth,
        device_id,
        resume_after_seq,
        runtime,
        route_owner,
    } = session;
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
    let (checkpoint, disconnect_generation) =
        tokio::task::spawn_blocking(move || -> Result<(RealtimeWindowCheckpoint, u64), String> {
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
        })
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
        .map_err(|join_error| format!("catchup blocking task failed: {join_error}"))?
        .map_err(|error| error.message)?;
        if !catchup.items.is_empty() {
            let next_after_seq = catchup.next_after_seq;
            send_framed_event_window(&mut writer, &ccp, &route, "catchup", catchup).await?;
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
                    &mut outbound_queue,
                    push_plan,
                    FramedRealtimeSessionContext {
                        runtime: &runtime,
                        route_owner: &route_owner,
                        auth: &auth,
                        tenant_id: tenant_id.as_str(),
                        organization_id: organization_id.as_str(),
                        principal_id: principal_id.as_str(),
                        principal_kind: principal_kind.as_str(),
                        device_id: device_id.as_str(),
                        ccp: &ccp,
                        route: &route,
                    },
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
                            &mut outbound_queue,
                            &envelope,
                            FramedRealtimeSessionContext {
                                runtime: &runtime,
                                route_owner: &route_owner,
                                auth: &auth,
                                tenant_id: tenant_id.as_str(),
                                organization_id: organization_id.as_str(),
                                principal_id: principal_id.as_str(),
                                principal_kind: principal_kind.as_str(),
                                device_id: device_id.as_str(),
                                ccp: &ccp,
                                route: &route,
                            },
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
    let cleanup_runtime = runtime.clone();
    let _ = tokio::task::spawn_blocking(move || {
        route_owner.finalize_active_client_route_disconnect(
            &cleanup_auth,
            cleanup_device_id.as_str(),
            cleanup_runtime.as_ref(),
        );
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
        blocking_owner
            .ensure_active_client_route_current_session(&blocking_auth, &blocking_device_id)
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
                ccp.encode_control(route, &frame)
                    .unwrap_or_default()
                    .as_slice(),
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
                ccp.encode_control(route, &frame)
                    .unwrap_or_default()
                    .as_slice(),
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
    write_framed_bytes(writer, ccp.encode_control(route, &frame)?.as_slice()).await
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
            runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                tenant_id: tenant.as_str(),
                organization_id: org.as_str(),
                principal_id: principal.as_str(),
                principal_kind: kind.as_str(),
                device_id: device.as_str(),
                after_seq,
                limit,
            })
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
    outbound_queue: &mut LinkOutboundQueueState,
    push_plan: Option<LinkBufferedPushPlan>,
    context: FramedRealtimeSessionContext<'_>,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    let mut driver = FramedPushDrainDriver {
        writer,
        runtime: context.runtime,
        route_owner: context.route_owner,
        auth: context.auth,
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        principal_id: context.principal_id,
        principal_kind: context.principal_kind,
        device_id: context.device_id,
        ccp: context.ccp,
        route: context.route,
    };
    match outbound_queue
        .drain_buffered_push_windows(push_plan, &mut driver)
        .await
    {
        Ok(LinkBufferedPushDrainStatus::Drained) | Ok(LinkBufferedPushDrainStatus::PullOnly) => {
            true
        }
        Ok(LinkBufferedPushDrainStatus::Disconnect(directive)) => {
            let _ =
                send_framed_goaway_and_close(writer, context.ccp, context.route, &directive).await;
            false
        }
        Err(FramedPushDrainError::Runtime(error)) => {
            let _ = send_framed_runtime_error(writer, context.ccp, context.route, &error).await;
            false
        }
        Err(FramedPushDrainError::Fence) => false,
        Err(FramedPushDrainError::Send) => false,
        Err(FramedPushDrainError::JoinFailed(message)) => {
            let _ = send_framed_runtime_error(
                writer,
                context.ccp,
                context.route,
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
    write_framed_bytes(writer, ccp.encode_control(route, &frame)?.as_slice()).await
}

async fn send_framed_runtime_error<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
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
            "code": code,
            "message": message.into()
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn handle_framed_client_envelope<W>(
    writer: &mut W,
    outbound_queue: &mut LinkOutboundQueueState,
    envelope: &CcpEnvelope,
    context: FramedRealtimeSessionContext<'_>,
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
            context.ccp,
            context.route,
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
                context.ccp,
                context.route,
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
        },
    ) {
        let _ = send_framed_business_error(
            writer,
            context.ccp,
            context.route,
            "invalid_frame",
            message,
        )
        .await;
        return true;
    }

    if !ensure_framed_route_session(
        context.route_owner,
        context.auth,
        context.device_id,
        writer,
        context.ccp,
        context.route,
    )
    .await
    {
        return false;
    }

    match frame.frame_type.as_str() {
        "subscriptions.sync" => {
            // sync_subscriptions_for_principal_kind performs blocking
            // Postgres/Redis IO. Run it on the blocking pool so the async
            // worker stays free to service other connections.
            let blocking_runtime = Arc::clone(context.runtime);
            let blocking_tenant = context.tenant_id.to_string();
            let blocking_org = context.organization_id.to_string();
            let blocking_principal = context.principal_id.to_string();
            let blocking_kind = context.principal_kind.to_string();
            let blocking_device = context.device_id.to_string();
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
                    let bytes = context
                        .ccp
                        .encode_business(
                            context.route,
                            "evt",
                            "cc.realtime.subscriptions.synced.v1",
                            json!({
                                "type": "subscriptions.synced",
                                "snapshot": snapshot
                            }),
                        )
                        .unwrap_or_default();
                    write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                }
                Ok(Err(error)) => {
                    let _ =
                        send_framed_runtime_error(writer, context.ccp, context.route, &error).await;
                    true
                }
                Err(join_error) => {
                    let _ = send_framed_runtime_error(
                        writer,
                        context.ccp,
                        context.route,
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
            let plan = outbound_queue.plan_pull(
                frame.after_seq,
                limit,
                outbound_queue.latest_realtime_seq(),
            );
            // list_events_for_principal_kind performs blocking Postgres IO.
            let blocking_runtime = Arc::clone(context.runtime);
            let blocking_tenant = context.tenant_id.to_string();
            let blocking_org = context.organization_id.to_string();
            let blocking_principal = context.principal_id.to_string();
            let blocking_kind = context.principal_kind.to_string();
            let blocking_device = context.device_id.to_string();
            let after_seq = plan.after_seq;
            let batch_limit = plan.batch.limit;
            let result = tokio::task::spawn_blocking(move || {
                blocking_runtime.list_events_for_principal_kind(RealtimeEventWindowQuery {
                    tenant_id: blocking_tenant.as_str(),
                    organization_id: blocking_org.as_str(),
                    principal_id: blocking_principal.as_str(),
                    principal_kind: blocking_kind.as_str(),
                    device_id: blocking_device.as_str(),
                    after_seq,
                    limit: batch_limit,
                })
            })
            .await;
            match result {
                Ok(Ok(window)) => {
                    let next_after_seq = window.next_after_seq;
                    let _ = send_framed_event_window(
                        writer,
                        context.ccp,
                        context.route,
                        "pull",
                        window,
                    )
                    .await;
                    let _ = outbound_queue.record_window_sent(plan.after_seq, next_after_seq);
                    true
                }
                Ok(Err(error)) => {
                    let _ =
                        send_framed_runtime_error(writer, context.ccp, context.route, &error).await;
                    true
                }
                Err(join_error) => {
                    let _ = send_framed_runtime_error(
                        writer,
                        context.ccp,
                        context.route,
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
                    let blocking_runtime = Arc::clone(context.runtime);
                    let blocking_tenant = context.tenant_id.to_string();
                    let blocking_org = context.organization_id.to_string();
                    let blocking_principal = context.principal_id.to_string();
                    let blocking_kind = context.principal_kind.to_string();
                    let blocking_device = context.device_id.to_string();
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
                            let bytes = context
                                .ccp
                                .encode_business(
                                    context.route,
                                    "evt",
                                    "cc.realtime.events.acked.v1",
                                    json!({
                                        "type": "events.acked",
                                        "ack": ack
                                    }),
                                )
                                .unwrap_or_default();
                            write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                        }
                        Ok(Err(error)) => {
                            let _ = send_framed_runtime_error(
                                writer,
                                context.ccp,
                                context.route,
                                &error,
                            )
                            .await;
                            true
                        }
                        Err(join_error) => {
                            let _ = send_framed_runtime_error(
                                writer,
                                context.ccp,
                                context.route,
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
                        context.ccp,
                        context.route,
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
                context.ccp,
                context.route,
                "frame_type_unsupported",
                format!("unsupported frame type: {}", frame.frame_type),
            )
            .await;
            true
        }
    }
}
