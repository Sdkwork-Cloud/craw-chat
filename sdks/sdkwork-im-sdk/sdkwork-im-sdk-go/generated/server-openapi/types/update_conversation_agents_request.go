package types


type UpdateConversationAgentsRequest struct {
	ExpectedGeneration int `json:"expectedGeneration"`
	AgentAssignments []ConversationAgentAssignment `json:"agentAssignments"`
}
