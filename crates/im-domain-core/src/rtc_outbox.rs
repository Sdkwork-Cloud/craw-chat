//! RTC outbox recipient resolution shared by calls-service write path and gateway relay.

/// Resolve durable realtime recipients for an RTC outbox payload.
pub fn resolve_rtc_outbox_recipients(
    event_type: &str,
    payload: &serde_json::Value,
) -> Vec<(String, String)> {
    if let Some(ids) = payload
        .get("recipient_principal_ids")
        .and_then(|value| value.as_array())
    {
        let recipients = string_array_recipients(ids);
        if !recipients.is_empty() {
            return recipients;
        }
    }

    match event_type {
        "rtc.session.invited" => string_array_field_recipients(payload, "added_participant_ids"),
        "rtc.credential.refreshed" | "rtc.credentials.revoked" => optional_string_recipient(
            payload
                .get("participant_id")
                .and_then(|value| value.as_str()),
        ),
        _ => Vec::new(),
    }
}

fn string_array_recipients(values: &[serde_json::Value]) -> Vec<(String, String)> {
    values
        .iter()
        .filter_map(|value| value.as_str())
        .map(|id| (id.to_owned(), "user".to_owned()))
        .collect()
}

fn string_array_field_recipients(
    payload: &serde_json::Value,
    field: &str,
) -> Vec<(String, String)> {
    payload
        .get(field)
        .and_then(|value| value.as_array())
        .map(|values| string_array_recipients(values))
        .unwrap_or_default()
}

fn optional_string_recipient(value: Option<&str>) -> Vec<(String, String)> {
    value
        .map(|id| vec![(id.to_owned(), "user".to_owned())])
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_explicit_recipient_principal_ids() {
        let payload = serde_json::json!({
            "recipient_principal_ids": ["u_alice", "u_bob"],
            "added_participant_ids": ["u_carol"],
        });
        let recipients = resolve_rtc_outbox_recipients("rtc.session.invited", &payload);
        assert_eq!(
            recipients,
            vec![
                ("u_alice".to_owned(), "user".to_owned()),
                ("u_bob".to_owned(), "user".to_owned()),
            ]
        );
    }

    #[test]
    fn falls_back_to_added_participant_ids_for_invited() {
        let payload = serde_json::json!({
            "added_participant_ids": ["u_carol"],
        });
        let recipients = resolve_rtc_outbox_recipients("rtc.session.invited", &payload);
        assert_eq!(recipients, vec![("u_carol".to_owned(), "user".to_owned())]);
    }

    #[test]
    fn empty_for_unknown_event_without_fields() {
        assert!(
            resolve_rtc_outbox_recipients("rtc.session.unknown", &serde_json::json!({})).is_empty()
        );
    }

    #[test]
    fn empty_recipient_array_falls_back_for_invited() {
        let payload = serde_json::json!({
            "recipient_principal_ids": [],
            "added_participant_ids": ["u_carol"],
        });
        let recipients = resolve_rtc_outbox_recipients("rtc.session.invited", &payload);
        assert_eq!(recipients, vec![("u_carol".to_owned(), "user".to_owned())]);
    }
}
