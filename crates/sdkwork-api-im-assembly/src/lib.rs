//! API assembly for sdkwork-im.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod conversation_outbox_relay;
mod generated;
mod outbox_relay_common;
mod rtc_outbox_relay;
mod social_outbox_relay;
mod social_realtime_wiring;
mod space_conversation_wiring;

pub use bootstrap::{assemble_api_router, ApiAssembly};
pub use conversation_outbox_relay::{
    spawn_conversation_outbox_relay_from_env, ConversationOutboxRelayHandle,
};
pub use rtc_outbox_relay::{spawn_rtc_outbox_relay_from_env, RtcOutboxRelayHandle};
pub use social_outbox_relay::{spawn_social_outbox_relay_from_env, SocialOutboxRelayHandle};
pub use social_realtime_wiring::wire_social_runtime_embedded_plane;

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
