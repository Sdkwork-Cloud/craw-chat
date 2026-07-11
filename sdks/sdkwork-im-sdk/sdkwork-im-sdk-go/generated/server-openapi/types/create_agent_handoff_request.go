package types


type CreateAgentHandoffRequest struct {
	ConversationId string `json:"conversationId"`
	TargetId string `json:"targetId"`
	TargetKind string `json:"targetKind"`
	HandoffSessionId string `json:"handoffSessionId"`
	HandoffReason string `json:"handoffReason"`
}
