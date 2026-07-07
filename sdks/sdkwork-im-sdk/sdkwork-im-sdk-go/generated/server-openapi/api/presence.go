package api

import (
    sdktypes "github.com/sdkwork/im-sdk-generated/types"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type PresenceApi struct {
    client *sdkhttp.Client
}

func NewPresenceApi(client *sdkhttp.Client) *PresenceApi {
    return &PresenceApi{client: client}
}

// Publish current client route presence heartbeat
func (a *PresenceApi) Heartbeat(body sdktypes.PresenceHeartbeatRequest) (sdktypes.PresenceHeartbeatResponse, error) {
    raw, err := a.client.Post(ImApiPath("/presence/heartbeat"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PresenceHeartbeatResponse
        return zero, err
    }
    return decodeResult[sdktypes.PresenceHeartbeatResponse](raw)
}

// Retrieve current principal presence
func (a *PresenceApi) MeRetrieve() (sdktypes.PresenceMeRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath("/presence/me"), nil, nil)
    if err != nil {
        var zero sdktypes.PresenceMeRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.PresenceMeRetrieveResponse](raw)
}
