package types


type PresenceView struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	DeviceId string `json:"deviceId"`
	Status string `json:"status"`
	UpdatedAt string `json:"updatedAt"`
}
