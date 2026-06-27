package types


type ContactRecommendationView struct {
	TenantId string `json:"tenantId"`
	OwnerUserId string `json:"ownerUserId"`
	TargetUserId string `json:"targetUserId"`
	RecommendationId string `json:"recommendationId"`
	TargetConversationId string `json:"targetConversationId"`
	CreatedAt string `json:"createdAt"`
}
