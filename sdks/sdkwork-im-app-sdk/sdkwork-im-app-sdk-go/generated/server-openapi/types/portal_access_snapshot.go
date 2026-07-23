package types


type PortalAccessSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	RecentItems []PortalAuditRecordView `json:"recentItems"`
	HasMore bool `json:"hasMore"`
}
