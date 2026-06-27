package types


type CreateConversationRequest struct {
	ConversationId string `json:"conversationId"`
	ConversationType string `json:"conversationType"`
	Kind string `json:"kind"`
	Title string `json:"title"`
	MemberIds []string `json:"memberIds"`
}
