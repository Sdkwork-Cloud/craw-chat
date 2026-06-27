package types


type ConversationProfileView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
	Notice string `json:"notice"`
	UpdatedAt string `json:"updatedAt"`
	UpdatedByPrincipalKind string `json:"updatedByPrincipalKind"`
	UpdatedByPrincipalId string `json:"updatedByPrincipalId"`
}
