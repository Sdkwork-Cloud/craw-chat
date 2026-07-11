import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class ChatApi {
  final HttpClient _client;

  ChatApi(this._client);

  /// List IM contacts
  Future<ContactsListResponse?> contactsList([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/chat/contacts'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactsListResponse.fromJson(map);
    })();
  }

  /// List current inbox window
  Future<InboxListResponse?> inboxList([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/chat/inbox'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : InboxListResponse.fromJson(map);
    })();
  }

  /// Create a conversation
  Future<ConversationsCreateResponse201?> conversationsCreate(CreateConversationRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsCreateResponse201.fromJson(map);
    })();
  }

  /// Create an agent dialog
  Future<ConversationsAgentDialogsCreateResponse201?> conversationsAgentDialogsCreate(CreateAgentDialogRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/agent_dialogs'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentDialogsCreateResponse201.fromJson(map);
    })();
  }

  /// Create an agent handoff
  Future<ConversationsAgentHandoffsCreateResponse201?> conversationsAgentHandoffsCreate(CreateAgentHandoffRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/agent_handoffs'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentHandoffsCreateResponse201.fromJson(map);
    })();
  }

  /// Create a system channel
  Future<ConversationsSystemChannelsCreateResponse201?> conversationsSystemChannelsCreate(CreateSystemChannelRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/system_channels'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsSystemChannelsCreateResponse201.fromJson(map);
    })();
  }

  /// Create a thread conversation
  Future<ConversationsThreadsCreateResponse201?> conversationsThreadsCreate(CreateThreadConversationRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/threads'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsThreadsCreateResponse201.fromJson(map);
    })();
  }

  /// Create a direct chat conversation binding
  Future<ConversationsDirectChatsBindingsCreateResponse201?> conversationsDirectChatsBindingsCreate(BindDirectChatRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/direct_chats/bindings'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsDirectChatsBindingsCreateResponse201.fromJson(map);
    })();
  }

  /// Retrieve agent handoff state
  Future<ConversationsAgentHandoffRetrieveResponse?> conversationsAgentHandoffRetrieve(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/agent_handoff'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentHandoffRetrieveResponse.fromJson(map);
    })();
  }

  /// Accept agent handoff
  Future<ConversationsAgentHandoffAcceptResponse?> conversationsAgentHandoffAccept(String conversationId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/agent_handoff/accept'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentHandoffAcceptResponse.fromJson(map);
    })();
  }

  /// Resolve agent handoff
  Future<ConversationsAgentHandoffResolveResponse?> conversationsAgentHandoffResolve(String conversationId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/agent_handoff/resolve'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentHandoffResolveResponse.fromJson(map);
    })();
  }

  /// Close agent handoff
  Future<ConversationsAgentHandoffCloseResponse?> conversationsAgentHandoffClose(String conversationId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/agent_handoff/close'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsAgentHandoffCloseResponse.fromJson(map);
    })();
  }

  /// Retrieve conversation summary
  Future<ConversationsRetrieveResponse?> conversationsRetrieve(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsRetrieveResponse.fromJson(map);
    })();
  }

  /// List conversation members
  Future<ConversationsMembersListResponse?> conversationsMembersList(String conversationId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersListResponse.fromJson(map);
    })();
  }

  /// Add a conversation member
  Future<ConversationsMembersAddResponse?> conversationsMembersAdd(String conversationId, AddConversationMemberRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/add'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersAddResponse.fromJson(map);
    })();
  }

  /// Remove a conversation member
  Future<ConversationsMembersRemoveResponse?> conversationsMembersRemove(String conversationId, RemoveConversationMemberRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/remove'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersRemoveResponse.fromJson(map);
    })();
  }

  /// Transfer conversation owner
  Future<ConversationsMembersTransferOwnerResponse?> conversationsMembersTransferOwner(String conversationId, TransferConversationOwnerRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/transfer_owner'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersTransferOwnerResponse.fromJson(map);
    })();
  }

  /// Change conversation member role
  Future<ConversationsMembersChangeRoleResponse?> conversationsMembersChangeRole(String conversationId, ChangeConversationMemberRoleRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/change_role'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersChangeRoleResponse.fromJson(map);
    })();
  }

  /// Leave a conversation
  Future<ConversationsMembersLeaveResponse?> conversationsMembersLeave(String conversationId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/leave'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersLeaveResponse.fromJson(map);
    })();
  }

  /// Accept a conversation invitation
  Future<ConversationsMembersAcceptInvitationResponse?> conversationsMembersAcceptInvitation(String conversationId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/members/accept_invitation'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMembersAcceptInvitationResponse.fromJson(map);
    })();
  }

  /// Retrieve conversation preferences
  Future<ConversationsPreferencesRetrieveResponse?> conversationsPreferencesRetrieve(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/preferences'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsPreferencesRetrieveResponse.fromJson(map);
    })();
  }

  /// Update conversation preferences
  Future<ConversationsPreferencesUpdateResponse?> conversationsPreferencesUpdate(String conversationId, UpdateConversationPreferencesRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/preferences'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsPreferencesUpdateResponse.fromJson(map);
    })();
  }

  /// Retrieve conversation profile
  Future<ConversationsProfileRetrieveResponse?> conversationsProfileRetrieve(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/profile'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsProfileRetrieveResponse.fromJson(map);
    })();
  }

  /// Update conversation profile
  Future<ConversationsProfileUpdateResponse?> conversationsProfileUpdate(String conversationId, UpdateConversationProfileRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/profile'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsProfileUpdateResponse.fromJson(map);
    })();
  }

  /// Retrieve read cursor
  Future<ConversationsReadCursorRetrieveResponse?> conversationsReadCursorRetrieve(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/read_cursor'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsReadCursorRetrieveResponse.fromJson(map);
    })();
  }

  /// Update read cursor
  Future<ConversationsReadCursorUpdateResponse?> conversationsReadCursorUpdate(String conversationId, UpdateReadCursorRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/read_cursor'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsReadCursorUpdateResponse.fromJson(map);
    })();
  }

  /// List member directory
  Future<ConversationsMemberDirectoryListResponse?> conversationsMemberDirectoryList(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/member_directory'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMemberDirectoryListResponse.fromJson(map);
    })();
  }

  /// List conversation message history
  Future<ConversationMessageListResponse?> conversationsMessagesList(String conversationId, [int? afterSeq, int? pageSize]) async {
    final query = buildQueryString([
      QueryParameterSpec('afterSeq', afterSeq, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/messages'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationMessageListResponse.fromJson(map);
    })();
  }

  /// Post a conversation message
  Future<ConversationsMessagesCreateResponse201?> conversationsMessagesCreate(String conversationId, PostMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/messages'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMessagesCreateResponse201.fromJson(map);
    })();
  }

  /// Publish a system channel message
  Future<ConversationsSystemChannelPublishResponse?> conversationsSystemChannelPublish(String conversationId, PostMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/system_channel/publish'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsSystemChannelPublishResponse.fromJson(map);
    })();
  }

  /// List pinned messages
  Future<ConversationsPinsListResponse?> conversationsPinsList(String conversationId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/pins'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsPinsListResponse.fromJson(map);
    })();
  }

  /// Retrieve message interaction summary
  Future<ConversationsMessagesInteractionSummaryRetrieveResponse?> conversationsMessagesInteractionSummaryRetrieve(String conversationId, String messageId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/conversations/${serializePathParameter(conversationId, const PathParameterSpec('conversationId', 'simple', false))}/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/interaction_summary'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationsMessagesInteractionSummaryRetrieveResponse.fromJson(map);
    })();
  }

  /// Edit a message
  Future<MessagesEditResponse?> messagesEdit(String messageId, EditMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/edit'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesEditResponse.fromJson(map);
    })();
  }

  /// Recall a message
  Future<MessagesRecallResponse?> messagesRecall(String messageId, RecallMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/recall'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesRecallResponse.fromJson(map);
    })();
  }

  /// List message favorites
  Future<MessagesFavoritesListResponse?> messagesFavoritesList([int? pageSize, String? cursor, String? favoriteType, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('favoriteType', favoriteType, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/chat/messages/favorites'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesFavoritesListResponse.fromJson(map);
    })();
  }

  /// Favorite a message
  Future<MessagesFavoritesCreateResponse201?> messagesFavoritesCreate(String messageId, FavoriteMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/favorites'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesFavoritesCreateResponse201.fromJson(map);
    })();
  }

  /// Delete a message favorite
  Future<void> messagesFavoritesDelete(String favoriteId) async {
    await _client.delete(ApiPaths.imPath('/chat/messages/favorites/${serializePathParameter(favoriteId, const PathParameterSpec('favoriteId', 'simple', false))}'));
  }

  /// Delete message visibility for the current principal
  Future<void> messagesVisibilityDelete(String messageId) async {
    await _client.delete(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/visibility'));
  }

  /// Add a message reaction
  Future<MessagesReactionsCreateResponse201?> messagesReactionsCreate(String messageId, MessageReactionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/reactions'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesReactionsCreateResponse201.fromJson(map);
    })();
  }

  /// Remove a message reaction
  Future<MessagesReactionsRemoveResponse?> messagesReactionsRemove(String messageId, MessageReactionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/reactions/remove'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesReactionsRemoveResponse.fromJson(map);
    })();
  }

  /// Pin a message
  Future<MessagesPinResponse?> messagesPin(String messageId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/pin'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesPinResponse.fromJson(map);
    })();
  }

  /// Unpin a message
  Future<MessagesUnpinResponse?> messagesUnpin(String messageId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/messages/${serializePathParameter(messageId, const PathParameterSpec('messageId', 'simple', false))}/unpin'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessagesUnpinResponse.fromJson(map);
    })();
  }

  /// Create a live, chat, or game room bound to a group conversation
  Future<RoomsCreateResponse201?> roomsCreate(CreateRoomRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/chat/rooms'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoomsCreateResponse201.fromJson(map);
    })();
  }

  /// Retrieve room metadata and active member count
  Future<RoomsRetrieveResponse?> roomsRetrieve(String roomId) async {
    final response = await _client.get(ApiPaths.imPath('/chat/rooms/${serializePathParameter(roomId, const PathParameterSpec('roomId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoomsRetrieveResponse.fromJson(map);
    })();
  }

  /// Enter a room as the authenticated principal
  Future<RoomsEnterResponse?> roomsEnter(String roomId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/rooms/${serializePathParameter(roomId, const PathParameterSpec('roomId', 'simple', false))}/enter'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoomsEnterResponse.fromJson(map);
    })();
  }

  /// Leave a room as the authenticated principal
  Future<RoomsLeaveResponse?> roomsLeave(String roomId) async {
    final response = await _client.post(ApiPaths.imPath('/chat/rooms/${serializePathParameter(roomId, const PathParameterSpec('roomId', 'simple', false))}/leave'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoomsLeaveResponse.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
class QueryParameterSpec {
  final String name;
  final dynamic value;
  final String style;
  final bool explode;
  final bool allowReserved;
  final String? contentType;

  const QueryParameterSpec(
    this.name,
    this.value,
    this.style,
    this.explode,
    this.allowReserved,
    this.contentType,
  );
}

String buildQueryString(List<QueryParameterSpec> parameters) {
  final pairs = <String>[];
  for (final parameter in parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) {
  final value = parameter.value;
  if (value == null) return;

  final contentType = parameter.contentType;
  if (contentType != null && contentType.trim().isNotEmpty) {
    pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(jsonEncode(value), parameter.allowReserved)}');
    return;
  }

  final style = parameter.style.trim().isEmpty ? 'form' : parameter.style;
  if (style == 'deepObject' && value is Map) {
    appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved);
    return;
  }
  if (value is Iterable) {
    appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  if (value is Map) {
    appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(value.toString(), parameter.allowReserved)}');
}

void appendArrayParameter(
  List<String> pairs,
  String name,
  Iterable values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = values.where((item) => item != null).map((item) => item.toString()).toList();
  if (serialized.isEmpty) return;
  if (style == 'form' && explode) {
    for (final item in serialized) {
      pairs.add('${urlEncode(name)}=${encodeQueryValue(item, allowReserved)}');
    }
    return;
  }
  pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
}

void appendObjectParameter(
  List<String> pairs,
  String name,
  Map values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    if (style == 'form' && explode) {
      pairs.add('${urlEncode(key.toString())}=${encodeQueryValue(value.toString(), allowReserved)}');
      return;
    }
    serialized.add(key.toString());
    serialized.add(value.toString());
  });
  if (serialized.isNotEmpty) {
    pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
  }
}

void appendDeepObjectParameter(List<String> pairs, String name, Map values, bool allowReserved) {
  values.forEach((key, value) {
    if (value != null) {
      pairs.add('${urlEncode('$name[$key]')}=${encodeQueryValue(value.toString(), allowReserved)}');
    }
  });
}

String encodeQueryValue(String value, bool allowReserved) {
  var encoded = urlEncode(value);
  if (!allowReserved) return encoded;
  const replacements = <String, String>{
    '%3A': ':',
    '%2F': '/',
    '%3F': '?',
    '%23': '#',
    '%5B': '[',
    '%5D': ']',
    '%40': '@',
    '%21': '!',
    '%24': r'$',
    '%26': '&',
    '%27': "'",
    '%28': '(',
    '%29': ')',
    '%2A': '*',
    '%2B': '+',
    '%2C': ',',
    '%3B': ';',
    '%3D': '=',
  };
  replacements.forEach((escaped, reserved) {
    encoded = encoded.replaceAll(escaped, reserved);
  });
  return encoded;
}

String urlEncode(String value) => Uri.encodeQueryComponent(value);
