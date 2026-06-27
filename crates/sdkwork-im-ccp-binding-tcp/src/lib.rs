use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

pub const CCP_TCP_PROTOCOL_ID: &str = "ccp/tcp/1";
pub const CCP_TCP_MAX_FRAME_BYTES: usize = 512 * 1024;
pub const CCP_TCP_FRAME_HEADER_BYTES: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TcpBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub payload: Vec<u8>,
    pub framed: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TcpBinding;

impl TcpBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<TcpBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        let payload = codec.encode(envelope)?;
        let framed = encode_length_prefixed_frame(&payload)?;
        Ok(TcpBindingMessage {
            protocol_id: TransportBinding::Tcp1.protocol_id(),
            content_type: codec.content_type(),
            payload,
            framed,
        })
    }

    pub fn decode<C>(
        &self,
        message: &TcpBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Tcp1.protocol_id() {
            return Err(CodecError::new("tcp binding protocol mismatch"));
        }

        codec.decode(&message.payload)
    }

    pub fn decode_framed<C>(&self, framed: &[u8], codec: &C) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        let payload = decode_length_prefixed_frame(framed)?;
        codec.decode(payload)
    }
}

pub fn encode_length_prefixed_frame(payload: &[u8]) -> Result<Vec<u8>, CodecError> {
    if payload.is_empty() {
        return Err(CodecError::new("tcp frame payload must not be empty"));
    }
    if payload.len() > CCP_TCP_MAX_FRAME_BYTES {
        return Err(CodecError::new("tcp frame payload exceeds maximum size"));
    }

    let length = u32::try_from(payload.len())
        .map_err(|_| CodecError::new("tcp frame payload length overflow"))?;
    let mut framed = Vec::with_capacity(CCP_TCP_FRAME_HEADER_BYTES + payload.len());
    framed.extend_from_slice(&length.to_be_bytes());
    framed.extend_from_slice(payload);
    Ok(framed)
}

pub fn decode_length_prefixed_frame(framed: &[u8]) -> Result<&[u8], CodecError> {
    if framed.len() < CCP_TCP_FRAME_HEADER_BYTES {
        return Err(CodecError::new("tcp frame is shorter than length header"));
    }

    let length = u32::from_be_bytes([
        framed[0],
        framed[1],
        framed[2],
        framed[3],
    ]) as usize;
    if length == 0 {
        return Err(CodecError::new("tcp frame payload must not be empty"));
    }
    if length > CCP_TCP_MAX_FRAME_BYTES {
        return Err(CodecError::new("tcp frame payload exceeds maximum size"));
    }

    let payload_start = CCP_TCP_FRAME_HEADER_BYTES;
    let payload_end = payload_start
        .checked_add(length)
        .ok_or_else(|| CodecError::new("tcp frame payload length overflow"))?;
    if framed.len() < payload_end {
        return Err(CodecError::new("tcp frame payload is incomplete"));
    }

    Ok(&framed[payload_start..payload_end])
}

pub fn framed_message_length(framed: &[u8]) -> Result<usize, CodecError> {
    if framed.len() < CCP_TCP_FRAME_HEADER_BYTES {
        return Err(CodecError::new("tcp frame is shorter than length header"));
    }

    let payload_length = u32::from_be_bytes([
        framed[0],
        framed[1],
        framed[2],
        framed[3],
    ]) as usize;
    if payload_length == 0 {
        return Err(CodecError::new("tcp frame payload must not be empty"));
    }
    if payload_length > CCP_TCP_MAX_FRAME_BYTES {
        return Err(CodecError::new("tcp frame payload exceeds maximum size"));
    }

    payload_length
        .checked_add(CCP_TCP_FRAME_HEADER_BYTES)
        .ok_or_else(|| CodecError::new("tcp frame length overflow"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
    use sdkwork_im_ccp_core::ProtocolVersion;

    #[test]
    fn test_tcp_binding_round_trips_length_prefixed_envelope() {
        let binding = TcpBinding::new();
        let codec = JsonEnvelopeCodec::new();
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Tcp1,
            "hello",
            "ccp.control.hello",
            None,
            None,
            std::iter::empty::<&str>(),
            None,
            "{}",
        );

        let message = binding
            .encode(&envelope, &codec)
            .expect("tcp binding should encode");
        assert_eq!(message.protocol_id, CCP_TCP_PROTOCOL_ID);
        assert_eq!(
            decode_length_prefixed_frame(&message.framed).expect("frame should decode"),
            message.payload.as_slice()
        );

        let decoded = binding
            .decode_framed(&message.framed, &codec)
            .expect("tcp binding should decode framed payload");
        assert_eq!(decoded.binding, TransportBinding::Tcp1);
        assert_eq!(decoded.kind, "hello");
    }

    #[test]
    fn test_tcp_binding_rejects_oversized_frame() {
        let payload = vec![0_u8; CCP_TCP_MAX_FRAME_BYTES + 1];
        let error = encode_length_prefixed_frame(&payload).expect_err("oversized frame");
        assert!(error.to_string().contains("maximum size"));
    }
}
