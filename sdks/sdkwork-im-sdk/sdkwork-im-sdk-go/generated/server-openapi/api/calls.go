package api

import (
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-sdk-generated/types"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type CallsApi struct {
    client *sdkhttp.Client
}

func NewCallsApi(client *sdkhttp.Client) *CallsApi {
    return &CallsApi{client: client}
}

// Create an IM call signaling session
func (a *CallsApi) SessionsCreate(body sdktypes.CreateRtcSessionRequest) (sdktypes.RtcSessionMutationResponse, error) {
    raw, err := a.client.Post(ImApiPath("/calls/sessions"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSessionMutationResponse
        return zero, err
    }
    return decodeResult[sdktypes.RtcSessionMutationResponse](raw)
}

// Retrieve IM call signaling session state
func (a *CallsApi) SessionsRetrieve(rtcSessionId string) (sdktypes.RtcSession, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/calls/sessions/%s", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// Invite participants into an IM call signaling session
func (a *CallsApi) SessionsInvite(rtcSessionId string, body sdktypes.InviteRtcSessionRequest) (sdktypes.RtcSessionMutationResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/invite", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSessionMutationResponse
        return zero, err
    }
    return decodeResult[sdktypes.RtcSessionMutationResponse](raw)
}

// Accept an IM call signaling session
func (a *CallsApi) SessionsAccept(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSessionMutationResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/accept", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSessionMutationResponse
        return zero, err
    }
    return decodeResult[sdktypes.RtcSessionMutationResponse](raw)
}

// Reject an IM call signaling session
func (a *CallsApi) SessionsReject(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSessionMutationResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/reject", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSessionMutationResponse
        return zero, err
    }
    return decodeResult[sdktypes.RtcSessionMutationResponse](raw)
}

// End an IM call signaling session
func (a *CallsApi) SessionsEnd(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSessionMutationResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/end", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSessionMutationResponse
        return zero, err
    }
    return decodeResult[sdktypes.RtcSessionMutationResponse](raw)
}

// Post an IM call signaling event
func (a *CallsApi) SessionsSignalsCreate(rtcSessionId string, body sdktypes.PostRtcSignalRequest) (sdktypes.RtcSignalEvent, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/signals", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSignalEvent
        return zero, err
    }
    return decodeResult[sdktypes.RtcSignalEvent](raw)
}

// Issue an RTC media participant credential for an IM call
func (a *CallsApi) SessionsCredentialsCreate(rtcSessionId string, body sdktypes.IssueRtcParticipantCredentialRequest) (sdktypes.RtcParticipantCredential, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/calls/sessions/%s/credentials", SerializePathParameter(rtcSessionId, PathParameterSpec{Name: "rtcSessionId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcParticipantCredential
        return zero, err
    }
    return decodeResult[sdktypes.RtcParticipantCredential](raw)
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
