//! Anomaly detection for suspicious behavior patterns.
//!
//! This module implements real-time anomaly detection to identify suspicious
//! patterns in user behavior, message rates, and connection patterns.
//!
//! ## Detected Anomalies
//!
//! - **Message rate spike**: Abnormally high message frequency
//! - **Suspicious content**: Potentially harmful or spam content
//! - **Abnormal connection**: Unusual connection patterns (multiple IPs, rapid reconnects)
//! - **Credential stuffing**: Multiple failed authentication attempts
//! - **Distributed attack**: Coordinated behavior across multiple accounts
//!
//! ## Architecture
//!
//! ```
//! ┌─────────────────┐
//! │ Request Stream  │
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Rate Tracker    │  ← Per-user, per-tenant, per-IP
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Content Analyzer│  ← Pattern matching, heuristics
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Action Log      │  ← Anomaly events with timestamps
//! └────────┬────────┘
//! ```

use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// Anomaly type classification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Message rate exceeds normal threshold.
    MessageRateSpike,
    /// Content matches suspicious patterns.
    SuspiciousContent,
    /// Unusual connection pattern (multiple IPs, rapid reconnects).
    AbnormalConnection,
    /// Multiple failed authentication attempts.
    CredentialStuffing,
    /// Coordinated behavior from multiple accounts.
    DistributedAttack,
    /// Abnormal geographic pattern (rapid location changes).
    GeographicAnomaly,
    /// Automated behavior (robot-like patterns).
    AutomatedBehavior,
}

impl AnomalyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnomalyType::MessageRateSpike => "message_rate_spike",
            AnomalyType::SuspiciousContent => "suspicious_content",
            AnomalyType::AbnormalConnection => "abnormal_connection",
            AnomalyType::CredentialStuffing => "credential_stuffing",
            AnomalyType::DistributedAttack => "distributed_attack",
            AnomalyType::GeographicAnomaly => "geographic_anomaly",
            AnomalyType::AutomatedBehavior => "automated_behavior",
        }
    }

    /// Get severity level (1 = low, 2 = medium, 3 = high).
    pub fn severity(&self) -> u8 {
        match self {
            AnomalyType::CredentialStuffing => 3,
            AnomalyType::DistributedAttack => 3,
            AnomalyType::SuspiciousContent => 2,
            AnomalyType::AbnormalConnection => 2,
            AnomalyType::MessageRateSpike => 1,
            AnomalyType::GeographicAnomaly => 2,
            AnomalyType::AutomatedBehavior => 2,
        }
    }
}

/// Anomaly event record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnomalyEvent {
    /// Anomaly type.
    pub anomaly_type: AnomalyType,
    /// User ID (if identified).
    pub user_id: Option<String>,
    /// Tenant ID.
    pub tenant_id: String,
    /// Client IP address.
    pub client_ip: IpAddr,
    /// Timestamp when anomaly was detected.
    pub detected_at: Instant,
    /// Additional context/details.
    pub details: String,
    /// Severity level.
    pub severity: u8,
    /// Recommended action.
    pub recommended_action: RecommendedAction,
}

/// Recommended action for handling anomaly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendedAction {
    /// Log and monitor (low severity).
    LogOnly,
    /// Apply rate limiting.
    RateLimit,
    /// Challenge with CAPTCHA or additional verification.
    Challenge,
    /// Temporary block (minutes to hours).
    TemporaryBlock,
    /// Permanent ban.
    PermanentBan,
    /// Alert administrators for manual review.
    AlertAdmin,
}

impl RecommendedAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecommendedAction::LogOnly => "log_only",
            RecommendedAction::RateLimit => "rate_limit",
            RecommendedAction::Challenge => "challenge",
            RecommendedAction::TemporaryBlock => "temporary_block",
            RecommendedAction::PermanentBan => "permanent_ban",
            RecommendedAction::AlertAdmin => "alert_admin",
        }
    }
}

/// Rate tracking for a single user/IP.
#[derive(Clone, Debug, Default)]
struct RateTrackerEntry {
    /// Message timestamps in the current window.
    message_times: VecDeque<Instant>,
    /// Failed auth attempt timestamps.
    failed_auth_times: VecDeque<Instant>,
    /// Connection timestamps.
    connection_times: VecDeque<Instant>,
    /// Last seen IPs.
    recent_ips: VecDeque<IpAddr>,
    /// Total message count.
    total_messages: u64,
    /// Total failed auth attempts.
    total_failed_auth: u64,
}

impl RateTrackerEntry {
    fn new() -> Self {
        Self {
            message_times: VecDeque::with_capacity(100),
            failed_auth_times: VecDeque::with_capacity(50),
            connection_times: VecDeque::with_capacity(20),
            recent_ips: VecDeque::with_capacity(10),
            total_messages: 0,
            total_failed_auth: 0,
        }
    }

    /// Record a message and return current rate (messages per minute).
    fn record_message(&mut self, window: Duration) -> f64 {
        let now = Instant::now();
        self.message_times.push_back(now);
        self.total_messages += 1;

        // Remove messages outside the window
        let cutoff = now - window;
        while let Some(front) = self.message_times.front() {
            if *front < cutoff {
                self.message_times.pop_front();
            } else {
                break;
            }
        }

        // Calculate rate (messages per minute)
        if window.as_secs() > 0 {
            self.message_times.len() as f64 / (window.as_secs_f64() / 60.0)
        } else {
            0.0
        }
    }

    /// Record a failed auth attempt and return count in window.
    fn record_failed_auth(&mut self, window: Duration) -> u32 {
        let now = Instant::now();
        self.failed_auth_times.push_back(now);
        self.total_failed_auth += 1;

        // Remove failed auths outside the window
        let cutoff = now - window;
        while let Some(front) = self.failed_auth_times.front() {
            if *front < cutoff {
                self.failed_auth_times.pop_front();
            } else {
                break;
            }
        }

        self.failed_auth_times.len() as u32
    }

    /// Record a connection and check for rapid reconnect pattern.
    fn record_connection(&mut self, window: Duration) -> bool {
        let now = Instant::now();
        self.connection_times.push_back(now);

        // Remove connections outside the window
        let cutoff = now - window;
        while let Some(front) = self.connection_times.front() {
            if *front < cutoff {
                self.connection_times.pop_front();
            } else {
                break;
            }
        }

        // Check for rapid reconnect (>5 connections in 1 minute)
        self.connection_times.len() > 5
    }

    /// Record an IP and check for multiple IP pattern.
    fn record_ip(&mut self, ip: IpAddr, window: Duration) -> bool {
        // Only add if different from last IP
        if self.recent_ips.back().map_or(true, |last| *last != ip) {
            self.recent_ips.push_back(ip);
        }

        // Keep only recent IPs (last 10)
        while self.recent_ips.len() > 10 {
            self.recent_ips.pop_front();
        }

        // Check for multiple IPs (>3 different IPs)
        self.recent_ips.len() > 3
    }

    /// Get current message rate.
    fn current_message_rate(&self) -> f64 {
        self.message_times.len() as f64
    }
}

/// Content analyzer for suspicious pattern detection.
#[derive(Clone, Debug)]
pub struct ContentAnalyzer {
    /// Suspicious content patterns (regex patterns).
    suspicious_patterns: Vec<String>,
    /// Spam indicators.
    spam_keywords: Vec<String>,
}

impl ContentAnalyzer {
    pub fn new() -> Self {
        Self {
            suspicious_patterns: vec![
                // URL patterns
                r"https?://[^\s]+",
                r"www\.[^\s]+",
                // Phone number patterns
                r"\+?\d{1,3}[\s-]?\d{3,4}[\s-]?\d{4}",
                // Email patterns
                r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
            ],
            spam_keywords: vec![
                "免费", "优惠", "促销", "中奖", "红包",
                "free", "winner", "prize", "discount", "click here",
            ],
        }
    }

    /// Check if content is suspicious.
    pub fn is_suspicious(&self, content: &str) -> Option<String> {
        // Check for spam keywords
        for keyword in &self.spam_keywords {
            if content.to_lowercase().contains(keyword) {
                return Some(format!("spam_keyword: {}", keyword));
            }
        }

        // Check for excessive URLs (potential phishing)
        let url_count = content.matches("http://").count() + content.matches("https://").count();
        if url_count > 3 {
            return Some(format!("excessive_urls: {}", url_count));
        }

        // Check for repetitive patterns (potential spam)
        if self.has_repetitive_pattern(content) {
            return Some("repetitive_pattern".to_string());
        }

        None
    }

    /// Check for repetitive patterns.
    fn has_repetitive_pattern(&self, content: &str) -> bool {
        let chars = content.chars().collect::<Vec<_>>();
        if chars.len() < 20 {
            return false;
        }

        // Check for character repetition (>50% same character)
        let char_counts: HashMap<char, u32> = chars.iter().fold(HashMap::new(), |mut counts, c| {
            *counts.entry(*c).or_insert(0) += 1;
            counts
        });

        if let Some(max_count) = char_counts.values().max() {
            *max_count as f64 > chars.len() as f64 * 0.5
        } else {
            false
        }
    }
}

impl Default for ContentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Anomaly detector for real-time suspicious behavior detection.
#[derive(Clone)]
pub struct AnomalyDetector {
    /// Rate trackers per user.
    user_trackers: DashMap<String, RateTrackerEntry>,
    /// Rate trackers per IP.
    ip_trackers: DashMap<IpAddr, RateTrackerEntry>,
    /// Content analyzer.
    content_analyzer: ContentAnalyzer,
    /// Anomaly event log.
    anomaly_log: Arc<Mutex<VecDeque<AnomalyEvent>>>,
    /// Configuration.
    config: AnomalyDetectorConfig,
    /// Statistics.
    stats: AnomalyStats,
}

/// Anomaly detector configuration.
#[derive(Clone, Debug)]
pub struct AnomalyDetectorConfig {
    /// Message rate threshold (messages per minute).
    pub message_rate_threshold: f64,
    /// Failed auth threshold (attempts per hour).
    pub failed_auth_threshold: u32,
    /// Connection rate threshold (connections per minute).
    pub connection_rate_threshold: u32,
    /// Multiple IP threshold (distinct IPs).
    pub multiple_ip_threshold: u32,
    /// Rate tracking window duration.
    pub rate_window: Duration,
    /// Failed auth window duration.
    pub auth_window: Duration,
    /// Maximum anomaly log entries.
    pub max_log_entries: usize,
}

impl AnomalyDetectorConfig {
    pub fn new() -> Self {
        Self {
            message_rate_threshold: 100.0, // 100 messages per minute
            failed_auth_threshold: 10,     // 10 failed auths per hour
            connection_rate_threshold: 10, // 10 connections per minute
            multiple_ip_threshold: 5,      // 5 different IPs
            rate_window: Duration::from_secs(60),
            auth_window: Duration::from_secs(3600),
            max_log_entries: 1000,
        }
    }

    pub fn from_env() -> Self {
        Self {
            message_rate_threshold: std::env::var("SDKWORK_IM_ANOMALY_MESSAGE_RATE_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100.0),
            failed_auth_threshold: std::env::var("SDKWORK_IM_ANOMALY_AUTH_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            connection_rate_threshold: std::env::var("SDKWORK_IM_ANOMALY_CONNECTION_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            multiple_ip_threshold: std::env::var("SDKWORK_IM_ANOMALY_IP_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            rate_window: Duration::from_secs(60),
            auth_window: Duration::from_secs(3600),
            max_log_entries: 1000,
        }
    }
}

impl Default for AnomalyDetectorConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Anomaly detection statistics.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AnomalyStats {
    /// Total anomalies detected.
    pub total_anomalies: AtomicU64,
    /// Anomalies by type.
    anomalies_by_type: HashMap<String, u64>,
    /// Messages analyzed.
    messages_analyzed: AtomicU64,
    /// Auth attempts analyzed.
    auth_attempts_analyzed: AtomicU64,
}

impl AnomalyDetector {
    /// Create a new anomaly detector with configuration validation.
    ///
    /// Invalid configuration values are automatically corrected to safe defaults
    /// with a warning log, rather than causing a panic (P0-4 fix).
    /// This ensures the service remains available even with misconfiguration.
    pub fn new(config: AnomalyDetectorConfig) -> Self {
        // Validate and auto-correct configuration to safe defaults
        let safe_config = if config.message_rate_threshold <= 0.0 {
            tracing::warn!(
                target: "sdkwork.im.gateway.anomaly_detector",
                invalid_value = config.message_rate_threshold,
                default_value = 100.0,
                "Invalid message_rate_threshold (must be positive), using default 100.0 messages/min"
            );
            AnomalyDetectorConfig {
                message_rate_threshold: 100.0,
                ..config.clone()
            }
        } else if config.failed_auth_threshold == 0 {
            tracing::warn!(
                target: "sdkwork.im.gateway.anomaly_detector",
                default_value = 10,
                "Invalid failed_auth_threshold (must be positive), using default 10 attempts"
            );
            AnomalyDetectorConfig {
                failed_auth_threshold: 10,
                ..config.clone()
            }
        } else if config.max_log_entries == 0 {
            tracing::warn!(
                target: "sdkwork.im.gateway.anomaly_detector",
                default_value = 1000,
                "Invalid max_log_entries (must be positive), using default 1000 entries"
            );
            AnomalyDetectorConfig {
                max_log_entries: 1000,
                ..config.clone()
            }
        } else {
            config
        };

        Self {
            user_trackers: DashMap::new(),
            ip_trackers: DashMap::new(),
            content_analyzer: ContentAnalyzer::new(),
            anomaly_log: Arc::new(Mutex::new(VecDeque::with_capacity(safe_config.max_log_entries))),
            config: safe_config,
            stats: AnomalyStats::default(),
        }
    }

    /// Check a message for anomalies.
    pub fn check_message(
        &self,
        user_id: &str,
        tenant_id: &str,
        client_ip: IpAddr,
        content: &str,
    ) -> Option<AnomalyEvent> {
        self.stats.messages_analyzed.fetch_add(1, Ordering::Relaxed);

        // Check user rate
        let user_tracker = self.user_trackers.entry(user_id.to_owned()).or_default();
        let user_rate = user_tracker.record_message(self.config.rate_window);

        if user_rate > self.config.message_rate_threshold {
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::MessageRateSpike,
                user_id: Some(user_id.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: format!("Message rate {:.1}/min exceeds threshold {:.1}/min", 
                    user_rate, self.config.message_rate_threshold),
                severity: AnomalyType::MessageRateSpike.severity(),
                recommended_action: RecommendedAction::RateLimit,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        // Check IP rate
        let ip_tracker = self.ip_trackers.entry(client_ip).or_default();
        let ip_rate = ip_tracker.record_message(self.config.rate_window);

        if ip_rate > self.config.message_rate_threshold * 5.0 { // Higher threshold for IP
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::DistributedAttack,
                user_id: Some(user_id.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: format!("IP message rate {:.1}/min suggests distributed attack", ip_rate),
                severity: AnomalyType::DistributedAttack.severity(),
                recommended_action: RecommendedAction::TemporaryBlock,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        // Check content
        if let Some(reason) = self.content_analyzer.is_suspicious(content) {
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::SuspiciousContent,
                user_id: Some(user_id.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: format!("Suspicious content detected: {}", reason),
                severity: AnomalyType::SuspiciousContent.severity(),
                recommended_action: RecommendedAction::Challenge,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        None
    }

    /// Check an authentication attempt for anomalies.
    pub fn check_auth_attempt(
        &self,
        user_id: Option<&str>,
        tenant_id: &str,
        client_ip: IpAddr,
        success: bool,
    ) -> Option<AnomalyEvent> {
        self.stats.auth_attempts_analyzed.fetch_add(1, Ordering::Relaxed);

        if success {
            return None;
        }

        // Track failed auth by IP
        let ip_tracker = self.ip_trackers.entry(client_ip).or_default();
        let failed_count = ip_tracker.record_failed_auth(self.config.auth_window);

        if failed_count >= self.config.failed_auth_threshold {
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::CredentialStuffing,
                user_id: user_id.map(|s| s.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: format!("{} failed auth attempts from IP in {} hour(s)", 
                    failed_count, self.config.auth_window.as_secs() / 3600),
                severity: AnomalyType::CredentialStuffing.severity(),
                recommended_action: RecommendedAction::TemporaryBlock,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        // Track failed auth by user (if identified)
        if let Some(uid) = user_id {
            let user_tracker = self.user_trackers.entry(uid.to_owned()).or_default();
            let user_failed = user_tracker.record_failed_auth(self.config.auth_window);

            if user_failed >= self.config.failed_auth_threshold {
                let event = AnomalyEvent {
                    anomaly_type: AnomalyType::CredentialStuffing,
                    user_id: Some(uid.to_owned()),
                    tenant_id: tenant_id.to_owned(),
                    client_ip,
                    detected_at: Instant::now(),
                    details: format!("{} failed auth attempts for user", user_failed),
                    severity: AnomalyType::CredentialStuffing.severity(),
                    recommended_action: RecommendedAction::AlertAdmin,
                };
                self.record_anomaly(event.clone());
                return Some(event);
            }
        }

        None
    }

    /// Check a connection for anomalies.
    pub fn check_connection(
        &self,
        user_id: &str,
        tenant_id: &str,
        client_ip: IpAddr,
    ) -> Option<AnomalyEvent> {
        // Check for rapid reconnect
        let user_tracker = self.user_trackers.entry(user_id.to_owned()).or_default();
        let rapid_reconnect = user_tracker.record_connection(self.config.rate_window);

        if rapid_reconnect {
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::AbnormalConnection,
                user_id: Some(user_id.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: "Rapid connection pattern detected (>5 connections/min)",
                severity: AnomalyType::AbnormalConnection.severity(),
                recommended_action: RecommendedAction::RateLimit,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        // Check for multiple IPs
        let multiple_ips = user_tracker.record_ip(client_ip, self.config.rate_window);
        if multiple_ips {
            let event = AnomalyEvent {
                anomaly_type: AnomalyType::AbnormalConnection,
                user_id: Some(user_id.to_owned()),
                tenant_id: tenant_id.to_owned(),
                client_ip,
                detected_at: Instant::now(),
                details: "Multiple IP addresses detected for user",
                severity: AnomalyType::AbnormalConnection.severity(),
                recommended_action: RecommendedAction::Challenge,
            };
            self.record_anomaly(event.clone());
            return Some(event);
        }

        None
    }

    /// Record an anomaly event.
    fn record_anomaly(&self, event: AnomalyEvent) {
        self.stats.total_anomalies.fetch_add(1, Ordering::Relaxed);

        let mut log = self.anomaly_log.lock().unwrap();
        
        // Evict old entries if at capacity
        if log.len() >= self.config.max_log_entries {
            log.pop_front();
        }
        
        log.push_back(event);

        tracing::warn!(
            target: "sdkwork.im.anomaly",
            event = "im.anomaly.detected",
            anomaly_type = %event.anomaly_type.as_str(),
            user_id = ?event.user_id,
            tenant_id = %event.tenant_id,
            client_ip = %event.client_ip,
            severity = event.severity,
            recommended_action = %event.recommended_action.as_str(),
            details = %event.details,
            "anomaly detected"
        );
    }

    /// Get recent anomaly events.
    pub fn recent_anomalies(&self, limit: usize) -> Vec<AnomalyEvent> {
        let log = self.anomaly_log.lock().unwrap();
        log.iter().rev().take(limit).cloned().collect()
    }

    /// Get anomaly statistics.
    pub fn stats(&self) -> AnomalyStats {
        self.stats.clone()
    }

    /// Clear all trackers (for testing).
    pub fn clear(&self) {
        self.user_trackers.clear();
        self.ip_trackers.clear();
        self.anomaly_log.lock().unwrap().clear();
    }

    /// Clean up stale tracker entries to prevent memory leaks.
    ///
    /// This should be called periodically (e.g., every 5 minutes) to remove
    /// entries for users/IPs that haven't been active recently.
    pub fn cleanup_stale_entries(&self) {
        let cutoff = Instant::now() - Duration::from_secs(3600); // 1 hour

        // Clean up user trackers
        self.user_trackers.retain(|_user_id, tracker| {
            // Keep if any activity in the last hour
            tracker.message_times.back().map_or(false, |t| *t > cutoff)
                || tracker.failed_auth_times.back().map_or(false, |t| *t > cutoff)
                || tracker.connection_times.back().map_or(false, |t| *t > cutoff)
        });

        // Clean up IP trackers
        self.ip_trackers.retain(|_ip, tracker| {
            tracker.message_times.back().map_or(false, |t| *t > cutoff)
                || tracker.failed_auth_times.back().map_or(false, |t| *t > cutoff)
                || tracker.connection_times.back().map_or(false, |t| *t > cutoff)
        });

        // Log cleanup stats
        let user_count = self.user_trackers.len();
        let ip_count = self.ip_trackers.len();
        tracing::debug!(
            target: "sdkwork.im.anomaly",
            user_trackers = user_count,
            ip_trackers = ip_count,
            "anomaly detector cleanup completed"
        );
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new(AnomalyDetectorConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anomaly_severity_levels() {
        assert_eq!(AnomalyType::CredentialStuffing.severity(), 3);
        assert_eq!(AnomalyType::MessageRateSpike.severity(), 1);
        assert_eq!(AnomalyType::SuspiciousContent.severity(), 2);
    }

    #[test]
    fn rate_tracker_message_rate() {
        let mut tracker = RateTrackerEntry::new();
        
        // Record 10 messages rapidly
        for _ in 0..10 {
            tracker.record_message(Duration::from_secs(60));
        }
        
        // Should have high rate
        assert!(tracker.current_message_rate() > 0.0);
    }

    #[test]
    fn content_analyzer_detects_spam() {
        let analyzer = ContentAnalyzer::new();
        
        // Check spam keyword
        assert!(analyzer.is_suspicious("免费优惠促销中奖").is_some());
        
        // Check excessive URLs
        assert!(analyzer.is_suspicious("http://a.com http://b.com http://c.com http://d.com").is_some());
        
        // Normal content should pass
        assert!(analyzer.is_suspicious("Hello, how are you?").is_none());
    }

    #[test]
    fn detector_message_rate_spike() {
        let detector = AnomalyDetector::new(AnomalyDetectorConfig {
            message_rate_threshold: 10.0, // Low threshold for test
            ..Default::default()
        });
        
        // Rapid messages should trigger anomaly
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        
        // Send 20 messages rapidly
        for i in 0..20 {
            let anomaly = detector.check_message(
                "user_1",
                "tenant_1",
                ip,
                "test message",
            );
            
            if i >= 10 && anomaly.is_some() {
                let event = anomaly.unwrap();
                assert_eq!(event.anomaly_type, AnomalyType::MessageRateSpike);
                assert_eq!(event.recommended_action, RecommendedAction::RateLimit);
                return;
            }
        }
        
        // Should have detected anomaly
        assert!(detector.stats().total_anomalies.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn detector_credential_stuffing() {
        let detector = AnomalyDetector::new(AnomalyDetectorConfig {
            failed_auth_threshold: 5,
            ..Default::default()
        });
        
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        
        // Failed auth attempts
        for _ in 0..5 {
            detector.check_auth_attempt(None, "tenant_1", ip, false);
        }
        
        // Should have detected credential stuffing
        assert!(detector.stats().total_anomalies.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn detector_suspicious_content() {
        let detector = AnomalyDetector::default();
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        
        let anomaly = detector.check_message(
            "user_1",
            "tenant_1",
            ip,
            "免费优惠中奖红包",
        );
        
        assert!(anomaly.is_some());
        let event = anomaly.unwrap();
        assert_eq!(event.anomaly_type, AnomalyType::SuspiciousContent);
    }
}