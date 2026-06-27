package types


type MessagePinMutationResult struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	IsPinned bool `json:"isPinned"`
	UpdatedAt string `json:"updatedAt"`
}
