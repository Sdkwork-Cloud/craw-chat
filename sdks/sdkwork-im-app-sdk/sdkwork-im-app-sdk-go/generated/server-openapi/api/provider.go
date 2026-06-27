package api

import (
    sdktypes "github.com/sdkwork/im-app-api-generated/types"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type ProviderApi struct {
    client *sdkhttp.Client
}

func NewProviderApi(client *sdkhttp.Client) *ProviderApi {
    return &ProviderApi{client: client}
}

// Retrieve media provider health
func (a *ProviderApi) MediaHealthRetrieve() (sdktypes.MediaHealthRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/media/provider_health"), nil, nil)
    if err != nil {
        var zero sdktypes.MediaHealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.MediaHealthRetrieveResponse](raw)
}

// Retrieve principal-profile provider health
func (a *ProviderApi) PrincipalProfileHealthRetrieve() (sdktypes.PrincipalProfileHealthRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/principal/profiles/provider_health"), nil, nil)
    if err != nil {
        var zero sdktypes.PrincipalProfileHealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.PrincipalProfileHealthRetrieveResponse](raw)
}
