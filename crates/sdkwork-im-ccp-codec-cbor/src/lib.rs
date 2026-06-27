use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::CcpEnvelope;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CborEnvelopeCodec;

impl CborEnvelopeCodec {
    pub fn new() -> Self {
        Self
    }
}

impl CcpCodec<CcpEnvelope> for CborEnvelopeCodec {
    fn codec_name(&self) -> &'static str {
        "ccp-cbor"
    }

    fn content_type(&self) -> &'static str {
        "application/ccp+cbor"
    }

    fn encode(&self, value: &CcpEnvelope) -> Result<Vec<u8>, CodecError> {
        let mut encoded = Vec::new();
        ciborium::into_writer(value, &mut encoded).map_err(CodecError::new)?;
        Ok(encoded)
    }

    fn decode(&self, bytes: &[u8]) -> Result<CcpEnvelope, CodecError> {
        ciborium::from_reader(bytes).map_err(CodecError::new)
    }
}
