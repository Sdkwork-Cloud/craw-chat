package types


type StreamView struct {
	TenantId string `json:"tenantId"`
	StreamId string `json:"streamId"`
	State string `json:"state"`
	OpenedAt string `json:"openedAt"`
}
