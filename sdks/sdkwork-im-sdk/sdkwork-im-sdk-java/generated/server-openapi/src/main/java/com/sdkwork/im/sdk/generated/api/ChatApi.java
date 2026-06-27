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

    /** List IM contacts */
    public ContactsResponse contactsList(Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/contacts"), query));
        return client.convertValue(raw, new TypeReference<ContactsResponse>() {});
    }

    /** Retrieve current inbox window */
    public InboxResponse inboxRetrieve(Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query));
        return client.convertValue(raw, new TypeReference<InboxResponse>() {});
    }

    /** Create a conversation */
    public CreateConversationResult conversationsCreate(CreateConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create an agent dialog */
    public CreateConversationResult conversationsAgentDialogsCreate(CreateAgentDialogRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create an agent handoff */
    public AckResponse conversationsAgentHandoffsCreate(CreateAgentDialogRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Create a system channel */
    public CreateConversationResult conversationsSystemChannelsCreate(CreateConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create a thread conversation */
    public CreateConversationResult conversationsThreadsCreate(CreateConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/threads"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Bind a direct chat conversation */
    public CreateConversationResult conversationsDirectChatsBind(BindDirectChatRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Retrieve agent handoff state */
    public AckResponse conversationsAgentHandoffRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff"));
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Accept agent handoff */
    public AckResponse conversationsAgentHandoffAccept(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/accept"), null);
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Resolve agent handoff */
    public AckResponse conversationsAgentHandoffResolve(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/resolve"), null);
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Close agent handoff */
    public AckResponse conversationsAgentHandoffClose(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/agent_handoff/close"), null);
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Retrieve conversation summary */
    public ConversationSummaryView conversationsRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<ConversationSummaryView>() {});
    }

    /** List conversation members */
    public ListMembersResponse conversationsMembersList(String conversationId, Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members"), query));
        return client.convertValue(raw, new TypeReference<ListMembersResponse>() {});
    }

    /** Add a conversation member */
    public ConversationMember conversationsMembersAdd(String conversationId, AddConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/add"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Remove a conversation member */
    public AckResponse conversationsMembersRemove(String conversationId, RemoveConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Transfer conversation owner */
    public ConversationMember conversationsMembersTransferOwner(String conversationId, TransferConversationOwnerRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/transfer_owner"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Change conversation member role */
    public ConversationMember conversationsMembersChangeRole(String conversationId, ChangeConversationMemberRoleRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/change_role"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Leave a conversation */
    public AckResponse conversationsMembersLeave(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/members/leave"), null);
        return client.convertValue(raw, new TypeReference<AckResponse>() {});
    }

    /** Retrieve conversation preferences */
    public ConversationPreferencesView conversationsPreferencesRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/preferences"));
        return client.convertValue(raw, new TypeReference<ConversationPreferencesView>() {});
    }

    /** Update conversation preferences */
    public ConversationPreferencesView conversationsPreferencesUpdate(String conversationId, UpdateConversationPreferencesRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/preferences"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationPreferencesView>() {});
    }

    /** Retrieve conversation profile */
    public ConversationProfileView conversationsProfileRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/profile"));
        return client.convertValue(raw, new TypeReference<ConversationProfileView>() {});
    }

    /** Update conversation profile */
    public ConversationProfileView conversationsProfileUpdate(String conversationId, UpdateConversationProfileRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/profile"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationProfileView>() {});
    }

    /** Retrieve read cursor */
    public ReadCursorView conversationsReadCursorRetrieve(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/read_cursor"));
        return client.convertValue(raw, new TypeReference<ReadCursorView>() {});
    }

    /** Update read cursor */
    public ReadCursorView conversationsReadCursorUpdate(String conversationId, UpdateReadCursorRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/read_cursor"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ReadCursorView>() {});
    }

    /** List member directory */
    public MemberDirectoryResponse conversationsMemberDirectoryList(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/member_directory"));
        return client.convertValue(raw, new TypeReference<MemberDirectoryResponse>() {});
    }

    /** List conversation message timeline */
    public TimelineResponse conversationsMessagesList(String conversationId, Integer afterSeq, Integer limit) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("afterSeq", afterSeq, "form", true, false, null),
            new QueryParameterSpec("limit", limit, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages"), query));
        return client.convertValue(raw, new TypeReference<TimelineResponse>() {});
    }

    /** Post a conversation message */
    public PostedMessageResponse conversationsMessagesCreate(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostedMessageResponse>() {});
    }

    /** Publish a system channel message */
    public PostedMessageResponse conversationsSystemChannelPublish(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/system_channel/publish"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostedMessageResponse>() {});
    }

    /** List pinned messages */
    public PinnedMessagesResponse conversationsPinsList(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/pins"));
        return client.convertValue(raw, new TypeReference<PinnedMessagesResponse>() {});
    }

    /** Retrieve message interaction summary */
    public MessageInteractionSummaryView conversationsMessagesInteractionSummaryRetrieve(String conversationId, String messageId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/conversations/" + serializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false)) + "/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/interaction_summary"));
        return client.convertValue(raw, new TypeReference<MessageInteractionSummaryView>() {});
    }

    /** Edit a message */
    public PostedMessageResponse messagesEdit(String messageId, EditMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/edit"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostedMessageResponse>() {});
    }

    /** Recall a message */
    public PostedMessageResponse messagesRecall(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/recall"), null);
        return client.convertValue(raw, new TypeReference<PostedMessageResponse>() {});
    }

    /** List message favorites */
    public FavoriteMessagesResponse messagesFavoritesList(Integer limit, String cursor, String favoriteType, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query));
        return client.convertValue(raw, new TypeReference<FavoriteMessagesResponse>() {});
    }

    /** Favorite a message */
    public MessageFavoriteView messagesFavoritesCreate(String messageId, FavoriteMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/favorites"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessageFavoriteView>() {});
    }

    /** Delete a message favorite */
    public DeleteMessageFavoriteResponse messagesFavoritesDelete(String favoriteId) throws Exception {
        Object raw = client.delete(ApiPaths.imPath("/chat/messages/favorites/" + serializePathParameter(favoriteId, new PathParameterSpec("favoriteId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<DeleteMessageFavoriteResponse>() {});
    }

    /** Delete message visibility for the current principal */
    public MessageVisibilityMutationResult messagesVisibilityDelete(String messageId) throws Exception {
        Object raw = client.delete(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/visibility"));
        return client.convertValue(raw, new TypeReference<MessageVisibilityMutationResult>() {});
    }

    /** Add a message reaction */
    public MessageReactionMutationResult messagesReactionsCreate(String messageId, MessageReactionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/reactions"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessageReactionMutationResult>() {});
    }

    /** Remove a message reaction */
    public MessageReactionMutationResult messagesReactionsDelete(String messageId, MessageReactionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/reactions/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessageReactionMutationResult>() {});
    }

    /** Pin a message */
    public MessagePinMutationResult messagesPinCreate(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/pin"), null);
        return client.convertValue(raw, new TypeReference<MessagePinMutationResult>() {});
    }

    /** Unpin a message */
    public MessagePinMutationResult messagesPinDelete(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/messages/" + serializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false)) + "/unpin"), null);
        return client.convertValue(raw, new TypeReference<MessagePinMutationResult>() {});
    }

    /** Create a live, chat, or game room bound to a group conversation */
    public CreateConversationResult roomsCreate(CreateRoomRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Get room metadata and active member count */
    public RoomView roomsGet(String roomId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<RoomView>() {});
    }

    /** Enter a room as the authenticated principal */
    public EnterRoomResponse roomsEnter(String roomId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + "/enter"), null);
        return client.convertValue(raw, new TypeReference<EnterRoomResponse>() {});
    }

    /** Leave a room as the authenticated principal */
    public EnterRoomResponse roomsLeave(String roomId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/chat/rooms/" + serializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false)) + "/leave"), null);
        return client.convertValue(raw, new TypeReference<EnterRoomResponse>() {});
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
