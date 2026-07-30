#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WebClient {
    H5,
    Pc,
}

const MOBILE_USER_AGENT_MARKERS: &[&str] = &[
    "android",
    "blackberry",
    "iemobile",
    "iphone",
    "ipad",
    "ipod",
    "mobile",
    "opera mini",
    "webos",
];

pub(crate) fn preferred_web_client(user_agent: Option<&str>) -> WebClient {
    let normalized = user_agent.unwrap_or_default().to_ascii_lowercase();
    if MOBILE_USER_AGENT_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        WebClient::H5
    } else {
        WebClient::Pc
    }
}

pub(crate) fn select_available_web_client(
    user_agent: Option<&str>,
    pc_available: bool,
    h5_available: bool,
) -> Option<WebClient> {
    match preferred_web_client(user_agent) {
        WebClient::H5 if h5_available => Some(WebClient::H5),
        WebClient::H5 if pc_available => Some(WebClient::Pc),
        WebClient::Pc if pc_available => Some(WebClient::Pc),
        WebClient::Pc if h5_available => Some(WebClient::H5),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_mobile_h5_and_desktop_pc() {
        assert_eq!(
            preferred_web_client(Some("Mozilla/5.0 (iPhone; Mobile)")),
            WebClient::H5
        );
        assert_eq!(
            preferred_web_client(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")),
            WebClient::Pc
        );
        assert_eq!(preferred_web_client(None), WebClient::Pc);
    }

    #[test]
    fn falls_back_to_the_available_client_in_both_directions() {
        assert_eq!(
            select_available_web_client(Some("iPhone Mobile"), true, false),
            Some(WebClient::Pc)
        );
        assert_eq!(
            select_available_web_client(Some("Windows NT"), false, true),
            Some(WebClient::H5)
        );
        assert_eq!(select_available_web_client(None, false, false), None);
    }
}
