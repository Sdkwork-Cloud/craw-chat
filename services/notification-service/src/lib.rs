//! Notification service runtime, HTTP adapters, and app builders.
//!
//! This crate owns notification request orchestration, idempotency,
//! recipient-scoped listing, and the notification HTTP surface for the
//! SDKWork IM platform. Business logic lives in [`NotificationRuntime`];
//! HTTP handlers, app builders, OpenAPI helpers, and DTOs are split into
//! focused modules re-exported from here.

pub mod app;
pub mod bootstrap;
pub mod dto;
pub mod error;
pub mod state;

mod handlers;
mod helpers;
mod openapi;
#[cfg(test)]
mod tests;

pub use app::{
    apply_public_http_guardrails, build_app, build_default_app, build_domain_api_router,
    build_public_app, build_public_app_from_api_router,
};
pub use bootstrap::{
    build_runtime_from_env, default_app_state, default_notification_runtime,
    ensure_durable_notification_runtime_from_env,
};
pub use dto::{
    NotificationRecipient, NotificationRequestDeliveryStatus, NotificationRequestResponse,
    NotificationRequestResult, RequestAutomationResultNotification,
    RequestMessagePostedNotifications, RequestNotification, RequestNotificationFanout,
};
pub use error::NotificationError;
pub use im_domain_core::notification::{NotificationStatus, NotificationTask};
pub use state::{AppState, NotificationRuntime};
