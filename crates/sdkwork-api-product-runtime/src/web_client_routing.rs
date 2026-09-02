#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WebClient {
    H5,
    Pc,
}

/// Shared mobile User-Agent markers from SDKWORK_DEPLOY_SPEC §8 (Adaptive Web).
/// Keep in sync with the dev proxy contract in `@sdkwork/app-topology`
/// `tools/topology/lib/adaptive-web.mjs` (`MOBILE_USER_AGENT_PATTERN`).
/// `ipad` is deliberately absent: tablets default to the desktop renderer.
const MOBILE_USER_AGENT_MARKERS: &[&str] = &[
    "android",
    "blackberry",
    "huaweibrowser",
    "harmonyos",
    "iemobile",
    "iphone",
    "ipod",
    "micromessenger",
    "mobile",
    "opera mini",
    "quark",
    "ucbrowser",
    "webos",
];

/// Select the renderer preferred by the requesting browser.
///
/// Detection order mirrors the shared Adaptive Web contract:
/// 1. `Sec-CH-UA-Mobile: ?1` → mobile;
/// 2. iPad carve-out → desktop tablet default;
/// 3. mobile User-Agent markers → H5;
/// 4. otherwise (or missing User-Agent) → PC.
pub(crate) fn preferred_web_client(
    user_agent: Option<&str>,
    sec_ch_ua_mobile: Option<&str>,
) -> WebClient {
    if sec_ch_ua_mobile.unwrap_or_default().trim() == "?1" {
        return WebClient::H5;
    }
    let normalized = user_agent.unwrap_or_default().to_ascii_lowercase();
    if normalized.contains("ipad") {
        return WebClient::Pc;
    }
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
    sec_ch_ua_mobile: Option<&str>,
    pc_available: bool,
    h5_available: bool,
) -> Option<WebClient> {
    match preferred_web_client(user_agent, sec_ch_ua_mobile) {
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
            preferred_web_client(Some("Mozilla/5.0 (iPhone; Mobile)"), None),
            WebClient::H5
        );
        assert_eq!(
            preferred_web_client(Some("Mozilla/5.0 (Linux; Android 15; Mobile)"), None),
            WebClient::H5
        );
        assert_eq!(
            preferred_web_client(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"), None),
            WebClient::Pc
        );
        assert_eq!(preferred_web_client(None, None), WebClient::Pc);
    }

    #[test]
    fn detects_shared_contract_mobile_markers() {
        for user_agent in [
            "Mozilla/5.0 (Linux; Android 13) Mobile MicroMessenger/8.0.49",
            "Mozilla/5.0 (Linux; Android 15) HuaweiBrowser/15.0 Mobile",
            "Mozilla/5.0 (Linux; Android 14) HarmonyOS 4.2.0 Mobile",
            "Mozilla/5.0 (Linux; U; Android 13) UCBrowser/13.5 Mobile",
            "Mozilla/5.0 (Linux; Android 12) Quark/5.9 Mobile",
            "Mozilla/5.0 (Linux; Android 14) Chrome Mobile/126.0",
            "Mozilla/5.0 (BlackBerry; U; BlackBerry 9900) Mobile",
            "Mozilla/5.0 (Windows Phone 10.0; Android) IEMobile/11.0",
            "Mozilla/5.0 (Linux; U; Android 4.0) Opera Mini/7.1",
            "Mozilla/5.0 (hpwOS; webOS) AppleWebKit",
        ] {
            assert_eq!(
                preferred_web_client(Some(user_agent), None),
                WebClient::H5,
                "UA {user_agent:?} should select H5"
            );
        }
    }

    #[test]
    fn ipad_defaults_to_desktop_even_with_mobile_marker() {
        assert_eq!(
            preferred_web_client(
                Some("Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) Mobile/15E148"),
                None
            ),
            WebClient::Pc
        );
    }

    #[test]
    fn sec_ch_ua_mobile_marker_overrides_user_agent() {
        assert_eq!(
            preferred_web_client(
                Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
                Some("?1")
            ),
            WebClient::H5
        );
        assert_eq!(
            preferred_web_client(
                Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
                Some("?0")
            ),
            WebClient::Pc
        );
        assert_eq!(
            preferred_web_client(Some("Mozilla/5.0 (iPhone; Mobile)"), Some("?1")),
            WebClient::H5
        );
        assert_eq!(preferred_web_client(None, Some("?1")), WebClient::H5);
    }

    #[test]
    fn falls_back_to_the_available_client_in_both_directions() {
        assert_eq!(
            select_available_web_client(Some("iPhone Mobile"), None, true, false),
            Some(WebClient::Pc)
        );
        assert_eq!(
            select_available_web_client(Some("Windows NT"), None, false, true),
            Some(WebClient::H5)
        );
        assert_eq!(
            select_available_web_client(Some("Windows NT"), Some("?1"), true, false),
            Some(WebClient::Pc)
        );
        assert_eq!(select_available_web_client(None, None, false, false), None);
    }
}
