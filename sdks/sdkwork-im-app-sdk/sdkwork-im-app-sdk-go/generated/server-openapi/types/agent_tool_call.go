package types


type AgentToolCall struct {
	TenantId string `json:"tenantId"`
	ExecutionId string `json:"executionId"`
	AgentId string `json:"agentId"`
	ToolCallId string `json:"toolCallId"`
	ToolName string `json:"toolName"`
	ArgumentsPayload string `json:"argumentsPayload"`
	ResultPayload string `json:"resultPayload"`
	State AgentToolCallState `json:"state"`
	RequestedAt string `json:"requestedAt"`
	CompletedAt string `json:"completedAt"`
}
