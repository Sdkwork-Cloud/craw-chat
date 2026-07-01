//! Ops service runtime, HTTP adapters, and app builders.
//!
//! This crate owns ops diagnostics, cluster state, lag reporting,
//! retention purge, and the ops HTTP surface for the SDKWork IM
//! platform. Business logic lives in [`OpsRuntime`]; HTTP handlers,
//! app builders, OpenAPI helpers, and DTOs are split into focused
//! modules re-exported from here.

pub mod app;
pub mod dto;
pub mod error;
pub mod state;

mod handlers;
mod helpers;
mod openapi;

pub use app::{
    apply_public_http_guardrails, build_app, build_business_router, build_default_app,
    build_domain_api_router, build_public_app, default_app_state,
};
pub use dto::{
    ClusterNodeView, ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse,
    ProjectionPlaneDiagnosticsView, ProjectionPlaneHealthView, ProjectionPlaneLogView,
    ProjectionPlaneMetricCounterView, ProjectionPlaneMetricsView, ProjectionPlaneTraceView,
    ProjectionReplayMetricsView, ProjectionReplayStatusView, ProjectionUpdateDelayView,
    ProviderBindingDriftItemView, ProviderBindingDriftView, ProviderBindingItemView,
    ProviderBindingSnapshotView, ProviderBindingsView, RealtimeInboxDiagnosticsView,
    RealtimeInboxHighRiskWindowView, RetentionPurgeResponse, RouteOwnershipView,
    RuntimeDirInspectionItem, RuntimeDirInspectionView, ServiceHealthView,
    SideEffectOutboxDiagnosticsView,
};
pub use error::OpsError;
pub use state::{AppState, OpsRuntime};
