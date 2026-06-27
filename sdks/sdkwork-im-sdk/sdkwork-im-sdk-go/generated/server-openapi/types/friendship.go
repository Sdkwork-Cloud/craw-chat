package types


type Friendship struct {
	TenantId string `json:"tenantId"`
	FriendshipId string `json:"friendshipId"`
	InitiatorUserId string `json:"initiatorUserId"`
	LeftUserId string `json:"leftUserId"`
	RightUserId string `json:"rightUserId"`
	UserHighId string `json:"userHighId"`
	UserLowId string `json:"userLowId"`
	Status string `json:"status"`
	CreatedAt string `json:"createdAt"`
}
