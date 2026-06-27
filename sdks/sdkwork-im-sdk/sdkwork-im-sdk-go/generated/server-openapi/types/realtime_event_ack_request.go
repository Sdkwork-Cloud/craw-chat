package types


type RealtimeEventAckRequest struct {
	EventIds []string `json:"eventIds"`
}
