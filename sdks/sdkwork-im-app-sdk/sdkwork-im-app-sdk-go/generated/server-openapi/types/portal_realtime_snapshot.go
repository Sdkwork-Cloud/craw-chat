package types


type PortalRealtimeSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
	Metrics PortalRealtimeMetrics `json:"metrics"`
}
