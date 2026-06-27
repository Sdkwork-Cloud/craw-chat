package types


type CreateAgentDialogRequest struct {
	AgentId string `json:"agentId"`
	ConversationId string `json:"conversationId"`
	Title string `json:"title"`
}
