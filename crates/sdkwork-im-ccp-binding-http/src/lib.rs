use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub schema: String,
    pub trace_id: Option<String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HttpBinding;

impl HttpBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<HttpBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        Ok(HttpBindingMessage {
            protocol_id: TransportBinding::Http1.protocol_id(),
            content_type: codec.content_type(),
            schema: envelope.schema.clone(),
            trace_id: envelope.trace_id.clone(),
            body: codec.encode(envelope)?,
        })
    }

    pub fn decode<C>(
        &self,
        message: &HttpBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Http1.protocol_id() {
            return Err(CodecError::new("http binding protocol mismatch"));
        }

        codec.decode(&message.body)
    }
}
