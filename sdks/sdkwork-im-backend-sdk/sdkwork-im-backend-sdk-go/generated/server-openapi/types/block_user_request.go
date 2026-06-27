package types


type BlockUserRequest struct {
	BlockId string `json:"blockId"`
	BlockedUserId string `json:"blockedUserId"`
	BlockerUserId string `json:"blockerUserId"`
	DirectChatId string `json:"directChatId"`
	EffectiveAt string `json:"effectiveAt"`
	EventId string `json:"eventId"`
	ExpiresAt string `json:"expiresAt"`
	Scope string `json:"scope"`
}
