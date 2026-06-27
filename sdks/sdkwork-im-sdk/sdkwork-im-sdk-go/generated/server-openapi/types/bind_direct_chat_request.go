package types


type BindDirectChatRequest struct {
	ConversationId string `json:"conversationId"`
	DirectChatId string `json:"directChatId"`
	LeftActorId string `json:"leftActorId"`
	LeftActorKind string `json:"leftActorKind"`
	RightActorId string `json:"rightActorId"`
	RightActorKind string `json:"rightActorKind"`
	TargetUserId string `json:"targetUserId"`
}
