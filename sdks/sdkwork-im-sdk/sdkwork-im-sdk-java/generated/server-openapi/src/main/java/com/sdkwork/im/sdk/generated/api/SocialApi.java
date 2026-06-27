package com.sdkwork.im.sdk.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.model.*;
import java.util.List;
import java.util.Map;

public class SocialApi {
    private final HttpClient client;

    public SocialApi(HttpClient client) {
        this.client = client;
    }

    /** Search social users */
    public SocialUserSearchResponse usersList(String q, Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("q", q, "form", true, false, null),
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/users"), query));
        return client.convertValue(raw, new TypeReference<SocialUserSearchResponse>() {});
    }

    /** List friend requests */
    public SocialFriendRequestListResponse friendRequestsList(String direction, String status, Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("direction", direction, "form", true, false, null),
            new QueryParameterSpec("status", status, "form", true, false, null),
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/friend_requests"), query));
        return client.convertValue(raw, new TypeReference<SocialFriendRequestListResponse>() {});
    }

    /** Create a friend request */
    public SocialFriendRequestMutationResponse friendRequestsCreate(SubmitFriendRequestRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/friend_requests"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SocialFriendRequestMutationResponse>() {});
    }

    /** Accept a friend request */
    public SocialFriendRequestAcceptanceResponse friendRequestsAccept(String requestId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/accept"), null);
        return client.convertValue(raw, new TypeReference<SocialFriendRequestAcceptanceResponse>() {});
    }

    /** Decline a friend request */
    public SocialFriendRequestMutationResponse friendRequestsDecline(String requestId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/decline"), null);
        return client.convertValue(raw, new TypeReference<SocialFriendRequestMutationResponse>() {});
    }

    /** Cancel a friend request */
    public SocialFriendRequestMutationResponse friendRequestsCancel(String requestId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/friend_requests/" + serializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false)) + "/cancel"), null);
        return client.convertValue(raw, new TypeReference<SocialFriendRequestMutationResponse>() {});
    }

    /** Remove a friendship */
    public SocialFriendshipMutationResponse friendshipsRemove(String friendshipId) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/friendships/" + serializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false)) + "/remove"), null);
        return client.convertValue(raw, new TypeReference<SocialFriendshipMutationResponse>() {});
    }

    /** List contact tags */
    public ContactTagsResponse contactsTagsList(Integer limit, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("limit", limit, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/contacts/tags"), query));
        return client.convertValue(raw, new TypeReference<ContactTagsResponse>() {});
    }

    /** Create a contact tag */
    public ContactTagView contactsTagsCreate(CreateContactTagRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/contacts/tags"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ContactTagView>() {});
    }

    /** Update a contact tag */
    public ContactTagView contactsTagsUpdate(String tagId, UpdateContactTagRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/social/contacts/tags/" + serializePathParameter(tagId, new PathParameterSpec("tagId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ContactTagView>() {});
    }

    /** Delete a contact tag */
    public DeleteContactTagResponse contactsTagsDelete(String tagId) throws Exception {
        Object raw = client.delete(ApiPaths.imPath("/social/contacts/tags/" + serializePathParameter(tagId, new PathParameterSpec("tagId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<DeleteContactTagResponse>() {});
    }

    /** Create a contact recommendation */
    public ContactRecommendationView contactsRecommendationsCreate(String targetUserId, CreateContactRecommendationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/social/contacts/" + serializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false)) + "/recommendations"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ContactRecommendationView>() {});
    }

    /** Retrieve contact preferences */
    public ContactPreferencesView contactsPreferencesRetrieve(String targetUserId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/social/contacts/" + serializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false)) + "/preferences"));
        return client.convertValue(raw, new TypeReference<ContactPreferencesView>() {});
    }

    /** Update contact preferences */
    public ContactPreferencesView contactsPreferencesUpdate(String targetUserId, UpdateContactPreferencesRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/social/contacts/" + serializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false)) + "/preferences"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ContactPreferencesView>() {});
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
