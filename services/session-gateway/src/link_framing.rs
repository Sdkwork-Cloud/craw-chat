use sdkwork_im_ccp_binding_quic::QuicBinding;
use sdkwork_im_ccp_binding_tcp::TcpBinding;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::ControlFrame;
use sdkwork_im_ccp_core::{CcpEnvelope, CcpRoute, ProtocolVersion, TransportBinding};
use sdkwork_im_runtime_link::{
    quic_framed_message_length, tcp_framed_message_length, CCP_QUIC_FRAME_HEADER_BYTES,
    CCP_QUIC_MAX_FRAME_BYTES, CCP_TCP_FRAME_HEADER_BYTES, CCP_TCP_MAX_FRAME_BYTES,
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct FramedStreamCcpCodec {
    transport: TransportBinding,
    codec: JsonEnvelopeCodec,
}

impl FramedStreamCcpCodec {
    pub fn new(transport: TransportBinding) -> Self {
        Self {
            transport,
            codec: JsonEnvelopeCodec::new(),
        }
    }

    pub fn encode_business(
        &self,
        route: &CcpRoute,
        kind: &str,
        schema: &str,
        payload: serde_json::Value,
    ) -> Result<Vec<u8>, String> {
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            self.transport.clone(),
            kind,
            schema,
            None,
            Some(route.clone()),
            std::iter::empty::<String>(),
            None,
            payload.to_string(),
        );
        self.encode_framed(&envelope)
    }

    pub fn encode_control(&self, route: &CcpRoute, frame: &ControlFrame) -> Result<Vec<u8>, String> {
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            self.transport.clone(),
            frame.frame_type(),
            format!("ccp.control.{}", frame.frame_type()),
            None,
            Some(route.clone()),
            ["control"],
            None,
            serde_json::to_string(frame).map_err(|error| format!("control encode failed: {error}"))?,
        );
        self.encode_framed(&envelope)
    }

    fn encode_framed(&self, envelope: &CcpEnvelope) -> Result<Vec<u8>, String> {
        match self.transport {
            TransportBinding::Tcp1 => TcpBinding::new()
                .encode(envelope, &self.codec)
                .map(|message| message.framed)
                .map_err(|error| format!("stream envelope encode failed: {error}")),
            TransportBinding::Quic1 => QuicBinding::new()
                .encode(envelope, &self.codec)
                .map(|message| message.framed)
                .map_err(|error| format!("stream envelope encode failed: {error}")),
            _ => Err("unsupported stream transport binding".into()),
        }
    }

    fn decode_framed(&self, framed: &[u8]) -> Result<CcpEnvelope, String> {
        match self.transport {
            TransportBinding::Tcp1 => TcpBinding::new()
                .decode_framed(framed, &self.codec)
                .map_err(|error| format!("stream envelope decode failed: {error}")),
            TransportBinding::Quic1 => QuicBinding::new()
                .decode_framed(framed, &self.codec)
                .map_err(|error| format!("stream envelope decode failed: {error}")),
            _ => Err("unsupported stream transport binding".into()),
        }
    }
}

pub fn encode_framed_control_envelope(
    binding: TransportBinding,
    frame: &ControlFrame,
) -> Result<Vec<u8>, String> {
    let envelope = CcpEnvelope::new(
        ProtocolVersion::new("ccp", 1, 0),
        binding,
        frame.frame_type(),
        format!("ccp.control.{}", frame.frame_type()),
        None,
        None,
        ["control"],
        None,
        serde_json::to_string(frame).map_err(|error| format!("control frame encode failed: {error}"))?,
    );
    match envelope.binding {
        TransportBinding::Tcp1 => TcpBinding::new()
            .encode(&envelope, &JsonEnvelopeCodec::new())
            .map(|message| message.framed)
            .map_err(|error| format!("stream envelope encode failed: {error}")),
        TransportBinding::Quic1 => QuicBinding::new()
            .encode(&envelope, &JsonEnvelopeCodec::new())
            .map(|message| message.framed)
            .map_err(|error| format!("stream envelope encode failed: {error}")),
        _ => Err("unsupported stream transport binding".into()),
    }
}

pub async fn read_framed_envelope<R>(
    reader: &mut R,
    transport: TransportBinding,
) -> Result<CcpEnvelope, String>
where
    R: AsyncRead + Unpin,
{
    let (header_bytes, max_frame_bytes, framed_message_length) = match transport {
        TransportBinding::Tcp1 => (
            CCP_TCP_FRAME_HEADER_BYTES,
            CCP_TCP_MAX_FRAME_BYTES,
            tcp_framed_message_length as fn(&[u8]) -> Result<usize, sdkwork_im_ccp_codec::CodecError>,
        ),
        TransportBinding::Quic1 => (
            CCP_QUIC_FRAME_HEADER_BYTES,
            CCP_QUIC_MAX_FRAME_BYTES,
            quic_framed_message_length as fn(&[u8]) -> Result<usize, sdkwork_im_ccp_codec::CodecError>,
        ),
        _ => return Err("unsupported stream transport binding".into()),
    };

    let mut header = vec![0_u8; header_bytes];
    reader
        .read_exact(&mut header)
        .await
        .map_err(|error| format!("stream frame header read failed: {error}"))?;
    let frame_length = framed_message_length(&header)
        .map_err(|error| format!("stream frame header invalid: {error}"))?;
    let payload_length = frame_length - header_bytes;
    if payload_length > max_frame_bytes {
        return Err("stream frame payload exceeds maximum size".into());
    }
    let mut payload = vec![0_u8; payload_length];
    reader
        .read_exact(&mut payload)
        .await
        .map_err(|error| format!("stream frame payload read failed: {error}"))?;
    let mut framed = Vec::with_capacity(frame_length);
    framed.extend_from_slice(&header);
    framed.extend_from_slice(&payload);

    FramedStreamCcpCodec::new(transport).decode_framed(&framed)
}

pub async fn write_framed_bytes<W>(writer: &mut W, bytes: &[u8]) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    writer
        .write_all(bytes)
        .await
        .map_err(|error| format!("stream write failed: {error}"))?;
    writer
        .flush()
        .await
        .map_err(|error| format!("stream flush failed: {error}"))
}

pub async fn send_framed_control_frame<W>(
    writer: &mut W,
    binding: TransportBinding,
    frame: &ControlFrame,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let envelope = encode_framed_control_envelope(binding, frame)?;
    write_framed_bytes(writer, envelope.as_slice()).await
}

pub async fn send_framed_error_and_close<W>(
    writer: &mut W,
    binding: TransportBinding,
    code: &str,
    message: &str,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    use sdkwork_im_ccp_control::{ControlFrame, ErrorFrame};
    let frame = ControlFrame::Error(ErrorFrame {
        code: code.into(),
        message: message.into(),
        retryable: false,
    });
    let _ = send_framed_control_frame(writer, binding, &frame).await;
    writer
        .shutdown()
        .await
        .map_err(|error| format!("stream shutdown failed: {error}"))
}
