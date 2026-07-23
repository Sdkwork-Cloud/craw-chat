package types


type PortalDashboardSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
	Metrics PortalOperationalMetrics `json:"metrics"`
}
