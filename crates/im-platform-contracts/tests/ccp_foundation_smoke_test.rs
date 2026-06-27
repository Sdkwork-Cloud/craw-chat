use sdkwork_im_ccp_binding_http::HttpBinding;
use sdkwork_im_ccp_binding_mqtt::MqttBinding;
use sdkwork_im_ccp_binding_sse::SseBinding;
use sdkwork_im_ccp_binding_ws::{WsBinding, WsOpcode};
use sdkwork_im_ccp_codec::{CcpCodec, CodecError};
use sdkwork_im_ccp_codec_cbor::CborEnvelopeCodec;
use sdkwork_im_ccp_codec_json::JsonEnvelopeCodec;
use sdkwork_im_ccp_control::{AuthBindFrame, ControlFrame, HelloFrame};
use sdkwork_im_ccp_core::{
    CapabilitySet, CcpEnvelope, CcpRoute, CcpScope, ProtocolVersion, TransportBinding,
};
use sdkwork_im_ccp_registry::CcpRegistry;

struct StubEnvelopeCodec;

impl CcpCodec<CcpEnvelope> for StubEnvelopeCodec {
    fn codec_name(&self) -> &'static str {
        "stub-envelope"
    }

    fn content_type(&self) -> &'static str {
        "application/x.ccp-envelope+json"
    }

    fn encode(&self, value: &CcpEnvelope) -> Result<Vec<u8>, CodecError> {
        serde_json::to_vec(value).map_err(CodecError::new)
    }

    fn decode(&self, bytes: &[u8]) -> Result<CcpEnvelope, CodecError> {
        serde_json::from_slice(bytes).map_err(CodecError::new)
    }
}

#[test]
fn test_step03_ccp_foundation_tranche_can_compose_envelope_control_and_codec_types() {
    let capabilities = CapabilitySet::from_iter(["presence.sync", "conversation.timeline"]);
    let protocol = ProtocolVersion::new("ccp", 1, 0);
    let binding = TransportBinding::Ws1;

    let hello = ControlFrame::Hello(HelloFrame {
        protocol: protocol.clone(),
        binding: binding.clone(),
        capabilities: capabilities.clone(),
        trace_id: Some("trace_hello".into()),
    });

    let auth_bind = ControlFrame::AuthBind(AuthBindFrame {
        principal_id: "1".into(),
        device_id: Some("d_demo".into()),
        session_id: Some("s_demo".into()),
        actor_kind: "user".into(),
    });

    assert_eq!(hello.frame_type(), "hello");
    assert_eq!(auth_bind.frame_type(), "auth_bind");
    assert_eq!(binding.protocol_id(), "ccp/ws/1");
    assert!(capabilities.supports("presence.sync"));

    let envelope = CcpEnvelope::new(
        protocol,
        binding,
        hello.frame_type(),
        "ccp.control.hello",
        Some(CcpScope::new("conversation", "c_demo")),
        Some(CcpRoute::for_principal("100001", "1", Some("d_demo"))),
        ["control", "negotiation"],
        Some("trace_hello".into()),
        serde_json::to_string(&hello).expect("hello frame should serialize"),
    );

    let codec = StubEnvelopeCodec;
    let encoded = codec.encode(&envelope).expect("envelope should encode");
    let decoded = codec.decode(&encoded).expect("envelope should decode");

    assert_eq!(codec.codec_name(), "stub-envelope");
    assert_eq!(codec.content_type(), "application/x.ccp-envelope+json");
    assert_eq!(decoded.kind, "hello");
    assert_eq!(decoded.schema, "ccp.control.hello");
}

#[test]
fn test_step03_ccp_registry_and_builtin_codecs_freeze_control_plane_contracts() {
    let registry = CcpRegistry::control_plane_v1();

    assert!(registry.supports_binding(&TransportBinding::Http1));
    assert!(registry.supports_binding(&TransportBinding::Ws1));
    assert!(registry.supports_binding(&TransportBinding::Sse1));
    assert!(registry.supports_binding(&TransportBinding::Mqtt1));
    assert!(registry.supports_capability("control"));
    assert!(registry.supports_capability("negotiation"));

    let hello_schema = registry
        .schema("ccp.control.hello")
        .expect("hello schema should be registered");
    assert_eq!(hello_schema.kind, "hello");
    assert!(hello_schema.requires_capability("control"));
    assert!(hello_schema.requires_capability("negotiation"));
    assert!(hello_schema.supports_binding(&TransportBinding::Ws1));
    assert!(hello_schema.supports_binding(&TransportBinding::Sse1));

    let protocol = ProtocolVersion::new("ccp", 1, 0);
    let binding = TransportBinding::Sse1;
    let capabilities = CapabilitySet::from_iter(["control", "negotiation"]);
    let hello = ControlFrame::Hello(HelloFrame {
        protocol: protocol.clone(),
        binding: binding.clone(),
        capabilities: capabilities.clone(),
        trace_id: Some("trace_registry".into()),
    });
    let payload = serde_json::to_string(&hello).expect("hello frame should serialize");
    let envelope = CcpEnvelope::new(
        protocol,
        binding,
        hello.frame_type(),
        "ccp.control.hello",
        Some(CcpScope::new("tenant", "100001")),
        Some(CcpRoute::for_principal("100001", "1", Some("d_demo"))),
        ["control", "negotiation"],
        Some("trace_registry".into()),
        payload,
    );

    let json_codec = JsonEnvelopeCodec::new();
    let cbor_codec = CborEnvelopeCodec::new();

    let json_encoded = json_codec.encode(&envelope).expect("json roundtrip encode");
    let json_decoded = json_codec
        .decode(&json_encoded)
        .expect("json roundtrip decode");
    let cbor_encoded = cbor_codec.encode(&envelope).expect("cbor roundtrip encode");
    let cbor_decoded = cbor_codec
        .decode(&cbor_encoded)
        .expect("cbor roundtrip decode");

    assert_eq!(json_codec.codec_name(), "ccp-json");
    assert_eq!(json_codec.content_type(), "application/ccp+json");
    assert_eq!(cbor_codec.codec_name(), "ccp-cbor");
    assert_eq!(cbor_codec.content_type(), "application/ccp+cbor");
    assert_eq!(json_decoded, envelope);
    assert_eq!(cbor_decoded, envelope);
}

#[test]
fn test_step03_ccp_bindings_adapt_envelopes_across_http_ws_sse_and_mqtt() {
    let registry = CcpRegistry::control_plane_v1();
    let json_codec = JsonEnvelopeCodec::new();
    let cbor_codec = CborEnvelopeCodec::new();

    let http_envelope = build_hello_envelope(TransportBinding::Http1, "trace_http");
    let http_binding = HttpBinding::new();
    let http_message = http_binding
        .encode(&http_envelope, &json_codec)
        .expect("http binding should encode");
    assert_eq!(
        http_message.protocol_id,
        TransportBinding::Http1.protocol_id()
    );
    assert_eq!(http_message.content_type, json_codec.content_type());
    assert_eq!(
        http_binding
            .decode(&http_message, &json_codec)
            .expect("http binding should decode"),
        http_envelope
    );

    let ws_envelope = build_hello_envelope(TransportBinding::Ws1, "trace_ws");
    let ws_binding = WsBinding::new();
    let ws_message = ws_binding
        .encode(&ws_envelope, &json_codec)
        .expect("ws binding should encode");
    assert_eq!(ws_message.protocol_id, TransportBinding::Ws1.protocol_id());
    assert_eq!(ws_message.opcode, WsOpcode::Text);
    assert_eq!(
        ws_binding
            .decode(&ws_message, &json_codec)
            .expect("ws binding should decode"),
        ws_envelope
    );

    let sse_envelope = build_hello_envelope(TransportBinding::Sse1, "trace_sse");
    let sse_binding = SseBinding::new();
    let sse_event = sse_binding
        .encode(&sse_envelope, &json_codec)
        .expect("sse binding should encode");
    assert_eq!(sse_event.protocol_id, TransportBinding::Sse1.protocol_id());
    assert_eq!(sse_event.event, "hello");
    assert_eq!(sse_event.id.as_deref(), Some("trace_sse"));
    assert_eq!(
        sse_binding
            .decode(&sse_event, &json_codec)
            .expect("sse binding should decode"),
        sse_envelope
    );

    let mqtt_envelope = build_hello_envelope(TransportBinding::Mqtt1, "trace_mqtt");
    let mqtt_binding = MqttBinding::new();
    let mqtt_message = mqtt_binding
        .encode(&mqtt_envelope, &cbor_codec)
        .expect("mqtt binding should encode");
    assert_eq!(
        mqtt_message.protocol_id,
        TransportBinding::Mqtt1.protocol_id()
    );
    assert!(mqtt_message.topic.contains("100001"));
    assert!(mqtt_message.topic.ends_with("/hello"));
    assert_eq!(
        mqtt_binding
            .decode(&mqtt_message, &cbor_codec)
            .expect("mqtt binding should decode"),
        mqtt_envelope
    );

    let hello_schema = registry
        .schema("ccp.control.hello")
        .expect("hello schema should stay registered");
    assert!(hello_schema.supports_binding(&TransportBinding::Http1));
    assert!(hello_schema.supports_binding(&TransportBinding::Ws1));
    assert!(hello_schema.supports_binding(&TransportBinding::Sse1));
    assert!(hello_schema.supports_binding(&TransportBinding::Mqtt1));
}

fn build_hello_envelope(binding: TransportBinding, trace_id: &str) -> CcpEnvelope {
    let capabilities = CapabilitySet::from_iter(["control", "negotiation"]);
    let protocol = ProtocolVersion::new("ccp", 1, 0);
    let hello = ControlFrame::Hello(HelloFrame {
        protocol: protocol.clone(),
        binding: binding.clone(),
        capabilities,
        trace_id: Some(trace_id.to_owned()),
    });

    CcpEnvelope::new(
        protocol,
        binding,
        hello.frame_type(),
        "ccp.control.hello",
        Some(CcpScope::new("tenant", "100001")),
        Some(CcpRoute::for_principal("100001", "1", Some("d_demo"))),
        ["control", "negotiation"],
        Some(trace_id.to_owned()),
        serde_json::to_string(&hello).expect("hello frame should serialize"),
    )
}
