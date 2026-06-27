use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::{header, HeaderMap, HeaderValue};
use im_app_context::{AppContext, AppContextError};
use crate::auth_context::RealtimeAuthContextResolver;
use sdkwork_im_ccp_binding_udp::{UdpBinding, CCP_UDP_MAX_DATAGRAM_BYTES};
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{AuthBindFrame, AuthOkFrame, ControlFrame, ErrorFrame};
use sdkwork_im_ccp_core::{CcpEnvelope, ProtocolVersion, TransportBinding};
use sdkwork_im_runtime_link::{
    LinkConnectionKey, LinkConnectionRecord, LinkConnectionRegistry, LinkHelloError, LinkSession,
    LinkShardDispatcher, LinkTransportKind, OutboundQueuePolicy,
};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::cluster_route_event_auth::validate_link_bind_addr_for_cleartext_tokens;
use crate::client_route_registration::ClientRouteRegistration;
use crate::client_route_state::ClientRouteState;
use crate::link_framing::{read_framed_envelope, send_framed_control_frame, send_framed_error_and_close};
use crate::link_quic::{resolve_quic_bind_addr, spawn_quic_listener};
use crate::link_realtime::serve_realtime_framed_session;
use crate::{ApiError, RealtimePlaneAssembly};

const REALTIME_TCP_BIND_ENV: &str = "SDKWORK_IM_REALTIME_TCP_BIND_ADDR";
const REALTIME_UDP_BIND_ENV: &str = "SDKWORK_IM_REALTIME_UDP_BIND_ADDR";
const REALTIME_MAX_LINK_CONNECTIONS_ENV: &str = "SDKWORK_IM_REALTIME_MAX_LINK_CONNECTIONS";
const REALTIME_MAX_LINK_CONNECTIONS_DEFAULT: usize = 10_000;
const REALTIME_HANDSHAKE_TIMEOUT_SECS_ENV: &str = "SDKWORK_IM_REALTIME_HANDSHAKE_TIMEOUT_SECS";
const REALTIME_HANDSHAKE_TIMEOUT_DEFAULT_SECS: u64 = 15;

fn resolve_handshake_timeout() -> Duration {
    let secs = std::env::var(REALTIME_HANDSHAKE_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(REALTIME_HANDSHAKE_TIMEOUT_DEFAULT_SECS)
        .max(1);
    Duration::from_secs(secs)
}

struct StreamLinkAuthResult {
    context: AppContext,
    device_id: String,
    resume_after_seq: Option<u64>,
}

#[derive(Clone)]
pub struct LinkTransportRuntime {
    assembly: RealtimePlaneAssembly,
    node_id: String,
    connection_registry: Arc<LinkConnectionRegistry>,
    shard_dispatcher: LinkShardDispatcher,
    connection_semaphore: Arc<Semaphore>,
    route_registration: ClientRouteRegistration,
    auth_resolver: RealtimeAuthContextResolver,
}

impl LinkTransportRuntime {
    pub fn new(
        assembly: RealtimePlaneAssembly,
        node_id: impl Into<String>,
        auth_resolver: RealtimeAuthContextResolver,
    ) -> Self {
        let node_id = node_id.into();
        let client_route_state = ClientRouteState::default();
        let route_registration = ClientRouteRegistration::new(
            node_id.clone(),
            assembly.realtime_cluster(),
            assembly.presence_runtime(),
            assembly.realtime_runtime(),
            client_route_state,
        );
        Self {
            assembly,
            node_id,
            connection_registry: Arc::new(LinkConnectionRegistry::new()),
            shard_dispatcher: LinkShardDispatcher::default_realtime(),
            connection_semaphore: Arc::new(Semaphore::new(resolve_max_link_connections())),
            route_registration,
            auth_resolver,
        }
    }

    pub(crate) fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    pub(crate) fn connection_semaphore(&self) -> Arc<Semaphore> {
        self.connection_semaphore.clone()
    }

    pub(crate) async fn serve_quic_connection(
        &self,
        connection: quinn::Connection,
    ) -> Result<(), String> {
        let peer_addr = connection.remote_address();
        let (mut send, mut recv) = connection
            .accept_bi()
            .await
            .map_err(|error| format!("quic bi stream accept failed: {error}"))?;
        let auth = self
            .complete_stream_link_handshake(
                &mut recv,
                &mut send,
                TransportBinding::Quic1,
                LinkTransportKind::Quic,
                peer_addr,
            )
            .await?;
        info!(
            target: "sdkwork.im",
            event = "im.link.quic.authenticated",
            peer = %peer_addr,
            node_id = %self.node_id,
            "quic link session entering realtime loop"
        );
        serve_realtime_framed_session(
            recv,
            send,
            TransportBinding::Quic1,
            auth.context,
            auth.device_id,
            auth.resume_after_seq,
            self.assembly.realtime_runtime(),
            self.route_registration.clone(),
        )
        .await
    }

    #[cfg(test)]
    pub(crate) fn assembly(&self) -> &RealtimePlaneAssembly {
        &self.assembly
    }

    pub fn spawn_listeners(&self) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();
        if let Some(bind_addr) = resolve_optional_bind_addr(REALTIME_TCP_BIND_ENV) {
            if let Err(error) = validate_link_bind_addr_for_cleartext_tokens("tcp", bind_addr) {
                warn!(target: "sdkwork.im", event = "im.link.tcp.bind_rejected", %error);
            } else {
                let runtime = self.clone();
                handles.push(tokio::spawn(async move {
                    if let Err(error) = runtime.serve_tcp_listener(bind_addr).await {
                        warn!(target: "sdkwork.im", event = "im.link.tcp.failed", %error);
                    }
                }));
            }
        }
        if let Some(bind_addr) = resolve_optional_bind_addr(REALTIME_UDP_BIND_ENV) {
            if let Err(error) = validate_link_bind_addr_for_cleartext_tokens("udp", bind_addr) {
                warn!(target: "sdkwork.im", event = "im.link.udp.bind_rejected", %error);
            } else {
                let runtime = self.clone();
                handles.push(tokio::spawn(async move {
                    if let Err(error) = runtime.serve_udp_listener(bind_addr).await {
                        warn!(target: "sdkwork.im", event = "im.link.udp.failed", %error);
                    }
                }));
            }
        }
        if let Some(bind_addr) = resolve_quic_bind_addr() {
            handles.push(spawn_quic_listener(self.clone(), bind_addr));
        }
        handles
    }

    async fn serve_tcp_listener(&self, bind_addr: SocketAddr) -> Result<(), String> {
        let listener = TcpListener::bind(bind_addr)
            .await
            .map_err(|error| format!("tcp link listener failed to bind {bind_addr}: {error}"))?;
        info!(
            target: "sdkwork.im",
            event = "im.link.tcp.listen",
            bind = %bind_addr,
            node_id = %self.node_id,
            "realtime tcp link listener started"
        );

        loop {
            let (mut stream, peer_addr) = listener
                .accept()
                .await
                .map_err(|error| format!("tcp link accept failed: {error}"))?;
            let permit = match self.connection_semaphore.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.link.tcp.overload",
                        peer = %peer_addr,
                        "rejecting tcp link connection at capacity"
                    );
                    let _ = stream.shutdown().await;
                    continue;
                }
            };
            let runtime = self.clone();
            tokio::spawn(async move {
                let _permit = permit;
                if let Err(error) = runtime.serve_tcp_connection(stream, peer_addr).await {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.link.tcp.session_error",
                        peer = %peer_addr,
                        %error,
                        "tcp link session ended with error"
                    );
                }
            });
        }
    }

    async fn serve_udp_listener(&self, bind_addr: SocketAddr) -> Result<(), String> {
        let socket = UdpSocket::bind(bind_addr)
            .await
            .map_err(|error| format!("udp link listener failed to bind {bind_addr}: {error}"))?;
        info!(
            target: "sdkwork.im",
            event = "im.link.udp.listen",
            bind = %bind_addr,
            node_id = %self.node_id,
            "realtime udp link listener started"
        );

        let socket = Arc::new(socket);
        let mut buffer = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES];
        loop {
            let (length, peer_addr) = socket
                .recv_from(&mut buffer)
                .await
                .map_err(|error| format!("udp link recv failed: {error}"))?;
            let runtime = self.clone();
            let datagram = buffer[..length].to_vec();
            let reply_socket = socket.clone();
            tokio::spawn(async move {
                if let Err(error) = runtime
                    .serve_udp_datagram(reply_socket.as_ref(), peer_addr, datagram.as_slice())
                    .await
                {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.link.udp.session_error",
                        peer = %peer_addr,
                        %error,
                        "udp link datagram handling failed"
                    );
                }
            });
        }
    }

    async fn serve_tcp_connection(
        &self,
        stream: TcpStream,
        peer_addr: SocketAddr,
    ) -> Result<(), String> {
        let (mut reader, mut writer) = tokio::io::split(stream);
        let auth = self
            .complete_stream_link_handshake(
                &mut reader,
                &mut writer,
                TransportBinding::Tcp1,
                LinkTransportKind::Tcp,
                peer_addr,
            )
            .await?;
        info!(
            target: "sdkwork.im",
            event = "im.link.tcp.authenticated",
            peer = %peer_addr,
            node_id = %self.node_id,
            "tcp link session entering realtime loop"
        );
        serve_realtime_framed_session(
            reader,
            writer,
            TransportBinding::Tcp1,
            auth.context,
            auth.device_id,
            auth.resume_after_seq,
            self.assembly.realtime_runtime(),
            self.route_registration.clone(),
        )
        .await
    }

    async fn complete_stream_link_handshake<R, W>(
        &self,
        reader: &mut R,
        writer: &mut W,
        binding: TransportBinding,
        transport_kind: LinkTransportKind,
        _peer_addr: SocketAddr,
    ) -> Result<StreamLinkAuthResult, String>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        let handshake_timeout = resolve_handshake_timeout();
        // Wrap the hello/auth_bind reads in a deadline so a peer that opens
        // a socket but never sends the handshake frames cannot hold a
        // connection permit indefinitely.
        let hello_envelope = tokio::time::timeout(
            handshake_timeout,
            read_framed_envelope(reader, binding.clone()),
        )
        .await
        .map_err(|_| format!("link handshake hello timed out after {}s", handshake_timeout.as_secs()))??;
        let hello = decode_control_frame(&hello_envelope, "hello")?;
        let ControlFrame::Hello(hello_frame) = hello else {
            return Err("first stream frame must be hello".into());
        };
        if hello_frame.binding != binding {
            let message = format!("stream link requires {} binding", binding.protocol_id());
            let _ = send_framed_error_and_close(
                writer,
                binding.clone(),
                "CCP_UNSUPPORTED_BINDING",
                message.as_str(),
            )
            .await;
            return Err(message);
        }

        let mut link_session = LinkSession::new(
            "_pending",
            "_pending",
            "user",
            "_pending",
            None,
            OutboundQueuePolicy::realtime_default(),
        );
        let hello_ack = link_session
            .negotiate_hello(&hello_frame)
            .map_err(map_hello_error)?;
        let resume_negotiated = hello_ack.capabilities.supports("session.resume");
        send_framed_control_frame(
            writer,
            binding.clone(),
            &ControlFrame::HelloAck(hello_ack),
        )
        .await?;

        let auth_envelope = tokio::time::timeout(
            handshake_timeout,
            read_framed_envelope(reader, binding.clone()),
        )
        .await
        .map_err(|_| format!("link handshake auth_bind timed out after {}s", handshake_timeout.as_secs()))??;
        let auth_bind = decode_control_frame(&auth_envelope, "auth_bind")?;
        let ControlFrame::AuthBind(auth_bind_frame) = auth_bind else {
            return Err("second stream frame must be auth_bind".into());
        };

        let auth = resolve_link_auth_context(&auth_bind_frame, &self.auth_resolver).await?;
        link_session = LinkSession::new(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            auth_bind_frame
                .device_id
                .as_deref()
                .or(auth.device_id.as_deref())
                .unwrap_or("_unknown"),
            auth_bind_frame
                .session_id
                .as_deref()
                .or(auth.session_id.as_deref()),
            OutboundQueuePolicy::realtime_default(),
        );
        if !link_session.matches_auth_bind(
            auth_bind_frame.principal_id.as_str(),
            auth_bind_frame.actor_kind.as_str(),
            auth_bind_frame.device_id.as_deref(),
            auth_bind_frame.session_id.as_deref(),
        ) {
            let _ = send_framed_error_and_close(
                writer,
                binding.clone(),
                "CCP_AUTH_FAILED",
                "auth_bind does not match authenticated context",
            )
            .await;
            return Err("auth_bind does not match authenticated context".into());
        }
        link_session.mark_authenticated();

        let device_id = auth_bind_frame
            .device_id
            .clone()
            .or_else(|| auth.device_id.clone())
            .ok_or_else(|| {
                format!(
                    "auth_bind.device_id is required for {} link sessions",
                    binding.protocol_id()
                )
            })?;

        // The route bind performs blocking Redis/Postgres IO. Run it on the
        // blocking thread pool so the async worker can service other
        // connections during the handshake.
        let route_registration = self.route_registration.clone();
        let handshake_auth = auth.clone();
        let handshake_device_id = device_id.clone();
        let protocol_id = binding.protocol_id().to_string();
        tokio::task::spawn_blocking(move || {
            route_registration.prepare_active_client_route(
                &handshake_auth,
                handshake_device_id.as_str(),
                protocol_id.as_str(),
                false,
            )
        })
        .await
        .map_err(|join_error| format!("link handshake blocking task failed: {join_error}"))?
        .map_err(map_api_error)?;

        let connection_key = LinkConnectionKey {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            device_id: device_id.clone(),
        };
        self.connection_registry.replace(LinkConnectionRecord {
            key: connection_key.clone(),
            transport: transport_kind,
            shard_id: self.shard_dispatcher.shard_for_key(&connection_key),
            session_id: auth.session_id.clone(),
        });

        let auth_ok = ControlFrame::AuthOk(AuthOkFrame {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            device_id: Some(device_id.clone()),
            session_id: auth.session_id.clone(),
        });
        send_framed_control_frame(writer, binding.clone(), &auth_ok).await?;

        let mut resume_after_seq = None;
        if resume_negotiated {
            let resume_envelope = read_framed_envelope(reader, binding.clone()).await?;
            let session_resume = decode_control_frame(&resume_envelope, "session_resume")?;
            let ControlFrame::SessionResume(session_resume_frame) = session_resume else {
                return Err("third stream frame must be session_resume when negotiated".into());
            };

            // Batch the two blocking Postgres setup calls into a single
            // spawn_blocking hop.
            let resume_runtime = self.assembly.realtime_runtime();
            let resume_tenant = auth.tenant_id.clone();
            let resume_org = auth.organization_id.clone();
            let resume_principal = auth.actor_id.clone();
            let resume_kind = auth.actor_kind.clone();
            let resume_device = device_id.clone();
            let checkpoint = tokio::task::spawn_blocking(
                move || -> Result<crate::realtime::RealtimeWindowCheckpoint, String> {
                    resume_runtime
                        .ensure_client_route_state_for_principal_kind(
                            resume_tenant.as_str(),
                            resume_org.as_str(),
                            resume_principal.as_str(),
                            resume_kind.as_str(),
                            resume_device.as_str(),
                        )
                        .map_err(|error| error.message)?;
                    resume_runtime
                        .window_checkpoint_for_principal_kind(
                            resume_tenant.as_str(),
                            resume_org.as_str(),
                            resume_principal.as_str(),
                            resume_kind.as_str(),
                            resume_device.as_str(),
                        )
                        .map_err(|error| error.message)
                },
            )
            .await
            .map_err(|join_error| format!("resume setup blocking task failed: {join_error}"))??;

            let directive = link_session
                .negotiate_session_resume(
                    &session_resume_frame,
                    checkpoint.latest_realtime_seq,
                    checkpoint.acked_through_seq,
                )
                .map_err(|error| format!("{}: {}", error.code(), error.message()))?;
            resume_after_seq = Some(
                directive
                    .catchup_after_seq
                    .max(checkpoint.trimmed_through_seq),
            );
            send_framed_control_frame(
                writer,
                binding,
                &ControlFrame::SessionResumed(directive.frame),
            )
            .await?;
        }

        Ok(StreamLinkAuthResult {
            context: auth,
            device_id,
            resume_after_seq,
        })
    }

    async fn serve_udp_datagram(
        &self,
        socket: &UdpSocket,
        peer_addr: SocketAddr,
        datagram: &[u8],
    ) -> Result<(), String> {
        let binding = UdpBinding::new();
        let codec = JsonEnvelopeCodec::new();
        let envelope = binding
            .decode_datagram(datagram, &codec)
            .map_err(|error| format!("udp datagram decode failed: {error}"))?;
        if envelope.binding != TransportBinding::Udp1 {
            return Err("udp datagram must use ccp/udp/1 binding".into());
        }

        let response = match decode_control_frame(&envelope, envelope.kind.as_str())? {
            ControlFrame::Hello(hello) => {
                if hello.binding != TransportBinding::Udp1 {
                    ControlFrame::Error(ErrorFrame {
                        code: "CCP_UNSUPPORTED_BINDING".into(),
                        message: "udp link requires ccp/udp/1 binding".into(),
                        retryable: false,
                    })
                } else {
                    let mut link_session = LinkSession::new(
                        "_pending",
                        "_pending",
                        "user",
                        "_pending",
                        None,
                        OutboundQueuePolicy::realtime_default(),
                    );
                    let hello_ack = link_session
                        .negotiate_hello(&hello)
                        .map_err(map_hello_error)?;
                    ControlFrame::HelloAck(hello_ack)
                }
            }
            ControlFrame::AuthBind(auth_bind_frame) => {
                let auth = resolve_link_auth_context(&auth_bind_frame, &self.auth_resolver).await?;
                let mut link_session = LinkSession::new(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    auth_bind_frame
                        .device_id
                        .as_deref()
                        .or(auth.device_id.as_deref())
                        .unwrap_or("_unknown"),
                    auth_bind_frame
                        .session_id
                        .as_deref()
                        .or(auth.session_id.as_deref()),
                    OutboundQueuePolicy::realtime_default(),
                );
                if !link_session.matches_auth_bind(
                    auth_bind_frame.principal_id.as_str(),
                    auth_bind_frame.actor_kind.as_str(),
                    auth_bind_frame.device_id.as_deref(),
                    auth_bind_frame.session_id.as_deref(),
                ) {
                    ControlFrame::Error(ErrorFrame {
                        code: "CCP_AUTH_FAILED".into(),
                        message: "auth_bind does not match authenticated context".into(),
                        retryable: false,
                    })
                } else if auth_bind_frame
                    .device_id
                    .as_ref()
                    .or(auth.device_id.as_ref())
                    .is_none()
                {
                    ControlFrame::Error(ErrorFrame {
                        code: "CCP_AUTH_FAILED".into(),
                        message: "auth_bind.device_id is required for udp link sessions".into(),
                        retryable: false,
                    })
                } else {
                    link_session.mark_authenticated();
                    let device_id = auth_bind_frame
                        .device_id
                        .clone()
                        .or_else(|| auth.device_id.clone())
                        .expect("device_id validated above");
                    if let Err(error) = self.route_registration.prepare_active_client_route(
                        &auth,
                        device_id.as_str(),
                        TransportBinding::Udp1.protocol_id(),
                        false,
                    ) {
                        ControlFrame::Error(ErrorFrame {
                            code: error.code.into(),
                            message: error.message,
                            retryable: false,
                        })
                    } else {
                        let connection_key = LinkConnectionKey {
                            tenant_id: auth.tenant_id.clone(),
                            principal_id: auth.actor_id.clone(),
                            device_id: device_id.clone(),
                        };
                        let shard_id = self.shard_dispatcher.shard_for_key(&connection_key);
                        self.connection_registry.replace(LinkConnectionRecord {
                            key: connection_key,
                            transport: LinkTransportKind::Udp,
                            shard_id,
                            session_id: auth_bind_frame.session_id.clone(),
                        });
                        ControlFrame::AuthOk(AuthOkFrame {
                            tenant_id: auth.tenant_id,
                            principal_id: auth.actor_id,
                            actor_kind: auth.actor_kind,
                            device_id: Some(device_id),
                            session_id: auth_bind_frame.session_id,
                        })
                    }
                }
            }
            other => ControlFrame::Error(ErrorFrame {
                code: "CCP_UNEXPECTED_CONTROL_FRAME".into(),
                message: format!("unexpected udp control frame: {}", other.frame_type()),
                retryable: false,
            }),
        };

        let response_envelope = encode_udp_control_envelope(&response)?;
        socket
            .send_to(response_envelope.as_slice(), peer_addr)
            .await
            .map_err(|error| format!("udp link response send failed: {error}"))?;
        info!(
            target: "sdkwork.im",
            event = "im.link.udp.responded",
            peer = %peer_addr,
            frame = response.frame_type(),
            "udp link datagram handled"
        );
        Ok(())
    }
}

pub fn spawn_link_transport_listeners(
    assembly: RealtimePlaneAssembly,
    node_id: impl Into<String>,
    auth_resolver: RealtimeAuthContextResolver,
) -> Vec<JoinHandle<()>> {
    LinkTransportRuntime::new(assembly, node_id, auth_resolver).spawn_listeners()
}

fn resolve_max_link_connections() -> usize {
    std::env::var(REALTIME_MAX_LINK_CONNECTIONS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(REALTIME_MAX_LINK_CONNECTIONS_DEFAULT)
}

fn resolve_optional_bind_addr(env_key: &str) -> Option<SocketAddr> {
    std::env::var(env_key)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<SocketAddr>().ok())
}

async fn resolve_link_auth_context(
    auth_bind: &AuthBindFrame,
    auth_resolver: &RealtimeAuthContextResolver,
) -> Result<AppContext, String> {
    let auth_token = auth_bind
        .auth_token
        .as_deref()
        .ok_or_else(|| "auth_bind.auth_token is required for link transports".to_owned())?;
    let access_token = auth_bind
        .access_token
        .as_deref()
        .ok_or_else(|| "auth_bind.access_token is required for link transports".to_owned())?;
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {auth_token}"))
            .map_err(|error| format!("invalid auth token header: {error}"))?,
    );
    headers.insert(
        "access-token",
        HeaderValue::from_str(access_token)
            .map_err(|error| format!("invalid access token header: {error}"))?,
    );
    if let Some(device_id) = auth_bind.device_id.as_deref() {
        headers.insert(
            "x-device-id",
            HeaderValue::from_str(device_id)
                .map_err(|error| format!("invalid device id header: {error}"))?,
        );
    }
    resolve_link_auth_context_from_headers(&headers, auth_resolver).await
}

async fn resolve_link_auth_context_from_headers(
    headers: &HeaderMap,
    auth_resolver: &RealtimeAuthContextResolver,
) -> Result<AppContext, String> {
    auth_resolver
        .resolve_from_headers(headers)
        .await
        .map_err(map_app_context_error)
}

fn map_app_context_error(error: AppContextError) -> String {
    format!("link auth context resolution failed: {error}")
}

fn map_api_error(error: ApiError) -> String {
    format!("{}: {}", error.code, error.message)
}

fn map_hello_error(error: LinkHelloError) -> String {
    format!("{}: {}", error.code(), error.message())
}

fn encode_udp_control_envelope(frame: &ControlFrame) -> Result<Vec<u8>, String> {
    let envelope = CcpEnvelope::new(
        ProtocolVersion::new("ccp", 1, 0),
        TransportBinding::Udp1,
        frame.frame_type(),
        format!("ccp.control.{}", frame.frame_type()),
        None,
        None,
        ["control"],
        None,
        serde_json::to_string(frame).map_err(|error| format!("control frame encode failed: {error}"))?,
    );
    UdpBinding::new()
        .encode(&envelope, &JsonEnvelopeCodec::new())
        .map(|message| message.payload)
        .map_err(|error| format!("udp envelope encode failed: {error}"))
}

fn decode_control_frame(envelope: &CcpEnvelope, expected_kind: &str) -> Result<ControlFrame, String> {
    if envelope.kind != expected_kind {
        return Err(format!(
            "expected control frame `{expected_kind}`, got `{}`",
            envelope.kind
        ));
    }
    serde_json::from_str(envelope.payload.as_str())
        .map_err(|error| format!("control frame decode failed: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RealtimeAuthContextResolver;
    use im_app_context::local_service_app_context;
    use sdkwork_im_ccp_binding_tcp::{
        TcpBinding, CCP_TCP_FRAME_HEADER_BYTES, CCP_TCP_MAX_FRAME_BYTES,
    };
    use sdkwork_im_ccp_control::{HelloFrame, SessionResumeFrame};
    use sdkwork_im_ccp_core::CapabilitySet;
    use sdkwork_im_runtime_link::{link_transport_kind_for_binding, tcp_framed_message_length};
    use tokio::io::AsyncReadExt;
    use tokio::net::UdpSocket;

    async fn read_tcp_envelope(stream: &mut TcpStream) -> Result<CcpEnvelope, String> {
        let mut header = [0_u8; CCP_TCP_FRAME_HEADER_BYTES];
        stream
            .read_exact(&mut header)
            .await
            .map_err(|error| format!("tcp frame header read failed: {error}"))?;
        let frame_length = tcp_framed_message_length(&header)
            .map_err(|error| format!("tcp frame header invalid: {error}"))?;
        let payload_length = frame_length - CCP_TCP_FRAME_HEADER_BYTES;
        if payload_length > CCP_TCP_MAX_FRAME_BYTES {
            return Err("tcp frame payload exceeds maximum size".into());
        }
        let mut payload = vec![0_u8; payload_length];
        stream
            .read_exact(&mut payload)
            .await
            .map_err(|error| format!("tcp frame payload read failed: {error}"))?;
        let mut framed = Vec::with_capacity(frame_length);
        framed.extend_from_slice(&header);
        framed.extend_from_slice(&payload);
        TcpBinding::new()
            .decode_framed(&framed, &JsonEnvelopeCodec::new())
            .map_err(|error| format!("tcp envelope decode failed: {error}"))
    }

    async fn send_tcp_control_frame(
        stream: &mut TcpStream,
        frame: &ControlFrame,
    ) -> Result<(), String> {
        let envelope = encode_tcp_control_envelope(frame)?;
        stream
            .write_all(envelope.as_slice())
            .await
            .map_err(|error| format!("tcp frame write failed: {error}"))?;
        stream
            .flush()
            .await
            .map_err(|error| format!("tcp frame flush failed: {error}"))
    }

    fn encode_tcp_control_envelope(frame: &ControlFrame) -> Result<Vec<u8>, String> {
        let binding = TransportBinding::Tcp1;
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            binding,
            frame.frame_type(),
            format!("ccp.control.{}", frame.frame_type()),
            None,
            None,
            ["control"],
            None,
            serde_json::to_string(frame)
                .map_err(|error| format!("control frame encode failed: {error}"))?,
        );
        TcpBinding::new()
            .encode(&envelope, &JsonEnvelopeCodec::new())
            .map(|message| message.framed)
            .map_err(|error| format!("tcp envelope encode failed: {error}"))
    }

    fn test_auth_bind_frame() -> AuthBindFrame {
        let context = local_service_app_context("100001", "1", "user", Some("d_tcp"), ["*"]);
        let headers = im_app_context::build_dual_token_headers_for_context(
            &context,
            context.permission_scope.iter(),
        );
        AuthBindFrame {
            principal_id: "1".into(),
            device_id: Some("d_tcp".into()),
            session_id: Some("s_demo".into()),
            actor_kind: "user".into(),
            auth_token: headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.strip_prefix("Bearer ").unwrap_or(value).to_owned()),
            access_token: headers
                .get("access-token")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
        }
    }

    fn encode_tcp_business_frame(schema: &str, kind: &str, payload: serde_json::Value) -> Vec<u8> {
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Tcp1,
            kind,
            schema,
            None,
            None,
            std::iter::empty::<String>(),
            None,
            payload.to_string(),
        );
        TcpBinding::new()
            .encode(&envelope, &JsonEnvelopeCodec::new())
            .expect("tcp business frame should encode")
            .framed
    }

    #[tokio::test]
    async fn test_tcp_link_listener_completes_hello_auth_handshake() {
        let assembly = RealtimePlaneAssembly::default();
        assembly.bind_node_runtime("node_tcp_test");
        let runtime = LinkTransportRuntime::new(assembly, "node_tcp_test", RealtimeAuthContextResolver::default());
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("tcp listener should bind");
        let address = listener.local_addr().expect("listener address");
        let server = tokio::spawn({
            let runtime = runtime.clone();
            async move {
                let (stream, _) = listener.accept().await.expect("tcp accept");
                runtime
                    .serve_tcp_connection(stream, address)
                    .await
                    .expect("tcp session should succeed");
            }
        });

        let mut client = TcpStream::connect(address)
            .await
            .expect("tcp client should connect");
        let hello = ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Tcp1,
            capabilities: CapabilitySet::from_iter(["control", "negotiation", "auth", "session"]),
            trace_id: Some("trace_tcp".into()),
        });
        send_tcp_control_frame(&mut client, &hello)
            .await
            .expect("hello should send");

        let hello_ack_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("hello ack should arrive");
        assert_eq!(hello_ack_envelope.kind, "hello_ack");

        let auth_bind = ControlFrame::AuthBind(test_auth_bind_frame());
        send_tcp_control_frame(&mut client, &auth_bind)
            .await
            .expect("auth bind should send");
        let auth_ok_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("auth ok should arrive");
        assert_eq!(auth_ok_envelope.kind, "auth_ok");

        let connected_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("realtime.connected should arrive");
        assert_eq!(connected_envelope.kind, "evt");
        let connected_payload: serde_json::Value =
            serde_json::from_str(connected_envelope.payload.as_str()).expect("connected json");
        assert_eq!(connected_payload["type"], "realtime.connected");

        client.shutdown().await.expect("client shutdown");
        server.abort();
    }

    #[tokio::test]
    async fn test_tcp_link_listener_completes_session_resume_when_negotiated() {
        let assembly = RealtimePlaneAssembly::default();
        assembly.bind_node_runtime("node_tcp_resume_test");
        let runtime = LinkTransportRuntime::new(assembly, "node_tcp_resume_test", RealtimeAuthContextResolver::default());
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("tcp listener should bind");
        let address = listener.local_addr().expect("listener address");
        let server = tokio::spawn({
            let runtime = runtime.clone();
            async move {
                let (stream, _) = listener.accept().await.expect("tcp accept");
                runtime
                    .serve_tcp_connection(stream, address)
                    .await
                    .expect("tcp session should succeed");
            }
        });

        let mut client = TcpStream::connect(address)
            .await
            .expect("tcp client should connect");
        let hello = ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Tcp1,
            capabilities: CapabilitySet::from_iter([
                "control",
                "negotiation",
                "auth",
                "session",
                "session.resume",
            ]),
            trace_id: Some("trace_tcp_resume".into()),
        });
        send_tcp_control_frame(&mut client, &hello)
            .await
            .expect("hello should send");

        let hello_ack_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("hello ack should arrive");
        assert_eq!(hello_ack_envelope.kind, "hello_ack");

        send_tcp_control_frame(&mut client, &ControlFrame::AuthBind(test_auth_bind_frame()))
            .await
            .expect("auth bind should send");
        let auth_ok_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("auth ok should arrive");
        assert_eq!(auth_ok_envelope.kind, "auth_ok");

        send_tcp_control_frame(
            &mut client,
            &ControlFrame::SessionResume(SessionResumeFrame {
                session_id: "s_demo".into(),
                last_acked_seq: None,
            }),
        )
        .await
        .expect("session_resume should send");
        let session_resumed = read_tcp_envelope(&mut client)
            .await
            .expect("session_resumed should arrive");
        assert_eq!(session_resumed.kind, "session_resumed");

        let connected_envelope = read_tcp_envelope(&mut client)
            .await
            .expect("realtime.connected should arrive");
        assert_eq!(connected_envelope.kind, "evt");
        let connected_payload: serde_json::Value =
            serde_json::from_str(connected_envelope.payload.as_str()).expect("connected json");
        assert_eq!(connected_payload["type"], "realtime.connected");

        client.shutdown().await.expect("client shutdown");
        server.abort();
    }

    #[tokio::test]
    async fn test_tcp_link_session_receives_live_push_event_window() {
        let assembly = RealtimePlaneAssembly::default();
        assembly.bind_node_runtime("node_tcp_push_test");
        let runtime = LinkTransportRuntime::new(assembly, "node_tcp_push_test", RealtimeAuthContextResolver::default());
        let delivery_runtime = runtime.assembly().realtime_runtime();
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("tcp listener should bind");
        let address = listener.local_addr().expect("listener address");
        let server = tokio::spawn({
            let runtime = runtime.clone();
            async move {
                let (stream, _) = listener.accept().await.expect("tcp accept");
                runtime
                    .serve_tcp_connection(stream, address)
                    .await
                    .expect("tcp session should succeed");
            }
        });

        let mut client = TcpStream::connect(address)
            .await
            .expect("tcp client should connect");
        let hello = ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Tcp1,
            capabilities: CapabilitySet::from_iter(["control", "negotiation", "auth", "session"]),
            trace_id: Some("trace_tcp_push".into()),
        });
        send_tcp_control_frame(&mut client, &hello)
            .await
            .expect("hello should send");
        let _hello_ack = read_tcp_envelope(&mut client)
            .await
            .expect("hello ack should arrive");

        send_tcp_control_frame(&mut client, &ControlFrame::AuthBind(test_auth_bind_frame()))
            .await
            .expect("auth bind should send");
        let _auth_ok = read_tcp_envelope(&mut client)
            .await
            .expect("auth ok should arrive");
        let connected = read_tcp_envelope(&mut client)
            .await
            .expect("realtime.connected should arrive");
        assert_eq!(connected.schema, "cc.realtime.connected.v1");

        client
            .write_all(encode_tcp_business_frame(
                "cc.realtime.subscriptions.sync.v1",
                "cmd",
                serde_json::json!({
                    "type": "subscriptions.sync",
                    "requestId": "req_tcp_push_1",
                    "items": [{
                        "scopeType": "conversation",
                        "scopeId": "c_demo",
                        "eventTypes": ["message.posted"]
                    }]
                }),
            ).as_slice())
            .await
            .expect("subscription sync should send");
        client.flush().await.expect("flush should succeed");

        let synced = read_tcp_envelope(&mut client)
            .await
            .expect("subscriptions.synced should arrive");
        assert_eq!(synced.schema, "cc.realtime.subscriptions.synced.v1");

        delivery_runtime
            .publish_scope_event_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                serde_json::json!({
                    "type": "message.posted",
                    "messageId": "msg_tcp_push_1",
                    "summary": "hello tcp push"
                })
                .to_string(),
                vec!["d_tcp".into()],
            )
            .expect("live publish should succeed");

        let pushed = read_tcp_envelope(&mut client)
            .await
            .expect("push window should arrive");
        assert_eq!(pushed.schema, "cc.realtime.event.window.v1");
        let pushed_payload: serde_json::Value =
            serde_json::from_str(pushed.payload.as_str()).expect("push json");
        assert_eq!(pushed_payload["type"], "event.window");
        assert_eq!(pushed_payload["reason"], "push");
        assert_eq!(pushed_payload["window"]["deviceId"], "d_tcp");
        assert_eq!(
            pushed_payload["window"]["items"].as_array().expect("items")[0]["eventType"],
            "message.posted"
        );

        client.shutdown().await.expect("client shutdown");
        server.abort();
    }

    #[tokio::test]
    async fn test_udp_link_listener_completes_auth_bind_and_registers_route() {
        let assembly = RealtimePlaneAssembly::default();
        assembly.bind_node_runtime("node_udp_auth_test");
        let runtime = LinkTransportRuntime::new(assembly, "node_udp_auth_test", RealtimeAuthContextResolver::default());
        let socket = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("udp socket should bind");
        let address = socket.local_addr().expect("udp address");
        let socket = Arc::new(socket);
        let server = tokio::spawn({
            let runtime = runtime.clone();
            let socket = socket.clone();
            async move {
                let mut buffer = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES];
                for _ in 0..2 {
                    let (length, peer) = socket.recv_from(&mut buffer).await.expect("udp recv");
                    runtime
                        .serve_udp_datagram(socket.as_ref(), peer, &buffer[..length])
                        .await
                        .expect("udp datagram should be handled");
                }
            }
        });

        let client = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("udp client should bind");
        let hello = ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Udp1,
            capabilities: CapabilitySet::from_iter(["control", "negotiation", "auth"]),
            trace_id: None,
        });
        client
            .send_to(
                encode_udp_control_envelope(&hello)
                    .expect("hello datagram")
                    .as_slice(),
                address,
            )
            .await
            .expect("hello datagram should send");
        let mut response = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES];
        let (length, _) = client.recv_from(&mut response).await.expect("hello ack");
        let hello_ack = UdpBinding::new()
            .decode_datagram(&response[..length], &JsonEnvelopeCodec::new())
            .expect("hello ack decode");
        assert_eq!(hello_ack.kind, "hello_ack");

        client
            .send_to(
                encode_udp_control_envelope(&ControlFrame::AuthBind(test_auth_bind_frame()))
                    .expect("auth bind datagram")
                    .as_slice(),
                address,
            )
            .await
            .expect("auth bind datagram should send");
        let (length, _) = client.recv_from(&mut response).await.expect("auth ok");
        let auth_ok = UdpBinding::new()
            .decode_datagram(&response[..length], &JsonEnvelopeCodec::new())
            .expect("auth ok decode");
        assert_eq!(auth_ok.kind, "auth_ok");

        let route = runtime
            .assembly()
            .realtime_cluster()
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_tcp")
            .expect("udp auth bind should register client route");
        assert_eq!(route.connection_kind, TransportBinding::Udp1.protocol_id());

        server.abort();
    }

    #[tokio::test]
    async fn test_udp_link_listener_responds_to_hello_datagram() {
        let runtime = LinkTransportRuntime::new(RealtimePlaneAssembly::default(), "node_udp_test", RealtimeAuthContextResolver::default());
        let socket = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("udp socket should bind");
        let address = socket.local_addr().expect("udp address");
        let socket = Arc::new(socket);
        let server = tokio::spawn({
            let runtime = runtime.clone();
            let socket = socket.clone();
            async move {
                let mut buffer = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES];
                let (length, peer) = socket.recv_from(&mut buffer).await.expect("udp recv");
                runtime
                    .serve_udp_datagram(socket.as_ref(), peer, &buffer[..length])
                    .await
                    .expect("udp datagram should be handled");
            }
        });

        let client = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("udp client should bind");
        let hello = ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Udp1,
            capabilities: CapabilitySet::from_iter(["control", "negotiation"]),
            trace_id: None,
        });
        let datagram = encode_udp_control_envelope(&hello).expect("hello datagram");
        client
            .send_to(datagram.as_slice(), address)
            .await
            .expect("hello datagram should send");

        let mut response = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES];
        let (length, _) = client.recv_from(&mut response).await.expect("udp response");
        let envelope = UdpBinding::new()
            .decode_datagram(&response[..length], &JsonEnvelopeCodec::new())
            .expect("response decode");
        assert_eq!(envelope.kind, "hello_ack");
        server.abort();
    }

    #[test]
    fn test_link_transport_kind_mapping_covers_tcp_udp_and_quic() {
        assert_eq!(
            link_transport_kind_for_binding(&TransportBinding::Tcp1),
            Some(LinkTransportKind::Tcp)
        );
        assert_eq!(
            link_transport_kind_for_binding(&TransportBinding::Udp1),
            Some(LinkTransportKind::Udp)
        );
        assert_eq!(
            link_transport_kind_for_binding(&TransportBinding::Quic1),
            Some(LinkTransportKind::Quic)
        );
    }
}
