package types


type EstablishExternalConnectionRequest struct {
	ConnectionId string `json:"connectionId"`
	ConnectionKind string `json:"connectionKind"`
	EstablishedAt string `json:"establishedAt"`
	EventId string `json:"eventId"`
	ExternalOrgName string `json:"externalOrgName"`
	ExternalTenantId string `json:"externalTenantId"`
}
