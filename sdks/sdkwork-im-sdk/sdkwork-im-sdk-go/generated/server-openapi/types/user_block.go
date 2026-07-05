package types


type UserBlock struct {
	TenantId string `json:"tenantId"`
	BlockId string `json:"blockId"`
	BlockerUserId string `json:"blockerUserId"`
	BlockedUserId string `json:"blockedUserId"`
	Scope string `json:"scope"`
	Status string `json:"status"`
	DirectChatId string `json:"directChatId"`
	ExpiresAt string `json:"expiresAt"`
	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
