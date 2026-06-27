use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::CcpEnvelope;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JsonEnvelopeCodec;

impl JsonEnvelopeCodec {
    pub fn new() -> Self {
        Self
    }
}

impl CcpCodec<CcpEnvelope> for JsonEnvelopeCodec {
    fn codec_name(&self) -> &'static str {
        "ccp-json"
    }

    fn content_type(&self) -> &'static str {
        "application/ccp+json"
    }

    fn encode(&self, value: &CcpEnvelope) -> Result<Vec<u8>, CodecError> {
        serde_json::to_vec(value).map_err(CodecError::new)
    }

    fn decode(&self, bytes: &[u8]) -> Result<CcpEnvelope, CodecError> {
        serde_json::from_slice(bytes).map_err(CodecError::new)
    }
}
