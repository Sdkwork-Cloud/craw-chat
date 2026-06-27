package types


type CreateConversationResult struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	Kind string `json:"kind"`
	CreatedAt string `json:"createdAt"`
}
