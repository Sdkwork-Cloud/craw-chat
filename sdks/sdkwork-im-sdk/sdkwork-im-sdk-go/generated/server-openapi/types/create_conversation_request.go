package types


type CreateConversationRequest struct {
	ConversationId string `json:"conversationId"`
	ConversationType string `json:"conversationType"`
	GroupName string `json:"groupName"`
	ClientRequestKey string `json:"clientRequestKey"`
	InitializeKnowledgebase bool `json:"initializeKnowledgebase"`
	MemberUserIds []string `json:"memberUserIds"`
	AgentAssignments []ConversationAgentAssignment `json:"agentAssignments"`
	PolicyVersion string `json:"policyVersion"`
	CapabilityFlags []string `json:"capabilityFlags"`
	HistoryVisibility string `json:"historyVisibility"`
	RetentionPolicyRef string `json:"retentionPolicyRef"`
}
