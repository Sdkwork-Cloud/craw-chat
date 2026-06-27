use sdkwork_utils_rust::{hmac_sha256, secure_compare};
use serde::{Deserialize, Serialize};

pub const REALTIME_CLUSTER_BUS_SECRET_ENV: &str = "SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET";
pub const DEFAULT_REALTIME_NODE_ID: &str = "session_gateway_local_1";
const REALTIME_LINK_ALLOW_INSECURE_BIND_ENV: &str = "SDKWORK_IM_REALTIME_LINK_ALLOW_INSECURE_BIND";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SignedClusterRouteEventEnvelope {
    v: u8,
    sig: String,
    event: serde_json::Value,
}

pub fn resolve_cluster_bus_secret_from_env() -> Result<String, String> {
    std::env::var(REALTIME_CLUSTER_BUS_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{REALTIME_CLUSTER_BUS_SECRET_ENV} is required when the realtime cluster bus is enabled"
            )
        })
}

pub fn validate_realtime_node_id_for_cluster(
    node_id: &str,
    cluster_enabled: bool,
) -> Result<(), String> {
    if cluster_enabled && node_id == DEFAULT_REALTIME_NODE_ID {
        return Err(format!(
            "default node id '{DEFAULT_REALTIME_NODE_ID}' is not allowed when cluster bus or redis route store is enabled; set SDKWORK_IM_REALTIME_NODE_ID"
        ));
    }
    Ok(())
}

pub fn sign_cluster_route_event(
    secret: &str,
    event: &serde_json::Value,
) -> Result<String, String> {
    let event_json =
        serde_json::to_string(event).map_err(|error| format!("serialize cluster event: {error}"))?;
    let signature = compute_hmac_hex(secret, event_json.as_bytes());
    let envelope = SignedClusterRouteEventEnvelope {
        v: 1,
        sig: signature,
        event: event.clone(),
    };
    serde_json::to_string(&envelope)
        .map_err(|error| format!("serialize signed cluster event envelope: {error}"))
}

pub fn verify_and_extract_cluster_route_event(
    secret: &str,
    payload: &str,
) -> Result<serde_json::Value, String> {
    let envelope: SignedClusterRouteEventEnvelope = serde_json::from_str(payload).map_err(|_| {
        "invalid signed cluster route event envelope".to_string()
    })?;
    if envelope.v != 1 {
        return Err("unsupported cluster route event envelope version".to_string());
    }
    let event_json = serde_json::to_string(&envelope.event)
        .map_err(|error| format!("serialize cluster event for verification: {error}"))?;
    let expected = compute_hmac_hex(secret, event_json.as_bytes());
    if !secure_compare(&expected, &envelope.sig) {
        return Err("cluster route event signature mismatch".to_string());
    }
    Ok(envelope.event)
}

pub fn validate_link_bind_addr_for_cleartext_tokens(
    transport: &str,
    bind_addr: std::net::SocketAddr,
) -> Result<(), String> {
    if bind_addr.ip().is_loopback() {
        return Ok(());
    }
    if parse_env_truthy(std::env::var(REALTIME_LINK_ALLOW_INSECURE_BIND_ENV).ok()) {
        tracing::warn!(
            target: "sdkwork.im",
            event = "im.link.insecure_bind_allowed",
            transport = transport,
            bind_addr = %bind_addr,
            "cleartext link transport bound to a non-loopback address; tokens may be exposed"
        );
        return Ok(());
    }
    Err(format!(
        "{transport} link transport cannot bind to non-loopback address {bind_addr} without {REALTIME_LINK_ALLOW_INSECURE_BIND_ENV}=true; use QUIC/TLS or bind to loopback"
    ))
}

fn compute_hmac_hex(secret: &str, payload: &[u8]) -> String {
    hmac_sha256(payload, secret.as_bytes())
}

fn parse_env_truthy(value: Option<String>) -> bool {
    value.is_some_and(|value| {
        matches!(
            value.trim(),
            "1" | "true" | "TRUE" | "True" | "yes" | "YES" | "Yes"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn signed_cluster_event_roundtrip() {
        let event = json!({
            "tenant_id": "t1",
            "principal_id": "u1",
            "principal_kind": "user",
            "device_id": "d1",
            "scope_type": "conversation",
            "scope_id": "c1",
            "event_type": "message.new",
            "payload": "{}"
        });
        let signed = sign_cluster_route_event("secret", &event).expect("sign");
        let restored =
            verify_and_extract_cluster_route_event("secret", signed.as_str()).expect("verify");
        assert_eq!(restored, event);
    }

    #[test]
    fn signed_cluster_event_rejects_tampered_payload() {
        let event = json!({ "tenant_id": "t1" });
        let signed = sign_cluster_route_event("secret", &event).expect("sign");
        let tampered = signed.replace("\"t1\"", "\"t2\"");
        assert!(verify_and_extract_cluster_route_event("secret", tampered.as_str()).is_err());
    }

    #[test]
    fn default_node_id_rejected_when_cluster_enabled() {
        assert!(validate_realtime_node_id_for_cluster(DEFAULT_REALTIME_NODE_ID, true).is_err());
        assert!(validate_realtime_node_id_for_cluster("node-a", true).is_ok());
        assert!(validate_realtime_node_id_for_cluster(DEFAULT_REALTIME_NODE_ID, false).is_ok());
    }
}
