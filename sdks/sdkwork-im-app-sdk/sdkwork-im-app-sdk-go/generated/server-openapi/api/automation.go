package api

import (
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-app-api-generated/types"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type AutomationApi struct {
    client *sdkhttp.Client
}

func NewAutomationApi(client *sdkhttp.Client) *AutomationApi {
    return &AutomationApi{client: client}
}

// Start an agent response stream
func (a *AutomationApi) AgentResponsesCreate(body sdktypes.StartAgentResponseRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(AppApiPath("/automation/agent_responses"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}

// Complete an agent response stream
func (a *AutomationApi) AgentResponsesComplete(streamId string, body sdktypes.CompleteAgentResponseRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(AppApiPath(fmt.Sprintf("/automation/agent_responses/%s/complete", SerializePathParameter(streamId, PathParameterSpec{Name: "streamId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}

// Append a frame to an agent response stream
func (a *AutomationApi) AgentResponsesFramesCreate(streamId string, body sdktypes.AppendAgentResponseDeltaRequest) (sdktypes.StreamFrame, error) {
    raw, err := a.client.Post(AppApiPath(fmt.Sprintf("/automation/agent_responses/%s/frames", SerializePathParameter(streamId, PathParameterSpec{Name: "streamId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamFrame
        return zero, err
    }
    return decodeResult[sdktypes.StreamFrame](raw)
}

// Request an agent tool call
func (a *AutomationApi) AgentToolCallsCreate(body sdktypes.RequestAgentToolCallRequest) (sdktypes.AgentToolCall, error) {
    raw, err := a.client.Post(AppApiPath("/automation/agent_tool_calls"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.AgentToolCall
        return zero, err
    }
    return decodeResult[sdktypes.AgentToolCall](raw)
}

// Request an automation execution
func (a *AutomationApi) ExecutionsCreate(body sdktypes.RequestAutomationExecution) (sdktypes.AutomationExecutionRequestResponse, error) {
    raw, err := a.client.Post(AppApiPath("/automation/executions"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.AutomationExecutionRequestResponse
        return zero, err
    }
    return decodeResult[sdktypes.AutomationExecutionRequestResponse](raw)
}

// Get an automation execution
func (a *AutomationApi) ExecutionsRetrieve(executionId string) (sdktypes.AutomationExecution, error) {
    raw, err := a.client.Get(AppApiPath(fmt.Sprintf("/automation/executions/%s", SerializePathParameter(executionId, PathParameterSpec{Name: "executionId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.AutomationExecution
        return zero, err
    }
    return decodeResult[sdktypes.AutomationExecution](raw)
}

// Complete an agent tool call
func (a *AutomationApi) AgentToolCallsComplete(executionId string, toolCallId string, body sdktypes.CompleteAgentToolCallRequest) (sdktypes.AgentToolCall, error) {
    raw, err := a.client.Post(AppApiPath(fmt.Sprintf("/automation/executions/%s/agent_tool_calls/%s/complete", SerializePathParameter(executionId, PathParameterSpec{Name: "executionId", Style: "simple", Explode: false}), SerializePathParameter(toolCallId, PathParameterSpec{Name: "toolCallId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.AgentToolCall
        return zero, err
    }
    return decodeResult[sdktypes.AgentToolCall](raw)
}

type PathParameterSpec struct {
    Name    string
    Style   string
    Explode bool
}

func SerializePathParameter(value interface{}, spec PathParameterSpec) string {
    if value == nil {
        return ""
    }
    style := spec.Style
    if style == "" {
        style = "simple"
    }

    switch typed := value.(type) {
    case []string:
        return SerializePathArray(spec.Name, stringSliceToInterface(typed), style, spec.Explode)
    case []int:
        return SerializePathArray(spec.Name, intSliceToInterface(typed), style, spec.Explode)
    case []interface{}:
        return SerializePathArray(spec.Name, typed, style, spec.Explode)
    case map[string]string:
        return SerializePathObject(spec.Name, stringMapToInterface(typed), style, spec.Explode)
    case map[string]int:
        return SerializePathObject(spec.Name, intMapToInterface(typed), style, spec.Explode)
    case map[string]interface{}:
        return SerializePathObject(spec.Name, typed, style, spec.Explode)
    default:
        return PathPrefix(spec.Name, style) + url.PathEscape(fmt.Sprint(value))
    }
}

func SerializePathArray(name string, values []interface{}, style string, explode bool) string {
    serialized := make([]string, 0, len(values))
    for _, item := range values {
        if item != nil {
            serialized = append(serialized, url.PathEscape(fmt.Sprint(item)))
        }
    }
    if len(serialized) == 0 {
        return PathPrefix(name, style)
    }
    if style == "matrix" {
        if explode {
            parts := make([]string, 0, len(serialized))
            for _, item := range serialized {
                parts = append(parts, ";"+name+"="+item)
            }
            return strings.Join(parts, "")
        }
        return ";" + name + "=" + strings.Join(serialized, ",")
    }
    separator := ","
    if explode {
        separator = "."
    }
    return PathPrefix(name, style) + strings.Join(serialized, separator)
}

func SerializePathObject(name string, values map[string]interface{}, style string, explode bool) string {
    entries := make([]string, 0, len(values)*2)
    exploded := make([]string, 0, len(values))
    for key, value := range values {
        if value == nil {
            continue
        }
        escapedKey := url.PathEscape(key)
        escapedValue := url.PathEscape(fmt.Sprint(value))
        if explode {
            if style == "matrix" {
                exploded = append(exploded, ";"+escapedKey+"="+escapedValue)
            } else {
                exploded = append(exploded, escapedKey+"="+escapedValue)
            }
        } else {
            entries = append(entries, escapedKey, escapedValue)
        }
    }
    if style == "matrix" {
        if explode {
            return strings.Join(exploded, "")
        }
        return ";" + name + "=" + strings.Join(entries, ",")
    }
    if explode {
        separator := ","
        if style == "label" {
            separator = "."
        }
        return PathPrefix(name, style) + strings.Join(exploded, separator)
    }
    return PathPrefix(name, style) + strings.Join(entries, ",")
}

func PathPrefix(name string, style string) string {
    if style == "label" {
        return "."
    }
    if style == "matrix" {
        return ";" + name
    }
    return ""
}


func stringSliceToInterface(values []string) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func intSliceToInterface(values []int) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func stringMapToInterface(values map[string]string) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}

func intMapToInterface(values map[string]int) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}
