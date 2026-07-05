package types


type OpenApiBlockUserRequest struct {
	BlockedUserId string `json:"blockedUserId"`
	Scope string `json:"scope"`
}
