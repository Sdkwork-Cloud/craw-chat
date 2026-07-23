package api

import (
    sdktypes "github.com/sdkwork/im-app-api-generated/types"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type PortalApi struct {
    client *sdkhttp.Client
}

func NewPortalApi(client *sdkhttp.Client) *PortalApi {
    return &PortalApi{client: client}
}

// Read the tenant portal access snapshot
func (a *PortalApi) AccessRetrieve() (sdktypes.AccessRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/access"), nil, nil)
    if err != nil {
        var zero sdktypes.AccessRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.AccessRetrieveResponse](raw)
}

// Read the tenant automation snapshot
func (a *PortalApi) AutomationRetrieve() (sdktypes.AutomationRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/automation"), nil, nil)
    if err != nil {
        var zero sdktypes.AutomationRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.AutomationRetrieveResponse](raw)
}

// Read the tenant conversations snapshot
func (a *PortalApi) ConversationSnapshotRetrieve() (sdktypes.ConversationSnapshotRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/conversations"), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationSnapshotRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationSnapshotRetrieveResponse](raw)
}

// Read the tenant dashboard snapshot
func (a *PortalApi) DashboardRetrieve() (sdktypes.DashboardRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/dashboard"), nil, nil)
    if err != nil {
        var zero sdktypes.DashboardRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.DashboardRetrieveResponse](raw)
}

// Read the tenant governance snapshot
func (a *PortalApi) GovernanceRetrieve() (sdktypes.GovernanceRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/governance"), nil, nil)
    if err != nil {
        var zero sdktypes.GovernanceRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.GovernanceRetrieveResponse](raw)
}

// Read the tenant portal home snapshot
func (a *PortalApi) HomeRetrieve() (sdktypes.HomeRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/home"), nil, nil)
    if err != nil {
        var zero sdktypes.HomeRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.HomeRetrieveResponse](raw)
}

// Read the tenant media snapshot
func (a *PortalApi) MediaRetrieve() (sdktypes.MediaRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/media"), nil, nil)
    if err != nil {
        var zero sdktypes.MediaRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.MediaRetrieveResponse](raw)
}

// Read the tenant realtime snapshot
func (a *PortalApi) RealtimeRetrieve() (sdktypes.RealtimeRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/realtime"), nil, nil)
    if err != nil {
        var zero sdktypes.RealtimeRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.RealtimeRetrieveResponse](raw)
}

// Read the current tenant workspace snapshot
func (a *PortalApi) WorkspaceRetrieve() (sdktypes.WorkspaceRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/portal/workspace"), nil, nil)
    if err != nil {
        var zero sdktypes.WorkspaceRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.WorkspaceRetrieveResponse](raw)
}
