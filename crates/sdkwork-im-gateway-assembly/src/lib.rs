//! Gateway assembly for sdkwork-im.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

mod bootstrap;
mod conversation_outbox_relay;
mod generated;
mod outbox_relay_common;
mod rtc_outbox_relay;
mod social_outbox_relay;
mod social_realtime_wiring;
mod space_conversation_wiring;

pub use bootstrap::{ApplicationAssembly, assemble_application_router};
pub use conversation_outbox_relay::{
    ConversationOutboxRelayHandle, spawn_conversation_outbox_relay_from_env,
};
pub use rtc_outbox_relay::{RtcOutboxRelayHandle, spawn_rtc_outbox_relay_from_env};
pub use social_outbox_relay::{SocialOutboxRelayHandle, spawn_social_outbox_relay_from_env};
pub use social_realtime_wiring::wire_social_runtime_embedded_plane;
pub use space_conversation_wiring::wire_space_conversation_binders;

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
