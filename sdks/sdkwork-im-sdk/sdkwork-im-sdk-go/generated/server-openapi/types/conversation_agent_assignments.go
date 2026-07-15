package types


type ConversationAgentAssignments struct {
	Generation int `json:"generation"`
	Source string `json:"source"`
	Agents []ConversationAgentAssignment `json:"agents"`
}
