package api

import (
    "encoding/json"
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-backend-api-generated/types"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type OpsApi struct {
    client *sdkhttp.Client
}

func NewOpsApi(client *sdkhttp.Client) *OpsApi {
    return &OpsApi{client: client}
}

// Retrieve ops health
func (a *OpsApi) HealthRetrieve() (sdktypes.HealthRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/health"), nil, nil)
    if err != nil {
        var zero sdktypes.HealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.HealthRetrieveResponse](raw)
}

// Retrieve cluster state
func (a *OpsApi) ClusterRetrieve() (sdktypes.ClusterRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/cluster"), nil, nil)
    if err != nil {
        var zero sdktypes.ClusterRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ClusterRetrieveResponse](raw)
}

// Retrieve projection lag
func (a *OpsApi) LagRetrieve(pageSize *int, cursor *string) (sdktypes.LagListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/ops/lag"), query), nil, nil)
    if err != nil {
        var zero sdktypes.LagListResponse
        return zero, err
    }
    return decodeResult[sdktypes.LagListResponse](raw)
}

// Retrieve replay status
func (a *OpsApi) ReplayStatusRetrieve() (sdktypes.ReplayStatusRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/replay_status"), nil, nil)
    if err != nil {
        var zero sdktypes.ReplayStatusRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ReplayStatusRetrieveResponse](raw)
}

// Retrieve commercial readiness
func (a *OpsApi) CommercialReadinessRetrieve() (sdktypes.CommercialReadinessRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/commercial_readiness"), nil, nil)
    if err != nil {
        var zero sdktypes.CommercialReadinessRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.CommercialReadinessRetrieveResponse](raw)
}

// Inspect runtime directory
func (a *OpsApi) RuntimeDirRetrieve() (sdktypes.RuntimeDirRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/runtime_dir"), nil, nil)
    if err != nil {
        var zero sdktypes.RuntimeDirRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.RuntimeDirRetrieveResponse](raw)
}

// List provider bindings
func (a *OpsApi) ProviderBindingsList(pageSize *int, cursor *string) (sdktypes.ProviderBindingSnapshotListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/ops/provider_bindings"), query), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderBindingSnapshotListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingSnapshotListResponse](raw)
}

// Retrieve provider binding drift
func (a *OpsApi) ProviderBindingsDriftRetrieve(pageSize *int, cursor *string) (sdktypes.ProviderBindingDriftListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/ops/provider_bindings/drift"), query), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderBindingDriftListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingDriftListResponse](raw)
}

// Retrieve diagnostics
func (a *OpsApi) DiagnosticsRetrieve() (sdktypes.DiagnosticsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/diagnostics"), nil, nil)
    if err != nil {
        var zero sdktypes.DiagnosticsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.DiagnosticsRetrieveResponse](raw)
}


type QueryParameterSpec struct {
    Name          string
    Value         interface{}
    Style         string
    Explode       bool
    AllowReserved bool
    ContentType   string
}

func BuildQueryString(parameters []QueryParameterSpec) string {
    pairs := make([]string, 0)
    for _, parameter := range parameters {
        AppendSerializedParameter(&pairs, parameter)
    }
    return strings.Join(pairs, "&")
}

func AppendSerializedParameter(pairs *[]string, parameter QueryParameterSpec) {
    if parameter.Value == nil {
        return
    }

    if parameter.ContentType != "" {
        encoded, _ := json.Marshal(parameter.Value)
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(string(encoded), parameter.AllowReserved))
        return
    }

    style := parameter.Style
    if style == "" {
        style = "form"
    }

    switch value := parameter.Value.(type) {
    case []string:
        AppendArrayParameter(pairs, parameter.Name, stringSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []int:
        AppendArrayParameter(pairs, parameter.Name, intSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []interface{}:
        AppendArrayParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
    case map[string]int:
        AppendObjectParameter(pairs, parameter.Name, intMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]string:
        AppendObjectParameter(pairs, parameter.Name, stringMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]interface{}:
        if style == "deepObject" {
            AppendDeepObjectParameter(pairs, parameter.Name, value, parameter.AllowReserved)
        } else {
            AppendObjectParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
        }
    default:
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(fmt.Sprint(value), parameter.AllowReserved))
    }
}

func AppendArrayParameter(pairs *[]string, name string, value []interface{}, style string, explode bool, allowReserved bool) {
    values := make([]string, 0, len(value))
    for _, item := range value {
        if item != nil {
            values = append(values, fmt.Sprint(item))
        }
    }
    if len(values) == 0 {
        return
    }
    if style == "form" && explode {
        for _, item := range values {
            *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(item, allowReserved))
        }
        return
    }
    *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(values, ","), allowReserved))
}

func AppendObjectParameter(pairs *[]string, name string, value map[string]interface{}, style string, explode bool, allowReserved bool) {
    entries := make([]string, 0, len(value)*2)
    for key, item := range value {
        if item == nil {
            continue
        }
        if style == "form" && explode {
            *pairs = append(*pairs, url.QueryEscape(key)+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
            continue
        }
        entries = append(entries, key, fmt.Sprint(item))
    }
    if len(entries) == 0 {
        return
    }
    if !(style == "form" && explode) {
        *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(entries, ","), allowReserved))
    }
}

func AppendDeepObjectParameter(pairs *[]string, name string, value map[string]interface{}, allowReserved bool) {
    for key, item := range value {
        if item == nil {
            continue
        }
        *pairs = append(*pairs, url.QueryEscape(fmt.Sprintf("%s[%s]", name, key))+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
    }
}

func EncodeQueryValue(value string, allowReserved bool) string {
    encoded := url.QueryEscape(value)
    if !allowReserved {
        return encoded
    }
    replacements := map[string]string{
        "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
        "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
        "%24": "$", "%26": "&", "%27": "'", "%28": "(",
        "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
        "%3B": ";", "%3D": "=",
    }
    for escaped, reserved := range replacements {
        encoded = strings.ReplaceAll(encoded, escaped, reserved)
    }
    return encoded
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
