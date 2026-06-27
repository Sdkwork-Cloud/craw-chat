use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use sdkwork_im_ccp_binding_ws::{CCP_WS_SUBPROTOCOL, WsBinding, WsBindingMessage, WsOpcode};
use sdkwork_im_ccp_codec::CcpCodec;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{
    AuthBindFrame, AuthOkFrame, ControlFrame, HelloAckFrame, HelloFrame, SessionResumeFrame,
    SessionResumedFrame,
};
use sdkwork_im_ccp_core::{CapabilitySet, CcpEnvelope, ProtocolVersion, TransportBinding};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::ClientRequestBuilder;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::command::CommandContext;
use crate::{CliError, build_websocket_url, resolve_access_token, resolve_authorization_header};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CcpSocketCodec {
    binding: WsBinding,
    codec: JsonEnvelopeCodec,
}

pub(crate) struct RealtimeSocket {
    pub(crate) inner: WsStream,
    pub(crate) ccp: CcpSocketCodec,
}

pub(crate) struct RealtimeSocketReader {
    pub(crate) inner: WsRead,
    pub(crate) ccp: CcpSocketCodec,
}

pub(crate) struct RealtimeSocketWriter {
    pub(crate) inner: WsWrite,
    pub(crate) ccp: CcpSocketCodec,
}

impl RealtimeSocket {
    pub(crate) fn split(self) -> (RealtimeSocketWriter, RealtimeSocketReader) {
        let (write, read) = self.inner.split();
        (
            RealtimeSocketWriter {
                inner: write,
                ccp: self.ccp,
            },
            RealtimeSocketReader {
                inner: read,
                ccp: self.ccp,
            },
        )
    }

    pub(crate) async fn close(&mut self) {
        let _ = self.inner.close(None).await;
    }
}

impl RealtimeSocketWriter {
    pub(crate) async fn close(&mut self) {
        let _ = self.inner.send(Message::Close(None)).await;
    }
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsRead = futures_util::stream::SplitStream<WsStream>;
type WsWrite = futures_util::stream::SplitSink<WsStream, Message>;

impl CcpSocketCodec {
    fn decode_envelope(&self, message: Message) -> Result<CcpEnvelope, CliError> {
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
            other => {
                return Err(CliError::runtime(format!(
                    "unsupported CCP websocket message: {other:?}"
                )));
            }
        };
        self.binding
            .decode(&binding_message, &self.codec)
            .map_err(|error| CliError::runtime(format!("failed to decode CCP envelope: {error}")))
    }

    pub(crate) fn decode_business_json(&self, message: Message) -> Result<Value, CliError> {
        let envelope = self.decode_envelope(message)?;
        if envelope.kind == "control" {
            let control: ControlFrame =
                serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                    CliError::runtime(format!(
                        "failed to decode CCP control frame payload: {error}"
                    ))
                })?;
            return Err(match control {
                ControlFrame::Error(frame) => CliError::runtime(format!(
                    "CCP control error {}: {}",
                    frame.code, frame.message
                )),
                other => CliError::runtime(format!(
                    "unexpected CCP control frame after handshake: {}",
                    other.frame_type()
                )),
            });
        }
        serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
            CliError::runtime(format!("failed to decode CCP business payload: {error}"))
        })
    }

    fn decode_control_frame(&self, message: Message) -> Result<ControlFrame, CliError> {
        let envelope = self.decode_envelope(message)?;
        serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
            CliError::runtime(format!("failed to decode CCP control frame: {error}"))
        })
    }

    pub(crate) fn encode_business_frame(
        &self,
        schema: &str,
        kind: &str,
        payload: &Value,
    ) -> Result<Message, CliError> {
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
        self.encode_envelope(&envelope)
    }

    fn encode_control_frame(&self, frame: &ControlFrame) -> Result<Message, CliError> {
        let schema = match frame {
            ControlFrame::Hello(_) => "cc.control.hello.v1",
            ControlFrame::HelloAck(_) => "cc.control.hello_ack.v1",
            ControlFrame::AuthBind(_) => "cc.control.auth_bind.v1",
            ControlFrame::AuthOk(_) => "cc.control.auth_ok.v1",
            ControlFrame::SessionResume(_) => "cc.control.session_resume.v1",
            ControlFrame::SessionResumed(_) => "cc.control.session_resumed.v1",
            ControlFrame::Heartbeat(_) => "cc.control.heartbeat.v1",
            ControlFrame::GoAway(_) => "cc.control.goaway.v1",
            ControlFrame::Error(_) => "cc.control.error.v1",
        };
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Ws1,
            "control",
            schema,
            None,
            None,
            Vec::<String>::new(),
            None,
            serde_json::to_string(frame).map_err(|error| {
                CliError::runtime(format!("failed to encode CCP control frame: {error}"))
            })?,
        );
        self.encode_envelope(&envelope)
    }

    fn encode_envelope(&self, envelope: &CcpEnvelope) -> Result<Message, CliError> {
        let message = self
            .binding
            .encode(envelope, &self.codec)
            .map_err(|error| {
                CliError::runtime(format!("failed to encode CCP envelope: {error}"))
            })?;
        Ok(match message.opcode {
            WsOpcode::Text => Message::Text(
                String::from_utf8(message.payload)
                    .expect("json websocket payload should remain utf8")
                    .into(),
            ),
            WsOpcode::Binary => Message::Binary(message.payload.into()),
        })
    }
}

pub(crate) async fn connect_realtime_socket(
    context: &CommandContext,
) -> Result<RealtimeSocket, CliError> {
    let ws_url = build_websocket_url(context.base_url.as_str(), REALTIME_WS)?;
    let mut request =
        ClientRequestBuilder::new(ws_url.parse().map_err(|error| {
            CliError::runtime(format!("failed to parse websocket url: {error}"))
        })?)
        .with_sub_protocol(CCP_WS_SUBPROTOCOL);
    let access_token = resolve_access_token(&context.auth)?;
    let authorization = resolve_authorization_header(&context.auth)?;
    request = request
        .with_header("authorization", authorization.as_str())
        .with_header("access-token", access_token.as_str());

    let (mut socket, response) = connect_async(request).await.map_err(|error| {
        format_realtime_connect_error(context.base_url.as_str(), &ws_url, error)
    })?;
    if !response
        .headers()
        .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
        .is_some_and(|value| value == CCP_WS_SUBPROTOCOL)
    {
        return Err(CliError::runtime(format!(
            "server did not negotiate CCP websocket subprotocol {CCP_WS_SUBPROTOCOL}; \
             verify sdkwork-im session-gateway is running and accepts sdkwork-im.ccp.ws.v1"
        )));
    }
    let ccp = CcpSocketCodec::default();
    complete_ccp_handshake(&mut socket, &ccp, context).await?;
    Ok(RealtimeSocket {
        inner: socket,
        ccp,
    })
}

fn format_realtime_connect_error(
    base_url: &str,
    ws_url: &str,
    error: tokio_tungstenite::tungstenite::Error,
) -> CliError {
    match error {
        tokio_tungstenite::tungstenite::Error::Io(io_error) => CliError::runtime(format!(
            "unable to connect realtime websocket to sdkwork-im service at {} using {}; verify the service is running and the --base-url is correct: {}",
            base_url, ws_url, io_error
        )),
        other => CliError::runtime(format!("failed to connect websocket: {other}")),
    }
}

async fn complete_ccp_handshake(
    socket: &mut WsStream,
    ccp: &CcpSocketCodec,
    context: &CommandContext,
) -> Result<(), CliError> {
    let hello = ControlFrame::Hello(HelloFrame {
        protocol: ProtocolVersion::new("ccp", 1, 0),
        binding: TransportBinding::Ws1,
        capabilities: CapabilitySet::from_iter(["payload.json", "session.resume"]),
        trace_id: None,
    });
    socket
        .send(ccp.encode_control_frame(&hello)?)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send CCP hello: {error}")))?;
    let hello_capabilities = match read_next_control_frame(socket, ccp).await? {
        ControlFrame::HelloAck(HelloAckFrame {
            accepted: true,
            protocol,
            binding,
            capabilities,
            ..
        }) if protocol.family == "ccp"
            && protocol.major == 1
            && binding == TransportBinding::Ws1 =>
        {
            capabilities
        }
        ControlFrame::HelloAck(_) => {
            return Err(CliError::runtime(
                "server rejected CCP hello negotiation or selected an incompatible binding",
            ));
        }
        ControlFrame::Error(frame) => {
            return Err(CliError::runtime(format!(
                "server rejected CCP hello: {}: {}",
                frame.code, frame.message
            )));
        }
        other => {
            return Err(CliError::runtime(format!(
                "expected hello_ack from websocket handshake, got {}",
                other.frame_type()
            )));
        }
    };

    let auth_bind = ControlFrame::AuthBind(AuthBindFrame {
        principal_id: context.auth.user_id.clone(),
        device_id: Some(context.auth.device_id.clone()),
        session_id: Some(context.auth.session_id.clone()),
        actor_kind: context.auth.actor_kind.clone(),
        auth_token: None,
        access_token: None,
    });
    socket
        .send(ccp.encode_control_frame(&auth_bind)?)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send CCP auth_bind: {error}")))?;
    let auth_ok = read_next_control_frame(socket, ccp).await?;
    match auth_ok {
        ControlFrame::AuthOk(AuthOkFrame {
            tenant_id,
            principal_id,
            actor_kind,
            device_id,
            session_id,
        }) if tenant_id == context.auth.tenant_id
            && principal_id == context.auth.user_id
            && actor_kind == context.auth.actor_kind
            && device_id.as_deref() == Some(context.auth.device_id.as_str())
            && session_id.as_deref() == Some(context.auth.session_id.as_str()) =>
        {
            Ok(())
        }
        ControlFrame::AuthOk(_) => Err(CliError::runtime(
            "server returned auth_ok with mismatched authority fields",
        )),
        ControlFrame::Error(frame) => Err(CliError::runtime(format!(
            "server rejected CCP auth_bind: {}: {}",
            frame.code, frame.message
        ))),
        other => Err(CliError::runtime(format!(
            "expected auth_ok from websocket handshake, got {}",
            other.frame_type()
        ))),
    }?;

    if hello_capabilities.supports("session.resume") {
        let session_resume = ControlFrame::SessionResume(SessionResumeFrame {
            session_id: context.auth.session_id.clone(),
            last_acked_seq: Some(0),
        });
        socket
            .send(ccp.encode_control_frame(&session_resume)?)
            .await
            .map_err(|error| {
                CliError::runtime(format!("failed to send CCP session_resume: {error}"))
            })?;

        match read_next_control_frame(socket, ccp).await? {
            ControlFrame::SessionResumed(SessionResumedFrame { session_id, .. })
                if session_id == context.auth.session_id => {}
            ControlFrame::SessionResumed(_) => {
                return Err(CliError::runtime(
                    "server returned session_resumed with mismatched session id",
                ));
            }
            ControlFrame::Error(frame) => {
                return Err(CliError::runtime(format!(
                    "server rejected CCP session_resume: {}: {}",
                    frame.code, frame.message
                )));
            }
            other => {
                return Err(CliError::runtime(format!(
                    "expected session_resumed from websocket handshake, got {}",
                    other.frame_type()
                )));
            }
        }
    }

    Ok(())
}

async fn read_next_control_frame(
    socket: &mut WsStream,
    ccp: &CcpSocketCodec,
) -> Result<ControlFrame, CliError> {
    loop {
        let message = if let Some(message) = timeout(Duration::from_secs(5), socket.next())
            .await
            .map_err(|_| CliError::runtime("timed out waiting for CCP control frame"))?
        {
            message
                .map_err(|error| CliError::runtime(format!("websocket receive failed: {error}")))?
        } else {
            return Err(CliError::runtime(
                "websocket closed before expected CCP control frame",
            ));
        };
        match message {
            Message::Ping(payload) => {
                socket.send(Message::Pong(payload)).await.map_err(|error| {
                    CliError::runtime(format!("failed to reply to websocket ping: {error}"))
                })?;
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                let reason = frame
                    .map(|frame| format!("code={} reason={}", u16::from(frame.code), frame.reason))
                    .unwrap_or_else(|| "without close frame".to_owned());
                return Err(CliError::runtime(format!(
                    "websocket closed before CCP control frame arrived: {reason}"
                )));
            }
            Message::Text(_) | Message::Binary(_) => return ccp.decode_control_frame(message),
            Message::Frame(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::Router;
    use axum::extract::ws::{Message as AxumMessage, WebSocketUpgrade};
    use axum::response::Response;
    use axum::routing::get;
    use tokio::net::TcpListener;

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
        (format!("http://127.0.0.1:{}", address.port()), handle)
    }

    fn encode_ccp_text(payload: Value, schema: &str, kind: &str) -> AxumMessage {
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
            .expect("ccp frame should encode");
        match message.opcode {
            WsOpcode::Text => AxumMessage::Text(
                String::from_utf8(message.payload)
                    .expect("json payload should remain utf8")
                    .into(),
            ),
            WsOpcode::Binary => AxumMessage::Binary(message.payload.into()),
        }
    }

    fn decode_ccp_text(message: AxumMessage) -> Value {
        let codec = JsonEnvelopeCodec::new();
        let binding = WsBinding::new();
        let binding_message = match message {
            AxumMessage::Text(text) => WsBindingMessage {
                protocol_id: TransportBinding::Ws1.protocol_id(),
                content_type: codec.content_type(),
                opcode: WsOpcode::Text,
                payload: text.to_string().into_bytes(),
            },
            AxumMessage::Binary(bytes) => WsBindingMessage {
                protocol_id: TransportBinding::Ws1.protocol_id(),
                content_type: codec.content_type(),
                opcode: WsOpcode::Binary,
                payload: bytes.to_vec(),
            },
            other => panic!("expected CCP text/binary frame, got {other:?}"),
        };
        let envelope = binding
            .decode(&binding_message, &codec)
            .expect("ccp frame should decode");
        serde_json::from_str(envelope.payload.as_str()).expect("ccp payload should be valid json")
    }

    #[tokio::test]
    async fn test_connect_realtime_socket_negotiates_ccp_subprotocol() {
        async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
            let ws = ws.protocols([CCP_WS_SUBPROTOCOL]);
            assert_eq!(ws.selected_protocol().unwrap(), CCP_WS_SUBPROTOCOL);
            ws.on_upgrade(|mut socket| async move {
                let hello = decode_ccp_text(
                    socket
                        .next()
                        .await
                        .expect("hello should arrive")
                        .expect("hello should decode"),
                );
                assert_eq!(hello["type"], "hello");
                socket
                    .send(encode_ccp_text(
                        serde_json::to_value(ControlFrame::HelloAck(HelloAckFrame {
                            protocol: ProtocolVersion::new("ccp", 1, 0),
                            binding: TransportBinding::Ws1,
                            capabilities: Default::default(),
                            accepted: true,
                        }))
                        .expect("hello ack should serialize"),
                        "cc.control.hello_ack.v1",
                        "control",
                    ))
                    .await
                    .expect("hello ack should send");
                let auth_bind = decode_ccp_text(
                    socket
                        .next()
                        .await
                        .expect("auth_bind should arrive")
                        .expect("auth_bind should decode"),
                );
                assert_eq!(auth_bind["type"], "auth_bind");
                socket
                    .send(encode_ccp_text(
                        serde_json::json!({
                            "type":"auth_ok",
                            "data":{
                                "tenant_id":"100001",
                                "principal_id":"1",
                                "actor_kind":"user",
                                "device_id":"d_demo",
                                "session_id":"s_demo"
                            }
                        }),
                        "cc.control.auth_ok.v1",
                        "control",
                    ))
                    .await
                    .expect("auth ok should send");
                socket
                    .send(encode_ccp_text(
                        serde_json::json!({
                            "type":"realtime.connected",
                            "deviceId":"d_demo"
                        }),
                        "cc.realtime.connected.v1",
                        "evt",
                    ))
                    .await
                    .expect("connected frame should send");
            })
        }

        let app = Router::new().route(REALTIME_WS, get(websocket_handler));
        let (base_url, handle) = spawn_server(app).await;
        let context = CommandContext {
            base_url,
            auth: crate::command::AuthInput {
                tenant_id: "100001".into(),
                user_id: "1".into(),
                actor_kind: "user".into(),
                session_id: "s_demo".into(),
                device_id: "d_demo".into(),
                permissions: Vec::new(),
                bearer_token: Some("test-token".into()),
            },
        };

        let mut socket = connect_realtime_socket(&context)
            .await
            .expect("ccp websocket should connect");
        let connected = crate::read_next_json_frame(&mut socket, Some(Duration::from_secs(5)))
            .await
            .expect("connected frame should decode");
        assert_eq!(connected["type"], "realtime.connected");
        assert_eq!(connected["deviceId"], "d_demo");

        socket.close().await;
        handle.abort();
        let _ = handle.await;
    }

    #[tokio::test]
    async fn test_connect_realtime_socket_rejects_missing_ccp_subprotocol() {
        async fn legacy_websocket_handler(ws: WebSocketUpgrade) -> Response {
            ws.on_upgrade(|_socket| async {})
        }

        let app = Router::new().route(REALTIME_WS, get(legacy_websocket_handler));
        let (base_url, handle) = spawn_server(app).await;
        let context = CommandContext {
            base_url,
            auth: crate::command::AuthInput {
                tenant_id: "100001".into(),
                user_id: "1".into(),
                actor_kind: "user".into(),
                session_id: "s_demo".into(),
                device_id: "d_demo".into(),
                permissions: Vec::new(),
                bearer_token: Some("test-token".into()),
            },
        };

        let result = connect_realtime_socket(&context).await;
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("legacy websocket upgrade must fail closed"),
        };
        assert!(
            error.to_string().contains(CCP_WS_SUBPROTOCOL)
                || error.to_string().to_ascii_lowercase().contains("subprotocol"),
            "error should explain missing CCP subprotocol negotiation, got {error}"
        );

        handle.abort();
        let _ = handle.await;
    }

    #[tokio::test]
    async fn test_connect_realtime_socket_completes_session_resume_before_connected_frame() {
        async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
            let ws = ws.protocols([CCP_WS_SUBPROTOCOL]);
            assert_eq!(ws.selected_protocol().unwrap(), CCP_WS_SUBPROTOCOL);
            ws.on_upgrade(|mut socket| async move {
                let hello = decode_ccp_text(
                    socket
                        .next()
                        .await
                        .expect("hello should arrive")
                        .expect("hello should decode"),
                );
                assert_eq!(hello["type"], "hello");
                socket
                    .send(encode_ccp_text(
                        serde_json::to_value(ControlFrame::HelloAck(HelloAckFrame {
                            protocol: ProtocolVersion::new("ccp", 1, 0),
                            binding: TransportBinding::Ws1,
                            capabilities: CapabilitySet::from_iter(["session.resume"]),
                            accepted: true,
                        }))
                        .expect("hello ack should serialize"),
                        "cc.control.hello_ack.v1",
                        "control",
                    ))
                    .await
                    .expect("hello ack should send");

                let auth_bind = decode_ccp_text(
                    socket
                        .next()
                        .await
                        .expect("auth_bind should arrive")
                        .expect("auth_bind should decode"),
                );
                assert_eq!(auth_bind["type"], "auth_bind");
                socket
                    .send(encode_ccp_text(
                        serde_json::to_value(ControlFrame::AuthOk(AuthOkFrame {
                            tenant_id: "100001".into(),
                            principal_id: "1".into(),
                            actor_kind: "user".into(),
                            device_id: Some("d_demo".into()),
                            session_id: Some("s_demo".into()),
                        }))
                        .expect("auth ok should serialize"),
                        "cc.control.auth_ok.v1",
                        "control",
                    ))
                    .await
                    .expect("auth ok should send");

                let session_resume = decode_ccp_text(
                    socket
                        .next()
                        .await
                        .expect("session_resume should arrive")
                        .expect("session_resume should decode"),
                );
                assert_eq!(session_resume["type"], "session_resume");
                assert_eq!(session_resume["data"]["session_id"], "s_demo");
                assert_eq!(session_resume["data"]["last_acked_seq"], 0);
                socket
                    .send(encode_ccp_text(
                        serde_json::to_value(ControlFrame::SessionResumed(SessionResumedFrame {
                            session_id: "s_demo".into(),
                            resumed: false,
                        }))
                        .expect("session resumed should serialize"),
                        "cc.control.session_resumed.v1",
                        "control",
                    ))
                    .await
                    .expect("session resumed should send");

                socket
                    .send(encode_ccp_text(
                        serde_json::json!({
                            "type":"realtime.connected",
                            "deviceId":"d_demo"
                        }),
                        "cc.realtime.connected.v1",
                        "evt",
                    ))
                    .await
                    .expect("connected frame should send");
            })
        }

        let app = Router::new().route(REALTIME_WS, get(websocket_handler));
        let (base_url, handle) = spawn_server(app).await;
        let context = CommandContext {
            base_url,
            auth: crate::command::AuthInput {
                tenant_id: "100001".into(),
                user_id: "1".into(),
                actor_kind: "user".into(),
                session_id: "s_demo".into(),
                device_id: "d_demo".into(),
                permissions: Vec::new(),
                bearer_token: Some("test-token".into()),
            },
        };

        let mut socket = connect_realtime_socket(&context)
            .await
            .expect("ccp websocket should connect");
        let connected = crate::read_next_json_frame(&mut socket, Some(Duration::from_secs(1)))
            .await
            .expect("connected frame should decode after session resume");
        assert_eq!(connected["type"], "realtime.connected");
        assert_eq!(connected["deviceId"], "d_demo");

        socket.close().await;
        handle.abort();
        let _ = handle.await;
    }
}
