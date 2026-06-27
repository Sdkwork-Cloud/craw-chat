use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_core::{CcpEnvelope, TransportBinding};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MqttBindingMessage {
    pub protocol_id: &'static str,
    pub content_type: &'static str,
    pub topic: String,
    pub qos: u8,
    pub retain: bool,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqttBinding;

impl MqttBinding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode<C>(
        &self,
        envelope: &CcpEnvelope,
        codec: &C,
    ) -> Result<MqttBindingMessage, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        Ok(MqttBindingMessage {
            protocol_id: TransportBinding::Mqtt1.protocol_id(),
            content_type: codec.content_type(),
            topic: topic_for_envelope(envelope),
            qos: 1,
            retain: false,
            payload: codec.encode(envelope)?,
        })
    }

    pub fn decode<C>(
        &self,
        message: &MqttBindingMessage,
        codec: &C,
    ) -> Result<CcpEnvelope, CodecError>
    where
        C: CcpCodec<CcpEnvelope>,
    {
        if message.protocol_id != TransportBinding::Mqtt1.protocol_id() {
            return Err(CodecError::new("mqtt binding protocol mismatch"));
        }

        codec.decode(&message.payload)
    }
}

fn topic_for_envelope(envelope: &CcpEnvelope) -> String {
    match &envelope.route {
        Some(route) => format!("ccp/{}/{}", route.tenant_id, envelope.kind),
        None => format!("ccp/public/{}", envelope.kind),
    }
}
