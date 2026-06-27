//! Automation service: execution lifecycle, agent response streams, tool call workflows,
//! governance inspection, and OpenAPI surface for the SDKWork IM automation capability.
//!
//! This crate is a public module assembly file. Business logic, handlers, DTOs, errors,
//! and runtime state live in focused submodules re-exported below.

pub mod app;
pub mod dto;
pub mod error;
pub mod runtime;
pub mod state;

mod constants;
mod handlers;
mod helpers;
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
pub use error::AutomationError;
pub use runtime::AutomationRuntime;
pub use state::{AppState, default_app_state};

// Re-export app builders and router constructors.
pub use app::{
    apply_public_http_guardrails, build_app, build_default_app, build_domain_api_router,
    build_public_app,
};

// Re-export public governance helpers.
pub use helpers::{automation_operator_override_permission, automation_tool_requires_operator_override};
