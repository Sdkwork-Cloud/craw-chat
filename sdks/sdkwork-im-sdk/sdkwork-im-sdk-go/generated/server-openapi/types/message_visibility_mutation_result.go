package types


type MessageVisibilityMutationResult struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	IsDeleted bool `json:"isDeleted"`
	UpdatedAt string `json:"updatedAt"`
}
