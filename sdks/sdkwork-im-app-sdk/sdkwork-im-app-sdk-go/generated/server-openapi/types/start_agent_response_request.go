package types


type StartAgentResponseRequest struct {
	ExecutionId string `json:"executionId"`
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	ConversationId string `json:"conversationId"`
	SchemaRef string `json:"schemaRef"`
	MemberId string `json:"memberId"`
	Agent AgentSubject `json:"agent"`
}
