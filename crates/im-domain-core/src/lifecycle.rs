//! Graceful shutdown and health check probes for production readiness.
//!
//! This module provides lifecycle management following `OPERATIONS_SPEC.md` requirements
//! for zero-downtime deployments and observability integration.
//!
//! ## Design Principles
//!
//! - Graceful shutdown with configurable timeout
//! - Health check probes (liveness, readiness, startup)
//! - Connection draining before termination
//! - In-flight request tracking
//!
//! ## Usage
//!
//! ```rust
//! use im_domain_core::lifecycle::{GracefulShutdown, HealthCheckProbes, ServiceState};
//!
//! let shutdown = GracefulShutdown::new(Duration::from_secs(30));
//! let probes = HealthCheckProbes::new();
//!
//! // Register shutdown signal
//! shutdown.register_signal_handler();
//!
//! // Check health
//! if probes.is_ready() {
//!     // Accept new requests
//! }
//!
//! // Wait for graceful shutdown
//! shutdown.wait().await;
//! ```

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Lifecycle errors.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum LifecycleError {
    #[error("shutdown timeout exceeded: timeout={timeout_ms}ms, remaining_requests={remaining}")]
    ShutdownTimeout {
        timeout_ms: u64,
        remaining: u64,
    },
    
    #[error("service not ready: reason={reason}")]
    NotReady {
        reason: String,
    },
    
    #[error("health check failed: component={component}, message={message}")]
    HealthCheckFailed {
        component: String,
        message: String,
    },
    
    #[error("signal handler registration failed: {message}")]
    SignalHandlerError {
        message: String,
    },
}

/// Service state enum.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceState {
    /// Service is starting up.
    Starting,
    /// Service is ready to accept requests.
    Ready,
    /// Service is alive but not ready (e.g., warming up).
    AliveButNotReady,
    /// Service is draining connections.
    Draining,
    /// Service is shutting down.
    ShuttingDown,
    /// Service has stopped.
    Stopped,
}

impl ServiceState {
    /// Check if service can accept new requests.
    pub fn can_accept_requests(&self) -> bool {
        matches!(self, ServiceState::Ready)
    }
    
    /// Check if service is still alive (for liveness probe).
    pub fn is_alive(&self) -> bool {
        !matches!(self, ServiceState::Stopped)
    }
    
    /// Check if service is shutting down.
    pub fn is_shutting_down(&self) -> bool {
        matches!(self, ServiceState::Draining | ServiceState::ShuttingDown | ServiceState::Stopped)
    }
}

/// Health check result for a component.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Component name.
    pub component: String,
    /// Whether the check passed.
    pub healthy: bool,
    /// Optional message.
    pub message: Option<String>,
    /// Timestamp of the check.
    pub checked_at: String,
}

/// Health check probes for Kubernetes integration.
///
/// Implements three probe types:
/// - Liveness: Is the service process alive?
/// - Readiness: Can the service accept requests?
/// - Startup: Has the service finished initialization?
pub struct HealthCheckProbes {
    /// Current service state.
    state: Arc<std::sync::Mutex<ServiceState>>,
    /// Liveness check results.
    liveness_checks: HashMap<String, Box<dyn Fn() -> HealthCheckResult + Send + Sync>>,
    /// Readiness check results.
    readiness_checks: HashMap<String, Box<dyn Fn() -> HealthCheckResult + Send + Sync>>,
    /// Startup check results.
    startup_checks: HashMap<String, Box<dyn Fn() -> HealthCheckResult + Send + Sync>>,
    /// Startup deadline.
    startup_deadline: Instant,
    /// Startup duration.
    startup_duration: Duration,
}

impl HealthCheckProbes {
    /// Create new health check probes.
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::Mutex::new(ServiceState::Starting)),
            liveness_checks: HashMap::new(),
            readiness_checks: HashMap::new(),
            startup_checks: HashMap::new(),
            startup_deadline: Instant::now() + Duration::from_secs(120),
            startup_duration: Duration::from_secs(120),
        }
    }
    
    /// Create with custom startup duration.
    pub fn with_startup_duration(startup_duration: Duration) -> Self {
        Self {
            state: Arc::new(std::sync::Mutex::new(ServiceState::Starting)),
            liveness_checks: HashMap::new(),
            readiness_checks: HashMap::new(),
            startup_checks: HashMap::new(),
            startup_deadline: Instant::now() + startup_duration,
            startup_duration,
        }
    }
    
    /// Set service state.
    pub fn set_state(&self, new_state: ServiceState) {
        *self.state.lock().unwrap() = new_state;
    }
    
    /// Get current state.
    pub fn get_state(&self) -> ServiceState {
        self.state.lock().unwrap().clone()
    }
    
    /// Get configured startup duration.
    pub fn startup_duration(&self) -> Duration {
        self.startup_duration
    }
    
    /// Get remaining time until startup deadline.
    pub fn remaining_startup_time(&self) -> Option<Duration> {
        let remaining = self.startup_deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            None
        } else {
            Some(remaining)
        }
    }
    
    /// Register a liveness check.
    pub fn register_liveness_check(&mut self, name: &str, check: Box<dyn Fn() -> HealthCheckResult + Send + Sync>) {
        self.liveness_checks.insert(name.to_string(), check);
    }
    
    /// Register a readiness check.
    pub fn register_readiness_check(&mut self, name: &str, check: Box<dyn Fn() -> HealthCheckResult + Send + Sync>) {
        self.readiness_checks.insert(name.to_string(), check);
    }
    
    /// Register a startup check.
    pub fn register_startup_check(&mut self, name: &str, check: Box<dyn Fn() -> HealthCheckResult + Send + Sync>) {
        self.startup_checks.insert(name.to_string(), check);
    }
    
    /// Liveness probe - is the service process alive?
    pub fn is_alive(&self) -> bool {
        self.get_state().is_alive()
    }
    
    /// Readiness probe - can the service accept requests?
    pub fn is_ready(&self) -> bool {
        let state = self.get_state();
        if !state.can_accept_requests() {
            return false;
        }
        
        // Run all readiness checks
        for check in self.readiness_checks.values() {
            let result = check();
            if !result.healthy {
                return false;
            }
        }
        
        true
    }
    
    /// Startup probe - has the service finished initialization?
    pub fn is_startup_complete(&self) -> bool {
        // Check if we're past the startup deadline
        if Instant::now() > self.startup_deadline {
            return true;
        }
        
        let state = self.get_state();
        if state == ServiceState::Ready || state == ServiceState::AliveButNotReady {
            return true;
        }
        
        // Run all startup checks
        for check in self.startup_checks.values() {
            let result = check();
            if !result.healthy {
                return false;
            }
        }
        
        true
    }
    
    /// Get detailed health status.
    pub fn get_health_status(&self) -> ServiceHealthStatus {
        let now = chrono::Utc::now().to_rfc3339();
        let state = self.get_state();
        
        let liveness_results: Vec<HealthCheckResult> = self.liveness_checks.values()
            .map(|check| check())
            .collect();
        
        let readiness_results: Vec<HealthCheckResult> = self.readiness_checks.values()
            .map(|check| check())
            .collect();
        
        let startup_results: Vec<HealthCheckResult> = self.startup_checks.values()
            .map(|check| check())
            .collect();
        
        ServiceHealthStatus {
            state,
            alive: self.is_alive(),
            ready: self.is_ready(),
            startup_complete: self.is_startup_complete(),
            liveness_checks: liveness_results,
            readiness_checks: readiness_results,
            startup_checks: startup_results,
            checked_at: now,
        }
    }
}

impl Default for HealthCheckProbes {
    fn default() -> Self {
        Self::new()
    }
}

/// Service health status report.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceHealthStatus {
    pub state: ServiceState,
    pub alive: bool,
    pub ready: bool,
    pub startup_complete: bool,
    pub liveness_checks: Vec<HealthCheckResult>,
    pub readiness_checks: Vec<HealthCheckResult>,
    pub startup_checks: Vec<HealthCheckResult>,
    pub checked_at: String,
}

/// Graceful shutdown manager.
///
/// Tracks in-flight requests and waits for them to complete
/// before terminating the service.
#[derive(Clone, Debug)]
pub struct GracefulShutdown {
    /// Shutdown timeout.
    timeout: Duration,
    /// Shutdown signal received flag.
    shutdown_requested: Arc<AtomicBool>,
    /// Number of in-flight requests.
    in_flight_requests: Arc<AtomicU64>,
    /// Request tracking by ID.
    request_tracker: Arc<std::sync::Mutex<HashMap<String, Instant>>>,
    /// Shutdown started timestamp.
    shutdown_started: Arc<std::sync::Mutex<Option<Instant>>>,
}

impl GracefulShutdown {
    /// Create new shutdown manager with specified timeout.
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            in_flight_requests: Arc::new(AtomicU64::new(0)),
            request_tracker: Arc::new(std::sync::Mutex::new(HashMap::new())),
            shutdown_started: Arc::new(std::sync::Mutex::new(None)),
        }
    }
    
    /// Request shutdown.
    pub fn request_shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        *self.shutdown_started.lock().unwrap() = Some(Instant::now());
    }
    
    /// Check if shutdown has been requested.
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }
    
    /// Start tracking a request.
    pub fn track_request(&self, request_id: &str) {
        self.in_flight_requests.fetch_add(1, Ordering::SeqCst);
        self.request_tracker.lock().unwrap().insert(request_id.to_string(), Instant::now());
    }
    
    /// Finish tracking a request.
    pub fn finish_request(&self, request_id: &str) {
        self.in_flight_requests.fetch_sub(1, Ordering::SeqCst);
        self.request_tracker.lock().unwrap().remove(request_id);
    }
    
    /// Get number of in-flight requests.
    pub fn get_in_flight_count(&self) -> u64 {
        self.in_flight_requests.load(Ordering::SeqCst)
    }
    
    /// Wait for all in-flight requests to complete.
    ///
    /// Returns Ok if all requests completed within timeout.
    /// Returns Err if timeout exceeded.
    pub fn wait_for_completion(&self) -> Result<(), LifecycleError> {
        let start = self.shutdown_started.lock().unwrap();
        let deadline = start.unwrap_or(Instant::now()) + self.timeout;
        
        while self.in_flight_requests.load(Ordering::SeqCst) > 0 {
            if Instant::now() > deadline {
                return Err(LifecycleError::ShutdownTimeout {
                    timeout_ms: self.timeout.as_millis() as u64,
                    remaining: self.in_flight_requests.load(Ordering::SeqCst),
                });
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        
        Ok(())
    }
    
    /// Force shutdown after timeout (cancel remaining requests).
    pub fn force_shutdown(&self) {
        self.request_tracker.lock().unwrap().clear();
        self.in_flight_requests.store(0, Ordering::SeqCst);
    }
    
    /// Get requests older than specified age (for draining).
    pub fn get_stale_requests(&self, max_age: Duration) -> Vec<String> {
        let now = Instant::now();
        self.request_tracker.lock().unwrap()
            .iter()
            .filter(|(_, start)| now.duration_since(**start) > max_age)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

impl Default for GracefulShutdown {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn service_state_can_accept_requests() {
        assert!(ServiceState::Ready.can_accept_requests());
        assert!(!ServiceState::Starting.can_accept_requests());
        assert!(!ServiceState::Draining.can_accept_requests());
        assert!(!ServiceState::ShuttingDown.can_accept_requests());
    }
    
    #[test]
    fn service_state_is_alive() {
        assert!(ServiceState::Starting.is_alive());
        assert!(ServiceState::Ready.is_alive());
        assert!(ServiceState::AliveButNotReady.is_alive());
        assert!(!ServiceState::Stopped.is_alive());
    }
    
    #[test]
    fn health_check_probes_initial_state() {
        let probes = HealthCheckProbes::new();
        assert_eq!(probes.get_state(), ServiceState::Starting);
        assert!(probes.is_alive());
        assert!(!probes.is_ready());
    }
    
    #[test]
    fn health_check_probes_set_state() {
        let probes = HealthCheckProbes::new();
        probes.set_state(ServiceState::Ready);
        assert_eq!(probes.get_state(), ServiceState::Ready);
        assert!(probes.is_ready());
    }
    
    #[test]
    fn health_check_with_custom_check() {
        let mut probes = HealthCheckProbes::new();
        probes.set_state(ServiceState::Ready);
        
        probes.register_readiness_check("db", Box::new(|| {
            HealthCheckResult {
                component: "database".to_string(),
                healthy: true,
                message: Some("connected".to_string()),
                checked_at: chrono::Utc::now().to_rfc3339(),
            }
        }));
        
        assert!(probes.is_ready());
        
        let status = probes.get_health_status();
        assert_eq!(status.readiness_checks.len(), 1);
        assert!(status.readiness_checks[0].healthy);
    }
    
    #[test]
    fn health_check_failed_check() {
        let mut probes = HealthCheckProbes::new();
        probes.set_state(ServiceState::Ready);
        
        probes.register_readiness_check("db", Box::new(|| {
            HealthCheckResult {
                component: "database".to_string(),
                healthy: false,
                message: Some("connection failed".to_string()),
                checked_at: chrono::Utc::now().to_rfc3339(),
            }
        }));
        
        assert!(!probes.is_ready());
    }
    
    #[test]
    fn graceful_shutdown_initial_state() {
        let shutdown = GracefulShutdown::new(Duration::from_secs(30));
        assert!(!shutdown.is_shutdown_requested());
        assert_eq!(shutdown.get_in_flight_count(), 0);
    }
    
    #[test]
    fn graceful_shutdown_request() {
        let shutdown = GracefulShutdown::new(Duration::from_secs(30));
        shutdown.request_shutdown();
        assert!(shutdown.is_shutdown_requested());
    }
    
    #[test]
    fn graceful_shutdown_tracking() {
        let shutdown = GracefulShutdown::new(Duration::from_secs(30));
        
        shutdown.track_request("req-1");
        shutdown.track_request("req-2");
        assert_eq!(shutdown.get_in_flight_count(), 2);
        
        shutdown.finish_request("req-1");
        assert_eq!(shutdown.get_in_flight_count(), 1);
        
        shutdown.finish_request("req-2");
        assert_eq!(shutdown.get_in_flight_count(), 0);
    }
    
    #[test]
    fn graceful_shutdown_wait_completion() {
        let shutdown = GracefulShutdown::new(Duration::from_millis(100));
        
        shutdown.track_request("req-1");
        shutdown.request_shutdown();
        
        // Simulate request completion
        shutdown.finish_request("req-1");
        
        let result = shutdown.wait_for_completion();
        assert!(result.is_ok());
    }
    
    #[test]
    fn graceful_shutdown_timeout() {
        let shutdown = GracefulShutdown::new(Duration::from_millis(50));
        
        shutdown.track_request("req-1");
        shutdown.request_shutdown();
        
        // Don't finish the request
        let result = shutdown.wait_for_completion();
        assert!(matches!(result, Err(LifecycleError::ShutdownTimeout { .. })));
    }
    
    #[test]
    fn graceful_shutdown_force() {
        let shutdown = GracefulShutdown::new(Duration::from_secs(30));
        
        shutdown.track_request("req-1");
        shutdown.track_request("req-2");
        assert_eq!(shutdown.get_in_flight_count(), 2);
        
        shutdown.force_shutdown();
        assert_eq!(shutdown.get_in_flight_count(), 0);
    }
    
    #[test]
    fn graceful_shutdown_stale_requests() {
        let shutdown = GracefulShutdown::new(Duration::from_secs(30));
        
        shutdown.track_request("old-req");
        // Simulate time passage by sleeping
        std::thread::sleep(Duration::from_millis(200));
        shutdown.track_request("new-req");
        
        let stale = shutdown.get_stale_requests(Duration::from_millis(100));
        assert!(stale.contains(&"old-req".to_string()));
        assert!(!stale.contains(&"new-req".to_string()));
    }
}