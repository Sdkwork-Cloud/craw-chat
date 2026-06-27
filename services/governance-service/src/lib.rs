//! Governance service library for the Sdkwork IM control plane.
//!
//! This crate assembles the control-plane HTTP runtime: protocol registry,
//! protocol governance, provider registry, provider policy, and realtime node
//! lifecycle surfaces. Business logic, handlers, DTOs, errors, OpenAPI
//! generation, and app wiring live in focused submodules.

mod app;
mod dto;
mod error;
mod handlers;
mod helpers;
mod openapi;
mod state;

pub use app::{
    apply_public_http_guardrails, build_app, build_app_with_cluster,
    build_app_with_cluster_and_governance_sinks, build_app_with_cluster_and_provider_registry,
    build_app_with_cluster_and_runtime_provider_registry,
    build_app_with_cluster_provider_registry_and_governance_sinks,
    build_app_with_cluster_runtime_provider_registry_and_governance_sinks,
    build_control_surface_with_cluster_and_governance_sinks, build_domain_api_router,
    build_public_app, default_control_state,
};
pub use openapi::{export_openapi_document, export_openapi_spec, render_openapi_document};
pub use state::AppState;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn unix_epoch_seconds(at: SystemTime) -> u64 {
        at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    #[test]
    fn test_unix_epoch_seconds_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_secs(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_seconds(before_epoch), 0);
    }

    #[test]
    fn test_unix_epoch_seconds_preserves_post_epoch_time() {
        let after_epoch = UNIX_EPOCH + Duration::from_secs(42);
        assert_eq!(unix_epoch_seconds(after_epoch), 42);
    }
}
