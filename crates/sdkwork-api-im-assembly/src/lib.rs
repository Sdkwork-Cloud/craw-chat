//! API assembly for sdkwork-im.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod conversation_outbox_relay;
mod generated;
mod ops_realtime_wiring;
mod outbox_relay_common;
mod rtc_outbox_relay;
mod social_outbox_relay;
mod social_realtime_wiring;
mod space_conversation_wiring;

pub use bootstrap::{
    ApiAssembly, assemble_api_router, assemble_api_router_with_realtime_bootstrap,
};
pub use conversation_outbox_relay::{
    ConversationOutboxRelayHandle, spawn_conversation_outbox_relay_from_env,
};
pub use rtc_outbox_relay::{RtcOutboxRelayHandle, spawn_rtc_outbox_relay_from_env};
pub use social_outbox_relay::{SocialOutboxRelayHandle, spawn_social_outbox_relay_from_env};
pub use social_realtime_wiring::wire_social_runtime_embedded_plane;

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
