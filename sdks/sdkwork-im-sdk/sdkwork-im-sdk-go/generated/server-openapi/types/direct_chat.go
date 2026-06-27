package types


type DirectChat struct {
	TenantId string `json:"tenantId"`
	DirectChatId string `json:"directChatId"`
	ConversationId string `json:"conversationId"`
	Status string `json:"status"`
}
