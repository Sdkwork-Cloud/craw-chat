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
    public SpaceView create(SpaceCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceView>() {});
    }

    /** List spaces */
    public SpaceListResponse list() throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces"));
        return client.convertValue(raw, new TypeReference<SpaceListResponse>() {});
    }

    /** Get a space */
    public SpaceView get(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceView>() {});
    }

    /** Update a space */
    public SpaceView update(String spaceId, SpaceUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceView>() {});
    }

    /** Delete a space */
    public Void delete(String spaceId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + ""));
        return null;
    }

    /** List spaces members */
    public SpaceMemberListResponse membersList(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members"));
        return client.convertValue(raw, new TypeReference<SpaceMemberListResponse>() {});
    }

    /** Create spaces members */
    public SpaceMemberView membersCreate(String spaceId, SpaceMemberCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceMemberView>() {});
    }

    /** Get spaces members */
    public SpaceMemberView membersGet(String spaceId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceMemberView>() {});
    }

    /** Update spaces members */
    public SpaceMemberView membersUpdate(String spaceId, String userId, SpaceMemberUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceMemberView>() {});
    }

    /** Delete spaces members */
    public Void membersDelete(String spaceId, String userId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return null;
    }

    /** List spaces groups */
    public SpaceGroupListResponse groupsList(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups"));
        return client.convertValue(raw, new TypeReference<SpaceGroupListResponse>() {});
    }

    /** Create spaces groups */
    public SpaceGroupView groupsCreate(String spaceId, SpaceGroupCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceGroupView>() {});
    }

    /** Get spaces groups */
    public SpaceGroupView groupsGet(String spaceId, String groupId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceGroupView>() {});
    }

    /** Update spaces groups */
    public SpaceGroupView groupsUpdate(String spaceId, String groupId, SpaceGroupUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceGroupView>() {});
    }

    /** Delete spaces groups */
    public Void groupsDelete(String spaceId, String groupId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return null;
    }

    /** List spaces groups members */
    public SpaceGroupMemberListResponse groupsMembersList(String spaceId, String groupId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members"));
        return client.convertValue(raw, new TypeReference<SpaceGroupMemberListResponse>() {});
    }

    /** Create spaces groups members */
    public SpaceGroupMemberView groupsMembersCreate(String spaceId, String groupId, SpaceGroupMemberCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceGroupMemberView>() {});
    }

    /** Get spaces groups members */
    public SpaceGroupMemberView groupsMembersGet(String spaceId, String groupId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceGroupMemberView>() {});
    }

    /** Update spaces groups members */
    public Void groupsMembersUpdate(String spaceId, String groupId, String userId, SpaceGroupMemberUpdateRequest body) throws Exception {
        client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""), body, null, null, "application/json");
        return null;
    }

    /** Delete spaces groups members */
    public Void groupsMembersDelete(String spaceId, String groupId, String userId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/members/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return null;
    }

    /** List spaces channels */
    public SpaceChannelListResponse channelsList(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels"));
        return client.convertValue(raw, new TypeReference<SpaceChannelListResponse>() {});
    }

    /** Create spaces channels */
    public SpaceChannelView channelsCreate(String spaceId, SpaceChannelCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceChannelView>() {});
    }

    /** Get spaces channels */
    public SpaceChannelView channelsGet(String spaceId, String channelId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceChannelView>() {});
    }

    /** Update spaces channels */
    public SpaceChannelView channelsUpdate(String spaceId, String channelId, SpaceChannelUpdateRequest body) throws Exception {
        Object raw = client.patch(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceChannelView>() {});
    }

    /** Delete spaces channels */
    public Void channelsDelete(String spaceId, String channelId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return null;
    }

    /** List spaces channels access Rules */
    public SpaceChannelAccessRuleListResponse channelsAccessRulesList(String spaceId, String channelId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules"));
        return client.convertValue(raw, new TypeReference<SpaceChannelAccessRuleListResponse>() {});
    }

    /** Create spaces channels access Rules */
    public SpaceChannelAccessRuleView channelsAccessRulesCreate(String spaceId, String channelId, SpaceChannelAccessRuleCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceChannelAccessRuleView>() {});
    }

    /** Delete spaces channels access Rules */
    public Void channelsAccessRulesDelete(String spaceId, String channelId, String ruleId) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/access_rules/" + serializePathParameter(ruleId, new PathParameterSpec("ruleId", "simple", false)) + ""));
        return null;
    }

    /** List spaces invites */
    public SpaceInviteListResponse invitesList(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites"));
        return client.convertValue(raw, new TypeReference<SpaceInviteListResponse>() {});
    }

    /** Create spaces invites */
    public SpaceInviteView invitesCreate(String spaceId, SpaceInviteCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceInviteView>() {});
    }

    /** Get spaces invites */
    public SpaceInviteView invitesGet(String spaceId, String inviteCode) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceInviteView>() {});
    }

    /** Revoke spaces invites */
    public Void invitesRevoke(String spaceId, String inviteCode) throws Exception {
        client.delete(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + ""));
        return null;
    }

    /** Accept spaces invites */
    public Void invitesAccept(String spaceId, String inviteCode) throws Exception {
        client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/invites/" + serializePathParameter(inviteCode, new PathParameterSpec("inviteCode", "simple", false)) + "/accept"), null);
        return null;
    }

    /** List spaces bans */
    public SpaceBanListResponse bansList(String spaceId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans"));
        return client.convertValue(raw, new TypeReference<SpaceBanListResponse>() {});
    }

    /** Create spaces bans */
    public SpaceBanView bansCreate(String spaceId, SpaceBanCreateRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SpaceBanView>() {});
    }

    /** Get spaces bans */
    public SpaceBanView bansGet(String spaceId, String userId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/spaces/" + serializePathParameter(spaceId, new PathParameterSpec("spaceId", "simple", false)) + "/bans/" + serializePathParameter(userId, new PathParameterSpec("userId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<SpaceBanView>() {});
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



}
