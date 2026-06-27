//! Sdkwork IM cloud gateway library.
//!
//! This crate assembles the cloud-deployment gateway for the Sdkwork IM
//! application: upstream HTTP/websocket proxying, route precedence, OpenAPI
//! aggregation, CORS, rate limiting, and circuit breaking. Public entrypoints
//! are the [`build_app`] family for router construction and
//! [`build_gateway_registry`] for the route registry.

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
