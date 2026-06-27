package com.sdkwork.im.app.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class AutomationApi {
    private final HttpClient client;
    
    public AutomationApi(HttpClient client) {
        this.client = client;
    }

    /** Start an agent response stream */
    public StreamSession agentResponsesCreate(StartAgentResponseRequest body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/agent_responses"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StreamSession>() {});
    }

    /** Complete an agent response stream */
    public StreamSession agentResponsesComplete(String streamId, CompleteAgentResponseRequest body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/agent_responses/" + serializePathParameter(streamId, new PathParameterSpec("streamId", "simple", false)) + "/complete"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StreamSession>() {});
    }

    /** Append a frame to an agent response stream */
    public StreamFrame agentResponsesFramesCreate(String streamId, AppendAgentResponseDeltaRequest body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/agent_responses/" + serializePathParameter(streamId, new PathParameterSpec("streamId", "simple", false)) + "/frames"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StreamFrame>() {});
    }

    /** Request an agent tool call */
    public AgentToolCall agentToolCallsCreate(RequestAgentToolCallRequest body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/agent_tool_calls"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<AgentToolCall>() {});
    }

    /** Request an automation execution */
    public AutomationExecutionRequestResponse executionsCreate(RequestAutomationExecution body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/executions"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<AutomationExecutionRequestResponse>() {});
    }

    /** Get an automation execution */
    public AutomationExecution executionsRetrieve(String executionId) throws Exception {
        Object raw = client.get(ApiPaths.appPath("/automation/executions/" + serializePathParameter(executionId, new PathParameterSpec("executionId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<AutomationExecution>() {});
    }

    /** Complete an agent tool call */
    public AgentToolCall agentToolCallsComplete(String executionId, String toolCallId, CompleteAgentToolCallRequest body) throws Exception {
        Object raw = client.post(ApiPaths.appPath("/automation/executions/" + serializePathParameter(executionId, new PathParameterSpec("executionId", "simple", false)) + "/agent_tool_calls/" + serializePathParameter(toolCallId, new PathParameterSpec("toolCallId", "simple", false)) + "/complete"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<AgentToolCall>() {});
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
