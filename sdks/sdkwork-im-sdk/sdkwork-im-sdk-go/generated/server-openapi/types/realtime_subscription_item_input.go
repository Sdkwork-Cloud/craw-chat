package types


type RealtimeSubscriptionItemInput struct {
	ScopeType string `json:"scopeType"`
	ScopeId string `json:"scopeId"`
	EventTypes []string `json:"eventTypes"`
}
