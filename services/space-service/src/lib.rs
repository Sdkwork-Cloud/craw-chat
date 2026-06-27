//! Space Service — space, group, channel, member, invitation, and ban handlers.

pub mod ban;
mod bootstrap;
pub mod channel;
pub mod channel_access_rule;
pub mod group;
pub mod group_member;
pub mod http;
pub mod id;
pub mod invitation;
mod openapi;
mod service_http;
pub mod space;
pub mod space_member;

pub use bootstrap::{
    app_state_from_postgres_pool, try_app_state_from_database_url_env,
    try_build_embedded_app_from_database_url_env, try_build_public_app_from_database_url_env,
};
pub use http::{AppState, build_app, build_embedded_app, build_public_app};
