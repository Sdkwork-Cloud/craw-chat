//! Sdkwork IM streaming service.
//!
//! Stream session lifecycle and frame append/query flows for the IM platform.

pub mod app;
pub mod dto;
pub mod error;
pub mod handlers;
pub mod helpers;
pub mod openapi;
pub mod state;

#[cfg(test)]
mod tests;

pub use crate::app::{
    apply_public_http_guardrails, build_app, build_default_app, build_domain_api_router,
    build_public_app,
};
pub use crate::dto::{
    AbortStreamRequest, AppendStreamFrameOutcome, AppendStreamFrameRequest,
    CheckpointStreamRequest, CompleteStreamRequest, ListStreamFramesQuery, OpenStreamRequest,
    StreamFrameDeliveryStatus, StreamFrameMutationResponse, StreamFrameWindow,
    StreamSessionDeliveryStatus, StreamSessionMutationOutcome, StreamSessionMutationResponse,
};
pub use crate::error::StreamingError;
pub use crate::helpers::{
    stream_abort_request_key, stream_append_request_key, stream_checkpoint_request_key,
    stream_complete_request_key, stream_open_request_key,
};
pub use crate::state::{AppState, StreamingRuntime, default_app_state};
