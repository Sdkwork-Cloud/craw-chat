package types


type FriendRequest struct {
	TenantId string `json:"tenantId"`
	RequestId string `json:"requestId"`
	RequesterUserId string `json:"requesterUserId"`
	TargetUserId string `json:"targetUserId"`
	Status string `json:"status"`
	RequestMessage string `json:"requestMessage"`
	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
