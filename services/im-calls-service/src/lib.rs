//! Sdkwork IM call signaling service.
//!
//! RTC call session lifecycle and signal relay flows for the IM platform.

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
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcParticipantCredentialResponse, RtcSessionDeliveryStatus,
    RtcSessionMutationOutcome, RtcSessionMutationResponse, RtcSignalEventResponse,
    UpdateRtcSessionRequest,
};
pub use crate::error::CallingError;
pub use crate::helpers::{
    rtc_session_accept_request_key, rtc_session_create_request_key, rtc_session_end_request_key,
    rtc_session_invite_request_key, rtc_session_reject_request_key,
};
pub use crate::state::{AppState, CallingRuntime, default_app_state};