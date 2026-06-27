use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SseBindingEvent {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub event: String,
    pub id: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SseBinding;

impl SseBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<SseBindingEvent, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        Ok(SseBindingEvent {
            protocol_id: TransportBinding::Sse1.protocol_id(),
            content_type: codec.content_type(),
            event: envelope.kind.clone(),
            id: envelope.trace_id.clone(),
            data: codec.encode(envelope)?,
        })
    }

    pub fn decode<C>(&self, event: &SseBindingEvent, codec: &C) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if event.protocol_id != TransportBinding::Sse1.protocol_id() {
            return Err(CodecError::new("sse binding protocol mismatch"));
        }

        codec.decode(&event.data)
    }
}
