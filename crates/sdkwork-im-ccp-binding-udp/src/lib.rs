use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

pub const CCP_UDP_PROTOCOL_ID: &str = "ccp/udp/1";
pub const CCP_UDP_MAX_DATAGRAM_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdpBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UdpBinding;

impl UdpBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<UdpBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        let payload = codec.encode(envelope)?;
        validate_datagram_payload(&payload)?;
        Ok(UdpBindingMessage {
            protocol_id: TransportBinding::Udp1.protocol_id(),
            content_type: codec.content_type(),
            payload,
        })
    }

    pub fn decode<C>(
        &self,
        message: &UdpBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Udp1.protocol_id() {
            return Err(CodecError::new("udp binding protocol mismatch"));
        }

        validate_datagram_payload(&message.payload)?;
        codec.decode(&message.payload)
    }

    pub fn decode_datagram<C>(&self, datagram: &[u8], codec: &C) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        validate_datagram_payload(datagram)?;
        codec.decode(datagram)
    }
}

pub fn validate_datagram_payload(payload: &[u8]) -> Result<(), CodecError> {
    if payload.is_empty() {
        return Err(CodecError::new("udp datagram payload must not be empty"));
    }
    if payload.len() > CCP_UDP_MAX_DATAGRAM_BYTES {
        return Err(CodecError::new("udp datagram payload exceeds maximum size"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
    use sdkwork_im_ccp_core::ProtocolVersion;

    #[test]
    fn test_udp_binding_round_trips_single_datagram_envelope() {
        let binding = UdpBinding::new();
        let codec = JsonEnvelopeCodec::new();
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Udp1,
            "heartbeat",
            "ccp.control.heartbeat",
            None,
            None,
            std::iter::empty::<&str>(),
            None,
            "{}",
        );

        let message = binding
            .encode(&envelope, &codec)
            .expect("udp binding should encode");
        assert_eq!(message.protocol_id, CCP_UDP_PROTOCOL_ID);

        let decoded = binding
            .decode_datagram(&message.payload, &codec)
            .expect("udp binding should decode datagram");
        assert_eq!(decoded.binding, TransportBinding::Udp1);
        assert_eq!(decoded.kind, "heartbeat");
    }

    #[test]
    fn test_udp_binding_rejects_empty_datagram() {
        let error = validate_datagram_payload(&[]).expect_err("empty datagram");
        assert!(error.to_string().contains("must not be empty"));
    }
}
