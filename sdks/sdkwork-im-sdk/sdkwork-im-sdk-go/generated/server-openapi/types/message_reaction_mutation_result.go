package types


type MessageReactionMutationResult struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	ReactionKey string `json:"reactionKey"`
	Count int `json:"count"`
	UpdatedAt string `json:"updatedAt"`
}
