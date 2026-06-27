//! CCP stream framing for QUIC bidirectional streams (`ccp/quic/1`).
//!
//! QUIC control and business envelopes reuse the same length-prefixed stream framing
//! as `ccp/tcp/1` per architecture doc 144.

use sdkwork_im_ccp_binding_tcp::{
    decode_length_prefixed_frame, encode_length_prefixed_frame,
    CCP_TCP_FRAME_HEADER_BYTES, CCP_TCP_MAX_FRAME_BYTES,
};
use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

pub const CCP_QUIC_PROTOCOL_ID: &str = "ccp/quic/1";
pub const CCP_QUIC_MAX_FRAME_BYTES: usize = CCP_TCP_MAX_FRAME_BYTES;
pub const CCP_QUIC_FRAME_HEADER_BYTES: usize = CCP_TCP_FRAME_HEADER_BYTES;

pub use sdkwork_im_ccp_binding_tcp::{
    decode_length_prefixed_frame as decode_quic_length_prefixed_frame,
    encode_length_prefixed_frame as encode_quic_length_prefixed_frame,
    framed_message_length as quic_framed_message_length,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuicBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub payload: Vec<u8>,
    pub framed: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QuicBinding;

impl QuicBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<QuicBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        let payload = codec.encode(envelope)?;
        let framed = encode_length_prefixed_frame(&payload)?;
        Ok(QuicBindingMessage {
            protocol_id: TransportBinding::Quic1.protocol_id(),
            content_type: codec.content_type(),
            payload,
            framed,
        })
    }

    pub fn decode<C>(
        &self,
        message: &QuicBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Quic1.protocol_id() {
            return Err(CodecError::new("quic binding protocol mismatch"));
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

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
    use sdkwork_im_ccp_core::ProtocolVersion;

    #[test]
    fn test_quic_binding_round_trips_length_prefixed_envelope() {
        let binding = QuicBinding::new();
        let codec = JsonEnvelopeCodec::new();
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Quic1,
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
            .expect("quic binding should encode");
        assert_eq!(message.protocol_id, CCP_QUIC_PROTOCOL_ID);
        assert_eq!(
            decode_length_prefixed_frame(&message.framed).expect("frame should decode"),
            message.payload.as_slice()
        );

        let decoded = binding
            .decode_framed(&message.framed, &codec)
            .expect("quic binding should decode framed payload");
        assert_eq!(decoded.binding, TransportBinding::Quic1);
        assert_eq!(decoded.kind, "hello");
    }
}
