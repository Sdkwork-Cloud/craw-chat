use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type SessionEpoch = u64;
pub type RtcSignalSenderMetadata = BTreeMap<String, String>;

/// RTC call session lifecycle state.
///
/// The state machine is split into non-terminal (active) states and terminal
/// (final) states. Terminal states are irreversible: once a session enters a
/// terminal state, it can never transition back to an active state.
///
/// ```text
/// initiating ─▶ ringing ─┬─▶ connecting ─▶ connected ─┬─▶ ended
///                         │                  │         │
///                         │                  └─▶ on_hold └─▶ reconnecting ─▶ connected
///                         │
///                         ├─▶ canceled
///                         ├─▶ timeout
///                         └─▶ rejected
///
/// started (legacy alias for initiating/ringing)
/// accepted (legacy alias for connecting)
/// failed (reachable from any active state on technical failure)
/// ```
///
/// Legacy states (`started`, `accepted`) are retained for backward
/// compatibility with existing migrations and clients. New code should prefer
/// the explicit lifecycle states.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtcSessionState {
    // Legacy states (kept for backward compatibility with existing data).
    Started,
    Accepted,
    Rejected,
    Ended,
    // Explicit lifecycle states (migration 0008).
    Initiating,
    Ringing,
    Connecting,
    Connected,
    OnHold,
    Reconnecting,
    Canceled,
    Failed,
    Timeout,
}

impl RtcSessionState {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Ended => "ended",
            Self::Initiating => "initiating",
            Self::Ringing => "ringing",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::OnHold => "on_hold",
            Self::Reconnecting => "reconnecting",
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.as_wire_value()
    }

    /// Parse a wire-value string into a `RtcSessionState`.
    ///
    /// Returns `None` for unknown values to allow forward compatibility
    /// with future states (the caller can decide whether to reject or
    /// treat as an unknown active state).
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "started" => Some(Self::Started),
            "accepted" => Some(Self::Accepted),
            "rejected" => Some(Self::Rejected),
            "ended" => Some(Self::Ended),
            "initiating" => Some(Self::Initiating),
            "ringing" => Some(Self::Ringing),
            "connecting" => Some(Self::Connecting),
            "connected" => Some(Self::Connected),
            "on_hold" => Some(Self::OnHold),
            "reconnecting" => Some(Self::Reconnecting),
            "canceled" => Some(Self::Canceled),
            "failed" => Some(Self::Failed),
            "timeout" => Some(Self::Timeout),
            _ => None,
        }
    }

    /// Terminal states are irreversible final states.
    /// Once a session enters a terminal state, no further state transitions
    /// are permitted and signals cannot be posted.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Rejected | Self::Ended | Self::Canceled | Self::Failed | Self::Timeout
        )
    }

    /// Active (non-terminal) states where the session can still transition
    /// and accept signals.
    pub fn is_active(&self) -> bool {
        !self.is_terminal()
    }

    /// States that represent a connected media flow (call in progress).
    pub fn is_media_connected(&self) -> bool {
        matches!(
            self,
            Self::Connected | Self::OnHold | Self::Reconnecting
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionParticipants {
    #[serde(default)]
    pub invited_ids: Vec<String>,
    #[serde(default)]
    pub accepted_ids: Vec<String>,
}

/// Signal rate tracker with sliding window algorithm for rate limiting.
///
/// This implements a sliding window counter that tracks signal events within
/// a configurable time window. It prevents signal flooding attacks by limiting
/// the number of signals a participant can send within the window period.
///
/// # Sliding Window Algorithm
///
/// The tracker uses a two-bucket sliding window for accurate rate calculation:
/// - Current bucket: signals in the current window slice
/// - Previous bucket: signals from the previous window slice
/// - Rate = current + previous * (1 - elapsed/window_size)
///
/// This provides smoother rate limiting than fixed windows while being
/// memory-efficient.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalRateTracker {
    /// Total signal count in the current window.
    #[serde(default)]
    pub signal_count: u32,
    /// ISO 8601 timestamp when the current window started.
    pub window_start: Option<String>,
    /// Signals from the previous window slice (for sliding window calculation).
    #[serde(default)]
    pub previous_count: u32,
    /// ISO 8601 timestamp when the previous window started.
    pub previous_window_start: Option<String>,
}

impl SignalRateTracker {
    /// Default window size in seconds (60 seconds).
    pub const DEFAULT_WINDOW_SECS: u64 = 60;
    /// Default maximum signals per window (100 signals per minute).
    pub const DEFAULT_MAX_SIGNALS: u32 = 100;
    
    /// Create a new rate tracker with default settings.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if a new signal is allowed within the rate limit.
    ///
    /// # Arguments
    ///
    /// * `max_signals` - Maximum signals allowed in the window
    /// * `window_secs` - Window size in seconds
    /// * `current_time` - Current ISO 8601 timestamp
    ///
    /// # Returns
    ///
    /// `true` if the signal is allowed, `false` if rate limit exceeded
    pub fn check_rate_limit(
        &self,
        max_signals: u32,
        window_secs: u64,
        current_time: &str,
    ) -> bool {
        let current_count = self.calculate_sliding_count(window_secs, current_time);
        current_count < max_signals
    }
    
    /// Record a signal event, updating the sliding window counters.
    ///
    /// # Arguments
    ///
    /// * `window_secs` - Window size in seconds
    /// * `current_time` - Current ISO 8601 timestamp
    ///
    /// # Returns
    ///
    /// The new signal count after recording
    pub fn record_signal(&mut self, window_secs: u64, current_time: &str) -> u32 {
        // Parse current window start time
        let current_window_start = self.window_start.as_deref()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.timestamp());
        
        let now_ts = chrono::DateTime::parse_from_rfc3339(current_time)
            .map(|dt| dt.timestamp())
            .unwrap_or(0);
        
        // Check if we need to slide the window
        if let Some(window_ts) = current_window_start {
            let elapsed_secs = now_ts.saturating_sub(window_ts) as u64;
            
            if elapsed_secs >= window_secs * 2 {
                // Window completely expired - reset counters
                self.previous_count = 0;
                self.previous_window_start = self.window_start.clone();
                self.signal_count = 1;
                self.window_start = Some(current_time.to_owned());
            } else if elapsed_secs >= window_secs {
                // Slide to new window - current becomes previous
                self.previous_count = self.signal_count;
                self.previous_window_start = self.window_start.clone();
                self.signal_count = 1;
                self.window_start = Some(current_time.to_owned());
            } else {
                // Still in current window - increment counter
                self.signal_count = self.signal_count.saturating_add(1);
            }
        } else {
            // No previous window - start new one
            self.signal_count = 1;
            self.window_start = Some(current_time.to_owned());
        }
        
        self.signal_count
    }
    
    /// Calculate the sliding window count using the standard algorithm.
    ///
    /// This provides accurate rate limiting that doesn't suffer from the
    /// "boundary problem" of fixed window rate limiters.
    pub fn calculate_sliding_count(&self, window_secs: u64, current_time: &str) -> u32 {
        let now_ts = chrono::DateTime::parse_from_rfc3339(current_time)
            .map(|dt| dt.timestamp())
            .unwrap_or(0);
        
        // Get current window start time
        let current_window_ts = self.window_start.as_deref()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.timestamp())
            .unwrap_or(now_ts);
        
        // Get previous window start time
        let previous_window_ts = self.previous_window_start.as_deref()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.timestamp());
        
        let elapsed_in_window = now_ts.saturating_sub(current_window_ts) as u64;
        
        // If previous window exists and is within 2x window size, use sliding calculation
        if let Some(prev_ts) = previous_window_ts {
            let window_gap = now_ts.saturating_sub(prev_ts) as u64;
            
            if window_gap < window_secs * 2 {
                // Calculate weight for previous window based on overlap
                let prev_elapsed = now_ts.saturating_sub(prev_ts) as u64;
                let prev_weight = if prev_elapsed >= window_secs {
                    // Previous window is fully outside - calculate overlap ratio
                    let overlap = prev_elapsed.saturating_sub(window_secs);
                    1.0 - (overlap as f64 / window_secs as f64)
                } else {
                    1.0 // Previous window still active
                };
                
                let weighted_previous = (self.previous_count as f64 * prev_weight.max(0.0).min(1.0)) as u32;
                return self.signal_count.saturating_add(weighted_previous);
            }
        }
        
        // Only current window counts
        if elapsed_in_window < window_secs {
            self.signal_count
        } else {
            0 // Window expired
        }
    }
    
    /// Reset the rate tracker.
    pub fn reset(&mut self) {
        self.signal_count = 0;
        self.window_start = None;
        self.previous_count = 0;
        self.previous_window_start = None;
    }
    
    /// Get the current signal count in the active window.
    pub fn current_count(&self) -> u32 {
        self.signal_count
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
    /// Lifecycle timestamps (migration 0008). Each timestamp records when
    /// the session first entered the corresponding state. `None` means the
    /// session has not yet reached that state.
    #[serde(default)]
    pub initiating_at: Option<String>,
    #[serde(default)]
    pub ringing_at: Option<String>,
    #[serde(default)]
    pub connecting_at: Option<String>,
    #[serde(default)]
    pub connected_at: Option<String>,
    #[serde(default)]
    pub on_hold_since: Option<String>,
    #[serde(default)]
    pub reconnecting_since: Option<String>,
    #[serde(default)]
    pub canceled_at: Option<String>,
    #[serde(default)]
    pub failed_at: Option<String>,
    #[serde(default)]
    pub timeout_at: Option<String>,
    /// Structured reason for the session ending (terminal states only).
    /// Values: normal, busy, declined, timeout, failed, canceled, unreachable.
    #[serde(default)]
    pub ended_reason: Option<String>,
    /// Structured failure cause for `failed` terminal state.
    /// Values: media_error, signaling_error, provider_error, ice_failure,
    /// network_error, dtls_failure, sdp_error, resource_exhausted.
    #[serde(default)]
    pub failure_reason: Option<String>,
    #[serde(default)]
    pub epoch: u64,
    #[serde(default = "default_session_version")]
    pub version: u64,
    #[serde(default)]
    pub participants: SessionParticipants,
    pub last_activity_at: Option<String>,
    #[serde(default)]
    pub signal_rate_tracker: SignalRateTracker,
}

fn default_session_version() -> u64 {
    1
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
    /// Merge two state records monotonically, preserving the newer session
    /// state and deduplicating signals by `signal_seq`.
    ///
    /// Conflict resolution follows this priority:
    /// 1. **Terminal irreversibility:** if `self` is already terminal and
    ///    `next` is non-terminal, `self` is kept regardless of timestamp
    ///    (a terminal state cannot be "undone" by a stale active write
    ///    that arrives late).
    /// 2. **Primary: `updated_at` timestamp.** The record with the newer
    ///    timestamp wins its `session` field. This handles out-of-order
    ///    delivery (e.g., a stale `rejected` arriving after a newer
    ///    `accepted`) by trusting wall-clock progress.
    /// 3. **Tiebreaker: state rank.** When timestamps are equal, higher
    ///    rank wins. Terminal states have rank >= 100, so on a tie a
    ///    terminal state wins over an active state (finality guarantee).
    pub fn merge_monotonic(self, next: Self) -> Self {
        let session = if self.session.state.is_terminal() && !next.session.state.is_terminal() {
            // Terminal irreversibility: keep the existing terminal state
            // even if `next` has a newer timestamp. A stale active write
            // arriving after the session ended must not resurrect it.
            self.session
        } else {
            // For all other cases (both active, both terminal, or
            // self=active + next=terminal), decide by timestamp first,
            // then rank as tiebreaker.
            match rfc3339_cmp(&self.updated_at, &next.updated_at) {
                std::cmp::Ordering::Less => next.session,
                std::cmp::Ordering::Greater => self.session,
                std::cmp::Ordering::Equal => {
                    if rtc_session_state_rank(&next.session.state)
                        >= rtc_session_state_rank(&self.session.state)
                    {
                        next.session
                    } else {
                        self.session
                    }
                }
            }
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

pub trait StateStore: Send + Sync {
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

pub use StateStore as RtcStateStore;

pub type Session = RtcSession;
pub type SessionState = RtcSessionState;
pub type SignalEvent = RtcSignalEvent;
pub type SignalSender = RtcSignalSender;
pub type StateRecord = RtcStateRecord;

pub fn encode_im_call_key_segments<const N: usize>(parts: [&str; N]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

/// Lifecycle rank used as a tiebreaker in `merge_monotonic` when two
/// records have the same `updated_at` timestamp.
///
/// Active states use ranks 0–9 (lifecycle progress order).
/// Terminal states use ranks 100+ (higher than any active state, so on a
/// timestamp tie the terminal state wins — finality guarantee).
fn rtc_session_state_rank(state: &RtcSessionState) -> u8 {
    match state {
        // Active states (lifecycle progress).
        RtcSessionState::Initiating => 0,
        RtcSessionState::Started => 1, // legacy alias for initiating/ringing
        RtcSessionState::Ringing => 2,
        RtcSessionState::Connecting => 3,
        RtcSessionState::Accepted => 4, // legacy alias for connecting
        RtcSessionState::Connected => 5,
        RtcSessionState::OnHold => 6,
        RtcSessionState::Reconnecting => 7,
        // Terminal states (final, irreversible).
        RtcSessionState::Canceled => 100,
        RtcSessionState::Rejected => 101,
        RtcSessionState::Timeout => 102,
        RtcSessionState::Failed => 103,
        RtcSessionState::Ended => 104,
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

    #[test]
    fn terminal_state_is_irreversible_over_stale_active_write() {
        // Existing record is terminal (Ended) with an older timestamp.
        let ended = rtc_state_record(
            RtcSessionState::Ended,
            "2026-05-06T00:00:05.000Z",
            vec![rtc_signal_event(1)],
        );
        // Incoming record is active (Connected) with a newer timestamp —
        // this represents a stale/duplicated active write that arrived late.
        let stale_connected = rtc_state_record(
            RtcSessionState::Connected,
            "2026-05-06T00:00:06.000Z",
            vec![rtc_signal_event(2)],
        );

        let merged = ended.merge_monotonic(stale_connected);

        // Terminal state must be preserved: the stale active write cannot
        // "undo" the terminal state even though it has a newer timestamp.
        assert_eq!(merged.session.state, RtcSessionState::Ended);
        // But signals from both records are still merged.
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
    fn active_to_terminal_transition_is_accepted() {
        let connected = rtc_state_record(
            RtcSessionState::Connected,
            "2026-05-06T00:00:05.000Z",
            vec![rtc_signal_event(1)],
        );
        let ended = rtc_state_record(
            RtcSessionState::Ended,
            "2026-05-06T00:00:06.000Z",
            vec![rtc_signal_event(2)],
        );

        let merged = connected.merge_monotonic(ended);

        assert_eq!(merged.session.state, RtcSessionState::Ended);
    }

    #[test]
    fn same_timestamp_terminal_wins_over_active_by_rank() {
        let active = rtc_state_record(
            RtcSessionState::Connected,
            "2026-05-06T00:00:05.000Z",
            vec![rtc_signal_event(1)],
        );
        let terminal = rtc_state_record(
            RtcSessionState::Canceled,
            "2026-05-06T00:00:05.000Z",
            vec![rtc_signal_event(2)],
        );

        let merged = active.merge_monotonic(terminal);

        // Same timestamp: terminal (rank 100) wins over active (rank 5).
        assert_eq!(merged.session.state, RtcSessionState::Canceled);
    }

    #[test]
    fn state_from_str_roundtrip_all_variants() {
        for expected in [
            RtcSessionState::Started,
            RtcSessionState::Accepted,
            RtcSessionState::Rejected,
            RtcSessionState::Ended,
            RtcSessionState::Initiating,
            RtcSessionState::Ringing,
            RtcSessionState::Connecting,
            RtcSessionState::Connected,
            RtcSessionState::OnHold,
            RtcSessionState::Reconnecting,
            RtcSessionState::Canceled,
            RtcSessionState::Failed,
            RtcSessionState::Timeout,
        ] {
            let wire = expected.as_wire_value();
            assert_eq!(RtcSessionState::from_str(wire), Some(expected));
        }
        assert_eq!(RtcSessionState::from_str("unknown"), None);
    }

    #[test]
    fn terminal_and_active_classification() {
        assert!(RtcSessionState::Ended.is_terminal());
        assert!(RtcSessionState::Rejected.is_terminal());
        assert!(RtcSessionState::Canceled.is_terminal());
        assert!(RtcSessionState::Failed.is_terminal());
        assert!(RtcSessionState::Timeout.is_terminal());

        assert!(RtcSessionState::Started.is_active());
        assert!(RtcSessionState::Accepted.is_active());
        assert!(RtcSessionState::Initiating.is_active());
        assert!(RtcSessionState::Ringing.is_active());
        assert!(RtcSessionState::Connecting.is_active());
        assert!(RtcSessionState::Connected.is_active());
        assert!(RtcSessionState::OnHold.is_active());
        assert!(RtcSessionState::Reconnecting.is_active());

        assert!(RtcSessionState::Connected.is_media_connected());
        assert!(RtcSessionState::OnHold.is_media_connected());
        assert!(RtcSessionState::Reconnecting.is_media_connected());
        assert!(!RtcSessionState::Ringing.is_media_connected());
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
                initiating_at: None,
                ringing_at: None,
                connecting_at: None,
                connected_at: None,
                on_hold_since: None,
                reconnecting_since: None,
                canceled_at: None,
                failed_at: None,
                timeout_at: None,
                ended_reason: None,
                failure_reason: None,
                epoch: 1,
                version: 1,
                participants: SessionParticipants::default(),
                last_activity_at: None,
                signal_rate_tracker: SignalRateTracker::default(),
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
