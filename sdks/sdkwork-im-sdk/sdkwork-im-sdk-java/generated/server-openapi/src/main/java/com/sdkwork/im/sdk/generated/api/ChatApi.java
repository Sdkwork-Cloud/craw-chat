package com.sdkwork.im.sdk.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.model.*;
import java.util.List;
import java.util.Map;

public class ChatApi {
    private final HttpClient client;

    public ChatApi(HttpClient client) {
        this.client = client;
    }

    /** List current inbox window */
    public InboxListResponse inboxList(Integer pageSize, String cursor, String conversationType, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("conversation_type", conversationType, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query));
        return client.convertValue(raw, new TypeReference<InboxListResponse>() {});
    }

    /** Create a conversation */
    public ConversationsCreateResponse201 conversationsCreate(CreateConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsCreateResponse201>() {});
    }

    /** Create an agent dialog */
    public ConversationsAgentDialogsCreateResponse201 conversationsAgentDialogsCreate(CreateAgentDialogRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsAgentDialogsCreateResponse201>() {});
    }

    /** Create an agent handoff */
    public ConversationsAgentHandoffsCreateResponse201 conversationsAgentHandoffsCreate(CreateAgentHandoffRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsAgentHandoffsCreateResponse201>() {});
    }

    /** Create a system channel */
    public ConversationsSystemChannelsCreateResponse201 conversationsSystemChannelsCreate(CreateSystemChannelRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsSystemChannelsCreateResponse201>() {});
    }

    /** Create a thread conversation */
    public ConversationsThreadsCreateResponse201 conversationsThreadsCreate(CreateThreadConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/threads"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsThreadsCreateResponse201>() {});
    }

    /** Create a direct chat conversation binding */
    public ConversationsDirectChatsBindingsCreateResponse201 conversationsDirectChatsBindingsCreate(BindDirectChatRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsDirectChatsBindingsCreateResponse201>() {});
    }

    /** Retrieve agent handoff state */
    public ConversationsAgentHandoffRetrieveResponse conversationsAgentHandoffRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff"));
        return client.convertValue(raw, new TypeReference<ConversationsAgentHandoffRetrieveResponse>() {});
    }

    /** Accept agent handoff */
    public ConversationsAgentHandoffAcceptResponse conversationsAgentHandoffAccept(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/accept"), null);
        return client.convertValue(raw, new TypeReference<ConversationsAgentHandoffAcceptResponse>() {});
    }

    /** Resolve agent handoff */
    public ConversationsAgentHandoffResolveResponse conversationsAgentHandoffResolve(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/resolve"), null);
        return client.convertValue(raw, new TypeReference<ConversationsAgentHandoffResolveResponse>() {});
    }

    /** Close agent handoff */
    public ConversationsAgentHandoffCloseResponse conversationsAgentHandoffClose(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/close"), null);
        return client.convertValue(raw, new TypeReference<ConversationsAgentHandoffCloseResponse>() {});
    }

    /** Retrieve conversation summary */
    public ConversationsRetrieveResponse conversationsRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<ConversationsRetrieveResponse>() {});
    }

    /** List conversation members */
    public ConversationsMembersListResponse conversationsMembersList(String conversationId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members"), query));
        return client.convertValue(raw, new TypeReference<ConversationsMembersListResponse>() {});
    }

    /** Retrieve the current conversation member */
    public ConversationsMembersCurrentRetrieveResponse conversationsMembersCurrentRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/current"));
        return client.convertValue(raw, new TypeReference<ConversationsMembersCurrentRetrieveResponse>() {});
    }

    /** Retrieve assigned group agents */
    public ConversationsAgentsRetrieveResponse conversationsAgentsRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agents"));
        return client.convertValue(raw, new TypeReference<ConversationsAgentsRetrieveResponse>() {});
    }

    /** Update assigned group agents */
    public ConversationsAgentsUpdateResponse conversationsAgentsUpdate(String conversationId, UpdateConversationAgentsRequest body) throws Exception {
        Object raw = client.put(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agents"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsAgentsUpdateResponse>() {});
    }

    /** Add a conversation member */
    public ConversationsMembersAddResponse conversationsMembersAdd(String conversationId, AddConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/add"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsMembersAddResponse>() {});
    }

    /** Remove a conversation member */
    public ConversationsMembersRemoveResponse conversationsMembersRemove(String conversationId, RemoveConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsMembersRemoveResponse>() {});
    }

    /** Transfer conversation owner */
    public ConversationsMembersTransferOwnerResponse conversationsMembersTransferOwner(String conversationId, TransferConversationOwnerRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/transfer_owner"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsMembersTransferOwnerResponse>() {});
    }

    /** Change conversation member role */
    public ConversationsMembersChangeRoleResponse conversationsMembersChangeRole(String conversationId, ChangeConversationMemberRoleRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/change_role"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsMembersChangeRoleResponse>() {});
    }

    /** Leave a conversation */
    public ConversationsMembersLeaveResponse conversationsMembersLeave(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/leave"), null);
        return client.convertValue(raw, new TypeReference<ConversationsMembersLeaveResponse>() {});
    }

    /** Accept a conversation invitation */
    public ConversationsMembersAcceptInvitationResponse conversationsMembersAcceptInvitation(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/accept_invitation"), null);
        return client.convertValue(raw, new TypeReference<ConversationsMembersAcceptInvitationResponse>() {});
    }

    /** Retrieve conversation preferences */
    public ConversationsPreferencesRetrieveResponse conversationsPreferencesRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/preferences"));
        return client.convertValue(raw, new TypeReference<ConversationsPreferencesRetrieveResponse>() {});
    }

    /** Update conversation preferences */
    public ConversationsPreferencesUpdateResponse conversationsPreferencesUpdate(String conversationId, UpdateConversationPreferencesRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/preferences"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsPreferencesUpdateResponse>() {});
    }

    /** Retrieve conversation profile */
    public ConversationsProfileRetrieveResponse conversationsProfileRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/profile"));
        return client.convertValue(raw, new TypeReference<ConversationsProfileRetrieveResponse>() {});
    }

    /** Update conversation profile */
    public ConversationsProfileUpdateResponse conversationsProfileUpdate(String conversationId, UpdateConversationProfileRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/profile"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsProfileUpdateResponse>() {});
    }

    /** Retrieve read cursor */
    public ConversationsReadCursorRetrieveResponse conversationsReadCursorRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/read_cursor"));
        return client.convertValue(raw, new TypeReference<ConversationsReadCursorRetrieveResponse>() {});
    }

    /** Update read cursor */
    public ConversationsReadCursorUpdateResponse conversationsReadCursorUpdate(String conversationId, UpdateReadCursorRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/read_cursor"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsReadCursorUpdateResponse>() {});
    }

    /** List member directory */
    public ConversationsMemberDirectoryListResponse conversationsMemberDirectoryList(String conversationId, String cursor, Integer pageSize) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/member_directory"), query));
        return client.convertValue(raw, new TypeReference<ConversationsMemberDirectoryListResponse>() {});
    }

    /** List conversation message history */
    public ConversationMessageListResponse conversationsMessagesList(String conversationId, String cursor, Integer pageSize) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages"), query));
        return client.convertValue(raw, new TypeReference<ConversationMessageListResponse>() {});
    }

    /** Post a conversation message */
    public ConversationsMessagesCreateResponse201 conversationsMessagesCreate(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsMessagesCreateResponse201>() {});
    }

    /** Publish a system channel message */
    public ConversationsSystemChannelPublishResponse conversationsSystemChannelPublish(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/system_channel/publish"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationsSystemChannelPublishResponse>() {});
    }

    /** List pinned messages */
    public ConversationsPinsListResponse conversationsPinsList(String conversationId, String cursor, Integer pageSize) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/pins"), query));
        return client.convertValue(raw, new TypeReference<ConversationsPinsListResponse>() {});
    }

    /** Retrieve message interaction summary */
    public ConversationsMessagesInteractionSummaryRetrieveResponse conversationsMessagesInteractionSummaryRetrieve(String conversationId, String messageId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/interaction_summary"));
        return client.convertValue(raw, new TypeReference<ConversationsMessagesInteractionSummaryRetrieveResponse>() {});
    }

    /** Edit a message */
    public MessagesEditResponse messagesEdit(String messageId, EditMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/edit"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessagesEditResponse>() {});
    }

    /** Recall a message */
    public MessagesRecallResponse messagesRecall(String messageId, RecallMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/recall"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessagesRecallResponse>() {});
    }

    /** List message favorites */
    public MessagesFavoritesListResponse messagesFavoritesList(Integer pageSize, String cursor, String favoriteType, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query));
        return client.convertValue(raw, new TypeReference<MessagesFavoritesListResponse>() {});
    }

    /** Favorite a message */
    public MessagesFavoritesCreateResponse201 messagesFavoritesCreate(String messageId, FavoriteMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/favorites"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessagesFavoritesCreateResponse201>() {});
    }

    /** Delete a message favorite */
    public Void messagesFavoritesDelete(String favoriteId) throws Exception {
        client.delete(ApiPaths.imPath("/chat/messages/favorites/" + serializePathParameter(favoriteId, new PathParameterSpec("favoriteId", "simple", false)) + ""));
        return null;
    }

    /** Delete message visibility for the current principal */
    public Void messagesVisibilityDelete(String messageId) throws Exception {
        client.delete(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/visibility"));
        return null;
    }

    /** Add a message reaction */
    public MessagesReactionsCreateResponse201 messagesReactionsCreate(String messageId, MessageReactionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/reactions"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessagesReactionsCreateResponse201>() {});
    }

    /** Remove a message reaction */
    public MessagesReactionsRemoveResponse messagesReactionsRemove(String messageId, MessageReactionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/reactions/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessagesReactionsRemoveResponse>() {});
    }

    /** Pin a message */
    public MessagesPinResponse messagesPin(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/pin"), null);
        return client.convertValue(raw, new TypeReference<MessagesPinResponse>() {});
    }

    /** Unpin a message */
    public MessagesUnpinResponse messagesUnpin(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/unpin"), null);
        return client.convertValue(raw, new TypeReference<MessagesUnpinResponse>() {});
    }

    /** Create a live, chat, or game room bound to a group conversation */
    public RoomsCreateResponse201 roomsCreate(CreateRoomRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RoomsCreateResponse201>() {});
    }

    /** Retrieve room metadata and active member count */
    public RoomsRetrieveResponse roomsRetrieve(String roomId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<RoomsRetrieveResponse>() {});
    }

    /** Enter a room as the authenticated principal */
    public RoomsEnterResponse roomsEnter(String roomId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + "/enter"), null);
        return client.convertValue(raw, new TypeReference<RoomsEnterResponse>() {});
    }

    /** Leave a room as the authenticated principal */
    public RoomsLeaveResponse roomsLeave(String roomId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + "/leave"), null);
        return client.convertValue(raw, new TypeReference<RoomsLeaveResponse>() {});
    }

    private record PathParameterSpec(String name, String style, boolean explode) {}

    private static String serializePathParameter(Object value, PathParameterSpec spec) {
        if (value == null) {
            return "";
        }
        String style = spec.style() == null || spec.style().isBlank() ? "simple" : spec.style();
        if (value instanceof Iterable<?> iterable) {
            return serializePathArray(spec.name(), iterable, style, spec.explode());
        }
        if (value instanceof Map<?, ?> map) {
            return serializePathObject(spec.name(), map, style, spec.explode());
        }
        return pathPrimitivePrefix(spec.name(), style) + pathEncode(String.valueOf(value));
    }

    private static String serializePathArray(String name, Iterable<?> values, String style, boolean explode) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(pathEncode(String.valueOf(item)));
            }
        }
        if (serialized.isEmpty()) {
            return pathPrefix(name, style);
        }
        if ("matrix".equals(style)) {
            if (explode) {
                List<String> parts = new java.util.ArrayList<>();
                for (String item : serialized) {
                    parts.add(";" + name + "=" + item);
                }
                return String.join("", parts);
            }
            return ";" + name + "=" + String.join(",", serialized);
        }
        String separator = explode ? "." : ",";
        return pathPrefix(name, style) + String.join(separator, serialized);
    }

    private static String serializePathObject(String name, Map<?, ?> values, String style, boolean explode) {
        List<String> entries = new java.util.ArrayList<>();
        List<String> exploded = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            String escapedKey = pathEncode(String.valueOf(key));
            String escapedValue = pathEncode(String.valueOf(value));
            if (explode) {
                if ("matrix".equals(style)) {
                    exploded.add(";" + escapedKey + "=" + escapedValue);
                } else {
                    exploded.add(escapedKey + "=" + escapedValue);
                }
            } else {
                entries.add(escapedKey);
                entries.add(escapedValue);
            }
        });
        if ("matrix".equals(style)) {
            if (explode) {
                return String.join("", exploded);
            }
            return ";" + name + "=" + String.join(",", entries);
        }
        if (explode) {
            String separator = "label".equals(style) ? "." : ",";
            return pathPrefix(name, style) + String.join(separator, exploded);
        }
        return pathPrefix(name, style) + String.join(",", entries);
    }

    private static String pathPrefix(String name, String style) {
        if ("label".equals(style)) {
            return ".";
        }
        if ("matrix".equals(style)) {
            return ";" + name;
        }
        return "";
    }

    private static String pathPrimitivePrefix(String name, String style) {
        if ("matrix".equals(style)) {
            return ";" + name + "=";
        }
        return pathPrefix(name, style);
    }

    private static String pathEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20");
    }

    private record QueryParameterSpec(String name, Object value, String style, boolean explode, boolean allowReserved, String contentType) {}

    private static String buildQueryString(List<QueryParameterSpec> parameters) throws Exception {
        List<String> pairs = new java.util.ArrayList<>();
        for (QueryParameterSpec parameter : parameters) {
            appendSerializedParameter(pairs, parameter);
        }
        return String.join("&", pairs);
    }

    private static void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) throws Exception {
        if (parameter.value() == null) {
            return;
        }
        if (parameter.contentType() != null && !parameter.contentType().isBlank()) {
            String json = clientObjectMapper().writeValueAsString(parameter.value());
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(json, parameter.allowReserved()));
            return;
        }

        String style = parameter.style() == null || parameter.style().isBlank() ? "form" : parameter.style();
        Object value = parameter.value();
        if ("deepObject".equals(style) && value instanceof Map<?, ?> map) {
            appendDeepObjectParameter(pairs, parameter.name(), map, parameter.allowReserved());
        } else if (value instanceof Iterable<?> iterable) {
            appendArrayParameter(pairs, parameter.name(), iterable, style, parameter.explode(), parameter.allowReserved());
        } else if (value instanceof Map<?, ?> map) {
            appendObjectParameter(pairs, parameter.name(), map, style, parameter.explode(), parameter.allowReserved());
        } else {
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(String.valueOf(value), parameter.allowReserved()));
        }
    }

    private static void appendArrayParameter(List<String> pairs, String name, Iterable<?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(String.valueOf(item));
            }
        }
        if (serialized.isEmpty()) {
            return;
        }
        if ("form".equals(style) && explode) {
            for (String item : serialized) {
                pairs.add(urlEncode(name) + "=" + encodeQueryValue(item, allowReserved));
            }
            return;
        }
        pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
    }

    private static void appendObjectParameter(List<String> pairs, String name, Map<?, ?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            if ("form".equals(style) && explode) {
                pairs.add(urlEncode(String.valueOf(key)) + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            } else {
                serialized.add(String.valueOf(key));
                serialized.add(String.valueOf(value));
            }
        });
        if (!serialized.isEmpty()) {
            pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
        }
    }

    private static void appendDeepObjectParameter(List<String> pairs, String name, Map<?, ?> values, boolean allowReserved) {
        values.forEach((key, value) -> {
            if (value != null) {
                pairs.add(urlEncode(name + "[" + key + "]") + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            }
        });
    }

    private static String encodeQueryValue(String value, boolean allowReserved) {
        String encoded = urlEncode(value);
        if (!allowReserved) {
            return encoded;
        }
        return encoded
            .replace("%3A", ":").replace("%2F", "/").replace("%3F", "?").replace("%23", "#")
            .replace("%5B", "[").replace("%5D", "]").replace("%40", "@").replace("%21", "!")
            .replace("%24", "$").replace("%26", "&").replace("%27", "'").replace("%28", "(")
            .replace("%29", ")").replace("%2A", "*").replace("%2B", "+").replace("%2C", ",")
            .replace("%3B", ";").replace("%3D", "=");
    }

    private static com.fasterxml.jackson.databind.ObjectMapper clientObjectMapper() {
        return new com.fasterxml.jackson.databind.ObjectMapper();
    }


    private static String urlEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8);
    }
}
