//! Priority queue for RTC signaling messages.
//!
//! This module implements a priority-based queue for RTC signaling messages
//! to ensure critical messages (offer/answer/candidate) are delivered first
//! during call setup and negotiation.
//!
//! ## Priority Levels
//!
//! - **Critical (0)**: SDP offer/answer, ICE candidates - must be delivered immediately
//! - **High (1)**: Call accept/reject/end - important for call state
//! - **Normal (2)**: Regular signaling messages
//! - **Low (3)**: Statistics, diagnostics - can be delayed
//!
//! ## Architecture
//!
//! ```text
//! [Critical Queue] -> offer, answer, candidate
//! [High Queue]    -> accept, reject, end
//! [Normal Queue]  -> regular messages
//! [Low Queue]     -> stats, diagnostics
//!
//! Pop order: Critical -> High -> Normal -> Low
//! ```

use std::collections::VecDeque;

use im_domain_core::rtc::SignalEvent;
use serde::{Deserialize, Serialize};

/// Signaling message priority level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RtcSignalPriority {
    /// Critical priority: SDP offer/answer, ICE candidates.
    /// Must be delivered immediately for call setup.
    Critical = 0,
    /// High priority: Call accept/reject/end.
    /// Important for call state transitions.
    High = 1,
    /// Normal priority: Regular signaling messages.
    Normal = 2,
    /// Low priority: Statistics, diagnostics.
    /// Can be delayed or dropped under load.
    Low = 3,
}

impl RtcSignalPriority {
    /// Classify signal type into priority level.
    pub fn from_signal_type(signal_type: &str) -> Self {
        match signal_type {
            // SDP offer/answer - critical for call setup
            "sdp.offer" | "sdp.answer" => RtcSignalPriority::Critical,
            
            // ICE candidates - critical for connectivity
            "ice.candidate" | "ice.candidate.complete" => RtcSignalPriority::Critical,
            
            // Call state changes - high priority
            "call.accept" | "call.reject" | "call.end" => RtcSignalPriority::High,
            
            // Media control - normal priority
            "media.mute" | "media.unmute" | "media.pause" | "media.resume" => RtcSignalPriority::Normal,
            
            // Statistics and diagnostics - low priority
            "stats.report" | "diagnostics.ping" | "diagnostics.pong" => RtcSignalPriority::Low,
            
            // Default to normal priority
            _ => RtcSignalPriority::Normal,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RtcSignalPriority::Critical => "critical",
            RtcSignalPriority::High => "high",
            RtcSignalPriority::Normal => "normal",
            RtcSignalPriority::Low => "low",
        }
    }
}

/// Priority queue statistics.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PriorityQueueStats {
    /// Total messages enqueued.
    pub total_enqueued: u64,
    /// Total messages dequeued.
    pub total_dequeued: u64,
    /// Current queue depth by priority.
    pub queue_depths: [usize; 4],
    /// Messages dropped due to overflow.
    pub messages_dropped: u64,
    /// Maximum queue depth observed.
    pub max_depth: usize,
}

/// Priority-based signal queue with overflow protection.
#[derive(Clone, Debug)]
pub struct PrioritySignalQueue {
    /// Per-priority queues (index = priority level).
    queues: [VecDeque<SignalEvent>; 4],
    /// Maximum queue depth per priority level.
    max_depth_per_priority: usize,
    /// Statistics tracker.
    stats: PriorityQueueStats,
}

impl PrioritySignalQueue {
    /// Create new priority queue with default max depth (1000 per priority).
    pub fn new() -> Self {
        Self::with_max_depth(1000)
    }

    /// Create new priority queue with custom max depth per priority level.
    pub fn with_max_depth(max_depth_per_priority: usize) -> Self {
        Self {
            queues: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            max_depth_per_priority,
            stats: PriorityQueueStats::default(),
        }
    }

    /// Push a signal into the appropriate priority queue.
    /// Returns false if the queue is full and the message was dropped.
    pub fn push(&mut self, signal: SignalEvent) -> bool {
        let priority = RtcSignalPriority::from_signal_type(&signal.signal_type);
        let queue_index = priority as usize;
        
        // Check if queue is full
        if self.queues[queue_index].len() >= self.max_depth_per_priority {
            self.stats.messages_dropped += 1;
            tracing::warn!(
                target: "sdkwork.im.rtc",
                event = "rtc.signal_queue.overflow",
                priority = %priority.as_str(),
                signal_type = %signal.signal_type,
                "signal queue overflow - message dropped"
            );
            return false;
        }
        
        self.queues[queue_index].push_back(signal);
        self.stats.total_enqueued += 1;
        
        // Update queue depths
        self.update_queue_depths();
        
        true
    }

    /// Push a signal with explicit priority.
    pub fn push_with_priority(&mut self, signal: SignalEvent, priority: RtcSignalPriority) -> bool {
        let queue_index = priority as usize;
        
        if self.queues[queue_index].len() >= self.max_depth_per_priority {
            self.stats.messages_dropped += 1;
            return false;
        }
        
        self.queues[queue_index].push_back(signal);
        self.stats.total_enqueued += 1;
        self.update_queue_depths();
        
        true
    }

    /// Pop the highest priority signal available.
    /// Returns None if all queues are empty.
    pub fn pop(&mut self) -> Option<SignalEvent> {
        // Check queues in priority order
        for queue in &mut self.queues {
            if let Some(signal) = queue.pop_front() {
                self.stats.total_dequeued += 1;
                self.update_queue_depths();
                return Some(signal);
            }
        }
        None
    }

    /// Peek at the highest priority signal without removing it.
    pub fn peek(&self) -> Option<&SignalEvent> {
        for queue in &self.queues {
            if let Some(signal) = queue.front() {
                return Some(signal);
            }
        }
        None
    }

    /// Get total queue depth across all priorities.
    pub fn total_depth(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }

    /// Get queue depth for a specific priority.
    pub fn depth_for_priority(&self, priority: RtcSignalPriority) -> usize {
        self.queues[priority as usize].len()
    }

    /// Check if all queues are empty.
    pub fn is_empty(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }

    /// Get the next priority level that has messages.
    pub fn next_available_priority(&self) -> Option<RtcSignalPriority> {
        for (index, queue) in self.queues.iter().enumerate() {
            if !queue.is_empty() {
                return Some(match index {
                    0 => RtcSignalPriority::Critical,
                    1 => RtcSignalPriority::High,
                    2 => RtcSignalPriority::Normal,
                    3 => RtcSignalPriority::Low,
                    _ => RtcSignalPriority::Normal,
                });
            }
        }
        None
    }

    /// Clear all queues.
    pub fn clear(&mut self) {
        for queue in &mut self.queues {
            queue.clear();
        }
        self.update_queue_depths();
    }

    /// Drain all messages from a specific priority queue.
    pub fn drain_priority(&mut self, priority: RtcSignalPriority) -> Vec<SignalEvent> {
        let queue_index = priority as usize;
        let drained: Vec<_> = self.queues[queue_index].drain(..).collect();
        self.stats.total_dequeued += drained.len() as u64;
        self.update_queue_depths();
        drained
    }

    /// Get statistics.
    pub fn stats(&self) -> &PriorityQueueStats {
        &self.stats
    }

    /// Update queue depth statistics.
    fn update_queue_depths(&mut self) {
        for (i, queue) in self.queues.iter().enumerate() {
            self.stats.queue_depths[i] = queue.len();
        }
        
        let total = self.total_depth();
        if total > self.stats.max_depth {
            self.stats.max_depth = total;
        }
    }

    /// Rebalance queues by moving low-priority messages to make room for higher priority.
    /// Returns the number of messages evicted.
    pub fn rebalance(&mut self) -> usize {
        let mut evicted = 0;
        
        // If critical queue is near capacity, evict from low priority
        if self.queues[0].len() > self.max_depth_per_priority * 8 / 10 {
            let low_count = self.queues[3].len();
            let evict_count = low_count / 2;
            
            for _ in 0..evict_count {
                self.queues[3].pop_back();
                evicted += 1;
                self.stats.messages_dropped += 1;
            }
        }
        
        self.update_queue_depths();
        evicted
    }
}

impl Default for PrioritySignalQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch signal processor for efficient bulk operations.
pub struct BatchSignalProcessor {
    queue: PrioritySignalQueue,
    /// Maximum batch size for processing.
    batch_size: usize,
}

impl BatchSignalProcessor {
    pub fn new(queue: PrioritySignalQueue, batch_size: usize) -> Self {
        Self { queue, batch_size }
    }

    /// Push multiple signals at once.
    pub fn push_batch(&mut self, signals: Vec<SignalEvent>) -> usize {
        let mut accepted = 0;
        for signal in signals {
            if self.queue.push(signal) {
                accepted += 1;
            }
        }
        accepted
    }

    /// Pop up to batch_size signals in priority order.
    pub fn pop_batch(&mut self) -> Vec<SignalEvent> {
        let mut batch = Vec::with_capacity(self.batch_size);
        
        while batch.len() < self.batch_size {
            match self.queue.pop() {
                Some(signal) => batch.push(signal),
                None => break,
            }
        }
        
        batch
    }

    /// Pop all critical and high priority messages.
    pub fn pop_urgent(&mut self) -> Vec<SignalEvent> {
        let mut urgent = Vec::new();
        
        // Get all critical
        urgent.extend(self.queue.drain_priority(RtcSignalPriority::Critical));
        
        // Get all high
        urgent.extend(self.queue.drain_priority(RtcSignalPriority::High));
        
        urgent
    }

    /// Get the underlying queue.
    pub fn queue(&self) -> &PrioritySignalQueue {
        &self.queue
    }

    /// Get mutable access to the underlying queue.
    pub fn queue_mut(&mut self) -> &mut PrioritySignalQueue {
        &mut self.queue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::rtc::{SignalSender, SessionState};

    fn create_test_signal(signal_type: &str, seq: u64) -> SignalEvent {
        SignalEvent {
            tenant_id: "test".to_string(),
            rtc_session_id: "session_1".to_string(),
            signal_seq: seq,
            conversation_id: None,
            rtc_mode: "video".to_string(),
            signal_type: signal_type.to_string(),
            schema_ref: None,
            payload: "{}".to_string(),
            sender: SignalSender {
                id: "user_1".to_string(),
                kind: "user".to_string(),
                member_id: None,
                device_id: None,
                session_id: None,
                metadata: Default::default(),
            },
            signaling_stream_id: None,
            occurred_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn priority_classification() {
        assert_eq!(
            RtcSignalPriority::from_signal_type("sdp.offer"),
            RtcSignalPriority::Critical
        );
        assert_eq!(
            RtcSignalPriority::from_signal_type("ice.candidate"),
            RtcSignalPriority::Critical
        );
        assert_eq!(
            RtcSignalPriority::from_signal_type("call.accept"),
            RtcSignalPriority::High
        );
        assert_eq!(
            RtcSignalPriority::from_signal_type("media.mute"),
            RtcSignalPriority::Normal
        );
        assert_eq!(
            RtcSignalPriority::from_signal_type("stats.report"),
            RtcSignalPriority::Low
        );
    }

    #[test]
    fn queue_priority_ordering() {
        let mut queue = PrioritySignalQueue::new();
        
        // Push in reverse priority order
        queue.push(create_test_signal("stats.report", 1)); // Low
        queue.push(create_test_signal("media.mute", 2)); // Normal
        queue.push(create_test_signal("call.accept", 3)); // High
        queue.push(create_test_signal("sdp.offer", 4)); // Critical
        
        // Should pop in priority order
        assert_eq!(queue.pop().unwrap().signal_seq, 4); // Critical
        assert_eq!(queue.pop().unwrap().signal_seq, 3); // High
        assert_eq!(queue.pop().unwrap().signal_seq, 2); // Normal
        assert_eq!(queue.pop().unwrap().signal_seq, 1); // Low
        assert!(queue.pop().is_none());
    }

    #[test]
    fn queue_overflow_protection() {
        let mut queue = PrioritySignalQueue::with_max_depth(2);
        
        assert!(queue.push(create_test_signal("sdp.offer", 1)));
        assert!(queue.push(create_test_signal("sdp.offer", 2)));
        assert!(!queue.push(create_test_signal("sdp.offer", 3))); // Overflow
        
        assert_eq!(queue.stats().messages_dropped, 1);
    }

    #[test]
    fn batch_processing() {
        let queue = PrioritySignalQueue::new();
        let mut processor = BatchSignalProcessor::new(queue, 10);
        
        let signals: Vec<_> = (0..20)
            .map(|i| create_test_signal("sdp.offer", i))
            .collect();
        
        let accepted = processor.push_batch(signals);
        assert_eq!(accepted, 20);
        
        let batch = processor.pop_batch();
        assert_eq!(batch.len(), 10);
    }

    #[test]
    fn urgent_messages_extraction() {
        let queue = PrioritySignalQueue::new();
        let mut processor = BatchSignalProcessor::new(queue, 10);
        
        processor.queue_mut().push(create_test_signal("stats.report", 1)); // Low
        processor.queue_mut().push(create_test_signal("sdp.offer", 2)); // Critical
        processor.queue_mut().push(create_test_signal("call.accept", 3)); // High
        processor.queue_mut().push(create_test_signal("sdp.answer", 4)); // Critical
        
        let urgent = processor.pop_urgent();
        
        // Should have all critical and high priority messages
        assert_eq!(urgent.len(), 3);
        assert!(urgent.iter().all(|s| s.signal_seq != 1)); // Low priority excluded
    }
}