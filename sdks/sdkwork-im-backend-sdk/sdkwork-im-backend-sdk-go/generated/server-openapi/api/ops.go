package api

import (
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
func (a *OpsApi) LagRetrieve() (sdktypes.LagRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/lag"), nil, nil)
    if err != nil {
        var zero sdktypes.LagRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.LagRetrieveResponse](raw)
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
func (a *OpsApi) ProviderBindingsList() (sdktypes.OpsProviderBindingsListResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/provider_bindings"), nil, nil)
    if err != nil {
        var zero sdktypes.OpsProviderBindingsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.OpsProviderBindingsListResponse](raw)
}

// Retrieve provider binding drift
func (a *OpsApi) ProviderBindingsDriftRetrieve() (sdktypes.OpsProviderBindingsDriftRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/ops/provider_bindings/drift"), nil, nil)
    if err != nil {
        var zero sdktypes.OpsProviderBindingsDriftRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.OpsProviderBindingsDriftRetrieveResponse](raw)
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
