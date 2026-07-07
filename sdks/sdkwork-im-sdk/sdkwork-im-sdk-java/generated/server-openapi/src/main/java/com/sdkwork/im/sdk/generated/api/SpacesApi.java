package com.sdkwork.im.sdk.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.model.*;
import java.util.List;
import java.util.Map;

public class SpacesApi {
    private final HttpClient client;

    public SpacesApi(HttpClient client) {
        this.client = client;
    }

    /** Create a space */
    public SpacesCreateResponse201 create(SpaceCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesCreateResponse201>() {});
    }

    /** List spaces */
    public SpacesListResponse list(Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces"), query));
        return client.convertValue(raw, new TypeReference<SpacesListResponse>() {});
    }

    /** Retrieve a space */
    public SpacesRetrieveResponse retrieve(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesRetrieveResponse>() {});
    }

    /** Update a space */
    public SpacesUpdateResponse update(String spaceId, SpaceUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesUpdateResponse>() {});
    }

    /** Delete a space */
    public Void delete(String spaceId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""));
        return null;
    }

    /** List spaces members */
    public SpacesMembersListResponse membersList(String spaceId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members"), query));
        return client.convertValue(raw, new TypeReference<SpacesMembersListResponse>() {});
    }

    /** Create spaces members */
    public SpacesMembersCreateResponse201 membersCreate(String spaceId, SpaceMemberCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesMembersCreateResponse201>() {});
    }

    /** retrieve spaces members */
    public SpacesMembersRetrieveResponse membersRetrieve(String spaceId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesMembersRetrieveResponse>() {});
    }

    /** Update spaces members */
    public SpacesMembersUpdateResponse membersUpdate(String spaceId, String userId, SpaceMemberUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesMembersUpdateResponse>() {});
    }

    /** Delete spaces members */
    public Void membersDelete(String spaceId, String userId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return null;
    }

    /** List spaces groups */
    public SpacesGroupsListResponse groupsList(String spaceId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups"), query));
        return client.convertValue(raw, new TypeReference<SpacesGroupsListResponse>() {});
    }

    /** Create spaces groups */
    public SpacesGroupsCreateResponse201 groupsCreate(String spaceId, SpaceGroupCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesGroupsCreateResponse201>() {});
    }

    /** retrieve spaces groups */
    public SpacesGroupsRetrieveResponse groupsRetrieve(String spaceId, String groupId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesGroupsRetrieveResponse>() {});
    }

    /** Update spaces groups */
    public SpacesGroupsUpdateResponse groupsUpdate(String spaceId, String groupId, SpaceGroupUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesGroupsUpdateResponse>() {});
    }

    /** Delete spaces groups */
    public Void groupsDelete(String spaceId, String groupId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return null;
    }

    /** List spaces groups members */
    public SpacesGroupsMembersListResponse groupsMembersList(String spaceId, String groupId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members"), query));
        return client.convertValue(raw, new TypeReference<SpacesGroupsMembersListResponse>() {});
    }

    /** Create spaces groups members */
    public SpacesGroupsMembersCreateResponse201 groupsMembersCreate(String spaceId, String groupId, SpaceGroupMemberCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesGroupsMembersCreateResponse201>() {});
    }

    /** retrieve spaces groups members */
    public SpacesGroupsMembersRetrieveResponse groupsMembersRetrieve(String spaceId, String groupId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesGroupsMembersRetrieveResponse>() {});
    }

    /** Update spaces groups members */
    public SpacesGroupsMembersUpdateResponse groupsMembersUpdate(String spaceId, String groupId, String userId, SpaceGroupMemberUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesGroupsMembersUpdateResponse>() {});
    }

    /** Delete spaces groups members */
    public Void groupsMembersDelete(String spaceId, String groupId, String userId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return null;
    }

    /** List spaces channels */
    public SpacesChannelsListResponse channelsList(String spaceId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels"), query));
        return client.convertValue(raw, new TypeReference<SpacesChannelsListResponse>() {});
    }

    /** Create spaces channels */
    public SpacesChannelsCreateResponse201 channelsCreate(String spaceId, SpaceChannelCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesChannelsCreateResponse201>() {});
    }

    /** retrieve spaces channels */
    public SpacesChannelsRetrieveResponse channelsRetrieve(String spaceId, String channelId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesChannelsRetrieveResponse>() {});
    }

    /** Update spaces channels */
    public SpacesChannelsUpdateResponse channelsUpdate(String spaceId, String channelId, SpaceChannelUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesChannelsUpdateResponse>() {});
    }

    /** Delete spaces channels */
    public Void channelsDelete(String spaceId, String channelId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return null;
    }

    /** List spaces channels access Rules */
    public SpacesChannelsAccessRulesListResponse channelsAccessRulesList(String spaceId, String channelId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules"), query));
        return client.convertValue(raw, new TypeReference<SpacesChannelsAccessRulesListResponse>() {});
    }

    /** Create spaces channels access Rules */
    public SpacesChannelsAccessRulesCreateResponse201 channelsAccessRulesCreate(String spaceId, String channelId, SpaceChannelAccessRuleCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesChannelsAccessRulesCreateResponse201>() {});
    }

    /** Delete spaces channels access Rules */
    public Void channelsAccessRulesDelete(String spaceId, String channelId, String ruleId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules/" + serializePathParameter(ruleId, new PathParameterSpec("ruleId", "simple", false)) + ""));
        return null;
    }

    /** List spaces invites */
    public SpacesInvitesListResponse invitesList(String spaceId, String status, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("status", status, "form", true, false, null),
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites"), query));
        return client.convertValue(raw, new TypeReference<SpacesInvitesListResponse>() {});
    }

    /** Create spaces invites */
    public SpacesInvitesCreateResponse201 invitesCreate(String spaceId, SpaceInviteCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesInvitesCreateResponse201>() {});
    }

    /** retrieve spaces invites */
    public SpacesInvitesRetrieveResponse invitesRetrieve(String spaceId, String inviteCode) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesInvitesRetrieveResponse>() {});
    }

    /** Delete spaces invites */
    public Void invitesDelete(String spaceId, String inviteCode) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + ""));
        return null;
    }

    /** Accept spaces invites */
    public SdkWorkCommandResponse invitesAccept(String spaceId, String inviteCode) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + "/accept"), null);
        return client.convertValue(raw, new TypeReference<SdkWorkCommandResponse>() {});
    }

    /** List spaces bans */
    public SpacesBansListResponse bansList(String spaceId, Integer pageSize, String cursor) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans"), query));
        return client.convertValue(raw, new TypeReference<SpacesBansListResponse>() {});
    }

    /** Create spaces bans */
    public SpacesBansCreateResponse201 bansCreate(String spaceId, SpaceBanCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpacesBansCreateResponse201>() {});
    }

    /** retrieve spaces bans */
    public SpacesBansRetrieveResponse bansRetrieve(String spaceId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpacesBansRetrieveResponse>() {});
    }

    /** Delete spaces bans */
    public Void bansDelete(String spaceId, String userId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return null;
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
