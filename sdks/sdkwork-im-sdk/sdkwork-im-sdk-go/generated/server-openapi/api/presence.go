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
func (a *PresenceApi) HeartbeatCreate(body sdktypes.PresenceHeartbeatRequest) (sdktypes.PresenceView, error) {
    raw, err := a.client.Post(ImApiPath("/presence/heartbeat"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PresenceView
        return zero, err
    }
    return decodeResult[sdktypes.PresenceView](raw)
}

// Retrieve current principal presence
func (a *PresenceApi) MeRetrieve() (sdktypes.PresenceView, error) {
    raw, err := a.client.Get(ImApiPath("/presence/me"), nil, nil)
    if err != nil {
        var zero sdktypes.PresenceView
        return zero, err
    }
    return decodeResult[sdktypes.PresenceView](raw)
}
