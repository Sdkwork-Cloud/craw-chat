package api

import (
    sdktypes "github.com/sdkwork/im-backend-api-generated/types"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type AutomationApi struct {
    client *sdkhttp.Client
}

func NewAutomationApi(client *sdkhttp.Client) *AutomationApi {
    return &AutomationApi{client: client}
}

// Retrieve automation governance
func (a *AutomationApi) GovernanceRetrieve() (sdktypes.GovernanceRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/automation/governance"), nil, nil)
    if err != nil {
        var zero sdktypes.GovernanceRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.GovernanceRetrieveResponse](raw)
}
