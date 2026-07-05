//! Space Service — space, group, channel, member, invitation, and ban handlers.

pub mod api_payload;
pub mod ban;
mod bootstrap;
pub mod channel;
pub mod channel_access_rule;
pub mod channel_conversation_binder;
pub mod group;
pub mod group_conversation_binder;
pub mod group_member;
pub mod http;
pub mod id;
mod journal_bootstrap;
pub mod invitation;
mod list_query;
pub(crate) mod openapi;
mod runtime_env;
mod space_materializer_metrics;
pub mod space;
pub mod space_access;
pub mod space_member;
mod write_authority;

pub use bootstrap::{
    app_state_from_postgres_pool, try_app_state_from_database_url_env,
    try_build_embedded_app_from_database_url_env, try_build_public_app_from_database_url_env,
};
pub use channel_conversation_binder::{
    CreateSpaceChannelConversationInput, SpaceChannelConversationBinder,
};
pub use group_conversation_binder::{
    CreateSpaceGroupConversationInput, SpaceGroupConversationBinder, SyncSpaceGroupMemberInput,
    TransferSpaceGroupOwnerInput,
};
pub use http::{AppState, build_app, build_embedded_app, build_public_app};
pub use space_materializer_metrics::{
    postgres_journal_append_failure_after_materialize_count,
    postgres_materialization_failure_count,
    record_postgres_journal_append_failures_after_materialize,
    record_postgres_materialization_failures,
    render_prometheus as render_space_materializer_prometheus,
};
