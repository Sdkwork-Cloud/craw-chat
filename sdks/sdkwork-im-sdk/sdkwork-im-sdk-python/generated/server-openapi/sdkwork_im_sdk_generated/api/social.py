from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import ContactPreferencesView, ContactRecommendationView, ContactTagsResponse, ContactTagView, CreateContactRecommendationRequest, CreateContactTagRequest, DeleteContactTagResponse, SocialFriendRequestAcceptanceResponse, SocialFriendRequestListResponse, SocialFriendRequestMutationResponse, SocialFriendshipMutationResponse, SocialUserSearchResponse, SubmitFriendRequestRequest, UpdateContactPreferencesRequest, UpdateContactTagRequest

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



class SocialApi:
    """social social API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.users = SocialUsersApi(client)
        self.friend_requests = SocialFriendRequestsApi(client)
        self.friendships = SocialFriendshipsApi(client)
        self.contacts = SocialContactsApi(client)


class SocialUsersApi:
    """social social.users API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, q: Optional[str] = None, limit: Optional[int] = None, cursor: Optional[str] = None) -> SocialUserSearchResponse:
        """Search social users"""
        query = build_query_string([
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/social/users", query))

class SocialFriendRequestsApi:
    """social social.friend_requests API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, direction: Optional[str] = None, status: Optional[str] = None, limit: Optional[int] = None, cursor: Optional[str] = None) -> SocialFriendRequestListResponse:
        """List friend requests"""
        query = build_query_string([
            {'name': 'direction', 'value': direction, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'status', 'value': status, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/social/friend_requests", query))

    def create(self, body: SubmitFriendRequestRequest) -> SocialFriendRequestMutationResponse:
        """Create a friend request"""
        return self._client.post(f"/im/v3/api/social/friend_requests", json=body)

    def create_accept(self, request_id: str) -> SocialFriendRequestAcceptanceResponse:
        """Accept a friend request"""
        return self._client.post(f"/im/v3/api/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/accept")

    def create_decline(self, request_id: str) -> SocialFriendRequestMutationResponse:
        """Decline a friend request"""
        return self._client.post(f"/im/v3/api/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/decline")

    def cancel(self, request_id: str) -> SocialFriendRequestMutationResponse:
        """Cancel a friend request"""
        return self._client.post(f"/im/v3/api/social/friend_requests/{serialize_path_parameter(request_id, {'name': 'requestId', 'style': 'simple', 'explode': False})}/cancel")

class SocialFriendshipsApi:
    """social social.friendships API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create_remove(self, friendship_id: str) -> SocialFriendshipMutationResponse:
        """Remove a friendship"""
        return self._client.post(f"/im/v3/api/social/friendships/{serialize_path_parameter(friendship_id, {'name': 'friendshipId', 'style': 'simple', 'explode': False})}/remove")

class SocialContactsApi:
    """social social.contacts API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.tags = SocialContactsTagsApi(client)
        self.recommendations = SocialContactsRecommendationsApi(client)
        self.preferences = SocialContactsPreferencesApi(client)


class SocialContactsTagsApi:
    """social social.contacts.tags API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, limit: Optional[int] = None, cursor: Optional[str] = None) -> ContactTagsResponse:
        """List contact tags"""
        query = build_query_string([
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/social/contacts/tags", query))

    def create(self, body: CreateContactTagRequest) -> ContactTagView:
        """Create a contact tag"""
        return self._client.post(f"/im/v3/api/social/contacts/tags", json=body)

    def update(self, tag_id: str, body: UpdateContactTagRequest) -> ContactTagView:
        """Update a contact tag"""
        return self._client.patch(f"/im/v3/api/social/contacts/tags/{serialize_path_parameter(tag_id, {'name': 'tagId', 'style': 'simple', 'explode': False})}", json=body)

    def delete(self, tag_id: str) -> DeleteContactTagResponse:
        """Delete a contact tag"""
        return self._client.delete(f"/im/v3/api/social/contacts/tags/{serialize_path_parameter(tag_id, {'name': 'tagId', 'style': 'simple', 'explode': False})}")

class SocialContactsRecommendationsApi:
    """social social.contacts.recommendations API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, target_user_id: str, body: CreateContactRecommendationRequest) -> ContactRecommendationView:
        """Create a contact recommendation"""
        return self._client.post(f"/im/v3/api/social/contacts/{serialize_path_parameter(target_user_id, {'name': 'targetUserId', 'style': 'simple', 'explode': False})}/recommendations", json=body)

class SocialContactsPreferencesApi:
    """social social.contacts.preferences API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, target_user_id: str) -> ContactPreferencesView:
        """Retrieve contact preferences"""
        return self._client.get(f"/im/v3/api/social/contacts/{serialize_path_parameter(target_user_id, {'name': 'targetUserId', 'style': 'simple', 'explode': False})}/preferences")

    def update(self, target_user_id: str, body: UpdateContactPreferencesRequest) -> ContactPreferencesView:
        """Update contact preferences"""
        return self._client.patch(f"/im/v3/api/social/contacts/{serialize_path_parameter(target_user_id, {'name': 'targetUserId', 'style': 'simple', 'explode': False})}/preferences", json=body)
