package com.sdkwork.im.sdk.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.model.*;
import java.util.List;
import java.util.Map;

public class CallsApi {
    private final HttpClient client;

    public CallsApi(HttpClient client) {
        this.client = client;
    }

    /** Create an IM call signaling session */
    public RtcSessionMutationResponse sessionsCreate(CreateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSessionMutationResponse>() {});
    }

    /** Retrieve IM call signaling session state */
    public RtcSession sessionsRetrieve(String rtcSessionId) throws Exception {
        Object raw = client.get(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** Invite participants into an IM call signaling session */
    public RtcSessionMutationResponse sessionsInvite(String rtcSessionId, InviteRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/invite"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSessionMutationResponse>() {});
    }

    /** Accept an IM call signaling session */
    public RtcSessionMutationResponse sessionsAccept(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/accept"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSessionMutationResponse>() {});
    }

    /** Reject an IM call signaling session */
    public RtcSessionMutationResponse sessionsReject(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/reject"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSessionMutationResponse>() {});
    }

    /** End an IM call signaling session */
    public RtcSessionMutationResponse sessionsEnd(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/end"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSessionMutationResponse>() {});
    }

    /** Post an IM call signaling event */
    public RtcSignalEvent sessionsSignalsCreate(String rtcSessionId, PostRtcSignalRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/signals"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSignalEvent>() {});
    }

    /** Issue an RTC media participant credential for an IM call */
    public RtcParticipantCredential sessionsCredentialsCreate(String rtcSessionId, IssueRtcParticipantCredentialRequest body) throws Exception {
        Object raw = client.post(ApiPaths.imPath("/calls/sessions/" + serializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false)) + "/credentials"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcParticipantCredential>() {});
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
