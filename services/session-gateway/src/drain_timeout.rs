use std::time::Duration;

pub const SESSION_GATEWAY_DRAIN_TIMEOUT_SECS_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS";
const DEFAULT_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS: u64 = 45;
const MIN_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS: u64 = 5;
const MAX_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS: u64 = 300;

pub fn resolve_session_gateway_drain_timeout() -> Result<Duration, String> {
    let configured = std::env::var(SESSION_GATEWAY_DRAIN_TIMEOUT_SECS_ENV).ok();
    resolve_session_gateway_drain_timeout_from_value(configured.as_deref())
}

fn resolve_session_gateway_drain_timeout_from_value(
    value: Option<&str>,
) -> Result<Duration, String> {
    let seconds = match value.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => value.parse::<u64>().map_err(|error| {
            format!("invalid {SESSION_GATEWAY_DRAIN_TIMEOUT_SECS_ENV} `{value}`: {error}")
        })?,
        None => DEFAULT_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS,
    };
    if !(MIN_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS..=MAX_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS)
        .contains(&seconds)
    {
        return Err(format!(
            "{SESSION_GATEWAY_DRAIN_TIMEOUT_SECS_ENV} must be between {MIN_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS} and {MAX_SESSION_GATEWAY_DRAIN_TIMEOUT_SECS} seconds"
        ));
    }
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::resolve_session_gateway_drain_timeout_from_value;

    #[test]
    fn drain_timeout_is_bounded_and_fail_fast() {
        assert_eq!(
            resolve_session_gateway_drain_timeout_from_value(None).expect("default drain timeout"),
            Duration::from_secs(45)
        );
        assert_eq!(
            resolve_session_gateway_drain_timeout_from_value(Some("30"))
                .expect("configured drain timeout"),
            Duration::from_secs(30)
        );
        for invalid in ["0", "4", "301", "invalid"] {
            assert!(
                resolve_session_gateway_drain_timeout_from_value(Some(invalid)).is_err(),
                "invalid drain timeout must fail: {invalid}"
            );
        }
    }
}
