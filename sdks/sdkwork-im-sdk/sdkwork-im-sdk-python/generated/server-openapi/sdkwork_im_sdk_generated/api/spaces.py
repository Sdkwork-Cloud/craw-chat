from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import SpaceBanCreateRequest, SpaceBanListResponse, SpaceBanView, SpaceChannelAccessRuleCreateRequest, SpaceChannelAccessRuleListResponse, SpaceChannelAccessRuleView, SpaceChannelCreateRequest, SpaceChannelListResponse, SpaceChannelUpdateRequest, SpaceChannelView, SpaceCreateRequest, SpaceGroupCreateRequest, SpaceGroupListResponse, SpaceGroupMemberCreateRequest, SpaceGroupMemberListResponse, SpaceGroupMemberUpdateRequest, SpaceGroupMemberView, SpaceGroupUpdateRequest, SpaceGroupView, SpaceInviteCreateRequest, SpaceInviteListResponse, SpaceInviteView, SpaceListResponse, SpaceMemberCreateRequest, SpaceMemberListResponse, SpaceMemberUpdateRequest, SpaceMemberView, SpaceUpdateRequest, SpaceView

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"

def serialize_path_parameter(value: Any, spec: Dict[str, Any]) -> str:
    if value is None:
        return ''

    style = str(spec.get('style') or 'simple')
    name = str(spec.get('name') or '')
    explode = bool(spec.get('explode'))
    if isinstance(value, (list, tuple)):
        return serialize_path_array(name, value, style, explode)
    if isinstance(value, dict):
        return serialize_path_object(name, value, style, explode)
    return path_prefix(name, style) + encode_path_value(serialize_path_primitive(value))


def serialize_path_array(name: str, values: Any, style: str, explode: bool) -> str:
    serialized = [encode_path_value(serialize_path_primitive(item)) for item in values if item is not None]
    if not serialized:
        return path_prefix(name, style)
    if style == 'matrix':
        return ''.join(f";{name}={item}" for item in serialized) if explode else f";{name}={','.join(serialized)}"
    return path_prefix(name, style) + ('.' if explode else ',').join(serialized)


def serialize_path_object(name: str, value: Dict[str, Any], style: str, explode: bool) -> str:
    entries = [(key, entry_value) for key, entry_value in value.items() if entry_value is not None]
    if not entries:
        return path_prefix(name, style)
    if style == 'matrix':
        if explode:
            return ''.join(f";{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
        return f";{name}={serialized}"
    if explode:
        separator = '.' if style == 'label' else ','
        serialized = separator.join(f"{encode_path_value(str(key))}={encode_path_value(serialize_path_primitive(entry_value))}" for key, entry_value in entries)
    else:
        serialized = ','.join(item for key, entry_value in entries for item in (encode_path_value(str(key)), encode_path_value(serialize_path_primitive(entry_value))))
    return path_prefix(name, style) + serialized


def path_prefix(name: str, style: str) -> str:
    if style == 'label':
        return '.'
    if style == 'matrix':
        return f";{name}"
    return ''


def encode_path_value(value: str) -> str:
    from urllib.parse import quote

    return quote(value, safe='')


def serialize_path_primitive(value: Any) -> str:
    if isinstance(value, dict):
        import json

        return json.dumps(value, separators=(',', ':'))
    return str(value)





class SpacesApi:
    """spaces spaces API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.members = SpacesMembersApi(client)
        self.groups = SpacesGroupsApi(client)
        self.channels = SpacesChannelsApi(client)
        self.invites = SpacesInvitesApi(client)
        self.bans = SpacesBansApi(client)


    def create(self, body: SpaceCreateRequest) -> SpaceView:
        """Create a space"""
        return self._client.post(f"/im/v3/api/spaces", json=body)

    def list(self) -> SpaceListResponse:
        """List spaces"""
        return self._client.get(f"/im/v3/api/spaces")

    def retrieve(self, space_id: str) -> SpaceView:
        """Get a space"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, body: SpaceUpdateRequest) -> SpaceView:
        """Update a space"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str) -> None:
        """Delete a space"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}")

class SpacesMembersApi:
    """spaces spaces.members API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str) -> SpaceMemberListResponse:
        """List spaces members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members")

    def create(self, space_id: str, body: SpaceMemberCreateRequest) -> SpaceMemberView:
        """Create spaces members"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members", json=body)

    def retrieve(self, space_id: str, user_id: str) -> SpaceMemberView:
        """Get spaces members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, user_id: str, body: SpaceMemberUpdateRequest) -> SpaceMemberView:
        """Update spaces members"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, user_id: str) -> None:
        """Delete spaces members"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

class SpacesGroupsApi:
    """spaces spaces.groups API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.members = SpacesGroupsMembersApi(client)


    def list(self, space_id: str) -> SpaceGroupListResponse:
        """List spaces groups"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups")

    def create(self, space_id: str, body: SpaceGroupCreateRequest) -> SpaceGroupView:
        """Create spaces groups"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups", json=body)

    def retrieve(self, space_id: str, group_id: str) -> SpaceGroupView:
        """Get spaces groups"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, group_id: str, body: SpaceGroupUpdateRequest) -> SpaceGroupView:
        """Update spaces groups"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, group_id: str) -> None:
        """Delete spaces groups"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

class SpacesGroupsMembersApi:
    """spaces spaces.groups.members API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, group_id: str) -> SpaceGroupMemberListResponse:
        """List spaces groups members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members")

    def create(self, space_id: str, group_id: str, body: SpaceGroupMemberCreateRequest) -> SpaceGroupMemberView:
        """Create spaces groups members"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members", json=body)

    def retrieve(self, space_id: str, group_id: str, user_id: str) -> SpaceGroupMemberView:
        """Get spaces groups members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, group_id: str, user_id: str, body: SpaceGroupMemberUpdateRequest) -> None:
        """Update spaces groups members"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, group_id: str, user_id: str) -> None:
        """Delete spaces groups members"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

class SpacesChannelsApi:
    """spaces spaces.channels API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.access_rules = SpacesChannelsAccessRulesApi(client)


    def list(self, space_id: str) -> SpaceChannelListResponse:
        """List spaces channels"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels")

    def create(self, space_id: str, body: SpaceChannelCreateRequest) -> SpaceChannelView:
        """Create spaces channels"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels", json=body)

    def retrieve(self, space_id: str, channel_id: str) -> SpaceChannelView:
        """Get spaces channels"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, channel_id: str, body: SpaceChannelUpdateRequest) -> SpaceChannelView:
        """Update spaces channels"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, channel_id: str) -> None:
        """Delete spaces channels"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

class SpacesChannelsAccessRulesApi:
    """spaces spaces.channels.access_rules API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, channel_id: str) -> SpaceChannelAccessRuleListResponse:
        """List spaces channels access Rules"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules")

    def create(self, space_id: str, channel_id: str, body: SpaceChannelAccessRuleCreateRequest) -> SpaceChannelAccessRuleView:
        """Create spaces channels access Rules"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules", json=body)

    def delete(self, space_id: str, channel_id: str, rule_id: str) -> None:
        """Delete spaces channels access Rules"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules/{serialize_path_parameter(rule_id, {'name': 'ruleId', 'style': 'simple', 'explode': False})}")

class SpacesInvitesApi:
    """spaces spaces.invites API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str) -> SpaceInviteListResponse:
        """List spaces invites"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites")

    def create(self, space_id: str, body: SpaceInviteCreateRequest) -> SpaceInviteView:
        """Create spaces invites"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites", json=body)

    def retrieve(self, space_id: str, invite_code: str) -> SpaceInviteView:
        """Get spaces invites"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}")

    def delete(self, space_id: str, invite_code: str) -> None:
        """Revoke spaces invites"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}")

    def create_accept(self, space_id: str, invite_code: str) -> None:
        """Accept spaces invites"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}/accept")

class SpacesBansApi:
    """spaces spaces.bans API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str) -> SpaceBanListResponse:
        """List spaces bans"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans")

    def create(self, space_id: str, body: SpaceBanCreateRequest) -> SpaceBanView:
        """Create spaces bans"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans", json=body)

    def retrieve(self, space_id: str, user_id: str) -> SpaceBanView:
        """Get spaces bans"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def delete(self, space_id: str, user_id: str) -> None:
        """Delete spaces bans"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")
