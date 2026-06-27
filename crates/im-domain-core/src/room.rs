//! Room domain models for live, chat, and game room bindings over IM conversations.

use serde::{Deserialize, Serialize};

pub const ROOM_BUSINESS_TYPE_LIVE: &str = "live_room";
pub const ROOM_BUSINESS_TYPE_CHAT: &str = "chat_room";
pub const ROOM_BUSINESS_TYPE_GAME: &str = "game_room";

pub const SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX: &str = "urn:sdkwork:sdkwork-im:message:custom:game.";

pub const ROOM_MEMBER_ATTRIBUTE_ROLE: &str = "roomRole";
pub const ROOM_MEMBER_ROLE_OWNER: &str = "owner";
pub const ROOM_MEMBER_ROLE_PARTICIPANT: &str = "participant";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomKind {
    Live,
    Chat,
    Game,
}

impl RoomKind {
    pub fn as_wire_value(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Chat => "chat",
            Self::Game => "game",
        }
    }

    pub fn business_type(self) -> &'static str {
        match self {
            Self::Live => ROOM_BUSINESS_TYPE_LIVE,
            Self::Chat => ROOM_BUSINESS_TYPE_CHAT,
            Self::Game => ROOM_BUSINESS_TYPE_GAME,
        }
    }

    pub fn default_history_visibility(self) -> &'static str {
        match self {
            Self::Live | Self::Chat => "shared",
            Self::Game => "joined",
        }
    }

    pub fn default_max_members(self) -> usize {
        match self {
            Self::Live => 10_000,
            Self::Chat => 1_000,
            Self::Game => 8,
        }
    }

    pub fn parse_wire_value(value: &str) -> Option<Self> {
        match value {
            "live" => Some(Self::Live),
            "chat" => Some(Self::Chat),
            "game" => Some(Self::Game),
            _ => None,
        }
    }
}

pub fn is_room_business_type(business_type: &str) -> bool {
    matches!(
        business_type,
        ROOM_BUSINESS_TYPE_LIVE | ROOM_BUSINESS_TYPE_CHAT | ROOM_BUSINESS_TYPE_GAME
    )
}

pub fn room_kind_from_business_type(business_type: &str) -> Option<RoomKind> {
    match business_type {
        ROOM_BUSINESS_TYPE_LIVE => Some(RoomKind::Live),
        ROOM_BUSINESS_TYPE_CHAT => Some(RoomKind::Chat),
        ROOM_BUSINESS_TYPE_GAME => Some(RoomKind::Game),
        _ => None,
    }
}

pub fn game_move_schema_ref(game_key: &str) -> String {
    format!("{SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX}{game_key}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_kind_wire_values_round_trip() {
        for kind in [RoomKind::Live, RoomKind::Chat, RoomKind::Game] {
            assert_eq!(
                RoomKind::parse_wire_value(kind.as_wire_value()),
                Some(kind)
            );
            assert_eq!(room_kind_from_business_type(kind.business_type()), Some(kind));
        }
    }

    #[test]
    fn test_game_move_schema_ref_uses_custom_prefix() {
        assert_eq!(
            game_move_schema_ref("chess.move"),
            "urn:sdkwork:sdkwork-im:message:custom:game.chess.move"
        );
    }
}
