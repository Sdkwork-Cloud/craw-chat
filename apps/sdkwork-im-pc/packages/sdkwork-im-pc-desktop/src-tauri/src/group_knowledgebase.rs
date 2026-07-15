use serde::Deserialize;

const LAUNCH_TICKET_PREFIX: &str = "gklt_";
const LAUNCH_TICKET_PAYLOAD_LENGTH: usize = 43;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenGroupKnowledgebaseRequest {
    pub launch_ticket: String,
}

fn is_valid_launch_ticket(ticket: &str) -> bool {
    ticket.len() == LAUNCH_TICKET_PREFIX.len() + LAUNCH_TICKET_PAYLOAD_LENGTH
        && ticket.starts_with(LAUNCH_TICKET_PREFIX)
        && ticket[LAUNCH_TICKET_PREFIX.len()..]
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

fn build_group_knowledgebase_deep_link(ticket: &str) -> Result<String, String> {
    if !is_valid_launch_ticket(ticket) {
        return Err("invalid group knowledge base launch request".to_string());
    }

    Ok(format!("sdkwork-knowledgebase://group-launch/{ticket}"))
}

#[tauri::command]
pub fn sdkwork_chat_pc_open_group_knowledgebase(
    request: OpenGroupKnowledgebaseRequest,
) -> Result<(), String> {
    let deep_link = build_group_knowledgebase_deep_link(request.launch_ticket.as_str())?;
    open::that(deep_link).map_err(|_| "unable to open SDKWork Knowledgebase".to_string())
}

#[cfg(test)]
mod tests {
    use super::{build_group_knowledgebase_deep_link, is_valid_launch_ticket};

    fn valid_ticket() -> String {
        format!("gklt_{}", "a".repeat(43))
    }

    #[test]
    fn accepts_exact_group_launch_ticket_shape() {
        assert!(is_valid_launch_ticket(valid_ticket().as_str()));
    }

    #[test]
    fn rejects_uri_delimiters_and_whitespace() {
        for ticket in [
            "",
            "ticket/value",
            "ticket?query",
            "ticket#fragment",
            "ticket value",
        ] {
            assert!(!is_valid_launch_ticket(ticket));
        }
    }

    #[test]
    fn rejects_ticket_with_the_wrong_payload_length() {
        assert!(!is_valid_launch_ticket(
            format!("gklt_{}", "a".repeat(42)).as_str()
        ));
        assert!(!is_valid_launch_ticket(
            format!("gklt_{}", "a".repeat(44)).as_str()
        ));
    }

    #[test]
    fn builds_only_the_fixed_group_launch_protocol() {
        let ticket = valid_ticket();
        assert_eq!(
            build_group_knowledgebase_deep_link(ticket.as_str()).unwrap(),
            format!("sdkwork-knowledgebase://group-launch/{ticket}")
        );
    }
}
