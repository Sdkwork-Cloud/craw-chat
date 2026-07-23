package types


type PortalModuleSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
}
