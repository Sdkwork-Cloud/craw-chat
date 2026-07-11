package types


type CreateThreadConversationRequest struct {
	ConversationId string `json:"conversationId"`
	ParentConversationId string `json:"parentConversationId"`
	RootMessageId string `json:"rootMessageId"`
}
