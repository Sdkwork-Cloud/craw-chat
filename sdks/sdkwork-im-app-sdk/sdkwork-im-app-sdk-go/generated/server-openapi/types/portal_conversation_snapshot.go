package types


type PortalConversationSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
	Metrics PortalConversationOperationalMetrics `json:"metrics"`
}
