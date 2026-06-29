//! Sdkwork IM cloud gateway library.
//!
//! This crate assembles the cloud-deployment gateway for the Sdkwork IM
//! application: upstream HTTP/websocket proxying, route precedence, OpenAPI
//! aggregation, CORS, rate limiting, and circuit breaking. Public entrypoints
//! are the [`build_app`] family for router construction and
//! [`build_gateway_registry`] for the route registry.
//!
//! # IM-owned Call Signaling Boundary
//!
//! Per `../sdkwork-rtc/docs/rtc-im-boundary.md`, IM owns the call signaling
//! surface `/im/v3/api/calls/*`. The gateway routes these paths to the
//! IM-owned upstream `im-calls-service`, which orchestrates call state
//! machines, signal delivery, and provider media session creation through
//! the `RtcProviderPort` contract from `sdkwork-rtc`. RTC media runtime
//! paths (the RTC app-api surface) and the retired RTC-owned signaling
//! service are out of scope for this gateway; call signaling is owned by
//! `im-calls-service` and media runtime is owned by the RTC product.
//!
//! # Dependency App-API Assembly
//!
//! Sibling product app-api surfaces are registered in [`registry`] and proxied
//! through the shared platform gateway root unless a split upstream override is
//! configured in `sdkwork-im-cloud-gateway-config`.
//!
//! - `"sdkwork-drive-app-api"` -> `/app/v3/api/drive/{*path}` -> `SdkworkDriveAppSdk`
//! - `"sdkwork-notary-app-api"` -> `/app/v3/api/notary/{*path}` -> `SdkworkNotaryAppSdk`
//! - `"sdkwork-course-app-api"` -> `COURSE_APP_API_SEGMENTS` (`courses`, `course_applications`) ->
//!   `SdkworkCourseAppSdk`
//! - `"sdkwork-knowledgebase-app-api"` -> `/app/v3/api/knowledge/{*path}` ->
//!   `SdkworkKnowledgebaseAppSdk`

pub mod gateway_protection;

mod app;
mod client;
mod constants;
mod cors;
mod embedded_session_gateway;
mod openapi;
mod proxy;
mod registry;
mod response;
mod runtime;
mod state;
mod web_framework;
mod websocket;
mod websocket_auth;

pub use app::{
    build_app, build_app_with_registry, build_app_with_registry_and_product_runtime,
    build_app_with_registry_product_runtime_and_embedded_services,
    build_app_with_registry_product_runtime_and_embedded_services_from_env,
};
pub use embedded_session_gateway::{
    EmbeddedSessionGatewayRuntime, bootstrap_embedded_session_gateway_runtime,
};
pub use registry::build_gateway_registry;
pub use state::GatewayState;
