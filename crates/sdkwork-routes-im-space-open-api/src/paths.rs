pub const PREFIX: &str = "/im/v3/api/spaces";

pub const SPACES: &str = "/im/v3/api/spaces";
pub const SPACE: &str = "/im/v3/api/spaces/{spaceId}";
pub const SPACE_MEMBERS: &str = "/im/v3/api/spaces/{spaceId}/members";
pub const SPACE_MEMBER: &str = "/im/v3/api/spaces/{spaceId}/members/{userId}";
pub const SPACE_GROUPS: &str = "/im/v3/api/spaces/{spaceId}/groups";
pub const SPACE_GROUP: &str = "/im/v3/api/spaces/{spaceId}/groups/{groupId}";
pub const SPACE_GROUP_MEMBERS: &str = "/im/v3/api/spaces/{spaceId}/groups/{groupId}/members";
pub const SPACE_GROUP_MEMBER: &str =
    "/im/v3/api/spaces/{spaceId}/groups/{groupId}/members/{userId}";
pub const SPACE_CHANNELS: &str = "/im/v3/api/spaces/{spaceId}/channels";
pub const SPACE_CHANNEL: &str = "/im/v3/api/spaces/{spaceId}/channels/{channelId}";
pub const SPACE_CHANNEL_ACCESS_RULES: &str =
    "/im/v3/api/spaces/{spaceId}/channels/{channelId}/access_rules";
pub const SPACE_CHANNEL_ACCESS_RULE: &str =
    "/im/v3/api/spaces/{spaceId}/channels/{channelId}/access_rules/{ruleId}";
pub const SPACE_INVITES: &str = "/im/v3/api/spaces/{spaceId}/invites";
pub const SPACE_INVITE: &str = "/im/v3/api/spaces/{spaceId}/invites/{inviteCode}";
pub const SPACE_INVITE_ACCEPT: &str = "/im/v3/api/spaces/{spaceId}/invites/{inviteCode}/accept";
pub const SPACE_BANS: &str = "/im/v3/api/spaces/{spaceId}/bans";
pub const SPACE_BAN: &str = "/im/v3/api/spaces/{spaceId}/bans/{userId}";
