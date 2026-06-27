package types


type AgentSubject struct {
	AgentId string `json:"agent_id"`
	SessionId string `json:"session_id"`
	Metadata StringMap `json:"metadata"`
}
