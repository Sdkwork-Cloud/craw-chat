//! Automation service: execution lifecycle, agent response streams, tool call workflows,
//! governance inspection, and OpenAPI surface for the SDKWork IM automation capability.
//!
//! This crate is a public module assembly file. Business logic, handlers, DTOs, errors,
//! and runtime state live in focused submodules re-exported below.

pub mod app;
pub mod bootstrap;
pub mod dto;
pub mod error;
pub mod runtime;
pub mod state;

mod constants;
mod handlers;
mod helpers;
mod metrics;
mod openapi;

// Re-export domain types so consumers can access them from the crate root.
pub use im_domain_core::automation::{
    AgentToolCall, AgentToolCallState, AutomationExecution, AutomationExecutionState,
};

// Re-export public DTOs and result types.
pub use dto::{
    AppendAgentResponseDeltaRequest, AutomationExecutionDeliveryStatus,
    AutomationExecutionRequestResponse, AutomationExecutionRequestResult,
    AutomationGovernanceSnapshot, CompleteAgentResponseRequest, CompleteAgentToolCallRequest,
    RequestAgentToolCallRequest, RequestAutomationExecution, StartAgentResponseRequest,
};

// Re-export error and state types.
pub use bootstrap::{
    build_runtime_from_env, default_app_state, default_automation_runtime,
    ensure_durable_automation_runtime_from_env,
};
pub use error::AutomationError;
pub use runtime::AutomationRuntime;
pub use state::AppState;

// Re-export app builders and router constructors.
pub use app::{
    apply_public_http_guardrails, build_app_api_router, build_app_public_router_from_api_router,
    build_backend_api_router, build_domain_api_router,
};
pub use openapi::build_automation_backend_openapi_document;

// Re-export public governance helpers.
pub use helpers::{
    automation_operator_override_permission, automation_tool_requires_operator_override,
};
