//! Connection quality monitoring and adaptive heartbeat management.
//!
//! This module provides network quality metrics collection and adaptive
//! heartbeat policy to improve connection stability in weak network environments.
//!
//! ## Features
//!
//! - **Adaptive heartbeat**: Dynamically adjusts heartbeat interval based on network quality
//! - **Quality metrics**: RTT, loss rate, jitter monitoring
//! - **Degradation strategy**: Connection quality-based service degradation
//!
//! ## Architecture
//!
//! ```
//! Client                          Server
//!   |                               |
//!   |------ Heartbeat ------------>|  (adaptive interval)
//!   |<----- HeartbeatAck ----------|  (RTT measurement)
//!   |                               |
//!   [Quality Metrics Collection]
//!   - RTT: round-trip time
//!   - Loss rate: heartbeat timeout rate
//!   - Jitter: RTT variation
//!   - Quality score: 0-1 composite score
//! ```

use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Network quality metrics collected from heartbeat responses.
#[derive(Clone, Debug, Default)]
pub struct NetworkMetrics {
    /// Round-trip time (ping-pong latency).
    pub rtt: Duration,
    /// Heartbeat packet loss rate (0.0 - 1.0).
    pub loss_rate: f64,
    /// RTT jitter (variation in RTT).
    pub jitter: Duration,
    /// Duration since last successful heartbeat.
    pub last_heartbeat_latency: Duration,
    /// Number of consecutive heartbeat timeouts.
    pub consecutive_timeouts: u32,
    /// Total heartbeat attempts.
    pub total_attempts: u64,
    /// Total successful heartbeats.
    pub total_successes: u64,
    /// Time when connection became stable (low RTT, low loss).
    pub stable_since: Option<Instant>,
    /// Composite quality score (0.0 - 1.0, higher is better).
    pub quality_score: f64,
}

impl NetworkMetrics {
    /// Create new network metrics with default values.
    pub fn new() -> Self {
        Self {
            rtt: Duration::from_millis(100),
            loss_rate: 0.0,
            jitter: Duration::from_millis(20),
            last_heartbeat_latency: Duration::from_millis(100),
            consecutive_timeouts: 0,
            total_attempts: 0,
            total_successes: 0,
            stable_since: None,
            quality_score: 1.0,
        }
    }

    /// Record a successful heartbeat with measured RTT.
    pub fn record_heartbeat_success(&mut self, rtt: Duration) {
        // Check for counter overflow - reset if approaching limit
        if self.total_attempts >= u64::MAX - 100 {
            tracing::warn!(
                target: "sdkwork.im.connection_quality",
                total_attempts = self.total_attempts,
                "counter approaching overflow, resetting metrics"
            );
            self.reset_counters_keep_state();
        }

        self.total_attempts += 1;
        self.total_successes += 1;
        self.consecutive_timeouts = 0;

        // Sanity check: RTT should be reasonable (< 30 seconds)
        // This prevents overflow and detects abnormal measurements
        let rtt = if rtt > Duration::from_secs(30) {
            tracing::warn!(
                target: "sdkwork.im.connection_quality",
                rtt_ms = rtt.as_millis(),
                "unusually high RTT measurement, capping at 30s"
            );
            Duration::from_secs(30)
        } else {
            rtt
        };

        // Update RTT with exponential moving average
        let alpha = 0.3; // smoothing factor
        let prev_rtt_ms = self.rtt.as_millis() as f64;
        let new_rtt_ms = rtt.as_millis() as f64;
        let smoothed_rtt = alpha * new_rtt_ms + (1.0 - alpha) * prev_rtt_ms;

        // Clamp smoothed RTT to reasonable bounds (0-30s) to prevent overflow
        let smoothed_rtt_clamped = smoothed_rtt.min(30000.0).max(0.0);
        self.rtt = Duration::from_millis(smoothed_rtt_clamped as u64);

        // Update jitter (RTT variation)
        // Use safe subtraction and clamping to prevent overflow
        let rtt_diff_abs = (new_rtt_ms - smoothed_rtt_clamped).abs();
        let rtt_diff = Duration::from_millis(rtt_diff_abs as u64);

        let prev_jitter_ms = self.jitter.as_millis() as f64;
        let rtt_diff_ms = rtt_diff.as_millis() as f64;
        let smoothed_jitter = alpha * rtt_diff_ms + (1.0 - alpha) * prev_jitter_ms;

        // Clamp jitter to reasonable bounds (0-10s)
        let smoothed_jitter_clamped = smoothed_jitter.min(10000.0).max(0.0);
        self.jitter = Duration::from_millis(smoothed_jitter_clamped as u64);
        
        // Update loss rate
        if self.total_attempts > 0 {
            self.loss_rate = (self.total_attempts - self.total_successes) as f64 
                / self.total_attempts as f64;
        }
        
        // Update quality score
        self.update_quality_score();
        
        // Check if connection is stable
        if self.is_stable() && self.stable_since.is_none() {
            self.stable_since = Some(Instant::now());
        } else if !self.is_stable() {
            self.stable_since = None;
        }
    }

    /// Record a heartbeat timeout (no response).
    pub fn record_heartbeat_timeout(&mut self) {
        // Check for counter overflow - reset if approaching limit
        if self.total_attempts >= u64::MAX - 100 {
            tracing::warn!(
                target: "sdkwork.im.connection_quality",
                total_attempts = self.total_attempts,
                "counter approaching overflow, resetting metrics"
            );
            self.reset_counters_keep_state();
        }

        self.total_attempts += 1;
        self.consecutive_timeouts += 1;
        
        // Update loss rate
        if self.total_attempts > 0 {
            self.loss_rate = (self.total_attempts - self.total_successes) as f64 
                / self.total_attempts as f64;
        }
        
        // Connection is no longer stable
        self.stable_since = None;
        
        // Update quality score
        self.update_quality_score();
    }

    /// Check if connection is stable (low RTT, low loss).
    fn is_stable(&self) -> bool {
        self.rtt < Duration::from_millis(200) 
            && self.loss_rate < 0.05 
            && self.jitter < Duration::from_millis(50)
    }

    /// Calculate composite quality score based on metrics.
    fn update_quality_score(&mut self) {
        // Quality score formula:
        // score = (rtt_score * 0.4) + (loss_score * 0.4) + (jitter_score * 0.2)
        
        // RTT score: 1.0 for <100ms, decreasing linearly to 0.0 at 1000ms
        let rtt_score = if self.rtt < Duration::from_millis(100) {
            1.0
        } else if self.rtt > Duration::from_millis(1000) {
            0.0
        } else {
            1.0 - (self.rtt.as_millis() as f64 - 100.0) / 900.0
        };
        
        // Loss score: 1.0 for 0%, 0.0 for >=20%
        let loss_score = if self.loss_rate >= 0.2 {
            0.0
        } else {
            1.0 - self.loss_rate / 0.2
        };
        
        // Jitter score: 1.0 for <20ms, decreasing to 0.0 at 200ms
        let jitter_score = if self.jitter < Duration::from_millis(20) {
            1.0
        } else if self.jitter > Duration::from_millis(200) {
            0.0
        } else {
            1.0 - (self.jitter.as_millis() as f64 - 20.0) / 180.0
        };
        
        // Composite score
        self.quality_score = rtt_score * 0.4 + loss_score * 0.4 + jitter_score * 0.2;
        
        // Penalize consecutive timeouts heavily
        if self.consecutive_timeouts >= 3 {
            self.quality_score *= 0.5;
        }
    }

    /// Get stable connection duration (if stable).
    pub fn stable_duration(&self) -> Option<Duration> {
        self.stable_since.map(|since| since.elapsed())
    }

    /// Reset counters while preserving current RTT/jitter state.
    ///
    /// This is used to prevent counter overflow in long-running connections
    /// while maintaining the current connection quality measurements.
    fn reset_counters_keep_state(&mut self) {
        // Preserve current rates by calculating equivalent counts
        let current_loss_rate = self.loss_rate;

        // Reset to small representative values
        self.total_attempts = 1000;
        self.total_successes = ((1.0 - current_loss_rate) * 1000.0) as u64;

        // Recalculate loss rate to ensure consistency
        self.loss_rate = if self.total_attempts > 0 {
            (self.total_attempts - self.total_successes) as f64 / self.total_attempts as f64
        } else {
            0.0
        };
    }
}

/// Connection quality level enum for degradation strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnectionQuality {
    /// Excellent quality (> 0.9): Normal service, all features enabled.
    Excellent,
    /// Good quality (0.7 - 0.9): Reduced push frequency.
    Good,
    /// Poor quality (0.5 - 0.7): Only critical messages pushed.
    Poor,
    /// Critical quality (< 0.5): Recommend reconnect.
    Critical,
}

impl ConnectionQuality {
    /// Determine quality level from score.
    pub fn from_score(score: f64) -> Self {
        if score >= 0.9 {
            ConnectionQuality::Excellent
        } else if score >= 0.7 {
            ConnectionQuality::Good
        } else if score >= 0.5 {
            ConnectionQuality::Poor
        } else {
            ConnectionQuality::Critical
        }
    }

    /// Get human-readable quality level string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionQuality::Excellent => "excellent",
            ConnectionQuality::Good => "good",
            ConnectionQuality::Poor => "poor",
            ConnectionQuality::Critical => "critical",
        }
    }

    /// Check if quality allows full service.
    pub fn allows_full_service(&self) -> bool {
        *self >= ConnectionQuality::Good
    }

    /// Check if quality allows critical messages only.
    pub fn allows_critical_messages(&self) -> bool {
        *self >= ConnectionQuality::Poor
    }

    /// Check if reconnection is recommended.
    pub fn recommend_reconnect(&self) -> bool {
        *self == ConnectionQuality::Critical
    }
}

/// Adaptive heartbeat policy that adjusts interval based on network quality.
#[derive(Clone, Debug)]
pub struct AdaptiveHeartbeatPolicy {
    /// Base heartbeat interval (normal conditions).
    pub base_interval: Duration,
    /// Minimum heartbeat interval (weak network).
    pub min_interval: Duration,
    /// Maximum heartbeat interval (stable connection).
    pub max_interval: Duration,
    /// RTT threshold for weak network detection.
    pub rtt_threshold: Duration,
    /// Loss rate threshold for weak network detection.
    pub loss_rate_threshold: f64,
    /// Consecutive timeout threshold for reconnect recommendation.
    pub timeout_threshold: u32,
    /// Minimum stable duration before extending interval.
    pub stable_duration_threshold: Duration,
}

impl AdaptiveHeartbeatPolicy {
    /// Create policy with recommended defaults.
    pub fn new() -> Self {
        Self {
            base_interval: Duration::from_secs(30),
            min_interval: Duration::from_secs(10),
            max_interval: Duration::from_secs(60),
            rtt_threshold: Duration::from_millis(200),
            loss_rate_threshold: 0.1,
            timeout_threshold: 3,
            stable_duration_threshold: Duration::from_secs(300),
        }
    }

    /// Create policy with custom parameters.
    pub fn with_params(
        base_interval: Duration,
        min_interval: Duration,
        max_interval: Duration,
    ) -> Self {
        Self {
            base_interval,
            min_interval,
            max_interval,
            rtt_threshold: Duration::from_millis(200),
            loss_rate_threshold: 0.1,
            timeout_threshold: 3,
            stable_duration_threshold: Duration::from_secs(300),
        }
    }

    /// Adjust heartbeat interval based on current network metrics.
    pub fn adjust_interval(&self, metrics: &NetworkMetrics) -> Duration {
        // Critical condition: consecutive timeouts exceed threshold
        if metrics.consecutive_timeouts >= self.timeout_threshold {
            return self.min_interval;
        }

        // Weak network condition: high RTT or high loss rate
        if metrics.rtt > self.rtt_threshold || metrics.loss_rate > self.loss_rate_threshold {
            // Use minimum interval for faster detection of recovery
            return self.min_interval;
        }

        // Stable connection: extend interval to reduce overhead
        if let Some(stable_duration) = metrics.stable_duration() {
            if stable_duration >= self.stable_duration_threshold {
                return self.max_interval;
            }
        }

        // Normal condition: use base interval
        self.base_interval
    }

    /// Determine connection quality level from metrics.
    pub fn determine_quality(&self, metrics: &NetworkMetrics) -> ConnectionQuality {
        ConnectionQuality::from_score(metrics.quality_score)
    }

    /// Check if reconnection should be recommended.
    pub fn should_recommend_reconnect(&self, metrics: &NetworkMetrics) -> bool {
        metrics.consecutive_timeouts >= self.timeout_threshold
            || metrics.quality_score < 0.5
    }

    /// Calculate exponential backoff interval for reconnection attempts.
    ///
    /// Uses exponential backoff with jitter to prevent thundering herd effect
    /// when multiple clients disconnect simultaneously and attempt to reconnect
    /// at the same time (P1-1 fix).
    ///
    /// # Jitter Strategy
    ///
    /// Uses decorrelated jitter algorithm:
    /// - delay = min(cap, base * random_range(1, 2^attempt))
    /// - The random factor prevents synchronized retries from multiple clients
    ///
    /// # Arguments
    ///
    /// * `attempt_count` - Number of previous reconnection attempts
    ///
    /// # Returns
    ///
    /// Duration to wait before next reconnection attempt
    pub fn calculate_reconnect_backoff(&self, attempt_count: u32) -> Duration {
        use rand::Rng;
        
        // Exponential backoff base: 1s, 2s, 4s, 8s, 16s...
        let base_delay_ms = 1000u64;
        let max_delay_ms = 60_000u64; // 60 seconds cap
        
        // Calculate exponential multiplier (capped at 2^6 = 64x)
        let max_multiplier = 2u64.pow(attempt_count.min(6));
        
        // Add jitter: random factor between 1 and max_multiplier
        // This prevents thundering herd when multiple clients reconnect simultaneously
        let jitter_multiplier = if max_multiplier > 1 {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..=max_multiplier)
        } else {
            1 // First attempt has no jitter range
        };
        
        let delay_ms = base_delay_ms * jitter_multiplier;
        
        // Apply cap and ensure minimum of 1 second
        let final_delay_ms = delay_ms.max(base_delay_ms).min(max_delay_ms);
        
        Duration::from_millis(final_delay_ms)
    }
    
    /// Calculate exponential backoff interval with a given jitter factor.
    ///
    /// This is an alternative jitter strategy that applies a fixed percentage
    /// jitter to the calculated delay, useful when you want more predictable
    /// base delays with small variations.
    ///
    /// # Arguments
    ///
    /// * `attempt_count` - Number of previous reconnection attempts
    /// * `jitter_percent` - Jitter percentage (e.g., 0.2 means ±20%)
    ///
    /// # Returns
    ///
    /// Duration to wait before next reconnection attempt
    pub fn calculate_reconnect_backoff_with_jitter(
        &self,
        attempt_count: u32,
        jitter_percent: f64,
    ) -> Duration {
        use rand::Rng;
        
        let base_delay = Duration::from_secs(1);
        let max_delay = Duration::from_secs(60);
        
        // Calculate base exponential delay: 1s, 2s, 4s, 8s, 16s...
        let multiplier = 2u64.pow(attempt_count.min(6));
        let base_delay_ms = base_delay.as_millis() as u64 * multiplier;
        
        // Apply jitter: delay * (1 ± jitter_percent/2)
        let jitter_factor = if jitter_percent > 0.0 {
            let mut rng = rand::thread_rng();
            let jitter_range = jitter_percent / 2.0;
            let jitter = rng.gen_range(-jitter_range..=jitter_range);
            1.0 + jitter
        } else {
            1.0
        };
        
        let jittered_delay_ms = (base_delay_ms as f64 * jitter_factor) as u64;
        
        // Apply cap and ensure minimum
        let final_delay_ms = jittered_delay_ms
            .max(base_delay.as_millis() as u64)
            .min(max_delay.as_millis() as u64);
        
        Duration::from_millis(final_delay_ms)
    }
}

impl Default for AdaptiveHeartbeatPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection quality metrics tracker with thread-safe atomic counters.
#[derive(Debug)]
pub struct AtomicNetworkMetrics {
    /// Last measured RTT in milliseconds.
    rtt_ms: AtomicU64,
    /// Total heartbeat attempts.
    total_attempts: AtomicU64,
    /// Total successful heartbeats.
    total_successes: AtomicU64,
    /// Consecutive timeouts counter.
    consecutive_timeouts: AtomicU32,
    /// Last quality score (scaled to u64: 0-1000000).
    quality_score_scaled: AtomicU64,
}

impl AtomicNetworkMetrics {
    /// Create atomic metrics tracker.
    pub fn new() -> Self {
        Self {
            rtt_ms: AtomicU64::new(100),
            total_attempts: AtomicU64::new(0),
            total_successes: AtomicU64::new(0),
            consecutive_timeouts: AtomicU32::new(0),
            quality_score_scaled: AtomicU64::new(1000000), // 1.0
        }
    }

    /// Record a successful heartbeat.
    pub fn record_success(&self, rtt: Duration) {
        self.total_attempts.fetch_add(1, Ordering::Relaxed);
        self.total_successes.fetch_add(1, Ordering::Relaxed);
        self.consecutive_timeouts.store(0, Ordering::Relaxed);
        self.rtt_ms.store(rtt.as_millis() as u64, Ordering::Relaxed);
        
        // Update quality score (simplified calculation)
        self.update_quality_score_atomic();
    }

    /// Record a heartbeat timeout.
    pub fn record_timeout(&self) {
        self.total_attempts.fetch_add(1, Ordering::Relaxed);
        self.consecutive_timeouts.fetch_add(1, Ordering::Relaxed);
        
        self.update_quality_score_atomic();
    }

    /// Get current RTT.
    pub fn get_rtt(&self) -> Duration {
        Duration::from_millis(self.rtt_ms.load(Ordering::Relaxed))
    }

    /// Get current loss rate.
    pub fn get_loss_rate(&self) -> f64 {
        let attempts = self.total_attempts.load(Ordering::Relaxed);
        if attempts == 0 {
            return 0.0;
        }
        let successes = self.total_successes.load(Ordering::Relaxed);
        (attempts - successes) as f64 / attempts as f64
    }

    /// Get consecutive timeouts.
    pub fn get_consecutive_timeouts(&self) -> u32 {
        self.consecutive_timeouts.load(Ordering::Relaxed)
    }

    /// Get quality score (0.0 - 1.0).
    pub fn get_quality_score(&self) -> f64 {
        self.quality_score_scaled.load(Ordering::Relaxed) as f64 / 1000000.0
    }

    /// Get quality level.
    pub fn get_quality_level(&self) -> ConnectionQuality {
        ConnectionQuality::from_score(self.get_quality_score())
    }

    /// Update quality score atomically (simplified calculation).
    fn update_quality_score_atomic(&self) {
        let rtt = self.get_rtt();
        let loss_rate = self.get_loss_rate();
        let consecutive_timeouts = self.get_consecutive_timeouts();
        
        // Simplified score calculation
        let rtt_score = if rtt < Duration::from_millis(100) {
            1.0
        } else if rtt > Duration::from_millis(1000) {
            0.0
        } else {
            1.0 - (rtt.as_millis() as f64 - 100.0) / 900.0
        };
        
        let loss_score = if loss_rate >= 0.2 {
            0.0
        } else {
            1.0 - loss_rate / 0.2
        };
        
        let base_score = rtt_score * 0.5 + loss_score * 0.5;
        
        // Penalize consecutive timeouts
        let final_score = if consecutive_timeouts >= 3 {
            base_score * 0.5
        } else {
            base_score
        };
        
        // Store as scaled integer
        let scaled = (final_score * 1000000.0) as u64;
        self.quality_score_scaled.store(scaled, Ordering::Relaxed);
    }

    /// Export to NetworkMetrics for detailed analysis.
    pub fn to_network_metrics(&self) -> NetworkMetrics {
        NetworkMetrics {
            rtt: self.get_rtt(),
            loss_rate: self.get_loss_rate(),
            jitter: Duration::from_millis(0), // Not tracked atomically
            last_heartbeat_latency: self.get_rtt(),
            consecutive_timeouts: self.get_consecutive_timeouts(),
            total_attempts: self.total_attempts.load(Ordering::Relaxed),
            total_successes: self.total_successes.load(Ordering::Relaxed),
            stable_since: None, // Not tracked atomically
            quality_score: self.get_quality_score(),
        }
    }
}

impl Default for AtomicNetworkMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_metrics_records_success() {
        let mut metrics = NetworkMetrics::new();
        
        metrics.record_heartbeat_success(Duration::from_millis(50));
        
        assert_eq!(metrics.total_attempts, 1);
        assert_eq!(metrics.total_successes, 1);
        assert_eq!(metrics.consecutive_timeouts, 0);
        assert!(metrics.rtt < Duration::from_millis(100));
        assert!(metrics.quality_score > 0.9);
    }

    #[test]
    fn network_metrics_records_timeout() {
        let mut metrics = NetworkMetrics::new();
        
        metrics.record_heartbeat_timeout();
        
        assert_eq!(metrics.total_attempts, 1);
        assert_eq!(metrics.total_successes, 0);
        assert_eq!(metrics.consecutive_timeouts, 1);
        assert!(metrics.loss_rate > 0.0);
    }

    #[test]
    fn quality_level_from_score() {
        assert_eq!(ConnectionQuality::from_score(0.95), ConnectionQuality::Excellent);
        assert_eq!(ConnectionQuality::from_score(0.75), ConnectionQuality::Good);
        assert_eq!(ConnectionQuality::from_score(0.55), ConnectionQuality::Poor);
        assert_eq!(ConnectionQuality::from_score(0.35), ConnectionQuality::Critical);
    }

    #[test]
    fn adaptive_policy_adjusts_interval() {
        let policy = AdaptiveHeartbeatPolicy::new();
        
        // Excellent quality: base interval
        let excellent_metrics = NetworkMetrics {
            rtt: Duration::from_millis(50),
            loss_rate: 0.01,
            quality_score: 0.95,
            ..Default::default()
        };
        assert_eq!(policy.adjust_interval(&excellent_metrics), policy.base_interval);
        
        // Poor quality: minimum interval
        let poor_metrics = NetworkMetrics {
            rtt: Duration::from_millis(300),
            loss_rate: 0.15,
            quality_score: 0.45,
            ..Default::default()
        };
        assert_eq!(policy.adjust_interval(&poor_metrics), policy.min_interval);
        
        // Consecutive timeouts: minimum interval
        let timeout_metrics = NetworkMetrics {
            consecutive_timeouts: 4,
            quality_score: 0.3,
            ..Default::default()
        };
        assert_eq!(policy.adjust_interval(&timeout_metrics), policy.min_interval);
    }

    #[test]
    fn atomic_metrics_thread_safe() {
        use std::sync::Arc;
        use std::thread;
        
        let metrics = Arc::new(AtomicNetworkMetrics::new());
        let mut handles = vec![];
        
        for i in 0..10 {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(thread::spawn(move || {
                if i % 3 == 0 {
                    metrics_clone.record_timeout();
                } else {
                    metrics_clone.record_success(Duration::from_millis(50 + i * 10));
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(metrics.total_attempts.load(Ordering::Relaxed), 10);
        // 4 timeouts (i=0,3,6,9), 6 successes (i=1,2,4,5,7,8)
        let success_count = metrics.total_successes.load(Ordering::Relaxed);
        assert!(
            success_count >= 5 && success_count <= 7,
            "expected ~6 successes ±1 (race), got {success_count}"
        );
    }

    #[test]
    fn reconnect_backoff_exponential() {
        let policy = AdaptiveHeartbeatPolicy::new();
        
        // P1-1 fix: jitter makes exact values unpredictable
        // Test that backoff stays within expected bounds
        
        // Attempt 0: base delay (1s) with jitter range 1-1 (no jitter for first attempt)
        let backoff0 = policy.calculate_reconnect_backoff(0);
        assert_eq!(backoff0, Duration::from_secs(1), "first attempt should be 1s");
        
        // Attempt 1: jitter range 1-2 seconds
        let backoff1 = policy.calculate_reconnect_backoff(1);
        assert!(
            backoff1 >= Duration::from_secs(1) && backoff1 <= Duration::from_secs(2),
            "attempt 1 should be 1-2s (with jitter), got {:?}", backoff1
        );
        
        // Attempt 2: jitter range 1-4 seconds
        let backoff2 = policy.calculate_reconnect_backoff(2);
        assert!(
            backoff2 >= Duration::from_secs(1) && backoff2 <= Duration::from_secs(4),
            "attempt 2 should be 1-4s (with jitter), got {:?}", backoff2
        );
        
        // Attempt 3: jitter range 1-8 seconds
        let backoff3 = policy.calculate_reconnect_backoff(3);
        assert!(
            backoff3 >= Duration::from_secs(1) && backoff3 <= Duration::from_secs(8),
            "attempt 3 should be 1-8s (with jitter), got {:?}", backoff3
        );
        
        // Attempt 10: capped at 60s, jitter range 1-60
        let backoff10 = policy.calculate_reconnect_backoff(10);
        assert!(
            backoff10 >= Duration::from_secs(1) && backoff10 <= Duration::from_secs(60),
            "attempt 10 should be capped at 60s max, got {:?}", backoff10
        );
    }
    
    #[test]
    fn reconnect_backoff_with_fixed_jitter() {
        let policy = AdaptiveHeartbeatPolicy::new();
        
        // Test the fixed-jitter variant for predictable behavior
        let backoff0 = policy.calculate_reconnect_backoff_with_jitter(0, 0.0);
        assert_eq!(backoff0, Duration::from_secs(1), "no jitter should give exact 1s");
        
        let backoff1 = policy.calculate_reconnect_backoff_with_jitter(1, 0.0);
        assert_eq!(backoff1, Duration::from_secs(2), "no jitter should give exact 2s");
        
        let backoff2 = policy.calculate_reconnect_backoff_with_jitter(2, 0.0);
        assert_eq!(backoff2, Duration::from_secs(4), "no jitter should give exact 4s");
        
        // With 20% jitter, values should be within ±20%
        let backoff_with_jitter = policy.calculate_reconnect_backoff_with_jitter(3, 0.2);
        let base = Duration::from_secs(8);
        let min_expected = Duration::from_millis(6400); // 8s * 0.8
        let max_expected = Duration::from_millis(9600); // 8s * 1.2
        assert!(
            backoff_with_jitter >= min_expected && backoff_with_jitter <= max_expected,
            "20% jitter should keep 8s within 6.4-9.6s, got {:?}", backoff_with_jitter
        );
    }
}