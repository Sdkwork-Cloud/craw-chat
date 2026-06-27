package api

import (
    sdktypes "github.com/sdkwork/im-backend-api-generated/types"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type AuditApi struct {
    client *sdkhttp.Client
}

func NewAuditApi(client *sdkhttp.Client) *AuditApi {
    return &AuditApi{client: client}
}

// List audit records
func (a *AuditApi) RecordsList() (sdktypes.RecordsListResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/audit/records"), nil, nil)
    if err != nil {
        var zero sdktypes.RecordsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.RecordsListResponse](raw)
}

// Record audit anchor
func (a *AuditApi) RecordsCreate() (sdktypes.RecordsCreateResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/audit/records"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.RecordsCreateResponse
        return zero, err
    }
    return decodeResult[sdktypes.RecordsCreateResponse](raw)
}

// Export audit bundle
func (a *AuditApi) ExportRetrieve() (sdktypes.ExportRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/audit/export"), nil, nil)
    if err != nil {
        var zero sdktypes.ExportRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ExportRetrieveResponse](raw)
}
