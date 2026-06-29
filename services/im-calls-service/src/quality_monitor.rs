//! RTC call quality monitoring and degradation strategies.
//!
//! This module provides real-time monitoring of RTC call quality metrics
//! and automatic degradation strategies for weak network conditions.
//!
//! ## Features
//!
//! - **Quality Metrics**: RTT, packet loss, jitter, bitrate monitoring
//! - **ICE Monitoring**: Connection state tracking with automatic restart
//! - **Degradation Actions**: Automatic video disable, bitrate reduction
//! - **Statistics Collection**: Call duration, quality scores, failure tracking
//!
//! ## Quality Score Calculation
//!
//! ```text
//! quality_score = rtt_score * 0.3 + packet_loss_score * 0.3 + jitter_score * 0.2 + bitrate_score * 0.2
//! ```
//!
//! - RTT score: 1.0 for <100ms, 0.0 for >500ms
//! - Packet loss score: 1.0 for 0%, 0.0 for >30%
//! - Jitter score: 1.0 for <30ms, 0.0 for >200ms
//! - Bitrate score: 1.0 for >1Mbps, 0.0 for <100Kbps

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// RTC call codec type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RtcCodec {
    /// VP8 video codec.
    Vp8,
    /// VP9 video codec.
    Vp9,
    /// H264 video codec.
    H264,
    /// Opus audio codec.
    Opus,
    /// PCMU audio codec.
    Pcmu,
    /// PCMA audio codec.
    Pcma,
}

impl RtcCodec {
    pub fn as_str(&self) -> &'static str {
        match self {
            RtcCodec::Vp8 => "VP8",
            RtcCodec::Vp9 => "VP9",
            RtcCodec::H264 => "H264",
            RtcCodec::Opus => "Opus",
            RtcCodec::Pcmu => "PCMU",
            RtcCodec::Pcma => "PCMA",
        }
    }
}

/// ICE connection state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IceConnectionState {
    /// ICE agent is gathering addresses.
    New,
    /// ICE agent has been given local and remote candidates, and is attempting to find a match.
    Checking,
    /// ICE agent has found a working connection.
    Connected,
    /// ICE agent has finished checking all candidate pairs and has selected a pair to use.
    Completed,
    /// ICE agent has failed to find a working connection.
    Failed,
    /// ICE agent has lost connectivity but may recover.
    Disconnected,
    /// ICE agent has shut down and is no longer responding.
    Closed,
}

impl IceConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            IceConnectionState::New => "new",
            IceConnectionState::Checking => "checking",
            IceConnectionState::Connected => "connected",
            IceConnectionState::Completed => "completed",
            IceConnectionState::Failed => "failed",
            IceConnectionState::Disconnected => "disconnected",
            IceConnectionState::Closed => "closed",
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, IceConnectionState::Connected | IceConnectionState::Completed)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, IceConnectionState::Failed | IceConnectionState::Closed)
    }

    pub fn needs_restart(&self) -> bool {
        matches!(self, IceConnectionState::Failed | IceConnectionState::Disconnected)
    }
}

/// ICE candidate pair information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IceCandidatePair {
    /// Local candidate IP.
    pub local_ip: String,
    /// Remote candidate IP.
    pub remote_ip: String,
    /// Local candidate type (host, srflx, prflx, relay).
    pub local_type: String,
    /// Remote candidate type.
    pub remote_type: String,
    /// Selected pair priority.
    pub priority: u64,
    /// Current RTT for this pair.
    pub rtt: Duration,
}

/// RTC quality degradation action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RtcDowngradeAction {
    /// Disable video track, keep audio only.
    DisableVideo,
    /// Reduce video bitrate (lower quality).
    ReduceBitrate,
    /// Reduce video resolution (e.g., 720p -> 480p).
    ReduceResolution,
    /// Disable simulcast (use single stream).
    DisableSimulcast,
    /// Switch to backup codec (VP9 -> VP8).
    SwitchCodec,
    /// Terminate call due to poor quality.
    TerminateCall,
}

impl RtcDowngradeAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            RtcDowngradeAction::DisableVideo => "disable_video",
            RtcDowngradeAction::ReduceBitrate => "reduce_bitrate",
            RtcDowngradeAction::ReduceResolution => "reduce_resolution",
            RtcDowngradeAction::DisableSimulcast => "disable_simulcast",
            RtcDowngradeAction::SwitchCodec => "switch_codec",
            RtcDowngradeAction::TerminateCall => "terminate_call",
        }
    }
}

/// RTC call quality metrics.
#[derive(Clone, Debug)]
pub struct RtcQualityMetrics {
    /// Round-trip time for media packets.
    pub rtt: Duration,
    /// Packet loss rate (0.0 - 1.0).
    pub packet_loss_rate: f64,
    /// Jitter (variation in packet arrival time).
    pub jitter: Duration,
    /// Current bitrate (bits per second).
    pub bitrate: u64,
    /// Current codec being used.
    pub codec: Option<RtcCodec>,
    /// ICE connection state.
    pub ice_state: IceConnectionState,
    /// Selected ICE candidate pair.
    pub selected_candidate_pair: Option<IceCandidatePair>,
    /// Quality score (0.0 - 1.0, higher is better).
    pub quality_score: f64,
    /// Timestamp of last update.
    pub last_updated: Instant,
    /// Call duration so far.
    pub call_duration: Duration,
    /// Number of ICE restarts attempted.
    pub ice_restart_count: u32,
    /// Number of codec switches.
    pub codec_switch_count: u32,
}

impl Default for RtcQualityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RtcQualityMetrics {
    pub fn new() -> Self {
        Self {
            rtt: Duration::from_millis(100),
            packet_loss_rate: 0.0,
            jitter: Duration::from_millis(30),
            bitrate: 1_000_000, // 1 Mbps
            codec: Some(RtcCodec::Opus),
            ice_state: IceConnectionState::New,
            selected_candidate_pair: None,
            quality_score: 1.0,
            last_updated: Instant::now(),
            call_duration: Duration::from_secs(0),
            ice_restart_count: 0,
            codec_switch_count: 0,
        }
    }

    /// Update quality score based on current metrics.
    pub fn update_quality_score(&mut self) {
        // RTT score: 1.0 for <100ms, linear degradation to 0.0 at 500ms
        let rtt_score = if self.rtt < Duration::from_millis(100) {
            1.0
        } else if self.rtt > Duration::from_millis(500) {
            0.0
        } else {
            1.0 - (self.rtt.as_millis() as f64 - 100.0) / 400.0
        };

        // Packet loss score: 1.0 for 0%, linear degradation to 0.0 at 30%
        let loss_score = if self.packet_loss_rate == 0.0 {
            1.0
        } else if self.packet_loss_rate >= 0.3 {
            0.0
        } else {
            1.0 - self.packet_loss_rate / 0.3
        };

        // Jitter score: 1.0 for <30ms, linear degradation to 0.0 at 200ms
        let jitter_score = if self.jitter < Duration::from_millis(30) {
            1.0
        } else if self.jitter > Duration::from_millis(200) {
            0.0
        } else {
            1.0 - (self.jitter.as_millis() as f64 - 30.0) / 170.0
        };

        // Bitrate score: 1.0 for >1Mbps, linear degradation to 0.0 at 100Kbps
        let bitrate_score = if self.bitrate >= 1_000_000 {
            1.0
        } else if self.bitrate <= 100_000 {
            0.0
        } else {
            (self.bitrate as f64 - 100_000.0) / 900_000.0
        };

        // Weighted composite score
        self.quality_score = rtt_score * 0.3 + loss_score * 0.3 + jitter_score * 0.2 + bitrate_score * 0.2;

        // Penalize if ICE is not connected
        if !self.ice_state.is_connected() {
            self.quality_score *= 0.5;
        }

        self.last_updated = Instant::now();
    }

    /// Suggest degradation action based on current quality.
    pub fn suggest_downgrade(&self) -> Option<RtcDowngradeAction> {
        if self.quality_score < 0.3 {
            Some(RtcDowngradeAction::TerminateCall)
        } else if self.quality_score < 0.5 {
            Some(RtcDowngradeAction::DisableVideo)
        } else if self.quality_score < 0.7 {
            if self.bitrate > 500_000 {
                Some(RtcDowngradeAction::ReduceBitrate)
            } else {
                Some(RtcDowngradeAction::ReduceResolution)
            }
        } else {
            None
        }
    }

    /// Check if ICE restart is recommended.
    pub fn should_restart_ice(&self) -> bool {
        self.ice_state.needs_restart() && self.ice_restart_count < 3
    }

    /// Record ICE state change.
    pub fn record_ice_state_change(&mut self, new_state: IceConnectionState) {
        self.ice_state = new_state;
        self.update_quality_score();
    }

    /// Record ICE restart attempt.
    pub fn record_ice_restart(&mut self) {
        self.ice_restart_count += 1;
    }

    /// Record codec switch.
    pub fn record_codec_switch(&mut self, new_codec: RtcCodec) {
        self.codec = Some(new_codec);
        self.codec_switch_count += 1;
    }
}

/// RTC quality monitor for tracking call statistics.
#[derive(Clone)]
pub struct RtcQualityMonitor {
    /// Call session ID.
    pub rtc_session_id: String,
    /// Current quality metrics.
    pub metrics: RtcQualityMetrics,
    /// Historical metrics snapshots (timestamp -> metrics).
    pub history: BTreeMap<u64, RtcQualityMetrics>,
    /// Maximum history entries to retain.
    pub max_history_entries: usize,
}

impl RtcQualityMonitor {
    pub fn new(rtc_session_id: String) -> Self {
        Self {
            rtc_session_id,
            metrics: RtcQualityMetrics::new(),
            history: BTreeMap::new(),
            max_history_entries: 100,
        }
    }

    /// Update current metrics and record history snapshot.
    pub fn update_metrics(&mut self, metrics: RtcQualityMetrics) {
        self.metrics = metrics.clone();
        self.metrics.update_quality_score();
        
        // Record history snapshot (every 5 seconds)
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Evict oldest entries if at capacity
        if self.history.len() >= self.max_history_entries {
            let oldest = self.history.keys().next().copied();
            if let Some(key) = oldest {
                self.history.remove(&key);
            }
        }
        
        self.history.insert(timestamp, metrics);
    }

    /// Get average quality score over last N snapshots.
    pub fn average_quality_score(&self, last_n: usize) -> f64 {
        let entries = self.history.iter().rev().take(last_n);
        let count = entries.len();
        if count == 0 {
            return self.metrics.quality_score;
        }
        
        let sum: f64 = entries.map(|(_, m)| m.quality_score).sum();
        sum / count as f64
    }

    /// Get quality trend (improving, stable, deteriorating).
    pub fn quality_trend(&self) -> QualityTrend {
        if self.history.len() < 2 {
            return QualityTrend::Stable;
        }
        
        let recent_avg = self.average_quality_score(5);
        let older_avg = self.average_quality_score(10);
        
        if recent_avg > older_avg + 0.05 {
            QualityTrend::Improving
        } else if recent_avg < older_avg - 0.05 {
            QualityTrend::Deteriorating
        } else {
            QualityTrend::Stable
        }
    }

    /// Generate call quality report.
    pub fn generate_report(&self) -> RtcQualityReport {
        let avg_score = self.average_quality_score(self.history.len());
        let trend = self.quality_trend();
        let min_score = self.history.values().map(|m| m.quality_score).reduce(f64::min).unwrap_or(0.0);
        let max_score = self.history.values().map(|m| m.quality_score).reduce(f64::max).unwrap_or(1.0);
        
        RtcQualityReport {
            rtc_session_id: self.rtc_session_id.clone(),
            call_duration: self.metrics.call_duration,
            average_quality_score: avg_score,
            min_quality_score: min_score,
            max_quality_score: max_score,
            quality_trend: trend,
            ice_restart_count: self.metrics.ice_restart_count,
            codec_switch_count: self.metrics.codec_switch_count,
            final_ice_state: self.metrics.ice_state.clone(),
            final_codec: self.metrics.codec.clone(),
        }
    }
}

/// Quality trend indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityTrend {
    /// Quality is improving.
    Improving,
    /// Quality is stable.
    Stable,
    /// Quality is deteriorating.
    Deteriorating,
}

/// RTC call quality report.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RtcQualityReport {
    /// Call session ID.
    pub rtc_session_id: String,
    /// Total call duration.
    pub call_duration: Duration,
    /// Average quality score over the call.
    pub average_quality_score: f64,
    /// Minimum quality score observed.
    pub min_quality_score: f64,
    /// Maximum quality score observed.
    pub max_quality_score: f64,
    /// Quality trend at call end.
    pub quality_trend: QualityTrend,
    /// Number of ICE restarts attempted.
    pub ice_restart_count: u32,
    /// Number of codec switches.
    pub codec_switch_count: u32,
    /// Final ICE connection state.
    pub final_ice_state: IceConnectionState,
    /// Final codec used.
    pub final_codec: Option<RtcCodec>,
}

/// Thread-safe quality monitor registry using DashMap.
pub struct RtcQualityMonitorRegistry {
    monitors: DashMap<String, RtcQualityMonitor>,
}

impl RtcQualityMonitorRegistry {
    pub fn new() -> Self {
        Self {
            monitors: DashMap::new(),
        }
    }

    /// Create or get monitor for a session.
    pub fn get_or_create(&self, rtc_session_id: &str) -> RtcQualityMonitor {
        self.monitors
            .entry(rtc_session_id.to_owned())
            .or_insert_with(|| RtcQualityMonitor::new(rtc_session_id.to_owned()))
            .clone()
    }

    /// Update metrics for a session.
    pub fn update(&self, rtc_session_id: &str, metrics: RtcQualityMetrics) {
        if let Some(mut monitor) = self.monitors.get_mut(rtc_session_id) {
            monitor.update_metrics(metrics);
        }
    }

    /// Remove monitor and generate final report.
    pub fn finalize(&self, rtc_session_id: &str) -> Option<RtcQualityReport> {
        self.monitors
            .remove(rtc_session_id)
            .map(|(_, monitor)| monitor.generate_report())
    }

    /// Get all active session IDs.
    pub fn active_sessions(&self) -> Vec<String> {
        self.monitors.iter().map(|entry| entry.key().clone()).collect()
    }
}

impl Default for RtcQualityMonitorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_metrics_calculates_score() {
        let metrics = RtcQualityMetrics::new();
        assert!(metrics.quality_score > 0.9);
    }

    #[test]
    fn quality_metrics_suggests_downgrade() {
        // Metrics tuned to land in the DisableVideo range (0.3 <= score < 0.5).
        // Previous values (rtt=400ms, loss=0.25, jitter=150ms, bitrate=200kbps)
        // produced a score ~0.206 which triggered TerminateCall (< 0.3).
        let mut poor_metrics = RtcQualityMetrics {
            rtt: Duration::from_millis(350),
            packet_loss_rate: 0.20,
            jitter: Duration::from_millis(100),
            bitrate: 300_000,
            ice_state: IceConnectionState::Connected,
            ..Default::default()
        };
        poor_metrics.update_quality_score();

        assert!(
            poor_metrics.quality_score >= 0.3 && poor_metrics.quality_score < 0.5,
            "quality_score {} should be in [0.3, 0.5) for DisableVideo",
            poor_metrics.quality_score
        );
        assert_eq!(
            poor_metrics.suggest_downgrade(),
            Some(RtcDowngradeAction::DisableVideo)
        );
    }

    #[test]
    fn ice_state_tracking() {
        let mut metrics = RtcQualityMetrics::new();
        metrics.record_ice_state_change(IceConnectionState::Connected);
        
        assert!(metrics.ice_state.is_connected());
        assert!(!metrics.should_restart_ice());
        
        metrics.record_ice_state_change(IceConnectionState::Failed);
        assert!(metrics.should_restart_ice());
        
        metrics.record_ice_restart();
        assert_eq!(metrics.ice_restart_count, 1);
    }

    #[test]
    fn monitor_registry_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let registry = Arc::new(RtcQualityMonitorRegistry::new());
        let mut handles = vec![];

        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            handles.push(thread::spawn(move || {
                let session_id = format!("session_{}", i);
                let monitor = registry_clone.get_or_create(&session_id);
                
                let metrics = RtcQualityMetrics {
                    rtt: Duration::from_millis(50 + i * 10),
                    ..Default::default()
                };
                registry_clone.update(&session_id, metrics);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(registry.active_sessions().len(), 10);
    }

    #[test]
    fn quality_report_generation() {
        let monitor = RtcQualityMonitor::new("test_session".to_string());
        let report = monitor.generate_report();
        
        assert_eq!(report.rtc_session_id, "test_session");
        assert!(report.average_quality_score > 0.0);
    }
}