package types


type ConversationPreferencesView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	IsPinned bool `json:"isPinned"`
	IsMuted bool `json:"isMuted"`
	IsMarkedUnread bool `json:"isMarkedUnread"`
	IsHidden bool `json:"isHidden"`
	UpdatedAt string `json:"updatedAt"`
}
