package types


type BlockUserRequest struct {
	BlockedUserId string `json:"blockedUserId"`
	Scope BlockScope `json:"scope"`
	DirectChatId string `json:"directChatId"`
	ExpiresAt string `json:"expiresAt"`
}
