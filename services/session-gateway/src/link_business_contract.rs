use sdkwork_im_ccp_core::CcpEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkClientBusinessFrame {
    pub frame_type: String,
    pub request_id: Option<String>,
}

pub fn expected_link_business_contract(frame_type: &str) -> Option<(&'static str, &'static str)> {
    match frame_type {
        "subscriptions.sync" => Some(("cmd", "cc.realtime.subscriptions.sync.v1")),
        "events.pull" => Some(("cmd", "cc.realtime.events.pull.v1")),
        "events.ack" => Some(("ack", "cc.realtime.events.ack.v1")),
        _ => None,
    }
}

pub fn validate_link_client_business_envelope(
    envelope: &CcpEnvelope,
    frame: &LinkClientBusinessFrame,
) -> Result<(), String> {
    if !matches!(envelope.kind.as_str(), "cmd" | "ack") {
        return Err(format!(
            "ccp client business frame kind must be cmd or ack, got {}",
            envelope.kind
        ));
    }

    let Some((expected_kind, expected_schema)) = expected_link_business_contract(&frame.frame_type)
    else {
        return Ok(());
    };

    if envelope.kind != expected_kind {
        return Err(format!(
            "frame `{}` requires ccp kind `{}`, got `{}`",
            frame.frame_type, expected_kind, envelope.kind
        ));
    }
    if envelope.schema != expected_schema {
        return Err(format!(
            "frame `{}` requires schema `{}`, got `{}`",
            frame.frame_type, expected_schema, envelope.schema
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_im_ccp_core::{CcpEnvelope, ProtocolVersion, TransportBinding};

    #[test]
    fn test_validate_link_client_business_envelope_rejects_kind_mismatch() {
        let envelope = CcpEnvelope::new(
            ProtocolVersion::new("ccp", 1, 0),
            TransportBinding::Tcp1,
            "ack",
            "cc.realtime.subscriptions.sync.v1",
            None,
            None,
            std::iter::empty::<String>(),
            None,
            "{}".to_owned(),
        );
        let frame = LinkClientBusinessFrame {
            frame_type: "subscriptions.sync".into(),
            request_id: None,
        };
        let error = validate_link_client_business_envelope(&envelope, &frame)
            .expect_err("subscriptions.sync must require cmd kind");
        assert!(error.contains("requires ccp kind `cmd`"));
    }
}
