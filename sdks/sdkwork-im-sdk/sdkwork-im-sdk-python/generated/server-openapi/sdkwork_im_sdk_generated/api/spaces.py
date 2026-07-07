from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import SdkWorkCommandResponse, SpaceBanCreateRequest, SpaceChannelAccessRuleCreateRequest, SpaceChannelCreateRequest, SpaceChannelUpdateRequest, SpaceCreateRequest, SpaceGroupCreateRequest, SpaceGroupMemberCreateRequest, SpaceGroupMemberUpdateRequest, SpaceGroupUpdateRequest, SpaceInviteCreateRequest, SpaceMemberCreateRequest, SpaceMemberUpdateRequest, SpacesBansCreateResponse201, SpacesBansListResponse, SpacesBansRetrieveResponse, SpacesChannelsAccessRulesCreateResponse201, SpacesChannelsAccessRulesListResponse, SpacesChannelsCreateResponse201, SpacesChannelsListResponse, SpacesChannelsRetrieveResponse, SpacesChannelsUpdateResponse, SpacesCreateResponse201, SpacesGroupsCreateResponse201, SpacesGroupsListResponse, SpacesGroupsMembersCreateResponse201, SpacesGroupsMembersListResponse, SpacesGroupsMembersRetrieveResponse, SpacesGroupsMembersUpdateResponse, SpacesGroupsRetrieveResponse, SpacesGroupsUpdateResponse, SpacesInvitesCreateResponse201, SpacesInvitesListResponse, SpacesInvitesRetrieveResponse, SpacesListResponse, SpacesMembersCreateResponse201, SpacesMembersListResponse, SpacesMembersRetrieveResponse, SpacesMembersUpdateResponse, SpacesRetrieveResponse, SpacesUpdateResponse, SpaceUpdateRequest

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


def build_query_string(parameters: List[Dict[str, Any]]) -> str:
    pairs: List[str] = []
    for parameter in parameters:
        append_serialized_parameter(pairs, parameter)
    return '&'.join(pairs)


def append_serialized_parameter(pairs: List[str], parameter: Dict[str, Any]) -> None:
    value = parameter.get('value')
    if value is None:
        return

    name = str(parameter.get('name') or '')
    allow_reserved = bool(parameter.get('allow_reserved'))
    content_type = parameter.get('content_type')
    if content_type:
        import json

        pairs.append(f"{encode_query_component(name)}={encode_query_value(json.dumps(value, separators=(',', ':')), allow_reserved)}")
        return

    style = str(parameter.get('style') or 'form')
    explode = bool(parameter.get('explode'))
    if style == 'deepObject':
        append_deep_object_parameter(pairs, name, value, allow_reserved)
        return
    if isinstance(value, (list, tuple)):
        append_array_parameter(pairs, name, value, style, explode, allow_reserved)
        return
    if isinstance(value, dict):
        append_object_parameter(pairs, name, value, style, explode, allow_reserved)
        return

    pairs.append(f"{encode_query_component(name)}={encode_query_value(serialize_primitive(value), allow_reserved)}")


def append_array_parameter(
    pairs: List[str],
    name: str,
    value: Any,
    style: str,
    explode: bool,
    allow_reserved: bool,
) -> None:
    values = [serialize_primitive(item) for item in value if item is not None]
    if not values:
        return

    if style == 'form' and explode:
        for item in values:
            pairs.append(f"{encode_query_component(name)}={encode_query_value(item, allow_reserved)}")
        return

    pairs.append(f"{encode_query_component(name)}={encode_query_value(','.join(values), allow_reserved)}")


def append_object_parameter(
    pairs: List[str],
    name: str,
    value: Dict[str, Any],
    style: str,
    explode: bool,
    allow_reserved: bool,
) -> None:
    entries = [(key, entry_value) for key, entry_value in value.items() if entry_value is not None]
    if not entries:
        return

    if style == 'form' and explode:
        for key, entry_value in entries:
            pairs.append(f"{encode_query_component(str(key))}={encode_query_value(serialize_primitive(entry_value), allow_reserved)}")
        return

    serialized = ','.join(
        item
        for key, entry_value in entries
        for item in (str(key), serialize_primitive(entry_value))
    )
    pairs.append(f"{encode_query_component(name)}={encode_query_value(serialized, allow_reserved)}")


def append_deep_object_parameter(pairs: List[str], name: str, value: Any, allow_reserved: bool) -> None:
    if not isinstance(value, dict):
        pairs.append(f"{encode_query_component(name)}={encode_query_value(serialize_primitive(value), allow_reserved)}")
        return

    for key, entry_value in value.items():
        if entry_value is None:
            continue
        pairs.append(f"{encode_query_component(f'{name}[{key}]')}={encode_query_value(serialize_primitive(entry_value), allow_reserved)}")


def serialize_primitive(value: Any) -> str:
    if isinstance(value, dict):
        import json

        return json.dumps(value, separators=(',', ':'))
    return str(value)


def encode_query_component(value: str) -> str:
    from urllib.parse import quote

    return quote(value, safe='')


def encode_query_value(value: str, allow_reserved: bool) -> str:
    from urllib.parse import quote

    return quote(value, safe=':/?#[]@!$&\'()*+,;=' if allow_reserved else '')



class SpacesApi:
    """spaces spaces API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.members = SpacesMembersApi(client)
        self.groups = SpacesGroupsApi(client)
        self.channels = SpacesChannelsApi(client)
        self.invites = SpacesInvitesApi(client)
        self.bans = SpacesBansApi(client)


    def create(self, body: SpaceCreateRequest) -> SpacesCreateResponse201:
        """Create a space"""
        return self._client.post(f"/im/v3/api/spaces", json=body)

    def list(self, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesListResponse:
        """List spaces"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces", query))

    def retrieve(self, space_id: str) -> SpacesRetrieveResponse:
        """Retrieve a space"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, body: SpaceUpdateRequest) -> SpacesUpdateResponse:
        """Update a space"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str) -> None:
        """Delete a space"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}")

class SpacesMembersApi:
    """spaces spaces.members API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesMembersListResponse:
        """List spaces members"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members", query))

    def create(self, space_id: str, body: SpaceMemberCreateRequest) -> SpacesMembersCreateResponse201:
        """Create spaces members"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members", json=body)

    def retrieve(self, space_id: str, user_id: str) -> SpacesMembersRetrieveResponse:
        """retrieve spaces members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, user_id: str, body: SpaceMemberUpdateRequest) -> SpacesMembersUpdateResponse:
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


    def list(self, space_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesGroupsListResponse:
        """List spaces groups"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups", query))

    def create(self, space_id: str, body: SpaceGroupCreateRequest) -> SpacesGroupsCreateResponse201:
        """Create spaces groups"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups", json=body)

    def retrieve(self, space_id: str, group_id: str) -> SpacesGroupsRetrieveResponse:
        """retrieve spaces groups"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, group_id: str, body: SpaceGroupUpdateRequest) -> SpacesGroupsUpdateResponse:
        """Update spaces groups"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, group_id: str) -> None:
        """Delete spaces groups"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}")

class SpacesGroupsMembersApi:
    """spaces spaces.groups.members API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, group_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesGroupsMembersListResponse:
        """List spaces groups members"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members", query))

    def create(self, space_id: str, group_id: str, body: SpaceGroupMemberCreateRequest) -> SpacesGroupsMembersCreateResponse201:
        """Create spaces groups members"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members", json=body)

    def retrieve(self, space_id: str, group_id: str, user_id: str) -> SpacesGroupsMembersRetrieveResponse:
        """retrieve spaces groups members"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/groups/{serialize_path_parameter(group_id, {'name': 'groupId', 'style': 'simple', 'explode': False})}/members/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, group_id: str, user_id: str, body: SpaceGroupMemberUpdateRequest) -> SpacesGroupsMembersUpdateResponse:
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


    def list(self, space_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesChannelsListResponse:
        """List spaces channels"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels", query))

    def create(self, space_id: str, body: SpaceChannelCreateRequest) -> SpacesChannelsCreateResponse201:
        """Create spaces channels"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels", json=body)

    def retrieve(self, space_id: str, channel_id: str) -> SpacesChannelsRetrieveResponse:
        """retrieve spaces channels"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

    def update(self, space_id: str, channel_id: str, body: SpaceChannelUpdateRequest) -> SpacesChannelsUpdateResponse:
        """Update spaces channels"""
        return self._client.patch(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, space_id: str, channel_id: str) -> None:
        """Delete spaces channels"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}")

class SpacesChannelsAccessRulesApi:
    """spaces spaces.channels.access_rules API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, channel_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesChannelsAccessRulesListResponse:
        """List spaces channels access Rules"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules", query))

    def create(self, space_id: str, channel_id: str, body: SpaceChannelAccessRuleCreateRequest) -> SpacesChannelsAccessRulesCreateResponse201:
        """Create spaces channels access Rules"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules", json=body)

    def delete(self, space_id: str, channel_id: str, rule_id: str) -> None:
        """Delete spaces channels access Rules"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/channels/{serialize_path_parameter(channel_id, {'name': 'channelId', 'style': 'simple', 'explode': False})}/access_rules/{serialize_path_parameter(rule_id, {'name': 'ruleId', 'style': 'simple', 'explode': False})}")

class SpacesInvitesApi:
    """spaces spaces.invites API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, status: Optional[str] = None, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesInvitesListResponse:
        """List spaces invites"""
        query = build_query_string([
            {'name': 'status', 'value': status, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites", query))

    def create(self, space_id: str, body: SpaceInviteCreateRequest) -> SpacesInvitesCreateResponse201:
        """Create spaces invites"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites", json=body)

    def retrieve(self, space_id: str, invite_code: str) -> SpacesInvitesRetrieveResponse:
        """retrieve spaces invites"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}")

    def delete(self, space_id: str, invite_code: str) -> None:
        """Delete spaces invites"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}")

    def create_accept(self, space_id: str, invite_code: str) -> SdkWorkCommandResponse:
        """Accept spaces invites"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/invites/{serialize_path_parameter(invite_code, {'name': 'inviteCode', 'style': 'simple', 'explode': False})}/accept")

class SpacesBansApi:
    """spaces spaces.bans API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, space_id: str, page_size: Optional[int] = None, cursor: Optional[str] = None) -> SpacesBansListResponse:
        """List spaces bans"""
        query = build_query_string([
            {'name': 'page_size', 'value': page_size, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans", query))

    def create(self, space_id: str, body: SpaceBanCreateRequest) -> SpacesBansCreateResponse201:
        """Create spaces bans"""
        return self._client.post(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans", json=body)

    def retrieve(self, space_id: str, user_id: str) -> SpacesBansRetrieveResponse:
        """retrieve spaces bans"""
        return self._client.get(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")

    def delete(self, space_id: str, user_id: str) -> None:
        """Delete spaces bans"""
        return self._client.delete(f"/im/v3/api/spaces/{serialize_path_parameter(space_id, {'name': 'spaceId', 'style': 'simple', 'explode': False})}/bans/{serialize_path_parameter(user_id, {'name': 'userId', 'style': 'simple', 'explode': False})}")
