package types


type RealtimeEventView struct {
	EventId string `json:"eventId"`
	Scope string `json:"scope"`
	ScopeId string `json:"scopeId"`
	EventType string `json:"eventType"`
	Payload string `json:"payload"`
	OccurredAt string `json:"occurredAt"`
}
