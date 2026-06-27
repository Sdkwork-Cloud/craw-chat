use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

pub const CCP_WS_SUBPROTOCOL: &str = "sdkwork-im.ccp.ws.v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WsOpcode {
    Text,
    Binary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WsBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub opcode: WsOpcode,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WsBinding;

impl WsBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<WsBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        let content_type = codec.content_type();
        Ok(WsBindingMessage {
            protocol_id: TransportBinding::Ws1.protocol_id(),
            content_type,
            opcode: opcode_for_content_type(content_type),
            payload: codec.encode(envelope)?,
        })
    }

    pub fn decode<C>(
        &self,
        message: &WsBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Ws1.protocol_id() {
            return Err(CodecError::new("ws binding protocol mismatch"));
        }

        codec.decode(&message.payload)
    }
}

fn opcode_for_content_type(content_type: &str) -> WsOpcode {
    if content_type.ends_with("+json") {
        WsOpcode::Text
    } else {
        WsOpcode::Binary
    }
}
