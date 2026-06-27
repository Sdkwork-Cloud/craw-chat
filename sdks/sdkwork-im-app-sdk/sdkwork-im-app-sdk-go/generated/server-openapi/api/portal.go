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

// Read the tenant portal sign-in snapshot
func (a *PortalApi) AccessRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/access"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant automation snapshot
func (a *PortalApi) AutomationRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/automation"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant conversations snapshot
func (a *PortalApi) ConversationSnapshotRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/conversations"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant dashboard snapshot
func (a *PortalApi) DashboardRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/dashboard"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant governance snapshot
func (a *PortalApi) GovernanceRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/governance"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant portal home snapshot
func (a *PortalApi) HomeRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/home"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant media snapshot
func (a *PortalApi) MediaRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/media"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant realtime snapshot
func (a *PortalApi) RealtimeRetrieve() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(AppApiPath("/portal/realtime"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the current tenant workspace snapshot
func (a *PortalApi) WorkspaceRetrieve() (sdktypes.PortalWorkspaceView, error) {
    raw, err := a.client.Get(AppApiPath("/portal/workspace"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalWorkspaceView
        return zero, err
    }
    return decodeResult[sdktypes.PortalWorkspaceView](raw)
}
