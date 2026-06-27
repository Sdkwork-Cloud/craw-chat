from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AckResponse, AddConversationMemberRequest, BindDirectChatRequest, ChangeConversationMemberRoleRequest, ContactsResponse, ConversationMember, ConversationPreferencesView, ConversationProfileView, ConversationSummaryView, CreateAgentDialogRequest, CreateConversationRequest, CreateConversationResult, CreateRoomRequest, DeleteMessageFavoriteResponse, EditMessageRequest, EnterRoomResponse, FavoriteMessageRequest, FavoriteMessagesResponse, InboxResponse, ListMembersResponse, MemberDirectoryResponse, MessageFavoriteView, MessageInteractionSummaryView, MessagePinMutationResult, MessageReactionMutationResult, MessageReactionRequest, MessageVisibilityMutationResult, PinnedMessagesResponse, PostedMessageResponse, PostMessageRequest, ReadCursorView, RemoveConversationMemberRequest, RoomView, TimelineResponse, TransferConversationOwnerRequest, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest

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



class ChatApi:
    """chat chat API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.contacts = ChatContactsApi(client)
        self.inbox = ChatInboxApi(client)
        self.conversations = ChatConversationsApi(client)
        self.messages = ChatMessagesApi(client)
        self.rooms = ChatRoomsApi(client)


class ChatContactsApi:
    """chat chat.contacts API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, limit: Optional[int] = None, cursor: Optional[str] = None) -> ContactsResponse:
        """List IM contacts"""
        query = build_query_string([
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/chat/contacts", query))

class ChatInboxApi:
    """chat chat.inbox API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, limit: Optional[int] = None, cursor: Optional[str] = None) -> InboxResponse:
        """Retrieve current inbox window"""
        query = build_query_string([
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/chat/inbox", query))

class ChatConversationsApi:
    """chat chat.conversations API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.agent_dialogs = ChatConversationsAgentDialogsApi(client)
        self.agent_handoffs = ChatConversationsAgentHandoffsApi(client)
        self.system_channels = ChatConversationsSystemChannelsApi(client)
        self.threads = ChatConversationsThreadsApi(client)
        self.direct_chats = ChatConversationsDirectChatsApi(client)
        self.members = ChatConversationsMembersApi(client)
        self.preferences = ChatConversationsPreferencesApi(client)
        self.profile = ChatConversationsProfileApi(client)
        self.read_cursor = ChatConversationsReadCursorApi(client)
        self.member_directory = ChatConversationsMemberDirectoryApi(client)
        self.messages = ChatConversationsMessagesApi(client)
        self.pins = ChatConversationsPinsApi(client)


    def create(self, body: CreateConversationRequest) -> CreateConversationResult:
        """Create a conversation"""
        return self._client.post(f"/im/v3/api/chat/conversations", json=body)

    def retrieve(self, conversation_id: str) -> ConversationSummaryView:
        """Retrieve conversation summary"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}")

class ChatConversationsAgentDialogsApi:
    """chat chat.conversations.agent_dialogs API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: CreateAgentDialogRequest) -> CreateConversationResult:
        """Create an agent dialog"""
        return self._client.post(f"/im/v3/api/chat/conversations/agent_dialogs", json=body)

class ChatConversationsAgentHandoffsApi:
    """chat chat.conversations.agent_handoffs API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: CreateAgentDialogRequest) -> AckResponse:
        """Create an agent handoff"""
        return self._client.post(f"/im/v3/api/chat/conversations/agent_handoffs", json=body)

    def retrieve(self, conversation_id: str) -> AckResponse:
        """Retrieve agent handoff state"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/agent_handoff")

    def accept(self, conversation_id: str) -> AckResponse:
        """Accept agent handoff"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/agent_handoff/accept")

    def resolve(self, conversation_id: str) -> AckResponse:
        """Resolve agent handoff"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/agent_handoff/resolve")

    def close(self, conversation_id: str) -> AckResponse:
        """Close agent handoff"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/agent_handoff/close")

class ChatConversationsSystemChannelsApi:
    """chat chat.conversations.system_channels API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: CreateConversationRequest) -> CreateConversationResult:
        """Create a system channel"""
        return self._client.post(f"/im/v3/api/chat/conversations/system_channels", json=body)

    def publish(self, conversation_id: str, body: PostMessageRequest) -> PostedMessageResponse:
        """Publish a system channel message"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/system_channel/publish", json=body)

class ChatConversationsThreadsApi:
    """chat chat.conversations.threads API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: CreateConversationRequest) -> CreateConversationResult:
        """Create a thread conversation"""
        return self._client.post(f"/im/v3/api/chat/conversations/threads", json=body)

class ChatConversationsDirectChatsApi:
    """chat chat.conversations.direct_chats API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def bind(self, body: BindDirectChatRequest) -> CreateConversationResult:
        """Bind a direct chat conversation"""
        return self._client.post(f"/im/v3/api/chat/conversations/direct_chats/bindings", json=body)

class ChatConversationsMembersApi:
    """chat chat.conversations.members API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, conversation_id: str, limit: Optional[int] = None, cursor: Optional[str] = None) -> ListMembersResponse:
        """List conversation members"""
        query = build_query_string([
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members", query))

    def add(self, conversation_id: str, body: AddConversationMemberRequest) -> ConversationMember:
        """Add a conversation member"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members/add", json=body)

    def remove(self, conversation_id: str, body: RemoveConversationMemberRequest) -> AckResponse:
        """Remove a conversation member"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members/remove", json=body)

    def transfer_owner(self, conversation_id: str, body: TransferConversationOwnerRequest) -> ConversationMember:
        """Transfer conversation owner"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members/transfer_owner", json=body)

    def change_role(self, conversation_id: str, body: ChangeConversationMemberRoleRequest) -> ConversationMember:
        """Change conversation member role"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members/change_role", json=body)

    def leave(self, conversation_id: str) -> AckResponse:
        """Leave a conversation"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/members/leave")

class ChatConversationsPreferencesApi:
    """chat chat.conversations.preferences API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, conversation_id: str) -> ConversationPreferencesView:
        """Retrieve conversation preferences"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/preferences")

    def update(self, conversation_id: str, body: UpdateConversationPreferencesRequest) -> ConversationPreferencesView:
        """Update conversation preferences"""
        return self._client.patch(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/preferences", json=body)

class ChatConversationsProfileApi:
    """chat chat.conversations.profile API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, conversation_id: str) -> ConversationProfileView:
        """Retrieve conversation profile"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/profile")

    def update(self, conversation_id: str, body: UpdateConversationProfileRequest) -> ConversationProfileView:
        """Update conversation profile"""
        return self._client.patch(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/profile", json=body)

class ChatConversationsReadCursorApi:
    """chat chat.conversations.read_cursor API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, conversation_id: str) -> ReadCursorView:
        """Retrieve read cursor"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/read_cursor")

    def update(self, conversation_id: str, body: UpdateReadCursorRequest) -> ReadCursorView:
        """Update read cursor"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/read_cursor", json=body)

class ChatConversationsMemberDirectoryApi:
    """chat chat.conversations.member_directory API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, conversation_id: str) -> MemberDirectoryResponse:
        """List member directory"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/member_directory")

class ChatConversationsMessagesApi:
    """chat chat.conversations.messages API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.interaction_summary = ChatConversationsMessagesInteractionSummaryApi(client)


    def list(self, conversation_id: str, after_seq: Optional[int] = None, limit: Optional[int] = None) -> TimelineResponse:
        """List conversation message timeline"""
        query = build_query_string([
            {'name': 'afterSeq', 'value': after_seq, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/messages", query))

    def create(self, conversation_id: str, body: PostMessageRequest) -> PostedMessageResponse:
        """Post a conversation message"""
        return self._client.post(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/messages", json=body)

class ChatConversationsMessagesInteractionSummaryApi:
    """chat chat.conversations.messages.interaction_summary API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self, conversation_id: str, message_id: str) -> MessageInteractionSummaryView:
        """Retrieve message interaction summary"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/interaction_summary")

class ChatConversationsPinsApi:
    """chat chat.conversations.pins API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, conversation_id: str) -> PinnedMessagesResponse:
        """List pinned messages"""
        return self._client.get(f"/im/v3/api/chat/conversations/{serialize_path_parameter(conversation_id, {'name': 'conversationId', 'style': 'simple', 'explode': False})}/pins")

class ChatMessagesApi:
    """chat chat.messages API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.favorites = ChatMessagesFavoritesApi(client)
        self.visibility = ChatMessagesVisibilityApi(client)
        self.reactions = ChatMessagesReactionsApi(client)
        self.pin = ChatMessagesPinApi(client)


    def edit(self, message_id: str, body: EditMessageRequest) -> PostedMessageResponse:
        """Edit a message"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/edit", json=body)

    def recall(self, message_id: str) -> PostedMessageResponse:
        """Recall a message"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/recall")

class ChatMessagesFavoritesApi:
    """chat chat.messages.favorites API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def list(self, limit: Optional[int] = None, cursor: Optional[str] = None, favorite_type: Optional[str] = None, q: Optional[str] = None) -> FavoriteMessagesResponse:
        """List message favorites"""
        query = build_query_string([
            {'name': 'limit', 'value': limit, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'cursor', 'value': cursor, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'favoriteType', 'value': favorite_type, 'style': 'form', 'explode': True, 'allow_reserved': False},
            {'name': 'q', 'value': q, 'style': 'form', 'explode': True, 'allow_reserved': False},
        ])
        return self._client.get(_append_query_string(f"/im/v3/api/chat/messages/favorites", query))

    def create(self, message_id: str, body: FavoriteMessageRequest) -> MessageFavoriteView:
        """Favorite a message"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/favorites", json=body)

    def delete(self, favorite_id: str) -> DeleteMessageFavoriteResponse:
        """Delete a message favorite"""
        return self._client.delete(f"/im/v3/api/chat/messages/favorites/{serialize_path_parameter(favorite_id, {'name': 'favoriteId', 'style': 'simple', 'explode': False})}")

class ChatMessagesVisibilityApi:
    """chat chat.messages.visibility API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def delete(self, message_id: str) -> MessageVisibilityMutationResult:
        """Delete message visibility for the current principal"""
        return self._client.delete(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/visibility")

class ChatMessagesReactionsApi:
    """chat chat.messages.reactions API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, message_id: str, body: MessageReactionRequest) -> MessageReactionMutationResult:
        """Add a message reaction"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/reactions", json=body)

    def delete(self, message_id: str, body: MessageReactionRequest) -> MessageReactionMutationResult:
        """Remove a message reaction"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/reactions/remove", json=body)

class ChatMessagesPinApi:
    """chat chat.messages.pin API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, message_id: str) -> MessagePinMutationResult:
        """Pin a message"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/pin")

    def delete(self, message_id: str) -> MessagePinMutationResult:
        """Unpin a message"""
        return self._client.post(f"/im/v3/api/chat/messages/{serialize_path_parameter(message_id, {'name': 'messageId', 'style': 'simple', 'explode': False})}/unpin")

class ChatRoomsApi:
    """chat chat.rooms API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self, body: CreateRoomRequest) -> CreateConversationResult:
        """Create a live, chat, or game room bound to a group conversation"""
        return self._client.post(f"/im/v3/api/chat/rooms", json=body)

    def get(self, room_id: str) -> RoomView:
        """Get room metadata and active member count"""
        return self._client.get(f"/im/v3/api/chat/rooms/{serialize_path_parameter(room_id, {'name': 'roomId', 'style': 'simple', 'explode': False})}")

    def enter(self, room_id: str) -> EnterRoomResponse:
        """Enter a room as the authenticated principal"""
        return self._client.post(f"/im/v3/api/chat/rooms/{serialize_path_parameter(room_id, {'name': 'roomId', 'style': 'simple', 'explode': False})}/enter")

    def leave(self, room_id: str) -> EnterRoomResponse:
        """Leave a room as the authenticated principal"""
        return self._client.post(f"/im/v3/api/chat/rooms/{serialize_path_parameter(room_id, {'name': 'roomId', 'style': 'simple', 'explode': False})}/leave")
