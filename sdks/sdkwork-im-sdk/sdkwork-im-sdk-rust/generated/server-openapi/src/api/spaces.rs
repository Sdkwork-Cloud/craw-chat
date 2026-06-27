use std::sync::Arc;

use crate::api::paths::im_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{SpaceBanCreateRequest, SpaceBanListResponse, SpaceBanView, SpaceChannelAccessRuleCreateRequest, SpaceChannelAccessRuleListResponse, SpaceChannelAccessRuleView, SpaceChannelCreateRequest, SpaceChannelListResponse, SpaceChannelUpdateRequest, SpaceChannelView, SpaceCreateRequest, SpaceGroupCreateRequest, SpaceGroupListResponse, SpaceGroupMemberCreateRequest, SpaceGroupMemberListResponse, SpaceGroupMemberUpdateRequest, SpaceGroupMemberView, SpaceGroupUpdateRequest, SpaceGroupView, SpaceInviteCreateRequest, SpaceInviteListResponse, SpaceInviteView, SpaceListResponse, SpaceMemberCreateRequest, SpaceMemberListResponse, SpaceMemberUpdateRequest, SpaceMemberView, SpaceUpdateRequest, SpaceView};

#[derive(Clone)]
pub struct SpacesApi {
    client: Arc<SdkworkHttpClient>,
}

impl SpacesApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Create a space
    pub async fn create(&self, body: &SpaceCreateRequest) -> Result<SpaceView, SdkworkError> {
        let path = im_path(&"/spaces".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// List spaces
    pub async fn list(&self) -> Result<SpaceListResponse, SdkworkError> {
        let path = im_path(&"/spaces".to_string());
        self.client.get(&path, None, None).await
    }

    /// Get a space
    pub async fn get(&self, space_id: &str) -> Result<SpaceView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update a space
    pub async fn update(&self, space_id: &str, body: &SpaceUpdateRequest) -> Result<SpaceView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete a space
    pub async fn delete(&self, space_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces members
    pub async fn members_list(&self, space_id: &str) -> Result<SpaceMemberListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/members", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces members
    pub async fn members_create(&self, space_id: &str, body: &SpaceMemberCreateRequest) -> Result<SpaceMemberView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/members", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces members
    pub async fn members_get(&self, space_id: &str, user_id: &str) -> Result<SpaceMemberView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update spaces members
    pub async fn members_update(&self, space_id: &str, user_id: &str, body: &SpaceMemberUpdateRequest) -> Result<SpaceMemberView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete spaces members
    pub async fn members_delete(&self, space_id: &str, user_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces groups
    pub async fn groups_list(&self, space_id: &str) -> Result<SpaceGroupListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces groups
    pub async fn groups_create(&self, space_id: &str, body: &SpaceGroupCreateRequest) -> Result<SpaceGroupView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces groups
    pub async fn groups_get(&self, space_id: &str, group_id: &str) -> Result<SpaceGroupView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update spaces groups
    pub async fn groups_update(&self, space_id: &str, group_id: &str, body: &SpaceGroupUpdateRequest) -> Result<SpaceGroupView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete spaces groups
    pub async fn groups_delete(&self, space_id: &str, group_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces groups members
    pub async fn groups_members_list(&self, space_id: &str, group_id: &str) -> Result<SpaceGroupMemberListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}/members", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces groups members
    pub async fn groups_members_create(&self, space_id: &str, group_id: &str, body: &SpaceGroupMemberCreateRequest) -> Result<SpaceGroupMemberView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}/members", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces groups members
    pub async fn groups_members_get(&self, space_id: &str, group_id: &str, user_id: &str) -> Result<SpaceGroupMemberView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update spaces groups members
    pub async fn groups_members_update(&self, space_id: &str, group_id: &str, user_id: &str, body: &SpaceGroupMemberUpdateRequest) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete spaces groups members
    pub async fn groups_members_delete(&self, space_id: &str, group_id: &str, user_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/groups/{}/members/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces channels
    pub async fn channels_list(&self, space_id: &str) -> Result<SpaceChannelListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces channels
    pub async fn channels_create(&self, space_id: &str, body: &SpaceChannelCreateRequest) -> Result<SpaceChannelView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces channels
    pub async fn channels_get(&self, space_id: &str, channel_id: &str) -> Result<SpaceChannelView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Update spaces channels
    pub async fn channels_update(&self, space_id: &str, channel_id: &str, body: &SpaceChannelUpdateRequest) -> Result<SpaceChannelView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete spaces channels
    pub async fn channels_delete(&self, space_id: &str, channel_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces channels access Rules
    pub async fn channels_access_rules_list(&self, space_id: &str, channel_id: &str) -> Result<SpaceChannelAccessRuleListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}/access_rules", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces channels access Rules
    pub async fn channels_access_rules_create(&self, space_id: &str, channel_id: &str, body: &SpaceChannelAccessRuleCreateRequest) -> Result<SpaceChannelAccessRuleView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}/access_rules", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Delete spaces channels access Rules
    pub async fn channels_access_rules_delete(&self, space_id: &str, channel_id: &str, rule_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/channels/{}/access_rules/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false)), serialize_path_parameter(rule_id, PathParameterSpec::new("ruleId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// List spaces invites
    pub async fn invites_list(&self, space_id: &str) -> Result<SpaceInviteListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/invites", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces invites
    pub async fn invites_create(&self, space_id: &str, body: &SpaceInviteCreateRequest) -> Result<SpaceInviteView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/invites", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces invites
    pub async fn invites_get(&self, space_id: &str, invite_code: &str) -> Result<SpaceInviteView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/invites/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(invite_code, PathParameterSpec::new("inviteCode", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Revoke spaces invites
    pub async fn invites_revoke(&self, space_id: &str, invite_code: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/invites/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(invite_code, PathParameterSpec::new("inviteCode", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// Accept spaces invites
    pub async fn invites_accept(&self, space_id: &str, invite_code: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/invites/{}/accept", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(invite_code, PathParameterSpec::new("inviteCode", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// List spaces bans
    pub async fn bans_list(&self, space_id: &str) -> Result<SpaceBanListResponse, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/bans", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Create spaces bans
    pub async fn bans_create(&self, space_id: &str, body: &SpaceBanCreateRequest) -> Result<SpaceBanView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/bans", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get spaces bans
    pub async fn bans_get(&self, space_id: &str, user_id: &str) -> Result<SpaceBanView, SdkworkError> {
        let path = im_path(&format!("/spaces/{}/bans/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Delete spaces bans
    pub async fn bans_delete(&self, space_id: &str, user_id: &str) -> Result<(), SdkworkError> {
        let path = im_path(&format!("/spaces/{}/bans/{}", serialize_path_parameter(space_id, PathParameterSpec::new("spaceId", "simple", false)), serialize_path_parameter(user_id, PathParameterSpec::new("userId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

}

struct PathParameterSpec<'a> {
    name: &'a str,
    style: &'a str,
    explode: bool,
}

impl<'a> PathParameterSpec<'a> {
    fn new(name: &'a str, style: &'a str, explode: bool) -> Self {
        Self { name, style, explode }
    }
}

fn serialize_path_parameter<T: serde::Serialize>(value: T, spec: PathParameterSpec<'_>) -> String {
    let value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    if value.is_null() {
        return String::new();
    }
    let style = if spec.style.is_empty() { "simple" } else { spec.style };
    match value {
        serde_json::Value::Array(values) => serialize_path_array(spec.name, &values, style, spec.explode),
        serde_json::Value::Object(values) => serialize_path_object(spec.name, &values, style, spec.explode),
        value => format!("{}{}", path_primitive_prefix(spec.name, style), percent_encode(&primitive_to_string(&value))),
    }
}

fn serialize_path_array(name: &str, values: &[serde_json::Value], style: &str, explode: bool) -> String {
    let serialized = values
        .iter()
        .filter(|value| !value.is_null())
        .map(|value| percent_encode(&primitive_to_string(value)))
        .collect::<Vec<_>>();
    if serialized.is_empty() {
        return path_prefix(name, style);
    }
    if style == "matrix" {
        if explode {
            return serialized.iter().map(|item| format!(";{}={}", name, item)).collect::<Vec<_>>().join("");
        }
        return format!(";{}={}", name, serialized.join(","));
    }
    let separator = if explode { "." } else { "," };
    format!("{}{}", path_prefix(name, style), serialized.join(separator))
}

fn serialize_path_object(
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
) -> String {
    let mut entries = Vec::new();
    let mut exploded = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        let escaped_key = percent_encode(key);
        let escaped_value = percent_encode(&primitive_to_string(value));
        if explode {
            if style == "matrix" {
                exploded.push(format!(";{}={}", escaped_key, escaped_value));
            } else {
                exploded.push(format!("{}={}", escaped_key, escaped_value));
            }
        } else {
            entries.push(escaped_key);
            entries.push(escaped_value);
        }
    }
    if style == "matrix" {
        if explode {
            return exploded.join("");
        }
        return format!(";{}={}", name, entries.join(","));
    }
    if explode {
        let separator = if style == "label" { "." } else { "," };
        return format!("{}{}", path_prefix(name, style), exploded.join(separator));
    }
    format!("{}{}", path_prefix(name, style), entries.join(","))
}

fn path_prefix(name: &str, style: &str) -> String {
    match style {
        "label" => ".".to_string(),
        "matrix" => format!(";{}", name),
        _ => String::new(),
    }
}

fn path_primitive_prefix(name: &str, style: &str) -> String {
    if style == "matrix" {
        format!(";{}=", name)
    } else {
        path_prefix(name, style)
    }
}



fn primitive_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        other => other.to_string(),
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{:02X}", byte).chars().collect(),
        })
        .collect()
}
