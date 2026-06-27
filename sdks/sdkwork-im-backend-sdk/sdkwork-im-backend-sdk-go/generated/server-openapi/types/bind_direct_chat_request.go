package types


type BindDirectChatRequest struct {
	BoundAt string `json:"boundAt"`
	ConversationId string `json:"conversationId"`
	DirectChatId string `json:"directChatId"`
	EventId string `json:"eventId"`
	LeftActorId string `json:"leftActorId"`
	RightActorId string `json:"rightActorId"`
}
