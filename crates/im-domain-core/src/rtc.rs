use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type RtcSignalSenderMetadata = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtcSessionState {
    Started,
    Accepted,
    Rejected,
    Ended,
}

impl RtcSessionState {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Ended => "ended",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalSender {
    pub id: String,
    pub kind: String,
    pub member_id: Option<String>,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: RtcSignalSenderMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSession {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
    pub initiator_id: String,
    pub initiator_kind: String,
    pub provider_plugin_id: Option<String>,
    pub provider_session_id: Option<String>,
    pub access_endpoint: Option<String>,
    pub provider_region: Option<String>,
    pub state: RtcSessionState,
    pub signaling_stream_id: Option<String>,
    pub artifact_message_id: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalEvent {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub signal_seq: u64,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
    pub sender: RtcSignalSender,
    pub signaling_stream_id: Option<String>,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcStateRecord {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub session: RtcSession,
    pub signals: Vec<RtcSignalEvent>,
    pub updated_at: String,
}

impl RtcStateRecord {
    pub fn merge_monotonic(self, next: Self) -> Self {
        let session = if rtc_session_state_rank(&next.session.state)
            >= rtc_session_state_rank(&self.session.state)
        {
            next.session
        } else {
            self.session
        };
        let mut signals_by_seq = BTreeMap::new();
        for signal in self.signals.into_iter().chain(next.signals) {
            signals_by_seq.insert(signal.signal_seq, signal);
        }
        Self {
            tenant_id: next.tenant_id,
            rtc_session_id: next.rtc_session_id,
            session,
            signals: signals_by_seq.into_values().collect(),
            updated_at: max_rfc3339_string(self.updated_at, next.updated_at),
        }
    }
}

pub trait RtcStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, sdkwork_communication_rtc_service::RtcContractError>;

    fn save_state(
        &self,
        record: RtcStateRecord,
    ) -> Result<(), sdkwork_communication_rtc_service::RtcContractError>;

    fn clear_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<bool, sdkwork_communication_rtc_service::RtcContractError>;
}

pub fn encode_im_call_key_segments<const N: usize>(parts: [&str; N]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn rtc_session_state_rank(state: &RtcSessionState) -> u8 {
    match state {
        RtcSessionState::Started => 0,
        RtcSessionState::Rejected => 1,
        RtcSessionState::Accepted => 2,
        RtcSessionState::Ended => 3,
    }
}

fn max_rfc3339_string(left: String, right: String) -> String {
    match rfc3339_cmp(left.as_str(), right.as_str()) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => right,
        std::cmp::Ordering::Greater => left,
    }
}

fn rfc3339_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    parse_rfc3339_to_millis(left)
        .unwrap_or_default()
        .cmp(&parse_rfc3339_to_millis(right).unwrap_or_default())
}

fn parse_rfc3339_to_millis(value: &str) -> Option<i128> {
    let value = value.trim();
    let date_time = value.strip_suffix('Z')?;
    let (date, time) = date_time.split_once('T')?;
    let mut date_parts = date.split('-');
    let year = date_parts.next()?.parse::<i32>().ok()?;
    let month = date_parts.next()?.parse::<u32>().ok()?;
    let day = date_parts.next()?.parse::<u32>().ok()?;
    if date_parts.next().is_some() {
        return None;
    }

    let mut time_parts = time.split(':');
    let hour = time_parts.next()?.parse::<u32>().ok()?;
    let minute = time_parts.next()?.parse::<u32>().ok()?;
    let second_part = time_parts.next()?;
    if time_parts.next().is_some() {
        return None;
    }

    let (second_text, millis_text) = second_part
        .split_once('.')
        .map_or((second_part, "0"), |(second, fraction)| (second, fraction));
    let second = second_text.parse::<u32>().ok()?;
    let millis = fraction_to_millis(millis_text)?;
    let days = days_from_civil(year, month, day)? as i128;
    Some(
        (((days * 24 + hour as i128) * 60 + minute as i128) * 60 + second as i128) * 1000
            + millis as i128,
    )
}

fn fraction_to_millis(value: &str) -> Option<u32> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Some(0);
    }
    let mut normalized = value.chars().take(3).collect::<String>();
    while normalized.len() < 3 {
        normalized.push('0');
    }
    normalized.parse::<u32>().ok()
}

fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_adjusted = month as i32 + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_adjusted + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146_097 + doe - 719_468) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_record_merge_preserves_accepted_session_over_stale_reject() {
        let accepted = rtc_state_record(
            RtcSessionState::Accepted,
            "2026-05-06T00:00:03.000Z",
            vec![rtc_signal_event(1), rtc_signal_event(2)],
        );
        let stale_reject = rtc_state_record(
            RtcSessionState::Rejected,
            "2026-05-06T00:00:02.000Z",
            vec![rtc_signal_event(1)],
        );

        let merged = accepted.merge_monotonic(stale_reject);

        assert_eq!(merged.session.state, RtcSessionState::Accepted);
        assert_eq!(merged.updated_at, "2026-05-06T00:00:03.000Z");
        assert_eq!(
            merged
                .signals
                .iter()
                .map(|signal| signal.signal_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn state_record_merge_compares_updated_at_by_rfc3339_instant() {
        let whole_second = rtc_state_record(
            RtcSessionState::Accepted,
            "2026-05-06T00:00:00Z",
            vec![rtc_signal_event(1)],
        );
        let later_fraction = rtc_state_record(
            RtcSessionState::Accepted,
            "2026-05-06T00:00:00.100Z",
            vec![rtc_signal_event(2)],
        );

        let merged = whole_second.merge_monotonic(later_fraction);

        assert_eq!(merged.updated_at, "2026-05-06T00:00:00.100Z");
        assert_eq!(
            merged
                .signals
                .iter()
                .map(|signal| signal.signal_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    fn rtc_state_record(
        state: RtcSessionState,
        updated_at: &str,
        signals: Vec<RtcSignalEvent>,
    ) -> RtcStateRecord {
        RtcStateRecord {
            tenant_id: "100001".into(),
            rtc_session_id: "rtc_demo".into(),
            session: RtcSession {
                tenant_id: "100001".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                initiator_id: "1".into(),
                initiator_kind: "user".into(),
                provider_plugin_id: Some("webrtc".into()),
                provider_session_id: Some("ps_demo".into()),
                access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
                provider_region: Some("cn-shanghai".into()),
                state,
                signaling_stream_id: Some("st_demo".into()),
                artifact_message_id: None,
                started_at: "2026-05-06T00:00:00.000Z".into(),
                ended_at: None,
            },
            signals,
            updated_at: updated_at.into(),
        }
    }

    fn rtc_signal_event(signal_seq: u64) -> RtcSignalEvent {
        RtcSignalEvent {
            tenant_id: "100001".into(),
            rtc_session_id: "rtc_demo".into(),
            signal_seq,
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            signal_type: format!("rtc.signal.{signal_seq}"),
            schema_ref: Some("webrtc.signal.v1".into()),
            payload: format!("{{\"seq\":{signal_seq}}}"),
            sender: RtcSignalSender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            signaling_stream_id: Some("st_demo".into()),
            occurred_at: format!("2026-05-06T00:00:0{signal_seq}.000Z"),
        }
    }
}
