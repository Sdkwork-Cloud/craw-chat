package types


type RequestAgentToolCallRequest struct {
	ExecutionId string `json:"executionId"`
	ToolCallId string `json:"toolCallId"`
	ToolName string `json:"toolName"`
	ArgumentsPayload string `json:"argumentsPayload"`
}
