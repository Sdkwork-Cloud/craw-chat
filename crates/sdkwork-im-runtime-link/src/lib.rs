use std::{collections::BTreeSet, future::Future, sync::OnceLock};

mod acceptor;

pub use acceptor::{
    link_transport_kind_for_binding, validate_udp_datagram_payload, LinkConnectionKey,
    LinkConnectionRecord, LinkConnectionRegistry, LinkShardDispatcher, LinkTransportKind,
    QuicBinding, QuicBindingMessage, TcpBinding, TcpBindingMessage, UdpBinding, UdpBindingMessage,
    CCP_QUIC_FRAME_HEADER_BYTES, CCP_QUIC_MAX_FRAME_BYTES, CCP_TCP_FRAME_HEADER_BYTES,
    CCP_TCP_MAX_FRAME_BYTES, CCP_UDP_MAX_DATAGRAM_BYTES, quic_framed_message_length,
    tcp_framed_message_length,
};

use sdkwork_im_ccp_binding_ws::CCP_WS_SUBPROTOCOL;
use sdkwork_im_ccp_control::{
    GoAwayFrame, HelloAckFrame, HelloFrame, SessionResumeFrame, SessionResumedFrame,
};
use sdkwork_im_ccp_core::{CapabilitySet, ProtocolVersion, TransportBinding};
use sdkwork_im_ccp_registry::{CcpRegistry, EffectiveProtocolSnapshot};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkConnectionState {
    Connected,
    HelloNegotiated,
    Authenticated,
    Active,
    Draining,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutboundQueuePolicy {
    pub soft_limit: usize,
    pub hard_limit: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkOutboundBatchPlan {
    pub limit: usize,
    pub pending_events: usize,
    pub backpressure_applied: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkPushMode {
    Immediate,
    PullOnly,
    Disconnect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkPushPlan {
    pub batch: LinkOutboundBatchPlan,
    pub mode: LinkPushMode,
    pub disconnect: Option<LinkGoAwayDirective>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkBufferedPushPlan {
    pub after_seq: u64,
    pub latest_realtime_seq: u64,
    pub push: LinkPushPlan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkBufferedPushFetchedWindow<TWindow> {
    pub window: TWindow,
    pub next_after_seq: Option<u64>,
    pub is_empty: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkBufferedPushDrainStatus {
    Drained,
    PullOnly,
    Disconnect(LinkGoAwayDirective),
}

pub trait LinkBufferedPushDrainDriver {
    type Window;
    type Error;

    fn fetch_window(
        &mut self,
        after_seq: u64,
        limit: usize,
    ) -> impl Future<Output = Result<LinkBufferedPushFetchedWindow<Self::Window>, Self::Error>> + '_;

    fn send_window(
        &mut self,
        window: Self::Window,
    ) -> impl Future<Output = Result<(), Self::Error>> + '_;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkOutboundWindowPlan {
    pub after_seq: u64,
    pub batch: LinkOutboundBatchPlan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkOutboundQueueState {
    last_sent_after_seq: u64,
    delivered_after_seq: u64,
    latest_realtime_seq: u64,
    queue_policy: OutboundQueuePolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkPushCursor {
    queue: LinkOutboundQueueState,
}

pub const DEFAULT_REALTIME_OUTBOUND_QUEUE_SOFT_LIMIT: usize = 128;
pub const DEFAULT_REALTIME_OUTBOUND_QUEUE_HARD_LIMIT: usize = 512;
pub const DEFAULT_REALTIME_OUTBOUND_QUEUE_DISCONNECT_LIMIT_MULTIPLIER: usize = 2;
pub const LINK_HELLO_PROTOCOL_FAMILY: &str = "ccp";
pub const LINK_HELLO_PROTOCOL_MAJOR: u16 = 1;
pub const LINK_HELLO_PROTOCOL_MINOR: u16 = 0;
pub const SESSION_DISCONNECT_CLOSE_CODE: u16 = 4001;
pub const SESSION_DISCONNECT_CLOSE_REASON: &str = "session.disconnect";
pub const SESSION_DISCONNECT_GOAWAY_CODE: &str = "SESSION_DISCONNECT";
pub const SESSION_DISCONNECT_GOAWAY_MESSAGE: &str = "session.disconnect";
pub const REALTIME_OVERLOAD_CLOSE_CODE: u16 = 4002;
pub const REALTIME_OVERLOAD_CLOSE_REASON: &str = "realtime.overload";
pub const REALTIME_OVERLOAD_GOAWAY_CODE: &str = "LINK_OVERLOAD";
pub const REALTIME_OVERLOAD_GOAWAY_MESSAGE: &str = "realtime.overload";
pub const LINK_IDLE_TIMEOUT_CLOSE_CODE: u16 = 4003;
pub const LINK_IDLE_TIMEOUT_CLOSE_REASON: &str = "link.idle_timeout";
pub const LINK_IDLE_TIMEOUT_GOAWAY_CODE: &str = "LINK_IDLE_TIMEOUT";
pub const LINK_IDLE_TIMEOUT_GOAWAY_MESSAGE: &str = "link.idle_timeout";
pub const LINK_WEBSOCKET_SUBPROTOCOL: &str = CCP_WS_SUBPROTOCOL;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkWebsocketMode {
    LegacyJson,
    CcpJson,
}

pub struct LinkWebsocketUpgradeHandoff<TContext> {
    mode: LinkWebsocketMode,
    context: TContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinkHelloPolicy {
    protocol: ProtocolVersion,
    allowed_bindings: BTreeSet<String>,
    capabilities: CapabilitySet,
}

impl LinkHelloPolicy {
    fn from_effective_snapshot(snapshot: &EffectiveProtocolSnapshot) -> Self {
        Self {
            protocol: parse_protocol_version(snapshot.protocol_version.as_str()),
            allowed_bindings: snapshot.allowed_bindings.clone(),
            capabilities: CapabilitySet::from_iter(snapshot.enabled_capabilities.iter().cloned()),
        }
    }

    fn allows_binding(&self, binding: &TransportBinding) -> bool {
        self.allowed_bindings.contains(binding.protocol_id())
    }

    fn negotiated_capabilities(&self, requested: &CapabilitySet) -> CapabilitySet {
        CapabilitySet {
            items: requested
                .items
                .intersection(&self.capabilities.items)
                .cloned()
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkHelloError {
    UnsupportedProtocol { wire_id: String },
    UnsupportedBinding { protocol_id: String },
}

impl LinkHelloError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedProtocol { .. } => "CCP_UNSUPPORTED_PROTOCOL",
            Self::UnsupportedBinding { .. } => "CCP_UNSUPPORTED_BINDING",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::UnsupportedProtocol { wire_id } => {
                format!("unsupported protocol: {wire_id}")
            }
            Self::UnsupportedBinding { protocol_id } => {
                format!("unsupported binding: {protocol_id}")
            }
        }
    }
}

fn default_hello_policy() -> LinkHelloPolicy {
    static HELLO_POLICY: OnceLock<LinkHelloPolicy> = OnceLock::new();
    HELLO_POLICY
        .get_or_init(|| {
            let registry = CcpRegistry::control_plane_v1();
            let snapshot = &registry
                .governance_snapshot()
                .expect("control plane registry should provide governance snapshot")
                .effective_snapshot;
            LinkHelloPolicy::from_effective_snapshot(snapshot)
        })
        .clone()
}

fn parse_protocol_version(wire_id: &str) -> ProtocolVersion {
    let (family, version) = wire_id
        .split_once('/')
        .expect("effective snapshot protocol version should contain a family separator");
    let (major, minor) = version
        .split_once('.')
        .expect("effective snapshot protocol version should contain a semantic version");
    let major = major
        .parse::<u16>()
        .expect("effective snapshot protocol major version should be parseable");
    let minor = minor
        .parse::<u16>()
        .expect("effective snapshot protocol minor version should be parseable");
    ProtocolVersion::new(family, major, minor)
}

impl OutboundQueuePolicy {
    pub fn new(soft_limit: usize, hard_limit: usize) -> Result<Self, &'static str> {
        if soft_limit == 0 || hard_limit == 0 {
            return Err("queue limits must be positive");
        }
        if soft_limit > hard_limit {
            return Err("soft limit must not exceed hard limit");
        }
        Ok(Self {
            soft_limit,
            hard_limit,
        })
    }

    pub fn realtime_default() -> Self {
        Self::new(
            DEFAULT_REALTIME_OUTBOUND_QUEUE_SOFT_LIMIT,
            DEFAULT_REALTIME_OUTBOUND_QUEUE_HARD_LIMIT,
        )
        .expect("default realtime queue policy should remain valid")
    }

    pub fn plan_stream_batch(&self, pending_events: usize) -> LinkOutboundBatchPlan {
        LinkOutboundBatchPlan {
            limit: pending_events.max(1).min(self.soft_limit),
            pending_events,
            backpressure_applied: pending_events > self.soft_limit,
        }
    }

    pub fn plan_pull_batch(
        &self,
        requested_limit: usize,
        pending_events: usize,
    ) -> LinkOutboundBatchPlan {
        let normalized_requested_limit = requested_limit.max(1);
        LinkOutboundBatchPlan {
            limit: normalized_requested_limit.min(self.hard_limit),
            pending_events,
            backpressure_applied: normalized_requested_limit > self.hard_limit
                || pending_events > self.soft_limit,
        }
    }

    pub fn plan_push_batch(&self, pending_events: usize) -> LinkPushPlan {
        let disconnect_limit = self
            .hard_limit
            .saturating_mul(DEFAULT_REALTIME_OUTBOUND_QUEUE_DISCONNECT_LIMIT_MULTIPLIER);
        LinkPushPlan {
            batch: self.plan_stream_batch(pending_events),
            mode: if pending_events > disconnect_limit {
                LinkPushMode::Disconnect
            } else if pending_events > self.hard_limit {
                LinkPushMode::PullOnly
            } else {
                LinkPushMode::Immediate
            },
            disconnect: if pending_events > disconnect_limit {
                Some(realtime_overload_goaway())
            } else {
                None
            },
        }
    }
}

pub fn pending_outbound_events(after_seq: u64, latest_realtime_seq: u64) -> usize {
    usize::try_from(latest_realtime_seq.saturating_sub(after_seq)).unwrap_or(usize::MAX)
}

impl LinkOutboundQueueState {
    pub fn new(
        last_sent_after_seq: u64,
        latest_realtime_seq: u64,
        queue_policy: OutboundQueuePolicy,
    ) -> Self {
        Self {
            last_sent_after_seq,
            delivered_after_seq: last_sent_after_seq,
            latest_realtime_seq: latest_realtime_seq.max(last_sent_after_seq),
            queue_policy,
        }
    }

    pub fn last_sent_after_seq(&self) -> u64 {
        self.last_sent_after_seq
    }

    pub fn delivered_after_seq(&self) -> u64 {
        self.delivered_after_seq
    }

    pub fn latest_realtime_seq(&self) -> u64 {
        self.latest_realtime_seq
    }

    pub fn plan_catchup(&self) -> Option<LinkOutboundWindowPlan> {
        self.plan_stream_window(self.last_sent_after_seq)
    }

    pub fn plan_pull(
        &mut self,
        requested_after_seq: Option<u64>,
        requested_limit: usize,
        latest_realtime_seq: u64,
    ) -> LinkOutboundWindowPlan {
        self.latest_realtime_seq = self.latest_realtime_seq.max(latest_realtime_seq);
        let overloaded_from_last_sent =
            pending_outbound_events(self.last_sent_after_seq, self.latest_realtime_seq)
                > self.queue_policy.hard_limit;
        let requested_after_seq = requested_after_seq.unwrap_or(self.last_sent_after_seq);
        let after_seq =
            if overloaded_from_last_sent && requested_after_seq < self.last_sent_after_seq {
                self.last_sent_after_seq
            } else {
                requested_after_seq
            };
        let pending_events = pending_outbound_events(after_seq, self.latest_realtime_seq);

        LinkOutboundWindowPlan {
            after_seq,
            batch: self
                .queue_policy
                .plan_pull_batch(requested_limit, pending_events),
        }
    }

    pub fn observe_latest_realtime_seq(
        &mut self,
        latest_realtime_seq: u64,
    ) -> Option<LinkBufferedPushPlan> {
        self.latest_realtime_seq = self.latest_realtime_seq.max(latest_realtime_seq);
        self.current_push_plan()
    }

    pub fn record_window_sent(
        &mut self,
        after_seq: u64,
        next_after_seq: Option<u64>,
    ) -> Option<LinkBufferedPushPlan> {
        let sent_through_seq = next_after_seq.unwrap_or(after_seq);
        self.last_sent_after_seq = self.last_sent_after_seq.max(sent_through_seq);
        self.delivered_after_seq = self.delivered_after_seq.max(sent_through_seq);
        self.current_push_plan()
    }

    pub fn record_client_ack(&mut self, acked_through_seq: u64) {
        self.last_sent_after_seq = self.last_sent_after_seq.max(acked_through_seq);
    }

    pub async fn drain_buffered_push_windows<TDriver>(
        &mut self,
        mut push_plan: Option<LinkBufferedPushPlan>,
        driver: &mut TDriver,
    ) -> Result<LinkBufferedPushDrainStatus, TDriver::Error>
    where
        TDriver: LinkBufferedPushDrainDriver,
    {
        loop {
            let Some(buffered_plan) = push_plan else {
                return Ok(LinkBufferedPushDrainStatus::Drained);
            };
            match buffered_plan.push.mode {
                LinkPushMode::PullOnly => return Ok(LinkBufferedPushDrainStatus::PullOnly),
                LinkPushMode::Disconnect => {
                    return Ok(LinkBufferedPushDrainStatus::Disconnect(
                        buffered_plan
                            .push
                            .disconnect
                            .clone()
                            .expect("disconnect plan should always carry a goaway directive"),
                    ));
                }
                LinkPushMode::Immediate => {}
            }

            let fetched = driver
                .fetch_window(buffered_plan.after_seq, buffered_plan.push.batch.limit)
                .await?;
            if fetched.is_empty {
                return Ok(LinkBufferedPushDrainStatus::Drained);
            }

            driver.send_window(fetched.window).await?;
            push_plan = self.record_window_sent(buffered_plan.after_seq, fetched.next_after_seq);
        }
    }

    fn plan_stream_window(&self, after_seq: u64) -> Option<LinkOutboundWindowPlan> {
        let pending_events = pending_outbound_events(after_seq, self.latest_realtime_seq);
        if pending_events == 0 {
            return None;
        }

        Some(LinkOutboundWindowPlan {
            after_seq,
            batch: self.queue_policy.plan_stream_batch(pending_events),
        })
    }

    fn current_push_plan(&self) -> Option<LinkBufferedPushPlan> {
        let pending_events =
            pending_outbound_events(self.delivered_after_seq, self.latest_realtime_seq);
        if pending_events == 0 {
            return None;
        }

        Some(LinkBufferedPushPlan {
            after_seq: self.delivered_after_seq,
            latest_realtime_seq: self.latest_realtime_seq,
            push: self.queue_policy.plan_push_batch(pending_events),
        })
    }
}

impl LinkPushCursor {
    pub fn new(
        delivered_after_seq: u64,
        latest_realtime_seq: u64,
        queue_policy: OutboundQueuePolicy,
    ) -> Self {
        Self {
            queue: LinkOutboundQueueState::new(
                delivered_after_seq,
                latest_realtime_seq,
                queue_policy,
            ),
        }
    }

    pub fn delivered_after_seq(&self) -> u64 {
        self.queue.delivered_after_seq()
    }

    pub fn latest_realtime_seq(&self) -> u64 {
        self.queue.latest_realtime_seq()
    }

    pub fn observe_latest_realtime_seq(
        &mut self,
        latest_realtime_seq: u64,
    ) -> Option<LinkBufferedPushPlan> {
        self.queue.observe_latest_realtime_seq(latest_realtime_seq)
    }

    pub fn acknowledge_window(
        &mut self,
        next_after_seq: Option<u64>,
    ) -> Option<LinkBufferedPushPlan> {
        let next_after_seq = next_after_seq?;
        if next_after_seq <= self.queue.delivered_after_seq() {
            return None;
        }

        self.queue
            .record_window_sent(self.queue.delivered_after_seq(), Some(next_after_seq))
    }
}

pub fn supported_websocket_subprotocols() -> [&'static str; 1] {
    [LINK_WEBSOCKET_SUBPROTOCOL]
}

pub fn select_websocket_mode(selected_subprotocol: Option<&str>) -> LinkWebsocketMode {
    match selected_subprotocol {
        Some(LINK_WEBSOCKET_SUBPROTOCOL) => LinkWebsocketMode::CcpJson,
        _ => LinkWebsocketMode::LegacyJson,
    }
}

pub fn prepare_websocket_upgrade<TContext>(
    selected_subprotocol: Option<&str>,
    context: TContext,
) -> LinkWebsocketUpgradeHandoff<TContext> {
    LinkWebsocketUpgradeHandoff {
        mode: select_websocket_mode(selected_subprotocol),
        context,
    }
}

impl<TContext> LinkWebsocketUpgradeHandoff<TContext> {
    pub fn mode(&self) -> LinkWebsocketMode {
        self.mode
    }

    pub fn context(&self) -> &TContext {
        &self.context
    }

    pub async fn execute<TSocket, THandler, TFuture>(self, socket: TSocket, handler: THandler)
    where
        THandler: FnOnce(TSocket, TContext, LinkWebsocketMode) -> TFuture,
        TFuture: Future<Output = ()>,
    {
        handler(socket, self.context, self.mode).await
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResumeWindow {
    pub last_seen_sync_seq: u64,
    pub last_checkpoint_seq: u64,
}

impl ResumeWindow {
    pub fn new(last_seen_sync_seq: u64, last_checkpoint_seq: u64) -> Self {
        Self {
            last_seen_sync_seq,
            last_checkpoint_seq,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResumeDecision {
    pub resume_required: bool,
    pub resume_from_sync_seq: u64,
    pub latest_sync_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkGoAwayDirective {
    pub close_code: u16,
    pub close_reason: &'static str,
    pub frame: GoAwayFrame,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkSessionResumeDirective {
    pub frame: SessionResumedFrame,
    pub catchup_after_seq: u64,
    pub decision: ResumeDecision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkSessionResumeError {
    SessionMismatch {
        expected_session_id: Option<String>,
        actual_session_id: String,
    },
}

impl LinkSessionResumeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::SessionMismatch { .. } => "CCP_RESUME_SESSION_MISMATCH",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::SessionMismatch {
                expected_session_id,
                actual_session_id,
            } => format!(
                "session_resume does not match authenticated session: expected {:?}, got {}",
                expected_session_id, actual_session_id
            ),
        }
    }
}

pub fn decide_resume(last_seen_sync_seq: u64, latest_sync_seq: u64) -> ResumeDecision {
    let resume_required = latest_sync_seq > last_seen_sync_seq;
    let resume_from_sync_seq = if latest_sync_seq == 0 {
        0
    } else if resume_required {
        last_seen_sync_seq.saturating_add(1)
    } else {
        latest_sync_seq
    };

    ResumeDecision {
        resume_required,
        resume_from_sync_seq,
        latest_sync_seq,
    }
}

pub fn session_disconnect_goaway() -> LinkGoAwayDirective {
    LinkGoAwayDirective {
        close_code: SESSION_DISCONNECT_CLOSE_CODE,
        close_reason: SESSION_DISCONNECT_CLOSE_REASON,
        frame: GoAwayFrame {
            code: SESSION_DISCONNECT_GOAWAY_CODE.into(),
            message: SESSION_DISCONNECT_GOAWAY_MESSAGE.into(),
        },
    }
}

pub fn realtime_overload_goaway() -> LinkGoAwayDirective {
    LinkGoAwayDirective {
        close_code: REALTIME_OVERLOAD_CLOSE_CODE,
        close_reason: REALTIME_OVERLOAD_CLOSE_REASON,
        frame: GoAwayFrame {
            code: REALTIME_OVERLOAD_GOAWAY_CODE.into(),
            message: REALTIME_OVERLOAD_GOAWAY_MESSAGE.into(),
        },
    }
}

/// Builds the go-away directive used when a link session is torn down
/// because the peer stayed silent past the configured idle timeout.
/// This lets clients distinguish idle-driven disconnects from session
/// disconnects or overload-driven ones and surface the right retry
/// guidance (idle disconnects should re-establish the link rather than
/// retry in place).
pub fn link_idle_timeout_goaway() -> LinkGoAwayDirective {
    LinkGoAwayDirective {
        close_code: LINK_IDLE_TIMEOUT_CLOSE_CODE,
        close_reason: LINK_IDLE_TIMEOUT_CLOSE_REASON,
        frame: GoAwayFrame {
            code: LINK_IDLE_TIMEOUT_GOAWAY_CODE.into(),
            message: LINK_IDLE_TIMEOUT_GOAWAY_MESSAGE.into(),
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkSession {
    pub tenant_id: String,
    pub principal_id: String,
    pub actor_kind: String,
    pub device_id: String,
    pub session_id: Option<String>,
    state: LinkConnectionState,
    queue_policy: OutboundQueuePolicy,
    resume_window: ResumeWindow,
    hello_policy: LinkHelloPolicy,
}

impl LinkSession {
    pub fn new(
        tenant_id: &str,
        principal_id: &str,
        actor_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
        queue_policy: OutboundQueuePolicy,
    ) -> Self {
        Self::new_with_hello_policy(
            tenant_id,
            principal_id,
            actor_kind,
            device_id,
            session_id,
            queue_policy,
            default_hello_policy(),
        )
    }

    pub fn new_with_effective_snapshot(
        tenant_id: &str,
        principal_id: &str,
        actor_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
        queue_policy: OutboundQueuePolicy,
        effective_snapshot: EffectiveProtocolSnapshot,
    ) -> Self {
        Self::new_with_hello_policy(
            tenant_id,
            principal_id,
            actor_kind,
            device_id,
            session_id,
            queue_policy,
            LinkHelloPolicy::from_effective_snapshot(&effective_snapshot),
        )
    }

    fn new_with_hello_policy(
        tenant_id: &str,
        principal_id: &str,
        actor_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
        queue_policy: OutboundQueuePolicy,
        hello_policy: LinkHelloPolicy,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            actor_kind: actor_kind.into(),
            device_id: device_id.into(),
            session_id: session_id.map(str::to_owned),
            state: LinkConnectionState::Connected,
            queue_policy,
            resume_window: ResumeWindow::default(),
            hello_policy,
        }
    }

    pub fn state(&self) -> LinkConnectionState {
        self.state.clone()
    }

    pub fn queue_policy(&self) -> &OutboundQueuePolicy {
        &self.queue_policy
    }

    pub fn resume_window(&self) -> &ResumeWindow {
        &self.resume_window
    }

    pub fn plan_stream_batch(&self, pending_events: usize) -> LinkOutboundBatchPlan {
        self.queue_policy.plan_stream_batch(pending_events)
    }

    pub fn plan_pull_batch(
        &self,
        requested_limit: usize,
        pending_events: usize,
    ) -> LinkOutboundBatchPlan {
        self.queue_policy
            .plan_pull_batch(requested_limit, pending_events)
    }

    pub fn plan_push_batch(&self, pending_events: usize) -> LinkPushPlan {
        self.queue_policy.plan_push_batch(pending_events)
    }

    pub fn start_outbound_queue(
        &self,
        last_sent_after_seq: u64,
        latest_realtime_seq: u64,
    ) -> LinkOutboundQueueState {
        LinkOutboundQueueState::new(
            last_sent_after_seq,
            latest_realtime_seq,
            self.queue_policy.clone(),
        )
    }

    pub fn start_push_cursor(
        &self,
        delivered_after_seq: u64,
        latest_realtime_seq: u64,
    ) -> LinkPushCursor {
        LinkPushCursor::new(
            delivered_after_seq,
            latest_realtime_seq,
            self.queue_policy.clone(),
        )
    }

    pub fn matches_auth_bind(
        &self,
        principal_id: &str,
        actor_kind: &str,
        device_id: Option<&str>,
        session_id: Option<&str>,
    ) -> bool {
        if principal_id != self.principal_id {
            return false;
        }
        if actor_kind != self.actor_kind {
            return false;
        }
        if device_id.is_some_and(|candidate| candidate != self.device_id.as_str()) {
            return false;
        }
        match (session_id, self.session_id.as_deref()) {
            (Some(candidate), Some(expected)) if candidate != expected => false,
            (Some(_), None) => false,
            _ => true,
        }
    }

    pub fn negotiate_hello(&mut self, hello: &HelloFrame) -> Result<HelloAckFrame, LinkHelloError> {
        if hello.protocol.family != self.hello_policy.protocol.family
            || hello.protocol.major != self.hello_policy.protocol.major
        {
            return Err(LinkHelloError::UnsupportedProtocol {
                wire_id: hello.protocol.wire_id(),
            });
        }
        if !self.hello_policy.allows_binding(&hello.binding) {
            return Err(LinkHelloError::UnsupportedBinding {
                protocol_id: hello.binding.protocol_id().to_owned(),
            });
        }

        self.mark_hello_negotiated();
        Ok(HelloAckFrame {
            protocol: self.hello_policy.protocol.clone(),
            binding: hello.binding.clone(),
            capabilities: self
                .hello_policy
                .negotiated_capabilities(&hello.capabilities),
            accepted: true,
        })
    }

    pub fn mark_hello_negotiated(&mut self) {
        self.state = LinkConnectionState::HelloNegotiated;
    }

    pub fn mark_authenticated(&mut self) {
        self.state = LinkConnectionState::Authenticated;
    }

    pub fn negotiate_session_resume(
        &self,
        frame: &SessionResumeFrame,
        latest_realtime_seq: u64,
        acked_through_seq: u64,
    ) -> Result<LinkSessionResumeDirective, LinkSessionResumeError> {
        if self.session_id.as_deref() != Some(frame.session_id.as_str()) {
            return Err(LinkSessionResumeError::SessionMismatch {
                expected_session_id: self.session_id.clone(),
                actual_session_id: frame.session_id.clone(),
            });
        }

        let acknowledged_seq = frame
            .last_acked_seq
            .unwrap_or(acked_through_seq)
            .max(acked_through_seq)
            .min(latest_realtime_seq);
        let decision = decide_resume(acknowledged_seq, latest_realtime_seq);

        Ok(LinkSessionResumeDirective {
            frame: SessionResumedFrame {
                session_id: frame.session_id.clone(),
                resumed: decision.resume_required,
            },
            catchup_after_seq: acknowledged_seq,
            decision,
        })
    }

    pub fn activate(&mut self, resume_window: ResumeWindow) {
        self.resume_window = resume_window;
        self.state = LinkConnectionState::Active;
    }

    pub fn mark_draining(&mut self) {
        self.state = LinkConnectionState::Draining;
    }
}

#[cfg(test)]
mod tests {
    use sdkwork_im_ccp_binding_ws::CCP_WS_SUBPROTOCOL;
    use sdkwork_im_ccp_control::{HelloFrame, SessionResumeFrame};
    use sdkwork_im_ccp_core::{CapabilitySet, ProtocolVersion, TransportBinding};
    use sdkwork_im_ccp_registry::CcpRegistry;
    use std::collections::BTreeSet;
    use std::task::{Context, Poll, Waker};

    use super::{
        LinkBufferedPushDrainDriver, LinkBufferedPushDrainStatus, LinkBufferedPushFetchedWindow,
        LinkHelloError, LinkPushMode, LinkSession, LinkSessionResumeDirective, LinkWebsocketMode,
        OutboundQueuePolicy, REALTIME_OVERLOAD_CLOSE_CODE, REALTIME_OVERLOAD_CLOSE_REASON,
        ResumeDecision, SessionResumedFrame, decide_resume, parse_protocol_version,
        pending_outbound_events, prepare_websocket_upgrade, select_websocket_mode,
        supported_websocket_subprotocols,
    };

    fn poll_ready<F>(future: F) -> F::Output
    where
        F: std::future::Future,
    {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = std::pin::pin!(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("test future should resolve immediately"),
        }
    }

    #[test]
    fn test_realtime_default_queue_policy_uses_owner_limits() {
        let policy = OutboundQueuePolicy::realtime_default();

        assert_eq!(policy.soft_limit, 128);
        assert_eq!(policy.hard_limit, 512);
    }

    #[test]
    fn test_link_session_matches_auth_bind_identity() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        assert!(session.matches_auth_bind("1", "user", Some("d_pad"), Some("s_demo")));
        assert!(session.matches_auth_bind("1", "user", None, Some("s_demo")));
    }

    #[test]
    fn test_link_session_rejects_mismatched_auth_bind_identity() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        assert!(!session.matches_auth_bind("other", "user", Some("d_pad"), Some("s_demo")));
        assert!(!session.matches_auth_bind("1", "agent", Some("d_pad"), Some("s_demo")));
        assert!(!session.matches_auth_bind("1", "user", Some("d_phone"), Some("s_demo")));
        assert!(!session.matches_auth_bind("1", "user", Some("d_pad"), Some("other")));
    }

    #[test]
    fn test_link_session_negotiates_supported_hello() {
        let expected_snapshot = CcpRegistry::control_plane_v1()
            .governance_snapshot()
            .expect("registry should expose governance snapshot")
            .effective_snapshot
            .clone();
        let mut session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );
        let hello = HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["session.resume", "payload.json", "ignored"]),
            trace_id: Some("trace-hello".into()),
        };

        let hello_ack = session
            .negotiate_hello(&hello)
            .expect("supported hello should be accepted");

        assert_eq!(session.state(), super::LinkConnectionState::HelloNegotiated);
        assert_eq!(
            hello_ack.protocol,
            parse_protocol_version(expected_snapshot.protocol_version.as_str())
        );
        assert_eq!(hello_ack.binding, TransportBinding::Ws1);
        assert_eq!(
            hello_ack.capabilities,
            CapabilitySet::from_iter(["payload.json", "session.resume"])
        );
        assert!(hello_ack.accepted);
    }

    #[test]
    fn test_link_session_negotiates_hello_from_effective_snapshot() {
        let registry = CcpRegistry::control_plane_v1();
        let mut snapshot = registry
            .governance_snapshot()
            .expect("registry should expose governance snapshot")
            .effective_snapshot
            .clone();
        snapshot.protocol_version = "ccp/1.1".into();
        snapshot.enabled_capabilities = ["session.resume"].into_iter().map(str::to_owned).collect();

        let mut session = LinkSession::new_with_effective_snapshot(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
            snapshot,
        );
        let hello = HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["session.resume", "payload.json"]),
            trace_id: Some("trace-governance".into()),
        };

        let hello_ack = session
            .negotiate_hello(&hello)
            .expect("effective snapshot should drive hello negotiation");

        assert_eq!(hello_ack.protocol, ProtocolVersion::new("ccp", 1, 1));
        assert_eq!(
            hello_ack.capabilities,
            CapabilitySet::from_iter(["session.resume"])
        );
    }

    #[test]
    fn test_link_session_rejects_binding_blocked_by_effective_snapshot() {
        let registry = CcpRegistry::control_plane_v1();
        let mut snapshot = registry
            .governance_snapshot()
            .expect("registry should expose governance snapshot")
            .effective_snapshot
            .clone();
        snapshot.allowed_bindings = BTreeSet::from(["ccp/http/1".to_string()]);

        let mut session = LinkSession::new_with_effective_snapshot(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
            snapshot,
        );
        let hello = HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["session.resume"]),
            trace_id: Some("trace-blocked-binding".into()),
        };

        let error = session
            .negotiate_hello(&hello)
            .expect_err("binding blocked by effective snapshot should be rejected");

        assert_eq!(
            error,
            LinkHelloError::UnsupportedBinding {
                protocol_id: "ccp/ws/1".into()
            }
        );
    }

    #[test]
    fn test_link_session_rejects_unsupported_hello_protocol() {
        let mut session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );
        let hello = HelloFrame {
            protocol: ProtocolVersion::new("other", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["payload.json"]),
            trace_id: None,
        };

        let error = session
            .negotiate_hello(&hello)
            .expect_err("unsupported protocol should be rejected");

        assert_eq!(
            error,
            LinkHelloError::UnsupportedProtocol {
                wire_id: "other/1.0".into()
            }
        );
    }

    #[test]
    fn test_link_session_rejects_unsupported_hello_binding() {
        let mut session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );
        let hello = HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Mqtt1,
            capabilities: CapabilitySet::from_iter(["payload.json"]),
            trace_id: None,
        };

        let error = session
            .negotiate_hello(&hello)
            .expect_err("unsupported binding should be rejected");

        assert_eq!(
            error,
            LinkHelloError::UnsupportedBinding {
                protocol_id: "ccp/mqtt/1".into()
            }
        );
    }

    #[test]
    fn test_runtime_link_decides_incremental_resume_window() {
        let decision = decide_resume(7, 11);

        assert_eq!(
            decision,
            ResumeDecision {
                resume_required: true,
                resume_from_sync_seq: 8,
                latest_sync_seq: 11,
            }
        );
    }

    #[test]
    fn test_runtime_link_decides_no_backfill_when_latest_sync_is_empty() {
        let decision = decide_resume(7, 0);

        assert_eq!(
            decision,
            ResumeDecision {
                resume_required: false,
                resume_from_sync_seq: 0,
                latest_sync_seq: 0,
            }
        );
    }

    #[test]
    fn test_runtime_link_builds_session_disconnect_goaway_owner_contract() {
        let directive = super::session_disconnect_goaway();

        assert_eq!(directive.close_code, 4001);
        assert_eq!(directive.close_reason, "session.disconnect");
        assert_eq!(directive.frame.code, "SESSION_DISCONNECT");
        assert_eq!(directive.frame.message, "session.disconnect");
    }

    #[test]
    fn test_runtime_link_builds_session_resumed_owner_contract() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let directive = session
            .negotiate_session_resume(
                &SessionResumeFrame {
                    session_id: "s_demo".into(),
                    last_acked_seq: Some(5),
                },
                9,
                4,
            )
            .expect("matching session_resume should be accepted");

        assert_eq!(
            directive,
            LinkSessionResumeDirective {
                frame: SessionResumedFrame {
                    session_id: "s_demo".into(),
                    resumed: true,
                },
                catchup_after_seq: 5,
                decision: ResumeDecision {
                    resume_required: true,
                    resume_from_sync_seq: 6,
                    latest_sync_seq: 9,
                },
            }
        );
    }

    #[test]
    fn test_runtime_link_exposes_websocket_upgrade_owner_contract() {
        assert_eq!(supported_websocket_subprotocols(), [CCP_WS_SUBPROTOCOL]);
        assert_eq!(
            select_websocket_mode(Some(CCP_WS_SUBPROTOCOL)),
            LinkWebsocketMode::CcpJson
        );
        assert_eq!(
            select_websocket_mode(Some("legacy.json")),
            LinkWebsocketMode::LegacyJson
        );
        assert_eq!(select_websocket_mode(None), LinkWebsocketMode::LegacyJson);
    }

    #[test]
    fn test_runtime_link_prepares_websocket_upgrade_handoff_owner_contract() {
        let handoff = prepare_websocket_upgrade(Some(CCP_WS_SUBPROTOCOL), "ctx");

        assert_eq!(handoff.mode(), LinkWebsocketMode::CcpJson);
        assert_eq!(handoff.context(), &"ctx");

        let mut observed = None;
        poll_ready(handoff.execute("socket", |socket, context, mode| {
            observed = Some((socket, context, mode));
            std::future::ready(())
        }));

        assert_eq!(
            observed,
            Some(("socket", "ctx", LinkWebsocketMode::CcpJson))
        );
    }

    #[test]
    fn test_runtime_link_plans_live_outbound_queue_batches_from_owner_limits() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let catchup = session.plan_stream_batch(520);
        let pull = session.plan_pull_batch(999, 520);

        assert_eq!(catchup.limit, 128);
        assert_eq!(catchup.pending_events, 520);
        assert!(catchup.backpressure_applied);

        assert_eq!(pull.limit, 512);
        assert_eq!(pull.pending_events, 520);
        assert!(pull.backpressure_applied);
    }

    #[test]
    fn test_runtime_link_degrades_live_push_to_pull_only_when_backlog_exceeds_hard_limit() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let push = session.plan_push_batch(700);

        assert_eq!(push.mode, LinkPushMode::PullOnly);
        assert_eq!(push.batch.limit, 128);
        assert_eq!(push.batch.pending_events, 700);
        assert!(push.batch.backpressure_applied);
    }

    #[test]
    fn test_runtime_link_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let mut push_cursor = session.start_push_cursor(128, 128);
        let overloaded = push_cursor
            .observe_latest_realtime_seq(700)
            .expect("observed backlog should produce a buffered push plan");

        assert_eq!(overloaded.after_seq, 128);
        assert_eq!(overloaded.latest_realtime_seq, 700);
        assert_eq!(overloaded.push.mode, LinkPushMode::PullOnly);

        let recovered = push_cursor
            .acknowledge_window(Some(640))
            .expect("reduced backlog should recover buffered push");

        assert_eq!(recovered.after_seq, 640);
        assert_eq!(recovered.latest_realtime_seq, 700);
        assert_eq!(recovered.push.mode, LinkPushMode::Immediate);
        assert_eq!(recovered.push.batch.limit, 60);
        assert_eq!(recovered.push.batch.pending_events, 60);
        assert_eq!(push_cursor.delivered_after_seq(), 640);
        assert_eq!(push_cursor.latest_realtime_seq(), 700);

        assert_eq!(push_cursor.acknowledge_window(Some(700)), None);
    }

    #[test]
    fn test_runtime_link_closes_connection_when_backlog_exceeds_overload_disconnect_limit() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let mut push_cursor = session.start_push_cursor(128, 128);
        let overloaded = push_cursor
            .observe_latest_realtime_seq(1200)
            .expect("observed backlog should produce an overload plan");

        assert_eq!(overloaded.after_seq, 128);
        assert_eq!(overloaded.latest_realtime_seq, 1200);
        assert_eq!(overloaded.push.mode, LinkPushMode::Disconnect);
        assert_eq!(
            overloaded
                .push
                .disconnect
                .as_ref()
                .expect("disconnect directive should exist")
                .close_code,
            REALTIME_OVERLOAD_CLOSE_CODE
        );
        assert_eq!(
            overloaded
                .push
                .disconnect
                .as_ref()
                .expect("disconnect directive should exist")
                .close_reason,
            REALTIME_OVERLOAD_CLOSE_REASON
        );
    }

    #[test]
    fn test_runtime_link_counts_pending_outbound_events_with_saturating_math() {
        assert_eq!(pending_outbound_events(10, 10), 0);
        assert_eq!(pending_outbound_events(10, 42), 32);
        assert_eq!(pending_outbound_events(42, 10), 0);
    }

    #[test]
    fn test_runtime_link_outbound_queue_state_owns_last_sent_seq_and_buffered_push_recovery() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let mut queue = session.start_outbound_queue(128, 128);

        assert_eq!(queue.last_sent_after_seq(), 128);
        assert_eq!(queue.delivered_after_seq(), 128);
        assert_eq!(queue.latest_realtime_seq(), 128);
        assert_eq!(queue.plan_catchup(), None);

        let overloaded = queue
            .observe_latest_realtime_seq(700)
            .expect("observed backlog should produce a buffered queue plan");
        assert_eq!(overloaded.after_seq, 128);
        assert_eq!(overloaded.latest_realtime_seq, 700);
        assert_eq!(overloaded.push.mode, LinkPushMode::PullOnly);

        let pull = queue.plan_pull(None, 999, 700);
        assert_eq!(pull.after_seq, 128);
        assert_eq!(pull.batch.limit, 512);
        assert_eq!(pull.batch.pending_events, 572);

        let recovered = queue
            .record_window_sent(pull.after_seq, Some(640))
            .expect("reduced backlog should recover buffered push");
        assert_eq!(queue.last_sent_after_seq(), 640);
        assert_eq!(queue.delivered_after_seq(), 640);
        assert_eq!(queue.latest_realtime_seq(), 700);
        assert_eq!(recovered.after_seq, 640);
        assert_eq!(recovered.push.mode, LinkPushMode::Immediate);
        assert_eq!(recovered.push.batch.limit, 60);
        assert_eq!(recovered.push.batch.pending_events, 60);
    }

    #[test]
    fn test_runtime_link_drops_stale_pull_replay_when_backlog_is_still_over_hard_limit() {
        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let mut queue = session.start_outbound_queue(128, 900);

        let pull = queue.plan_pull(Some(0), 999, 900);

        assert_eq!(pull.after_seq, 128);
        assert_eq!(pull.batch.limit, 512);
        assert_eq!(pull.batch.pending_events, 772);
        assert!(pull.batch.backpressure_applied);
    }

    #[test]
    fn test_runtime_link_drains_buffered_push_windows_via_owner_async_loop() {
        struct RecordingBufferedPushDrainDriver {
            fetched: Vec<(u64, usize)>,
            sent: Vec<(u64, usize)>,
        }

        impl LinkBufferedPushDrainDriver for RecordingBufferedPushDrainDriver {
            type Window = (u64, usize);
            type Error = &'static str;

            fn fetch_window(
                &mut self,
                after_seq: u64,
                limit: usize,
            ) -> impl Future<
                Output = Result<LinkBufferedPushFetchedWindow<Self::Window>, Self::Error>,
            > + '_ {
                self.fetched.push((after_seq, limit));
                std::future::ready(Ok(LinkBufferedPushFetchedWindow {
                    window: (after_seq, limit),
                    next_after_seq: Some(700),
                    is_empty: false,
                }))
            }

            fn send_window(
                &mut self,
                window: Self::Window,
            ) -> impl Future<Output = Result<(), Self::Error>> + '_ {
                self.sent.push(window);
                std::future::ready(Ok(()))
            }
        }

        let session = LinkSession::new(
            "100001",
            "1",
            "user",
            "d_pad",
            Some("s_demo"),
            OutboundQueuePolicy::realtime_default(),
        );

        let mut queue = session.start_outbound_queue(128, 700);
        let push_plan = queue
            .record_window_sent(128, Some(640))
            .expect("reduced backlog should produce a buffered push recovery plan");
        let mut driver = RecordingBufferedPushDrainDriver {
            fetched: Vec::new(),
            sent: Vec::new(),
        };

        let outcome = poll_ready(queue.drain_buffered_push_windows(Some(push_plan), &mut driver))
            .expect("runtime owner drain loop should stay green");

        assert_eq!(outcome, LinkBufferedPushDrainStatus::Drained);
        assert_eq!(driver.fetched, vec![(640, 60)]);
        assert_eq!(driver.sent, vec![(640, 60)]);
        assert_eq!(queue.last_sent_after_seq(), 700);
        assert_eq!(queue.delivered_after_seq(), 700);
        assert_eq!(queue.latest_realtime_seq(), 700);
    }
}
