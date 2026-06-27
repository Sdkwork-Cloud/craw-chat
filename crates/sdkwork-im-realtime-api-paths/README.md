//! Canonical HTTP paths for the IM realtime open-api surface (`/im/v3/api/realtime/*` and presence).

pub const PREFIX: &str = "/im/v3/api/realtime";

pub const REALTIME_SUBSCRIPTIONS_SYNC: &str = "/im/v3/api/realtime/subscriptions/sync";
pub const REALTIME_WS: &str = "/im/v3/api/realtime/ws";
pub const REALTIME_EVENTS_ACK: &str = "/im/v3/api/realtime/events/ack";
pub const REALTIME_EVENTS: &str = "/im/v3/api/realtime/events";

pub const PRESENCE_HEARTBEAT: &str = "/im/v3/api/presence/heartbeat";
pub const PRESENCE_ME: &str = "/im/v3/api/presence/me";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realtime_paths_remain_stable_for_contracts() {
        assert_eq!(REALTIME_WS, "/im/v3/api/realtime/ws");
        assert_eq!(REALTIME_SUBSCRIPTIONS_SYNC, "/im/v3/api/realtime/subscriptions/sync");
        assert_eq!(PRESENCE_ME, "/im/v3/api/presence/me");
    }
}
