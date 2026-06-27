//! Interaction Service — deprecated public HTTP surface.
//!
//! Reactions, pins, threads, and conversation settings are owned by the `chat` OpenAPI tag under
//! `/im/v3/api/chat/*`. Do not mount `/im/v3/api/interactions/*` in gateway or SDK contracts.

mod http;

pub use http::{build_app, build_public_app};
